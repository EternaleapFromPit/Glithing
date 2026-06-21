# Implementation Plan

Source reviewed: `C:\Users\vpanf\.gemini\antigravity-ide\brain\b40cf037-a253-4f60-b088-a8df695c57a2\project_analysis.md`

## What from the analysis is still relevant

The analysis is broadly useful as a roadmap, but some of its claims are now stale. The parts that still matter are the remaining compiler and runtime gaps that block broader C#-style code and package compatibility:

- `delegate` declarations and delegate-based APIs
- nested classes and more complete type/member nesting
- branch-state merging in the borrow checker
- safer and more complete runtime handling for shared data
- package linking with better source mapping than naive concatenation
- NuGet/package emission that is structurally correct, not just C output with a package extension
- parser error recovery
- type aliases
- `static` semantics where they are expected to have real meaning
- better support for generic and framework-heavy code paths beyond the current subset

The analysis is also still relevant as a description of the compiler architecture:

- lexer -> parser -> AST
- borrow checking
- typed lowering / TIR
- LLVM emission
- bytecode emission
- diagnostics, leak analysis, cycle detection, package linking, and toolchain discovery

## What has already been implemented

The following items from the analysis are now implemented or superseded by later work in the repo:

- `lib.rs` has already been split into smaller modules
- prefix `--` / `++` and unary `-` support are implemented
- `extern` function declarations are supported
- division and modulo operators are implemented
- bitwise `&` support is implemented
- LLVM lowering for `sizeof(T)` is implemented
- generic `Rc<T>` LLVM layout and drop path are implemented for the current ownership slice, including nested instantiations such as `Rc<List<int>>`
- ownership-cycle diagnostics now look through owned wrappers like `List<T>` and `Task<T>` in addition to direct class fields
- task result access now retains string/object results and task drop paths release stored results for pointer-backed values
- `System.Ownership` now has a real source package surface with `Rc<T>`, `Weak<T>`, and ownership helper aliases instead of a placeholder stub
- `System.Ownership` now also exposes `shared<T>`, `borrow<T>`, `view<T>`, and `weakref<T>` aliases, and the compiler tracks those wrappers in ownership summaries, borrow checking, and cycle diagnostics
- `System.WeakReference` is available as a package surface for weak-reference-based framework code
- `List<T>.ToArray()` is lowered in the LLVM path for the supported scalar list types, which makes the current LINQ `ToArray` package path usable for the existing subset
- expanded `params` calls now lower on the LLVM path by packing trailing arguments into a typed array at the TIR boundary
- `System.Linq` now has a stable non-delegate surface for `Count`, `Any`, `First`, `FirstOrDefault`, `ToList`, and `ToArray`; the delegate-based sequence operators remain outside the current compiler boundary
- `System.Collections.Generic` now also recognizes `IReadOnlyDictionary<K,V>` as a view-style framework surface, and the compiler lowers it through the existing dictionary runtime path without treating it as owned
- `System.Threading.Tasks` now has a slightly richer surface with `ValueTask<T>.AsTask()` and a fixed-arity `Task.WhenAll` helper for the supported subset, in addition to `FromResult`
- `delegate` declarations are now first-class in the parser, typed IR, and LLVM pipeline, including generic instantiations such as `Predicate<int>` and namespace-qualified forms, and `ValueTask<T>.FromResult` lowers through the task runtime path
- the stale prototype package `packages/System.CSharp/System.CSharp.gl` has been removed; the actual frontend remains Rust-implemented compiler code, not a package-defined lexer/parser surface
- borrow-check control-flow joins now preserve moved/borrowed state across `if`, `switch`, `try/finally`, and early-return paths
- xUnit test execution now runs through the Rust runtime path instead of the old native C runner
- `using Xunit;` now resolves through a dedicated compatibility package, and the compiler auto-discovers `[Fact]` methods into native test registrations for C#-style test classes
- the current README has been updated to describe the implemented subset, the LLVM path, and the remaining safety boundaries
- the stale `fixing_plan.md` has been removed
- linked package and workspace sources now carry file markers that let diagnostics report the originating file path for warnings and compatibility messages instead of only the concatenated buffer location
- package declarations now default to public visibility unless explicitly marked otherwise, and cross-package access is checked explicitly in TIR instead of relying on the old “everything is reachable” behavior

