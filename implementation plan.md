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
- borrow-check control-flow joins now preserve moved/borrowed state across `if`, `switch`, `try/finally`, and early-return paths
- xUnit test execution now runs through the Rust runtime path instead of the old native C runner
- the current README has been updated to describe the implemented subset, the LLVM path, and the remaining safety boundaries
- the stale `fixing_plan.md` has been removed
- linked package and workspace sources now carry file markers that let diagnostics report the originating file path for warnings and compatibility messages instead of only the concatenated buffer location

## Current implementation boundary

The compiler currently supports a practical subset of the language and runtime, but it is not a full C# implementation. The implemented path is enough to compile the current supported examples and run the current test suite, including the leak tests.

The current boundary is:

- native LLVM-backed compilation is the primary executable path
- the public C emission path has been removed
- `Rc<T>` and `sizeof(T)` are supported in the LLVM path where the current tests need them
- the runtime and standard library still need further work before broader ASP.NET Core or EF Core compatibility is realistic

## Next work items

1. Remove the public C emission path and make the default CLI output a native binary instead.
2. Redesign NuGet/package output so it no longer depends on generated C source.
3. Migrate C-oriented tests to assert LLVM IR, bytecode, diagnostics, or native output instead.
4. Improve framework compatibility for collections, tasks, delegates, and async lowering.
5. Add stronger borrow-check analysis across the remaining control-flow joins and loop exit edges.
6. Replace any remaining compatibility stubs with real lowering or real diagnostics.
7. Tighten package/source linking and improve error reporting.
8. Keep the README and implementation notes synchronized with actual compiler behavior.
9. Expand framework compatibility in small, test-driven slices where the runtime model already exists, and keep unsupported members on explicit diagnostics with rewrite guidance.
10. Add the missing Conduit integration-test fixture surface in small steps:
   - `AddLogging` and the `DbContextOptions`/`DbContextOptionsBuilder.Options` path used by `SliceFixture`
   - `IServiceScopeFactory` and scope resolution from `Microsoft.Extensions.DependencyInjection`
   - `DbContextOptionsBuilder.UseInMemoryDatabase`
   - `MediatR.IRequest`, `MediatR.IRequest<TResponse>`, `MediatR.IMediator`, and `Send`
   - `SingleOrDefaultAsync` for the current EF query surface
   - service registration and resolution helpers needed by `SliceFixture`

   **Tests:** compile a minimal fixture-style snippet that exercises the new DI, EF, and MediatR symbols without falling back to opaque compatibility warnings.

11. Add the Swagger/OpenAPI startup slice used by `Conduit.Program`:
   - `Microsoft.OpenApi.Models` types for `OpenApiInfo`, `OpenApiSecurityScheme`, `OpenApiReference`, and `OpenApiSecurityRequirement`
   - `AddSwaggerGen` with typed configuration callbacks for `AddSecurityDefinition`, `AddSecurityRequirement`, `SwaggerDoc`, `CustomSchemaIds`, `DocInclusionPredicate`, `TagActionsBy`, and `SupportNonNullableReferenceTypes`
   - `System.Array.Empty<T>()` and `InvalidOperationException` for the security and startup helper paths

   **Tests:** compile a minimal Swagger configuration snippet that exercises the OpenAPI models and callback surface without opaque fallback warnings.

12. Add the `Microsoft.EntityFrameworkCore.InMemory` provider slice:
   - `InMemoryDatabaseRoot`
   - `UseInMemoryDatabase` provider extension(s)
   - the minimal in-memory provider options surface needed for `DbContextOptionsBuilder`

   **Tests:** compile a minimal in-memory EF configuration snippet that uses the provider package surface directly and through the current Conduit fixture path.

13. Harden package extension-method support:
   - require an explicit `this` receiver on extension methods inside package extension blocks
   - keep extension methods separate from ordinary instance methods so instance members win resolution
   - resolve extension methods only from imported packages and report ambiguous calls clearly

   **Tests:** valid package extension call, missing `this` receiver rejected, ambiguous extension call rejected, and instance-method precedence over an imported extension method.

## Notes

This document intentionally avoids repeating stale claims from the analysis file that are already resolved in the current repo state. It is meant to be the live implementation plan going forward, not a historical bug list.
