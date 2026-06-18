use super::LlvmEmitter;

pub(super) fn finish_module(emitter: &LlvmEmitter) -> Result<String, String> {
        let mut out = String::new();
        out.push_str("; ModuleID = 'glitching'\n");
        for type_def in &emitter.type_defs {
            out.push_str(type_def);
        }
        out.push_str("declare i32 @printf(ptr, ...)\n");
        out.push_str("declare ptr @calloc(i64, i64)\n");
        out.push_str("declare ptr @realloc(ptr, i64)\n");
        out.push_str("declare void @free(ptr)\n");
        out.push_str("declare i32 @strcmp(ptr, ptr)\n");
        out.push_str("declare i32 @strncmp(ptr, ptr, i64)\n");
        out.push_str("declare i64 @strlen(ptr)\n");
        out.push_str("declare i64 @strtoll(ptr, ptr, i32)\n");
        out.push_str("declare double @strtod(ptr, ptr)\n");
        out.push_str("declare ptr @strstr(ptr, ptr)\n");
        out.push_str("declare i32 @snprintf(ptr, i64, ptr, ...)\n");
        out.push_str("declare ptr @memcpy(ptr, ptr, i64)\n");
        out.push_str("declare ptr @getenv(ptr)\n");
        out.push_str("declare void @GlitchRestHost_Run(ptr, i32, i32, ptr, ptr)\n");
        out.push_str("declare ptr @System_IO_File_ReadAllText(ptr)\n");
        out.push_str("declare void @System_IO_File_WriteAllText(ptr, ptr)\n");
        out.push_str("declare void @System_Console_WriteLine_String(ptr)\n");
        out.push_str("declare void @System_Console_WriteLine_I64(i64)\n");
        out.push_str("declare void @System_Console_WriteLine_Double(double)\n");
        out.push_str("declare void @System_Console_WriteLine_Bool(i1)\n");
        out.push_str("declare ptr @GlitchString_Lock()\n");
        out.push_str("declare void @GlitchString_Unlock(ptr)\n");
        out.push_str("declare i64 @GlitchLiveAllocations_Add(i64)\n");
        out.push_str("declare i64 @GlitchLiveAllocations_Load()\n");
        out.push_str("@glitch_exception_pending = internal global ptr null\n");
        out.push_str(
            "define dllexport ptr @glitch_take_pending_exception() {\nentry:\n  %value = load ptr, ptr @glitch_exception_pending\n  store ptr null, ptr @glitch_exception_pending\n  ret ptr %value\n}\n",
        );
        out.push_str(
            "define dllexport void @glitch_object_drop(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %drop\n\
drop:\n  %drop_ptr_ptr = getelementptr inbounds { i64, ptr }, ptr %value, i32 0, i32 1\n  %drop_ptr = load ptr, ptr %drop_ptr_ptr\n  %has_drop = icmp ne ptr %drop_ptr, null\n  br i1 %has_drop, label %call_drop, label %done\n\
call_drop:\n  call void %drop_ptr(ptr %value)\n  br label %done\n\
done:\n  ret void\n}\n",
        );
        out.push_str(
            "%glitch.task = type { i32, ptr }\n\
            define ptr @glitch_task_from_result_ptr(ptr %result) {\n\
            entry:\n\
              %task = call ptr @glitch_calloc(i64 1, i64 16)\n\
              %completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0\n\
              store i32 1, ptr %completed_ptr\n\
              %result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1\n\
              store ptr %result, ptr %result_ptr\n\
              ret ptr %task\n\
            }\n\
            define ptr @glitch_task_from_result_i32(i32 %result) {\n\
            entry:\n\
              %val_ptr = inttoptr i32 %result to ptr\n\
              %task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)\n\
              ret ptr %task\n\
            }\n\
            define ptr @glitch_task_from_result_i64(i64 %result) {\n\
            entry:\n\
              %val_ptr = inttoptr i64 %result to ptr\n\
              %task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)\n\
              ret ptr %task\n\
            }\n\
            define ptr @glitch_task_from_result_double(double %result) {\n\
            entry:\n\
              %cast = bitcast double %result to i64\n\
              %val_ptr = inttoptr i64 %cast to ptr\n\
              %task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)\n\
              ret ptr %task\n\
            }\n\
            define ptr @glitch_task_get_result_ptr(ptr %task) {\n\
            entry:\n\
              %is_null = icmp eq ptr %task, null\n\
              br i1 %is_null, label %null_case, label %normal_case\n\
            null_case:\n\
              ret ptr null\n\
            normal_case:\n\
              %result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1\n\
              %result = load ptr, ptr %result_ptr\n\
              ret ptr %result\n\
            }\n\
            define i32 @glitch_task_get_result_i32(ptr %task) {\n\
            entry:\n\
              %ptr = call ptr @glitch_task_get_result_ptr(ptr %task)\n\
              %val = ptrtoint ptr %ptr to i32\n\
              ret i32 %val\n\
            }\n\
            define i64 @glitch_task_get_result_i64(ptr %task) {\n\
            entry:\n\
              %ptr = call ptr @glitch_task_get_result_ptr(ptr %task)\n\
              %val = ptrtoint ptr %ptr to i64\n\
              ret i64 %val\n\
            }\n\
            define double @glitch_task_get_result_double(ptr %task) {\n\
            entry:\n\
              %ptr = call ptr @glitch_task_get_result_ptr(ptr %task)\n\
              %val = ptrtoint ptr %ptr to i64\n\
              %res = bitcast i64 %val to double\n\
              ret double %res\n\
            }\n",
        );
        out.push_str(
            "define ptr @glitch_calloc(i64 %count, i64 %size) {\nentry:\n  %value = call ptr @calloc(i64 %count, i64 %size)\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %count_alloc\ncount_alloc:\n  %live = call i64 @GlitchLiveAllocations_Add(i64 1)\n  br label %done\ndone:\n  ret ptr %value\n}\n",
        );
        out.push_str(
            "define ptr @glitch_realloc(ptr %old, i64 %size) {\nentry:\n  %value = call ptr @realloc(ptr %old, i64 %size)\n  %old_null = icmp eq ptr %old, null\n  %new_valid = icmp ne ptr %value, null\n  %count_it = and i1 %old_null, %new_valid\n  br i1 %count_it, label %count_alloc, label %done\ncount_alloc:\n  %live = call i64 @GlitchLiveAllocations_Add(i64 1)\n  br label %done\ndone:\n  ret ptr %value\n}\n",
        );
        out.push_str(
            "define void @glitch_free(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  call void @free(ptr %value)\n  %live = call i64 @GlitchLiveAllocations_Add(i64 -1)\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str(
            "define i64 @glitch_string_length(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %empty, label %read_len\nempty:\n  ret i64 0\nread_len:\n  %length_ptr = getelementptr i8, ptr %value, i64 -8\n  %length = load i64, ptr %length_ptr\n  ret i64 %length\n}\n",
        );
        out.push_str(
            "define i1 @glitch_route_match(ptr %template, ptr %path) {\nentry:\n  %template_index = alloca i64\n  %path_index = alloca i64\n  store i64 0, ptr %template_index\n  store i64 0, ptr %path_index\n  br label %scan\nscan:\n  %ti = load i64, ptr %template_index\n  %pi = load i64, ptr %path_index\n  %template_ptr = getelementptr inbounds i8, ptr %template, i64 %ti\n  %path_ptr = getelementptr inbounds i8, ptr %path, i64 %pi\n  %template_char = load i8, ptr %template_ptr\n  %path_char = load i8, ptr %path_ptr\n  %template_done = icmp eq i8 %template_char, 0\n  br i1 %template_done, label %finish, label %inspect\ninspect:\n  %parameter_start = icmp eq i8 %template_char, 123\n  br i1 %parameter_start, label %skip_parameter_name, label %literal\nliteral:\n  %same = icmp eq i8 %template_char, %path_char\n  br i1 %same, label %advance_both, label %no_match\nadvance_both:\n  %ti_next = add i64 %ti, 1\n  %pi_next = add i64 %pi, 1\n  store i64 %ti_next, ptr %template_index\n  store i64 %pi_next, ptr %path_index\n  br label %scan\nskip_parameter_name:\n  %parameter_template_index = alloca i64\n  %after_open = add i64 %ti, 1\n  store i64 %after_open, ptr %parameter_template_index\n  br label %parameter_name_loop\nparameter_name_loop:\n  %parameter_ti = load i64, ptr %parameter_template_index\n  %parameter_template_ptr = getelementptr inbounds i8, ptr %template, i64 %parameter_ti\n  %parameter_template_char = load i8, ptr %parameter_template_ptr\n  %parameter_name_done = icmp eq i8 %parameter_template_char, 125\n  %parameter_template_done = icmp eq i8 %parameter_template_char, 0\n  br i1 %parameter_template_done, label %no_match, label %parameter_name_check\nparameter_name_check:\n  br i1 %parameter_name_done, label %consume_parameter, label %advance_parameter_name\nadvance_parameter_name:\n  %parameter_ti_next = add i64 %parameter_ti, 1\n  store i64 %parameter_ti_next, ptr %parameter_template_index\n  br label %parameter_name_loop\nconsume_parameter:\n  %after_close = add i64 %parameter_ti, 1\n  store i64 %after_close, ptr %template_index\n  %parameter_path_start = load i64, ptr %path_index\n  %parameter_first_ptr = getelementptr inbounds i8, ptr %path, i64 %parameter_path_start\n  %parameter_first = load i8, ptr %parameter_first_ptr\n  %parameter_empty = icmp eq i8 %parameter_first, 0\n  %parameter_slash = icmp eq i8 %parameter_first, 47\n  %parameter_invalid = or i1 %parameter_empty, %parameter_slash\n  br i1 %parameter_invalid, label %no_match, label %parameter_path_loop\nparameter_path_loop:\n  %parameter_pi = load i64, ptr %path_index\n  %parameter_path_ptr = getelementptr inbounds i8, ptr %path, i64 %parameter_pi\n  %parameter_path_char = load i8, ptr %parameter_path_ptr\n  %parameter_path_done = icmp eq i8 %parameter_path_char, 0\n  %parameter_path_query = icmp eq i8 %parameter_path_char, 63\n  %parameter_path_slash = icmp eq i8 %parameter_path_char, 47\n  %parameter_path_end = or i1 %parameter_path_done, %parameter_path_query\n  %parameter_segment_done = or i1 %parameter_path_end, %parameter_path_slash\n  br i1 %parameter_segment_done, label %scan, label %advance_parameter_path\nadvance_parameter_path:\n  %parameter_pi_next = add i64 %parameter_pi, 1\n  store i64 %parameter_pi_next, ptr %path_index\n  br label %parameter_path_loop\nfinish:\n  %path_done = icmp eq i8 %path_char, 0\n  %path_query = icmp eq i8 %path_char, 63\n  %path_finished = or i1 %path_done, %path_query\n  ret i1 %path_finished\nno_match:\n  ret i1 false\n}\n",
        );
        out.push_str(
            "define ptr @glitch_path_segment_string(ptr %path, i64 %target) {\nentry:\n  %position = alloca i64\n  %segment = alloca i64\n  %start = alloca i64\n  store i64 0, ptr %position\n  store i64 0, ptr %segment\n  br label %skip_slashes\nskip_slashes:\n  %skip_position = load i64, ptr %position\n  %skip_ptr = getelementptr inbounds i8, ptr %path, i64 %skip_position\n  %skip_char = load i8, ptr %skip_ptr\n  %skip_end = icmp eq i8 %skip_char, 0\n  %skip_query = icmp eq i8 %skip_char, 63\n  %skip_done = or i1 %skip_end, %skip_query\n  br i1 %skip_done, label %missing, label %skip_check\nskip_check:\n  %is_slash = icmp eq i8 %skip_char, 47\n  br i1 %is_slash, label %advance_slash, label %begin_segment\nadvance_slash:\n  %after_slash = add i64 %skip_position, 1\n  store i64 %after_slash, ptr %position\n  br label %skip_slashes\nbegin_segment:\n  store i64 %skip_position, ptr %start\n  %current_segment = load i64, ptr %segment\n  %is_target = icmp eq i64 %current_segment, %target\n  br i1 %is_target, label %scan_target, label %skip_segment\nscan_target:\n  %target_position = load i64, ptr %position\n  %target_ptr = getelementptr inbounds i8, ptr %path, i64 %target_position\n  %target_char = load i8, ptr %target_ptr\n  %target_end = icmp eq i8 %target_char, 0\n  %target_query = icmp eq i8 %target_char, 63\n  %target_slash = icmp eq i8 %target_char, 47\n  %target_path_end = or i1 %target_end, %target_query\n  %target_done = or i1 %target_path_end, %target_slash\n  br i1 %target_done, label %copy_target, label %advance_target\nadvance_target:\n  %target_next = add i64 %target_position, 1\n  store i64 %target_next, ptr %position\n  br label %scan_target\ncopy_target:\n  %target_start = load i64, ptr %start\n  %target_length = sub i64 %target_position, %target_start\n  %target_data = call ptr @glitch_string_allocate(i64 %target_length)\n  %target_source = getelementptr inbounds i8, ptr %path, i64 %target_start\n  call ptr @memcpy(ptr %target_data, ptr %target_source, i64 %target_length)\n  ret ptr %target_data\nskip_segment:\n  %segment_position = load i64, ptr %position\n  %segment_ptr = getelementptr inbounds i8, ptr %path, i64 %segment_position\n  %segment_char = load i8, ptr %segment_ptr\n  %segment_end = icmp eq i8 %segment_char, 0\n  %segment_query = icmp eq i8 %segment_char, 63\n  %segment_done = or i1 %segment_end, %segment_query\n  br i1 %segment_done, label %missing, label %segment_check\nsegment_check:\n  %segment_slash = icmp eq i8 %segment_char, 47\n  br i1 %segment_slash, label %next_segment, label %advance_segment\nadvance_segment:\n  %segment_next_position = add i64 %segment_position, 1\n  store i64 %segment_next_position, ptr %position\n  br label %skip_segment\nnext_segment:\n  %next_segment_value = add i64 %current_segment, 1\n  store i64 %next_segment_value, ptr %segment\n  br label %skip_slashes\nmissing:\n  %empty = call ptr @glitch_string_allocate(i64 0)\n  ret ptr %empty\n}\ndefine i64 @glitch_path_segment_i64(ptr %path, i64 %target) {\nentry:\n  %text = call ptr @glitch_path_segment_string(ptr %path, i64 %target)\n  %value = call i64 @strtoll(ptr %text, ptr null, i32 10)\n  call void @glitch_string_release(ptr %text)\n  ret i64 %value\n}\n",
        );
        out.push_str(
            "define ptr @glitch_query_value_string(ptr %path, ptr %key, i64 %key_length) {\nentry:\n  %position = alloca i64\n  store i64 0, ptr %position\n  br label %find_query\nfind_query:\n  %find_position = load i64, ptr %position\n  %find_ptr = getelementptr inbounds i8, ptr %path, i64 %find_position\n  %find_char = load i8, ptr %find_ptr\n  %find_end = icmp eq i8 %find_char, 0\n  br i1 %find_end, label %query_missing, label %find_check\nfind_check:\n  %find_question = icmp eq i8 %find_char, 63\n  br i1 %find_question, label %next_pair, label %advance_find\nadvance_find:\n  %find_next = add i64 %find_position, 1\n  store i64 %find_next, ptr %position\n  br label %find_query\nnext_pair:\n  %pair_position = load i64, ptr %position\n  %pair_start = add i64 %pair_position, 1\n  store i64 %pair_start, ptr %position\n  br label %inspect_pair\ninspect_pair:\n  %inspect_position = load i64, ptr %position\n  %inspect_ptr = getelementptr inbounds i8, ptr %path, i64 %inspect_position\n  %inspect_char = load i8, ptr %inspect_ptr\n  %inspect_end = icmp eq i8 %inspect_char, 0\n  br i1 %inspect_end, label %query_missing, label %compare_key\ncompare_key:\n  %key_cmp = call i32 @strncmp(ptr %inspect_ptr, ptr %key, i64 %key_length)\n  %key_equal = icmp eq i32 %key_cmp, 0\n  %after_key_position = add i64 %inspect_position, %key_length\n  %after_key_ptr = getelementptr inbounds i8, ptr %path, i64 %after_key_position\n  %after_key_char = load i8, ptr %after_key_ptr\n  %has_equals = icmp eq i8 %after_key_char, 61\n  %key_match = and i1 %key_equal, %has_equals\n  br i1 %key_match, label %scan_value, label %skip_pair\nscan_value:\n  %value_start = add i64 %after_key_position, 1\n  store i64 %value_start, ptr %position\n  br label %value_loop\nvalue_loop:\n  %value_position = load i64, ptr %position\n  %value_ptr = getelementptr inbounds i8, ptr %path, i64 %value_position\n  %value_char = load i8, ptr %value_ptr\n  %value_end = icmp eq i8 %value_char, 0\n  %value_amp = icmp eq i8 %value_char, 38\n  %value_done = or i1 %value_end, %value_amp\n  br i1 %value_done, label %copy_value, label %advance_value\nadvance_value:\n  %value_next = add i64 %value_position, 1\n  store i64 %value_next, ptr %position\n  br label %value_loop\ncopy_value:\n  %value_length = sub i64 %value_position, %value_start\n  %value_data = call ptr @glitch_string_allocate(i64 %value_length)\n  %value_source = getelementptr inbounds i8, ptr %path, i64 %value_start\n  call ptr @memcpy(ptr %value_data, ptr %value_source, i64 %value_length)\n  ret ptr %value_data\nskip_pair:\n  %skip_pair_position = load i64, ptr %position\n  %skip_pair_ptr = getelementptr inbounds i8, ptr %path, i64 %skip_pair_position\n  %skip_pair_char = load i8, ptr %skip_pair_ptr\n  %skip_pair_end = icmp eq i8 %skip_pair_char, 0\n  br i1 %skip_pair_end, label %query_missing, label %skip_pair_check\nskip_pair_check:\n  %skip_pair_amp = icmp eq i8 %skip_pair_char, 38\n  br i1 %skip_pair_amp, label %next_pair, label %advance_skip_pair\nadvance_skip_pair:\n  %skip_pair_next = add i64 %skip_pair_position, 1\n  store i64 %skip_pair_next, ptr %position\n  br label %skip_pair\nquery_missing:\n  %query_empty = call ptr @glitch_string_allocate(i64 0)\n  ret ptr %query_empty\n}\ndefine i64 @glitch_query_value_i64(ptr %path, ptr %key, i64 %key_length) {\nentry:\n  %text = call ptr @glitch_query_value_string(ptr %path, ptr %key, i64 %key_length)\n  %value = call i64 @strtoll(ptr %text, ptr null, i32 10)\n  call void @glitch_string_release(ptr %text)\n  ret i64 %value\n}\n",
        );
        out.push_str(
            "define ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length) {\nentry:\n  %match = call ptr @strstr(ptr %json, ptr %token)\n  %missing = icmp eq ptr %match, null\n  br i1 %missing, label %json_missing, label %after_token\n  \nafter_token:\n  %cursor = alloca ptr\n  %token_end = getelementptr inbounds i8, ptr %match, i64 %token_length\n  store ptr %token_end, ptr %cursor\n  br label %find_colon\nfind_colon:\n  %colon_ptr = load ptr, ptr %cursor\n  %colon_char = load i8, ptr %colon_ptr\n  %colon_end = icmp eq i8 %colon_char, 0\n  br i1 %colon_end, label %json_missing, label %colon_check\ncolon_check:\n  %is_colon = icmp eq i8 %colon_char, 58\n  br i1 %is_colon, label %after_colon, label %advance_colon\nadvance_colon:\n  %colon_next = getelementptr inbounds i8, ptr %colon_ptr, i64 1\n  store ptr %colon_next, ptr %cursor\n  br label %find_colon\nafter_colon:\n  %value_candidate = getelementptr inbounds i8, ptr %colon_ptr, i64 1\n  store ptr %value_candidate, ptr %cursor\n  br label %skip_json_space\nskip_json_space:\n  %space_ptr = load ptr, ptr %cursor\n  %space_char = load i8, ptr %space_ptr\n  %is_space = icmp eq i8 %space_char, 32\n  %is_tab = icmp eq i8 %space_char, 9\n  %is_cr = icmp eq i8 %space_char, 13\n  %is_lf = icmp eq i8 %space_char, 10\n  %space_a = or i1 %is_space, %is_tab\n  %space_b = or i1 %is_cr, %is_lf\n  %whitespace = or i1 %space_a, %space_b\n  br i1 %whitespace, label %advance_json_space, label %value_kind\nadvance_json_space:\n  %space_next = getelementptr inbounds i8, ptr %space_ptr, i64 1\n  store ptr %space_next, ptr %cursor\n  br label %skip_json_space\nvalue_kind:\n  %is_quote = icmp eq i8 %space_char, 34\n  br i1 %is_quote, label %quoted_start, label %plain_start\nquoted_start:\n  %quoted_value = getelementptr inbounds i8, ptr %space_ptr, i64 1\n  store ptr %quoted_value, ptr %cursor\n  br label %scan_quoted\nscan_quoted:\n  %quoted_ptr = load ptr, ptr %cursor\n  %quoted_char = load i8, ptr %quoted_ptr\n  %quoted_done = icmp eq i8 %quoted_char, 34\n  %quoted_end = icmp eq i8 %quoted_char, 0\n  br i1 %quoted_end, label %json_missing, label %quoted_check\nquoted_check:\n  br i1 %quoted_done, label %copy_quoted, label %advance_quoted\nadvance_quoted:\n  %quoted_next = getelementptr inbounds i8, ptr %quoted_ptr, i64 1\n  store ptr %quoted_next, ptr %cursor\n  br label %scan_quoted\ncopy_quoted:\n  %quoted_start_int = ptrtoint ptr %quoted_value to i64\n  %quoted_end_int = ptrtoint ptr %quoted_ptr to i64\n  %quoted_length = sub i64 %quoted_end_int, %quoted_start_int\n  %quoted_data = call ptr @glitch_string_allocate(i64 %quoted_length)\n  call ptr @memcpy(ptr %quoted_data, ptr %quoted_value, i64 %quoted_length)\n  ret ptr %quoted_data\nplain_start:\n  store ptr %space_ptr, ptr %cursor\n  br label %scan_plain\nscan_plain:\n  %plain_ptr = load ptr, ptr %cursor\n  %plain_char = load i8, ptr %plain_ptr\n  %plain_end = icmp eq i8 %plain_char, 0\n  %plain_comma = icmp eq i8 %plain_char, 44\n  %plain_brace = icmp eq i8 %plain_char, 125\n  %plain_space = icmp eq i8 %plain_char, 32\n  %plain_a = or i1 %plain_end, %plain_comma\n  %plain_b = or i1 %plain_brace, %plain_space\n  %plain_done = or i1 %plain_a, %plain_b\n  br i1 %plain_done, label %copy_plain, label %advance_plain\nadvance_plain:\n  %plain_next = getelementptr inbounds i8, ptr %plain_ptr, i64 1\n  store ptr %plain_next, ptr %cursor\n  br label %scan_plain\ncopy_plain:\n  %plain_start_int = ptrtoint ptr %space_ptr to i64\n  %plain_end_int = ptrtoint ptr %plain_ptr to i64\n  %plain_length = sub i64 %plain_end_int, %plain_start_int\n  %plain_data = call ptr @glitch_string_allocate(i64 %plain_length)\n  call ptr @memcpy(ptr %plain_data, ptr %space_ptr, i64 %plain_length)\n  ret ptr %plain_data\njson_missing:\n  %json_empty = call ptr @glitch_string_allocate(i64 0)\n  ret ptr %json_empty\n}\ndefine i64 @glitch_json_value_i64(ptr %json, ptr %token, i64 %token_length) {\nentry:\n  %text = call ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length)\n  %value = call i64 @strtoll(ptr %text, ptr null, i32 10)\n  call void @glitch_string_release(ptr %text)\n  ret i64 %value\n}\ndefine double @glitch_json_value_double(ptr %json, ptr %token, i64 %token_length) {\nentry:\n  %text = call ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length)\n  %value = call double @strtod(ptr %text, ptr null)\n  call void @glitch_string_release(ptr %text)\n  ret double %value\n}\ndefine i1 @glitch_json_value_bool(ptr %json, ptr %token, i64 %token_length) {\nentry:\n  %text = call ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length)\n  %first = load i8, ptr %text\n  %is_t = icmp eq i8 %first, 116\n  %is_T = icmp eq i8 %first, 84\n  %is_one = icmp eq i8 %first, 49\n  %is_true_text = or i1 %is_t, %is_T\n  %value = or i1 %is_true_text, %is_one\n  call void @glitch_string_release(ptr %text)\n  ret i1 %value\n}\n",
        );
        out.push_str(
            "define ptr @glitch_i64_to_string(i64 %value) {\nentry:\n  %buffer = alloca [32 x i8]\n  %buffer_ptr = getelementptr inbounds [32 x i8], ptr %buffer, i64 0, i64 0\n  %length_i32 = call i32 (ptr, i64, ptr, ...) @snprintf(ptr %buffer_ptr, i64 32, ptr getelementptr inbounds ([5 x i8], ptr @.fmt_json_i64, i64 0, i64 0), i64 %value)\n  %length = sext i32 %length_i32 to i64\n  %text = call ptr @glitch_string_allocate(i64 %length)\n  call ptr @memcpy(ptr %text, ptr %buffer_ptr, i64 %length)\n  ret ptr %text\n}\ndefine ptr @glitch_double_to_string(double %value) {\nentry:\n  %buffer = alloca [64 x i8]\n  %buffer_ptr = getelementptr inbounds [64 x i8], ptr %buffer, i64 0, i64 0\n  %length_i32 = call i32 (ptr, i64, ptr, ...) @snprintf(ptr %buffer_ptr, i64 64, ptr getelementptr inbounds ([6 x i8], ptr @.fmt_json_double, i64 0, i64 0), double %value)\n  %length = sext i32 %length_i32 to i64\n  %text = call ptr @glitch_string_allocate(i64 %length)\n  call ptr @memcpy(ptr %text, ptr %buffer_ptr, i64 %length)\n  ret ptr %text\n}\n",
        );
        out.push_str(
            "define ptr @glitch_string_allocate(i64 %length) {\nentry:\n  %with_null = add i64 %length, 1\n  %size = add i64 %with_null, 16\n  %node = call ptr @glitch_calloc(i64 1, i64 %size)\n  %refs_ptr = getelementptr inbounds %glitch.string_node, ptr %node, i32 0, i32 0\n  store i64 1, ptr %refs_ptr\n  %length_ptr = getelementptr inbounds %glitch.string_node, ptr %node, i32 0, i32 1\n  store i64 %length, ptr %length_ptr\n  %data = getelementptr inbounds %glitch.string_node, ptr %node, i32 0, i32 2, i64 0\n  ret ptr %data\n}\n",
        );
        out.push_str(
            "define ptr @glitch_string_concat(ptr %left, ptr %right) {\nentry:\n  %left_length = call i64 @glitch_string_length(ptr %left)\n  %right_length = call i64 @glitch_string_length(ptr %right)\n  %length = add i64 %left_length, %right_length\n  %data = call ptr @glitch_string_allocate(i64 %length)\n  %left_empty = icmp eq i64 %left_length, 0\n  br i1 %left_empty, label %copy_right, label %copy_left\ncopy_left:\n  call ptr @memcpy(ptr %data, ptr %left, i64 %left_length)\n  br label %copy_right\ncopy_right:\n  %right_empty = icmp eq i64 %right_length, 0\n  br i1 %right_empty, label %done, label %copy_right_data\ncopy_right_data:\n  %right_target = getelementptr inbounds i8, ptr %data, i64 %left_length\n  call ptr @memcpy(ptr %right_target, ptr %right, i64 %right_length)\n  br label %done\ndone:\n  ret ptr %data\n}\n",
        );
        out.push_str(
            "define void @glitch_string_retain(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %retain\nretain:\n  %refs_ptr = getelementptr i8, ptr %value, i64 -16\n  %refs = load i64, ptr %refs_ptr\n  %is_static = icmp uge i64 %refs, 1000000\n  br i1 %is_static, label %done, label %do_retain\ndo_retain:\n  %old_refs = atomicrmw add ptr %refs_ptr, i64 1 seq_cst\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str(
            "define void @glitch_string_release(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  %refs_ptr = getelementptr i8, ptr %value, i64 -16\n  %refs = load i64, ptr %refs_ptr\n  %is_static = icmp uge i64 %refs, 1000000\n  br i1 %is_static, label %done, label %do_release\ndo_release:\n  %old_refs = atomicrmw sub ptr %refs_ptr, i64 1 seq_cst\n  %destroy = icmp eq i64 %old_refs, 1\n  br i1 %destroy, label %free_node, label %done\nfree_node:\n  %node = getelementptr i8, ptr %value, i64 -16\n  call void @glitch_free(ptr %node)\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str(
            "define i1 @glitch_string_equals(ptr %left, ptr %right) {\nentry:\n  %same = icmp eq ptr %left, %right\n  br i1 %same, label %true_case, label %check_null\ncheck_null:\n  %left_null = icmp eq ptr %left, null\n  %right_null = icmp eq ptr %right, null\n  %any_null = or i1 %left_null, %right_null\n  br i1 %any_null, label %false_case, label %compare\ncompare:\n  %cmp = call i32 @strcmp(ptr %left, ptr %right)\n  %eq = icmp eq i32 %cmp, 0\n  ret i1 %eq\ntrue_case:\n  ret i1 true\nfalse_case:\n  ret i1 false\n}\n",
        );
        out.push_str(
            "define void @glitch_delegate_retain(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %retain\nretain:\n  %refs_ptr = getelementptr inbounds %glitch.delegate, ptr %value, i32 0, i32 0\n  %refs = load i64, ptr %refs_ptr\n  %is_static = icmp uge i64 %refs, 1000000\n  br i1 %is_static, label %done, label %do_retain\ndo_retain:\n  %old_refs = atomicrmw add ptr %refs_ptr, i64 1 seq_cst\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str(
            "define void @glitch_delegate_release(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  %refs_ptr = getelementptr inbounds %glitch.delegate, ptr %value, i32 0, i32 0\n  %refs = load i64, ptr %refs_ptr\n  %is_static = icmp uge i64 %refs, 1000000\n  br i1 %is_static, label %done, label %do_release\ndo_release:\n  %old_refs = atomicrmw sub ptr %refs_ptr, i64 1 seq_cst\n  %destroy = icmp eq i64 %old_refs, 1\n  br i1 %destroy, label %call_destroy, label %done\ncall_destroy:\n  %destroy_ptr = getelementptr inbounds %glitch.delegate, ptr %value, i32 0, i32 3\n  %destroy_fn = load ptr, ptr %destroy_ptr\n  %has_destroy = icmp ne ptr %destroy_fn, null\n  br i1 %has_destroy, label %invoke_destroy, label %free_delegate\ninvoke_destroy:\n  %env_ptr = getelementptr inbounds %glitch.delegate, ptr %value, i32 0, i32 2\n  %env = load ptr, ptr %env_ptr\n  call void %destroy_fn(ptr %env)\n  br label %free_delegate\nfree_delegate:\n  call void @glitch_free(ptr %value)\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str(
            "define void @glitch_destroy_boxed_scalar(ptr %object) {\nentry:\n  ret void\n}\n",
        );
        out.push_str(
            "define void @glitch_box_retain(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %retain\nretain:\n  %refs_ptr = getelementptr inbounds { i64, ptr, i64 }, ptr %value, i32 0, i32 0\n  %refs = load i64, ptr %refs_ptr\n  %is_static = icmp uge i64 %refs, 1000000\n  br i1 %is_static, label %done, label %do_retain\ndo_retain:\n  %old_refs = atomicrmw add ptr %refs_ptr, i64 1 seq_cst\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str(
            "define void @glitch_box_release(ptr %value) {\nentry:\n  %is_null = icmp eq ptr %value, null\n  br i1 %is_null, label %done, label %release\nrelease:\n  %refs_ptr = getelementptr inbounds { i64, ptr, i64 }, ptr %value, i32 0, i32 0\n  %refs = load i64, ptr %refs_ptr\n  %is_static = icmp uge i64 %refs, 1000000\n  br i1 %is_static, label %done, label %do_release\ndo_release:\n  %old_refs = atomicrmw sub ptr %refs_ptr, i64 1 seq_cst\n  %destroy = icmp eq i64 %old_refs, 1\n  br i1 %destroy, label %call_destroy, label %done\ncall_destroy:\n  %destroy_ptr = getelementptr inbounds { i64, ptr, i64 }, ptr %value, i32 0, i32 1\n  %destroy_fn = load ptr, ptr %destroy_ptr\n  %has_destroy = icmp ne ptr %destroy_fn, null\n  br i1 %has_destroy, label %invoke_destroy, label %free_box\ninvoke_destroy:\n  call void %destroy_fn(ptr %value)\n  br label %free_box\nfree_box:\n  call void @glitch_free(ptr %value)\n  br label %done\ndone:\n  ret void\n}\n",
        );
        out.push_str(
            "define i1 @glitch_object_equals(ptr %left, ptr %right) {\nentry:\n  %same = icmp eq ptr %left, %right\n  br i1 %same, label %true_case, label %check_null\ncheck_null:\n  %left_null = icmp eq ptr %left, null\n  %right_null = icmp eq ptr %right, null\n  %any_null = or i1 %left_null, %right_null\n  br i1 %any_null, label %false_case, label %load_drop\nload_drop:\n  %left_drop_ptr_ptr = getelementptr inbounds { i64, ptr }, ptr %left, i32 0, i32 1\n  %right_drop_ptr_ptr = getelementptr inbounds { i64, ptr }, ptr %right, i32 0, i32 1\n  %left_drop_ptr = load ptr, ptr %left_drop_ptr_ptr\n  %right_drop_ptr = load ptr, ptr %right_drop_ptr_ptr\n  %same_drop = icmp eq ptr %left_drop_ptr, %right_drop_ptr\n  br i1 %same_drop, label %check_boxed_scalar, label %false_case\n\
check_boxed_scalar:\n  %is_boxed_scalar = icmp eq ptr %left_drop_ptr, @glitch_destroy_boxed_scalar\n  br i1 %is_boxed_scalar, label %load_tag, label %false_case\n\
load_tag:\n  %left_tag_ptr = getelementptr inbounds { i64, ptr, i32 }, ptr %left, i32 0, i32 2\n  %right_tag_ptr = getelementptr inbounds { i64, ptr, i32 }, ptr %right, i32 0, i32 2\n  %left_tag = load i32, ptr %left_tag_ptr\n  %right_tag = load i32, ptr %right_tag_ptr\n  %same_tag = icmp eq i32 %left_tag, %right_tag\n  br i1 %same_tag, label %dispatch, label %false_case\n\
dispatch:\n  switch i32 %left_tag, label %false_case [\n    i32 1, label %compare_bool\n    i32 2, label %compare_byte\n    i32 3, label %compare_short\n    i32 4, label %compare_int\n    i32 5, label %compare_uint\n    i32 6, label %compare_long\n    i32 7, label %compare_double\n    i32 8, label %compare_decimal\n  ]\n\
compare_bool:\n  %left_bool_ptr = getelementptr inbounds { i64, ptr, i32, i1 }, ptr %left, i32 0, i32 3\n  %right_bool_ptr = getelementptr inbounds { i64, ptr, i32, i1 }, ptr %right, i32 0, i32 3\n  %left_bool = load i1, ptr %left_bool_ptr\n  %right_bool = load i1, ptr %right_bool_ptr\n  %bool_eq = icmp eq i1 %left_bool, %right_bool\n  ret i1 %bool_eq\n\
compare_byte:\n  %left_byte_ptr = getelementptr inbounds { i64, ptr, i32, i8 }, ptr %left, i32 0, i32 3\n  %right_byte_ptr = getelementptr inbounds { i64, ptr, i32, i8 }, ptr %right, i32 0, i32 3\n  %left_byte = load i8, ptr %left_byte_ptr\n  %right_byte = load i8, ptr %right_byte_ptr\n  %byte_eq = icmp eq i8 %left_byte, %right_byte\n  ret i1 %byte_eq\n\
compare_short:\n  %left_short_ptr = getelementptr inbounds { i64, ptr, i32, i16 }, ptr %left, i32 0, i32 3\n  %right_short_ptr = getelementptr inbounds { i64, ptr, i32, i16 }, ptr %right, i32 0, i32 3\n  %left_short = load i16, ptr %left_short_ptr\n  %right_short = load i16, ptr %right_short_ptr\n  %short_eq = icmp eq i16 %left_short, %right_short\n  ret i1 %short_eq\n\
compare_int:\n  %left_int_ptr = getelementptr inbounds { i64, ptr, i32, i32 }, ptr %left, i32 0, i32 3\n  %right_int_ptr = getelementptr inbounds { i64, ptr, i32, i32 }, ptr %right, i32 0, i32 3\n  %left_int = load i32, ptr %left_int_ptr\n  %right_int = load i32, ptr %right_int_ptr\n  %int_eq = icmp eq i32 %left_int, %right_int\n  ret i1 %int_eq\n\
compare_uint:\n  %left_uint_ptr = getelementptr inbounds { i64, ptr, i32, i32 }, ptr %left, i32 0, i32 3\n  %right_uint_ptr = getelementptr inbounds { i64, ptr, i32, i32 }, ptr %right, i32 0, i32 3\n  %left_uint = load i32, ptr %left_uint_ptr\n  %right_uint = load i32, ptr %right_uint_ptr\n  %uint_eq = icmp eq i32 %left_uint, %right_uint\n  ret i1 %uint_eq\n\
compare_long:\n  %left_long_ptr = getelementptr inbounds { i64, ptr, i32, i64 }, ptr %left, i32 0, i32 3\n  %right_long_ptr = getelementptr inbounds { i64, ptr, i32, i64 }, ptr %right, i32 0, i32 3\n  %left_long = load i64, ptr %left_long_ptr\n  %right_long = load i64, ptr %right_long_ptr\n  %long_eq = icmp eq i64 %left_long, %right_long\n  ret i1 %long_eq\n\
compare_double:\n  %left_double_ptr = getelementptr inbounds { i64, ptr, i32, double }, ptr %left, i32 0, i32 3\n  %right_double_ptr = getelementptr inbounds { i64, ptr, i32, double }, ptr %right, i32 0, i32 3\n  %left_double = load double, ptr %left_double_ptr\n  %right_double = load double, ptr %right_double_ptr\n  %double_eq = fcmp oeq double %left_double, %right_double\n  ret i1 %double_eq\n\
compare_decimal:\n  %left_decimal_ptr = getelementptr inbounds { i64, ptr, i32, double }, ptr %left, i32 0, i32 3\n  %right_decimal_ptr = getelementptr inbounds { i64, ptr, i32, double }, ptr %right, i32 0, i32 3\n  %left_decimal = load double, ptr %left_decimal_ptr\n  %right_decimal = load double, ptr %right_decimal_ptr\n  %decimal_eq = fcmp oeq double %left_decimal, %right_decimal\n  ret i1 %decimal_eq\n\
true_case:\n  ret i1 true\n\
false_case:\n  ret i1 false\n}\n",
        );
        out.push_str("@.fmt_i64 = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\"\n");
        out.push_str("@.fmt_i32 = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\"\n");
        out.push_str("@.fmt_double = private unnamed_addr constant [4 x i8] c\"%f\\0A\\00\"\n");
        out.push_str("@.fmt_str = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\"\n");
        out.push_str("@.fmt_json_i64 = private unnamed_addr constant [5 x i8] c\"%lld\\00\"\n");
        out.push_str("@.fmt_json_double = private unnamed_addr constant [6 x i8] c\"%.17g\\00\"\n");
        out.push_str("@.json_true = private unnamed_addr constant [5 x i8] c\"true\\00\"\n");
        out.push_str("@.json_false = private unnamed_addr constant [6 x i8] c\"false\\00\"\n");
        out.push_str("@.env_report_leaks = private unnamed_addr constant [20 x i8] c\"GLITCH_REPORT_LEAKS\\00\"\n");
        for global in &emitter.globals {
            out.push_str(global);
        }
        out.push('\n');
        out.push_str(&emitter.body);
        Ok(out)
    }

