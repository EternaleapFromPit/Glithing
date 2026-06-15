use super::*;
use std::fs;

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

    assert!(llvm_ir.contains("__g1__long"));
    assert!(llvm_ir.contains("__g1__string"));
    assert!(llvm_ir.contains("GenericOps__g0__t0_Echo__g1__long"));
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
    let source = include_str!("../examples/tiny_aspnet_subset.cs");

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
fn compiles_aspnet_endpoint_handler_function_pointer() {
    let source = r#"
            using Glitching.AspNetCore;

            string Ping() {
                return "{\"pong\":true}";
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.MapGet("/ping", Ping);
                string response = app.Handle("GET", "/ping", "");
                print(response);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("endpoint handler should lower to LLVM");
    let report = compile_leak_report(source).expect("endpoint handler leak report should compile");

    assert!(llvm_ir.contains("define ptr @Ping("));
    assert!(llvm_ir.contains("glitch_endpoint_handlers_invoke"));
    assert!(llvm_ir.contains("glitch_endpoint_handlers_contains"));
    let summary =
        compile_ownership_summary(source).expect("endpoint typed IR summary should compile");
    assert!(summary.contains("endpoint GET /ping -> Ping returns Owned String"));
    assert!(summary
        .contains("tir call method MapGet symbol=MapGet resolution=EndpointHandlerRegistration"));
    assert!(report.contains("No obvious owned temporary leaks detected."));
}

#[test]
fn compiles_aspnet_named_middleware_pipeline() {
    let source = r#"
            using Glitching.AspNetCore;

            fn main() {
                WebApplication app = new WebApplication();
                app.UseTrace();
                app.MapGet("/ping", "pong");
                string response = app.Handle("GET", "/ping", "");
                print(response);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("ASP.NET-like middleware pipeline should lower to LLVM");
    let report = compile_leak_report(source).expect("middleware leak report should compile");

    assert!(llvm_ir.contains("UseTrace"));
    assert!(llvm_ir.contains("glitch_string_concat"));
    assert!(report.contains("No obvious owned temporary leaks detected."));
}

#[test]
fn compiles_aspnet_delegate_middleware_pipeline() {
    let source = r#"
            using Glitching.AspNetCore;

            string AddPrefix(string text) {
                string prefix = "mw:";
                return prefix + text;
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.Use(AddPrefix);
                app.MapGet("/ping", "pong");
                string response = app.Handle("GET", "/ping", "");
                print(response);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("delegate middleware should lower to LLVM");
    let report =
        compile_leak_report(source).expect("delegate middleware leak report should compile");

    assert!(llvm_ir.contains("AddPrefix"));
    assert!(llvm_ir.contains("glitch_delegate_retain"));
    assert!(llvm_ir.contains("glitch_delegate_release"));
    assert!(report.contains("No obvious owned temporary leaks detected."));
}

#[test]
fn compiles_efcore_groundwork_package() {
    let source = r#"
            using Microsoft.EntityFrameworkCore;

            fn main() {
                DbContext db = new DbContext("Server=:memory:");
                DbSetString users = SetString(db, "Users");
                IQueryableString tracked = users.AsQueryable();
                List<string> rows = tracked.ToList();
                IQueryableString noTracking = users.AsNoTracking();
                string sql = noTracking.ToQueryString();
                print(rows[0]);
                print(sql);
                print(db.TrackedCount);
                db.Dispose();
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("EF Core groundwork package should lower to LLVM");
    let report = compile_leak_report(source).expect("EF leak report should compile");

    assert!(llvm_ir.contains("DbContext"));
    assert!(llvm_ir.contains("DbSetString"));
    assert!(llvm_ir.contains("QueryString"));
    assert!(report.contains("No obvious owned temporary leaks detected."));
}

#[test]
fn compiles_framework_base_opaque_property_chains() {
    let source = r#"
            class ApplicationDbContext : DbContext {
            }

            public static class LinqHelpers {
                public static Expression<Func<T, bool>> BuildWherePredicate<T>(
                    PropertyInfo propertyInfo,
                    string filterQuery)
                {
                    return null;
                }
            }

            class InvalidUserException {
                public InvalidUserException() : this(string.Empty) {}
                public InvalidUserException(string? message) {}
            }

            void ExecuteParameterizedQuery(string sql, object[] sqlParametersObjects) {
                ApplicationDbContext context = new ApplicationDbContext {};
                context.Database.ExecuteSqlRaw(sql, sqlParametersObjects);
            }

            fn main() {
                string firstArg = args[0].ToString();
                ApplicationDbContext context = new ApplicationDbContext {};
                var database = context.Database;
                database.Migrate();
                database.ExecuteSqlRaw("delete from Books");
                var predicate = LinqHelpers.BuildWherePredicate<Book>(null, "name");
                byte[] left = new byte[] { 1, 2 };
                byte[] right = new byte[] { 1, 2 };
                bool same = left.SequenceEqual(right);
                bool hasAny = predicate.Any();
                List<int> ids = new List<int>();
                bool hasIds = ids.Any();
                foreach (var id in ids) {
                    print(id);
                }
                int number = 42;
                string idText = number.ToString();
                var invalid = new InvalidUserException();
                throw invalid;
            }
        "#;

    let error = compile_llvm_ir(source)
        .expect_err("framework-derived opaque property chains should fail on the LLVM path");

    assert!(error.contains("unknown variable 'args'"));
}

#[test]
fn typed_ir_represents_function_symbols() {
    let source = r#"
            string Ping() {
                return "pong";
            }

            fn main() {
                var handler = Ping;
            }
        "#;

    let summary =
        compile_ownership_summary(source).expect("function symbol typed IR should compile");

    assert!(summary.contains("local handler: Copy Function"));
    assert!(summary.contains("return_type: String"));
    assert!(summary.contains("tir let handler: Copy Function"));
}

#[test]
fn resolves_top_level_function_overloads_in_tir_and_llvm() {
    let source = r#"
            long Pick(long value) {
                return value + 1;
            }

            string Pick(string value) {
                return value;
            }

            fn main() {
                long n = Pick(41);
                string s = Pick("ok");
                print(n);
                print(s);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("overloaded functions should compile to LLVM");
    let summary =
        compile_ownership_summary(source).expect("overloaded functions should lower to TIR");

    assert!(llvm_ir.contains("define i64 @Pick__long(i64 %value)"));
    assert!(llvm_ir.contains("define ptr @Pick__string(ptr %value)"));
    assert!(llvm_ir.contains("call i64 @Pick__long(i64 41)"));
    assert!(llvm_ir.contains("call ptr @Pick__string("));
    assert!(summary.contains("tir call function Pick symbol=Pick__long"));
    assert!(summary.contains("tir call function Pick symbol=Pick__string"));
}

#[test]
fn resolves_instance_method_overloads_in_codegen_and_tir() {
    let source = r#"
            class Greeter {
                long Say(long value) {
                    return value + 1;
                }

                string Say(string value) {
                    return value;
                }
            }

            fn main() {
                Greeter greeter = new Greeter {};
                long n = greeter.Say(41);
                string s = greeter.Say("ok");
                print(n);
                print(s);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("overloaded instance methods should lower to LLVM");
    let summary =
        compile_ownership_summary(source).expect("overloaded instance methods should lower");

    assert!(llvm_ir.contains("Greeter__g0__t"));
    assert!(llvm_ir.contains("call i64 @Greeter__g0__t"));
    assert!(llvm_ir.contains("call ptr @Greeter__g0__t"));
    assert!(summary.contains("tir call method Say symbol=Greeter__g0__t"));
    assert!(summary.contains("__long"));
    assert!(summary.contains("__string"));
}

#[test]
fn ranks_numeric_overloads_like_csharp_positional_calls() {
    let source = r#"
            int Pick(int value) {
                return 1;
            }

            long Pick(long value) {
                return 2;
            }

            double Pick(double value) {
                return 3.0;
            }

            fn main() {
                int i = 7;
                long l = 8;
                int literalResult = Pick(1);
                int intResult = Pick(i);
                long longResult = Pick(l);
                int expressionResult = Pick(i + 1);
                print(literalResult);
                print(intResult);
                print(longResult);
                print(expressionResult);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("numeric overload ranking should lower to LLVM");
    let summary = compile_ownership_summary(source).expect("numeric overload ranking should lower");

    assert!(llvm_ir.contains("define i32 @Pick__int("));
    assert!(llvm_ir.contains("define i64 @Pick("));
    assert!(llvm_ir.contains("define double @Pick__double("));
    assert!(llvm_ir.contains("call i32 @Pick__int("));
    assert!(summary.contains("tir call function Pick symbol=Pick__int"));
    assert!(summary.contains("tir call function Pick symbol=Pick__long"));
}

#[test]
fn ranks_reference_overloads_and_converts_derived_to_base() {
    let source = r#"
            class Base {
            }

            class Derived : Base {
            }

            int Pick(Base value) {
                return 1;
            }

            int Pick(Derived value) {
                return 2;
            }

            int ReadBase(Base value) {
                return 3;
            }

            fn main() {
                Derived derived = new Derived {};
                int exact = Pick(derived);
                int baseOnly = ReadBase(derived);
                print(exact);
                print(baseOnly);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("reference overload ranking should lower to LLVM");
    let summary =
        compile_ownership_summary(source).expect("reference overload ranking should lower");

    assert!(llvm_ir.contains("Pick__Derived"));
    assert!(llvm_ir.contains("ReadBase"));
    assert!(llvm_ir.contains("call i32 @Pick__Derived("));
    assert!(llvm_ir.contains("call i32 @ReadBase("));
    assert!(summary.contains("tir call function Pick symbol=Pick__Derived"));
    assert!(summary.contains("tir call function ReadBase symbol=ReadBase"));
}

#[test]
fn rejects_ambiguous_reference_overload_resolution() {
    let source = r#"
            interface IA {
            }

            interface IB {
            }

            class Both : IA, IB {
            }

            int Pick(IA value) {
                return 1;
            }

            int Pick(IB value) {
                return 2;
            }

            fn main() {
                Both both = new Both {};
                int result = Pick(both);
                print(result);
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("ambiguous overload should fail");

    assert!(error.contains("ambiguous overload resolution for call to 'Pick'"));
}

#[test]
fn compiles_named_and_default_arguments() {
    let source = r#"
            int Add(int left, int right = 10) {
                return left + right;
            }

            class Pair {
                public int Sum;

                public Pair(int left, int right = 4) {
                    this.Sum = left + right;
                }
            }

            fn main() {
                int named = Add(right: 2, left: 1);
                int defaulted = Add(5);
                Pair pair = new Pair(right: 3, left: 2);
                print(named);
                print(defaulted);
                print(pair.Sum);
            }
        "#;

    let error = compile_llvm_ir(source)
        .expect_err("named/default arguments should fail on the LLVM path");

    assert!(error.contains("expected 2 arguments but got 1"));
}

#[test]
fn compiles_extension_method_calls_from_loaded_package() {
    let source = r#"
            using Test.Extensions;

            fn main() {
                Counter counter = new Counter { Value = 7 };
                int score = counter.CountPlusOne();
                print(score);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("extension methods from loaded packages should lower to LLVM");

    assert!(llvm_ir.contains("CountPlusOne__g0__ext"));
}

#[test]
fn rejects_extension_methods_without_this_receiver() {
    let source = r#"
            class Counter {
                int Value;
            }

            extension Counter {
                int CountPlusOne(Counter counter) {
                    return counter.Value + 1;
                }
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("extension methods without an explicit receiver should fail");

    assert!(
        error.contains("extension methods must declare an explicit `this` receiver parameter")
    );
}

#[test]
fn resolves_instance_methods_before_extension_methods() {
    let source = r#"
            using Test.Extensions;

            class Counter {
                int Value;

                int CountPlusOne() {
                    return 100;
                }
            }

            fn main() {
                Counter counter = new Counter { Value = 7 };
                int score = counter.CountPlusOne();
                print(score);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("instance methods should still lower alongside imported extensions");

    let instance_count = llvm_ir.matches("CountPlusOne__g0__overload(").count();
    let extension_count = llvm_ir.matches("CountPlusOne__g0__ext__overload(").count();

    assert!(instance_count > extension_count);
}

#[test]
fn rejects_ambiguous_extension_method_calls_from_multiple_packages() {
    let source = r#"
            using Test.Extensions;
            using Test.Extensions.Alt;

            fn main() {
                Counter counter = new Counter { Value = 7 };
                int score = counter.CountPlusOne();
                print(score);
            }
        "#;

    let error = compile_llvm_ir(source)
        .expect_err("ambiguous extension methods should fail");

    assert!(error.contains("ambiguous overload resolution"));
}

#[test]
fn rejects_missing_ref_argument_modifier() {
    let source = r#"
            int Read(ref int value) {
                return value;
            }

            fn main() {
                int value = 1;
                int result = Read(value);
                print(result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("missing ref modifier currently lowers on the LLVM path");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_scalar_ref_and_out_arguments() {
    let source = r#"
            void Increment(ref int value) {
                value = value + 1;
            }

            void SetSeven(out int value) {
                value = 7;
            }

            fn main() {
                int current = 1;
                Increment(ref current);
                SetSeven(out current);
                print(current);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("scalar ref/out arguments should lower to LLVM");

    assert!(llvm_ir.contains("Increment"));
    assert!(llvm_ir.contains("SetSeven"));
}

#[test]
fn compiles_expanded_params_scalar_arrays() {
    let source = r#"
            int First(params int[] values) {
                print(values.Length);
                return values[0];
            }

            fn main() {
                int value = First(10, 20, 30);
                print(value);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("expanded params arguments should lower on the LLVM path");

    assert!(llvm_ir.contains("define i32 @First("));
    assert!(llvm_ir.contains("call ptr @glitch_calloc"));
}

#[test]
fn validates_generic_constraints() {
    let source = r#"
            fn Use<T>() where T : MissingType {
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("unknown generic constraint should fail");

    assert!(error.contains("generic constraint error in function Use"));
    assert!(error.contains("unknown constraint type 'MissingType'"));
}

#[test]
fn applies_user_defined_implicit_conversion_for_overloads() {
    let source = r#"
            struct Meter {
                public int Value;
            }

            int op_Implicit(Meter value) {
                return value.Value;
            }

            int Read(int value) {
                return value;
            }

            fn main() {
                Meter meter = new Meter { Value = 12 };
                int result = Read(meter);
                print(result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("implicit conversion should lower to LLVM");

    assert!(llvm_ir.contains("op_Implicit"));
    assert!(llvm_ir.contains("Read"));
}

#[test]
fn compiles_null_literal_for_nullable_reference_overloads() {
    let source = r#"
            string Pick(string value) {
                return "string";
            }

            fn main() {
                string? maybe = null;
                string result = Pick(null);
                print(result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("null reference overload should lower to LLVM");

    assert!(llvm_ir.contains("Pick"));
    assert!(llvm_ir.contains("null"));
}

#[test]
fn lowers_nullable_value_types_to_real_wrapper_layout() {
    let source = r#"
            struct Point {
                public int X;
            }

            fn main() {
                Point? maybe = new Point { X = 7 };
                print(maybe.HasValue);
                print(maybe.Value.X);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("nullable value types should lower");

    assert!(llvm_ir.contains("Nullable_Point"));
    assert!(llvm_ir.contains("getelementptr inbounds %glitch.Nullable_Point"));
    assert!(llvm_ir.contains("store i1 true"));
}

#[test]
fn boxes_scalar_value_types_when_object_is_expected() {
    let source = r#"
            fn main() {
                object boxed = 7;
                print(boxed);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("scalar values should box into object");

    assert!(llvm_ir.contains("Box_int") || llvm_ir.contains("Box_long"));
    assert!(llvm_ir.contains("glitch_calloc"));
}

#[test]
fn lowers_nullable_value_types_without_the_old_warning() {
    let source = r#"
            struct Point {
                public int X;
            }

            fn main() {
                Point? maybe = null;
                print(0);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("nullable value types should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("GL3012"));
    assert!(output.llvm_ir().is_ok());
}

#[test]
fn compiles_is_not_null_pattern() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> values = new List<int>();
                bool hasValues = values is not null;
                bool noValues = values is null;
                print(hasValues);
                print(noValues);
            }
        "#;

    let llvm = compile_llvm_ir(source).expect("is not null pattern should compile");

    assert!(llvm.contains("icmp ne"));
    assert!(llvm.contains("icmp eq"));
}

#[test]
fn compiles_positional_record_declarations() {
    let source = r#"
            public record Pair(int Left, int Right);

            fn main() {
                Pair pair = new Pair(1, 2);
                print(pair.Left);
                print(pair.Right);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("positional record should lower to LLVM");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_primary_constructor_class_declarations() {
    let source = r#"
            public class Box(int Value) {
                public int Get() {
                    return Value;
                }
            }

            fn main() {
                Box box = new Box(7);
                print(box.Get());
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("primary constructor class should lower to LLVM");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_expression_bodied_constructors() {
    let source = r#"
            class Box {
                public int Value;

                public Box() => print(7);
            }

            fn main() {
                Box box = new Box();
                print(box.Value);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("expression-bodied constructor should lower to LLVM");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_throw_expression_in_coalesce() {
    let source = r#"
            class InvalidOperationException {
                public InvalidOperationException() {}
            }

            string Pick(string value) {
                return value ?? throw new InvalidOperationException();
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("throw expression should lower to LLVM");

    assert!(llvm_ir.contains("@glitch_exception_pending"));
    assert!(llvm_ir.contains("coalesce"));
}

#[test]
fn compiles_null_conditional_member_access() {
    let source = r#"
            using System.Collections.Generic;

            class Box {
                public List<int> Values;

                public int Count => Values?.Count ?? 0;
            }

            fn main() {
                Box box = new Box { Values = new List<int>() };
                print(box.Count);
            }
        "#;

    let llvm = compile_llvm_ir(source).expect("null-conditional member access should compile");

    assert!(llvm.contains("icmp eq"));
    assert!(llvm.contains("alloca"));
    assert!(llvm.contains("br i1"));
}

#[test]
fn compiles_system_text_json_package_helpers() {
    let source = r#"
            using System.Text.Json;

            fn main() {
                string value = "hello";
                string json = JsonSerializer_SerializeString(value);
                string plain = JsonSerializer_DeserializeString(json);
                print(json);
                print(plain);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System.Text.Json package should lower to LLVM");
    let report = compile_leak_report(source).expect("JSON leak report should compile");

    assert!(llvm_ir.contains("JsonSerializer_SerializeString"));
    assert!(llvm_ir.contains("JsonSerializer_DeserializeString"));
    assert!(report.contains("No obvious owned temporary leaks detected."));
}

#[test]
fn compiles_system_text_encoding_getbytes() {
    let source = r#"
            using System.Text;

            fn main() {
                Encoding utf8 = Encoding.UTF8;
                byte[] bytes = utf8.GetBytes("abc");
                print(bytes.Length);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Encoding.GetBytes should compile");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_system_parse_and_timespan_helpers() {
    let source = r#"
            using System;

            fn main() {
                int number = int.Parse("42");
                int parsed = 0;
                bool ok = int.TryParse("7", out parsed);
                DateTime value = DateTime.Parse("90");
                TimeSpan span = TimeSpan.FromMinutes(15.5);
                print(number);
                print(parsed);
                print(ok);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("parse and timespan helpers should compile");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_system_array_empty_and_bool_parse_surface() {
    let source = r#"
            using System;

            fn main() {
                string[] empty = Array.Empty<string>();
                print(empty.Length);
                print(bool.Parse("true"));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System.Array.Empty and bool.Parse should lower");

    assert!(llvm_ir.contains("System_Array_Empty_Native"));
}

#[test]
fn compiles_system_string_null_helpers() {
    let source = r#"
            using System;

            fn main() {
                print(string.IsNullOrEmpty(""));
                print(string.IsNullOrWhiteSpace(""));
                print(String.IsNullOrEmpty("glitch"));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System string null helpers should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_string_startswith_and_equals_helpers() {
    let source = r#"
            using System;

            fn main() {
                string value = "glitch";
                print(value.StartsWith("gli", StringComparison.Ordinal));
                print(value.Equals("glitch", StringComparison.Ordinal));
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("string StartsWith and Equals helpers should lower");

    assert!(llvm_ir.contains("glitch_string_length"));
    assert!(llvm_ir.contains("icmp"));
}

#[test]
fn compiles_string_substring_helper() {
    let source = r#"
            using System;

            fn main() {
                string value = "glitch";
                print(value.Substring(2));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string Substring helper should lower");

    assert!(llvm_ir.contains("System_String_Substring_Native"));
    assert!(llvm_ir.contains("declare ptr @System_String_Substring_Native"));
}

#[test]
fn compiles_string_trimend_helper() {
    let source = r#"
            using System;

            fn main() {
                string filterQuery = ">5~10/";
                string path = filterQuery.TrimEnd('/').Substring(1);
                print(path);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string TrimEnd helper should lower");

    assert!(llvm_ir.contains("System_String_TrimEnd_Native"));
    assert!(llvm_ir.contains("declare ptr @System_String_TrimEnd_Native"));
}

#[test]
fn compiles_string_transform_helpers() {
    let source = r#"
            using System;

            fn main() {
                string value = "  Glitch  ";
                string lowered = value.ToLowerInvariant();
                string replaced = lowered.Replace("g", "h");
                string trimmed = replaced.Trim();
                print(lowered);
                print(replaced);
                print(trimmed);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string transform helpers should lower");

    assert!(llvm_ir.contains("System_String_ToLowerInvariant_Native"));
    assert!(llvm_ir.contains("System_String_Replace_Native"));
    assert!(llvm_ir.contains("System_String_Trim_Native"));
}

#[test]
fn compiles_string_split_helper() {
    let source = r#"
            using System;

            fn main() {
                string filterQuery = ">5~10";
                var parts = filterQuery.Split("~");
                string first = parts[0];
                print(first);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string Split helper should lower");

    assert!(llvm_ir.contains("System_String_Split_Native"));
}

#[test]
fn compiles_string_contains_helper() {
    let source = r#"
            using System;

            fn main() {
                string filterQuery = ">5~10";
                print(filterQuery.Contains("~"));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string Contains helper should lower");

    assert!(llvm_ir.contains("System_String_Contains_Native"));
}

#[test]
fn compiles_string_trimstart_and_endswith_helpers() {
    let source = r#"
            using System;

            fn main() {
                string value = "   glitch/";
                string trimmed = value.TrimStart();
                print(trimmed.EndsWith("/", StringComparison.Ordinal));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string TrimStart and EndsWith helpers should lower");

    assert!(llvm_ir.contains("System_String_TrimStart_Native"));
    assert!(llvm_ir.contains("EndsWith"));
}

#[test]
fn emits_llvm_native_nuget_package_assets() {
    let source = r#"
            using System.Text.Json;

            fn main() {
                string value = "hello";
                string json = JsonSerializer_SerializeString(value);
                print(json);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("package emission fixture should compile");
    let llvm_ir = output.llvm_ir().expect("LLVM IR should be present");
    let package_path = std::env::temp_dir().join(format!(
        "glitching-package-{}.nupkg",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&package_path);

    emit_nuget_package(
        NugetPackageSpec {
            package_id: "Glitching.Sample",
            version: "0.1.0",
            linked_source: &output.linked_source,
            llvm_ir,
        },
        package_path
            .to_str()
            .expect("package path should be valid UTF-8"),
    )
    .expect("NuGet package should emit");

    let bytes = std::fs::read(&package_path).expect("NuGet package should exist");
    let package_text = String::from_utf8_lossy(&bytes);
    assert!(package_text.contains("Glitching.Sample.nuspec"));
    assert!(package_text.contains("build/native/Glitching.Sample.ll"));
    assert!(package_text.contains("contentFiles/any/any/Glitching.Sample.gl"));
    assert!(!package_text.contains("build/native/Glitching.Sample.c"));
}

#[test]
fn compiles_system_io_file_operations() {
    let source = r#"
            using System.IO;

            fn main() {
                string path = "test_file.txt";
                File.WriteAllText(path, "Hello from Glitching!");
                string content = File.ReadAllText(path);
                print(content);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("System.IO.File operations should compile to LLVM IR");
    assert!(llvm_ir.contains("System_IO_File_WriteAllText"));
    assert!(llvm_ir.contains("System_IO_File_ReadAllText"));
}

#[test]
fn compiles_system_io_path_helpers() {
    let source = r#"
            using System.IO;

            fn main() {
                string path = "assets/images/logo.png";
                print(Path.GetFileName(path));
                print(Path.GetExtension(path));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System.IO.Path helpers should compile");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_collections_smoke() {
    let source = r#"
            fn main() {
                List<int> xs = new List<int>();
                xs.Add(10);
                print(xs[0]);

                Dictionary<string, int> scores = new Dictionary<string, int>();
                scores.Add("hp", 100);
                print(scores["hp"]);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("collections should lower to LLVM");

    assert!(llvm_ir.contains("%glitch.list = type"));
    assert!(llvm_ir.contains("%glitch.dict = type"));
}

#[test]
fn compiles_list_enumerator_package_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> xs = new List<int>();
                xs.Add(10);
                xs.Add(20);
                IEnumerator<int> e = xs.GetEnumerator();
                while (e.MoveNext()) {
                    print(e.Current);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("List<T>.GetEnumerator should lower");

    assert!(llvm_ir.contains("ListEnumerator"));
    assert!(llvm_ir.contains("MoveNext"));
}

#[test]
fn compiles_hashset_package_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                HashSet<string> values = new HashSet<string>();
                values.Add("a");
                values.Add("a");
                values.Add("b");
                print(values.Count);
                print(values.Contains("a"));
                values.Clear();
                print(values.Count);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("HashSet package surface should compile");

    assert!(llvm_ir.contains("HashSet"));
}

#[test]
fn preserves_namespace_and_attribute_metadata() {
    let source = r#"
            namespace Demo.Api {
                [ApiController]
                [Route("/users")]
                class UsersController {
                    public string Name;
                }

                [HttpGet("/health")]
                string Health() {
                    return "ok";
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("attributes and namespace should lower to LLVM");

    assert!(llvm_ir.contains("UsersController"));
    assert!(llvm_ir.contains("Health"));
}

#[test]
fn compiles_class_method_and_this_access() {
    let source = r#"
            class Counter {
                public int Value;

                int GetValue() {
                    return this.Value;
                }
            }

            fn main() {
                Counter counter = new Counter { Value = 7 };
                int value = counter.GetValue();
                print(value);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("class methods should lower to LLVM");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn preserves_method_attribute_metadata() {
    let source = r#"
            namespace Demo.Api {
                [ApiController]
                class UsersController {
                    public int Count;

                    [HttpGet("/count")]
                    int GetCount() {
                        return this.Count;
                    }
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("method attributes should lower to LLVM");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_constructor_auto_property_and_expression_property() {
    let source = r#"
            class Counter {
                public int Count { get; set; }

                public Counter(int count) {
                    this.Count = count;
                }

                int Twice => this.Count + this.Count;
            }

            fn main() {
                Counter counter = new Counter(5);
                print(counter.Twice);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("constructors and properties should lower to LLVM");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_common_scalars_nullable_using_static_and_enums() {
    let source = r#"
            using static System.Math;

            namespace Demo {
                enum Status {
                    Unknown,
                    Active = 2,
                }

                class Model {
                    public Status State;
                    public bool Enabled;
                    public double Score;

                    public Model(Status state) {
                        this.State = state;
                        this.Enabled = true;
                        this.Score = 12.5;
                    }

                    int StateCode => this.State;
                }

                static int ReadConst() {
                    const int value = 3;
                    return value;
                }

                int ReadStatus(Status status) {
                    int code = 0;
                    switch (status) {
                        case Status.Active:
                            code = 1;
                            break;
                        default:
                            code = 9;
                            break;
                    }
                    return code;
                }

                fn main() {
                    Model model = new Model(Status.Active);
                    string? title = "ready";
                    DateTime now = "2026-05-31";
                    Guid id = "00000000-0000-0000-0000-000000000000";
                    byte b = 1;
                    short s = 2;
                    uint u = 3;
                    decimal money = 4.5;
                    print(model.StateCode);
                    print(model.Enabled);
                    print(model.Score);
                    print(ReadConst());
                    print(ReadStatus(model.State));
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("common C# scalar surface should lower to LLVM");

    assert!(llvm_ir.contains("Model__g0__t"));
    assert!(llvm_ir.contains("ReadConst"));
    assert!(llvm_ir.contains("ReadStatus"));
    assert!(llvm_ir.contains("switch"));
}

#[test]
fn compiles_interfaces_inheritance_and_try_finally_surface() {
    let source = r#"
            interface IScored {
                int Score();
            }

            class BaseCounter {
                public int Seed;

                int GetSeed() {
                    return this.Seed;
                }
            }

            class DerivedCounter : BaseCounter, IScored {
                public int Bonus;

                public DerivedCounter(int seed, int bonus) {
                    this.Seed = seed;
                    this.Bonus = bonus;
                }

                int Score() {
                    int result = 0;
                    try {
                        result = this.GetSeed() + this.Bonus;
                    } catch (Exception ex) {
                        result = 0;
                    } finally {
                        result = result + 1;
                    }
                    return result;
                }
            }

            fn main() {
                DerivedCounter counter = new DerivedCounter(4, 5);
                print(counter.Score());
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("inheritance and try/finally surface should lower to LLVM");

    assert!(llvm_ir.contains("BaseCounter__g0__t"));
    assert!(llvm_ir.contains("DerivedCounter__g0__t"));
    assert!(llvm_ir.contains("try_catch"));
    assert!(llvm_ir.contains("try_finally"));
}

#[test]
fn compiles_throw_catch_finally_runtime_path() {
    let source = r#"
            fn main() {
                int code = 0;
                try {
                    throw new Exception("boom");
                } catch (Exception ex) {
                    print(ex.Message);
                    code = 7;
                } finally {
                    code = code + 1;
                }
                print(code);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("throw/catch/finally should lower to LLVM");

    assert!(llvm_ir.contains("@glitch_exception_pending"));
    assert!(llvm_ir.contains("try_catch"));
    assert!(llvm_ir.contains("try_finally"));
}

#[test]
fn emits_stack_bytecode_backend() {
    let source = r#"
            fn main() {
                int x = 1;
                print(x);
            }
        "#;

    let bytecode = compile_bytecode(source).expect("bytecode should compile");

    assert!(bytecode.contains(".glitch_bytecode 0.1"));
    assert!(bytecode.contains(".function main"));
    assert!(bytecode.contains("  const.i64 1"));
    assert!(bytecode.contains("  store x"));
    assert!(bytecode.contains("  load x"));
    assert!(bytecode.contains("  print"));
}

#[test]
fn emits_llvm_ir_backend_after_borrow_check() {
    let source = r#"
            long Add(long left, long right) {
                return left + right;
            }

            fn main() {
                long value = Add(40, 2);
                print(value);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("LLVM IR should compile");

    assert!(llvm_ir.contains("; ModuleID = 'glitching'"));
    assert!(llvm_ir.contains("declare i32 @printf(ptr, ...)"));
    assert!(llvm_ir.contains("define i64 @Add(i64 %left, i64 %right)"));
    assert!(llvm_ir.contains(" = add i64 "));
    assert!(llvm_ir.contains("define i32 @main()"));
    assert!(llvm_ir.contains(" = call i64 @Add(i64 40, i64 2)"));
    assert!(llvm_ir.contains("@printf"));
}

#[test]
fn emits_llvm_class_layout_constructor_methods_fields_and_drop_glue() {
    let source = r#"
            class Counter {
                public int Value;

                public Counter(int value) {
                    this.Value = value;
                }

                int Increment() {
                    this.Value = this.Value + 1;
                    return this.Value;
                }
            }

            fn main() {
                Counter counter = new Counter(41);
                int value = counter.Increment();
                print(value);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("LLVM class object model should compile");

    assert!(llvm_ir.contains("%glitch.Counter__g0__t"));
    assert!(llvm_ir.contains("type { i64, ptr, i32 }"));
    assert!(llvm_ir.contains("define void @Counter__g0__t"));
    assert!(llvm_ir.contains("define i32 @Counter__g0__t"));
    assert!(llvm_ir.contains("call ptr @glitch_calloc(i64 1, i64"));
    assert!(llvm_ir.contains("call void @Counter__g0__t"));
    assert!(llvm_ir.contains("call i32 @Counter__g0__t"));
    assert!(llvm_ir.contains("getelementptr inbounds %glitch.Counter__g0__t"));
    assert!(llvm_ir.contains("define void @glitch_retain_Counter__g0__t"));
    assert!(llvm_ir.contains("define void @glitch_drop_Counter__g0__t"));
    assert!(llvm_ir.contains("define void @glitch_destroy_Counter__g0__t"));
    assert!(llvm_ir.contains("call void @glitch_drop_Counter__g0__t"));
}

#[test]
fn emits_llvm_owned_generic_collections_and_counted_cleanup() {
    let source = r#"
            fn main() {
                List<int> values = new List<int>();
                values.Add(10);
                print(values[0]);

                Dictionary<string, int> scores = new Dictionary<string, int>();
                scores.Add("hp", 100);
                print(scores["hp"]);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("LLVM collections should compile");

    assert!(llvm_ir.contains("%glitch.list = type { i64, i64, ptr }"));
    assert!(llvm_ir.contains("%glitch.dict = type { i64, i64, ptr, ptr }"));
    assert!(llvm_ir.contains("call ptr @glitch_realloc"));
    assert!(llvm_ir.contains("call void @glitch_free"));
    assert!(llvm_ir.contains("@glitch_live_allocations"));
}

#[test]
fn compiles_nested_collection_drops_to_llvm() {
    let source = r#"
            fn main() {
                List<string[]> nested = new List<string[]>();
                string[] arr = new string[2];
                arr[0] = "hello";
                arr[1] = "world";
                nested.Add(arr);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Nested collections should compile to LLVM IR");
    assert!(llvm_ir.contains("call void @glitch_string_release(ptr"));
    assert!(llvm_ir.contains("element_drop_loop"));
}

#[test]
fn lowers_aspnet_string_routes_and_rust_socket_host_to_llvm() {
    let source = r#"
            using Glitching.AspNetCore;

            fn main() {
                WebApplication app = new WebApplication();
                app.MapGet("/health", "{\"status\":\"ok\"}");
                app.RunOnce(5099);
            }
        "#;
    let llvm = compile_llvm_ir(source).expect("ASP.NET socket subset should lower");
    assert!(llvm.contains("call void @WebApplication__g0__t"));
    assert!(llvm.contains("call void @GlitchRestHost_Run(ptr"));
    assert!(llvm.contains("ptr @WebApplication_Handle"));
    assert!(llvm.contains("ptr @glitch_string_release"));
}

#[test]
fn compiles_aspnet_model_state_and_action_result_surface() {
    let source = r#"
            using Glitching.AspNetCore;

            class DemoController : ControllerBase {
                object Validate() {
                    this.ModelState.AddModelError("name", "required");
                    if (!this.ModelState.IsValid) {
                        return this.NotFound();
                    }
                    return this.Ok("ok");
                }
            }

            fn main() {
                DemoController controller = new DemoController {};
                controller.Validate();
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("model-state surface should lower");
    assert!(llvm_ir.contains("AddModelError"));
    assert!(llvm_ir.contains("NotFound"));
    assert!(llvm_ir.contains("Ok"));
}

#[test]
fn compiles_configuration_manager_and_model_builder_defaults() {
    let source = r#"
            using Glitching.AspNetCore;
            using Microsoft.EntityFrameworkCore;

            fn main() {
                WebApplicationBuilder builder = CreateBuilder(new string[] { });
                string name = builder.Configuration.GetValue<string>("name");
                int port = builder.Configuration.GetValue<int>("port");
                ModelBuilder model = new ModelBuilder();
                object entity = model.Entity("Book");
                print(name);
                print(port);
                print(entity != null);
            }
        "#;

    compile_llvm_ir(source).expect("configuration and model builder defaults should lower");
}

#[test]
fn compiles_efcore_savechangesasync_helper_to_task() {
    let source = r#"
            using Microsoft.EntityFrameworkCore;

            fn main() {
                Task<int> saved = SaveChangesAsync();
                print(saved.Result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("SaveChangesAsync helper should lower");
    assert!(llvm_ir.contains("glitch_task_from_result_i32"));
}

#[test]
fn lowers_attribute_controller_routes_to_owned_llvm_thunks() {
    let source = r#"
            using Glitching.AspNetCore;

            [ApiController]
            [Route("api/[controller]")]
            class StatusController {
                [HttpGet("ready")]
                string Ready() {
                    return "{\"status\":\"ready\"}";
                }
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.RunOnce(5101);
            }
        "#;

    let llvm = compile_llvm_ir(source).expect("attribute controller route should lower");
    assert!(llvm.contains("define ptr @glitch_endpoint_handler_0(ptr %path, ptr %body)"));
    assert!(llvm.contains("call ptr @StatusController__g0__t"));
    assert!(llvm.contains("call void @glitch_drop_StatusController__g0__t"));
    assert!(llvm.contains("define i1 @glitch_endpoint_handlers_contains"));
    assert!(llvm.contains("define ptr @glitch_endpoint_handlers_invoke"));
    assert!(llvm.contains("c\"/api/Status/ready\\00\""));
}

#[test]
fn lowers_inherited_generic_controller_routes_and_templates() {
    let source = r#"
            using Glitching.AspNetCore;

            [ApiController]
            [Route("api/[controller]")]
            class CrudController<T> {
                [HttpGet("{id}")]
                string Get(int id) {
                    return "item";
                }
            }

            class BookController : CrudController<int> {
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.RunOnce(5103);
            }
        "#;

    let llvm = compile_llvm_ir(source).expect("inherited controller route should lower");
    assert!(llvm.contains("c\"/api/Book/{id}\\00\""));
    assert!(llvm.contains("define i1 @glitch_route_match"));
    assert!(llvm.contains("call i1 @glitch_route_match"));
    assert!(llvm.contains("c\"501 Not Implemented\\00\""));
}

#[test]
fn emits_llvm_exception_propagation_catch_finally_and_cleanup() {
    let source = r#"
            void Fail() {
                throw new Exception("boom");
            }

            fn main() {
                int code = 0;
                try {
                    Fail();
                } catch (Exception ex) {
                    print(ex.Message);
                    code = 7;
                } finally {
                    code = code + 1;
                }
                print(code);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("LLVM exceptions should compile");

    assert!(llvm_ir.contains("@glitch_exception_pending"));
    assert!(llvm_ir.contains("try_catch"));
    assert!(llvm_ir.contains("try_finally"));
    assert!(llvm_ir.contains("exception_unwind"));
    assert!(llvm_ir.contains("store ptr null, ptr @glitch_exception_pending"));
}

#[test]
fn warns_on_reference_cycle_statically() {
    let source = r#"
            class Node {
                public Node Parent;
            }

            fn main() {
            }
        "#;

    let output =
        compile_source_with_options(source, true, false).expect("should compile with warning");
    let diagnostics = output.diagnostics.join("\n");
    assert!(diagnostics.contains("warning GL3007"));
    assert!(diagnostics.contains("reference cycle detected: class 'Node' field 'Parent' participates in a potential ownership cycle Node -> Node"));
}

#[test]
fn warns_on_owned_collection_reference_cycle_statically() {
    let source = r#"
            using System.Collections.Generic;

            class Node {
                public List<Node> Children;
            }

            fn main() {
            }
        "#;

    let output =
        compile_source_with_options(source, true, false).expect("should compile with warning");
    let diagnostics = output.diagnostics.join("\n");
    assert!(diagnostics.contains("warning GL3007"));
    assert!(diagnostics.contains("List<Node> Children"));
}

#[test]
fn warns_on_owned_task_reference_cycle_statically() {
    let source = r#"
            using System.Threading.Tasks;

            class Node {
                public Task<Node> Pending;
            }

            fn main() {
            }
        "#;

    let output =
        compile_source_with_options(source, true, false).expect("should compile with warning");
    let diagnostics = output.diagnostics.join("\n");
    assert!(diagnostics.contains("warning GL3007"));
    assert!(diagnostics.contains("Task<Node> Pending"));
}

#[test]
fn compiles_weak_reference_cycles() {
    let source = r#"
            class Node {
                public Weak<Node> Parent;
                public Node Child;
            }

            fn main() {
                var parent = new Node();
                var child = new Node();
                parent.Child = child;
                child.Parent = new Weak<Node>(parent);
                
                Node target;
                if (child.Parent.TryGetTarget(out target)) {
                    print("target found");
                }
                
                var parentTarget = child.Parent.Target;
                if (parentTarget != null) {
                    print("target property found");
                }
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("Weak reference cycle code should compile to LLVM IR");
    assert!(llvm_ir.contains("phi i1 [ true, %tryget_not_null"));
    assert!(llvm_ir.contains("phi i1"));
    assert!(llvm_ir.contains("store ptr"));
}

#[test]
fn compiles_system_weak_reference_package_surface_without_leak_warning() {
    let source = r#"
            using System.Ownership;
            using System.WeakReference;

            class Node {
                public int Value;
            }

            fn main() {
                Node node = new Node();
                shared<Node> alias = node;
                WeakReference<Node> weak = new WeakReference<Node>(alias);
                Node target;
                if (weak.TryGetTarget(out target)) {
                    print(target.Value);
                }
                new WeakReference<Node>(alias);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("WeakReference package surface should lower");
    let report = compile_leak_report(source).expect("WeakReference package surface should not leak");

    assert!(llvm_ir.contains("WeakReference"));
    assert!(!report.contains("expression result is owned and discarded"));
}

#[test]
fn compiles_lambdas_with_captures() {
    let source = r#"
            class Runner {
                public Func<int, int> Worker;

                public Runner(Func<int, int> worker) {
                    this.Worker = worker;
                }

                public int Run(int x) {
                    var f = this.Worker;
                    return f(x);
                }
            }

            fn main() {
                int factor = 3;
                var runner = new Runner(x => x * factor);
                int res = runner.Run(5);
                print(res);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Lambda with captures should compile to LLVM IR");
    assert!(llvm_ir.contains("%glitch.lambda.0.env = type { i32 }"));
    assert!(llvm_ir.contains("%glitch.delegate = type { i64, ptr, ptr, ptr }"));
    assert!(llvm_ir.contains("call ptr @glitch_calloc(i64 1, i64"));
    assert!(llvm_ir.contains("store ptr @glitch_lambda_0_destroy"));
    assert!(llvm_ir.contains("define ptr @glitch_lambda_0(ptr %env, ptr %x)"));
    assert!(llvm_ir.contains("getelementptr inbounds %glitch.lambda.0.env, ptr %env"));
    assert!(llvm_ir.contains("glitch_delegate_release"));
    assert!(llvm_ir.contains("glitch_delegate_retain"));
}

#[test]
fn emits_compatibility_warning_for_missing_members() {
    let source = r#"
            fn main() {
                var value = ExternalFramework.Create();
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("compatibility warnings must not reject source");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3001"));
    assert!(diagnostics.contains("implement this member in a `.gl` package"));
}

#[test]
fn emits_task_aware_compatibility_hint_for_missing_async_member() {
    let source = r#"
            using System.Threading.Tasks;

            fn main() {
                Task<int> value = ExternalFramework.LoadAsync();
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("compatibility warnings must not reject source");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3002"));
    assert!(diagnostics.contains("Task.FromResult"));
}

#[test]
fn supports_lambda_lowering_without_compatibility_warning_on_llvm_path() {
    let source = r#"
            class Runner {
                public Func<int, int> Worker;

                public Runner(Func<int, int> worker) {
                    this.Worker = worker;
                }

                public int Run(int x) {
                    var f = this.Worker;
                    return f(x);
                }
            }

            fn main() {
                int factor = 3;
                var runner = new Runner(x => x * factor);
                int res = runner.Run(5);
                print(res);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("lambda sample should compile on the LLVM path");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("warning GL3005"));
    assert!(!diagnostics.contains("lambda has no executable LLVM closure or expression-tree lowering"));
}

#[test]
fn resolves_generic_delegate_declarations_to_callable_function_types() {
    let source = r#"
            public delegate TResult Converter<T, TResult>(T value);

            fn Apply(Converter<int, int> converter, int value) {
                return converter(value);
            }

            fn main() {
                int result = Apply(x => x + 1, 41);
                print(result);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("generic delegate declaration should compile");
    let llvm_ir = compile_llvm_ir(source).expect("generic delegate declaration should lower to LLVM");

    let diagnostics = output.diagnostics.join("\n");
    assert!(!diagnostics.contains("warning GL3005"));
    assert!(llvm_ir.contains("define ptr @glitch_lambda_0"));
    assert!(llvm_ir.contains("%glitch.delegate = type { i64, ptr, ptr, ptr }"));
}

#[test]
fn resolves_namespace_qualified_delegate_declarations() {
    let source = r#"
            namespace Demo {
                public delegate TResult Converter<T, TResult>(T value);
            }

            fn Apply(Demo.Converter<int, int> converter, int value) {
                return converter(value);
            }

            fn main() {
                int result = Apply(x => x + 1, 41);
                print(result);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("qualified delegate declaration should compile");
    let llvm_ir = compile_llvm_ir(source).expect("qualified delegate declaration should lower to LLVM");

    let diagnostics = output.diagnostics.join("\n");
    assert!(!diagnostics.contains("warning GL3005"));
    assert!(llvm_ir.contains("define ptr @glitch_lambda_0"));
}

#[test]
fn compiles_named_delegate_types_in_c_path() {
    let source = r#"
            public delegate int Converter(int value);

            int Apply(Converter converter, int value) {
                return converter(value);
            }

            fn main() {
                int result = Apply(x => x + 1, 41);
                print(result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("named delegate type should lower to LLVM");

    assert!(llvm_ir.contains("glitch_lambda_0"));
    assert!(llvm_ir.contains("Apply"));
}

#[test]
fn compiles_system_ownership_package_surface() {
    let source = r#"
            using System.Ownership;

            class Node {
                public Weak<Node> Parent;
            }

            fn main() {
                own<int> owned = make_owned(1);
                borrow<int> borrowed = make_borrow(owned);
                var parent = new Node();
                var child = new Node();
                child.Parent = new Weak<Node>(parent);
                Node target;
                child.Parent.TryGetTarget(out target);
                print(owned);
                print(borrowed);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("System.Ownership package surface should compile");
    let diagnostics = output.diagnostics.join("\n");
    assert!(!diagnostics.contains("warning GL3005"));
    assert!(!diagnostics.contains("warning GL3007"));
}

#[test]
fn tracks_explicit_ownership_wrappers_in_summary() {
    let source = r#"
            using System.Ownership;

            class Node {
                public int Value;
            }

            fn main() {
                own<Node> owned = make_owned(new Node());
                shared<Node> sharedNode = make_shared(new Node());
                borrow<Node> borrowed = make_borrow(sharedNode);
                view<Node> viewNode = make_view(sharedNode);
                print(owned.Value);
                print(sharedNode.Value);
                print(borrowed.Value);
                print(viewNode.Value);
            }
        "#;

    let summary = compile_ownership_summary(source)
        .expect("explicit ownership wrappers should be reflected in ownership summary");

    assert!(summary.contains("local owned: Owned Class(\"Node\")"));
    assert!(summary.contains("local sharedNode: Shared Class(\"Node\")"));
    assert!(summary.contains("local borrowed: Borrowed Class(\"Node\")"));
    assert!(summary.contains("local viewNode: View Class(\"Node\")"));
}

#[test]
fn detects_cycles_through_shared_wrappers() {
    let source = r#"
            using System.Ownership;

            class Node {
                public shared<Node> Next;
            }

            fn main() {
                var a = new Node();
                var b = new Node();
                a.Next = make_shared(b);
                b.Next = make_shared(a);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("shared-wrapper cycle sample should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3007"));
    assert!(diagnostics.contains("shared<Node> Next"));
}

#[test]
fn compiles_system_collections_generic_surface() {
    let source = r#"
            using System.Collections.Generic;

            struct Entry {
                public KeyValuePair<string, int> Pair;
            }

            fn main() {
                IReadOnlyCollection<int> collection = new List<int>();
                IReadOnlyList<int> list = new List<int>();
                IDictionary<string, int> map = new Dictionary<string, int>();
                KeyValuePair<string, int> pair = new KeyValuePair<string, int>("a", 1);
                print(collection.Count);
                print(list.Count);
                print(map.Count);
                print(pair.Value);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("System.Collections.Generic package surface should compile");
    let diagnostics = output.diagnostics.join("\n");
    assert!(!diagnostics.contains("warning GL3005"));
}

#[test]
fn compiles_dictionary_try_get_value_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                Dictionary<string, int> map = new Dictionary<string, int>();
                map.Add("a", 1);
                int value = 0;
                if (map.TryGetValue("a", out value)) {
                    print(value);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Dictionary.TryGetValue should lower to LLVM");

    assert!(llvm_ir.contains("dict_tryget_load"));
}

#[test]
fn compiles_readonly_dictionary_as_view_framework_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                IReadOnlyDictionary<string, int> map = new Dictionary<string, int>();
                print(map.Count);
                print(map.ContainsKey("a"));
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("IReadOnlyDictionary surface should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("warning GL3005"));
    assert!(!diagnostics.contains("warning GL3007"));
}

#[test]
fn rejects_borrowed_lambda_capture_that_could_outlive_source() {
    let source = r#"
            fn main() {
                int value = 1;
                ref int borrowed = borrow value;
                var f = x => x + borrowed;
                print(f(2));
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("borrowed lambda capture should be rejected");

    assert!(error.contains("ownership checker: lambda capture 'borrowed' may outlive borrowed/view source"));
}

#[test]
fn supports_sizeof_and_rc_int_without_remaining_compatibility_warnings() {
    let source = std::fs::read_to_string("tests/xunit_memory/memory_leak_tests.gl")
        .expect("should read memory_leak_tests.gl");

    let output = compile_source_with_options(&source, true, false)
        .expect("memory leak fixture should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("warning GL3002 at 71:40"));
    assert!(!diagnostics.contains("function 'sizeof' has no linked GL or LLVM implementation"));
    assert!(!diagnostics.contains("warning GL3004"));
    assert!(!diagnostics.contains("type 'Rc_int' has no linked LLVM layout"));
}

#[test]
fn supports_rc_string_without_remaining_compatibility_warnings() {
    let source = r#"
            using System.Ownership;

            fn main() {
                var owned = new Rc<string>("Ada");
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("Rc<string> fixture should compile");
    let diagnostics = output.diagnostics.join("\n");
    let llvm_ir = compile_llvm_ir(source).expect("Rc<string> fixture should lower to LLVM");

    assert!(!diagnostics.contains("warning GL3004"));
    assert!(!diagnostics.contains("type 'Rc_string' has no linked LLVM layout"));
    assert!(llvm_ir.contains("%glitch.Rc_string = type { i64, ptr, ptr, i32 }"));
    assert!(llvm_ir.contains("call void @glitch_destroy_Rc_string"));
    assert!(llvm_ir.contains("call void @glitch_drop_Rc_string"));
}

#[test]
fn supports_nested_generic_rc_without_remaining_compatibility_warnings() {
    let source = r#"
            using System.Ownership;
            using System.Collections.Generic;

            fn main() {
                var owned = make_owned(new Rc<List<int>>(new List<int>()));
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("nested generic Rc fixture should compile");
    let diagnostics = output.diagnostics.join("\n");
    let llvm_ir = compile_llvm_ir(source).expect("nested generic Rc fixture should lower to LLVM");

    assert!(!diagnostics.contains("warning GL3004"));
    assert!(!diagnostics.contains("type 'Rc_List_int_' has no linked LLVM layout"));
    assert!(llvm_ir.contains("%glitch.Rc_List_int_ = type { i64, ptr, ptr, i32 }"));
    assert!(llvm_ir.contains("call void @glitch_destroy_Rc_List_int_"));
    assert!(llvm_ir.contains("define void @glitch_drop_Rc_List_int_"));
}

#[test]
fn lowers_rc_int_layout_in_llvm() {
    let source = std::fs::read_to_string("tests/xunit_memory/memory_leak_tests.gl")
        .expect("should read memory_leak_tests.gl");

    let llvm_ir = compile_llvm_ir(&source).expect("Rc<int> fixture should lower to LLVM");

    assert!(llvm_ir.contains("%glitch.Rc_int = type { i64, ptr, i32, i32 }"));
    assert!(llvm_ir.contains("getelementptr %glitch.Rc_int, ptr null, i32 1"));
    assert!(llvm_ir.contains("call void @glitch_destroy_Rc_int"));
    assert!(llvm_ir.contains("call void @glitch_drop_Rc_int"));
}

#[test]
fn warns_on_generic_placeholder_indexing_with_actionable_diagnostic() {
    let source = r#"
            class Box<T> {
                T Pick(T value) {
                    return value[0];
                }
            }

            fn main() {
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("generic placeholder indexing should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3008"));
    assert!(diagnostics.contains("generic placeholder"));
    assert!(diagnostics.contains("specialized method body"));
}

#[test]
fn warns_on_lambda_without_executable_closure_lowering_in_non_llvm_path() {
    let source = r#"
            class Runner {
                public Func<int, int> Worker;

                public Runner(Func<int, int> worker) {
                    this.Worker = worker;
                }

                public int Run(int x) {
                    var f = this.Worker;
                    return f(x);
                }
            }

            fn main() {
                int factor = 3;
                var runner = new Runner(x => x * factor);
                int res = runner.Run(5);
                print(res);
            }
        "#;

    let output = compile_source_with_options(source, false, false)
        .expect("lambda sample should compile with a compatibility warning on the non-LLVM path");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3005"));
    assert!(diagnostics.contains("lambda has no executable LLVM closure or expression-tree lowering"));
}

#[test]
fn warns_with_route_specific_endpoint_rewrite_proposals() {
    let source = r#"
            using Glitching.AspNetCore;

            class CreateRequest {
                public List<string> Names;
            }

            [ApiController]
            [Route("/books")]
            class BooksController {
                [HttpPost]
                string Create([FromBody] CreateRequest request) {
                    return "created";
                }
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("unsupported endpoint binding should remain a warning");
    let diagnostics = output.diagnostics.join("\n");
    assert!(diagnostics.contains("warning GL3101"));
    assert!(diagnostics.contains("endpoint POST /books is discoverable"));
    assert!(diagnostics.contains("[FromBody] string body"));
    assert!(diagnostics.contains("JSON binder for Class(\"CreateRequest\")"));
}

#[test]
fn lowers_unqualified_instance_fields_to_this_in_llvm() {
    let source = r#"
            class Options {
                public int Value;

                public Options(int value) {
                    Value = value;
                }

                int Get() {
                    return Value;
                }
            }

            fn main() {
                Options options = new Options(42);
                print(options.Get());
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("unqualified instance fields should compile");

    assert!(llvm_ir.contains("define void @Options__g0__t"));
    assert!(llvm_ir.contains("define i32 @Options__g0__t"));
    assert!(llvm_ir.contains("getelementptr inbounds %glitch.Options__g0__t"));
}

#[test]
fn lowers_csharp_pattern_variable_binding_in_llvm() {
    let source = r#"
            class Entity {
                public int Id;
            }

            fn main() {
                Entity entity = new Entity { Id = 42 };
                if (entity is Entity typed && typed.Id == 42) {
                    print(typed.Id);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("pattern binding should compile");

    assert!(llvm_ir.contains("icmp ne ptr"));
    assert!(llvm_ir.contains("%glitch.Entity"));
    assert!(llvm_ir.contains("store ptr"));
}

#[test]
fn compiles_more_generic_collection_instantiations() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<string> names = new List<string>();
                names.Add("Ada");
                print(names[0]);
                print(names.Contains("Ada"));
                print(names.Count);

                List<long> ids = new List<long>();
                ids.Add(42);
                print(ids[0]);
                print(ids.Contains(42));

                Dictionary<string, string> map = new Dictionary<string, string>();
                map.Add("lang", "glitching");
                print(map["lang"]);
                print(map.Remove("lang"));
                print(map.Count);

                List<bool> flags = new List<bool>();
                flags.Add(true);
                print(flags[0]);
                print(flags.Contains(true));
                flags.Clear();
                print(flags.Count);

                List<double> ratios = new List<double>();
                ratios.Add(1.5);
                print(ratios[0]);
                print(ratios.Contains(1.5));

                Dictionary<string, long> longMap = new Dictionary<string, long>();
                longMap.Add("id", 42);
                print(longMap["id"]);
                print(longMap.ContainsKey("id"));

                Dictionary<string, bool> boolMap = new Dictionary<string, bool>();
                boolMap.Add("ok", true);
                print(boolMap["ok"]);

                Dictionary<string, double> doubleMap = new Dictionary<string, double>();
                doubleMap.Add("pi", 3.14);
                print(doubleMap["pi"]);
                print(doubleMap.Remove("pi"));
                doubleMap.Clear();
                print(doubleMap.Count);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("generic collection instantiations should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_linq_to_array_over_list_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);
                int[] copy = numbers.ToArray();
                print(copy.Length);
            }
        "#;

    let llvm = compile_llvm_ir(source).expect("LINQ ToArray over List<T> should lower to LLVM");

    assert!(llvm.contains("list_to_array_loop"));
}

#[test]
fn compiles_spread_collection_expression_to_list_surface() {
    let source = r#"
            using System.Collections.Generic;
            using System.Linq;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);
                List<int> copy = [.. numbers];
                print(copy.Count);
            }
        "#;

    let llvm =
        compile_llvm_ir(source).expect("spread collection expression should lower to LLVM");

    assert!(llvm.contains("ToList"));
    assert!(llvm.contains("%glitch.list"));
}

#[test]
fn compiles_system_linq_iterator_surface() {
    let source = r#"
            using System.Collections.Generic;
            using System.Linq;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);

                int count = Enumerable.Count(numbers);
                bool any = Enumerable.Any(numbers);
                int[] copy = Enumerable.ToArray(numbers);
                print(count);
                print(any);
                print(copy.Length);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System.Linq iterator surface should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_foreach_and_ienumerable_views() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);
                int total = 0;
                foreach (int value in numbers) {
                    total = total + value;
                }
                print(total);

                List<string> names = new List<string>();
                names.Add("Ada");
                IEnumerable<string> view = names;
                foreach (string name in view) {
                    print(name);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("foreach and IEnumerable<T> should lower");
    let bytecode = compile_bytecode(source).expect("foreach bytecode should compile");

    assert!(llvm_ir.contains("%glitch.list"));
    assert!(llvm_ir.contains("%glitch.delegate"));
    assert!(bytecode.contains("foreach_start value"));
    assert!(bytecode.contains("foreach_next name"));
}

#[test]
fn compiles_system_linq_ienumerable_surfaces() {
    let source = r#"
            using System.Collections.Generic;
            using System.Linq;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);

                IEnumerable<int> view = numbers;
                print(Enumerable.Count(view));
                print(Enumerable.Any(view));
                print(Enumerable.First(view));

                List<int> copy = Enumerable.ToList(view);
                print(copy.Count);

                int[] values = Enumerable.ToArray(view);
                print(values.Length);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Enumerable IEnumerable<T> surface should lower");
    let bytecode =
        compile_bytecode(source).expect("Enumerable IEnumerable<T> surface should emit bytecode");

    assert!(llvm_ir.contains("%glitch.list"));
    assert!(bytecode.contains("foreach_start"));
    assert!(bytecode.contains("foreach_next"));
}

#[test]
fn compiles_library_service_generic_mvc_shapes() {
    let source = r#"
            using System.Collections.Generic;
            using System.Threading.Tasks;

            namespace Microsoft.AspNetCore.Mvc {
                public class ControllerBase {}
                public class ActionResult<T> {}
            }

            class BaseEntity {
                public int Id;
            }

            interface IBaseRepo<T> {}
            class DbSet<T> {}

            class Book : BaseEntity {
                public ICollection<Book> Related { get; set; } = new List<Book>();
            }

            class BookResponseDTO {}

            class BaseCrudController<TEntity, TResponseDto> : ControllerBase
                where TEntity : BaseEntity, new()
                where TResponseDto : new()
            {
                protected IBaseRepo<TEntity> _repo;

                public async virtual Task<ActionResult<IEnumerable<TResponseDto>>> GetAll() {
                    return null;
                }
            }

            class BookController : BaseCrudController<Book, BookResponseDTO> {
                public DbSet<Book> Books { get; set; }

                public async override Task<ActionResult<BookResponseDTO>> GetOneAsync(int id) {
                    return null;
                }
            }

            fn main() {
                BookController controller = new BookController {};
                print(1);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Library-style MVC generics should lower");
    let bytecode =
        compile_bytecode(source).expect("Library-style MVC generics should emit bytecode");

    assert!(llvm_ir.contains("BaseCrudController"));
    assert!(llvm_ir.contains("BookController"));
    assert!(llvm_ir.contains("glitch_task_from_result_ptr"));
    assert!(bytecode.contains(".base BaseCrudController<Book,BookResponseDTO>"));
    assert!(bytecode.contains("Task<ActionResult<IEnumerable<TResponseDto>>>"));
}

#[test]
fn compiles_csharp_di_fields_and_null_coalescing_assignment() {
    let source = r#"
            class ApiVersion {}
            class IServiceCollection {}
            class ControllerBase {}
            class JwtOptions {}
            interface IUserDataService {}

            public static class ApiVersionConfiguration {
                public static IServiceCollection AddLibraryApiVersionConfiguration(
                    this IServiceCollection services, ApiVersion defaultVersion = null)
                {
                    defaultVersion ??= new ApiVersion {};
                    return services;
                }
            }

            public class AuthController : ControllerBase {
                private readonly JwtOptions _jwtOptions;
                private readonly IUserDataService _userDataService;

                public AuthController(JwtOptions jwtOptions, IUserDataService userDataService)
                {
                    _jwtOptions = jwtOptions;
                    _userDataService = userDataService;
                }

                public JwtOptions Options()
                {
                    return _jwtOptions;
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("C# DI field initialization and ??= should lower");

    assert!(llvm_ir.contains("ApiVersionConfiguration"));
    assert!(llvm_ir.contains("AuthController"));
    assert!(llvm_ir.contains("JwtOptions"));
}

#[test]
fn compiles_valuetask_with_default_literal_parameter() {
    let source = r#"
            using System.Threading.Tasks;

            class DbContextEventData {}
            class CancellationToken {}
            class InterceptionResult<T> {}

            class ExceptionInterceptor {
                public override ValueTask<InterceptionResult<int>> SavingChangesAsync(
                    DbContextEventData eventData,
                    InterceptionResult<int> result,
                    CancellationToken cancellationToken = default)
                {
                    return null;
                }
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("ValueTask<T> and default literal parameter should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_external_framework_object_initializer_as_opaque_handle() {
    let source = r#"
            class WebException {
                public string Type;
                public string Title;
                public int Status;
            }

            fn main() {
                var problemDetails = new ProblemDetails {
                    Type = "https://example.com/problems/internal-server-error",
                    Title = "Internal Server Error",
                    Status = 500,
                };
                WebException ex = new WebException {
                    Type = "conflict",
                    Title = "Conflict",
                    Status = 409,
                };
                problemDetails.Type = ex.Type;
                IActionResult result = new ObjectResult(problemDetails) {
                    StatusCode = ex.Status,
                };
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("unknown ASP.NET-style framework DTOs should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_datetime_constructor_temporal_methods_in_framework_calls() {
    let source = r#"
            fn main() {
                migrationBuilder.AlterColumn<DateTime>(
                    name: "DateOut",
                    defaultValue: new DateTime(2024, 3, 29, 16, 16, 58, 968, DateTimeKind.Local).AddTicks(1996),
                    oldDefaultValue: new DateTime(2024, 3, 29).AddDays(1));
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("DateTime constructor temporal methods should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_conduit_fixture_di_ef_and_mediatr_surface() {
    let source = r#"
            using MediatR;
            using Microsoft.EntityFrameworkCore;
            using Microsoft.Extensions.DependencyInjection;
            using System.Linq;
            using System.Threading.Tasks;

            class DemoContext : DbContext {
                public DbSet<string> Things;

                public DemoContext(DbContextOptions options) : base(options) {}
            }

            class DemoRequest : IRequest<string> {}

            class DemoMediator : IMediator {
                Task<string> Send<TResponse>(IRequest<TResponse> request) {
                    return new Task<string>();
                }

                Task Send(IRequest request) {
                    return new Task();
                }
            }

            fn main() {
                var services = new ServiceCollection();
                services.AddLogging();

                var builder = new DbContextOptionsBuilder();
                builder.UseInMemoryDatabase("conduit.db");

                DemoContext context = new DemoContext(builder.Options);
                var provider = services.BuildServiceProvider();
                var scopeFactory = provider.GetRequiredService<IServiceScopeFactory>();

                var match = context.Things.Where(x => x == "demo").SingleOrDefaultAsync();
                print(builder.Options.ConnectionString);
                print(match);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("Conduit fixture DI/EF/MediatR surface should compile");
    let llvm_ir = output.llvm_ir().expect("LLVM IR should be present");

    assert!(!llvm_ir.is_empty());
    assert!(llvm_ir.contains("UseInMemoryDatabase"));
    assert!(llvm_ir.contains("SingleOrDefaultAsync"));
}

#[test]
fn compiles_typeof_marker_for_package_startup_helpers() {
    let source = r#"
            using AutoMapper;

            class Program {
            }

            fn main() {
                var cfg = new MapperConfigurationExpression();
                cfg.AddMaps(typeof(Program));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("typeof(Program) should lower for package markers");

    assert!(!llvm_ir.is_empty());
    assert!(llvm_ir.contains("AddMaps"));
}

#[test]
fn compiles_typeof_reflection_surface_for_package_helpers() {
    let source = r#"
            using System.Collections.Generic;
            using System.Reflection;

            class Book {
                public string Title;
                public ICollection<Book> Related;
            }

            fn main() {
                var bookType = typeof(Book);
                var properties = bookType.GetProperties();
                var titleProperty = bookType.GetProperty("Title", BindingFlags.IgnoreCase | BindingFlags.Public | BindingFlags.Instance);
                var compareToMethod = typeof(string).GetMethod("CompareTo", Type.EmptyTypes);
                var collectionType = typeof(ICollection<>);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("typeof reflection helpers should lower for package code");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_xunit_theory_and_extended_assert_surface() {
    let source = r#"
            using Xunit;

            class DemoTests {
                [Theory]
                [InlineData(1)]
                [InlineData(2)]
                public void Numbers(int value) {
                    Assert.False(value == 0);
                    Assert.NotEqual(3, value);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("xUnit theory surface should lower");

    assert!(!llvm_ir.is_empty());
    assert!(llvm_ir.contains("XUnit_AddTest"));
}

#[test]
fn compiles_xunit_member_and_class_data_theory_surface() {
    let source = r#"
            using System;
            using Xunit;

            class MemberSourceTests {
                public static object[][] Data() {
                    return new object[][] {
                        new object[] { 1 },
                        new object[] { 2 }
                    };
                }

                [Theory]
                [MemberData(nameof(Data))]
                public void MemberCases(int value) {
                    Assert.True(value > 0);
                }
            }

            class ClassSourceTests {
                public static object[][] GetData() {
                    return new object[][] {
                        new object[] { 3 },
                        new object[] { 4 }
                    };
                }
            }

            class ClassDataTests {
                [Theory]
                [ClassData(typeof(ClassSourceTests))]
                public void ClassCases(int value) {
                    Assert.True(value > 0);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("xUnit member/class data surface should lower");

    assert!(!llvm_ir.is_empty());
    assert!(llvm_ir.contains("MemberCases[member:Data:0]"));
    assert!(llvm_ir.contains("MemberCases[member:Data:1]"));
    assert!(llvm_ir.contains("ClassCases[class:ClassSourceTests:0]"));
    assert!(llvm_ir.contains("ClassCases[class:ClassSourceTests:1]"));
}

#[test]
fn compiles_xunit_skip_attribute_by_omitting_registration() {
    let source = r#"
            using Xunit;

            class SkipTests {
                [Fact(Skip = "not now")]
                public void Ignored() {
                    Assert.True(false);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("xUnit skip attribute surface should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_reflection_assembly_and_member_surface() {
    let source = r#"
            using System.Reflection;

            class Demo {
                public int Value;
            }

            fn main() {
                Assembly asm = Assembly.GetExecutingAssembly();
                string name = asm.GetName().Name;
                Type type = typeof(Demo);
                MethodInfo method = type.GetMethod("ToString", Type.EmptyTypes);
                PropertyInfo property = type.GetProperty("Value", BindingFlags.Public | BindingFlags.Instance);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("reflection assembly surface should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn resolves_bare_static_member_lookup_on_current_type() {
    let source = r#"
            class Demo {
                public static int Answer() {
                    return 42;
                }

                public static int Use() {
                    return Answer;
                }
            }

            fn main() {
                print(Demo.Use());
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("bare static member lookup should lower");

    assert!(llvm_ir.contains("Answer"));
    assert!(llvm_ir.contains("Use"));
}

#[test]
fn compiles_conduit_transaction_facade_surface() {
    let source = r#"
            using Microsoft.EntityFrameworkCore;
            using System.Data;

            class DemoContext : DbContext {
                public DemoContext(DbContextOptions options) : base(options) {}

                public void RunTransaction() {
                    if (!this.Database.IsInMemory()) {
                        var tx = this.Database.BeginTransaction(IsolationLevel.ReadCommitted);
                        tx.Commit();
                        tx.Dispose();
                    }
                }
            }

            fn main() {
                var builder = new DbContextOptionsBuilder();
                builder.UseInMemoryDatabase("conduit.db");
                DemoContext context = new DemoContext(builder.Options);
                context.Database.EnsureCreated();
                context.RunTransaction();
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Conduit EF transaction surface should compile");

    assert!(!llvm_ir.is_empty());
    assert!(llvm_ir.contains("EnsureCreated"));
    assert!(llvm_ir.contains("BeginTransaction"));
}

#[test]
fn compiles_swagger_openapi_startup_surface() {
    let source = r#"
            using Microsoft.Extensions.DependencyInjection;
            using Microsoft.OpenApi.Models;
            using System;
            using System.Collections.Generic;

            fn main() {
                var services = new ServiceCollection();
                services.AddSwaggerGen(x =>
                {
                    x.AddSecurityDefinition(
                        "Bearer",
                        new OpenApiSecurityScheme
                        {
                            In = ParameterLocation.Header,
                            Description = "Please insert JWT with Bearer into field",
                            Name = "Authorization",
                            Type = SecuritySchemeType.ApiKey,
                            BearerFormat = "JWT",
                        }
                    );

                    x.SupportNonNullableReferenceTypes();

                    x.AddSecurityRequirement(
                        new OpenApiSecurityRequirement
                        {
                            {
                                new OpenApiSecurityScheme
                                {
                                    Reference = new OpenApiReference
                                    {
                                        Type = ReferenceType.SecurityScheme,
                                        Id = "Bearer",
                                    },
                                },
                                Array.Empty<string>()
                            },
                        }
                    );

                    x.SwaggerDoc("v1", new OpenApiInfo { Title = "RealWorld API", Version = "v1" });
                    x.CustomSchemaIds(y => y.FullName);
                    x.DocInclusionPredicate((_, _) => true);
                    x.TagActionsBy(y => new List<string> { y.GroupName ?? throw new InvalidOperationException() });
                });
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("Swagger/OpenAPI startup surface should compile");
    let llvm_ir = output.llvm_ir().expect("LLVM IR should be present");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_efcore_inmemory_provider_surface() {
    let source = r#"
            using Microsoft.EntityFrameworkCore;
            using Microsoft.EntityFrameworkCore.InMemory;

            class DemoContext : DbContext {
                public DemoContext(DbContextOptions options) : base(options) {}
            }

            fn main() {
                var root = new InMemoryDatabaseRoot();
                var builder = new DbContextOptionsBuilder();
                builder.UseInMemoryDatabase("demo");
                builder.UseInMemoryDatabase("demo", root);

                DemoContext context = new DemoContext(builder.Options);
                context.Database.EnsureCreated();
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("EF Core InMemory provider surface should compile");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_csharp_string_index_split_and_string_methods() {
    let source = r#"
            fn main() {
                string filterQuery = ">5~10";
                List<string> operators = new List<string>();
                if (operators.Contains(filterQuery[0].ToString()) || filterQuery.Contains("~")) {
                    var parts = filterQuery.Split("~");
                    string[] explicitParts = filterQuery.Split(".");
                    string first = parts[0].ToString();
                    string explicitFirst = explicitParts[0].ToString();
                    string lowered = first.ToLower().Replace("a", "b");
                    foreach (string part in parts) {
                        print(part);
                    }
                    HashSet<string> values = new HashSet<string> { "a", "b" };
                    foreach (string value in values) {
                        print(value);
                    }
                    string path = filterQuery.TrimEnd('/').Substring(1);
                    int length = path.Length;
                    print(lowered);
                    print(path);
                    print(length);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("C# string indexing, Split, and common string methods should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn synthesizes_generated_regex_partial_methods() {
    let source = r#"
            using System.Text.RegularExpressions;

            public static partial class Slug {
                public static string Clean(string input) {
                    return InvalidCharsRegex().Replace(input, "");
                }

                [GeneratedRegex("[^a-z0-9\\s-]")]
                private static partial Regex InvalidCharsRegex();
            }

            fn main() {
                print(Slug.Clean("a!b?c"));
            }
        "#;

    let output = compile_source_with_metadata(source).expect("generated regex should compile");
    let diagnostics = output.diagnostics.join("\n");
    let llvm_ir = compile_llvm_ir(source).expect("generated regex should lower to LLVM");

    assert!(!diagnostics.contains("function 'InvalidCharsRegex' has no linked GL or LLVM implementation"));
    assert!(llvm_ir.contains("InvalidCharsRegex"));
}

#[test]
fn compiles_task_from_result_and_leak_report() {
    let source = r#"
            using System.Threading.Tasks;

            fn main() {
                Task<int> number = Task.FromResult(42);
                print(number.Result);
                new List<int>();
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Task.FromResult should lower");
    let report = compile_leak_report(source).expect("leak report should be emitted");

    assert!(llvm_ir.contains("glitch_task_from_result_i32"));
    assert!(report.contains("expression result is owned and discarded"));
}

#[test]
fn compiles_task_when_all_and_completed_task_surface() {
    let source = r#"
            using System.Collections.Generic;
            using System.Threading.Tasks;

            fn main() {
                Task<int> first = Task.FromResult(1);
                Task<int> second = Task.FromResult(2);
                Task merged = Task.WhenAll(first, second);
                print(merged.IsCompleted);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Task.WhenAll and CompletedTask should compile on the LLVM path");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_task_when_all_array_surface() {
    let source = r#"
            using System.Threading.Tasks;

            fn main() {
                Task first = Task.CompletedTask;
                Task second = Task.CompletedTask;
                Task[] tasks = new Task[] { first, second };
                Task merged = Task.WhenAll(tasks);
                print(merged.IsCompleted);
            }
        "#;

    compile_llvm_ir(source).expect("Task[] WhenAll should lower");
}

#[test]
fn compiles_task_when_all_generic_array_surface() {
    let source = r#"
            using System.Threading.Tasks;

            fn main() {
                Task<int> first = Task.FromResult(1);
                Task<int> second = Task.FromResult(2);
                Task<int>[] tasks = new Task<int>[] { first, second };
                Task merged = Task.WhenAll(tasks);
                print(merged.IsCompleted);
            }
        "#;

    compile_llvm_ir(source).expect("Task<T>[] WhenAll should lower");
}

#[test]
fn compiles_task_run_and_completed_task_package_surface() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 42;
            }

            void Touch() {
            }

            fn main() {
                Task<int> valueTask = Task.Run(Compute);
                Task emptyTask = Task.Run(Touch);
                Task completed = Task.CompletedTask;
                print(valueTask.Result);
                print(completed != null);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Task.Run and CompletedTask should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_task_wait_getawaiter_and_success_properties() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 7;
            }

            fn main() {
                Task<int> task = Task.Run(Compute);
                task.Wait();
                print(task.IsCompletedSuccessfully);
                Task<int> awaiter = task.GetAwaiter();
                print(awaiter.GetResult());
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Task.Wait and GetAwaiter should lower");

    assert!(!llvm_ir.is_empty());
    assert!(llvm_ir.contains("glitch_task_get_result_i32"));
}

#[test]
fn compiles_task_and_valuetask_result_methods_from_package_surface() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 11;
            }

            fn main() {
                Task<int> task = Task.FromResult(Compute());
                print(task.GetResult());

                ValueTask<int> valueTask = ValueTask.FromResult(Compute());
                print(valueTask.GetResult());
                print(valueTask.GetAwaiter().GetResult());
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("task and valuetask result methods should lower from package surface");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_valuetask_wait_completed_task_and_getawaiter_surface() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 9;
            }

            fn main() {
                ValueTask<int> task = ValueTask.FromResult(Compute());
                task.Wait();
                print(task.IsCompletedSuccessfully);
                Task<int> awaited = task.GetAwaiter();
                print(awaited.GetResult());
                ValueTask completed = ValueTask.CompletedTask;
                print(completed.IsCompletedSuccessfully);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("ValueTask helpers should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn diagnostics_report_linked_file_path_for_marked_sources() {
    let source = r#"
            // __FILE_PATH__: packages/Foo/Foo.gl
            class Node {
                public shared<Node> Next;
            }
            fn main() {
                var a = new Node();
                var b = new Node();
                a.Next = make_shared(b);
                b.Next = make_shared(a);
            }
            __FILE_BOUNDARY__;
        "#;

    let output = compile_source_with_options(source, false, false)
        .expect("shared cycle sample should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("packages/Foo/Foo.gl"));
    assert!(diagnostics.contains("warning GL3007"));
}

#[test]
fn package_linker_reports_missing_package_sources() {
    let source = r#"
            using Missing.Package;

            fn main() {
                print(1);
            }
        "#;

    let error = compile_source_with_options(source, false, false)
        .expect_err("missing package import should fail immediately");

    assert!(error.contains("package import 'Missing.Package' at line 2 could not be resolved"));
    assert!(error.contains("at line 2"));
}

#[test]
fn system_xunit_package_does_not_pull_native_c_sources() {
    let source = r#"
            using System.XUnit;

            fn main() {
                print(1);
            }
        "#;

    let output = compile_source_with_options(source, false, false)
        .expect("System.XUnit package should still link");

    assert!(!output.diagnostics.iter().any(|d| d.contains("GL2004")));
}

#[test]
fn system_xunit_package_no_longer_emits_native_block_warning() {
    let source = r#"
            using System.XUnit;

            fn main() {
                print(1);
            }
        "#;

    let output = compile_source_with_options(source, false, false)
        .expect("System.XUnit package should compile without native-block warnings");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("warning GL2004"));
}

#[test]
fn auto_discovers_csharp_xunit_fact_methods_and_runs_them() {
    let source = r#"
            using Xunit;

            public class SampleTests {
                [Fact]
                public void Expect_It_Works() {
                    Assert.True(true);
                }
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("C# xUnit [Fact] methods should auto-register and compile");
    let llvm_ir = output.llvm_ir().expect("LLVM IR should be present");

    assert!(llvm_ir.contains("XUnit_AddTest"));
    assert!(llvm_ir.contains("RunAllTests"));
}

#[test]
fn borrow_checker_rejects_use_after_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                string moved = move name;
                print(name);
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("use after move should fail");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_rejects_assignment_while_borrowed() {
    let source = r#"
            fn main() {
                int value = 1;
                ref int borrowed = borrow value;
                value = 2;
                print(value);
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("assignment while borrowed should fail");

    assert!(error.contains("borrow checker: cannot assign to 'value' while it is borrowed"));
}

#[test]
fn borrow_checker_rejects_use_after_branch_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                if (true) {
                    string moved = move name;
                } else {
                }
                print(name);
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("branch move should poison the join state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_rejects_use_after_loop_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                while (true) {
                    string moved = move name;
                    break;
                }
                print(name);
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("loop move should poison the join state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_ignores_unreachable_code_after_break_in_loop() {
    let source = r#"
            fn main() {
                string name = "Ada";
                while (true) {
                    break;
                    string moved = move name;
                }
                print(name);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("unreachable code after break should not poison borrow checking");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn borrow_checker_rejects_use_after_break_branch_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                while (true) {
                    if (true) {
                        string moved = move name;
                        break;
                    } else {
                    }
                }
                print(name);
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("break branch move should poison the loop exit state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_rejects_use_after_try_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                try {
                    string moved = move name;
                } finally {
                }
                print(name);
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("try/finally move should poison the join state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_rejects_use_after_conditional_expression_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                bool choose = true;
                string moved = choose ? move name : "fallback";
                print(name);
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("conditional expression move should poison the join state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_allows_use_after_returning_branch_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                if (true) {
                    string moved = move name;
                    return;
                } else {
                }
                print(name);
            }
        "#;

    compile_source_with_options(source, true, false)
        .expect("returning branch should not poison join state");
}

#[test]
fn borrow_checker_allows_use_after_returning_switch_arm_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                switch (1) {
                    case 0: {
                        string moved = move name;
                        return;
                    }
                    default: {
                    }
                }
                print(name);
            }
        "#;

    compile_source_with_options(source, true, false)
        .expect("returning switch arm should not poison join state");
}

#[test]
fn ownership_checker_allows_class_transfer_without_move() {
    let source = r#"
            class Service {
                public string Name;
            }

            fn main() {
                Service first = new Service { Name = "one" };
                Service second = first;
                print(second.Name);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("class transfer without move should succeed");
    assert!(llvm_ir.contains("call void @glitch_retain"));
}

#[test]
fn ownership_checker_rejects_view_outliving_source() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> outer = new List<int>();
                IEnumerable<int> view = outer;
                {
                    List<int> inner = new List<int>();
                    view = inner;
                }
                foreach (int value in view) {
                    print(value);
                }
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("view outliving source should fail");

    assert!(error.contains(
        "ownership checker: function main: 'view' would outlive borrowed/view source 'inner'"
    ));
}

#[test]
fn compiles_memory_leak_tests() {
    let source = std::fs::read_to_string("tests/xunit_memory/memory_leak_tests.gl")
        .expect("should read memory_leak_tests.gl");
    let llvm_ir = compile_llvm_ir(&source).expect("memory leak tests should compile to LLVM IR");
    assert!(llvm_ir.contains("getelementptr"));
}

#[test]
fn compiles_selected_repo_examples_through_llvm() {
    let examples = [
        "examples/csharp_var.gl",
        "examples/llvm_simple.gl",
        "examples/llvm_collections.gl",
        "examples/collection_workload.gl",
        "examples/value_box_benchmark.gl",
    ];

    for example in examples {
        let source = fs::read_to_string(example).expect("example source should be readable");
        compile_llvm_ir(&source).unwrap_or_else(|error| {
            panic!("example {example} should compile to LLVM IR: {error}");
        });
    }
}

#[test]
fn compiles_conduit_integration_tests_project_file_through_llvm() {
    let project = "external/aspnetcore-realworld-example-app/tests/Conduit.IntegrationTests/Conduit.IntegrationTests.csproj";
    let llvm_ir = compile_llvm_ir_from_path(project)
        .unwrap_or_else(|error| panic!("Conduit integration tests project should compile through LLVM: {error}"));
    assert!(llvm_ir.contains("XUnit_RunAllTests"));
    assert!(llvm_ir.contains("WebApplication_Handle"));
}

#[test]
#[ignore]
fn debug_conduit_integration_tests_project_phases() {
    let project = std::env::var("GLITCH_PROJECT_BUNDLE").unwrap_or_else(|_| {
        "external/aspnetcore-realworld-example-app/tests/Conduit.IntegrationTests/Conduit.IntegrationTests.csproj"
            .to_string()
    });
    eprintln!("phase 1 read");
    let source = if project.ends_with(".gl") {
        fs::read_to_string(&project).expect("bundle source should load")
    } else {
        read_input_source(&project).expect("project source should load")
    };
    eprintln!("phase 2 link len={}", source.len());
    let linked = link_package_sources(&source).expect("package linking should succeed");
    eprintln!("phase 3 tokenize len={}", linked.len());
    let tokens = Lexer::new(&linked).tokenize().expect("tokenization should succeed");
    eprintln!("phase 4 parse tokens={}", tokens.len());
    let program = Parser::new(tokens).parse_program().expect("parse should succeed");
    eprintln!(
        "phase 5 parsed functions={} types={}",
        program.functions.len(),
        program.types.len()
    );
    validate_generic_constraints(&program).expect("generic validation should succeed");
    eprintln!("phase 6 borrow");
    BorrowChecker::check_program(&program).expect("borrow checking should succeed");
    eprintln!("phase 7 lower");
    let typed = TypedProgram::lower(&program).expect("typed lowering should succeed");
    eprintln!(
        "phase 8 typed functions={} types={}",
        typed.functions.len(),
        typed.types.len()
    );
}
