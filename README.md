# GlitchC Prototype

This is a small compiler prototype for a C#-inspired systems language with Rust-like ownership checks.

Accepted source file extensions are `.gl` and `.cs`. The parser is the same for both; `.cs` is intended for C#-migration snippets.

Current implemented subset:

- `fn name() { ... }`
- C#-style free functions with typed parameters and return values, e.g. `int Add(int a, int b) { return a + b; }`
- top-level C#-style `using System;` and `using Alias = System.Name;` directives
- `let`, `let mut`, and C#-style `var`
- C#-style `int` and `long` local declarations
- `string` local declarations and string literals
- integer literals
- fixed-size arrays with `int[] xs = { 1, 2, 3 };`
- `new int[] { 1, 2, 3 }`
- array literals with `[1, 2, 3]`
- array indexing with `xs[0]`
- `struct` declarations with fields
- `ref struct` declarations with `ref int` / `ref long` fields
- `class` declarations lowered to owned heap allocations
- object initializers with `new Type { Field = value }`
- field access with `value.Field`
- `List<int>` with `new List<int>()`, `.Add(value)`, and indexing
- `Dictionary<string, int>` with `new Dictionary<string, int>()`, `.Add(key, value)`, and indexing
- `HashSet<string>` with `new HashSet<string>()`, `.Add(value)`, `.Contains(value)`, and `.Clear()`
- `System.Collections.Generic.List<int>` and `System.Collections.Generic.Dictionary<string, int>`, including `using` aliases
- C#-style `Thread` with `new Thread(worker)`, `.Start()`, and `.Join()`
- `System.Threading.Tasks.Task` with `Task.Run(worker)` and `task.Wait()`
- `System.Threading.Tasks.Task<T>` for `int`, `long`, and owned `string` results with `Task.Run(worker)`, `task.Result`, and `task.GetResult()`
- `System.Threading.Tasks.Task.IsCompleted`, `Task.WhenAll(Task[])`, and `ValueTask<T>.AsTask()`
- `System.Threading.Tasks.Task.WhenAll(Task<T>[])` for the supported task-array subset
- `System.Linq.Enumerable` now has working `IEnumerable<T>` overloads for `Count`, `Any`, `First`, `FirstOrDefault`, `ToList`, and `ToArray`
- `System.Array.Empty<T>()` is present on the current core-library surface, and `bool.Parse` / the `string` null helpers compile on the current path
- `int.Parse`, `int.TryParse`, `DateTime.Parse`, and `TimeSpan.FromMinutes` are present on the current `System` surface
- `System.IO.Path.GetExtension` and `System.IO.Path.GetFileName` are available on the current file-system surface
- `System.Text.Encoding.UTF8.GetBytes` is available on the current text-encoding surface
- `System.Type` now carries package-backed reflection metadata for `GetMethod`, `GetProperty`, `GetProperties`, `GetGenericArguments`, and `GetGenericTypeDefinition`, which is enough for the current package reflection slice and open generic checks such as `ICollection<>`
- compiler intrinsics such as `sizeof(T)` in the LLVM backend
- a concrete `Rc<int>` LLVM layout and drop glue for the current ownership test slice
- generated-regex partial methods for the slug helper pattern used by the ASP.NET sample
- variables
- mutable assignment, e.g. `x = x + 1;`
- `if` / `else` with scalar comparison conditions
- `while` loops
- C#-style `for` loops lowered through `while`
- `move x` with drop ownership transfer for moved values
- `borrow x`
- `borrow mut x`
- block scopes with scoped borrows and scoped drops
- `return;` and `return expr;` with scoped cleanup before function exit
- recursive drops for owned fields inside user-defined `struct` and `class` values
- owned `string` values with automatic cleanup
- owned values such as `string`, `List<int>`, classes, and structs containing owned fields require `move` instead of implicit copies
- `Task<string>.Result` transfers owned string results to the caller; task cleanup will not free a moved-out result
- `print(expr)`
- `+` on owned integers
- scalar comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=`
- typed IR to LLVM IR generation
- LLVM bitcode and native executable generation through Clang
- Rust static runtime linkage for the LLVM HTTP socket host
- reference-counted dynamic LLVM strings with deterministic release
- typed `try` / `catch` / `finally` exception propagation in LLVM
- a small ASP.NET-style helper surface, including `ModelState.AddModelError`, `ControllerBase.Ok`, and `ControllerBase.NotFound`
- simple configuration and model-builder defaults for the current ASP.NET / EF compatibility slice
- default CLI output builds a native executable when no explicit output is requested
- NuGet package emission produces LLVM-native assets and linked source metadata
- source-level packages with `package Name;` and `native "C source";`

`var x = value;` is accepted as a conversion-friendly spelling for `let mut x = value;`.

Supported C#-compatibility examples:

```csharp
using System;
using System.Collections.Generic;

int x = 10;
long y = x + 20;
int[] scores = { 7, 8, 9 };
var moreScores = new int[] { 10, 11, 12 };
print(scores[0]);
```

Larger supported example:

```csharp
struct Point {
    int X;
    int Y;
}

class Player {
    public string Name;
    public int Hp;
}

ref struct IntView {
    public ref int Data;
    public int Length;
}

