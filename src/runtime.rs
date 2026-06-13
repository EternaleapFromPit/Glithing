use std::ffi::{c_char, c_void, CStr, CString};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

// ---------------------------------------------------------------------------
// Thread-safe string-registry lock
// ---------------------------------------------------------------------------
// The LLVM-emitted glitch_string_allocate / _retain / _release functions
// manipulate a global linked list (@glitch_string_allocations).  They call
// these two functions as a mutex guard around every list mutation so that
// concurrent request threads do not corrupt the registry.

static STRING_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

// SAFETY: called from LLVM IR before any access to the string registry.
// Returns an opaque token (the raw MutexGuard pointer) that the caller must
// pass back to GlitchString_Unlock.  Because the guard is heap-allocated here
// and freed in Unlock, it survives across the C ABI boundary.
#[no_mangle]
pub extern "C" fn GlitchString_Lock() -> *mut std::ffi::c_void {
    let lock = STRING_LOCK.get_or_init(|| Mutex::new(()));
    // We box the guard to give it a stable address across the FFI boundary.
    // The guard keeps the mutex locked until it is dropped in GlitchString_Unlock.
    let guard = match lock.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(), // poisoned — recover
    };
    let boxed: Box<std::sync::MutexGuard<'static, ()>> =
        unsafe { std::mem::transmute(Box::new(guard)) };
    Box::into_raw(boxed) as *mut std::ffi::c_void
}

// SAFETY: `token` must be a pointer returned by GlitchString_Lock and must
// not be used again after this call.
#[no_mangle]
pub unsafe extern "C" fn GlitchString_Unlock(token: *mut std::ffi::c_void) {
    if !token.is_null() {
        drop(Box::from_raw(
            token as *mut std::sync::MutexGuard<'static, ()>,
        ));
    }
}

type RequestHandler =
    unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char, *const c_char) -> *mut c_char;
type StringRelease = unsafe extern "C" fn(*mut c_char);

#[no_mangle]
pub unsafe extern "C" fn GlitchRestHost_Run(
    app: *mut c_void,
    port: i32,
    max_requests: i32,
    handler: RequestHandler,
    release: StringRelease,
) {
    let _ = std::panic::catch_unwind(|| run_host(app, port, max_requests, handler, release));
}

fn run_host(
    app: *mut c_void,
    port: i32,
    max_requests: i32,
    handler: RequestHandler,
    release: StringRelease,
) {
    let port = std::env::var("GLITCH_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .or_else(|| u16::try_from(port).ok())
        .unwrap_or(5000);
    let max_requests = std::env::var("GLITCH_MAX_REQUESTS")
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(max_requests);

    let listener = match TcpListener::bind(("0.0.0.0", port)) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Glitching HTTP host failed to bind port {port}: {error}");
            return;
        }
    };
    eprintln!("Glitching HTTP host listening on port {port}");

    // Thread-safe request counter. If max_requests <= 0, run forever.
    let remaining = Arc::new(AtomicI32::new(max_requests));

    for connection in listener.incoming() {
        let Ok(stream) = connection else {
            continue;
        };

        // Check and decrement remaining before spawning.
        if max_requests > 0 {
            let prev = remaining.fetch_sub(1, Ordering::SeqCst);
            if prev <= 0 {
                break;
            }
        }

        // Each connection is handled on its own thread.
        // Handler and release are function pointers — they may be called
        // concurrently; the gl program's own synchronisation must protect
        // shared mutable state (e.g. the string-allocation registry).
        let remaining_clone = remaining.clone();
        let app_addr = app as usize;
        let _ = std::thread::spawn(move || {
            let mut stream = stream;
            handle_connection(app_addr as *mut c_void, &mut stream, handler, release);
            drop(remaining_clone);
        });
    }
}

/// Response envelope written back by the generated handler.
/// Layout must match the LLVM struct emitted by the compiler:
///   %glitch.response = type { i32, ptr, ptr }
///   fields: status_code, body (owned ptr), content_type (static ptr)
#[allow(dead_code)]
#[repr(C)]
pub struct GlitchResponse {
    pub status_code: i32,
    /// Owned string body (must be released via `release`).
    pub body: *mut c_char,
    /// Static content-type string (not owned, never released).
    pub content_type: *const c_char,
}

