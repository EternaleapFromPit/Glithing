use super::*;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::time::Duration;

fn free_tcp_port() -> u16 {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("should bind a local port");
    let port = listener
        .local_addr()
        .expect("listener should expose local addr")
        .port();
    drop(listener);
    port
}

fn wait_for_port(port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        match TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
            Ok(_) => return,
            Err(_) if std::time::Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(25))
            }
            Err(error) => panic!("host did not accept connections on {addr}: {error}"),
        }
    }
}

fn http_get(port: u16, path: &str) -> String {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let mut stream =
        TcpStream::connect_timeout(&addr, Duration::from_secs(5)).expect("should connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("should set read timeout");
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .expect("should set write timeout");
    let request = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
    stream
        .write_all(request.as_bytes())
        .expect("should write request");
    let mut response = String::new();
    let mut chunk = [0_u8; 4096];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => response.push_str(&String::from_utf8_lossy(&chunk[..read])),
            Err(error) if error.kind() == std::io::ErrorKind::ConnectionReset => break,
            Err(error) => panic!("should read response: {error}"),
        }
    }
    response
}

#[test]
fn compiles_csharp_control_flow_smoke() {
    let source = r#"
            int ClampSmall(int value) {
                if (value > 10) {
                    return 10;
                } else {
                    return value;
                }
            }

            fn main() {
                int total = 0;
                for (int i = 0; i < 4; i = i + 1) {
                    total = total + ClampSmall(i);
                }
                while (total < 20) {
                    total = total + 1;
                }
                print(total);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("control flow should lower to LLVM");

    assert!(llvm_ir.contains("define i32 @ClampSmall(i32 %value)"));
    assert!(llvm_ir.contains("br i1"));
    assert!(llvm_ir.contains("define i32 @main()"));
}

#[test]
fn compiles_task_generic_smoke() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 42;
            }

            string LoadName() {
                return "Ada";
            }

                double LoadRatio() {
                    return 1.5;
                }

                fn main() {
                    Task<int> numberTask = Task.Run(Compute);
                    int value = numberTask.Result;
                    print(value);

                    Task<string> nameTask = Task.Run(LoadName);
                    string name = nameTask.GetResult();
                    print(name);

                    Task<double> ratioTask = Task.Run(LoadRatio);
                    double ratio = ratioTask.GetAwaiter().GetResult();
                    print(ratio);

                    Task<double> fromRatio = Task.FromResult(2.5);
                    print(fromRatio.Result);
                }
            "#;

    let llvm_ir = compile_llvm_ir(source).expect("Task<T> should lower to LLVM");

    assert!(llvm_ir.contains("glitch_delegate_wrapper_Compute"));
    assert!(llvm_ir.contains("glitch_task_from_result_i32"));
    assert!(llvm_ir.contains("glitch_task_get_result_i32"));
    assert!(llvm_ir.contains("glitch_task_run_bool"));
    assert!(llvm_ir.contains("glitch_task_from_result_bool"));
    assert!(llvm_ir.contains("glitch_task_get_result_bool"));
    assert!(llvm_ir.contains("glitch_task_from_result_ptr"));
    assert!(llvm_ir.contains("glitch_task_get_result_ptr"));
    assert!(llvm_ir.contains("glitch_task_from_result_double"));
    assert!(llvm_ir.contains("glitch_task_get_result_double"));
    assert!(llvm_ir.contains("glitch_string_retain"));
    assert!(llvm_ir.contains("glitch_string_release"));
}

#[test]
fn runs_task_generic_example_natively() {
    let output_exe = emit_native_executable_from_path("task-generic-example", "examples/task_generic.cs");
    let output = run_native_executable(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("42"));
    assert!(stdout.contains("Ada"));
    assert!(stdout.contains("true"));
}

#[test]
fn records_generic_package_method_instantiations_from_call_sites() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 42;
            }

            fn main() {
                Task.Run(Compute);
            }
        "#;

    let summary = compile_ownership_summary(source)
        .expect("generic package method instantiation summary should compile");

    assert!(summary.contains("generic"));
    assert!(summary.contains("Task"));
}

