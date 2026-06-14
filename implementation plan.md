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
- `using Xunit;` now resolves through a dedicated compatibility package, and the compiler auto-discovers `[Fact]` methods into native test registrations for C#-style test classes
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

## Recently completed

- Public C emission has been removed from the CLI surface, and the default output path now produces a native executable.
- NuGet/package emission now packages LLVM-native assets and linked source metadata instead of generated C source.
- Current regression tests cover the native-output path and the package payload shape.
- The Conduit fixture DI/EF/MediatR compile slice passes again.
- The Swagger/OpenAPI startup slice is implemented and covered by tests.
- The EF Core InMemory provider slice is implemented and covered by tests.
- `System.Threading.Tasks` now exposes `Task.IsCompleted` on the LLVM path, and the fixed-arity `Task.WhenAll` path remains green.
- Package extension-method support is implemented with explicit `this` receivers and ambiguity checks.
- `System.Linq.Enumerable` now has working `IEnumerable<T>` overloads for `Count`, `Any`, `First`, `FirstOrDefault`, `ToList`, and `ToArray` instead of returning placeholder defaults.
- `System.Array.Empty<T>()` is present on the current core-library surface, and `bool.Parse` plus the `string` null helpers now compile without breaking unrelated package loads.
- `int.Parse`, `int.TryParse`, `DateTime.Parse`, and `TimeSpan.FromMinutes` are now available on the current `System` surface without breaking unrelated package loads.
- `System.IO.Path.GetExtension` and `System.IO.Path.GetFileName` now resolve to real native helpers instead of placeholder values.
- `HashSet<T>` is now available as a package-backed collection surface built on the existing list runtime path.
- `System.Text.Encoding.UTF8.GetBytes` now compiles on the current text-encoding surface and handles null input explicitly.
- Generated regex partial methods used by the ASP.NET sample now synthesize a real `Regex` construction, and the `System.Text.RegularExpressions` surface has a working narrow replace helper for the slug use case.
- `System.Type` now lowers as a real reflection token with `GetMethod`, `GetProperty`, `GetProperties`, `GetGenericArguments`, and `GetGenericTypeDefinition` support for the current package-backed reflection slice, including open generic names such as `ICollection<>`.
- Borrow-check joins now ignore early-return branches so safe post-`if` and post-`switch` code is not poisoned by a branch that exits the function.
- Loop-aware ownership joins now preserve the state at `break` / `continue` exits instead of flattening them into the loop base state, so loop-exiting branches no longer lose the ownership effects that occurred before the exit.
- The ASP.NET helper surface now records model-state errors and exposes concrete `Ok` / `NotFound` results instead of null placeholders.
- Missing-member diagnostics now give task-aware rewrite suggestions for async-style members instead of only a generic default-return hint.
- `Task.WhenAll(Task<T>[])` is now accepted on the LLVM path as part of the supported task-array slice.
- `ConfigurationManager.Get<T>()` / `GetValue<T>()` now return typed defaults, `ModelBuilder.Entity(string)` returns a builder object, and the EF `SaveChangesAsync()` helper now produces a real completed task.
- package-linker errors now include the source `using` line number that requested the missing package, which makes unresolved imports easier to localize.
- The README and this plan have been synchronized with the current compiler and package behavior.

## Next work items

1. Improve framework compatibility for collections, tasks, delegates, and async lowering.
2. Add stronger borrow-check analysis across any remaining control-flow joins that still need explicit tracking beyond the current loop-exit and early-return handling.
3. Replace any remaining compatibility stubs with real lowering or real diagnostics, especially in package surfaces that still return typed defaults as placeholders.
4. Expand framework compatibility in small, test-driven slices where the runtime model already exists, and keep unsupported members on explicit diagnostics with rewrite guidance.
5. Add additional sample/runtime acceptance work only where a concrete blocker remains after the current compile gates.

## Notes

This document intentionally avoids repeating stale claims from the analysis file that are already resolved in the current repo state. It is meant to be the live implementation plan going forward, not a historical bug list.