## Current implementation boundary

The compiler currently supports a practical subset of the language and runtime, but it is not a full C# implementation. The implemented path is enough to compile the current supported examples and run the current test suite, including the leak tests.

The current boundary is:

- native LLVM-backed compilation is the primary executable path
- the public C emission path has been removed
- `Rc<T>` and `sizeof(T)` are supported in the LLVM path where the current tests need them
- async methods now lower to compiler-generated worker-thread state-machine entrypoints with hidden resume symbols and blocking `await` semantics over the native task runtime
- the current async gate supports local declarations, assignments, `if` / `else`, `return`, multiple sequential awaits, `try` / `catch` / `finally`, and the currently exercised loop shapes on the blocking worker-thread runtime
- the current async gate now includes `switch` in addition to the previously supported control-flow shapes, and it still rejects borrowed/view values that stay live across `await`
- the runtime and standard library still need further work before broader ASP.NET Core or EF Core compatibility is realistic
- the current regression suite is green, including the Conduit integration-tests project compile gate through LLVM

## Recently completed

- Blocking async state-machine lowering now exists for `async Task`, `async Task<T>`, `async ValueTask`, and `async ValueTask<T>` on the native LLVM path.
- The compiler emits explicit async state types plus hidden resume/destroy symbols and runs them on the existing worker-thread task runtime instead of treating async methods like ordinary synchronous functions.
- `await` now cleans up temporary child task handles after result extraction instead of leaking them on the LLVM path.
- New regression coverage checks emitted async state/resume symbols, nested async calls with sequential awaits, borrowed-local rejection across `await`, unsupported loop suspension diagnostics, and native async execution for `Task<int>` and `Task<string>`.
- The LLVM allocation registry and leak counter are now thread-safe for concurrent workloads by using atomic RMW updates in the runtime prelude and atomic leak-count reads in the exit path.
- A regression test now checks that the LLVM runtime emits atomic allocation-counter operations instead of plain load/store updates.
- Nested collection drop recursion now has direct regression coverage for `List<List<string>>`, confirming that recursive LLVM drop glue is preserved for nested owned collections.
- The `examples/aspnet_load_test.cs` workload now compiles through LLVM and is covered by a direct regression test.
- A native execution smoke test now runs `examples/llvm_simple.gl` end to end, proving the `emit-exe` path works for supported examples.
- Public C emission has been removed from the CLI surface, and the default output path now produces a native executable.
- NuGet/package emission now packages LLVM-native assets and linked source metadata instead of generated C source.
- Current regression tests cover the native-output path and the package payload shape.
- The Conduit fixture DI/EF/MediatR compile slice passes again.
- The Swagger/OpenAPI startup slice is implemented and covered by tests.
- The EF Core InMemory provider slice is implemented and covered by tests.
- `System.Threading.Tasks` now exposes `Task.IsCompleted` on the LLVM path, and the fixed-arity `Task.WhenAll` path remains green.
- Package extension-method support is implemented with explicit `this` receivers and ambiguity checks.
- The RealWorld sample now has package surfaces for `Microsoft.AspNetCore.Authorization`, `Microsoft.AspNetCore.Mvc`, `Microsoft.AspNetCore.Mvc.Filters`, `Microsoft.AspNetCore.Mvc.ApplicationModels`, `Microsoft.AspNetCore.Http`, and `Microsoft.AspNetCore.Builder`, which closes the first compile-time package gaps discovered while trying to build the sample.
- Open generic `typeof(IPipelineBehavior<,>)` style syntax now parses as an open-arity placeholder instead of failing immediately, which lets the sample's DI registrations reach semantic resolution.
- `System.Linq.Enumerable` now has working `IEnumerable<T>` overloads for `Count`, `Any`, `First`, `FirstOrDefault`, `ToList`, and `ToArray` instead of returning placeholder defaults.
- `System.Collections.Generic` and `System.Linq` now import `System` explicitly so shared exception helpers like `InvalidOperationException` resolve without false-positive layout warnings.
- `System.Array.Empty<T>()` is present on the current core-library surface, and `bool.Parse` plus the `string` null helpers now compile without breaking unrelated package loads.
- `int.Parse`, `int.TryParse`, `DateTime.Parse`, and `TimeSpan.FromMinutes` are now available on the current `System` surface without breaking unrelated package loads.
- `System.IO.Path.GetExtension` and `System.IO.Path.GetFileName` now resolve to real native helpers instead of placeholder values.
- `HashSet<T>` is now available as a package-backed collection surface built on the existing list runtime path.
- `System.Text.Encoding.UTF8.GetBytes` now compiles on the current text-encoding surface and handles null input explicitly.
- Generated regex partial methods used by the ASP.NET sample now synthesize a real `Regex` construction, and the `System.Text.RegularExpressions` surface has a working narrow replace helper for the slug use case.
- nullable value-type syntax now emits an explicit diagnostic instead of being silently treated as a reference-nullability hint
- `System.Type` now lowers as a real reflection token with `GetMethod`, `GetProperty`, `GetProperties`, `GetGenericArguments`, and `GetGenericTypeDefinition` support for the current package-backed reflection slice, including open generic names such as `ICollection<>`.
- `System.Type` ownership edges are modeled as views where needed so the reflection slice no longer trips cycle diagnostics on static metadata fields.
- Borrow-check joins now ignore early-return branches so safe post-`if` and post-`switch` code is not poisoned by a branch that exits the function.
- Loop-aware ownership joins now preserve the state at `break` / `continue` exits instead of flattening them into the loop base state, so loop-exiting branches no longer lose the ownership effects that occurred before the exit.
- The ASP.NET helper surface now records model-state errors and exposes concrete `Ok` / `NotFound` results instead of null placeholders.
- Missing-member diagnostics now give task-aware rewrite suggestions for async-style members instead of only a generic default-return hint.
- `Task.WhenAll(Task<T>[])` is now accepted on the LLVM path as part of the supported task-array slice.
- `ConfigurationManager.Get<T>()` / `GetValue<T>()` now return typed defaults, `ModelBuilder.Entity(string)` returns a builder object, and the EF `SaveChangesAsync()` helper now produces a real completed task.
- package-linker errors now include the source `using` line number that requested the missing package, which makes unresolved imports easier to localize.
- package linking now uses exact namespace/package declarations instead of substring matches, which prevents `System.*` packages from accidentally shadowing `System`.
- The README and this plan have been synchronized with the current compiler and package behavior.
- Native xUnit execution is now part of the example acceptance path for the stable sorting fixture, and the broader runtime-surface fixture is kept as a `.gl` source-level LLVM gate until the remaining task/runtime-accounting gaps are closed.
- The testing split is now explicit: Rust tests keep the compiler/product gates (`.csproj`, package linking, LLVM/native output, diagnostics), while broader semantic coverage moves into `.gl` / `.cs` fixtures that the compiler can compile directly and, where the runtime slice is stable, execute natively.
- Generic method inference now treats C#-style integer literals as `int` when they fit and `long` when they do not, which keeps concrete LLVM specializations aligned with default numeric literal behavior without retyping the whole expression pipeline.
- The supported collection runtime slice now has native execution coverage, including a direct `List`/`Dictionary` example and a larger collection workload that runs through the LLVM-native path with leak reporting enabled.
- Native `bool` printing on the LLVM executable path now emits `true` / `false` text through LLVM-side formatting instead of numeric `1` / `0`, while preserving output order with the existing `printf`-based runtime.
- The Rust socket-host runtime now joins spawned request-worker threads before returning from the host loop, and a direct runtime unit test covers multi-request handling on the current host path.
- Borrow/ownership loop-flow handling now distinguishes `break` from `continue` for `for` loops, so unreachable increment paths after `break` no longer poison state while `continue` still flows through the increment step.
- Known package stubs now surface explicit `GL3013` diagnostics for the current `ConfigurationManager.Get*` and AutoMapper `Map(...)` placeholder surfaces instead of silently looking implemented just because a package method body exists.
- The LLVM task/runtime slice now lowers `Task.WhenAll(...)`, `Task.Wait()`, and task completion-status checks through explicit runtime helpers instead of package no-ops, and there is native execution coverage for the `WhenAll + Wait + IsCompletedSuccessfully` path with leak reporting enabled.
- `Task.FromResult(...)` now retains pointer-backed lvalue payloads on the LLVM path before storing them into task results, which closes the obvious double-release/use-after-free edge for shared string/class-style sources in the current synchronous task model.
- `Task.Run(...)` now uses a Rust worker-thread runtime for the supported zero-argument delegate slice instead of invoking delegates inline in the LLVM backend, and `await` / `GetResult()` / `Wait()` now join that task handle before reading the result slot.
- The task runtime now owns delegate lifetimes across worker threads and destroys captured lambda environments after completion, with direct runtime coverage for background execution plus native compiler tests for delegate cleanup and leak reporting.
- ASP.NET-style endpoint thunks now unwrap `Task<string>` and `Task<class>` handler results through the native task runtime instead of treating route handlers as synchronous pointer returns only.
- The `Glitching.AspNetCore` package surface now accepts delegate-style `MapGet<T>` / `MapPost<T>` registrations for the current zero-argument handler slice, which lets async route handlers reach typed endpoint lowering.
- There is direct LLVM regression coverage for async route handlers, including `app.MapGet("/health", HealthAsync)` lowering through `glitch_task_get_result_ptr(...)`.
- The native smoke for compiler-generated `WebApplication_Handle` on that async route path now runs cleanly: the Rust host converts incoming request parts into real Glitch strings, the response path exits without host-side faults, and the temporary `ServiceProvider(\"\")` object-slot placeholder has been removed from the ASP.NET package surfaces.
- Attribute-routed async controller actions that read DI state through `this` now survive the native `app.Handle(...)` path: endpoint thunks keep controller instances alive through task-result extraction and response shaping before releasing them, and there is direct native regression coverage for the `async controller + ServiceProvider + app.Handle` crash shape.
- `WebApplicationBuilder.Build()` now constructs the returned app with the built `ServiceProvider` instead of patching it in after allocation, which keeps builder-style controller routes on the leak-free native path and removes the need for a manual `app.Services = ...` assignment in the current DI slice.
- Additional ASP.NET/DI placeholder surfaces now stop pretending to work silently: generic `UseMiddleware<T>()` and service-registration markers like `AddEndpointsApiExplorer()` / `AddMemoryCache()` emit explicit `GL3013` compatibility warnings with regression coverage instead of compiling as opaque no-ops.
- Authentication, Swagger option, and logging configuration marker chains now also emit explicit `GL3013` diagnostics instead of silently looking executable, including `AddAuthentication()` / `AddJwtBearer(...)`, `SwaggerDoc(...)` / `SwaggerEndpoint(...)`, and `ClearProviders()` / `AddSerilog(...)` / `ConfigureSerilog()`.
- Host-side native allocation helpers now participate in the same live-allocation accounting as LLVM-emitted allocations, which keeps request-owned Glitch strings from driving the leak counter negative on clean shutdown.
- `try` / `catch` / `finally` ownership flow now propagates `finally` effects into recorded loop-exit snapshots, so `break` / `continue` paths no longer skip cleanup or moves performed in `finally`.
- Additional package placeholders now produce explicit `GL3013` diagnostics for ASP.NET-style no-op host configuration markers such as `UseSwagger` / `UseSwaggerUI` / `UseStaticFiles`.
- Additional compatibility-only framework surfaces now also emit explicit `GL3013` diagnostics instead of looking runtime-ready: more ASP.NET host markers (`UseCors`, `UseAuthentication`, `UseMvc`, `UseHttpsRedirection`, `UseRouting`, `UseEndpoints`, `MapControllers`, `Run`) and EF database/transaction markers (`EnsureCreated`, `EnsureDeleted`, `Migrate`, `ExecuteSqlRaw`, `BeginTransaction`, `Commit`, `Rollback`, `Dispose`).
- Package type merging now keys on package id as well as name/namespace, which prevents package extension carrier types from accidentally merging with root types that only share the same short name.
- The native test harness now recovers cleanly from a transient runtime-library build failure by retrying the Rust runtime build and ignoring a poisoned in-process native-build mutex.
- `Dictionary<K,V>.GetEnumerator()` now lowers through a real LLVM snapshot-enumerator path instead of the old null placeholder, and there is regression coverage for both direct enumerator lowering and dictionary `foreach` lowering on the LLVM path.
- Dictionary enumeration is now covered end to end on the native path as well: both direct `IEnumerator<KeyValuePair<K,V>>.Current` access and `foreach` over `Dictionary<string,int>` execute correctly with leak reporting enabled.
- Generic `ServiceProvider.GetRequiredService<T>()` / `GetService<T>()` calls now have a real lowering slice for the currently supported cases (service-provider/self surfaces, scope-factory-style surfaces, and concrete-class activation), and the previous blanket compatibility warning for every generic lookup has been removed.
- Explicit DI registrations now affect LLVM-native service resolution for the current safe slice: `AddTransient<TService,TImplementation>()` and `AddScoped<TService,TImplementation>()` are tracked by the compiler, `GetRequiredService<T>()` prefers those registrations for interface lookup, and `GetService<T>()` now returns null/default for missing interface/class registrations instead of fabricating an object.
- Singleton DI resolution now has a real reuse slice on the native LLVM path: `AddSingleton<T>(value)` reuses the same registered instance on repeated `GetRequiredService<T>()` calls when the singleton source is a stable local/field expression, and there is native regression coverage for identity-preserving lookup.
- Singleton DI resolution now also accepts temporary constructor/object-initializer sources on the native LLVM path by hoisting them into hidden function-entry locals for deterministic reuse, and `WebApplicationBuilder.Build()` now propagates the builder's tracked service registrations into direct `app.Services.GetRequiredService<T>()` lookups.
- The blocking async gate now accepts `await` inside `switch` bodies and keeps the existing borrowed/view-across-suspension rejection in place.
- `.csproj` parsing no longer rejects ordinary `.NET` `TargetFramework` values like `net7.0`; Glitching now treats that metadata as informational instead of requiring a synthetic `gl*` target.
- ASP.NET controller discovery no longer depends on `[ApiController]` alone: route-attributed MVC controllers and `Controller` / `ControllerBase`-derived classes now reach endpoint collection and route lowering on the LLVM path.
- The native endpoint binder now handles nullable query primitives (`bool?`, `int?`, `long?`) and treats `CancellationToken` as a default request-scoped token handle on the current controller/runtime slice, with both LLVM and native regression coverage.
- Void-compatible startup/configuration lambdas now resolve through delegate-typed package members on the LLVM path, including statement-bodied and assignment-bodied configuration lambdas used by `AddDbContext`, `AddLocalization`, `AddSwaggerGen`, `AddMvc`, `AddJsonOptions`, and `AddJwtBearer`.
- C#-style static-class extension methods with `this` receivers are now parsed, registered, symbolized, and lowered consistently, including receiver-type dispatch through loaded package types such as `IServiceCollection` / `ServiceCollection`.

