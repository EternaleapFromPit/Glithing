; ModuleID = 'glitching'
%glitch.array = type { i64, ptr }
%glitch.list = type { i64, i64, ptr }
%glitch.dict = type { i64, i64, ptr, ptr }
%glitch.string_node = type { i64, i64, [0 x i8] }
%glitch.delegate = type { i64, ptr, ptr, ptr }
%glitch.Box__g1__t0 = type { i64, ptr, ptr }
%glitch.Box_int___g0__t0 = type { i64, ptr, i32 }
declare i32 @printf(ptr, ...)
declare ptr @calloc(i64, i64)
declare ptr @realloc(ptr, i64)
declare void @free(ptr)
declare i32 @strcmp(ptr, ptr)
declare i32 @strncmp(ptr, ptr, i64)
declare i64 @strlen(ptr)
declare i64 @strtoll(ptr, ptr, i32)
declare double @strtod(ptr, ptr)
declare ptr @strstr(ptr, ptr)
declare i32 @snprintf(ptr, i64, ptr, ...)
declare ptr @memcpy(ptr, ptr, i64)
declare ptr @getenv(ptr)
declare void @GlitchRestHost_Run(ptr, i32, i32, ptr, ptr)
declare ptr @System_IO_File_ReadAllText(ptr)
declare void @System_IO_File_WriteAllText(ptr, ptr)
declare void @System_Console_WriteLine_String(ptr)
declare void @System_Console_WriteLine_I64(i64)
declare void @System_Console_WriteLine_Double(double)
declare void @System_Console_WriteLine_Bool(i1)
declare ptr @GlitchString_Lock()
declare void @GlitchString_Unlock(ptr)
@glitch_live_allocations = internal global i64 0
@glitch_exception_pending = internal global ptr null
%glitch.task = type { i32, ptr }
define ptr @glitch_task_from_result_ptr(ptr %result) {
entry:
%task = call ptr @glitch_calloc(i64 1, i64 16)
%completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0
store i32 1, ptr %completed_ptr
%result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1
store ptr %result, ptr %result_ptr
ret ptr %task
}
define ptr @glitch_task_from_result_i32(i32 %result) {
entry:
%val_ptr = inttoptr i32 %result to ptr
%task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)
ret ptr %task
}
define ptr @glitch_task_from_result_i64(i64 %result) {
entry:
%val_ptr = inttoptr i64 %result to ptr
%task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)
ret ptr %task
}
define ptr @glitch_task_from_result_double(double %result) {
entry:
%cast = bitcast double %result to i64
%val_ptr = inttoptr i64 %cast to ptr
%task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)
ret ptr %task
}
define ptr @glitch_task_get_result_ptr(ptr %task) {
entry:
%is_null = icmp eq ptr %task, null
br i1 %is_null, label %null_case, label %normal_case
null_case:
ret ptr null
normal_case:
%result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1
%result = load ptr, ptr %result_ptr
ret ptr %result
}
define i32 @glitch_task_get_result_i32(ptr %task) {
entry:
%ptr = call ptr @glitch_task_get_result_ptr(ptr %task)
%val = ptrtoint ptr %ptr to i32
ret i32 %val
}
define i64 @glitch_task_get_result_i64(ptr %task) {
entry:
%ptr = call ptr @glitch_task_get_result_ptr(ptr %task)
%val = ptrtoint ptr %ptr to i64
ret i64 %val
}
define double @glitch_task_get_result_double(ptr %task) {
entry:
%ptr = call ptr @glitch_task_get_result_ptr(ptr %task)
%val = ptrtoint ptr %ptr to i64
%res = bitcast i64 %val to double
ret double %res
}
define ptr @glitch_calloc(i64 %count, i64 %size) {
entry:
  %value = call ptr @calloc(i64 %count, i64 %size)
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %count_alloc
count_alloc:
  %old_live = atomicrmw add ptr @glitch_live_allocations, i64 1 seq_cst
  br label %done
done:
  ret ptr %value
}
define ptr @glitch_realloc(ptr %old, i64 %size) {
entry:
  %value = call ptr @realloc(ptr %old, i64 %size)
  %old_null = icmp eq ptr %old, null
  %new_valid = icmp ne ptr %value, null
  %count_it = and i1 %old_null, %new_valid
  br i1 %count_it, label %count_alloc, label %done
count_alloc:
  %old_live = atomicrmw add ptr @glitch_live_allocations, i64 1 seq_cst
  br label %done
done:
  ret ptr %value
}
define void @glitch_free(ptr %value) {
entry:
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %release
release:
  call void @free(ptr %value)
  %old_live = atomicrmw sub ptr @glitch_live_allocations, i64 1 seq_cst
  br label %done
done:
  ret void
}
define i64 @glitch_string_length(ptr %value) {
entry:
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %empty, label %read_len
empty:
  ret i64 0
read_len:
  %length_ptr = getelementptr i8, ptr %value, i64 -8
  %length = load i64, ptr %length_ptr
  ret i64 %length
}
define i1 @glitch_route_match(ptr %template, ptr %path) {
entry:
  %template_index = alloca i64
  %path_index = alloca i64
  store i64 0, ptr %template_index
  store i64 0, ptr %path_index
  br label %scan
scan:
  %ti = load i64, ptr %template_index
  %pi = load i64, ptr %path_index
  %template_ptr = getelementptr inbounds i8, ptr %template, i64 %ti
  %path_ptr = getelementptr inbounds i8, ptr %path, i64 %pi
  %template_char = load i8, ptr %template_ptr
  %path_char = load i8, ptr %path_ptr
  %template_done = icmp eq i8 %template_char, 0
  br i1 %template_done, label %finish, label %inspect
inspect:
  %parameter_start = icmp eq i8 %template_char, 123
  br i1 %parameter_start, label %skip_parameter_name, label %literal
literal:
  %same = icmp eq i8 %template_char, %path_char
  br i1 %same, label %advance_both, label %no_match
advance_both:
  %ti_next = add i64 %ti, 1
  %pi_next = add i64 %pi, 1
  store i64 %ti_next, ptr %template_index
  store i64 %pi_next, ptr %path_index
  br label %scan
skip_parameter_name:
  %parameter_template_index = alloca i64
  %after_open = add i64 %ti, 1
  store i64 %after_open, ptr %parameter_template_index
  br label %parameter_name_loop
parameter_name_loop:
  %parameter_ti = load i64, ptr %parameter_template_index
  %parameter_template_ptr = getelementptr inbounds i8, ptr %template, i64 %parameter_ti
  %parameter_template_char = load i8, ptr %parameter_template_ptr
  %parameter_name_done = icmp eq i8 %parameter_template_char, 125
  %parameter_template_done = icmp eq i8 %parameter_template_char, 0
  br i1 %parameter_template_done, label %no_match, label %parameter_name_check
parameter_name_check:
  br i1 %parameter_name_done, label %consume_parameter, label %advance_parameter_name
advance_parameter_name:
  %parameter_ti_next = add i64 %parameter_ti, 1
  store i64 %parameter_ti_next, ptr %parameter_template_index
  br label %parameter_name_loop
consume_parameter:
  %after_close = add i64 %parameter_ti, 1
  store i64 %after_close, ptr %template_index
  %parameter_path_start = load i64, ptr %path_index
  %parameter_first_ptr = getelementptr inbounds i8, ptr %path, i64 %parameter_path_start
  %parameter_first = load i8, ptr %parameter_first_ptr
  %parameter_empty = icmp eq i8 %parameter_first, 0
  %parameter_slash = icmp eq i8 %parameter_first, 47
  %parameter_invalid = or i1 %parameter_empty, %parameter_slash
  br i1 %parameter_invalid, label %no_match, label %parameter_path_loop
parameter_path_loop:
  %parameter_pi = load i64, ptr %path_index
  %parameter_path_ptr = getelementptr inbounds i8, ptr %path, i64 %parameter_pi
  %parameter_path_char = load i8, ptr %parameter_path_ptr
  %parameter_path_done = icmp eq i8 %parameter_path_char, 0
  %parameter_path_query = icmp eq i8 %parameter_path_char, 63
  %parameter_path_slash = icmp eq i8 %parameter_path_char, 47
  %parameter_path_end = or i1 %parameter_path_done, %parameter_path_query
  %parameter_segment_done = or i1 %parameter_path_end, %parameter_path_slash
  br i1 %parameter_segment_done, label %scan, label %advance_parameter_path
advance_parameter_path:
  %parameter_pi_next = add i64 %parameter_pi, 1
  store i64 %parameter_pi_next, ptr %path_index
  br label %parameter_path_loop
finish:
  %path_done = icmp eq i8 %path_char, 0
  %path_query = icmp eq i8 %path_char, 63
  %path_finished = or i1 %path_done, %path_query
  ret i1 %path_finished
no_match:
  ret i1 false
}
define ptr @glitch_path_segment_string(ptr %path, i64 %target) {
entry:
  %position = alloca i64
  %segment = alloca i64
  %start = alloca i64
  store i64 0, ptr %position
  store i64 0, ptr %segment
  br label %skip_slashes
skip_slashes:
  %skip_position = load i64, ptr %position
  %skip_ptr = getelementptr inbounds i8, ptr %path, i64 %skip_position
  %skip_char = load i8, ptr %skip_ptr
  %skip_end = icmp eq i8 %skip_char, 0
  %skip_query = icmp eq i8 %skip_char, 63
  %skip_done = or i1 %skip_end, %skip_query
  br i1 %skip_done, label %missing, label %skip_check
skip_check:
  %is_slash = icmp eq i8 %skip_char, 47
  br i1 %is_slash, label %advance_slash, label %begin_segment
advance_slash:
  %after_slash = add i64 %skip_position, 1
  store i64 %after_slash, ptr %position
  br label %skip_slashes
begin_segment:
  store i64 %skip_position, ptr %start
  %current_segment = load i64, ptr %segment
  %is_target = icmp eq i64 %current_segment, %target
  br i1 %is_target, label %scan_target, label %skip_segment
scan_target:
  %target_position = load i64, ptr %position
  %target_ptr = getelementptr inbounds i8, ptr %path, i64 %target_position
  %target_char = load i8, ptr %target_ptr
  %target_end = icmp eq i8 %target_char, 0
  %target_query = icmp eq i8 %target_char, 63
  %target_slash = icmp eq i8 %target_char, 47
  %target_path_end = or i1 %target_end, %target_query
  %target_done = or i1 %target_path_end, %target_slash
  br i1 %target_done, label %copy_target, label %advance_target
advance_target:
  %target_next = add i64 %target_position, 1
  store i64 %target_next, ptr %position
  br label %scan_target
copy_target:
  %target_start = load i64, ptr %start
  %target_length = sub i64 %target_position, %target_start
  %target_data = call ptr @glitch_string_allocate(i64 %target_length)
  %target_source = getelementptr inbounds i8, ptr %path, i64 %target_start
  call ptr @memcpy(ptr %target_data, ptr %target_source, i64 %target_length)
  ret ptr %target_data
skip_segment:
  %segment_position = load i64, ptr %position
  %segment_ptr = getelementptr inbounds i8, ptr %path, i64 %segment_position
  %segment_char = load i8, ptr %segment_ptr
  %segment_end = icmp eq i8 %segment_char, 0
  %segment_query = icmp eq i8 %segment_char, 63
  %segment_done = or i1 %segment_end, %segment_query
  br i1 %segment_done, label %missing, label %segment_check
segment_check:
  %segment_slash = icmp eq i8 %segment_char, 47
  br i1 %segment_slash, label %next_segment, label %advance_segment
advance_segment:
  %segment_next_position = add i64 %segment_position, 1
  store i64 %segment_next_position, ptr %position
  br label %skip_segment
next_segment:
  %next_segment_value = add i64 %current_segment, 1
  store i64 %next_segment_value, ptr %segment
  br label %skip_slashes
missing:
  %empty = call ptr @glitch_string_allocate(i64 0)
  ret ptr %empty
}
define i64 @glitch_path_segment_i64(ptr %path, i64 %target) {
entry:
  %text = call ptr @glitch_path_segment_string(ptr %path, i64 %target)
  %value = call i64 @strtoll(ptr %text, ptr null, i32 10)
  call void @glitch_string_release(ptr %text)
  ret i64 %value
}
define ptr @glitch_query_value_string(ptr %path, ptr %key, i64 %key_length) {
entry:
  %position = alloca i64
  store i64 0, ptr %position
  br label %find_query
find_query:
  %find_position = load i64, ptr %position
  %find_ptr = getelementptr inbounds i8, ptr %path, i64 %find_position
  %find_char = load i8, ptr %find_ptr
  %find_end = icmp eq i8 %find_char, 0
  br i1 %find_end, label %query_missing, label %find_check
find_check:
  %find_question = icmp eq i8 %find_char, 63
  br i1 %find_question, label %next_pair, label %advance_find
advance_find:
  %find_next = add i64 %find_position, 1
  store i64 %find_next, ptr %position
  br label %find_query
next_pair:
  %pair_position = load i64, ptr %position
  %pair_start = add i64 %pair_position, 1
  store i64 %pair_start, ptr %position
  br label %inspect_pair
inspect_pair:
  %inspect_position = load i64, ptr %position
  %inspect_ptr = getelementptr inbounds i8, ptr %path, i64 %inspect_position
  %inspect_char = load i8, ptr %inspect_ptr
  %inspect_end = icmp eq i8 %inspect_char, 0
  br i1 %inspect_end, label %query_missing, label %compare_key
compare_key:
  %key_cmp = call i32 @strncmp(ptr %inspect_ptr, ptr %key, i64 %key_length)
  %key_equal = icmp eq i32 %key_cmp, 0
  %after_key_position = add i64 %inspect_position, %key_length
  %after_key_ptr = getelementptr inbounds i8, ptr %path, i64 %after_key_position
  %after_key_char = load i8, ptr %after_key_ptr
  %has_equals = icmp eq i8 %after_key_char, 61
  %key_match = and i1 %key_equal, %has_equals
  br i1 %key_match, label %scan_value, label %skip_pair
scan_value:
  %value_start = add i64 %after_key_position, 1
  store i64 %value_start, ptr %position
  br label %value_loop
value_loop:
  %value_position = load i64, ptr %position
  %value_ptr = getelementptr inbounds i8, ptr %path, i64 %value_position
  %value_char = load i8, ptr %value_ptr
  %value_end = icmp eq i8 %value_char, 0
  %value_amp = icmp eq i8 %value_char, 38
  %value_done = or i1 %value_end, %value_amp
  br i1 %value_done, label %copy_value, label %advance_value
advance_value:
  %value_next = add i64 %value_position, 1
  store i64 %value_next, ptr %position
  br label %value_loop
copy_value:
  %value_length = sub i64 %value_position, %value_start
  %value_data = call ptr @glitch_string_allocate(i64 %value_length)
  %value_source = getelementptr inbounds i8, ptr %path, i64 %value_start
  call ptr @memcpy(ptr %value_data, ptr %value_source, i64 %value_length)
  ret ptr %value_data
skip_pair:
  %skip_pair_position = load i64, ptr %position
  %skip_pair_ptr = getelementptr inbounds i8, ptr %path, i64 %skip_pair_position
  %skip_pair_char = load i8, ptr %skip_pair_ptr
  %skip_pair_end = icmp eq i8 %skip_pair_char, 0
  br i1 %skip_pair_end, label %query_missing, label %skip_pair_check
skip_pair_check:
  %skip_pair_amp = icmp eq i8 %skip_pair_char, 38
  br i1 %skip_pair_amp, label %next_pair, label %advance_skip_pair
advance_skip_pair:
  %skip_pair_next = add i64 %skip_pair_position, 1
  store i64 %skip_pair_next, ptr %position
  br label %skip_pair
query_missing:
  %query_empty = call ptr @glitch_string_allocate(i64 0)
  ret ptr %query_empty
}
define i64 @glitch_query_value_i64(ptr %path, ptr %key, i64 %key_length) {
entry:
  %text = call ptr @glitch_query_value_string(ptr %path, ptr %key, i64 %key_length)
  %value = call i64 @strtoll(ptr %text, ptr null, i32 10)
  call void @glitch_string_release(ptr %text)
  ret i64 %value
}
define ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length) {
entry:
  %match = call ptr @strstr(ptr %json, ptr %token)
  %missing = icmp eq ptr %match, null
  br i1 %missing, label %json_missing, label %after_token
  
after_token:
  %cursor = alloca ptr
  %token_end = getelementptr inbounds i8, ptr %match, i64 %token_length
  store ptr %token_end, ptr %cursor
  br label %find_colon
find_colon:
  %colon_ptr = load ptr, ptr %cursor
  %colon_char = load i8, ptr %colon_ptr
  %colon_end = icmp eq i8 %colon_char, 0
  br i1 %colon_end, label %json_missing, label %colon_check
colon_check:
  %is_colon = icmp eq i8 %colon_char, 58
  br i1 %is_colon, label %after_colon, label %advance_colon
advance_colon:
  %colon_next = getelementptr inbounds i8, ptr %colon_ptr, i64 1
  store ptr %colon_next, ptr %cursor
  br label %find_colon
after_colon:
  %value_candidate = getelementptr inbounds i8, ptr %colon_ptr, i64 1
  store ptr %value_candidate, ptr %cursor
  br label %skip_json_space
skip_json_space:
  %space_ptr = load ptr, ptr %cursor
  %space_char = load i8, ptr %space_ptr
  %is_space = icmp eq i8 %space_char, 32
  %is_tab = icmp eq i8 %space_char, 9
  %is_cr = icmp eq i8 %space_char, 13
  %is_lf = icmp eq i8 %space_char, 10
  %space_a = or i1 %is_space, %is_tab
  %space_b = or i1 %is_cr, %is_lf
  %whitespace = or i1 %space_a, %space_b
  br i1 %whitespace, label %advance_json_space, label %value_kind
advance_json_space:
  %space_next = getelementptr inbounds i8, ptr %space_ptr, i64 1
  store ptr %space_next, ptr %cursor
  br label %skip_json_space
value_kind:
  %is_quote = icmp eq i8 %space_char, 34
  br i1 %is_quote, label %quoted_start, label %plain_start
quoted_start:
  %quoted_value = getelementptr inbounds i8, ptr %space_ptr, i64 1
  store ptr %quoted_value, ptr %cursor
  br label %scan_quoted
scan_quoted:
  %quoted_ptr = load ptr, ptr %cursor
  %quoted_char = load i8, ptr %quoted_ptr
  %quoted_done = icmp eq i8 %quoted_char, 34
  %quoted_end = icmp eq i8 %quoted_char, 0
  br i1 %quoted_end, label %json_missing, label %quoted_check
quoted_check:
  br i1 %quoted_done, label %copy_quoted, label %advance_quoted
advance_quoted:
  %quoted_next = getelementptr inbounds i8, ptr %quoted_ptr, i64 1
  store ptr %quoted_next, ptr %cursor
  br label %scan_quoted
copy_quoted:
  %quoted_start_int = ptrtoint ptr %quoted_value to i64
  %quoted_end_int = ptrtoint ptr %quoted_ptr to i64
  %quoted_length = sub i64 %quoted_end_int, %quoted_start_int
  %quoted_data = call ptr @glitch_string_allocate(i64 %quoted_length)
  call ptr @memcpy(ptr %quoted_data, ptr %quoted_value, i64 %quoted_length)
  ret ptr %quoted_data
plain_start:
  store ptr %space_ptr, ptr %cursor
  br label %scan_plain
scan_plain:
  %plain_ptr = load ptr, ptr %cursor
  %plain_char = load i8, ptr %plain_ptr
  %plain_end = icmp eq i8 %plain_char, 0
  %plain_comma = icmp eq i8 %plain_char, 44
  %plain_brace = icmp eq i8 %plain_char, 125
  %plain_space = icmp eq i8 %plain_char, 32
  %plain_a = or i1 %plain_end, %plain_comma
  %plain_b = or i1 %plain_brace, %plain_space
  %plain_done = or i1 %plain_a, %plain_b
  br i1 %plain_done, label %copy_plain, label %advance_plain
advance_plain:
  %plain_next = getelementptr inbounds i8, ptr %plain_ptr, i64 1
  store ptr %plain_next, ptr %cursor
  br label %scan_plain
copy_plain:
  %plain_start_int = ptrtoint ptr %space_ptr to i64
  %plain_end_int = ptrtoint ptr %plain_ptr to i64
  %plain_length = sub i64 %plain_end_int, %plain_start_int
  %plain_data = call ptr @glitch_string_allocate(i64 %plain_length)
  call ptr @memcpy(ptr %plain_data, ptr %space_ptr, i64 %plain_length)
  ret ptr %plain_data
json_missing:
  %json_empty = call ptr @glitch_string_allocate(i64 0)
  ret ptr %json_empty
}
define i64 @glitch_json_value_i64(ptr %json, ptr %token, i64 %token_length) {
entry:
  %text = call ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length)
  %value = call i64 @strtoll(ptr %text, ptr null, i32 10)
  call void @glitch_string_release(ptr %text)
  ret i64 %value
}
define double @glitch_json_value_double(ptr %json, ptr %token, i64 %token_length) {
entry:
  %text = call ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length)
  %value = call double @strtod(ptr %text, ptr null)
  call void @glitch_string_release(ptr %text)
  ret double %value
}
define i1 @glitch_json_value_bool(ptr %json, ptr %token, i64 %token_length) {
entry:
  %text = call ptr @glitch_json_value_string(ptr %json, ptr %token, i64 %token_length)
  %first = load i8, ptr %text
  %is_t = icmp eq i8 %first, 116
  %is_T = icmp eq i8 %first, 84
  %is_one = icmp eq i8 %first, 49
  %is_true_text = or i1 %is_t, %is_T
  %value = or i1 %is_true_text, %is_one
  call void @glitch_string_release(ptr %text)
  ret i1 %value
}
define ptr @glitch_i64_to_string(i64 %value) {
entry:
  %buffer = alloca [32 x i8]
  %buffer_ptr = getelementptr inbounds [32 x i8], ptr %buffer, i64 0, i64 0
  %length_i32 = call i32 (ptr, i64, ptr, ...) @snprintf(ptr %buffer_ptr, i64 32, ptr getelementptr inbounds ([5 x i8], ptr @.fmt_json_i64, i64 0, i64 0), i64 %value)
  %length = sext i32 %length_i32 to i64
  %text = call ptr @glitch_string_allocate(i64 %length)
  call ptr @memcpy(ptr %text, ptr %buffer_ptr, i64 %length)
  ret ptr %text
}
define ptr @glitch_double_to_string(double %value) {
entry:
  %buffer = alloca [64 x i8]
  %buffer_ptr = getelementptr inbounds [64 x i8], ptr %buffer, i64 0, i64 0
  %length_i32 = call i32 (ptr, i64, ptr, ...) @snprintf(ptr %buffer_ptr, i64 64, ptr getelementptr inbounds ([6 x i8], ptr @.fmt_json_double, i64 0, i64 0), double %value)
  %length = sext i32 %length_i32 to i64
  %text = call ptr @glitch_string_allocate(i64 %length)
  call ptr @memcpy(ptr %text, ptr %buffer_ptr, i64 %length)
  ret ptr %text
}
define ptr @glitch_string_allocate(i64 %length) {
entry:
  %with_null = add i64 %length, 1
  %size = add i64 %with_null, 16
  %node = call ptr @glitch_calloc(i64 1, i64 %size)
  %refs_ptr = getelementptr inbounds %glitch.string_node, ptr %node, i32 0, i32 0
  store i64 1, ptr %refs_ptr
  %length_ptr = getelementptr inbounds %glitch.string_node, ptr %node, i32 0, i32 1
  store i64 %length, ptr %length_ptr
  %data = getelementptr inbounds %glitch.string_node, ptr %node, i32 0, i32 2, i64 0
  ret ptr %data
}
define ptr @glitch_string_concat(ptr %left, ptr %right) {
entry:
  %left_length = call i64 @glitch_string_length(ptr %left)
  %right_length = call i64 @glitch_string_length(ptr %right)
  %length = add i64 %left_length, %right_length
  %data = call ptr @glitch_string_allocate(i64 %length)
  %left_empty = icmp eq i64 %left_length, 0
  br i1 %left_empty, label %copy_right, label %copy_left
copy_left:
  call ptr @memcpy(ptr %data, ptr %left, i64 %left_length)
  br label %copy_right
copy_right:
  %right_empty = icmp eq i64 %right_length, 0
  br i1 %right_empty, label %done, label %copy_right_data
copy_right_data:
  %right_target = getelementptr inbounds i8, ptr %data, i64 %left_length
  call ptr @memcpy(ptr %right_target, ptr %right, i64 %right_length)
  br label %done
done:
  ret ptr %data
}
define void @glitch_string_retain(ptr %value) {
entry:
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %retain
retain:
  %refs_ptr = getelementptr i8, ptr %value, i64 -16
  %refs = load i64, ptr %refs_ptr
  %is_static = icmp uge i64 %refs, 1000000
  br i1 %is_static, label %done, label %do_retain
do_retain:
  %old_refs = atomicrmw add ptr %refs_ptr, i64 1 seq_cst
  br label %done
done:
  ret void
}
define void @glitch_string_release(ptr %value) {
entry:
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %release
release:
  %refs_ptr = getelementptr i8, ptr %value, i64 -16
  %refs = load i64, ptr %refs_ptr
  %is_static = icmp uge i64 %refs, 1000000
  br i1 %is_static, label %done, label %do_release
do_release:
  %old_refs = atomicrmw sub ptr %refs_ptr, i64 1 seq_cst
  %destroy = icmp eq i64 %old_refs, 1
  br i1 %destroy, label %free_node, label %done
free_node:
  %node = getelementptr i8, ptr %value, i64 -16
  call void @glitch_free(ptr %node)
  br label %done
done:
  ret void
}
define void @glitch_delegate_retain(ptr %value) {
entry:
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %retain
retain:
  %refs_ptr = getelementptr inbounds %glitch.delegate, ptr %value, i32 0, i32 0
  %refs = load i64, ptr %refs_ptr
  %is_static = icmp uge i64 %refs, 1000000
  br i1 %is_static, label %done, label %do_retain
do_retain:
  %old_refs = atomicrmw add ptr %refs_ptr, i64 1 seq_cst
  br label %done
done:
  ret void
}
define void @glitch_delegate_release(ptr %value) {
entry:
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %release
release:
  %refs_ptr = getelementptr inbounds %glitch.delegate, ptr %value, i32 0, i32 0
  %refs = load i64, ptr %refs_ptr
  %is_static = icmp uge i64 %refs, 1000000
  br i1 %is_static, label %done, label %do_release
do_release:
  %old_refs = atomicrmw sub ptr %refs_ptr, i64 1 seq_cst
  %destroy = icmp eq i64 %old_refs, 1
  br i1 %destroy, label %call_destroy, label %done
call_destroy:
  %destroy_ptr = getelementptr inbounds %glitch.delegate, ptr %value, i32 0, i32 3
  %destroy_fn = load ptr, ptr %destroy_ptr
  %has_destroy = icmp ne ptr %destroy_fn, null
  br i1 %has_destroy, label %invoke_destroy, label %free_delegate
invoke_destroy:
  %env_ptr = getelementptr inbounds %glitch.delegate, ptr %value, i32 0, i32 2
  %env = load ptr, ptr %env_ptr
  call void %destroy_fn(ptr %env)
  br label %free_delegate
free_delegate:
  call void @glitch_free(ptr %value)
  br label %done
done:
  ret void
}
define void @glitch_box_retain(ptr %value) {
entry:
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %retain
retain:
  %refs_ptr = getelementptr inbounds { i64, ptr, i64 }, ptr %value, i32 0, i32 0
  %refs = load i64, ptr %refs_ptr
  %is_static = icmp uge i64 %refs, 1000000
  br i1 %is_static, label %done, label %do_retain
do_retain:
  %old_refs = atomicrmw add ptr %refs_ptr, i64 1 seq_cst
  br label %done
done:
  ret void
}
define void @glitch_box_release(ptr %value) {
entry:
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %release
release:
  %refs_ptr = getelementptr inbounds { i64, ptr, i64 }, ptr %value, i32 0, i32 0
  %refs = load i64, ptr %refs_ptr
  %is_static = icmp uge i64 %refs, 1000000
  br i1 %is_static, label %done, label %do_release
do_release:
  %old_refs = atomicrmw sub ptr %refs_ptr, i64 1 seq_cst
  %destroy = icmp eq i64 %old_refs, 1
  br i1 %destroy, label %call_destroy, label %done
call_destroy:
  %destroy_ptr = getelementptr inbounds { i64, ptr, i64 }, ptr %value, i32 0, i32 1
  %destroy_fn = load ptr, ptr %destroy_ptr
  %has_destroy = icmp ne ptr %destroy_fn, null
  br i1 %has_destroy, label %invoke_destroy, label %free_box
invoke_destroy:
  call void %destroy_fn(ptr %value)
  br label %free_box
free_box:
  call void @glitch_free(ptr %value)
  br label %done
done:
  ret void
}
@.fmt_i64 = private unnamed_addr constant [6 x i8] c"%lld\0A\00"
@.fmt_i32 = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@.fmt_double = private unnamed_addr constant [4 x i8] c"%f\0A\00"
@.fmt_str = private unnamed_addr constant [4 x i8] c"%s\0A\00"
@.fmt_json_i64 = private unnamed_addr constant [5 x i8] c"%lld\00"
@.fmt_json_double = private unnamed_addr constant [6 x i8] c"%.17g\00"
@.json_true = private unnamed_addr constant [5 x i8] c"true\00"
@.json_false = private unnamed_addr constant [6 x i8] c"false\00"
@.env_report_leaks = private unnamed_addr constant [20 x i8] c"GLITCH_REPORT_LEAKS\00"
@.str.0 = private unnamed_addr constant { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"404\00" }

