use super::*;

#[test]
fn warns_for_configure_await_async_scheduler_compat_surface() {
    // Use a Task<int> local variable as the receiver for ConfigureAwait,
    // so the receiver type is properly IrType::Task(_) which matches our GL3013 bucket.
    let source = r#"
            using System.Threading.Tasks;

            async Task<int> Compute() {
                return 42;
            }

            async Task DoWork() {
                Task<int> t = Compute();
                int result = await t.ConfigureAwait(false);
                print(result);
            }

            fn main() {
                DoWork().Wait();
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("ConfigureAwait should still compile with a compat warning");
    let diagnostics = output.diagnostics.join("\n");

    // ConfigureAwait on a Task<T> receiver should emit GL3013 (async-scheduler compat).
    assert!(diagnostics.contains("warning GL3013"), "{diagnostics}");
    assert!(!diagnostics.contains("warning GL3001"), "{diagnostics}");
}

#[test]
fn warns_for_cancellation_token_throw_if_cancellation_requested_compat() {
    let source = r#"
            using System.Threading;

            void DoWork(CancellationToken cancellationToken) {
                cancellationToken.ThrowIfCancellationRequested();
            }

            fn main() {
                DoWork(new CancellationToken());
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("ThrowIfCancellationRequested should compile with a compat warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"), "{diagnostics}");
    assert!(!diagnostics.contains("warning GL3001"), "{diagnostics}");
}

#[test]
fn compiles_cancellation_token_source_without_layout_error() {
    let source = r#"
            using System.Threading;

            fn main() {
                CancellationTokenSource cts = new CancellationTokenSource();
                print(cts == null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("CancellationTokenSource construction should not emit GL3004");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("GL3004"), "{diagnostics}");
}

#[test]
fn lowers_specialized_generic_interface_method_dispatch_in_llvm() {
    let source = r#"
            interface IProcessor<T> {
                T Process(T value);
            }

            class IntProcessor : IProcessor<int> {
                public int Process(int value) {
                    return value + 1;
                }
            }

            fn main() {
                IProcessor<int> processor = new IntProcessor();
                int result = processor.Process(41);
                print(result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("generic interface dispatch should lower to LLVM IR");

    assert!(llvm_ir.contains("IntProcessor"));
    assert!(llvm_ir.contains("Process"));
}

#[test]
fn generic_interface_dispatch_on_specialized_owner_emits_drop_glue() {
    let source = r#"
            interface IFactory<T> {
                T Create();
            }

            class BoxFactory : IFactory<string> {
                public string Create() {
                    return "hello";
                }
            }

            fn main() {
                IFactory<string> factory = new BoxFactory();
                string value = factory.Create();
                print(value);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("specialized generic interface dispatch should emit drop glue");

    assert!(llvm_ir.contains("BoxFactory"));
    assert!(llvm_ir.contains("glitch_drop_BoxFactory") || llvm_ir.contains("BoxFactory__g0__t"));
}

#[test]
fn task_delay_emits_compat_warning_not_gl3001() {
    let source = r#"
            using System.Threading.Tasks;

            async Task DoWork() {
                await Task.Delay(100);
            }

            fn main() {
                DoWork().Wait();
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("Task.Delay should compile with a compat warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"), "{diagnostics}");
    assert!(!diagnostics.contains("warning GL3001"), "{diagnostics}");
}

#[test]
fn mediator_send_without_registered_handler_compiles_to_default_task_value() {
    // Previously this hard-errored during LLVM emission with
    // "no IRequestHandler<DemoRequest, _> implementation found".
    // After the fix it should produce a default Task value (no-op) so
    // programs with partially-wired MediatR still compile.
    let source = r#"
            using MediatR;
            using Microsoft.Extensions.DependencyInjection;
            using System.Threading;
            using System.Threading.Tasks;

            class DemoRequest : IRequest<string> {}

            class DemoApp {
                Task<string> Run() {
                    var mediator = new Mediator(new ServiceCollection().BuildServiceProvider());
                    // No IRequestHandler<DemoRequest, string> class is defined here.
                    return mediator.Send(new DemoRequest());
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("mediator Send with no registered handler should still produce LLVM IR");

    // The fallback path must emit a task helper so the caller gets a well-typed Task<string>.
    assert!(
        llvm_ir.contains("glitch_task_from_result_ptr") || llvm_ir.contains("glitch_task_completed"),
        "expected a default-task helper in IR:\n{llvm_ir}"
    );
}

#[test]
fn mediator_send_with_void_response_and_no_handler_emits_completed_task() {
    let source = r#"
            using MediatR;
            using Microsoft.Extensions.DependencyInjection;
            using System.Threading;
            using System.Threading.Tasks;

            class VoidRequest : IRequest {}

            class DemoApp {
                Task Run() {
                    var mediator = new Mediator(new ServiceCollection().BuildServiceProvider());
                    return mediator.Send(new VoidRequest());
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("mediator Send<void> with no handler should emit a CompletedTask fallback");

    assert!(
        llvm_ir.contains("glitch_task_completed"),
        "expected glitch_task_completed in IR:\n{llvm_ir}"
    );
}