## Next work items

1. Extend the async/runtime slice from blocking worker-thread state machines to non-blocking scheduling and host integration beyond `Task.Run` threads.
2. Widen async lowering coverage to more control-flow shapes, starting with loops and exception regions once ownership/state restoration is explicit enough.
3. Harden nested collection drop glue so recursive owned graphs release correctly in all supported collection shapes.
4. Improve framework compatibility for collections, tasks, delegates, and async lowering beyond the current synchronous task surface.
5. Add stronger borrow-check analysis across any remaining control-flow joins that still need explicit tracking beyond the current loop-exit, early-return, and `finally`-propagation handling.
6. Replace any remaining compatibility stubs with real lowering or real diagnostics, especially in package surfaces that still return typed defaults or behave as no-op placeholders.
   - The next DI/runtime step after the current compiler-tracked singleton/transient/scoped slice is wider provider-state coverage across more scopes and controller-activation paths, plus reuse of constructed dependency graphs instead of only direct service registrations.
7. Expand framework compatibility in small, test-driven slices where the runtime model already exists, and keep unsupported members on explicit diagnostics with rewrite guidance.
8. Add additional sample/runtime acceptance work only where a concrete blocker remains after the current compile gates.
   - The next useful acceptance step is still a native ASP.NET-style host smoke path once the remaining async/runtime and package-helper gaps are closed.
