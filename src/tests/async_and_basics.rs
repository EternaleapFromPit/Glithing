use super::*;

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
    assert!(llvm_ir.contains("glitch_task_from_result_ptr"));
    assert!(llvm_ir.contains("glitch_task_get_result_ptr"));
    assert!(llvm_ir.contains("glitch_task_from_result_double"));
    assert!(llvm_ir.contains("glitch_task_get_result_double"));
    assert!(llvm_ir.contains("glitch_string_retain"));
    assert!(llvm_ir.contains("glitch_string_release"));
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
    assert!(llvm_ir.contains("glitch_task_from_result_i32"));
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

    assert!(llvm_ir.contains("glitch_task_from_result_ptr"));
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

    assert!(llvm_ir.contains("glitch_task_from_result_i32"));
    assert!(llvm_ir.contains("glitch_task_get_result_i32"));
    assert!(llvm_ir.contains("glitch_task_get_result_ptr"));
    assert!(llvm_ir.contains("glitch_string_retain"));
    assert!(llvm_ir.contains("glitch_string_release"));
    assert!(bytecode.contains("  await"));
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
fn runs_llvm_simple_example_natively() {
    let output_exe =
        emit_native_executable_from_source("llvm-simple", include_str!("../../examples/llvm_simple.gl"));
    let output = run_native_executable(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("42"));
}

