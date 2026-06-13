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

    let c = compile_source(source).expect("control flow should compile");

    assert!(c.contains("static int ClampSmall(int value);"));
    assert!(c.contains("while ((i < 4))"));
    assert!(c.contains("while ((total < 20))"));
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

            bool IsReady() {
                return true;
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

                Task<bool> readyTask = Task.Run(IsReady);
                bool ready = readyTask.Result;
                print(ready);

                Task<double> ratioTask = Task.Run(LoadRatio);
                double ratio = ratioTask.GetAwaiter().GetResult();
                print(ratio);

                Task<bool> completed = Task.FromResult(true);
                print(completed.IsCompleted);
                print(completed.GetResult());

                Task<double> fromRatio = Task.FromResult(2.5);
                print(fromRatio.Result);
            }
        "#;

    let c = compile_source(source).expect("Task<T> should compile");

    assert!(c.contains("struct GlitchTask_i32 numberTask = GlitchTask_i32_run(Compute);"));
    assert!(c.contains("int value = GlitchTask_i32_result(&numberTask);"));
    assert!(c.contains("char * name = GlitchTask_string_result(&nameTask);"));
    assert!(c.contains("struct GlitchTask_bool readyTask = GlitchTask_bool_run(IsReady);"));
    assert!(c.contains("int ready = GlitchTask_bool_result(&readyTask);"));
    assert!(c.contains("struct GlitchTask_f64 ratioTask = GlitchTask_f64_run(LoadRatio);"));
    assert!(c.contains("double ratio = GlitchTask_f64_result(&ratioTask);"));
    assert!(c.contains("struct GlitchTask_bool completed = GlitchTask_bool_from_result(1);"));
    assert!(c.contains("printf(\"%d\\n\", 1);"));
    assert!(c.contains("struct GlitchTask_f64 fromRatio = GlitchTask_f64_from_result(2.5);"));
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

    let c = compile_source(source).expect("ValueTask.FromResult should compile");
    let llvm_ir = compile_llvm_ir(source).expect("ValueTask.FromResult should lower to LLVM");

    assert!(c.contains("struct GlitchTask_i32 number = GlitchTask_i32_from_result(7);"));
    assert!(llvm_ir.contains("glitch_task_from_result_i32"));
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

    compile_source(source).expect("delegate declaration should parse and compile");
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

    let c = compile_source(source).expect("async/await should compile");
    let bytecode = compile_bytecode(source).expect("async/await bytecode should compile");

    assert!(c.contains("static struct GlitchTask_i32 LoadNumber(void);"));
    assert!(c.contains("struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result(42);"));
    assert!(c.contains("struct GlitchTask_string __glitch_return = GlitchTask_string_from_owned(glitch_strdup(\"Ada\"));"));
    assert!(c.contains("int value = GlitchTask_i32_result(&numberTask);"));
    assert!(c.contains("char * name = GlitchTask_string_result(&nameTask);"));
    assert!(bytecode.contains("  await"));
}