9. Add explicit generic method type-argument support in method-call lowering, so calls like `GetRequiredService<ILoggerFactory>()` specialize their return type instead of degrading to `Unknown("T")`.

## C# Standard v7 Gap Analysis

The standard-v7 C# specification is still a useful target for the language front-end, but not every section is compatible with the current GC-free, memory-safe model.

### Still implementable without GC

These parts can still be widened with compiler/runtime work and do not require reintroducing tracing GC:

- nullable value types and lifted conversions
- boxing and unboxing for supported value types
- user-defined conversions
- anonymous function conversions, method group conversions, and delegate invocation
- pattern matching and switch-based flow analysis
- ref locals / ref-safe contexts / byref-friendly lowering
- async functions and iterator lowering into owned state machines
- arrays and array members
- namespaces, using directives, attributes, enums, structs, classes, interfaces, and most access modifiers
- `lock`/`using`-style runtime wrappers when backed by explicit runtime primitives
- collection, task, and framework slices where ownership can be proven or diagnostics can be made explicit

### Not implementable exactly in the current memory-safe model

These standard sections depend on raw memory access or GC-style runtime behavior, so they cannot be implemented with exact C# semantics without relaxing the model:

- unsafe code (§23), including pointer types, pointer indirection, pointer arithmetic, pointer comparisons, and pointer-member access
- the `fixed` statement and fixed/moveable-variable semantics
- fixed-size buffers as raw-pointer-backed storage
- exact raw-pointer forms of `stackalloc`
- GC-dependent runtime behaviors such as pinning, resurrection, finalization queues, and exact weak-reference / object-header semantics
- compatibility for APIs that assume those runtime behaviors, if they are required to match .NET exactly rather than approximated through diagnostics or alternate helpers

