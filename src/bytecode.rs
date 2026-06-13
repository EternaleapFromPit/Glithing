use crate::ast::*;

pub(crate) struct BytecodeEmitter {
    out: String,
    label: usize,
}

impl BytecodeEmitter {
    pub(crate) fn emit_program(program: &Program) -> String {
        let mut emitter = Self {
            out: String::new(),
            label: 0,
        };
        emitter.out.push_str(".glitch_bytecode 0.1\n");
        if let Some(package_id) = &program.package_id {
            emitter.out.push_str(&format!(".package {package_id}\n"));
        }
        for enum_def in &program.enums {
            emitter.out.push_str(&format!(".enum {}\n", enum_def.name));
            for variant in &enum_def.variants {
                if let Some(value) = variant.value {
                    emitter
                        .out
                        .push_str(&format!("  .variant {} = {}\n", variant.name, value));
                } else {
                    emitter
                        .out
                        .push_str(&format!("  .variant {}\n", variant.name));
                }
            }
        }
        for ty in &program.types {
            emitter.emit_type(ty);
        }
        for function in &program.functions {
            emitter.emit_function(function, &function.name);
        }
        emitter.out
    }

    fn emit_type(&mut self, ty: &TypeDef) {
        self.out
            .push_str(&format!(".type {} {:?}\n", ty.name, ty.kind));
        for base in &ty.bases {
            self.out.push_str(&format!("  .base {base}\n"));
        }
        for field in &ty.fields {
            self.out.push_str(&format!(
                "  .field {} {}\n",
                type_name(&field.ty),
                field.name
            ));
        }
        for constructor in &ty.constructors {
            self.out.push_str(&format!(".function {}.ctor\n", ty.name));
            self.emit_params(&constructor.params);
            self.emit_stmts(&constructor.body);
            self.out.push_str("  ret\n.end\n");
        }
        for method in &ty.methods {
            self.emit_function(method, &format!("{}.{}", ty.name, method.name));
        }
    }

    fn emit_function(&mut self, function: &Function, symbol: &str) {
        self.out.push_str(&format!(".function {symbol}\n"));
        self.out.push_str(&format!(
            "  .returns {}\n",
            type_name(&function.return_type)
        ));
        self.emit_params(&function.params);
        self.emit_stmts(&function.body);
        self.out.push_str("  ret\n.end\n");
    }

    fn emit_params(&mut self, params: &[Param]) {
        for param in params {
            self.out.push_str(&format!(
                "  .param {} {}\n",
                type_name(&param.ty),
                param.name
            ));
        }
    }

