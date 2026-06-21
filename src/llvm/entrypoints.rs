use super::*;

impl LlvmEmitter {
    pub(super) fn register_function(&mut self, function: &TypedFunction) {
        self.functions.insert(
            function.symbol.clone(),
            LlFunctionSig {
                return_type: llvm_ir_type(&function.return_type),
                params: function
                    .params
                    .iter()
                    .map(|param| llvm_ir_type(&param.ty))
                    .collect(),
                required_params: function.required_params,
            },
        );
    }

    pub(super) fn emit_startup_wrapper(&mut self) -> Result<(), String> {
        let Some(startup) = self.startup.clone() else {
            return Ok(());
        };
        if startup.symbol == "main" {
            return Ok(());
        }
        let signature = self.functions.get(&startup.symbol).cloned().ok_or_else(|| {
            format!(
                "LLVM backend: startup symbol '{}' was not registered",
                startup.symbol
            )
        })?;
        self.body
            .push_str("define i32 @main(i32 %argc, ptr %argv) {\nentry:\n");
        let rendered_args = if startup.accepts_args {
            let array = self.tmp();
            let has_args = self.tmp();
            let argc_minus_one = self.tmp();
            let argc = self.tmp();
            let argv = self.tmp();
            let len_ptr = self.tmp();
            let len = self.tmp();
            let data_ptr = self.tmp();
            self.body.push_str(&format!(
                "  {array} = alloca %glitch.array\n  {has_args} = icmp ugt i32 %argc, 0\n  {argc_minus_one} = sub i32 %argc, 1\n  {argc} = select i1 {has_args}, i32 {argc_minus_one}, i32 0\n  {argv} = getelementptr inbounds ptr, ptr %argv, i64 1\n  {len_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 0\n  {len} = zext i32 {argc} to i64\n  store i64 {len}, ptr {len_ptr}\n  {data_ptr} = getelementptr inbounds %glitch.array, ptr {array}, i32 0, i32 1\n  store ptr {argv}, ptr {data_ptr}\n"
            ));
            "ptr ".to_string() + &array
        } else {
            String::new()
        };
        let call_result = if signature.return_type == LlType::Void {
            self.body.push_str(&format!(
                "  call void @{}({})\n  ret i32 0\n",
                sanitize(&startup.symbol),
                rendered_args
            ));
            None
        } else {
            let result = self.tmp();
            let args = if rendered_args.is_empty() {
                String::new()
            } else {
                rendered_args.clone()
            };
            self.body.push_str(&format!(
                "  {result} = call {} @{}({})\n",
                signature.return_type.as_ir(),
                sanitize(&startup.symbol),
                args
            ));
            Some(result)
        };
        if let Some(result) = call_result {
            if startup.returns_int {
                self.body.push_str(&format!("  ret i32 {result}\n"));
            } else {
                self.body.push_str("  ret i32 0\n");
            }
        }
        self.body.push_str("}\n\n");
        Ok(())
    }

    pub(super) fn emit_web_application_handle_wrapper(
        &mut self,
        program: &TypedProgram,
    ) -> Result<(), String> {
        let Some(web_app) = program.types.iter().find(|ty| ty.name == "WebApplication") else {
            return Ok(());
        };
        let Some(handle) = web_app.methods.iter().find(|method| method.name == "Handle") else {
            return Ok(());
        };
        self.body.push_str(&format!(
            "define ptr @WebApplication_Handle(ptr %self, ptr %method, ptr %path, ptr %body) {{\nentry:\n  %result = call ptr @{}(ptr %self, ptr %method, ptr %path, ptr %body)\n  ret ptr %result\n}}\n\n",
            sanitize(&handle.symbol)
        ));
        Ok(())
    }
}