### Plan implication

The next safe expansion path is to continue implementing the language and framework slices listed above, while keeping the unsafe/GC-dependent sections as explicit diagnostics or limited compatibility shims instead of silent lowering.

The current RealWorld blocker is narrower now: the LLVM compile gate is green and controller routes are discovered, but native startup/serving still stops before a successful smoke response because broader host/runtime and package-backed framework behavior remains incomplete.

The nullable value-type and boxing/unboxing slice is currently implemented for the supported subset and covered by tests; further widening will be incremental rather than foundational.

### Unsupported syntax policy

The following common C# surfaces should be treated as hard diagnostics in the current memory-safe model:

- `unsafe` blocks and all pointer syntax
- `fixed` statements
- raw-pointer `stackalloc`
- fixed-size buffers
- finalizers that depend on GC reachability
- pinning and pin-sensitive interop patterns
- exact weak-reference / object-header semantics

Rewrite guidance should be specific and actionable:

- replace pointer code with owned arrays, `ref struct` views, or a narrow native helper
- replace `fixed` with a scoped copy or a `ref struct` view that does not escape
- replace raw-pointer `stackalloc` with a bounded owned buffer, or move the operation into a native helper
- replace finalizers with `Dispose` / `using`
- replace pinning assumptions with copy-based data flow
- replace exact weak-reference behavior with `Weak<T>`-style compatibility helpers or a diagnostic that exact .NET semantics are unavailable