    fn emit_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.emit_stmt(stmt);
        }
    }

    fn emit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, expr, .. } => {
                self.emit_expr(expr);
                self.out.push_str(&format!("  store {name}\n"));
            }
            Stmt::Assign { name, expr } => {
                self.emit_expr(expr);
                self.out.push_str(&format!("  store {name}\n"));
            }
            Stmt::AssignTarget { target, expr } => {
                self.emit_expr(target);
                self.emit_expr(expr);
                self.out.push_str("  store_target\n");
            }
            Stmt::Block(body) => self.emit_stmts(body),
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let else_label = self.next_label("else");
                let end_label = self.next_label("endif");
                self.emit_expr(condition);
                self.out.push_str(&format!("  br_false {else_label}\n"));
                self.emit_stmts(then_body);
                self.out.push_str(&format!("  br {end_label}\n"));
                self.out.push_str(&format!("{else_label}:\n"));
                self.emit_stmts(else_body);
                self.out.push_str(&format!("{end_label}:\n"));
            }
            Stmt::Try {
                try_body,
                catch,
                finally_body,
            } => {
                let catch_label = self.next_label("catch");
                let finally_label = self.next_label("finally");
                let end_label = self.next_label("endtry");
                self.out.push_str(&format!("  try {catch_label}\n"));
                self.emit_stmts(try_body);
                self.out.push_str(&format!("  br {finally_label}\n"));
                self.out.push_str(&format!("{catch_label}:\n"));
                if let Some(catch) = catch {
                    if let Some(name) = &catch.name {
                        self.out.push_str(&format!("  catch_store {name}\n"));
                    }
                    self.emit_stmts(&catch.body);
                } else {
                    self.out.push_str("  rethrow\n");
                }
                self.out.push_str(&format!("{finally_label}:\n"));
                self.emit_stmts(finally_body);
                self.out.push_str("  end_try\n");
                self.out.push_str(&format!("{end_label}:\n"));
            }
            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                let end_label = self.next_label("endswitch");
                self.emit_expr(expr);
                for case in cases {
                    let case_label = self.next_label("case");
                    self.emit_expr(&case.value);
                    self.out.push_str(&format!("  switch_eq {case_label}\n"));
                    self.out.push_str(&format!("{case_label}:\n"));
                    self.emit_stmts(&case.body);
                }
                self.emit_stmts(default);
                self.out.push_str(&format!("{end_label}:\n"));
            }
            Stmt::While { condition, body } => {
                let start = self.next_label("while");
                let end = self.next_label("endwhile");
                self.out.push_str(&format!("{start}:\n"));
                self.emit_expr(condition);
                self.out.push_str(&format!("  br_false {end}\n"));
                self.emit_stmts(body);
                self.out.push_str(&format!("  br {start}\n"));
                self.out.push_str(&format!("{end}:\n"));
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                if let Some(init) = init {
                    self.emit_stmt(init);
                }
                let start = self.next_label("for");
                let end = self.next_label("endfor");
                self.out.push_str(&format!("{start}:\n"));
                if let Some(condition) = condition {
                    self.emit_expr(condition);
                    self.out.push_str(&format!("  br_false {end}\n"));
                }
                self.emit_stmts(body);
                if let Some(increment) = increment {
                    self.emit_stmt(increment);
                }
                self.out.push_str(&format!("  br {start}\n"));
                self.out.push_str(&format!("{end}:\n"));
            }
            Stmt::ForEach {
                item_name,
                collection,
                body,
                ..
            } => {
                let start = self.next_label("foreach");
                let end = self.next_label("endforeach");
                self.emit_expr(collection);
                self.out
                    .push_str(&format!("  foreach_start {item_name} {end}\n"));
                self.out.push_str(&format!("{start}:\n"));
                self.emit_stmts(body);
                self.out
                    .push_str(&format!("  foreach_next {item_name} {start}\n"));
                self.out.push_str(&format!("{end}:\n"));
            }
            Stmt::Print(expr) => {
                self.emit_expr(expr);
                self.out.push_str("  print\n");
            }
            Stmt::Expr(expr) => {
                self.emit_expr(expr);
                self.out.push_str("  pop\n");
            }
            Stmt::Return(Some(expr)) => {
                self.emit_expr(expr);
                self.out.push_str("  ret_value\n");
            }
            Stmt::Return(None) => self.out.push_str("  ret\n"),
            Stmt::Throw(expr) => {
                self.emit_expr(expr);
                self.out.push_str("  throw\n");
            }
            Stmt::Break => self.out.push_str("  break\n"),
            Stmt::Continue => self.out.push_str("  continue\n"),
        }
    }

    fn emit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Int(value) => self.out.push_str(&format!("  const.i64 {value}\n")),
            Expr::Float(value) => self.out.push_str(&format!("  const.f64 {value}\n")),
            Expr::Bool(value) => self.out.push_str(&format!("  const.bool {value}\n")),
            Expr::Null => self.out.push_str("  const.null\n"),
            Expr::String(value) => self.out.push_str(&format!("  const.string {:?}\n", value)),
            Expr::Var(name) => self.out.push_str(&format!("  load {name}\n")),
            Expr::Move(name) => self.out.push_str(&format!("  move {name}\n")),
            Expr::ArrayLiteral(values) => {
                for value in values {
                    self.emit_expr(value);
                }
                self.out
                    .push_str(&format!("  new_array {}\n", values.len()));
            }
            Expr::NewArray { length, values, .. } => {
                if let Some(length) = length {
                    self.emit_expr(length);
                }
                for value in values {
                    self.emit_expr(value);
                }
                self.out
                    .push_str(&format!("  new_array {}\n", values.len()));
            }
            Expr::Index { target, index } => {
                self.emit_expr(target);
                self.emit_expr(index);
                self.out.push_str("  index\n");
            }
            Expr::Field { target, name } => {
                self.emit_expr(target);
                self.out.push_str(&format!("  field {name}\n"));
            }
            Expr::IsPattern { expr, ty, name } => {
                self.emit_expr(expr);
                self.out
                    .push_str(&format!("  is_pattern {} {:?}\n", type_name(ty), name));
            }
            Expr::MethodCall { target, name, args } => {
                self.emit_expr(target);
                for arg in args {
                    self.emit_expr(arg);
                }
                self.out
                    .push_str(&format!("  call_method {name} {}\n", args.len()));
            }
            Expr::FunctionCall { name, args } => {
                for arg in args {
                    self.emit_expr(arg);
                }
                self.out
                    .push_str(&format!("  call {name} {}\n", args.len()));
            }
            Expr::NewObject {
                type_name,
                args,
                fields,
            } => {
                for arg in args {
                    self.emit_expr(arg);
                }
                for field in fields {
                    self.emit_expr(&field.expr);
                    self.out.push_str(&format!("  init_field {}\n", field.name));
                }
                self.out.push_str(&format!(
                    "  new_object {type_name} {} {}\n",
                    args.len(),
                    fields.len()
                ));
            }
            Expr::NewCollection(ty) => self
                .out
                .push_str(&format!("  new_collection {}\n", type_name(ty))),
            Expr::NewThread(entry) => self.out.push_str(&format!("  new_thread {entry}\n")),
            Expr::Borrow { mutable, name } => {
                let op = if *mutable { "borrow_mut" } else { "borrow" };
                self.out.push_str(&format!("  {op} {name}\n"));
            }
            Expr::Await(inner) => {
                self.emit_expr(inner);
                self.out.push_str("  await\n");
            }
            Expr::Throw(inner) => {
                self.emit_expr(inner);
                self.out.push_str("  throw\n");
            }
            Expr::Assign { target, value } => {
                self.emit_expr(target);
                self.emit_expr(value);
                self.out.push_str("  assign\n");
            }
            Expr::Lambda { params, body } => {
                self.emit_expr(body);
                self.out
                    .push_str(&format!("  lambda {}\n", params.join(",")));
            }
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                self.emit_expr(condition);
                self.emit_expr(when_true);
                self.emit_expr(when_false);
                self.out.push_str("  conditional\n");
            }
            Expr::Unary { op, expr } => {
                self.emit_expr(expr);
                self.out.push_str(&format!("  unary {:?}\n", op));
            }
            Expr::IncDec {
                name,
                delta,
                prefix,
            } => {
                self.out
                    .push_str(&format!("  incdec {name} {delta} {prefix}\n"));
            }
            Expr::Binary { left, op, right } => {
                self.emit_expr(left);
                self.emit_expr(right);
                self.out.push_str(&format!("  binary {:?}\n", op));
            }
            Expr::NamedArg { expr, .. } | Expr::RefArg { expr, .. } => self.emit_expr(expr),
        }
    }

    fn next_label(&mut self, prefix: &str) -> String {
        let label = format!("{prefix}_{}", self.label);
        self.label += 1;
        label
    }
}

fn type_name(ty: &TypeSyntax) -> String {
    match ty {
        TypeSyntax::Scalar(scalar) => format!("{scalar:?}"),
        TypeSyntax::String => "string".to_string(),
        TypeSyntax::Array(inner) => format!("{}[]", type_name(inner)),
        TypeSyntax::Ref(inner) => format!("ref {}", type_name(inner)),
        TypeSyntax::Named(name) => name.clone(),
        TypeSyntax::GenericNamed { name, args } => format!(
            "{}<{}>",
            name,
            args.iter().map(type_name).collect::<Vec<_>>().join(",")
        ),
        TypeSyntax::List(inner) => format!("List<{}>", type_name(inner)),
        TypeSyntax::Dictionary(key, value) => {
            format!("Dictionary<{},{}>", type_name(key), type_name(value))
        }
        TypeSyntax::IEnumerable(inner) => format!("IEnumerable<{}>", type_name(inner)),
        TypeSyntax::Thread => "Thread".to_string(),
        TypeSyntax::Task(inner) => format!("Task<{}>", type_name(inner)),
        TypeSyntax::Nullable(inner) => format!("{}?", type_name(inner)),
        TypeSyntax::Void => "void".to_string(),
    }
}
