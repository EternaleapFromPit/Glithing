use super::*;

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
    assert!(llvm_ir.contains("@GlitchLiveAllocations_Add"));
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
fn compiles_nested_list_drops_to_llvm() {
    let source = r#"
            fn main() {
                List<List<string>> nested = new List<List<string>>();
                List<string> inner = new List<string>();
                inner.Add("hello");
                nested.Add(inner);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Nested list drops should compile to LLVM IR");
    assert!(llvm_ir.contains("call void @glitch_string_release(ptr"));
    assert!(llvm_ir.matches("element_drop_loop").count() >= 2);
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
    assert!(llvm.contains("define ptr @glitch_endpoint_handler_0(ptr %app, ptr %path, ptr %body)"));
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
fn lowers_controller_routes_without_apicontroller_attribute() {
    let source = r#"
            using Glitching.AspNetCore;

            [Route("articles")]
            class ArticlesController : Controller {
                [HttpGet]
                string Get() {
                    return "{\"status\":\"ready\"}";
                }
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.RunOnce(5105);
            }
        "#;

    let llvm =
        compile_llvm_ir(source).expect("route-attributed MVC controller should lower");
    assert!(llvm.contains("define ptr @glitch_endpoint_handler_0(ptr %app, ptr %path, ptr %body)"));
    assert!(llvm.contains("c\"/articles\\00\""));
}

#[test]
fn lowers_nullable_query_and_cancellation_token_controller_parameters() {
    let source = r#"
            using Glitching.AspNetCore;
            using System.Threading;

            [Route("articles")]
            class ArticlesController : Controller {
                [HttpGet]
                string Get([FromQuery] int? limit, CancellationToken cancellationToken) {
                    if (limit.HasValue) {
                        return "limited";
                    }
                    return "all";
                }
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.RunOnce(5106);
            }
        "#;

    let llvm =
        compile_llvm_ir(source).expect("nullable query and cancellation token parameters should lower");
    assert!(llvm.contains("glitch_query_value_string"));
    assert!(llvm.contains("strtoll"));
    assert!(llvm.contains("Nullable_int"));
    assert!(llvm.contains("CancellationToken"));
}

#[test]
fn warns_for_configuration_manager_stub_members() {
    let source = r#"
            using Glitching.AspNetCore;

            class DemoOptions {
                public string Name;
            }

            fn main() {
                WebApplicationBuilder builder = CreateBuilder(new string[] { });
                DemoOptions options = builder.Configuration.Get<DemoOptions>();
                print(options == null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("configuration stub access should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"));
    assert!(diagnostics.contains("ConfigurationManager"));
    assert!(diagnostics.contains("compatibility stub"));
}

#[test]
fn configuration_manager_value_and_section_surfaces_lower_without_stub_warning() {
    let source = r#"
            using Glitching.AspNetCore;

            fn main() {
                WebApplicationBuilder builder = CreateBuilder(new string[] { });
                ConfigurationManager jwt = builder.Configuration.GetSection("Jwt");
                string issuer = jwt["Issuer"];
                int port = builder.Configuration.GetValue<int>("Port");
                bool rebuild = builder.Configuration.GetValue<bool>("RebuildDataBase");
                long maxItems = builder.Configuration.GetValue<long>("MaxItems");
                string connection = builder.Configuration.GetConnectionString("Library");
                print(issuer);
                print(port);
                print(rebuild);
                print(maxItems);
                print(connection);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("configuration manager value/section surfaces should compile");
    let diagnostics = output.diagnostics.join("\n");
    let llvm_ir = output.llvm_ir().expect("LLVM IR should be present");

    assert!(!diagnostics.contains("ConfigurationManager"), "{diagnostics}");
    assert!(llvm_ir.contains("GlitchRestHost_read_env_string"));
    assert!(llvm_ir.contains("GlitchRestHost_read_env_int"));
    assert!(llvm_ir.contains("GlitchRestHost_read_env_bool"));
    assert!(llvm_ir.contains("GlitchRestHost_read_env_i64"));
    assert!(llvm_ir.contains("GlitchRestHost_read_connection_string"));
}

#[test]
fn compiles_service_provider_generic_lookup_lowering() {
    let source = r#"
            using Microsoft.Extensions.DependencyInjection;

            class Service {
                public string Name;
            }

            fn main() {
                ServiceCollection services = new ServiceCollection();
                ServiceProvider provider = services.BuildServiceProvider();
                Service service = provider.GetRequiredService<Service>();
                print(service == null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("service-provider generic lookup should compile");
    let diagnostics = output.diagnostics.join("\n");
    let llvm_ir = output.llvm_ir().expect("LLVM IR should be present");

    assert!(!diagnostics.contains("GL3013"));
    assert!(llvm_ir.contains("service_lookup"));
}

#[test]
fn warns_for_automapper_map_stub_members() {
    let source = r#"
            using AutoMapper;

            class Person {
                public string Name;
            }

            class User {
                public string Name;
            }

            fn main() {
                Mapper mapper = new Mapper();
                User user = mapper.Map<Person, User>(new Person { Name = "Ada" });
                print(user == null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("AutoMapper stub access should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"));
    assert!(diagnostics.contains("AutoMapper `Map(...)`"));
}

#[test]
fn warns_for_noop_swagger_host_configuration_members() {
    let source = r#"
            using Glitching.AspNetCore;

            fn main() {
                WebApplication app = new WebApplication();
                app.UseSwagger();
                app.UseSwaggerUI();
                print(app != null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("Swagger host configuration markers should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"));
    assert!(diagnostics.contains("no-op compatibility surface"));
}

#[test]
fn warns_for_noop_generic_middleware_activation_members() {
    let source = r#"
            using Glitching.AspNetCore;

            class TraceMiddleware {
            }

            fn main() {
                WebApplication app = new WebApplication();
                app.UseMiddleware<TraceMiddleware>();
                print(app != null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("generic middleware activation markers should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"));
    assert!(diagnostics.contains("no-op compatibility surface"));
}

#[test]
fn warns_for_noop_service_registration_markers() {
    let source = r#"
            using Microsoft.Extensions.DependencyInjection;

            fn main() {
                ServiceCollection services = new ServiceCollection();
                services.AddEndpointsApiExplorer();
                services.AddMemoryCache();
                print(services != null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("service-registration markers should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"));
    assert!(diagnostics.contains("service-registration/configuration call is still a compatibility surface"));
}

#[test]
fn authentication_builder_configuration_surfaces_lower_without_placeholder_warning() {
    let source = r#"
            using Microsoft.Extensions.DependencyInjection;
            using Microsoft.AspNetCore.Authentication.JwtBearer;

            fn main() {
                ServiceCollection services = new ServiceCollection();
                AuthenticationBuilder auth = services.AddAuthentication();
                auth.AddJwtBearer(options => {});
                print(auth != null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("authentication builder configuration surfaces should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("GL3013"), "{diagnostics}");
}

#[test]
fn startup_configuration_delegate_surfaces_lower_without_placeholder_warning() {
    let source = r#"
            using Microsoft.Extensions.DependencyInjection;
            using Microsoft.AspNetCore.Authentication.JwtBearer;

            class Marks {
                public int A;
                public int B;
                public int C;
                public int D;
                public int E;
                public int F;
            }

            fn main() {
                Marks marks = new Marks();
                ServiceCollection services = new ServiceCollection();
                services.AddLocalization(x => marks.A = 11);
                services.AddSwaggerGen(x => marks.B = 12);
                services.AddCors(x => marks.C = 13);
                services.AddMvc(opt => marks.D = 14).AddJsonOptions(opt => marks.E = 15);

                AuthenticationBuilder auth = services.AddAuthentication();
                auth.AddJwtBearer(options => marks.F = 16);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("startup configuration delegates should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("GL3013"), "{diagnostics}");
}

#[test]
fn startup_configuration_surfaces_resolve_without_missing_member_fallbacks() {
    let source = r#"
            using Microsoft.Extensions.DependencyInjection;
            using Microsoft.AspNetCore.Authentication.JwtBearer;
            using Microsoft.EntityFrameworkCore;

            class DemoContext : DbContext {
                public DemoContext(DbContextOptions options) : base(options) {}
            }

            fn main() {
                ServiceCollection services = new ServiceCollection();
                services.AddDbContext<DemoContext>(options => {
                    options.UseSqlite("demo.db");
                });
                services.AddLocalization(x => x.ResourcesPath = "Resources");
                services.AddSwaggerGen(x => {
                    x.SupportNonNullableReferenceTypes();
                });
                services.AddCors();
                services.AddMvc(opt => {
                    opt.EnableEndpointRouting = false;
                }).AddJsonOptions(opt => {});

                AuthenticationBuilder auth = services.AddAuthentication();
                auth.AddJwtBearer(options => {});
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("startup configuration surfaces should compile through package members");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"), "{diagnostics}");
    for member in [
        "member 'AddDbContext'",
        "member 'AddLocalization'",
        "member 'AddSwaggerGen'",
        "member 'AddMvc'",
        "member 'AddJsonOptions'",
        "member 'AddJwtBearer'",
    ] {
        assert!(!diagnostics.contains(member), "{diagnostics}");
    }
    assert!(!diagnostics.contains("static or opaque member 'ResourcesPath'"), "{diagnostics}");
}

#[test]
fn warns_for_swagger_option_configuration_markers() {
    let source = r#"
            using Microsoft.Extensions.DependencyInjection;
            using Microsoft.OpenApi.Models;
            using Glitching.AspNetCore;

            fn main() {
                SwaggerGenOptions gen = new SwaggerGenOptions();
                gen.SwaggerDoc("v1", new OpenApiInfo());
                gen.SupportNonNullableReferenceTypes();

                SwaggerUiOptions ui = new SwaggerUiOptions();
                ui.SwaggerEndpoint("/swagger/v1/swagger.json", "v1");

                print(gen != null);
                print(ui != null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("Swagger option markers should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"));
    assert!(diagnostics.contains("Swagger/OpenAPI configuration call is still a compatibility surface"));
}

#[test]
fn warns_for_logging_configuration_markers() {
    let source = r#"
            using Glitching.AspNetCore;

            fn main() {
                WebApplicationBuilder builder = CreateBuilder(System.Array.Empty<string>());
                builder.Logging.ClearProviders();
                builder.Logging.AddSerilog("logger", true);
                builder.ConfigureSerilog();
                print(builder != null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("logging configuration markers should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"));
    assert!(diagnostics.contains("logging configuration call is still a compatibility surface"));
}

#[test]
fn compiles_dictionary_enumerator_lowering_without_stub_warning() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                Dictionary<string, int> values = new Dictionary<string, int>();
                IEnumerator<KeyValuePair<string, int>> e = values.GetEnumerator();
                print(e == null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("dictionary enumerator should compile through LLVM lowering");
    let diagnostics = output.diagnostics.join("\n");
    let llvm_ir = output.llvm_ir().expect("LLVM IR should be present");

    assert!(!diagnostics.contains("GL3013"));
    assert!(llvm_ir.contains("DictionaryEnumerator"));
}

#[test]
fn warns_for_database_facade_compatibility_surfaces() {
    let source = r#"
            using Microsoft.EntityFrameworkCore;
            using System.Data;

            fn main() {
                DatabaseFacade database = new DatabaseFacade("demo");
                database.EnsureCreated();
                database.Migrate();
                database.ExecuteSqlRaw("delete from Books", null);
                DatabaseTransaction tx = database.BeginTransaction(IsolationLevel.ReadCommitted);
                tx.Commit();
                tx.Rollback();
                tx.Dispose();
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("database facade compatibility markers should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"));
    assert!(diagnostics.contains("Entity Framework database/transaction member is still a compatibility surface"));
}

#[test]
fn warns_for_additional_noop_host_configuration_members() {
    let source = r#"
            using Glitching.AspNetCore;

            fn main() {
                WebApplication app = new WebApplication();
                app.UseCors();
                app.UseAuthentication();
                app.UseHttpsRedirection();
                app.UseRouting();
                app.UseEndpoints();
                app.MapControllers();
                app.Run();
                print(app != null);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("host configuration markers should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3013"));
    assert!(diagnostics.contains("ASP.NET-style host configuration member is still a no-op compatibility surface"));
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

