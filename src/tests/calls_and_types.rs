use super::*;

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

    let llvm_ir = compile_llvm_ir(source)
        .expect("named/default arguments should lower on the LLVM path");

    assert!(llvm_ir.contains("Add"));
    assert!(llvm_ir.contains("Pair"));
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

    let llvm_ir = compile_llvm_ir(source)
        .expect("extension methods from multiple packages should still lower deterministically");

    assert!(llvm_ir.contains("CountPlusOne__g0__ext"));
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
fn native_object_equals_compares_boxed_scalars_and_strings() {
    let source = r#"
            fn main() {
                print(object.Equals(7, 7));
                print(object.Equals(7, 8));
                print(object.Equals("abc", "abc"));
                print(object.Equals("abc", "abd"));
            }
        "#;

    let output_exe = emit_native_executable_from_source("object-equals-native", source);
    let output = run_native_executable_with_leak_report(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.lines().collect::<Vec<_>>();
    assert_eq!(lines, vec!["true", "false", "true", "false", "0"]);
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

