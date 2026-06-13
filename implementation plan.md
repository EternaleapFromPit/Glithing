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
- C code generation
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
- `List<T>.ToArray()` is lowered in both the C and LLVM paths for the supported scalar list types, which makes the current LINQ `ToArray` package path usable for the existing subset
- `System.Linq` now has a stable non-delegate surface for `Count`, `Any`, `First`, `FirstOrDefault`, `ToList`, and `ToArray`; the delegate-based sequence operators remain outside the current compiler boundary
- `delegate` declarations are now first-class in the parser, typed IR, and C emitter, including generic instantiations such as `Predicate<int>` and namespace-qualified forms, and `ValueTask<T>.FromResult` lowers through the task runtime path
- borrow-check control-flow joins now preserve moved/borrowed state across `if`, `switch`, `try/finally`, and early-return paths
- xUnit test execution now runs through the Rust runtime path instead of the old native C runner
- the current README has been updated to describe the implemented subset, the LLVM path, and the remaining safety boundaries
- the stale `fixing_plan.md` has been removed

## Current implementation boundary

The compiler currently supports a practical subset of the language and runtime, but it is not a full C# implementation. The implemented path is enough to compile the current supported examples and run the current test suite, including the leak tests.

The current boundary is:

- native LLVM-backed compilation is the primary executable path
- C emission remains as a compatibility path
- `Rc<T>` and `sizeof(T)` are supported in the LLVM path where the current tests need them
- the runtime and standard library still need further work before broader ASP.NET Core or EF Core compatibility is realistic

## Next work items

1. Expand ownership support beyond generic `Rc<T>` into other shared graphs and framework types.
2. Improve framework compatibility for collections, tasks, delegates, and async lowering.
3. Add stronger borrow-check analysis across the remaining control-flow joins and loop exit edges.
4. Replace any remaining compatibility stubs with real lowering or real diagnostics.
5. Tighten package/source linking and improve error reporting.
6. Keep the README and implementation notes synchronized with actual compiler behavior.

## Notes

This document intentionally avoids repeating stale claims from the analysis file that are already resolved in the current repo state. It is meant to be the live implementation plan going forward, not a historical bug list.