fn handle_connection(
    app: *mut c_void,
    stream: &mut TcpStream,
    handler: RequestHandler,
    release: StringRelease,
) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(10)));
    let mut request = Vec::with_capacity(4096);
    let mut chunk = [0_u8; 4096];
    loop {
        let Ok(read) = stream.read(&mut chunk) else {
            return;
        };
        if read == 0 {
            return;
        }
        request.extend_from_slice(&chunk[..read]);
        if request.windows(4).any(|window| window == b"\r\n\r\n") || request.len() >= 1024 * 1024 {
            break;
        }
    }

    let header_end = request
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
        .unwrap_or(request.len());

    let (method, path, content_length, auth_header) = {
        let header = String::from_utf8_lossy(&request[..header_end]);
        let mut request_line = header.lines().next().unwrap_or_default().split_whitespace();
        let method = request_line.next().unwrap_or("GET").to_string();
        let path = request_line.next().unwrap_or("/").to_string();
        let mut content_length: usize = 0;
        let mut auth_header = String::new();
        for line in header.lines().skip(1) {
            if let Some((name, value)) = line.split_once(':') {
                let name = name.trim();
                let value = value.trim();
                if name.eq_ignore_ascii_case("content-length") {
                    content_length = value.parse().unwrap_or_default();
                } else if name.eq_ignore_ascii_case("authorization") {
                    auth_header = value.to_string();
                }
            }
        }
        (method, path, content_length, auth_header)
    };

    // Append auth header info to path as a query parameter so handlers can
    // extract it without a separate parameter slot (compatible with the current
    // single-pointer path ABI used by the endpoint thunks).
    let path_with_auth = if auth_header.is_empty() {
        path
    } else {
        let sep = if path.contains('?') { '&' } else { '?' };
        format!("{path}{sep}__auth={auth_header}")
    };

    while request.len().saturating_sub(header_end) < content_length {
        let Ok(read) = stream.read(&mut chunk) else {
            return;
        };
        if read == 0 {
            break;
        }
        request.extend_from_slice(&chunk[..read]);
    }
    let body_end = header_end.saturating_add(content_length).min(request.len());
    let body = &request[header_end..body_end];

    let method_ffi = ffi_string(method.as_bytes());
    let path_ffi = ffi_string(path_with_auth.as_bytes());
    let body_ffi = ffi_string(body);

    // The handler returns a raw JSON string (or "404" / "401" sentinel).
    let response = unsafe {
        handler(
            app,
            method_ffi.as_ptr(),
            path_ffi.as_ptr(),
            body_ffi.as_ptr(),
        )
    };

    let response_bytes = if response.is_null() {
        b"null".as_slice()
    } else {
        unsafe { CStr::from_ptr(response) }.to_bytes()
    };

    let (status, content_type) = classify_response(response_bytes);
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
        response_bytes.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(response_bytes);
    let _ = stream.flush();
    if !response.is_null() {
        unsafe { release(response) };
    }
}

/// Map a handler response body to an HTTP status code and content-type.
/// Handlers signal non-200 conditions by returning a sentinel JSON string
/// of the form `{"__status":NNN,...}` or one of the legacy sentinel bytes.
fn classify_response(body: &[u8]) -> (&'static str, &'static str) {
    // Legacy numeric sentinels kept for backward compat.
    if body == b"404" {
        return ("404 Not Found", "application/json");
    }
    if body == b"401" {
        return ("401 Unauthorized", "application/json");
    }
    if body == b"403" {
        return ("403 Forbidden", "application/json");
    }
    if body == b"400" {
        return ("400 Bad Request", "application/json");
    }
    if body == b"500" {
        return ("500 Internal Server Error", "application/json");
    }
    if body == b"501" {
        return ("501 Not Implemented", "application/json");
    }
    // Structured envelope: `{"__status":NNN, ...}`
    if let Some(status) = extract_status_code(body) {
        let phrase = match status {
            200 => "200 OK",
            201 => "201 Created",
            204 => "204 No Content",
            400 => "400 Bad Request",
            401 => "401 Unauthorized",
            403 => "403 Forbidden",
            404 => "404 Not Found",
            409 => "409 Conflict",
            422 => "422 Unprocessable Entity",
            500 => "500 Internal Server Error",
            _ => "200 OK",
        };
        return (phrase, "application/json");
    }
    // Default: plain text if body doesn't look like JSON.
    if body.first().copied() == Some(b'{') || body.first().copied() == Some(b'[') {
        ("200 OK", "application/json")
    } else {
        ("200 OK", "text/plain; charset=utf-8")
    }
}

fn extract_status_code(body: &[u8]) -> Option<u16> {
    let s = std::str::from_utf8(body).ok()?;
    // Look for `"__status":NNN`
    let marker = "\"__status\":";
    let start = s.find(marker)? + marker.len();
    let rest = s[start..].trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

fn ffi_string(bytes: &[u8]) -> CString {
    CString::new(bytes).unwrap_or_else(|_| {
        CString::new(
            bytes
                .iter()
                .copied()
                .filter(|byte| *byte != 0)
                .collect::<Vec<_>>(),
        )
        .unwrap()
    })
}

extern "C" {
    fn malloc(size: usize) -> *mut std::ffi::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn System_IO_File_ReadAllText(path: *const c_char) -> *mut c_char {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = CStr::from_ptr(path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let content = match std::fs::read_to_string(path_str) {
        Ok(s) => s,
        Err(_) => String::new(),
    };
    let bytes = content.as_bytes();
    let len = bytes.len();
    let ptr = malloc(len + 1) as *mut c_char;
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, ptr, len);
    *ptr.add(len) = 0;
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn System_IO_File_WriteAllText(path: *const c_char, contents: *const c_char) {
    if path.is_null() {
        return;
    }
    let c_str = CStr::from_ptr(path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return,
    };
    let content_str = if contents.is_null() {
        ""
    } else {
        match CStr::from_ptr(contents).to_str() {
            Ok(s) => s,
            Err(_) => return,
        }
    };
    let _ = std::fs::write(path_str, content_str);
}

#[no_mangle]
pub unsafe extern "C" fn System_Console_WriteLine_String(value: *const c_char) {
    if value.is_null() {
        println!("null");
    } else {
        let s = CStr::from_ptr(value);
        println!("{}", s.to_string_lossy());
    }
}

#[no_mangle]
pub extern "C" fn System_Console_WriteLine_I64(value: i64) {
    println!("{value}");
}

#[no_mangle]
pub extern "C" fn System_Console_WriteLine_Double(value: f64) {
    println!("{value}");
}

#[no_mangle]
pub extern "C" fn System_Console_WriteLine_Bool(value: bool) {
    println!("{value}");
}