## Completed

1. Monomorphize generic package methods by recording concrete call-site instantiations and specializing package bodies where the runtime model already exists.
   - Implemented by recording concrete instantiations from typed call sites, specializing matching package functions, and emitting concrete LLVM bodies for those specializations.
   - Covered by regression tests for instantiation collection and LLVM specialization output.
2. Generic placeholder indexing now emits an explicit compatibility warning instead of silently lowering to a typed default.
   - The diagnostics pass now reports GL3008 when a generic placeholder is indexed before specialization/runtime support exists.
   - Covered by a regression test that checks the warning text and rewrite guidance.
3. Loop-exit borrow-check joins now preserve moved state from `break` paths through nested control-flow joins.
   - The ownership checker now snapshots loop-exit state and merges it back into the post-loop state, so a move inside a `break` branch is no longer lost at the loop join.
   - Covered by a regression test for a moved value followed by `break` inside an `if` within a loop.
4. `System.String`/`string` prefix and equality helpers now lower through real LLVM string handling instead of returning typed defaults.
   - The LLVM emitter now treats the `System.String` package alias as string-like for indexing, length, concatenation, and retain/release paths.
   - `StartsWith` and `Equals` are implemented in the core package as real loop-based comparisons and covered by a regression test.
5. `System.String.Substring(int)` now lowers through a typed runtime helper instead of returning the original string placeholder.
   - The `System` package now declares `System_String_Substring_Native`, and the Rust runtime provides the allocation/copy helper used by LLVM-native executables.
   - Covered by a regression test that checks the LLVM IR contains the helper call and declaration.
