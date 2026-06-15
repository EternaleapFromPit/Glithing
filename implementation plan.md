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
- the current regression suite is green, including the Conduit integration-tests project compile gate through LLVM

## Recently completed

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

## Next work items

1. Improve framework compatibility for collections, tasks, delegates, and async lowering.
2. Add stronger borrow-check analysis across any remaining control-flow joins that still need explicit tracking beyond the current loop-exit and early-return handling.
3. Replace any remaining compatibility stubs with real lowering or real diagnostics, especially in package surfaces that still return typed defaults as placeholders.
4. Expand framework compatibility in small, test-driven slices where the runtime model already exists, and keep unsupported members on explicit diagnostics with rewrite guidance.
5. Add additional sample/runtime acceptance work only where a concrete blocker remains after the current compile gates.

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

The current RealWorld blocker is narrower now: the LLVM compile gate is green, so the next acceptance step is native entrypoint synthesis and smoke execution rather than more package import surgery.

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
12. Borrow-check flow now treats `break` / `continue` as terminating the current path and preserves loop-exit snapshots across nested joins.
   - The borrow checker now stops analyzing unreachable statements after `break` / `continue` and carries loop-exit state through enclosing joins instead of pretending the path is still live.
   - Covered by a regression test that keeps unreachable code after `break` from poisoning the loop state, plus the existing break-branch regression.
13. The task-like package surface now exposes the common awaiter/result helpers directly, and those awaiter methods move the underlying wrapper instead of borrowing it unsafely.
   - `Task`, `Task<T>`, `ValueTask`, and `ValueTask<T>` now expose `GetAwaiter()` and `GetResult()` where the current async/task lowering expects them.
   - Covered by regression tests that compile direct package-surface calls to `GetResult()` and `GetAwaiter().GetResult()` on `Task<T>` and `ValueTask<T>`.
14. `List<T>.GetEnumerator()` now returns a real package enumerator backed by a borrowed view of the list.
   - The enumerator uses the existing ownership wrappers so `foreach`-style traversal can read the live list without taking ownership of it.
   - Covered by a regression test that calls `GetEnumerator()`, `MoveNext()`, and `Current` directly on a concrete list.
15. The compatibility analyzer no longer flags the package-defined list enumerator as an opaque heap layout when it appears in the current collection surface.
   - This keeps the new enumerator slice from re-triggering the generic-layout warning path used for genuine unknown allocations.
   - Covered indirectly by the existing `Rc<T>` compatibility regression tests remaining warning-free after the collection update.
16. Nullable value types now produce an explicit warning with rewrite guidance instead of being silently erased to the underlying reference-or-scalar type.
   - The analyzer now catches `T?` when `T` is a value type and points at the `Nullable<T>` / lifted-conversion gap directly.
   - Covered by a regression test that asserts the warning for `Point?`.

## Notes

This document intentionally avoids repeating stale claims from the analysis file that are already resolved in the current repo state. It is meant to be the live implementation plan going forward, not a historical bug list.