fn main() {
    string name = "Ada";
    Point p = new Point { X = 2, Y = 3 };
    Player player = new Player { Name = "Ada", Hp = 100 };
    int x = 42;
    IntView view = new IntView { Data = borrow mut x, Length = 1 };

    List<int> xs = new List<int>();
    xs.Add(10);

    Dictionary<string, int> scores = new Dictionary<string, int>();
    scores.Add("hp", 100);

    print(name);
    print(p.X);
    print(player.Hp);
    print(view.Data);
    print(xs[0]);
    print(scores["hp"]);
}
```

Threading subset:

```csharp
fn worker() {
    print(42);
}

fn main() {
    Thread t = new Thread(worker);
    t.Start();
    t.Join();
}
```

`System.Collections.Generic` subset:

```csharp
using System.Collections.Generic;
using Collections = System.Collections.Generic;

fn main() {
    System.Collections.Generic.List<int> xs = new System.Collections.Generic.List<int>();
    xs.Add(10);

    Collections.Dictionary<string, int> scores = new Collections.Dictionary<string, int>();
    scores.Add("hp", 100);
}
```

`System.Threading.Tasks` subset:

```csharp
using System.Threading.Tasks;

fn worker() {
    print(42);
}

fn main() {
    Task task = Task.Run(worker);
    task.Wait();
}
```

Run:

```powershell
cargo run -- examples\ok.gl
cargo run -- examples\csharp_data_structures.gl
cargo run -- examples\ref_struct.gl
cargo run -- examples\thread.gl
cargo run -- examples\old_csharp.cs
cargo run -- examples\system_collections_generic.cs
cargo run -- examples\system_threading_tasks.cs
cargo run -- examples\scopes.gl
```

Emit LLVM IR:

```powershell
cargo run -- examples\llvm_simple.gl --emit-llvm-ir out.ll
```

Emit LLVM bitcode:

```powershell
cargo run -- examples\llvm_simple.gl --emit-llvm-bc out.bc
```

Build a native executable directly from LLVM IR:

```powershell
cargo run -- examples\llvm_simple.gl --emit-exe out.exe
.\out.exe
```

LLVM-only commands use the native LLVM toolchain. On Windows, the compiler searches
`PATH`, `GLITCH_LLVM_BIN`, and `C:\Program Files\LLVM\bin`. It also discovers installed
Visual Studio 2022 MSVC and Windows SDK libraries for native linking.

Run the native ASP.NET-like ownership load test:

```powershell
.\scripts\measure-aspnet-load.ps1
```

The test builds `examples/aspnet_load_test.cs`, performs 2,000,000 in-process routed
requests per run, and samples peak process memory. LLVM-generated allocations use counted
allocation wrappers. A native `main` exits with code `1` when tracked allocations remain
after deterministic cleanup and `0` when the tracked count returns to zero.

Build the one-request Rust HTTP host smoke test:

```powershell
cargo build
cargo run -- examples\aspnet_socket_smoke.gl --emit-exe target\aspnet-socket-smoke.exe
```

It serves `GET /health` on port `5099`, returns `{"status":"ok"}`, then exits. Set
`GLITCH_REPORT_LEAKS=1` to print the remaining tracked allocation count at process exit.

Emit a NuGet package:

```powershell
cargo run -- examples\old_csharp.cs --emit-nuget Glitching.Sample.0.1.0.nupkg --package-id Glitching.Sample --package-version 0.1.0
```

Build the standalone `System.Collections.Generic` package from Glitching-lang source:

```powershell
cargo run -- stdlib\System.Collections.Generic.gl --emit-nuget System.Collections.Generic.0.1.0.nupkg --package-version 0.1.0
```

Build the standalone `System.Threading.Tasks` package from Glitching-lang source:

```powershell
cargo run -- stdlib\System.Threading.Tasks.gl --emit-nuget System.Threading.Tasks.0.1.0.nupkg --package-version 0.1.0
```

Package emission writes LLVM IR plus linked source metadata into the `.nupkg`.

Expected rejected examples:

```powershell
cargo run -- examples\use_after_move.gl
cargo run -- examples\borrow_conflict.gl
```

This is intentionally a C#-compatibility subset, not full C#. The typed LLVM backend now
supports user class layouts, constructors, instance methods, fields, reference-counted class
ownership, and owned `List<T>` / `Dictionary<K,V>` buffers for the currently lowered scalar,
string, and pointer element types. It also has concrete LLVM lowering for `sizeof(T)` and the
current `Rc<int>` ownership/layout path used by the memory-leak tests. It does not use a tracing GC.

Important remaining safety boundaries:

- possible reference cycles produce warnings with `Weak<T>` rewrite proposals, but unchanged
  source can still accept cycle-leak risk;
- arrays, lists, and dictionaries release direct string/class elements, but recursively nested
  collection element graphs still need complete drop glue;
- dynamic LLVM strings use deterministic reference counting; the current allocation registry is
  single-threaded and must become synchronized before concurrent request execution;
- the socket host is implemented in the linked Rust runtime and is currently a synchronous
  HTTP/1.1 loop;
- C-only `native` package blocks remain legacy-backend-only;
- the full async scheduler, executable delegate middleware, generated controller routing, JSON
  object serialization, and EF query execution are still incomplete;
- the allocation counter covers LLVM allocations routed through `glitch_calloc`,
  `glitch_realloc`, and `glitch_free`; it is not a replacement for an external native memory
  sanitizer.

The cloned `Backend/Library` source tree currently parses, lowers, and links to a native
executable in the supported subset. Compatibility diagnostics still identify unresolved ASP.NET
Core, third-party, controller, and EF Core behavior; successful linking does not yet mean that
application routes or database operations are functionally equivalent to .NET.