6. `System.String.TrimEnd(...)` now lowers through a typed runtime helper instead of relying on an opaque compatibility fallback.
   - The `System` package now declares `System_String_TrimEnd_Native`, and the Rust runtime trims and copies the trailing slice into a fresh owned string.
   - Covered by a regression test that compiles the `TrimEnd('/').Substring(1)` path used by the existing string-surface fixture.
7. `System.String.ToLower()`, `System.String.ToLowerInvariant()`, `System.String.Replace(...)`, and `System.String.Trim()` now lower through typed runtime helpers instead of returning placeholders.
   - The `System` package now declares dedicated native helpers for these transforms, and the Rust runtime allocates fresh owned strings for each transformed result.
   - Covered by a regression test that checks the LLVM IR references the transform helpers.
8. `System.String.Split(string)` now lowers through a typed runtime helper instead of returning a placeholder array.
   - The `System` package now declares `System_String_Split_Native`, and the Rust runtime returns an owned `string[]` split on the first separator byte.
   - Covered by a regression test that exercises the current string split fixture path.
9. `System.String.Contains(string)` now lowers through a typed runtime helper instead of relying on an opaque fallback.
   - The `System` package now declares `System_String_Contains_Native`, and the Rust runtime checks for a substring match without introducing GC-managed state.
   - Covered by a regression test that checks the LLVM IR references the helper.
10. `System.String.TrimStart(...)` now lowers through a typed runtime helper, and `System.String.EndsWith(...)` is now implemented as a real suffix comparison instead of a placeholder.
   - The `System` package now declares `System_String_TrimStart_Native`, and the Rust runtime trims leading characters into a fresh owned string.
   - `EndsWith` now uses the same direct LLVM string indexing path as `StartsWith`, and is covered by a regression test.
11. `System.Array.Empty<T>()` now lowers through a dedicated native helper instead of depending on generic array state indexing in the package body.
   - The `System` package now declares `System_Array_Empty_Native`, and the Rust runtime returns a zero-length owned array node that the LLVM path can reuse across concrete instantiations.
   - Covered by a regression test that checks the LLVM IR references the helper.
12. Generic specialization now uses a fixed-point worklist and only lowers concrete instantiations.
   - The LLVM backend now collects newly discovered specializations from already-specialized bodies, which is the missing piece for transitive generic package calls.
   - It also skips unresolved placeholder instantiations instead of trying to monomorphize abstract `T` arguments directly.
   - Ownership-wrapper helpers such as `make_view<T>` remain emitted as template-compatible fallbacks for the current package surface, so the compiler still supports the existing generic collection and xUnit slices while moving toward stricter monomorphization.
   - Covered by a regression test where `Outer<int>` pulls in `Inner<int>` from its own body.