define void @glitch_destroy_Box__g1__t0(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Box__g1__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Box__g1__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Box__g1__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Box__g1__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Box__g1__t0, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Box__g1__t0(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Box_int___g0__t0(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Box_int___g0__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Box_int___g0__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Box_int___g0__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Box_int___g0__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Box_int___g0__t0, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Box_int___g0__t0(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @Box__g1__t0_ctor(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Box__g1__t0, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Box__g1__t0_Get__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Box__g1__t0, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define i32 @main() {
entry:
  %t0 = alloca ptr
  store ptr null, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_drop_Box_int___g0__t0(ptr %t1)
  store ptr null, ptr %t0
  %t2 = load ptr, ptr %t0
  %t3 = call i32 @Box__g1__t0_Get__g0__owner_int(ptr %t2)
  %t4 = load ptr, ptr @glitch_exception_pending
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %exception_unwind, label %call_continue_0
call_continue_0:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_i32, i64 0, i64 0), i32 %t3)
  %t6 = load ptr, ptr %t0
  call void @glitch_drop_Box_int___g0__t0(ptr %t6)
  %t7 = load atomic i64, ptr @glitch_live_allocations seq_cst, align 8
  %t8 = icmp ne i64 %t7, 0
  %t9 = load ptr, ptr @glitch_exception_pending
  %t10 = icmp ne ptr %t9, null
  %t11 = or i1 %t8, %t10
  %t12 = zext i1 %t11 to i32
  %t13 = call ptr @getenv(ptr @.env_report_leaks)
  %t14 = icmp ne ptr %t13, null
  br i1 %t14, label %report_leaks_1, label %main_return_2
report_leaks_1:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t7)
  br label %main_return_2
main_return_2:
  ret i32 %t12
exception_unwind:
  %t15 = load ptr, ptr %t0
  call void @glitch_drop_Box_int___g0__t0(ptr %t15)
  %t16 = load atomic i64, ptr @glitch_live_allocations seq_cst, align 8
  %t17 = icmp ne i64 %t16, 0
  %t18 = load ptr, ptr @glitch_exception_pending
  %t19 = icmp ne ptr %t18, null
  %t20 = or i1 %t17, %t19
  %t21 = zext i1 %t20 to i32
  %t22 = call ptr @getenv(ptr @.env_report_leaks)
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %report_leaks_3, label %main_return_4
report_leaks_3:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t16)
  br label %main_return_4
main_return_4:
  ret i32 %t21
}

define void @Box__g1__t0_ctor__owner_int(ptr %this, i32 %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %value, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Box__g1__t0, ptr %t2, i32 0, i32 2
  %t4 = load i32, ptr %t1
  store i32 %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define i32 @Box__g1__t0_Get__g0__owner_int(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Box__g1__t0, ptr %t1, i32 0, i32 2
  %t3 = load i32, ptr %t2
  ret i32 %t3
exception_unwind:
  ret i32 0
}

define i1 @glitch_endpoint_handlers_contains(ptr %app, ptr %method, ptr %path) {
entry:
  ret i1 false
}

define ptr @glitch_endpoint_handlers_invoke(ptr %app, ptr %method, ptr %path, ptr %body) {
entry:
  ret ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.0, i32 0, i32 2, i64 0)
}