#[test]
fn compiles_tiny_aspnet_like_supported_subset() {
    let source = include_str!("../examples/tiny_aspnet_subset.cs");

    let c = compile_source(source).expect("tiny ASP.NET-like supported subset should compile");
    let report = compile_leak_report(source).expect("leak report should compile");

    assert!(c.contains("/* metadata: attributes=ApiController, Route(\"/hello\") */"));
    assert!(c.contains("/* metadata: attributes=HttpGet(\"/\") */"));
    assert!(c.contains("static struct GlitchTask_string HelloController_Get"));
    assert!(c.contains("GlitchTask_string_from_owned(ServiceProvider_GetRequiredService"));
    assert!(c.contains("char * controllerText = GlitchTask_string_result(&controllerTask);"));
    assert!(c.contains("static void glitch_register_attribute_routes(struct WebApplication * app)"));
    assert!(c.contains("WebApplication_MapGet(app, \"/hello\", \"HelloController.Get\");"));
    assert!(c.contains("WebApplication_MapGet(app"));
    assert!(c.contains("WebApplication_MapPost(app"));
    assert!(c.contains("WebApplication_Handle(app"));
    assert!(c.contains("static char * HealthEndpoint(void);"));
    assert!(c.contains("GlitchEndpointHandlers_Add(app, \"GET\", \"/health\", HealthEndpoint);"));
    assert!(c.contains("GlitchEndpointHandlers_Contains(self, method, path)"));
    assert!(c.contains("GlitchEndpointHandlers_Invoke(self, method, path, body)"));
    assert!(c.contains("GlitchRestHost_Run"));
    assert!(c.contains("SystemTextJson_SerializeString"));
    assert!(c.contains("JsonSerializer_SerializeString(controllerText)"));
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

    let c = compile_source(source).expect("endpoint handler should compile");
    let report = compile_leak_report(source).expect("endpoint handler leak report should compile");

    assert!(c.contains("typedef char *(*GlitchEndpointHandler)(void);"));
    assert!(c.contains("static char * Ping(void);"));
    assert!(c.contains("GlitchEndpointHandlers_Add(app, \"GET\", \"/ping\", Ping);"));
    assert!(c.contains(
        "char * handlerResponse = GlitchEndpointHandlers_Invoke(self, method, path, body);"
    ));
    assert!(c.contains("free(handlerResponse);"));
    assert!(c.contains("static void GlitchEndpointHandlers_RemoveApp(struct WebApplication *app)"));
    assert!(c.contains("GlitchEndpointHandlers_RemoveApp(value);"));
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

    let c = compile_source(source).expect("ASP.NET-like middleware pipeline should compile");
    let report = compile_leak_report(source).expect("middleware leak report should compile");

    assert!(c.contains("static void WebApplication_UseTrace(struct WebApplication * self);"));
    assert!(c.contains("WebApplication_UseTrace(app);"));
    assert!(c.contains("List_string_contains(&self->Middleware, \"trace\")"));
    assert!(c.contains("glitch_string_concat(prefix, current)"));
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

    let c = compile_source(source).expect("delegate middleware should compile");
    let report =
        compile_leak_report(source).expect("delegate middleware leak report should compile");

    assert!(c.contains("typedef char *(*GlitchMiddlewareHandler)(char *);"));
    assert!(c.contains("static char * AddPrefix(char * text);"));
    assert!(c.contains("GlitchMiddlewareHandlers_Add(app, \"AddPrefix\", AddPrefix);"));
    assert!(c.contains("char * current = GlitchMiddlewareHandlers_Apply(self, text);"));
    assert!(c.contains("GlitchMiddlewareHandlers_RemoveApp(value);"));
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

    let c = compile_source(source).expect("EF Core groundwork package should compile");
    let report = compile_leak_report(source).expect("EF leak report should compile");

    assert!(
        c.contains("struct DbContext * db = DbContext_new(glitch_strdup(\"Server=:memory:\"));")
    );
    assert!(c.contains("struct DbSetString * users = SetString(db, \"Users\");"));
    assert!(c.contains("static struct IQueryableString * DbSetString_AsNoTracking"));
    assert!(c.contains("SqlProvider_BuildSelectAll(provider, self->Table)"));
    assert!(c.contains("List_string_add(&values, glitch_string_concat(prefix, query))"));
    assert!(c.contains("DbContext_Dispose(db);"));
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

    let c =
        compile_source(source).expect("framework-derived opaque property chains should compile");

    assert!(c.contains("struct ApplicationDbContext_Database *"));
    assert!(c.contains("struct string_array * args = NULL;"));
    assert!(c.contains("struct object_array * sqlParametersObjects"));
    assert!(c.contains("struct LinqHelpers_BuildWherePredicate * predicate = NULL;"));
    assert!(c.contains("int same = 1;"));
    assert!(c.contains("int hasAny = 1;"));
    assert!(c.contains("int hasIds = (ids.len > 0);"));
    assert!(c.contains("int id = ids.data["));
    assert!(c.contains("char * idText = glitch_strdup(\"\");"));
    assert!(c.contains("struct InvalidUserException * invalid = InvalidUserException_new(NULL);"));
    assert!(c.contains("glitch_throw(glitch_exception_new(\"\"));"));
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

    let c = compile_source(source).expect("overloaded functions should compile to C");
    let llvm_ir = compile_llvm_ir(source).expect("overloaded functions should compile to LLVM");
    let summary =
        compile_ownership_summary(source).expect("overloaded functions should lower to TIR");

    assert!(c.contains("static long long Pick__long(long long value);"));
    assert!(c.contains("static char * Pick__string(char * value);"));
    assert!(c.contains("long long n = Pick__long(41);"));
    assert!(c.contains("char * s = Pick__string(\"ok\");"));
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

    let c = compile_source(source).expect("overloaded instance methods should compile to C");
    let summary =
        compile_ownership_summary(source).expect("overloaded instance methods should lower");

    assert!(
        c.contains("static long long Greeter_Say__long(struct Greeter * self, long long value);")
    );
    assert!(c.contains("static char * Greeter_Say__string(struct Greeter * self, char * value);"));
    assert!(c.contains("long long n = Greeter_Say__long(greeter, 41);"));
    assert!(c.contains("char * s = Greeter_Say__string(greeter, \"ok\");"));
    assert!(summary.contains("tir call method Say symbol=Greeter_Say__long"));
    assert!(summary.contains("tir call method Say symbol=Greeter_Say__string"));
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

    let c = compile_source(source).expect("numeric overload ranking should compile");
    let summary = compile_ownership_summary(source).expect("numeric overload ranking should lower");

    assert!(c.contains("int literalResult = Pick__int(1);"));
    assert!(c.contains("int intResult = Pick__int(i);"));
    assert!(c.contains("long long longResult = Pick__long(l);"));
    assert!(c.contains("int expressionResult = Pick__int((i + 1));"));
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

    let c = compile_source(source).expect("reference overload ranking should compile");
    let summary =
        compile_ownership_summary(source).expect("reference overload ranking should lower");

    assert!(c.contains("int exact = Pick__Derived(derived);"));
    assert!(c.contains("int baseOnly = ReadBase(&derived->__base);"));
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

    let error = compile_source(source).expect_err("ambiguous overload should fail");

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

    let c = compile_source(source).expect("named/default arguments should compile");

    assert!(c.contains("int named = Add(1, 2);"));
    assert!(c.contains("int defaulted = Add(5, 10);"));
    assert!(c.contains("struct Pair * pair = Pair_new(2, 3);"));
}

