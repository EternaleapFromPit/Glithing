# Fix task cleanup ordering and async WhenAll liveness

## Goal Description

The repository had three failing tests related to async task cleanup and faulted-result handling:
1. `faulted_tasks_report_status_bits_in_llvm_and_native_output`
2. `runs_async_when_all_string_payloads_natively`
3. `task_result_faults_can_be_caught_natively`

Two of those are now fixed by moving task cleanup ahead of exception propagation and by treating fresh `Task` values as moves instead of shared stores. The remaining blocker is the async `WhenAll` string case: the generated async state currently drops the `tasks` array too early across the post-`await` continuation, so the method returns `null` instead of continuing to the final string concatenation.

## User Review Required

[!IMPORTANT]
- Confirm that moving task-handle cleanup ahead of exception propagation is acceptable. This changes runtime behavior for faulted `Task<T>` access paths and can affect any code that relies on delayed task destruction.
- Approve modifications to the LLVM task-result lowering paths and task ownership-on-store behavior. Those are already landed in the current branch.

## Open Questions

[!WARNING]
- Should we add a separate regression test for temporary `Task<T>.Exception` / `Task<T>.IsFaulted` access so this cleanup edge stays covered?
- The remaining async `WhenAll` failure likely needs a dedicated continuation/liveness slice rather than more task cleanup.

## Proposed Changes

---
### llvm/stmt_expr/expressions.rs
- Move task-handle cleanup ahead of the exception check in `Task<T>.Result` and `await Task<T>` lowering so cleanup still runs when the task faults.
- Keep the result-retain logic, but ensure the task handle is dropped even when control transfers to `catch`.
- Add the same temporary-drop ordering for task `Exception` / status-bit property access so temp task expressions do not leak.

---
### llvm/collections/basics.rs
- Apply the same cleanup ordering to `Task.GetResult()` / `Task.AsTask()` lowering.
- Keep successful result retention intact, but ensure the original task handle is destroyed even if the result access faults.
- Treat fresh `Task` values coming from `Task.Run`, `Task.FromResult`, and `Task.WhenAll` as move-only stores so they do not get over-retained on assignment.

---
### llvm/runtime.rs
- Verify that faulted task destruction still frees task payloads after the compiler-side cleanup ordering change.
- Keep the current task payload ownership model intact unless the cleanup change reveals a separate runtime bug.

---
### tests/async_and_basics.rs
- Re-run the three failing tests after the cleanup fix.
- Keep the `faulted_tasks_report_status_bits_in_llvm_and_native_output` and `task_result_faults_can_be_caught_natively` regressions.
- Add a dedicated async `WhenAll` continuation regression once the remaining liveness bug is fixed.

## Verification Plan

### Automated Tests
- Run `cargo test` to ensure the suite is green again.
- Specifically monitor the three previously failing tests for success.

### Manual Verification
- Inspect the generated LLVM IR for a faulted `Task<T>.Result` access to confirm cleanup happens before exception propagation.
- Run the native executable for the failing cases and verify it exits with status 0 and no leak count.
- Inspect the async `MergeAsync` IR separately; it still drops the `tasks` array before the post-`await` continuation, which is the current remaining blocker.

---
