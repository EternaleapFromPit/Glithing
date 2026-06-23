use super::*;

#[test]
fn compiles_system_text_json_package_helpers() {
    let source = r#"
            using System.Text.Json;

            fn main() {
                string value = "hello";
                string json = JsonSerializer_SerializeString(value);
                string plain = JsonSerializer_DeserializeString(json);
                print(json);
                print(plain);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System.Text.Json package should lower to LLVM");
    let report = compile_leak_report(source).expect("JSON leak report should compile");

    assert!(llvm_ir.contains("JsonSerializer_SerializeString"));
    assert!(llvm_ir.contains("JsonSerializer_DeserializeString"));
    assert!(report.contains("No obvious owned temporary leaks detected."));
}

#[test]
fn compiles_system_text_encoding_getbytes() {
    let source = r#"
            using System.Text;

            fn main() {
                Encoding utf8 = Encoding.UTF8;
                byte[] bytes = utf8.GetBytes("abc");
                print(bytes.Length);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("Encoding.GetBytes should compile");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_system_parse_and_timespan_helpers() {
    let source = r#"
            using System;

            fn main() {
                int number = int.Parse("42");
                int parsed = 0;
                bool ok = int.TryParse("7", out parsed);
                DateTime value = DateTime.Parse("90");
                TimeSpan span = TimeSpan.FromMinutes(15.5);
                print(number);
                print(parsed);
                print(ok);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("parse and timespan helpers should compile");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_system_array_empty_and_bool_parse_surface() {
    let source = r#"
            using System;

            fn main() {
                string[] empty = Array.Empty<string>();
                print(empty.Length);
                print(bool.Parse("true"));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System.Array.Empty and bool.Parse should lower");

    assert!(llvm_ir.contains("System_Array_Empty_Native"));
    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_system_string_null_helpers() {
    let source = r#"
            using System;

            fn main() {
                print(string.IsNullOrEmpty(""));
                print(string.IsNullOrWhiteSpace(""));
                print(String.IsNullOrEmpty("glitch"));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System string null helpers should lower");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_string_startswith_and_equals_helpers() {
    let source = r#"
            using System;

            fn main() {
                string value = "glitch";
                print(value.StartsWith("gli", StringComparison.Ordinal));
                print(value.Equals("glitch", StringComparison.Ordinal));
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("string StartsWith and Equals helpers should lower");

    assert!(llvm_ir.contains("glitch_string_length"));
    assert!(llvm_ir.contains("icmp"));
}

#[test]
fn compiles_string_substring_helper() {
    let source = r#"
            using System;

            fn main() {
                string value = "glitch";
                print(value.Substring(2));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string Substring helper should lower");

    assert!(llvm_ir.contains("System_String_Substring_Native"));
    assert!(llvm_ir.contains("declare ptr @System_String_Substring_Native"));
}

#[test]
fn compiles_string_trimend_helper() {
    let source = r#"
            using System;

            fn main() {
                string filterQuery = ">5~10/";
                string path = filterQuery.TrimEnd('/').Substring(1);
                print(path);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string TrimEnd helper should lower");

    assert!(llvm_ir.contains("System_String_TrimEnd_Native"));
    assert!(llvm_ir.contains("declare ptr @System_String_TrimEnd_Native"));
}

#[test]
fn compiles_string_transform_helpers() {
    let source = r#"
            using System;

            fn main() {
                string value = "  Glitch  ";
                string lowered = value.ToLowerInvariant();
                string replaced = lowered.Replace("g", "h");
                string trimmed = replaced.Trim();
                print(lowered);
                print(replaced);
                print(trimmed);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string transform helpers should lower");

    assert!(llvm_ir.contains("System_String_ToLowerInvariant_Native"));
    assert!(llvm_ir.contains("System_String_Replace_Native"));
    assert!(llvm_ir.contains("System_String_Trim_Native"));
}

#[test]
fn compiles_string_split_helper() {
    let source = r#"
            using System;

            fn main() {
                string filterQuery = ">5~10";
                var parts = filterQuery.Split("~");
                string first = parts[0];
                print(first);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string Split helper should lower");

    assert!(llvm_ir.contains("System_String_Split_Native"));
}

#[test]
fn compiles_string_contains_helper() {
    let source = r#"
            using System;

            fn main() {
                string filterQuery = ">5~10";
                print(filterQuery.Contains("~"));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string Contains helper should lower");

    assert!(llvm_ir.contains("System_String_Contains_Native"));
}

#[test]
fn compiles_string_trimstart_and_endswith_helpers() {
    let source = r#"
            using System;

            fn main() {
                string value = "   glitch/";
                string trimmed = value.TrimStart();
                print(trimmed.EndsWith("/", StringComparison.Ordinal));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("string TrimStart and EndsWith helpers should lower");

    assert!(llvm_ir.contains("System_String_TrimStart_Native"));
    assert!(llvm_ir.contains("EndsWith"));
}

#[test]
fn emits_llvm_native_nuget_package_assets() {
    let source = r#"
            using System.Text.Json;

            fn main() {
                string value = "hello";
                string json = JsonSerializer_SerializeString(value);
                print(json);
            }
        "#;

    let output = compile_source_with_options(source, true, false)
        .expect("package emission fixture should compile");
    let llvm_ir = output.llvm_ir().expect("LLVM IR should be present");
    let package_path = crate::toolchain::native_host_temp_dir().join(format!(
        "glitching-package-{}.nupkg",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&package_path);

    emit_nuget_package(
        NugetPackageSpec {
            package_id: "Glitching.Sample",
            version: "0.1.0",
            linked_source: &output.linked_source,
            llvm_ir,
            dependencies: &[],
            content_files: &[],
        },
        package_path
            .to_str()
            .expect("package path should be valid UTF-8"),
    )
    .expect("NuGet package should emit");

    let bytes = std::fs::read(&package_path).expect("NuGet package should exist");
    let package_text = String::from_utf8_lossy(&bytes);
    assert!(package_text.contains("Glitching.Sample.nuspec"));
    assert!(package_text.contains("build/native/Glitching.Sample.ll"));
    assert!(package_text.contains("contentFiles/any/any/Glitching.Sample.gl"));
    assert!(!package_text.contains("build/native/Glitching.Sample.c"));
}

#[test]
fn compiles_system_io_file_operations() {
    let source = r#"
            using System.IO;

            fn main() {
                string path = "test_file.txt";
                File.WriteAllText(path, "Hello from Glitching!");
                string content = File.ReadAllText(path);
                print(content);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("System.IO.File operations should compile to LLVM IR");
    assert!(llvm_ir.contains("System_IO_File_WriteAllText"));
    assert!(llvm_ir.contains("System_IO_File_ReadAllText"));
}

#[test]
fn compiles_system_io_path_helpers() {
    let source = r#"
            using System.IO;

            fn main() {
                string path = "assets/images/logo.png";
                print(Path.GetFileName(path));
                print(Path.GetExtension(path));
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("System.IO.Path helpers should compile");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_collections_smoke() {
    let source = r#"
            fn main() {
                List<int> xs = new List<int>();
                xs.Add(10);
                print(xs[0]);

                Dictionary<string, int> scores = new Dictionary<string, int>();
                scores.Add("hp", 100);
                print(scores["hp"]);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("collections should lower to LLVM");

    assert!(llvm_ir.contains("%glitch.list = type"));
    assert!(llvm_ir.contains("%glitch.dict = type"));
}

#[test]
fn compiles_list_enumerator_package_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> xs = new List<int>();
                xs.Add(10);
                xs.Add(20);
                IEnumerator<int> e = xs.GetEnumerator();
                while (e.MoveNext()) {
                    print(e.Current);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("List<T>.GetEnumerator should lower");

    assert!(llvm_ir.contains("ListEnumerator"));
    assert!(llvm_ir.contains("MoveNext"));
}

#[test]
fn compiles_dictionary_foreach_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                Dictionary<string, int> values = new Dictionary<string, int>();
                values.Add("a", 1);
                values.Add("b", 2);
                foreach (KeyValuePair<string, int> pair in values) {
                    print(pair.Key);
                    print(pair.Value);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("dictionary foreach should lower to LLVM");

    assert!(llvm_ir.contains("dict_foreach_condition"));
}

#[test]
fn compiles_dictionary_enumerator_runtime_shape() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                Dictionary<string, int> values = new Dictionary<string, int>();
                values.Add("a", 1);
                values.Add("b", 2);

                IEnumerator<KeyValuePair<string, int>> enumerator = values.GetEnumerator();
                while (enumerator.MoveNext()) {
                    KeyValuePair<string, int> pair = enumerator.Current;
                    print(pair.Key);
                    print(pair.Value);
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("dictionary enumerator runtime shape should lower");

    assert!(llvm_ir.contains("dict_enum_keys_loop"));
    assert!(llvm_ir.contains("DictionaryEnumerator"));
}

#[test]
fn runs_dictionary_enumerator_natively_without_leaks() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                Dictionary<string, int> values = new Dictionary<string, int>();
                values.Add("a", 1);
                values.Add("b", 2);

                IEnumerator<KeyValuePair<string, int>> enumerator = values.GetEnumerator();
                while (enumerator.MoveNext()) {
                    KeyValuePair<string, int> pair = enumerator.Current;
                    print(pair.Key);
                    print(pair.Value);
                }
            }
        "#;

    let output_exe = emit_native_executable_from_source("dict-enumerator-native", source);
    let output = run_native_executable_with_leak_report(&output_exe);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "dictionary enumerator sample should exit cleanly\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert_eq!(stdout, "a\r\n1\r\nb\r\n2\r\n0\r\n");
}

#[test]
fn runs_dictionary_foreach_natively_without_leaks() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                Dictionary<string, int> values = new Dictionary<string, int>();
                values.Add("a", 1);
                values.Add("b", 2);

                foreach (KeyValuePair<string, int> pair in values) {
                    print(pair.Key);
                    print(pair.Value);
                }
            }
        "#;

    let output_exe = emit_native_executable_from_source("dict-foreach-native", source);
    let output = run_native_executable_with_leak_report(&output_exe);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "dictionary foreach sample should exit cleanly\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert_eq!(stdout, "a\r\n1\r\nb\r\n2\r\n0\r\n");
}

#[test]
fn compiles_hashset_package_surface() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                HashSet<string> values = new HashSet<string>();
                values.Add("a");
                values.Add("a");
                values.Add("b");
                print(values.Count);
                print(values.Contains("a"));
                values.Clear();
                print(values.Count);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("HashSet package surface should compile");

    assert!(llvm_ir.contains("HashSet"));
}

#[test]
fn runs_native_collection_example() {
    let output_exe =
        emit_native_executable_from_path("llvm-collections", "examples/llvm_collections.gl");
    let output = run_native_executable_with_leak_report(&output_exe);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "native collection example should exit cleanly\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert_eq!(stdout, "2\r\n20\r\ntrue\r\n2\r\n100\r\ntrue\r\ntrue\r\n50\r\n0\r\n");
}

#[test]
fn runs_native_collection_workload_without_leaks() {
    let output_exe =
        emit_native_executable_from_path("collection-workload", "examples/collection_workload.gl");
    let output = run_native_executable_with_leak_report(&output_exe);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "native collection workload should exit cleanly\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert_eq!(stdout, "200050000\r\n0\r\n");
}

#[test]
fn runs_recursive_collection_task_graph_without_leaks() {
    let source = r#"
            using System.Collections.Generic;
            using System.Threading.Tasks;

            string MakeName() {
                return "Ada";
            }

            fn main() {
                Dictionary<string, List<Task<string>>> graph = new Dictionary<string, List<Task<string>>>();
                List<Task<string>> tasks = new List<Task<string>>();
                tasks.Add(Task.FromResult(MakeName()));
                graph.Add("names", tasks);
                print(graph["names"][0].Result);
            }
        "#;

    let output_exe = emit_native_executable_from_source("recursive-collection-task-graph", source);
    let output = run_native_executable_with_leak_report(&output_exe);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "recursive owned graph sample should exit cleanly\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("Ada"));
}

#[test]
fn runs_nested_collection_field_assignment_without_leaks() {
    let source = r#"
            using System.Collections.Generic;
            using System.Threading.Tasks;

            class Holder {
                public List<Task<string>> Items = new List<Task<string>>();
            }

            string MakeName() {
                return "Ada";
            }

            fn main() {
                List<Task<string>> tasks = new List<Task<string>>();
                tasks.Add(Task.FromResult(MakeName()));
                Holder holder = new Holder();
                holder.Items = tasks;
                print(holder.Items[0].Result);
            }
        "#;

    let output_exe = emit_native_executable_from_source("nested-collection-field-assignment", source);
    let output = run_native_executable_with_leak_report(&output_exe);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "nested collection field assignment sample should exit cleanly\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("Ada"));
}

#[test]
fn preserves_namespace_and_attribute_metadata() {
    let source = r#"
            namespace Demo.Api {
                [ApiController]
                [Route("/users")]
                class UsersController {
                    public string Name;
                }

                [HttpGet("/health")]
                string Health() {
                    return "ok";
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("attributes and namespace should lower to LLVM");

    assert!(llvm_ir.contains("UsersController"));
    assert!(llvm_ir.contains("Health"));
}

#[test]
fn compiles_class_method_and_this_access() {
    let source = r#"
            class Counter {
                public int Value;

                int GetValue() {
                    return this.Value;
                }
            }

            fn main() {
                Counter counter = new Counter { Value = 7 };
                int value = counter.GetValue();
                print(value);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("class methods should lower to LLVM");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn preserves_method_attribute_metadata() {
    let source = r#"
            namespace Demo.Api {
                [ApiController]
                class UsersController {
                    public int Count;

                    [HttpGet("/count")]
                    int GetCount() {
                        return this.Count;
                    }
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("method attributes should lower to LLVM");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_constructor_auto_property_and_expression_property() {
    let source = r#"
            class Counter {
                public int Count { get; set; }

                public Counter(int count) {
                    this.Count = count;
                }

                int Twice => this.Count + this.Count;
            }

            fn main() {
                Counter counter = new Counter(5);
                print(counter.Twice);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("constructors and properties should lower to LLVM");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn compiles_common_scalars_nullable_using_static_and_enums() {
    let source = r#"
            using static System.Math;

            namespace Demo {
                enum Status {
                    Unknown,
                    Active = 2,
                }

                class Model {
                    public Status State;
                    public bool Enabled;
                    public double Score;

                    public Model(Status state) {
                        this.State = state;
                        this.Enabled = true;
                        this.Score = 12.5;
                    }

                    int StateCode => this.State;
                }

                static int ReadConst() {
                    const int value = 3;
                    return value;
                }

                int ReadStatus(Status status) {
                    int code = 0;
                    switch (status) {
                        case Status.Active:
                            code = 1;
                            break;
                        default:
                            code = 9;
                            break;
                    }
                    return code;
                }

                fn main() {
                    Model model = new Model(Status.Active);
                    string? title = "ready";
                    DateTime now = "2026-05-31";
                    Guid id = "00000000-0000-0000-0000-000000000000";
                    byte b = 1;
                    short s = 2;
                    uint u = 3;
                    decimal money = 4.5;
                    print(model.StateCode);
                    print(model.Enabled);
                    print(model.Score);
                    print(ReadConst());
                    print(ReadStatus(model.State));
                }
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("common C# scalar surface should lower to LLVM");

    assert!(llvm_ir.contains("Model__g0__t"));
    assert!(llvm_ir.contains("ReadConst"));
    assert!(llvm_ir.contains("ReadStatus"));
    assert!(llvm_ir.contains("switch"));
}

#[test]
fn compiles_interfaces_inheritance_and_try_finally_surface() {
    let source = r#"
            interface IScored {
                int Score();
            }

            class BaseCounter {
                public int Seed;

                int GetSeed() {
                    return this.Seed;
                }
            }

            class DerivedCounter : BaseCounter, IScored {
                public int Bonus;

                public DerivedCounter(int seed, int bonus) {
                    this.Seed = seed;
                    this.Bonus = bonus;
                }

                int Score() {
                    int result = 0;
                    try {
                        result = this.GetSeed() + this.Bonus;
                    } catch (Exception ex) {
                        result = 0;
                    } finally {
                        result = result + 1;
                    }
                    return result;
                }
            }

            fn main() {
                DerivedCounter counter = new DerivedCounter(4, 5);
                print(counter.Score());
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("inheritance and try/finally surface should lower to LLVM");

    assert!(llvm_ir.contains("BaseCounter__g0__t"));
    assert!(llvm_ir.contains("DerivedCounter__g0__t"));
    assert!(llvm_ir.contains("try_catch"));
    assert!(llvm_ir.contains("try_finally"));
}

#[test]
fn compiles_throw_catch_finally_runtime_path() {
    let source = r#"
            fn main() {
                int code = 0;
                try {
                    throw new Exception("boom");
                } catch (Exception ex) {
                    print(ex.Message);
                    code = 7;
                } finally {
                    code = code + 1;
                }
                print(code);
            }
        "#;

    let llvm_ir =
        compile_llvm_ir(source).expect("throw/catch/finally should lower to LLVM");

    assert!(llvm_ir.contains("@glitch_exception_pending"));
    assert!(llvm_ir.contains("try_catch"));
    assert!(llvm_ir.contains("try_finally"));
}

#[test]
fn emits_stack_bytecode_backend() {
    let source = r#"
            fn main() {
                int x = 1;
                print(x);
            }
        "#;

    let bytecode = compile_bytecode(source).expect("bytecode should compile");

    assert!(bytecode.contains(".glitch_bytecode 0.1"));
    assert!(bytecode.contains(".function main"));
    assert!(bytecode.contains("  const.i64 1"));
    assert!(bytecode.contains("  store x"));
    assert!(bytecode.contains("  load x"));
    assert!(bytecode.contains("  print"));
}

#[test]
fn emits_llvm_ir_backend_after_borrow_check() {
    let source = r#"
            long Add(long left, long right) {
                return left + right;
            }

            fn main() {
                long value = Add(40, 2);
                print(value);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("LLVM IR should compile");

    assert!(llvm_ir.contains("; ModuleID = 'glitching'"));
    assert!(llvm_ir.contains("declare i32 @printf(ptr, ...)"));
    assert!(llvm_ir.contains("define i64 @Add(i64 %left, i64 %right)"));
    assert!(llvm_ir.contains(" = add i64 "));
    assert!(llvm_ir.contains("define i32 @main()"));
    assert!(llvm_ir.contains(" = call i64 @Add(i64 40, i64 2)"));
    assert!(llvm_ir.contains("@printf"));
}

