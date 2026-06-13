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
- a concrete `Rc<int>` LLVM layout and drop path are implemented for the current ownership test slice
- xUnit test execution now runs through the Rust runtime path instead of the old native C runner
- the current README has been updated to describe the implemented subset, the LLVM path, and the remaining safety boundaries
- the stale `fixing_plan.md` has been removed

## Current implementation boundary

The compiler currently supports a practical subset of the language and runtime, but it is not a full C# implementation. The implemented path is enough to compile the current supported examples and run the current test suite, including the leak tests.

The current boundary is:

- native LLVM-backed compilation is the primary executable path
- C emission remains as a compatibility path
- `Rc<int>` and `sizeof(T)` are supported in the LLVM path where the current tests need them
- the runtime and standard library still need further work before broader ASP.NET Core or EF Core compatibility is realistic

## Next work items

1. Expand generic and ownership support beyond the current narrow `Rc<int>` case.
2. Improve framework compatibility for collections, tasks, delegates, and async lowering.
3. Add stronger borrow-check analysis across control-flow joins.
4. Replace any remaining compatibility stubs with real lowering or real diagnostics.
5. Tighten package/source linking and improve error reporting.
6. Keep the README and implementation notes synchronized with actual compiler behavior.

## Notes

This document intentionally avoids repeating stale claims from the analysis file that are already resolved in the current repo state. It is meant to be the live implementation plan going forward, not a historical bug list.
