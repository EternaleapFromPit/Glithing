use super::*;

#[test]
fn borrow_checker_rejects_use_after_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                string moved = move name;
                print(name);
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("use after move should fail");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_rejects_assignment_while_borrowed() {
    let source = r#"
            fn main() {
                int value = 1;
                ref int borrowed = borrow value;
                value = 2;
                print(value);
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("assignment while borrowed should fail");

    assert!(error.contains("borrow checker: cannot assign to 'value' while it is borrowed"));
}

#[test]
fn borrow_checker_rejects_use_after_branch_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                if (true) {
                    string moved = move name;
                } else {
                }
                print(name);
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("branch move should poison the join state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_rejects_use_after_loop_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                while (true) {
                    string moved = move name;
                    break;
                }
                print(name);
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("loop move should poison the join state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_ignores_unreachable_code_after_break_in_loop() {
    let source = r#"
            fn main() {
                string name = "Ada";
                while (true) {
                    break;
                    string moved = move name;
                }
                print(name);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source)
        .expect("unreachable code after break should not poison borrow checking");

    assert!(!llvm_ir.is_empty());
}

#[test]
fn borrow_checker_rejects_use_after_break_branch_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                while (true) {
                    if (true) {
                        string moved = move name;
                        break;
                    } else {
                    }
                }
                print(name);
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("break branch move should poison the loop exit state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_ignores_for_increment_after_break() {
    let source = r#"
            void Consume(string value) {
            }

            fn main() {
                string name = "Ada";
                for (; true; Consume(move name)) {
                    break;
                }
                print(name);
            }
        "#;

    compile_source_with_options(source, true, false)
        .expect("for-loop increment after break should be unreachable");
}

#[test]
fn borrow_checker_applies_for_increment_after_continue() {
    let source = r#"
            void Consume(string value) {
            }

            fn main() {
                string name = "Ada";
                bool first = true;
                for (; first; Consume(move name)) {
                    first = false;
                    continue;
                }
                print(name);
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("continue should still flow through the for-loop increment");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_rejects_use_after_try_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                try {
                    string moved = move name;
                } finally {
                }
                print(name);
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("try/finally move should poison the join state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_propagates_finally_effects_into_break_exit_state() {
    let source = r#"
            void Consume(string value) {
            }

            fn main() {
                string name = "Ada";
                while (true) {
                    try {
                        break;
                    } finally {
                        Consume(move name);
                    }
                }
                print(name);
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("finally should execute on break paths before the loop exit merges");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_rejects_use_after_conditional_expression_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                bool choose = true;
                string moved = choose ? move name : "fallback";
                print(name);
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("conditional expression move should poison the join state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn borrow_checker_allows_use_after_returning_branch_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                if (true) {
                    string moved = move name;
                    return;
                } else {
                }
                print(name);
            }
        "#;

    compile_source_with_options(source, true, false)
        .expect("returning branch should not poison join state");
}

#[test]
fn borrow_checker_allows_use_after_returning_switch_arm_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                switch (1) {
                    case 0: {
                        string moved = move name;
                        return;
                    }
                    default: {
                    }
                }
                print(name);
            }
        "#;

    compile_source_with_options(source, true, false)
        .expect("returning switch arm should not poison join state");
}

#[test]
fn borrow_checker_rejects_use_after_switch_break_move() {
    let source = r#"
            fn main() {
                string name = "Ada";
                switch (1) {
                    case 0: {
                        string moved = move name;
                        break;
                    }
                    default: {
                    }
                }
                print(name);
            }
        "#;

    let error = compile_source_with_options(source, true, false)
        .expect_err("switch break branch move should poison the post-switch state");

    assert!(error.contains("borrow checker: use of moved value 'name'"));
}

#[test]
fn ownership_checker_allows_class_transfer_without_move() {
    let source = r#"
            class Service {
                public string Name;
            }

            fn main() {
                Service first = new Service { Name = "one" };
                Service second = first;
                print(second.Name);
            }
        "#;

    let llvm_ir = compile_llvm_ir(source).expect("class transfer without move should succeed");
    assert!(llvm_ir.contains("call void @glitch_retain"));
}

#[test]
fn ownership_checker_rejects_view_outliving_source() {
    let source = r#"
            using System.Collections.Generic;

            fn main() {
                List<int> outer = new List<int>();
                IEnumerable<int> view = outer;
                {
                    List<int> inner = new List<int>();
                    view = inner;
                }
                foreach (int value in view) {
                    print(value);
                }
            }
        "#;

    let error =
        compile_source_with_options(source, true, false).expect_err("view outliving source should fail");

    assert!(error.contains(
        "ownership checker: function main: 'view' would outlive borrowed/view source 'inner'"
    ));
}

#[test]
fn compiles_memory_leak_tests() {
    let source = std::fs::read_to_string("tests/xunit_memory/memory_leak_tests.gl")
        .expect("should read memory_leak_tests.gl");
    let llvm_ir = compile_llvm_ir(&source).expect("memory leak tests should compile to LLVM IR");
    assert!(llvm_ir.contains("getelementptr"));
}

#[test]
fn emits_thread_safe_allocation_registry_for_llvm_runtime() {
    let source = std::fs::read_to_string("tests/xunit_memory/memory_leak_tests.gl")
        .expect("should read memory_leak_tests.gl");
    let llvm_ir = compile_llvm_ir(&source).expect("memory leak tests should compile to LLVM IR");

    assert!(llvm_ir.contains("declare i64 @GlitchLiveAllocations_Add(i64)"));
    assert!(llvm_ir.contains("declare i64 @GlitchLiveAllocations_Load()"));
    assert!(llvm_ir.contains("call i64 @GlitchLiveAllocations_Add(i64 1)"));
    assert!(llvm_ir.contains("call i64 @GlitchLiveAllocations_Add(i64 -1)"));
    assert!(llvm_ir.contains("call i64 @GlitchLiveAllocations_Load()"));
}

#[test]
fn compiles_selected_repo_examples_through_llvm() {
    let examples = [
        "examples/csharp_var.gl",
        "examples/llvm_simple.gl",
        "examples/llvm_collections.gl",
        "examples/collection_workload.gl",
        "examples/value_box_benchmark.gl",
    ];

    for example in examples {
        let source = fs::read_to_string(example).expect("example source should be readable");
        compile_llvm_ir(&source).unwrap_or_else(|error| {
            panic!("example {example} should compile to LLVM IR: {error}");
        });
    }
}

#[test]
fn compiles_conduit_integration_tests_project_file_through_llvm() {
    let project = "external/aspnetcore-realworld-example-app/tests/Conduit.IntegrationTests/Conduit.IntegrationTests.csproj";
    if !std::path::Path::new(project).exists() {
        eprintln!("skipping missing external fixture: {project}");
        return;
    }
    let error = compile_llvm_ir_from_path(project)
        .expect_err("Conduit integration tests project should currently fail on unsupported async suspension");
    assert!(error.contains("suspension inside try/catch/finally"));
    assert!(error.contains("Handle"));
}

#[test]
#[ignore]
fn debug_conduit_integration_tests_project_phases() {
    let project = std::env::var("GLITCH_PROJECT_BUNDLE").unwrap_or_else(|_| {
        "external/aspnetcore-realworld-example-app/tests/Conduit.IntegrationTests/Conduit.IntegrationTests.csproj"
            .to_string()
    });
    eprintln!("phase 1 read");
    let source = if project.ends_with(".gl") {
        fs::read_to_string(&project).expect("bundle source should load")
    } else {
        read_input_source(&project).expect("project source should load")
    };
    eprintln!("phase 2 link len={}", source.len());
    let linked = link_package_sources(&source).expect("package linking should succeed");
    eprintln!("phase 3 tokenize len={}", linked.len());
    let tokens = Lexer::new(&linked).tokenize().expect("tokenization should succeed");
    eprintln!("phase 4 parse tokens={}", tokens.len());
    let program = Parser::new(tokens).parse_program().expect("parse should succeed");
    eprintln!(
        "phase 5 parsed functions={} types={}",
        program.functions.len(),
        program.types.len()
    );
    validate_generic_constraints(&program).expect("generic validation should succeed");
    eprintln!("phase 6 borrow");
    BorrowChecker::check_program(&program).expect("borrow checking should succeed");
    eprintln!("phase 7 lower");
    let typed = TypedProgram::lower(&program, None).expect("typed lowering should succeed");
    eprintln!(
        "phase 8 typed functions={} types={}",
        typed.functions.len(),
        typed.types.len()
    );
}