#[test]
fn compiles_extension_method_calls() {
    let source = r#"
            class Counter {
                public int Value;
            }

            int ScorePlus(this Counter counter, int bonus = 1) {
                return counter.Value + bonus;
            }

            fn main() {
                Counter counter = new Counter { Value = 7 };
                int score = counter.ScorePlus(bonus: 5);
                int defaulted = counter.ScorePlus();
                print(score);
                print(defaulted);
            }
        "#;

    let c = compile_source(source).expect("extension methods should compile");

    assert!(c.contains("static int ScorePlus(struct Counter * counter, int bonus);"));
    assert!(c.contains("int score = ScorePlus(counter, 5);"));
    assert!(c.contains("int defaulted = ScorePlus(counter, 1);"));
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

    let error = compile_source(source).expect_err("missing ref modifier should fail");

    assert!(error.contains("no overload of 'Read' matches argument types"));
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

    let c = compile_source(source).expect("scalar ref/out arguments should compile");

    assert!(c.contains("static void Increment(int * value);"));
    assert!(c.contains("static void SetSeven(int * value);"));
    assert!(c.contains("*value = (*value + 1);"));
    assert!(c.contains("*value = 7;"));
    assert!(c.contains("Increment(&current);"));
    assert!(c.contains("SetSeven(&current);"));
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

    let c = compile_source(source).expect("expanded params arguments should compile");

    assert!(c.contains("static int First(struct GlitchArray_int values);"));
    assert!(c.contains("printf(\"%d\\n\", values.len);"));
    assert!(c.contains("int __glitch_return = values.data[0];"));
    assert!(c.contains("int value = First((struct GlitchArray_int){(int[]){10, 20, 30}, 3});"));
}

