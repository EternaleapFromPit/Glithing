# Runtime Dispatch and Framework Completion

## Status
Generic monomorphization, constrained generic indexing, service-registration compat surface expansion, and async scheduler compat surfaces are complete. Slice 2 and 3 compat additions landed July 2026.

## Changes Landed (July 2026)

### Slice 2 — Compatibility surface audit (partial)
- AddAuthentication, AddAuthorization, AddHostedService, AddSingleton (all arities) on IServiceCollection / ServiceCollection now emit GL3013 (prescriptive compat warning) instead of GL3001 (unhelpful missing-member error).
- GL3001 is now suppressed when GL3013 already fired for the same method call, eliminating duplicate diagnostics.
- SynchronizationContext, TaskScheduler, CancellationTokenSource, CancellationToken construction now bypasses the GL3004 (no-layout) check; these are context-only opaque types.
- Tests added in `src/tests/compat_surfaces.rs`.

### Slice 3 — Async runtime breadth (partial)
- Task.ConfigureAwait(...), ValueTask.ConfigureAwait(...), Task.Delay(...) now emit GL3013 (async-scheduler compat surface) instead of GL3001.
- CancellationToken.ThrowIfCancellationRequested() and CancellationToken.IsCancellationRequested now emit GL3013 with prescriptive cancellation guidance.
- TaskScheduler.* and SynchronizationContext.* now emit GL3013 (scheduler compat) instead of GL3001.

### Slice 4 — Housekeeping
- Deleted temp files from repo root: 	emp_conduit.ll, 	emp_aspnet_socket_smoke.*, 	emp_config_probe.*, 	emp_delegate_probe.*.

## Summary
The compiler now has concrete generic layout specialization, specialized generic methods, and safe lowering for constrained placeholder indexing. What remains is the runtime and framework surface that still falls back to warnings or typed defaults: package dispatch, ASP.NET-style host behavior, async/runtime breadth, and the remaining EF/DI compatibility markers.

## Remaining Implementation Slices

1. **Generic interface and virtual dispatch on specialized package bodies**
   - Resolve interface-typed calls after generic specialization so package bodies can invoke the concrete implementation instead of falling back to placeholder behavior.
   - Keep inherited generic owners, controller thunks, and service dependencies wired to concrete layouts.
   - Preserve current owner-layout specialization and do not regress existing generic method emission.

   **Tests:** interface dispatch through loaded packages, inherited generic controller/service dispatch, and vtable/drop glue regressions for specialized owner types.

2. **Finish the remaining compatibility-only package surfaces**
   - Replace the warning-only or typed-default markers that still show up in diagnostics with real lowering where the runtime model already exists.
   - Focus on the surfaces already referenced by the current warnings: AddEndpointsApiExplorer, AddMemoryCache, AddHttpContextAccessor, AddOptions, AddLogging, AddApiVersioning, AddVersionedApiExplorer, AddAutoMapper, AddMediatR, AddValidatorsFromAssemblyContaining, AddRepositories, AddDataServices, Swagger/SwaggerUI configuration members, AddSerilog, ConfigureSerilog, UseStaticFiles, UseMiddleware, UseAuthentication, UseHttpsRedirection, UseRouting, UseEndpoints, MapControllers, Run, EnsureCreated, EnsureDeleted, Migrate, ExecuteSqlRaw, BeginTransaction, Commit, Rollback, Dispose, and FindAsync.
   - (Added July 2026) AddAuthentication, AddAuthorization, AddHostedService — DONE.
   - Keep unsupported members explicit: if a surface still lacks a runtime model, emit a source-specific diagnostic with rewrite guidance.

   **Tests:** each implemented surface stops emitting its compatibility warning, and each unsupported surface still emits a prescriptive diagnostic instead of a silent default.

3. **Expand async/runtime beyond the blocking worker-thread gate**
   - (Added July 2026) ConfigureAwait, Task.Delay, CancellationToken.*, TaskScheduler.*, SynchronizationContext.* — DONE (compat surfaces, prescriptive GL3013 instead of GL3001).
   - Add the remaining async/runtime pieces that are still explicitly called out as incomplete: real cancellation propagation, event-loop-based scheduling, and broader host scheduling behavior.
   - Keep the current blocking async state-machine lowering intact while extending runtime execution, not replacing it.
   - Preserve deterministic cleanup for tasks, delegates, and async state where the runtime already has ownership support.

   **Tests:** async cancellation coverage, runtime scheduling coverage, and regression tests for continuation cleanup and task payload ownership.

4. **Keep docs and acceptance gates synchronized**
   - Keep `README.md` and `implementation plan.md` aligned with actual compiler behavior.
   - Remove stale or already-finished work items when a slice lands.
   - Add or update smoke tests only where they cover a concrete remaining blocker.

   **Tests:** full `cargo test` after each slice, plus targeted smoke tests for any newly implemented runtime surface.