13. Borrow-check flow now treats `break` / `continue` as terminating the current path and preserves loop-exit snapshots across nested joins.
14. Concrete generic owner types now survive TIR lowering and emit specialized LLVM layouts plus owner-specialized constructor/method bodies.
   - User-defined generic object types such as `Box<int>` are no longer collapsed back to the template type during TIR signature and field resolution.
   - The LLVM backend now collects concrete generic object instantiations, registers concrete object layouts, and redirects constructor/instance-method emission to owner-specialized symbols.
   - Existing `Rc<T>` / `WeakReference<T>` compatibility lowering remains intact, including nested `Rc<List<int>>` coverage and weak-reference leak-analysis exemptions.
   - Covered by a regression test for concrete `Box<int>` layout + instance-method lowering, with the full suite green after the compatibility regressions were fixed.
   - The borrow checker now stops analyzing unreachable statements after `break` / `continue` and carries loop-exit state through enclosing joins instead of pretending the path is still live.
   - Covered by a regression test that keeps unreachable code after `break` from poisoning the loop state, plus the existing break-branch regression.
14. The task-like package surface now exposes the common awaiter/result helpers directly, and those awaiter methods move the underlying wrapper instead of borrowing it unsafely.
   - `Task`, `Task<T>`, `ValueTask`, and `ValueTask<T>` now expose `GetAwaiter()` and `GetResult()` where the current async/task lowering expects them.
   - Covered by regression tests that compile direct package-surface calls to `GetResult()` and `GetAwaiter().GetResult()` on `Task<T>` and `ValueTask<T>`.
15. `List<T>.GetEnumerator()` now returns a real package enumerator backed by a borrowed view of the list.
   - The enumerator uses the existing ownership wrappers so `foreach`-style traversal can read the live list without taking ownership of it.
   - Covered by a regression test that calls `GetEnumerator()`, `MoveNext()`, and `Current` directly on a concrete list.
16. The compatibility analyzer no longer flags the package-defined list enumerator as an opaque heap layout when it appears in the current collection surface.
   - This keeps the new enumerator slice from re-triggering the generic-layout warning path used for genuine unknown allocations.
   - Covered indirectly by the existing `Rc<T>` compatibility regression tests remaining warning-free after the collection update.
16. Nullable value types now produce an explicit warning with rewrite guidance instead of being silently erased to the underlying reference-or-scalar type.
   - The analyzer now catches `T?` when `T` is a value type and points at the `Nullable<T>` / lifted-conversion gap directly.
   - Covered by a regression test that asserts the warning for `Point?`.
17. Reference cycles over owned graphs are now treated as a diagnostic boundary rather than an automatic safety guarantee.
   - The cycle checker reports `GL3007` with a source-specific `Weak<T>` rewrite hint when it finds an ownership loop.
   - The policy is explicit that arbitrary cyclic ownership is not automatically leak-free in the current memory-safe model.
18. Ownership-wrapper helpers such as `make_view<T>` now lower as compiler intrinsics at the call site instead of relying on a special generic-specialization path.
   - This keeps the current collection and xUnit package surfaces working while reducing the number of generic helper bodies that need monomorphization.
   - Covered by the existing collection, `Rc<T>`, and xUnit regression tests that exercise `make_view<T>` transitively.
19. Generic methods on concrete generic owner types now monomorphize through the LLVM worklist, including transitive calls between owner-specialized generic methods.
   - The generic-specialization pass now tracks owner-specialized generic method templates separately from the base method definitions and discovers instantiations from real typed call sites instead of relying only on the older symbol-only instantiation list.
   - Generic `this` types on generic owners are now represented as `Owner<T...>` placeholders in TIR and rewritten through owner specialization, which keeps owner-aware method dispatch working inside specialized bodies.
   - Field-target ownership checking now falls back through monomorphized generic owner names so constructor/primary-constructor assignment checks keep working on framework-heavy generic types.
   - Covered by regression tests for direct and transitive `Box<int>.Method<string>(...)` specialization, with the full suite green after the Conduit compile-gate regression was fixed.

## Notes

This document intentionally avoids repeating stale claims from the analysis file that are already resolved in the current repo state. It is meant to be the live implementation plan going forward, not a historical bug list.
