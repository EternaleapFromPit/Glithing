use super::*;

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
fn compiles_mediatr_send_infers_response_from_request_contract() {
    let source = r#"
            using MediatR;
            using Microsoft.Extensions.DependencyInjection;
            using System.Threading;
            using System.Threading.Tasks;

            class DemoRequest : IRequest<string> {}

            class DemoHandler : IRequestHandler<DemoRequest, string> {
                Task<string> Handle(DemoRequest request, CancellationToken cancellationToken) {
                    return new Task<string>();
                }
            }

            class DemoApp {
                Task<string> Run() {
                    var mediator = new Mediator(new ServiceCollection().BuildServiceProvider());
                    return mediator.Send(new DemoRequest());
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("mediator Send should infer the generic response from IRequest<T>");

    assert!(!llvm_ir.is_empty());
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
    assert!(llvm_ir.contains("glitch_task_when_all2"));
    assert!(llvm_ir.contains("glitch_task_is_completed"));
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

    let llvm_ir = compile_llvm_ir(source).expect("Task[] WhenAll should lower");
    assert!(llvm_ir.contains("glitch_task_when_all_array"));
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

    let llvm_ir = compile_llvm_ir(source).expect("Task<T>[] WhenAll should lower");
    assert!(llvm_ir.contains("glitch_task_when_all_array"));
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
    assert!(llvm_ir.contains("glitch_task_completed"));
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
    assert!(llvm_ir.contains("glitch_task_wait"));
    assert!(llvm_ir.contains("glitch_task_is_completed"));
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
fn task_from_result_retains_lvalue_string_payloads() {
    let source = r#"
            using System.Threading.Tasks;

            fn main() {
                string name = "Ada";
                Task<string> task = Task.FromResult(name);
                print(task.Result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("Task.FromResult should retain pointer-backed lvalue payloads");

    assert!(llvm_ir.contains("glitch_task_from_result_ptr"));
    assert!(llvm_ir.contains("glitch_string_retain"));
    assert!(llvm_ir.contains("glitch_string_release"));
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
    assert!(llvm_ir.contains("glitch_task_wait"));
    assert!(llvm_ir.contains("glitch_task_is_completed"));
}

#[test]
fn native_task_when_all_wait_surface_releases_tasks() {
    let source = r#"
            using System.Threading.Tasks;

            int First() {
                return 1;
            }

            int Second() {
                return 2;
            }

            fn main() {
                Task<int> first = Task.Run(First);
                Task<int> second = Task.Run(Second);
                Task merged = Task.WhenAll(first, second);
                merged.Wait();
                print(merged.IsCompletedSuccessfully);
                print(first.Result + second.Result);
            }
        "#;

    let output_exe = emit_native_executable_from_source("native-task-whenall", source);
    let output = run_native_executable_with_leak_report(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"));
    assert!(stdout.contains("3"));
    assert!(stdout.contains("0"));
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
fn runs_gl_xunit_sorting_fixture_directory_natively() {
    let output_exe = emit_native_executable_from_path("xunit-sorting", "examples/xunit_sorting");
    let output = run_native_executable_with_leak_report(&output_exe);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "status={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status.code()
    );
    assert!(stdout.contains("0"));
    assert!(stderr.contains("[xUnit] Completed: 4 passed, 0 failed."));
    assert!(stderr.contains("[xUnit] Memory tracking: 0 mallocs, 0 frees. Clean."));
    assert!(stderr.contains("BubbleSort_OrdersValues"));
    assert!(stderr.contains("BinarySearch_ReturnsMinusOneForMissingValue"));
}

#[test]
fn native_single_bubblesort_xunit_fact_is_leak_free() {
    let source = r#"
            using System.Collections.Generic;
            using Xunit;

            public static class NumberAlgorithms {
                public static List<int> Clone(List<int> source) {
                    List<int> copy = new List<int>();
                    int index = 0;
                    while (index < source.Count) {
                        copy.Add(source[index]);
                        index = index + 1;
                    }
                    return copy;
                }

                public static List<int> BubbleSort(List<int> source) {
                    List<int> values = Clone(source);
                    int outer = 0;
                    while (outer < values.Count) {
                        int inner = 0;
                        while (inner + 1 < values.Count - outer) {
                            if (values[inner] > values[inner + 1]) {
                                int temp = values[inner];
                                values[inner] = values[inner + 1];
                                values[inner + 1] = temp;
                            }
                            inner = inner + 1;
                        }
                        outer = outer + 1;
                    }
                    return values;
                }

                public static bool IsSorted(List<int> values) {
                    int index = 0;
                    while (index + 1 < values.Count) {
                        if (values[index] > values[index + 1]) {
                            return false;
                        }
                        index = index + 1;
                    }
                    return true;
                }
            }

            public class NumberAlgorithmsTests {
                private static List<int> BuildSample() {
                    List<int> values = new List<int>();
                    values.Add(5);
                    values.Add(1);
                    values.Add(4);
                    values.Add(2);
                    values.Add(3);
                    return values;
                }

                [Fact]
                public void BubbleSort_OrdersValues() {
                    List<int> values = BuildSample();
                    List<int> sorted = NumberAlgorithms.BubbleSort(values);

                    Assert.Equal(5, values[0]);
                    Assert.Equal(1, sorted[0]);
                    Assert.Equal(2, sorted[1]);
                    Assert.Equal(3, sorted[2]);
                    Assert.Equal(4, sorted[3]);
                    Assert.Equal(5, sorted[4]);
                    Assert.True(NumberAlgorithms.IsSorted(sorted));
                }
            }

            public static void main() {
                RunAllTests();
            }
        "#;

    let output_exe = emit_native_executable_from_source("xunit-bubblesort-one", source);
    let output = run_native_executable_with_leak_report(&output_exe);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "status={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status.code()
    );
    assert!(stderr.contains("[xUnit] Completed: 1 passed, 0 failed."));
}

#[test]
fn native_single_binarysearch_xunit_fact_is_leak_free() {
    let source = r#"
            using System.Collections.Generic;
            using Xunit;

            public static class NumberAlgorithms {
                public static List<int> Clone(List<int> source) {
                    List<int> copy = new List<int>();
                    int index = 0;
                    while (index < source.Count) {
                        copy.Add(source[index]);
                        index = index + 1;
                    }
                    return copy;
                }

                public static List<int> BubbleSort(List<int> source) {
                    List<int> values = Clone(source);
                    int outer = 0;
                    while (outer < values.Count) {
                        int inner = 0;
                        while (inner + 1 < values.Count - outer) {
                            if (values[inner] > values[inner + 1]) {
                                int temp = values[inner];
                                values[inner] = values[inner + 1];
                                values[inner + 1] = temp;
                            }
                            inner = inner + 1;
                        }
                        outer = outer + 1;
                    }
                    return values;
                }

                public static int BinarySearch(List<int> sorted, int value) {
                    int left = 0;
                    int right = sorted.Count - 1;
                    while (left <= right) {
                        int middle = (left + right) / 2;
                        int current = sorted[middle];
                        if (current == value) {
                            return middle;
                        }
                        if (current < value) {
                            left = middle + 1;
                        } else {
                            right = middle - 1;
                        }
                    }
                    return -1;
                }
            }

            public class NumberAlgorithmsTests {
                private static List<int> BuildSample() {
                    List<int> values = new List<int>();
                    values.Add(5);
                    values.Add(1);
                    values.Add(4);
                    values.Add(2);
                    values.Add(3);
                    return values;
                }

                [Fact]
                public void BinarySearch_FindsExistingValue() {
                    List<int> sorted = NumberAlgorithms.BubbleSort(BuildSample());
                    Assert.Equal(3, NumberAlgorithms.BinarySearch(sorted, 4));
                    Assert.Equal(0, NumberAlgorithms.BinarySearch(sorted, 1));
                    Assert.Equal(4, NumberAlgorithms.BinarySearch(sorted, 5));
                }
            }

            public static void main() {
                RunAllTests();
            }
        "#;

    let output_exe = emit_native_executable_from_source("xunit-binarysearch-one", source);
    let output = run_native_executable_with_leak_report(&output_exe);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "status={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status.code()
    );
    assert!(stderr.contains("[xUnit] Completed: 1 passed, 0 failed."));
}

#[test]
fn native_call_argument_temporary_collections_do_not_leak() {
    let source = r#"
            using System.Collections.Generic;

            List<int> Build() {
                List<int> values = new List<int>();
                values.Add(5);
                values.Add(1);
                return values;
            }

            List<int> Clone(List<int> source) {
                List<int> copy = new List<int>();
                int index = 0;
                while (index < source.Count) {
                    copy.Add(source[index]);
                    index = index + 1;
                }
                return copy;
            }

            fn main() {
                List<int> sorted = Clone(Build());
                print(sorted.Count);
            }
        "#;

    let output_exe = emit_native_executable_from_source("native-call-temp-collections", source);
    let output = run_native_executable_with_leak_report(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2"));
    assert!(stdout.contains("0"));
}

#[test]
fn native_lambda_call_arguments_release_after_invoke() {
    let source = r#"
            public delegate int Work();

            int Apply(Work work) {
                return work();
            }

            fn main() {
                int delta = 1;
                int answer = Apply(() => delta);
                print(answer);
            }
        "#;

    let output_exe = emit_native_executable_from_source("native-call-temp-lambda", source);
    let output = run_native_executable_with_leak_report(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0"));
}

#[test]
fn native_task_run_static_call_releases_delegate_temporary() {
    let source = r#"
            using System.Threading.Tasks;

            int Compute() {
                return 42;
            }

            fn main() {
                Task<int> task = Task.Run(Compute);
                print(task.Result);
            }
        "#;

    let output_exe = emit_native_executable_from_source("native-task-run-static", source);
    let output = run_native_executable_with_leak_report(&output_exe);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("42"));
    assert!(stdout.contains("0"));
}

#[test]
fn compiles_gl_xunit_runtime_surface_fixture_directory_through_llvm() {
    let llvm_ir = compile_llvm_ir_from_path("examples/xunit_runtime_surface")
        .expect("GL runtime-surface xUnit fixture directory should compile through LLVM");

    assert!(llvm_ir.contains("XUnit_AddTest"));
    assert!(llvm_ir.contains("ListAndEnumerator_SurfaceWorks"));
    assert!(llvm_ir.contains("glitch_lambda_0"));
    assert!(llvm_ir.contains("glitch_delegate_wrapper_RuntimeSurfaceComputeAnswer"));
    assert!(llvm_ir.contains("RunAllTests"));
}