#[test]
fn specializes_generic_package_methods_into_concrete_llvm_bodies() {
    let source = r#"
            class GenericOps {
                public static T Echo<T>(T value) {
                    return value;
                }
            }

            fn main() {
                int number = GenericOps.Echo(42);
                string name = GenericOps.Echo("Ada");
                print(number);
                print(name);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("generic package method should specialize");

    assert!(llvm_ir.contains("__g1__int"));
    assert!(llvm_ir.contains("__g1__string"));
    assert!(llvm_ir.contains("GenericOps__g0__t0_Echo__g1__int"));
}

#[test]
fn specializes_transitive_generic_package_methods_into_concrete_llvm_bodies() {
    let source = r#"
            class GenericOps {
                public static T Inner<T>(T value) {
                    return value;
                }

                public static T Outer<T>(T value) {
                    return GenericOps.Inner(value);
                }
            }

            fn main() {
                int number = GenericOps.Outer(42);
                print(number);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("transitive generic package methods should specialize");

    assert!(llvm_ir.contains("GenericOps__g0__t0_Outer__g1__int"));
    assert!(llvm_ir.contains("GenericOps__g0__t0_Inner__g1__int"));
}

#[test]
fn infers_generic_integer_literals_with_csharp_default_numeric_types() {
    let source = r#"
            class GenericOps {
                public static T Echo<T>(T value) {
                    return value;
                }
            }

            fn main() {
                int small = GenericOps.Echo(42);
                long big = GenericOps.Echo(3000000000);
                print(small);
                print(big);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("generic integer literal inference should lower to LLVM");

    assert!(llvm_ir.contains("GenericOps__g0__t0_Echo__g1__int"));
    assert!(llvm_ir.contains("GenericOps__g0__t0_Echo__g1__long"));
}

#[test]
fn specializes_generic_methods_on_concrete_generic_owner_types() {
    let source = r#"
            class Box<T> {
                public U Echo<U>(U value) {
                    return value;
                }
            }

            fn main() {
                Box<int> box = new Box<int>();
                string name = box.Echo("Ada");
                print(name);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("generic methods on concrete generic owners should specialize");

    assert!(llvm_ir.contains("Box__g1__t0_Echo__g1__owner_int__string"));
    assert!(llvm_ir.contains("call ptr @Box__g1__t0_Echo__g1__owner_int__string"));
}

#[test]
fn specializes_transitive_generic_methods_on_concrete_generic_owner_types() {
    let source = r#"
            class Box<T> {
                public U Inner<U>(U value) {
                    return value;
                }

                public U Outer<U>(U value) {
                    return this.Inner(value);
                }
            }

            fn main() {
                Box<int> box = new Box<int>();
                string name = box.Outer("Ada");
                print(name);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("transitive owner generic methods should specialize");

    assert!(llvm_ir.contains("Box__g1__t0_Outer__g1__owner_int__string"));
    assert!(llvm_ir.contains("Box__g1__t0_Inner__g1__owner_int__string"));
}

#[test]
fn compiles_valuetask_from_result_like_task() {
    let source = r#"
            using System.Threading.Tasks;

            fn main() {
                ValueTask<int> number = ValueTask.FromResult(7);
                print(number.Result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("ValueTask.FromResult should lower to LLVM");

    assert!(llvm_ir.contains("glitch_task_from_result_i32"));
    assert!(llvm_ir.contains("glitch_task_get_result_i32"));
}

#[test]
fn compiles_valuetask_as_task_surface() {
    let source = r#"
            using System.Threading.Tasks;

            fn main() {
                ValueTask<int> number = ValueTask.FromResult(7);
                Task<int> task = number.AsTask();
                print(task.Result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("ValueTask.AsTask should lower to LLVM");
    assert!(llvm_ir.contains("glitch_task_get_result_i32"));
}

#[test]
fn parses_delegate_declarations_in_framework_packages() {
    let source = r#"
            delegate bool Predicate<T>(T item);

            fn main() {
                print(true);
            }
        "#;

    compile_llvm_ir(source).expect("delegate declaration should parse and lower to LLVM");
}

#[test]
fn lowers_task_run_delegate_invocation_in_llvm() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 42;
            }

            fn main() {
                Task<int> numberTask = Task.Run(Compute);
                int value = numberTask.Result;
                print(value);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Task.Run should lower to LLVM IR");

    assert!(llvm_ir.contains("glitch_delegate_wrapper_Compute"));
    assert!(llvm_ir.contains("glitch_task_run_i32"));
}

#[test]
fn tasks_retain_and_release_string_results_in_llvm() {
    let source = r#"
            using System.Threading.Tasks;

            string LoadName() {
                return "Ada";
            }

            fn main() {
                Task<string> nameTask = Task.Run(LoadName);
                string name = nameTask.Result;
                print(name);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Task<string> should lower to LLVM IR");

    assert!(llvm_ir.contains("glitch_task_run_ptr"));
    assert!(llvm_ir.contains("glitch_task_get_result_ptr"));
    assert!(llvm_ir.contains("glitch_string_retain"));
    assert!(llvm_ir.contains("glitch_string_release"));
}

#[test]
fn compiles_async_await_task_lowering() {
    let source = r#"
            using System.Threading.Tasks;

            async Task<int> LoadNumber() {
                return 42;
            }

            async Task<string> LoadName() {
                return "Ada";
            }

            fn main() {
                Task<int> numberTask = LoadNumber();
                int value = await numberTask;
                print(value);

                Task<string> nameTask = LoadName();
                string name = await nameTask;
                print(name);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("async/await should lower to LLVM");
    let bytecode = compile_bytecode(source).expect("async/await bytecode should compile");

    assert!(llvm_ir.contains("%glitch_async_state_LoadNumber = type { i32, i32 }"));
    assert!(llvm_ir.contains("%glitch_async_state_LoadName = type { i32, ptr }"));
    assert!(llvm_ir.contains("define i32 @glitch_async_resume_LoadNumber(ptr %env)"));
    assert!(llvm_ir.contains("define ptr @glitch_async_resume_LoadName(ptr %env)"));
    assert!(llvm_ir.contains("define ptr @LoadNumber()"));
    assert!(llvm_ir.contains("define ptr @LoadName()"));
    assert!(llvm_ir.contains("glitch_task_get_result_i32"));
    assert!(llvm_ir.contains("glitch_task_get_result_ptr"));
    assert!(llvm_ir.contains("glitch_string_retain"));
    assert!(llvm_ir.contains("glitch_string_release"));
    assert!(bytecode.contains("  await"));
}

#[test]
fn compiles_nested_async_with_two_sequential_awaits() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 41;
            }

            async Task<int> Inner() {
                Task<int> work = Task.Run(Compute);
                return await work;
            }

            async Task<int> Outer() {
                int left = await Inner();
                Task<int> work = Task.Run(Compute);
                int right = await work;
                return left + right;
            }

            fn main() {
                Task<int> value = Outer();
                print(value.Result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("nested async lowering should compile");

    assert!(llvm_ir.contains("%glitch_async_state_Inner = type { i32, i32 }"));
    assert!(llvm_ir.contains("%glitch_async_state_Outer = type { i32, i32 }"));
    assert!(llvm_ir.contains("define i32 @glitch_async_resume_Inner(ptr %env)"));
    assert!(llvm_ir.contains("define i32 @glitch_async_resume_Outer(ptr %env)"));
    assert!(llvm_ir.contains("glitch_task_run_i32"));
    assert!(llvm_ir.contains("glitch_task_get_result_i32"));
}

#[test]
fn rejects_borrowed_local_live_across_await() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 1;
            }

            async Task<int> Broken() {
                int value = 41;
                ref int borrowed = borrow value;
                int next = await Task.Run(Compute);
                return borrowed + next;
            }
        "#;

    let error = compile_llvm_ir(source).expect_err("borrow across await should fail");

    assert!(error.contains("async lowering: 'borrowed' stays Borrowed across await"));
    assert!(error.contains("shorten the borrow before await"));
}

#[test]
fn compiles_await_inside_while_in_current_async_gate() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 1;
            }

            async Task<int> Broken() {
                int total = 0;
                while (total < 2) {
                    total = total + await Task.Run(Compute);
                }
                return total;
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("await in while should compile in the blocking gate");

    assert!(llvm_ir.contains("glitch_async_resume_Broken"));
    assert!(llvm_ir.contains("glitch_task_get_result_i32"));
}

#[test]
fn compiles_await_inside_switch_in_current_async_gate() {
    let source = r#"
            using System.Threading.Tasks;

            int One() {
                return 1;
            }

            async Task<int> ComputeAsync(int choice) {
                switch (choice) {
                    case 0:
                        return await Task.Run(One);
                    default:
                        return await Task.Run(One) + 1;
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("await in switch should compile in the blocking gate");

    assert!(llvm_ir.contains("glitch_async_resume_ComputeAsync"));
    assert!(llvm_ir.contains("glitch_task_get_result_i32"));
}

#[test]
fn runs_async_task_sample_natively() {
    let source = r#"
            using System.Threading.Tasks;

            int StepOne() {
                return 20;
            }

            int StepTwo() {
                return 22;
            }

            async Task<int> ComputeAsync() {
                Task<int> firstTask = Task.Run(StepOne);
                int first = await firstTask;
                Task<int> secondTask = Task.Run(StepTwo);
                int second = await secondTask;
                return first + second;
            }

            fn main() {
                Task<int> value = ComputeAsync();
                print(value.Result);
            }
        "#;

    let output_exe = emit_native_executable_from_source("async-native", source);
    let output = run_native_executable(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("42"));
}

#[test]
fn runs_async_string_result_natively() {
    let source = r#"
            using System.Threading.Tasks;

            string MakeName() {
                return "Ada";
            }

            async Task<string> LoadNameAsync() {
                string name = await Task.Run(MakeName);
                return name;
            }

            fn main() {
                Task<string> value = LoadNameAsync();
                print(value.Result);
            }
        "#;

    let output_exe = emit_native_executable_from_source("async-string-native", source);
    let output = run_native_executable(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Ada"));
}

#[test]
fn compiles_tiny_aspnet_like_supported_subset() {
    let source = include_str!("../../examples/tiny_aspnet_subset.cs");

    let llvm_ir =
        compile_llvm_ir(source).expect("tiny ASP.NET-like supported subset should compile");
    let report = compile_leak_report(source).expect("leak report should compile");

    assert!(llvm_ir.contains("WebApplication_Handle"));
    assert!(llvm_ir.contains("GlitchRestHost_Run"));
    assert!(llvm_ir.contains("glitch_endpoint_handlers_contains"));
    assert!(llvm_ir.contains("glitch_endpoint_handlers_invoke"));
    assert!(llvm_ir.contains("JsonSerializer_SerializeString"));
    assert!(report.contains("No obvious owned temporary leaks detected."));
}

#[test]
fn compiles_aspnet_load_test_example_through_llvm() {
    let source = include_str!("../../examples/aspnet_load_test.cs");

    let llvm_ir = compile_llvm_ir(source)
        .expect("aspnet load test example should compile to LLVM IR");

    assert!(llvm_ir.contains("WebApplication_Handle"));
    assert!(llvm_ir.contains("System_Console_WriteLine_I64"));
    assert!(llvm_ir.contains("GlitchLiveAllocations_Load"));
}

#[test]
fn emits_native_tiny_aspnet_supported_subset_executable() {
    let output_exe =
        emit_native_executable_from_source("tiny-aspnet-subset", include_str!("../../examples/tiny_aspnet_subset.cs"));

    assert!(output_exe.exists());
}

#[test]
fn lowers_async_aspnet_route_handlers_through_task_results() {
    let source = r#"
            using Glitching.AspNetCore;
            using System.Threading.Tasks;

            string BuildHealth() {
                return "{\"status\":\"ok\"}";
            }

            async Task<string> HealthAsync() {
                Task<string> work = Task.Run(BuildHealth);
                return await work;
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.MapGet("/health", HealthAsync);
                app.RunOnce(5111);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("async route handlers should lower to LLVM");

    assert!(llvm_ir.contains("define ptr @glitch_endpoint_handler_0(ptr %app, ptr %path, ptr %body)"));
    assert!(llvm_ir.contains("call ptr @HealthAsync()"));
    assert!(llvm_ir.contains("call ptr @glitch_task_get_result_ptr(ptr %result)"));
}

#[test]
fn runs_llvm_simple_example_natively() {
    let output_exe =
        emit_native_executable_from_source("llvm-simple", include_str!("../../examples/llvm_simple.gl"));
    let output = run_native_executable(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("42"));
}

#[test]
fn runs_async_aspnet_route_natively() {
    let port = free_tcp_port();
    let source = format!(
        r#"
            using Glitching.AspNetCore;
            using System.Threading.Tasks;

            string BuildHealth() {{
                return "{{\"status\":\"ok\"}}";
            }}

            async Task<string> HealthAsync() {{
                Task<string> work = Task.Run(BuildHealth);
                return await work;
            }}

            fn main() {{
                WebApplication app = new WebApplication();
                app.MapGet("/health", HealthAsync);
                app.RunOnce({port});
            }}
        "#
    );

    let output_exe = emit_native_executable_from_source("async-aspnet-route", &source);
    let child = Command::new(&output_exe)
        .env("GLITCH_MAX_REQUESTS", "2")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("native async host should start");

    wait_for_port(port);
    let response = http_get(port, "/health");
    let output = child
        .wait_with_output()
        .expect("native async host should exit cleanly");

    assert!(
        output.status.success(),
        "status={:?}\nresponse={}\nstdout={}\nstderr={}",
        output.status.code(),
        response,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("{\"status\":\"ok\"}"));
}

#[test]
fn runs_async_controller_route_with_di_via_app_handle_natively() {
    let source = r#"
            using System.Threading.Tasks;
            using Glitching.AspNetCore;
            using Microsoft.Extensions.DependencyInjection;

            [ApiController]
            [Route("/hello")]
            class HelloController {
                public ServiceProvider Provider;

                [HttpGet("/")]
                async Task<string> Get() {
                    return this.Provider.GetRequiredService("message");
                }
            }

            fn main() {
                ServiceCollection services = new ServiceCollection();
                services.AddSingleton("message", "hello from di");
                WebApplication app = new WebApplication();
                ServiceProvider routeProvider = services.BuildServiceProvider();
                app.Services = move routeProvider;
                string response = app.Handle("GET", "/hello", "");
                print(response);
            }
        "#;

    let output_exe =
        emit_native_executable_from_source("async-controller-route-di", source);
    let output = run_native_executable(&output_exe);

    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello from di"));
}

#[test]
fn builder_build_preserves_service_provider_for_controller_routes() {
    let source = r#"
            using System.Threading.Tasks;
            using Glitching.AspNetCore;
            using Microsoft.Extensions.DependencyInjection;

            [ApiController]
            [Route("/hello")]
            class HelloController {
                public ServiceProvider Provider;

                [HttpGet("/")]
                async Task<string> Get() {
                    return this.Provider.GetRequiredService("message");
                }
            }

            fn main() {
                WebApplicationBuilder builder = CreateBuilder(System.Array.Empty<string>());
                builder.Services.AddSingleton("message", "hello from builder");
                WebApplication app = builder.Build();
                string response = app.Handle("GET", "/hello", "");
                print(response);
            }
        "#;

    let output_exe =
        emit_native_executable_from_source("builder-controller-route-di", source);
    let output = run_native_executable(&output_exe);

    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello from builder"));
}