#[test]
fn validates_generic_constraints() {
    let source = r#"
            fn Use<T>() where T : MissingType {
            }
        "#;

    let error = compile_source(source).expect_err("unknown generic constraint should fail");

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

    let c = compile_source(source).expect("implicit conversion should compile");

    assert!(c.contains("static int op_Implicit(struct Meter value);"));
    assert!(c.contains("int result = Read(op_Implicit(meter));"));
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

    let c = compile_source(source).expect("null reference overload should compile");

    assert!(c.contains("char * maybe = NULL;"));
    assert!(c.contains("char * result = Pick(NULL);"));
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

    let c = compile_source(source).expect("System.Text.Json package should compile");
    let report = compile_leak_report(source).expect("JSON leak report should compile");

    assert!(c.contains("static char * SystemTextJson_SerializeString(char * value)"));
    assert!(c.contains("char * json = JsonSerializer_SerializeString(value);"));
    assert!(c.contains("char * plain = JsonSerializer_DeserializeString(json);"));
    assert!(report.contains("No obvious owned temporary leaks detected."));
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
    println!("ACTUAL LLVM IR:\n{}", llvm_ir);
    assert!(llvm_ir.contains("call void @System_IO_File_WriteAllText"));
    assert!(llvm_ir.contains("call ptr @System_IO_File_ReadAllText"));

    let c = compile_source(source).expect("System.IO.File operations should compile to C");
    assert!(
        c.contains("System_IO_File_WriteAllText(path, glitch_strdup(\"Hello from Glitching!\"));")
    );
    assert!(c.contains("char * content = System_IO_File_ReadAllText(path);"));
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

    let c = compile_source(source).expect("collections should compile");

    assert!(c.contains("struct List_int xs = List_int_new();"));
    assert!(c.contains("Dict_string_int_add(&scores"));
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

    let c = compile_source(source).expect("attributes and namespace should compile");

    assert!(c.contains(
        "/* metadata: namespace=Demo.Api attributes=ApiController, Route(\"/users\") */"
    ));
    assert!(c.contains("/* metadata: namespace=Demo.Api attributes=HttpGet(\"/health\") */"));
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

    let c = compile_source(source).expect("class methods should compile");

    assert!(c.contains("static int Counter_GetValue(struct Counter * self);"));
    assert!(c.contains("static int Counter_GetValue(struct Counter * self) {"));
    assert!(c.contains("int __glitch_return = self->Value;"));
    assert!(c.contains("int value = Counter_GetValue(counter);"));
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

    let c = compile_source(source).expect("method attributes should compile");

    assert!(c.contains("/* metadata: namespace=Demo.Api attributes=HttpGet(\"/count\") */"));
    assert!(c.contains("static int UsersController_GetCount(struct UsersController * self) {"));
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

    let c = compile_source(source).expect("constructors and properties should compile");

    assert!(c.contains("static struct Counter * Counter_new(int count);"));
    assert!(c.contains("static struct Counter * Counter_new(int count) {"));
    assert!(c.contains("self->Count = count;"));
    assert!(c.contains("static int Counter_get_Twice(struct Counter * self) {"));
    assert!(c.contains("struct Counter * counter = Counter_new(5);"));
    assert!(c.contains("printf(\"%d\\n\", Counter_get_Twice(counter));"));
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

    let c = compile_source(source).expect("common C# scalar surface should compile");

    assert!(c.contains("enum Status {"));
    assert!(c.contains("Status_Active = 2"));
    assert!(c.contains("static struct Model * Model_new(int state);"));
    assert!(c.contains("struct Model * model = Model_new(Status_Active);"));
    assert!(c.contains("char * title = glitch_strdup(\"ready\");"));
    assert!(c.contains("unsigned char b = 1;"));
    assert!(c.contains("short s = 2;"));
    assert!(c.contains("unsigned int u = 3;"));
    assert!(c.contains("long double money = 4.5;"));
    assert!(c.contains("switch (status) {"));
    assert!(c.contains("case Status_Active:"));
    assert!(c.contains("printf(\"%f\\n\", model->Score);"));
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

    let c = compile_source(source).expect("inheritance and try/finally surface should compile");

    assert!(c.contains("/* interface IScored */"));
    assert!(c.contains("struct BaseCounter __base;"));
    assert!(c.contains("self->__base.Seed = seed;"));
    assert!(c.contains("BaseCounter_GetSeed(&self->__base)"));
    assert!(c.contains("if (setjmp(__glitch_frame.env) == 0)"));
    assert!(c.contains("struct GlitchException * ex = &__glitch_frame.exception;"));
    assert!(c.contains("int __glitch_return = result;"));
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

    let c = compile_source(source).expect("throw/catch/finally should compile");

    assert!(c.contains("glitch_exception_push(&__glitch_frame);"));
    assert!(c.contains("glitch_throw(glitch_exception_from_owned(glitch_strdup(\"boom\")));"));
    assert!(c.contains("struct GlitchException * ex = &__glitch_frame.exception;"));
    assert!(c.contains("printf(\"%s\\n\", ex->message);"));
    assert!(c.contains("code = (code + 1);"));
    assert!(c.contains("glitch_exception_free(&__glitch_frame.exception);"));
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

    assert!(llvm_ir.contains("%glitch.Counter = type { i64, ptr, i32 }"));
    assert!(llvm_ir.contains("define void @Counter_ctor(ptr %this, i32 %value)"));
    assert!(llvm_ir.contains("define i32 @Counter_Increment(ptr %this)"));
    assert!(llvm_ir.contains("call ptr @glitch_calloc(i64 1, i64"));
    assert!(llvm_ir.contains("call void @Counter_ctor(ptr"));
    assert!(llvm_ir.contains("call i32 @Counter_Increment(ptr"));
    assert!(llvm_ir.contains("getelementptr inbounds %glitch.Counter"));
    assert!(llvm_ir.contains("define void @glitch_retain_Counter(ptr %object)"));
    assert!(llvm_ir.contains("define void @glitch_drop_Counter(ptr %object)"));
    assert!(llvm_ir.contains("define void @glitch_destroy_Counter(ptr %object)"));
    assert!(llvm_ir.contains("call void @glitch_drop_Counter(ptr"));
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
    assert!(llvm.contains("call void @WebApplication_MapGet"));
    assert!(llvm.contains("call void @GlitchRestHost_Run(ptr"));
    assert!(llvm.contains("ptr @WebApplication_Handle"));
    assert!(llvm.contains("ptr @glitch_string_release"));
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
    assert!(llvm.contains("call ptr @StatusController_Ready(ptr %controller)"));
    assert!(llvm.contains("call void @glitch_drop_StatusController(ptr %controller)"));
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
    println!("LLVM IR:\n{}", llvm_ir);

    // Assert that the environment struct is defined
    assert!(llvm_ir.contains("%glitch.lambda.0.env = type { i32 }"));
    // Assert that the delegate struct now carries refcount, invoke, env, and destroy pointers.
    assert!(llvm_ir.contains("%glitch.delegate = type { i64, ptr, ptr, ptr }"));
    // Assert that the environment is heap-allocated and linked to a destroy helper.
    assert!(llvm_ir.contains("call ptr @glitch_calloc(i64 1, i64"));
    assert!(llvm_ir.contains("store ptr @glitch_lambda_0_destroy"));
    // Assert that the delegate function signature matches and it loads the capture
    assert!(llvm_ir.contains("define ptr @glitch_lambda_0(ptr %env, ptr %x)"));
    assert!(llvm_ir.contains("getelementptr inbounds %glitch.lambda.0.env, ptr %env"));
    // Assert that calling and releasing the delegate use the refcounted runtime helpers.
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

    let c = compile_source(source).expect("named delegate type should compile in C path");

    assert!(c.contains("GlitchDelegate"));
    assert!(c.contains("Apply"));
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
fn rejects_borrowed_lambda_capture_that_could_outlive_source() {
    let source = r#"
            fn main() {
                int value = 1;
                ref int borrowed = borrow value;
                var f = x => x + borrowed;
                print(f(2));
            }
        "#;

    let error = compile_source(source).expect_err("borrowed lambda capture should be rejected");

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
fn warns_on_lambda_without_executable_closure_lowering_in_c_path() {
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

    let output = compile_source_with_options(source, false, true)
        .expect("lambda sample should compile with a compatibility warning on the C path");
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

    assert!(llvm_ir.contains("define void @Options_ctor(ptr %this, i32 %value)"));
    assert!(llvm_ir.contains("define i32 @Options_Get(ptr %this)"));
    assert!(llvm_ir.contains("getelementptr inbounds %glitch.Options"));
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

    let c = compile_source(source).expect("generic collection instantiations should compile");

    assert!(c.contains("struct List_string names = List_string_new();"));
    assert!(c.contains("List_string_add(&names, \"Ada\")"));
    assert!(c.contains("printf(\"%s\\n\", List_string_get(&names, 0));"));
    assert!(c.contains("List_string_contains(&names, \"Ada\")"));
    assert!(c.contains("struct List_i64 ids = List_i64_new();"));
    assert!(c.contains("List_i64_add(&ids, 42)"));
    assert!(c.contains("List_i64_contains(&ids, 42)"));
    assert!(c.contains("struct Dict_string_string map = Dict_string_string_new();"));
    assert!(c.contains("Dict_string_string_add(&map, \"lang\", \"glitching\")"));
    assert!(c.contains("Dict_string_string_remove(&map, \"lang\")"));
    assert!(c.contains("struct List_bool flags = List_bool_new();"));
    assert!(c.contains("List_bool_add(&flags, 1)"));
    assert!(c.contains("List_bool_contains(&flags, 1)"));
    assert!(c.contains("List_bool_clear(&flags);"));
    assert!(c.contains("struct List_f64 ratios = List_f64_new();"));
    assert!(c.contains("List_f64_add(&ratios, 1.5)"));
    assert!(c.contains("List_f64_contains(&ratios, 1.5)"));
    assert!(c.contains("struct Dict_string_i64 longMap = Dict_string_i64_new();"));
    assert!(c.contains("Dict_string_i64_add(&longMap, \"id\", 42)"));
    assert!(c.contains("Dict_string_i64_contains_key(&longMap, \"id\")"));
    assert!(c.contains("struct Dict_string_bool boolMap = Dict_string_bool_new();"));
    assert!(c.contains("Dict_string_bool_add(&boolMap, \"ok\", 1)"));
    assert!(c.contains("struct Dict_string_f64 doubleMap = Dict_string_f64_new();"));
    assert!(c.contains("Dict_string_f64_add(&doubleMap, \"pi\", 3.14)"));
    assert!(c.contains("Dict_string_f64_remove(&doubleMap, \"pi\")"));
    assert!(c.contains("Dict_string_f64_clear(&doubleMap);"));
    assert!(c.contains("Dict_string_string_free(&map);"));
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

    let c = compile_source(source).expect("LINQ ToArray over List<T> should compile");
    let llvm = compile_llvm_ir(source).expect("LINQ ToArray over List<T> should lower to LLVM");

    assert!(c.contains("List_int_to_array(&numbers)"));
    assert!(c.contains("copy.len"));
    assert!(llvm.contains("%glitch.array"));
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

    let c = compile_source(source).expect("System.Linq iterator surface should compile");

    assert!(c.contains("List_int_to_array"));
    assert!(c.contains("Count"));
    assert!(c.contains("Any"));
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

    let c = compile_source(source).expect("foreach and IEnumerable<T> should compile");
    let bytecode = compile_bytecode(source).expect("foreach bytecode should compile");

    assert!(
        c.contains("struct IEnumerable_string view = IEnumerable_string_from_List_string(&names);")
    );
    assert!(c.contains("for (int __glitch_foreach_i_"));
    assert!(c.contains("int value = numbers.data[__glitch_foreach_i_"));
    assert!(c.contains("char * name = view.source->data[__glitch_foreach_i_"));
    assert!(c.contains("List_string_free(&names);"));
    assert!(c.contains("List_int_free(&numbers);"));
    assert!(bytecode.contains("foreach_start value"));
    assert!(bytecode.contains("foreach_next name"));
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

    let c = compile_source(source).expect("Library-style MVC generics should compile");
    let bytecode =
        compile_bytecode(source).expect("Library-style MVC generics should emit bytecode");

    assert!(c.contains("struct IEnumerable_Book * Related;"));
    assert!(c.contains("struct DbSet_Book * Books;"));
    assert!(c.contains("static struct GlitchTask_ptr BaseCrudController_GetAll"));
    assert!(c.contains("static struct GlitchTask_ptr BookController_GetOneAsync"));
    assert!(c.contains("GlitchTask_ptr_from_result(NULL)"));
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

    let c = compile_source(source).expect("C# DI field initialization and ??= should compile");

    assert!(c.contains("if ((defaultVersion == NULL))"));
    assert!(c.contains("self->_jwtOptions = jwtOptions;"));
    assert!(c.contains("self->_userDataService = userDataService;"));
    assert!(!c.contains("glitch_drop_JwtOptions(jwtOptions)"));
    assert!(!c.contains("glitch_drop_IUserDataService(userDataService)"));
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

    let c =
        compile_source(source).expect("ValueTask<T> and default literal parameter should compile");

    assert!(c.contains("static struct GlitchTask_ptr ExceptionInterceptor_SavingChangesAsync"));
    assert!(c.contains("GlitchTask_ptr_from_result(NULL)"));
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

    let c = compile_source(source)
        .expect("unknown ASP.NET-style framework DTOs should compile as opaque handles");

    assert!(c.contains("struct ProblemDetails * problemDetails = NULL;"));
    assert!(c.contains("struct IActionResult * result = NULL;"));
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

    let c = compile_source(source)
        .expect("DateTime constructor temporal methods should compile in framework calls");

    assert!(c.contains("glitch_strdup(\"\")"));
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

    let c = compile_source(source)
        .expect("C# string indexing, Split, and common string methods should compile");

    assert!(c.contains("List_string_contains(&operators, \"\")"));
    assert!(c.contains("struct string_array * parts = NULL;"));
    assert!(c.contains("struct string_array * explicitParts = NULL;"));
    assert!(c.contains("char * part = \"\";"));
    assert!(c.contains("struct HashSet_string * values = NULL;"));
    assert!(c.contains("char * value = \"\";"));
    assert!(c.contains("char * path = glitch_strdup(\"\");"));
    assert!(c.contains("int length = 0;"));
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

    let c = compile_source(source).expect("Task.FromResult should compile");
    let report = compile_leak_report(source).expect("leak report should be emitted");

    assert!(c.contains("struct GlitchTask_i32 number = GlitchTask_i32_from_result(42);"));
    assert!(report.contains("expression result is owned and discarded"));
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

    let error = compile_source(source).expect_err("use after move should fail");

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

    let error = compile_source(source).expect_err("assignment while borrowed should fail");

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

    let error = compile_source(source).expect_err("branch move should poison the join state");

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

    let error = compile_source(source).expect_err("loop move should poison the join state");

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

    let error = compile_source(source).expect_err("try/finally move should poison the join state");

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

    compile_source(source).expect("returning branch should not poison join state");
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

    let error = compile_source(source).expect_err("view outliving source should fail");

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
