# Fixing Plan: Glitching Compiler Stabilization

This document outlines the current diagnostic analysis of why compiling and linking `tests/xunit_memory/memory_leak_tests.gl` fails on the LLVM backend, and details the non-intrusive fixes required to stabilize the existing compiler without introducing new features.

---

## 1. What is Actually Happening?

When compiling `memory_leak_tests.gl` to a native executable via the LLVM backend (`--emit-exe`), the build pipeline fails at two distinct phases:

### Issue A: LLVM IR Syntax Validation Failure (Clang Frontend Error)
* **Symptom:** 
  ```
  error: clang failed: target/memory_leak_tests.exe.ll.tmp:652:20: error: use of undefined value '%exception_unwind'
  ```
* **Root Cause:** In [llvm.rs](file:///d:/Repos/Glitching/src/llvm.rs), when the compiler processes a lambda function, it temporarily registers `exception_unwind` as the active unwind label. If the lambda body contains any expression that can throw or propagate exceptions, the generated LLVM IR will branch to `%exception_unwind`. However, unlike regular functions (`emit_typed_function`), the lambda emitter (`emit_lambda_function`) forgets to print the actual label declaration block (`exception_unwind:\n  ret ptr null`) before closing the function body. Clang rejects the resulting malformed IR file.

### Issue B: Undefined Native Symbols (Linker Error)
* **Symptom:**
  ```
  lld-link: error: undefined symbol: Native_LeakDetector_Reset
  lld-link: error: undefined symbol: Native_LeakDetector_HasLeaks
  lld-link: error: undefined symbol: Native_LeakDetector_GetMallocCount
  lld-link: error: undefined symbol: Native_LeakDetector_GetFreeCount
  ```
* **Root Cause:** The tests rely on `System.LeakDetector` and `System.XUnit`, which utilize external C implementations for memory tracking and test running. The native implementation functions (like `Native_LeakDetector_Reset`) are written in `packages/System.XUnit/native/xunit_runner.c`. While Gllang compiles the `.gl` source files into LLVM IR, the compiler toolchain driver (`src/toolchain.rs`) never compiles or links these native `.c` package files, causing MSVC `lld-link` to fail with unresolved external symbols.

---

## 2. Stabilization Plan (No New Features)

To make compilation of the existing code stable and prevent regressions, the following minimal, targeted adjustments are proposed:

### Phase 1: Fix Lambda Exception Unwinding
* **Action:** Update `emit_lambda_function` in `src/llvm.rs` to append the exception unwind label block right before the closing brace of the function block (matching how it is done for regular functions in `emit_typed_function`).
* **Code Change:**
  ```rust
  self.body.push_str(&format!("{}:\n", self.current_unwind_label));
  self.terminated = false;
  self.body.push_str("  ret ptr null\n");
  self.body.push_str("}\n\n");
  ```

### Phase 2: Implement Automatic Package Native Code Linking
* **Action:** Automatically identify and link package-level native C files when compiling to a native executable.
* **Steps:**
  1. Add a helper function `find_package_native_sources(linked_source: &str) -> Vec<PathBuf>` in `src/linker.rs`. This scans the transitively imported package paths for any `.c` files in a `native/` subdirectory.
  2. Add a `native_sources: Vec<PathBuf>` field to `CompileOutput` in `src/lib.rs` and populate it using the linker helper.
  3. Update `emit_native_executable` in `src/toolchain.rs` to accept `native_sources: &[PathBuf]` and pass them to the `clang` invocation, resetting the type check using `-x none` before them so Clang compiles them as native C files.

---

## 3. Verification Plan

1. **Unit Tests:** Ensure all existing 63 cargo unit tests in `src/tests.rs` continue to pass successfully (`cargo test`).
2. **LLVM Output Verification:** Run `cargo run -- tests/xunit_memory/memory_leak_tests.gl --emit-llvm-ir target/memory_leak_tests.ll` and verify that the generated IR compiles cleanly.
3. **Executable Verification:** Run `cargo run -- tests/xunit_memory/memory_leak_tests.gl --emit-exe target/memory_leak_tests.exe` to check that compilation and linking complete without errors.
4. **Execution Verification:** Execute `target/memory_leak_tests.exe` directly to ensure the test runner executes and reports the status of the 6 memory leak test cases.
