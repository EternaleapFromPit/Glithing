# Async `WhenAll` Liveness and Continuation Ownership

## Status
Implemented and covered by regression tests.

## Summary
The async state-machine path now keeps `Task.WhenAll(...)` inputs and nested async-owned values alive across the post-`await` continuation, then drops them exactly once after the final result is consumed. The blocking worker-thread runtime model stays unchanged.

## Coverage
- `async Task<string>` with `Task.WhenAll(tasks)` and post-`await` string concatenation
- nested async parent/child ownership across `await`
- native execution with leak reporting enabled
- existing `Task.Result`, `Task.Run`, and `ValueTask<T>` behavior remains covered by the current suite

## Notes
- Owned values may cross suspension when the async gate can prove them safe.
- Borrowed and view values remain rejected across suspension points.
- Framework-fluent ASP.NET helpers were left untouched unless they were part of the async continuation path.
