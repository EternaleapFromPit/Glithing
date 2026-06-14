use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub fn run() -> Result<(), String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let sample_dir = manifest_dir
        .join("external")
        .join("aspnetcore-realworld-example-app")
        .join("src")
        .join("Conduit");
    if !sample_dir.is_dir() {
        return Err(format!(
            "RealWorld sample directory not found: {}",
            sample_dir.display()
        ));
    }

    let sample_source = crate::read_input_source(sample_dir.to_str().ok_or_else(|| {
        format!("sample path is not valid UTF-8: {}", sample_dir.display())
    })?)?;
    let compiled = crate::compile_source_with_options(&sample_source, true, false)?;
    let llvm_ir = compiled.llvm_ir()?.to_string();
    let output_exe = temp_executable_path();
    let _ = std::fs::remove_file(&output_exe);

    crate::toolchain::emit_native_executable(
        &llvm_ir,
        &compiled.native_sources,
        output_exe
            .to_str()
            .ok_or_else(|| format!("output path is not valid UTF-8: {}", output_exe.display()))?,
    )?;

    let port = free_tcp_port()?;
    let child = Command::new(&output_exe)
        .current_dir(&sample_dir)
        .env("GLITCH_HTTP_PORT", port.to_string())
        .env("GLITCH_HTTP_MAX_REQUESTS", "2")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to start RealWorld native binary {}: {e}", output_exe.display()))?;

    let child = wait_for_port_ready(child, port, Duration::from_secs(20))?;

    let articles = http_get(port, "/api/articles")?;
    if !articles.starts_with("HTTP/1.1 200") {
        return fail_with_response("GET /api/articles", articles, child);
    }

    let swagger = http_get(port, "/swagger/v1/swagger.json")?;
    if !swagger.starts_with("HTTP/1.1 200") {
        return fail_with_response("GET /swagger/v1/swagger.json", swagger, child);
    }

    wait_for_exit(child, Duration::from_secs(10))?;
    Ok(())
}

fn temp_executable_path() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("glitching-realworld-smoke-{stamp}.exe"))
}

fn free_tcp_port() -> Result<u16, String> {
    let listener = TcpListener::bind(("127.0.0.1", 0))
        .map_err(|e| format!("failed to reserve a free TCP port: {e}"))?;
    let port = listener
        .local_addr()
        .map_err(|e| format!("failed to query reserved TCP port: {e}"))?
        .port();
    drop(listener);
    Ok(port)
}

fn wait_for_port_ready(mut child: Child, port: u16, timeout: Duration) -> Result<Child, String> {
    let deadline = Instant::now() + timeout;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|e| format!("failed to poll RealWorld host: {e}"))?
        {
            let output = child
                .wait_with_output()
                .map_err(|e| format!("failed to collect RealWorld host output: {e}"))?;
            return Err(format!(
                "RealWorld host exited before it accepted requests: {}\nstdout:\n{}\nstderr:\n{}",
                describe_exit_status(status),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        match TcpStream::connect_timeout(&addr, Duration::from_millis(200)) {
            Ok(_) => return Ok(child),
            Err(err) if Instant::now() >= deadline => {
                return Err(format!("timed out waiting for {addr} to accept connections: {err}"));
            }
            Err(_) => std::thread::sleep(Duration::from_millis(100)),
        }
    }
}

fn wait_for_exit(mut child: Child, timeout: Duration) -> Result<(), String> {
    let deadline = Instant::now() + timeout;
    loop {
        match child
            .try_wait()
            .map_err(|e| format!("failed to poll RealWorld host: {e}"))?
        {
            Some(status) if status.success() => return Ok(()),
            Some(status) => {
                let output = child
                    .wait_with_output()
                    .map_err(|e| format!("failed to collect RealWorld host output: {e}"))?;
                return Err(format!(
                    "RealWorld host exited unsuccessfully: {}\nstdout:\n{}\nstderr:\n{}",
                    describe_exit_status(status),
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            None if Instant::now() >= deadline => {
                let _ = child.kill();
                let output = child
                    .wait_with_output()
                    .map_err(|e| format!("failed to collect RealWorld host output after timeout: {e}"))?;
                return Err(format!(
                    "RealWorld host did not exit after the smoke requests\nstdout:\n{}\nstderr:\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            None => std::thread::sleep(Duration::from_millis(100)),
        }
    }
}

fn http_get(port: u16, path: &str) -> Result<String, String> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(5))
        .map_err(|e| format!("failed to connect to {addr}: {e}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("failed to set read timeout: {e}"))?;
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("failed to set write timeout: {e}"))?;
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|e| format!("failed to write HTTP request: {e}"))?;
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("failed to read HTTP response: {e}"))?;
    Ok(response)
}

fn fail_with_response(
    label: &str,
    response: String,
    mut child: Child,
) -> Result<(), String> {
    let _ = child.kill();
    let output = child
        .wait_with_output()
        .map_err(|e| format!("failed to collect RealWorld host output after {label}: {e}"))?;
    Err(format!(
        "{label} returned an unexpected response:\n{response}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ))
}

fn describe_exit_status(status: ExitStatus) -> String {
    match status.code() {
        Some(code) => format!("exit code {code}"),
        None => "terminated by signal".to_string(),
    }
}
