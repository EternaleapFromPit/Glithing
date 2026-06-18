use super::*;

#[test]
fn resolves_generic_delegate_declarations_to_callable_function_types() {
    let source = r#"
            public delegate TResult Converter<T, TResult>(T value);

            fn Apply(Converter<int, int> converter, int value) {
                return converter(value);
            }

            fn main() {
                int result = Apply(x => x + 1, 41);
                print(result);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("generic delegate declaration should compile");
    let llvm_ir = compile_llvm_ir(source).expect("generic delegate declaration should lower to LLVM");

    let diagnostics = output.diagnostics.join("\n");
    assert!(!diagnostics.contains("warning GL3005"));
    assert!(llvm_ir.contains("define ptr @glitch_lambda_0"));
    assert!(llvm_ir.contains("%glitch.delegate = type { i64, ptr, ptr, ptr }"));
}

#[test]
fn resolves_namespace_qualified_delegate_declarations() {
    let source = r#"
            namespace Demo {
                public delegate TResult Converter<T, TResult>(T value);
            }

            fn Apply(Demo.Converter<int, int> converter, int value) {
                return converter(value);
            }

            fn main() {
                int result = Apply(x => x + 1, 41);
                print(result);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("qualified delegate declaration should compile");
    let llvm_ir = compile_llvm_ir(source).expect("qualified delegate declaration should lower to LLVM");

    let diagnostics = output.diagnostics.join("\n");
    assert!(!diagnostics.contains("warning GL3005"));
    assert!(llvm_ir.contains("define ptr @glitch_lambda_0"));
}

#[test]
fn compiles_named_delegate_types_in_c_path() {
    let source = r#"
            public delegate int Converter(int value);

            int Apply(Converter converter, int value) {
                return converter(value);
            }

            fn main() {
                int result = Apply(x => x + 1, 41);
                print(result);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("named delegate type should lower to LLVM");

    assert!(llvm_ir.contains("glitch_lambda_0"));
    assert!(llvm_ir.contains("Apply"));
}

#[test]
fn compiles_system_ownership_package_surface() {
    let source = r#"
            using System.Ownership;

            class Node {
                public Weak<Node> Parent;
            }

            fn main() {
                own<int> owned = make_owned(1);
                borrow<int> borrowed = make_borrow(owned);
                var parent = new Node();
                var child = new Node();
                child.Parent = new Weak<Node>(parent);
                Node target;
                child.Parent.TryGetTarget(out target);
                print(owned);
                print(borrowed);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("System.Ownership package surface should compile");
    let diagnostics = output.diagnostics.join("\n");
    assert!(!diagnostics.contains("warning GL3005"));
    assert!(!diagnostics.contains("warning GL3007"));
}

#[test]
fn tracks_explicit_ownership_wrappers_in_summary() {
    let source = r#"
            using System.Ownership;

            class Node {
                public int Value;
            }

            fn main() {
                own<Node> owned = make_owned(new Node());
                shared<Node> sharedNode = make_shared(new Node());
                borrow<Node> borrowed = make_borrow(sharedNode);
                view<Node> viewNode = make_view(sharedNode);
                print(owned.Value);
                print(sharedNode.Value);
                print(borrowed.Value);
                print(viewNode.Value);
            }
        "#;

    let summary = compile_ownership_summary(source)
        .expect("explicit ownership wrappers should be reflected in ownership summary");

    assert!(summary.contains("local owned: Owned Class(\"Node\")"));
    assert!(summary.contains("local sharedNode: Shared Class(\"Node\")"));
    assert!(summary.contains("local borrowed: Borrowed Class(\"Node\")"));
    assert!(summary.contains("local viewNode: View Class(\"Node\")"));
}

#[test]
fn detects_cycles_through_shared_wrappers() {
    let source = r#"
            using System.Ownership;

            class Node {
                public shared<Node> Next;
            }

            fn main() {
                var a = new Node();
                var b = new Node();
                a.Next = make_shared(b);
                b.Next = make_shared(a);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("shared-wrapper cycle sample should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3007"));
    assert!(diagnostics.contains("shared<Node> Next"));
}

#[test]
fn compiles_system_collections_generic_surface() {
    let source = r#"
            using System.Collections.Generic;

            struct Entry {
                public KeyValuePair<string, int> Pair;
            }

            fn main() {
                IReadOnlyCollection<int> collection = new List<int>();
                IReadOnlyList<int> list = new List<int>();
                IDictionary<string, int> map = new Dictionary<string, int>();
                KeyValuePair<string, int> pair = new KeyValuePair<string, int>("a", 1);
                print(collection.Count);
                print(list.Count);
                print(map.Count);
                print(pair.Value);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("System.Collections.Generic package surface should compile");
    let diagnostics = output.diagnostics.join("\n");
    assert!(!diagnostics.contains("warning GL3005"));
}

#[test]
fn compiles_dictionary_try_get_value_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                Dictionary<string, int> map = new Dictionary<string, int>();
                map.Add("a", 1);
                int value = 0;
                if (map.TryGetValue("a", out value)) {
                    print(value);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Dictionary.TryGetValue should lower to LLVM");

    assert!(llvm_ir.contains("dict_tryget_load"));
}

#[test]
fn compiles_readonly_dictionary_as_view_framework_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                IReadOnlyDictionary<string, int> map = new Dictionary<string, int>();
                print(map.Count);
                print(map.ContainsKey("a"));
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("IReadOnlyDictionary surface should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("warning GL3005"));
    assert!(!diagnostics.contains("warning GL3007"));
}

#[test]
fn rejects_borrowed_lambda_capture_that_could_outlive_source() {
    let source = r#"
            fn main() {
                int value = 1;
                ref int borrowed = borrow value;
                var f = x => x + borrowed;
                print(f(2));
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("borrowed lambda capture should be rejected");

    assert!(error.contains("ownership checker: lambda capture 'borrowed' may outlive borrowed/view source"));
}

#[test]
fn supports_sizeof_and_rc_int_without_remaining_compatibility_warnings() {
    let source = std::fs::read_to_string("tests/xunit_memory/memory_leak_tests.gl")
        .expect("should read memory_leak_tests.gl");

    let output = compile_source_with_options(&source, true, false)
        .expect("memory leak fixture should compile");
    let diagnostics = output.diagnostics.join("\n");

    assert!(!diagnostics.contains("warning GL3002 at 71:40"));
    assert!(!diagnostics.contains("function 'sizeof' has no linked GL or LLVM implementation"));
    assert!(!diagnostics.contains("warning GL3004"));
    assert!(!diagnostics.contains("type 'Rc_int' has no linked LLVM layout"));
}

#[test]
fn supports_rc_string_without_remaining_compatibility_warnings() {
    let source = r#"
            using System.Ownership;

            fn main() {
                var owned = new Rc<string>("Ada");
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("Rc<string> fixture should compile");
    let diagnostics = output.diagnostics.join("\n");
    let llvm_ir = compile_llvm_ir(source).expect("Rc<string> fixture should lower to LLVM");

    assert!(!diagnostics.contains("warning GL3004"));
    assert!(!diagnostics.contains("type 'Rc_string' has no linked LLVM layout"));
    assert!(llvm_ir.contains("%glitch.Rc_string = type { i64, ptr, ptr, i32 }"));
    assert!(llvm_ir.contains("call void @glitch_destroy_Rc_string"));
    assert!(llvm_ir.contains("call void @glitch_drop_Rc_string"));
}

#[test]
fn supports_nested_generic_rc_without_remaining_compatibility_warnings() {
    let source = r#"
            using System.Ownership;
            using System.Collections.Generic;

            fn main() {
                var owned = make_owned(new Rc<List<int>>(new List<int>()));
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("nested generic Rc fixture should compile");
    let diagnostics = output.diagnostics.join("\n");
    let llvm_ir = compile_llvm_ir(source).expect("nested generic Rc fixture should lower to LLVM");

    assert!(!diagnostics.contains("warning GL3004"));
    assert!(!diagnostics.contains("type 'Rc_List_int_' has no linked LLVM layout"));
    assert!(llvm_ir.contains("%glitch.Rc_List_int_ = type { i64, ptr, ptr, i32 }"));
    assert!(llvm_ir.contains("call void @glitch_destroy_Rc_List_int_"));
    assert!(llvm_ir.contains("define void @glitch_drop_Rc_List_int_"));
}

#[test]
fn lowers_rc_int_layout_in_llvm() {
    let source = std::fs::read_to_string("tests/xunit_memory/memory_leak_tests.gl")
        .expect("should read memory_leak_tests.gl");

    let llvm_ir = compile_llvm_ir(&source).expect("Rc<int> fixture should lower to LLVM");

    assert!(llvm_ir.contains("%glitch.Rc_int = type { i64, ptr, i32, i32 }"));
    assert!(llvm_ir.contains("getelementptr %glitch.Rc_int, ptr null, i32 1"));
    assert!(llvm_ir.contains("call void @glitch_destroy_Rc_int"));
    assert!(llvm_ir.contains("call void @glitch_drop_Rc_int"));
}

#[test]
fn warns_on_generic_placeholder_indexing_with_actionable_diagnostic() {
    let source = r#"
            class Box<T> {
                T Pick(T value) {
                    return value[0];
                }
            }

            fn main() {
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("generic placeholder indexing should still compile with a warning");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3008"));
    assert!(diagnostics.contains("generic placeholder"));
    assert!(diagnostics.contains("specialized method body"));
}

#[test]
fn lowers_concrete_generic_box_layout_and_instance_methods_in_llvm() {
    let source = r#"
            class Box<T> {
                public T Value;

                public Box(T value) {
                    this.Value = value;
                }

                public T Get() {
                    return this.Value;
                }
            }

            fn main() {
                Box<int> box = new Box<int>(42);
                print(box.Get());
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("concrete generic box should lower to LLVM");

    assert!(llvm_ir.contains("%glitch.Box_int___g0__t0 = type { i64, ptr, i32 }"));
    assert!(llvm_ir.contains("define void @Box__g1__t0_ctor__owner_int"));
    assert!(llvm_ir.contains("define i32 @Box__g1__t0_Get__g0__owner_int"));
    assert!(llvm_ir.contains("call void @Box__g1__t0_ctor__owner_int"));
    assert!(llvm_ir.contains("call i32 @Box__g1__t0_Get__g0__owner_int"));
}

#[test]
fn warns_on_lambda_without_executable_closure_lowering_in_non_llvm_path() {
    let source = r#"
            class Runner {
                public Func<int, int> Worker;

                public Runner(Func<int, int> worker) {
                    this.Worker = worker;
                }

                public int Run(int x) {
                    var f = this.Worker;
                    return f(x);
                }
            }

            fn main() {
                int factor = 3;
                var runner = new Runner(x => x * factor);
                int res = runner.Run(5);
                print(res);
            }
        "#;

    let output = compile_source_with_options(source, false, false)
        .expect("lambda sample should compile with a compatibility warning on the non-LLVM path");
    let diagnostics = output.diagnostics.join("\n");

    assert!(diagnostics.contains("warning GL3005"));
    assert!(diagnostics.contains("lambda has no executable LLVM closure or expression-tree lowering"));
}

#[test]
fn warns_with_route_specific_endpoint_rewrite_proposals() {
    let source = r#"
            using Glitching.AspNetCore;

            class CreateRequest {
                public List<string> Names;
            }

            [ApiController]
            [Route("/books")]
            class BooksController {
                [HttpPost]
                string Create([FromBody] CreateRequest request) {
                    return "created";
                }
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("unsupported endpoint binding should remain a warning");
    let diagnostics = output.diagnostics.join("\n");
    assert!(diagnostics.contains("warning GL3101"));
    assert!(diagnostics.contains("endpoint POST /books is discoverable"));
    assert!(diagnostics.contains("[FromBody] string body"));
    assert!(diagnostics.contains("JSON binder for Class(\"CreateRequest\")"));
}

#[test]
fn lowers_unqualified_instance_fields_to_this_in_llvm() {
    let source = r#"
            class Options {
                public int Value;

                public Options(int value) {
                    Value = value;
                }

                int Get() {
                    return Value;
                }
            }

            fn main() {
                Options options = new Options(42);
                print(options.Get());
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("unqualified instance fields should compile");

    assert!(llvm_ir.contains("define void @Options__g0__t"));
    assert!(llvm_ir.contains("define i32 @Options__g0__t"));
    assert!(llvm_ir.contains("getelementptr inbounds %glitch.Options__g0__t"));
}

#[test]
fn lowers_csharp_pattern_variable_binding_in_llvm() {
    let source = r#"
            class Entity {
                public int Id;
            }

            fn main() {
                Entity entity = new Entity { Id = 42 };
                if (entity is Entity typed && typed.Id == 42) {
                    print(typed.Id);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("pattern binding should compile");

    assert!(llvm_ir.contains("icmp ne ptr"));
    assert!(llvm_ir.contains("%glitch.Entity"));
    assert!(llvm_ir.contains("store ptr"));
}

#[test]
fn compiles_more_generic_collection_instantiations() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<string> names = new List<string>();
                names.Add("Ada");
                print(names[0]);
                print(names.Contains("Ada"));
                print(names.Count);

                List<long> ids = new List<long>();
                ids.Add(42);
                print(ids[0]);
                print(ids.Contains(42));

                Dictionary<string, string> map = new Dictionary<string, string>();
                map.Add("lang", "glitching");
                print(map["lang"]);
                print(map.Remove("lang"));
                print(map.Count);

                List<bool> flags = new List<bool>();
                flags.Add(true);
                print(flags[0]);
                print(flags.Contains(true));
                flags.Clear();
                print(flags.Count);

                List<double> ratios = new List<double>();
                ratios.Add(1.5);
                print(ratios[0]);
                print(ratios.Contains(1.5));

                Dictionary<string, long> longMap = new Dictionary<string, long>();
                longMap.Add("id", 42);
                print(longMap["id"]);
                print(longMap.ContainsKey("id"));

                Dictionary<string, bool> boolMap = new Dictionary<string, bool>();
                boolMap.Add("ok", true);
                print(boolMap["ok"]);

                Dictionary<string, double> doubleMap = new Dictionary<string, double>();
                doubleMap.Add("pi", 3.14);
                print(doubleMap["pi"]);
                print(doubleMap.Remove("pi"));
                doubleMap.Clear();
                print(doubleMap.Count);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("generic collection instantiations should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_linq_to_array_over_list_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);
                int[] copy = numbers.ToArray();
                print(copy.Length);
            }
        "#;

    let llvm = compile_llvm_ir(source).expect("LINQ ToArray over List<T> should lower to LLVM");

    assert!(llvm.contains("list_to_array_loop"));
}

#[test]
fn compiles_spread_collection_expression_to_list_surface() {
    let source = r#"
            using System.Collections.Generic;
            using System.Linq;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);
                List<int> copy = [.. numbers];
                print(copy.Count);
            }
        "#;

    let llvm =
        compile_llvm_ir(source).expect("spread collection expression should lower to LLVM");

    assert!(llvm.contains("ToList"));
    assert!(llvm.contains("%glitch.list"));
}

#[test]
fn compiles_system_linq_iterator_surface() {
    let source = r#"
            using System.Collections.Generic;
            using System.Linq;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);

                int count = Enumerable.Count(numbers);
                bool any = Enumerable.Any(numbers);
                int[] copy = Enumerable.ToArray(numbers);
                print(count);
                print(any);
                print(copy.Length);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System.Linq iterator surface should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_foreach_and_ienumerable_views() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);
                int total = 0;
                foreach (int value in numbers) {
                    total = total + value;
                }
                print(total);

                List<string> names = new List<string>();
                names.Add("Ada");
                IEnumerable<string> view = names;
                foreach (string name in view) {
                    print(name);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("foreach and IEnumerable<T> should lower");
    let bytecode = compile_bytecode(source).expect("foreach bytecode should compile");

    assert!(llvm_ir.contains("%glitch.list"));
    assert!(llvm_ir.contains("%glitch.delegate"));
    assert!(bytecode.contains("foreach_start value"));
    assert!(bytecode.contains("foreach_next name"));
}

#[test]
fn compiles_system_linq_ienumerable_surfaces() {
    let source = r#"
            using System.Collections.Generic;
            using System.Linq;

            fn main() {
                List<int> numbers = new List<int>();
                numbers.Add(1);
                numbers.Add(2);

                IEnumerable<int> view = numbers;
                print(Enumerable.Count(view));
                print(Enumerable.Any(view));
                print(Enumerable.First(view));

                List<int> copy = Enumerable.ToList(view);
                print(copy.Count);

                int[] values = Enumerable.ToArray(view);
                print(values.Length);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Enumerable IEnumerable<T> surface should lower");
    let bytecode =
        compile_bytecode(source).expect("Enumerable IEnumerable<T> surface should emit bytecode");

    assert!(llvm_ir.contains("%glitch.list"));
    assert!(bytecode.contains("foreach_start"));
    assert!(bytecode.contains("foreach_next"));
}

