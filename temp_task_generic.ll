; ModuleID = 'glitching'
%glitch.array = type { i64, ptr }
%glitch.list = type { i64, i64, ptr }
%glitch.dict = type { i64, i64, ptr, ptr }
%glitch.string_node = type { i64, i64, [0 x i8] }
%glitch.delegate = type { i64, ptr, ptr, ptr }
%glitch.Task__g0__t0 = type { i64, ptr, i1, i1, ptr }
%glitch.ValueTask__g0__t1 = type { i64, ptr, i1, i1, ptr }
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
declare void @GlitchTask_RunVoid(ptr, ptr)
declare void @GlitchTask_RunI32(ptr, ptr)
declare void @GlitchTask_RunBool(ptr, ptr)
declare void @GlitchTask_RunI64(ptr, ptr)
declare void @GlitchTask_RunDouble(ptr, ptr)
declare void @GlitchTask_RunPtr(ptr, ptr)
declare void @GlitchTask_Wait(ptr)
declare i1 @GlitchTask_IsCompleted(ptr)
declare void @GlitchTask_Destroy(ptr)
declare i32 @GlitchRestHost_read_env_int(ptr, i32)
declare ptr @GlitchString_Lock()
declare void @GlitchString_Unlock(ptr)
declare i64 @GlitchLiveAllocations_Add(i64)
declare i64 @GlitchLiveAllocations_Load()
@glitch_exception_pending = internal global ptr null
define dllexport ptr @glitch_take_pending_exception() {
entry:
  %value = load ptr, ptr @glitch_exception_pending
  store ptr null, ptr @glitch_exception_pending
  ret ptr %value
}
define dllexport void @glitch_object_drop(ptr %value) {
entry:
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %drop
drop:
  %drop_ptr_ptr = getelementptr inbounds { i64, ptr }, ptr %value, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_drop, label %call_drop, label %done
call_drop:
  call void %drop_ptr(ptr %value)
  br label %done
done:
  ret void
}
%glitch.task = type { i32, ptr, ptr }
define ptr @glitch_task_from_result_ptr(ptr %result) {
entry:
%task_size_ptr = getelementptr %glitch.task, ptr null, i32 1
%task_size = ptrtoint ptr %task_size_ptr to i64
%task = call ptr @glitch_calloc(i64 1, i64 %task_size)
%completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0
store i32 1, ptr %completed_ptr
%result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1
store ptr %result, ptr %result_ptr
%state_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 2
store ptr null, ptr %state_ptr
ret ptr %task
}
define ptr @glitch_task_completed() {
entry:
%task = call ptr @glitch_task_from_result_ptr(ptr null)
ret ptr %task
}
define ptr @glitch_task_run_void(ptr %delegate) {
entry:
%task_size_ptr = getelementptr %glitch.task, ptr null, i32 1
%task_size = ptrtoint ptr %task_size_ptr to i64
%task = call ptr @glitch_calloc(i64 1, i64 %task_size)
%completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0
store i32 0, ptr %completed_ptr
%result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1
store ptr null, ptr %result_ptr
%state_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 2
store ptr null, ptr %state_ptr
call void @GlitchTask_RunVoid(ptr %task, ptr %delegate)
ret ptr %task
}
define ptr @glitch_task_run_i32(ptr %delegate) {
entry:
%task_size_ptr = getelementptr %glitch.task, ptr null, i32 1
%task_size = ptrtoint ptr %task_size_ptr to i64
%task = call ptr @glitch_calloc(i64 1, i64 %task_size)
%completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0
store i32 0, ptr %completed_ptr
%result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1
store ptr null, ptr %result_ptr
%state_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 2
store ptr null, ptr %state_ptr
call void @GlitchTask_RunI32(ptr %task, ptr %delegate)
ret ptr %task
}
define ptr @glitch_task_run_bool(ptr %delegate) {
entry:
%task_size_ptr = getelementptr %glitch.task, ptr null, i32 1
%task_size = ptrtoint ptr %task_size_ptr to i64
%task = call ptr @glitch_calloc(i64 1, i64 %task_size)
%completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0
store i32 0, ptr %completed_ptr
%result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1
store ptr null, ptr %result_ptr
%state_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 2
store ptr null, ptr %state_ptr
call void @GlitchTask_RunBool(ptr %task, ptr %delegate)
ret ptr %task
}
define ptr @glitch_task_run_i64(ptr %delegate) {
entry:
%task_size_ptr = getelementptr %glitch.task, ptr null, i32 1
%task_size = ptrtoint ptr %task_size_ptr to i64
%task = call ptr @glitch_calloc(i64 1, i64 %task_size)
%completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0
store i32 0, ptr %completed_ptr
%result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1
store ptr null, ptr %result_ptr
%state_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 2
store ptr null, ptr %state_ptr
call void @GlitchTask_RunI64(ptr %task, ptr %delegate)
ret ptr %task
}
define ptr @glitch_task_run_double(ptr %delegate) {
entry:
%task_size_ptr = getelementptr %glitch.task, ptr null, i32 1
%task_size = ptrtoint ptr %task_size_ptr to i64
%task = call ptr @glitch_calloc(i64 1, i64 %task_size)
%completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0
store i32 0, ptr %completed_ptr
%result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1
store ptr null, ptr %result_ptr
%state_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 2
store ptr null, ptr %state_ptr
call void @GlitchTask_RunDouble(ptr %task, ptr %delegate)
ret ptr %task
}
define ptr @glitch_task_run_ptr(ptr %delegate) {
entry:
%task_size_ptr = getelementptr %glitch.task, ptr null, i32 1
%task_size = ptrtoint ptr %task_size_ptr to i64
%task = call ptr @glitch_calloc(i64 1, i64 %task_size)
%completed_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 0
store i32 0, ptr %completed_ptr
%result_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 1
store ptr null, ptr %result_ptr
%state_ptr = getelementptr inbounds %glitch.task, ptr %task, i32 0, i32 2
store ptr null, ptr %state_ptr
call void @GlitchTask_RunPtr(ptr %task, ptr %delegate)
ret ptr %task
}
define ptr @glitch_task_from_result_i32(i32 %result) {
entry:
%val_ptr = inttoptr i32 %result to ptr
%task = call ptr @glitch_task_from_result_ptr(ptr %val_ptr)
ret ptr %task
}
define ptr @glitch_task_from_result_bool(i1 %result) {
entry:
%val_i64 = zext i1 %result to i64
%val_ptr = inttoptr i64 %val_i64 to ptr
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
call void @GlitchTask_Wait(ptr %task)
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
define i1 @glitch_task_get_result_bool(ptr %task) {
entry:
%ptr = call ptr @glitch_task_get_result_ptr(ptr %task)
%val = ptrtoint ptr %ptr to i64
%res = trunc i64 %val to i1
ret i1 %res
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
define void @glitch_task_wait(ptr %task) {
entry:
call void @GlitchTask_Wait(ptr %task)
ret void
}
define i1 @glitch_task_is_completed(ptr %task) {
entry:
%completed = call i1 @GlitchTask_IsCompleted(ptr %task)
ret i1 %completed
}
define ptr @JsonSerializer_Serialize_Native(ptr %value) {
entry:
%is_null = icmp eq ptr %value, null
br i1 %is_null, label %json_serialize_null, label %json_serialize_value
json_serialize_null:
ret ptr null
json_serialize_value:
%length = call i64 @glitch_string_length(ptr %value)
%total = add i64 %length, 2
%quoted = call ptr @glitch_string_allocate(i64 %total)
store i8 34, ptr %quoted
%content = getelementptr inbounds i8, ptr %quoted, i64 1
call ptr @memcpy(ptr %content, ptr %value, i64 %length)
%closing_index = add i64 %length, 1
%closing = getelementptr inbounds i8, ptr %quoted, i64 %closing_index
store i8 34, ptr %closing
ret ptr %quoted
}
define ptr @JsonSerializer_Deserialize_Native(ptr %json) {
entry:
%is_null = icmp eq ptr %json, null
br i1 %is_null, label %json_deserialize_null, label %json_deserialize_value
json_deserialize_null:
ret ptr null
json_deserialize_value:
%length = call i64 @glitch_string_length(ptr %json)
%too_short = icmp ult i64 %length, 2
br i1 %too_short, label %json_copy_full, label %json_check_quotes
json_check_quotes:
%first = load i8, ptr %json
%last_index = sub i64 %length, 1
%last_ptr = getelementptr inbounds i8, ptr %json, i64 %last_index
%last = load i8, ptr %last_ptr
%first_quote = icmp eq i8 %first, 34
%last_quote = icmp eq i8 %last, 34
%quoted = and i1 %first_quote, %last_quote
br i1 %quoted, label %json_copy_inner, label %json_copy_full
json_copy_inner:
%inner_len = sub i64 %length, 2
%inner = call ptr @glitch_string_allocate(i64 %inner_len)
%source = getelementptr inbounds i8, ptr %json, i64 1
call ptr @memcpy(ptr %inner, ptr %source, i64 %inner_len)
ret ptr %inner
json_copy_full:
%copy = call ptr @glitch_string_allocate(i64 %length)
call ptr @memcpy(ptr %copy, ptr %json, i64 %length)
ret ptr %copy
}
define ptr @glitch_task_when_all2(ptr %left, ptr %right) {
entry:
call void @glitch_task_wait(ptr %left)
call void @glitch_task_wait(ptr %right)
%task = call ptr @glitch_task_completed()
ret ptr %task
}
define ptr @glitch_task_when_all_array(ptr %tasks) {
entry:
%task = call ptr @glitch_task_completed()
ret ptr %task
}
define ptr @glitch_calloc(i64 %count, i64 %size) {
entry:
  %value = call ptr @calloc(i64 %count, i64 %size)
  %is_null = icmp eq ptr %value, null
  br i1 %is_null, label %done, label %count_alloc
count_alloc:
  %live = call i64 @GlitchLiveAllocations_Add(i64 1)
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
  %live = call i64 @GlitchLiveAllocations_Add(i64 1)
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
  %live = call i64 @GlitchLiveAllocations_Add(i64 -1)
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
define i1 @glitch_string_equals(ptr %left, ptr %right) {
entry:
  %same = icmp eq ptr %left, %right
  br i1 %same, label %true_case, label %check_null
check_null:
  %left_null = icmp eq ptr %left, null
  %right_null = icmp eq ptr %right, null
  %any_null = or i1 %left_null, %right_null
  br i1 %any_null, label %false_case, label %compare
compare:
  %cmp = call i32 @strcmp(ptr %left, ptr %right)
  %eq = icmp eq i32 %cmp, 0
  ret i1 %eq
true_case:
  ret i1 true
false_case:
  ret i1 false
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
define void @glitch_destroy_boxed_scalar(ptr %object) {
entry:
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
define i1 @glitch_object_equals(ptr %left, ptr %right) {
entry:
  %same = icmp eq ptr %left, %right
  br i1 %same, label %true_case, label %check_null
check_null:
  %left_null = icmp eq ptr %left, null
  %right_null = icmp eq ptr %right, null
  %any_null = or i1 %left_null, %right_null
  br i1 %any_null, label %false_case, label %load_drop
load_drop:
  %left_drop_ptr_ptr = getelementptr inbounds { i64, ptr }, ptr %left, i32 0, i32 1
  %right_drop_ptr_ptr = getelementptr inbounds { i64, ptr }, ptr %right, i32 0, i32 1
  %left_drop_ptr = load ptr, ptr %left_drop_ptr_ptr
  %right_drop_ptr = load ptr, ptr %right_drop_ptr_ptr
  %same_drop = icmp eq ptr %left_drop_ptr, %right_drop_ptr
  br i1 %same_drop, label %check_boxed_scalar, label %false_case
check_boxed_scalar:
  %is_boxed_scalar = icmp eq ptr %left_drop_ptr, @glitch_destroy_boxed_scalar
  br i1 %is_boxed_scalar, label %load_tag, label %false_case
load_tag:
  %left_tag_ptr = getelementptr inbounds { i64, ptr, i32 }, ptr %left, i32 0, i32 2
  %right_tag_ptr = getelementptr inbounds { i64, ptr, i32 }, ptr %right, i32 0, i32 2
  %left_tag = load i32, ptr %left_tag_ptr
  %right_tag = load i32, ptr %right_tag_ptr
  %same_tag = icmp eq i32 %left_tag, %right_tag
  br i1 %same_tag, label %dispatch, label %false_case
dispatch:
  switch i32 %left_tag, label %false_case [
    i32 1, label %compare_bool
    i32 2, label %compare_byte
    i32 3, label %compare_short
    i32 4, label %compare_int
    i32 5, label %compare_uint
    i32 6, label %compare_long
    i32 7, label %compare_double
    i32 8, label %compare_decimal
  ]
compare_bool:
  %left_bool_ptr = getelementptr inbounds { i64, ptr, i32, i1 }, ptr %left, i32 0, i32 3
  %right_bool_ptr = getelementptr inbounds { i64, ptr, i32, i1 }, ptr %right, i32 0, i32 3
  %left_bool = load i1, ptr %left_bool_ptr
  %right_bool = load i1, ptr %right_bool_ptr
  %bool_eq = icmp eq i1 %left_bool, %right_bool
  ret i1 %bool_eq
compare_byte:
  %left_byte_ptr = getelementptr inbounds { i64, ptr, i32, i8 }, ptr %left, i32 0, i32 3
  %right_byte_ptr = getelementptr inbounds { i64, ptr, i32, i8 }, ptr %right, i32 0, i32 3
  %left_byte = load i8, ptr %left_byte_ptr
  %right_byte = load i8, ptr %right_byte_ptr
  %byte_eq = icmp eq i8 %left_byte, %right_byte
  ret i1 %byte_eq
compare_short:
  %left_short_ptr = getelementptr inbounds { i64, ptr, i32, i16 }, ptr %left, i32 0, i32 3
  %right_short_ptr = getelementptr inbounds { i64, ptr, i32, i16 }, ptr %right, i32 0, i32 3
  %left_short = load i16, ptr %left_short_ptr
  %right_short = load i16, ptr %right_short_ptr
  %short_eq = icmp eq i16 %left_short, %right_short
  ret i1 %short_eq
compare_int:
  %left_int_ptr = getelementptr inbounds { i64, ptr, i32, i32 }, ptr %left, i32 0, i32 3
  %right_int_ptr = getelementptr inbounds { i64, ptr, i32, i32 }, ptr %right, i32 0, i32 3
  %left_int = load i32, ptr %left_int_ptr
  %right_int = load i32, ptr %right_int_ptr
  %int_eq = icmp eq i32 %left_int, %right_int
  ret i1 %int_eq
compare_uint:
  %left_uint_ptr = getelementptr inbounds { i64, ptr, i32, i32 }, ptr %left, i32 0, i32 3
  %right_uint_ptr = getelementptr inbounds { i64, ptr, i32, i32 }, ptr %right, i32 0, i32 3
  %left_uint = load i32, ptr %left_uint_ptr
  %right_uint = load i32, ptr %right_uint_ptr
  %uint_eq = icmp eq i32 %left_uint, %right_uint
  ret i1 %uint_eq
compare_long:
  %left_long_ptr = getelementptr inbounds { i64, ptr, i32, i64 }, ptr %left, i32 0, i32 3
  %right_long_ptr = getelementptr inbounds { i64, ptr, i32, i64 }, ptr %right, i32 0, i32 3
  %left_long = load i64, ptr %left_long_ptr
  %right_long = load i64, ptr %right_long_ptr
  %long_eq = icmp eq i64 %left_long, %right_long
  ret i1 %long_eq
compare_double:
  %left_double_ptr = getelementptr inbounds { i64, ptr, i32, double }, ptr %left, i32 0, i32 3
  %right_double_ptr = getelementptr inbounds { i64, ptr, i32, double }, ptr %right, i32 0, i32 3
  %left_double = load double, ptr %left_double_ptr
  %right_double = load double, ptr %right_double_ptr
  %double_eq = fcmp oeq double %left_double, %right_double
  ret i1 %double_eq
compare_decimal:
  %left_decimal_ptr = getelementptr inbounds { i64, ptr, i32, double }, ptr %left, i32 0, i32 3
  %right_decimal_ptr = getelementptr inbounds { i64, ptr, i32, double }, ptr %right, i32 0, i32 3
  %left_decimal = load double, ptr %left_decimal_ptr
  %right_decimal = load double, ptr %right_decimal_ptr
  %decimal_eq = fcmp oeq double %left_decimal, %right_decimal
  ret i1 %decimal_eq
true_case:
  ret i1 true
false_case:
  ret i1 false
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
@.str.0 = private unnamed_addr global { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"Ada\00" }
define i32 @glitch_delegate_wrapper_Compute_0(ptr %env) {
entry:
  %t_wrap_9 = call i32 @Compute()
  ret i32 %t_wrap_9
}

define ptr @glitch_delegate_wrapper_LoadName_1(ptr %env) {
entry:
  %t_wrap_25 = call ptr @LoadName()
  ret ptr %t_wrap_25
}

define i1 @glitch_delegate_wrapper_IsReady_2(ptr %env) {
entry:
  %t_wrap_42 = call i1 @IsReady()
  ret i1 %t_wrap_42
}

define double @glitch_delegate_wrapper_LoadRatio_3(ptr %env) {
entry:
  %t_wrap_59 = call double @LoadRatio()
  ret double %t_wrap_59
}

@.str.1 = private unnamed_addr global { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"404\00" }

define void @glitch_destroy_Task__g0__t0(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Task__g0__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Task__g0__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Task__g0__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Task__g0__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Task__g0__t0, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Task__g0__t0(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ValueTask__g0__t1(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValueTask__g0__t1(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValueTask__g0__t1, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValueTask__g0__t1(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValueTask__g0__t1, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValueTask__g0__t1, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValueTask__g0__t1(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define ptr @Task__g0__t0_CompletedTask__g0() {
entry:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t0_Run__g0__fn(ptr %function) {
entry:
  %t0 = alloca ptr
  store ptr %function, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  %t3 = load ptr, ptr %t2
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = call ptr %t3(ptr %t5)
  %t7 = call ptr @Task__g0__t0_CompletedTask__g0()
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t7
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t0_Run__g1__fn_ret_T(ptr %function) {
entry:
  %t0 = alloca ptr
  store ptr %function, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  %t3 = load ptr, ptr %t2
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = call ptr %t3(ptr %t5)
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Task__g0__t0_Wait__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @Task__g0__t0_GetAwaiter__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  store ptr null, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define void @Task__g0__t0_GetResult__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @Task__g0__t0_WhenAll__g0__array_task_void(ptr %tasks) {
entry:
  %t0 = alloca ptr
  store ptr %tasks, ptr %t0
  %t1 = call ptr @Task__g0__t0_CompletedTask__g0()
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t0_WhenAll__g1__array_task_T(ptr %tasks) {
entry:
  %t0 = alloca ptr
  store ptr %tasks, ptr %t0
  %t1 = call ptr @Task__g0__t0_CompletedTask__g0()
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t0_FromResult__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t0_WhenAll__g0__task_T_task_T(ptr %first, ptr %second) {
entry:
  %t0 = alloca ptr
  store ptr %first, ptr %t0
  %t1 = alloca ptr
  store ptr %second, ptr %t1
  %t2 = call ptr @Task__g0__t0_CompletedTask__g0()
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define void @ValueTask__g0__t1_Wait__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @ValueTask__g0__t1_GetAwaiter__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  store ptr null, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @ValueTask__g0__t1_FromResult__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ValueTask__g0__t1_GetResult__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ValueTask__g0__t1, ptr %t1, i32 0, i32 4
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @ValueTask__g0__t1_AsTask__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ValueTask__g0__t1, ptr %t1, i32 0, i32 4
  %t3 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define i32 @Compute() {
entry:
  %t0 = trunc i64 42 to i32
  ret i32 %t0
exception_unwind:
  ret i32 0
}

define ptr @LoadName() {
entry:
  ret ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.0, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define i1 @IsReady() {
entry:
  ret i1 1
exception_unwind:
  ret i1 0
}

define double @LoadRatio() {
entry:
  ret double 1.5
exception_unwind:
  ret double 0.0
}

define i32 @main() {
entry:
  %t0 = alloca ptr
  store ptr null, ptr %t0
  %t1 = alloca i32
  store i32 0, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca i1
  store i1 0, ptr %t5
  %t6 = alloca ptr
  store ptr null, ptr %t6
  %t7 = alloca double
  store double 0.0, ptr %t7
  %t8 = alloca ptr
  store ptr null, ptr %t8
  %t11 = getelementptr %glitch.delegate, ptr null, i32 1
  %t12 = ptrtoint ptr %t11 to i64
  %t10 = call ptr @glitch_calloc(i64 1, i64 %t12)
  %t13 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 0
  store i64 1, ptr %t13
  %t14 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 1
  store ptr @glitch_delegate_wrapper_Compute_0, ptr %t14
  %t15 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 2
  store ptr null, ptr %t15
  %t16 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 3
  store ptr null, ptr %t16
  call void @glitch_delegate_retain(ptr %t10)
  %t17 = call ptr @glitch_task_run_ptr(ptr %t10)
  call void @glitch_delegate_release(ptr %t10)
  %t18 = load ptr, ptr %t0
  %t19 = icmp eq ptr %t18, null
  br i1 %t19, label %task_release_done_1, label %task_release_0
task_release_0:
  call void @glitch_task_wait(ptr %t18)
  %t20 = getelementptr inbounds %glitch.task, ptr %t18, i32 0, i32 1
  %t21 = load ptr, ptr %t20
  call void @GlitchTask_Destroy(ptr %t18)
  call void @glitch_free(ptr %t18)
  br label %task_release_done_1
task_release_done_1:
  store ptr %t17, ptr %t0
  %t22 = load ptr, ptr %t0
  %t23 = call i32 @glitch_task_get_result_i32(ptr %t22)
  store i32 %t23, ptr %t1
  %t24 = load i32, ptr %t1
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_i32, i64 0, i64 0), i32 %t24)
  %t27 = getelementptr %glitch.delegate, ptr null, i32 1
  %t28 = ptrtoint ptr %t27 to i64
  %t26 = call ptr @glitch_calloc(i64 1, i64 %t28)
  %t29 = getelementptr inbounds %glitch.delegate, ptr %t26, i32 0, i32 0
  store i64 1, ptr %t29
  %t30 = getelementptr inbounds %glitch.delegate, ptr %t26, i32 0, i32 1
  store ptr @glitch_delegate_wrapper_LoadName_1, ptr %t30
  %t31 = getelementptr inbounds %glitch.delegate, ptr %t26, i32 0, i32 2
  store ptr null, ptr %t31
  %t32 = getelementptr inbounds %glitch.delegate, ptr %t26, i32 0, i32 3
  store ptr null, ptr %t32
  call void @glitch_delegate_retain(ptr %t26)
  %t33 = call ptr @glitch_task_run_ptr(ptr %t26)
  call void @glitch_delegate_release(ptr %t26)
  %t34 = load ptr, ptr %t2
  %t35 = icmp eq ptr %t34, null
  br i1 %t35, label %task_release_done_3, label %task_release_2
task_release_2:
  call void @glitch_task_wait(ptr %t34)
  %t36 = getelementptr inbounds %glitch.task, ptr %t34, i32 0, i32 1
  %t37 = load ptr, ptr %t36
  call void @glitch_string_release(ptr %t37)
  call void @GlitchTask_Destroy(ptr %t34)
  call void @glitch_free(ptr %t34)
  br label %task_release_done_3
task_release_done_3:
  store ptr %t33, ptr %t2
  %t38 = load ptr, ptr %t2
  %t39 = call ptr @glitch_task_get_result_ptr(ptr %t38)
  call void @glitch_string_retain(ptr %t39)
  %t40 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t40)
  store ptr %t39, ptr %t3
  %t41 = load ptr, ptr %t3
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t41)
  %t44 = getelementptr %glitch.delegate, ptr null, i32 1
  %t45 = ptrtoint ptr %t44 to i64
  %t43 = call ptr @glitch_calloc(i64 1, i64 %t45)
  %t46 = getelementptr inbounds %glitch.delegate, ptr %t43, i32 0, i32 0
  store i64 1, ptr %t46
  %t47 = getelementptr inbounds %glitch.delegate, ptr %t43, i32 0, i32 1
  store ptr @glitch_delegate_wrapper_IsReady_2, ptr %t47
  %t48 = getelementptr inbounds %glitch.delegate, ptr %t43, i32 0, i32 2
  store ptr null, ptr %t48
  %t49 = getelementptr inbounds %glitch.delegate, ptr %t43, i32 0, i32 3
  store ptr null, ptr %t49
  call void @glitch_delegate_retain(ptr %t43)
  %t50 = call ptr @glitch_task_run_ptr(ptr %t43)
  call void @glitch_delegate_release(ptr %t43)
  %t51 = load ptr, ptr %t4
  %t52 = icmp eq ptr %t51, null
  br i1 %t52, label %task_release_done_5, label %task_release_4
task_release_4:
  call void @glitch_task_wait(ptr %t51)
  %t53 = getelementptr inbounds %glitch.task, ptr %t51, i32 0, i32 1
  %t54 = load ptr, ptr %t53
  call void @GlitchTask_Destroy(ptr %t51)
  call void @glitch_free(ptr %t51)
  br label %task_release_done_5
task_release_done_5:
  store ptr %t50, ptr %t4
  %t55 = load ptr, ptr %t4
  %t56 = call i1 @glitch_task_get_result_bool(ptr %t55)
  store i1 %t56, ptr %t5
  %t57 = load i1, ptr %t5
  %t58 = select i1 %t57, ptr getelementptr inbounds ([5 x i8], ptr @.json_true, i64 0, i64 0), ptr getelementptr inbounds ([6 x i8], ptr @.json_false, i64 0, i64 0)
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t58)
  %t61 = getelementptr %glitch.delegate, ptr null, i32 1
  %t62 = ptrtoint ptr %t61 to i64
  %t60 = call ptr @glitch_calloc(i64 1, i64 %t62)
  %t63 = getelementptr inbounds %glitch.delegate, ptr %t60, i32 0, i32 0
  store i64 1, ptr %t63
  %t64 = getelementptr inbounds %glitch.delegate, ptr %t60, i32 0, i32 1
  store ptr @glitch_delegate_wrapper_LoadRatio_3, ptr %t64
  %t65 = getelementptr inbounds %glitch.delegate, ptr %t60, i32 0, i32 2
  store ptr null, ptr %t65
  %t66 = getelementptr inbounds %glitch.delegate, ptr %t60, i32 0, i32 3
  store ptr null, ptr %t66
  call void @glitch_delegate_retain(ptr %t60)
  %t67 = call ptr @glitch_task_run_ptr(ptr %t60)
  call void @glitch_delegate_release(ptr %t60)
  %t68 = load ptr, ptr %t6
  %t69 = icmp eq ptr %t68, null
  br i1 %t69, label %task_release_done_7, label %task_release_6
task_release_6:
  call void @glitch_task_wait(ptr %t68)
  %t70 = getelementptr inbounds %glitch.task, ptr %t68, i32 0, i32 1
  %t71 = load ptr, ptr %t70
  call void @GlitchTask_Destroy(ptr %t68)
  call void @glitch_free(ptr %t68)
  br label %task_release_done_7
task_release_done_7:
  store ptr %t67, ptr %t6
  %t72 = load ptr, ptr %t6
  %t73 = call double @glitch_task_get_result_double(ptr %t72)
  store double %t73, ptr %t7
  %t74 = load double, ptr %t7
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_double, i64 0, i64 0), double %t74)
  %t75 = call ptr @glitch_task_from_result_ptr(ptr 1)
  %t76 = load ptr, ptr %t8
  %t77 = icmp eq ptr %t76, null
  br i1 %t77, label %task_release_done_9, label %task_release_8
task_release_8:
  call void @glitch_task_wait(ptr %t76)
  %t78 = getelementptr inbounds %glitch.task, ptr %t76, i32 0, i32 1
  %t79 = load ptr, ptr %t78
  call void @GlitchTask_Destroy(ptr %t76)
  call void @glitch_free(ptr %t76)
  br label %task_release_done_9
task_release_done_9:
  store ptr %t75, ptr %t8
  %t80 = load ptr, ptr %t8
  %t81 = call i1 @glitch_task_is_completed(ptr %t80)
  %t82 = select i1 %t81, ptr getelementptr inbounds ([5 x i8], ptr @.json_true, i64 0, i64 0), ptr getelementptr inbounds ([6 x i8], ptr @.json_false, i64 0, i64 0)
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t82)
  %t83 = load ptr, ptr %t8
  %t84 = call i1 @glitch_task_get_result_bool(ptr %t83)
  %t85 = select i1 %t84, ptr getelementptr inbounds ([5 x i8], ptr @.json_true, i64 0, i64 0), ptr getelementptr inbounds ([6 x i8], ptr @.json_false, i64 0, i64 0)
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t85)
  %t86 = load ptr, ptr %t8
  %t87 = icmp eq ptr %t86, null
  br i1 %t87, label %task_release_done_11, label %task_release_10
task_release_10:
  call void @glitch_task_wait(ptr %t86)
  %t88 = getelementptr inbounds %glitch.task, ptr %t86, i32 0, i32 1
  %t89 = load ptr, ptr %t88
  call void @GlitchTask_Destroy(ptr %t86)
  call void @glitch_free(ptr %t86)
  br label %task_release_done_11
task_release_done_11:
  %t90 = load ptr, ptr %t6
  %t91 = icmp eq ptr %t90, null
  br i1 %t91, label %task_release_done_13, label %task_release_12
task_release_12:
  call void @glitch_task_wait(ptr %t90)
  %t92 = getelementptr inbounds %glitch.task, ptr %t90, i32 0, i32 1
  %t93 = load ptr, ptr %t92
  call void @GlitchTask_Destroy(ptr %t90)
  call void @glitch_free(ptr %t90)
  br label %task_release_done_13
task_release_done_13:
  %t94 = load ptr, ptr %t4
  %t95 = icmp eq ptr %t94, null
  br i1 %t95, label %task_release_done_15, label %task_release_14
task_release_14:
  call void @glitch_task_wait(ptr %t94)
  %t96 = getelementptr inbounds %glitch.task, ptr %t94, i32 0, i32 1
  %t97 = load ptr, ptr %t96
  call void @GlitchTask_Destroy(ptr %t94)
  call void @glitch_free(ptr %t94)
  br label %task_release_done_15
task_release_done_15:
  %t98 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t98)
  %t99 = load ptr, ptr %t2
  %t100 = icmp eq ptr %t99, null
  br i1 %t100, label %task_release_done_17, label %task_release_16
task_release_16:
  call void @glitch_task_wait(ptr %t99)
  %t101 = getelementptr inbounds %glitch.task, ptr %t99, i32 0, i32 1
  %t102 = load ptr, ptr %t101
  call void @glitch_string_release(ptr %t102)
  call void @GlitchTask_Destroy(ptr %t99)
  call void @glitch_free(ptr %t99)
  br label %task_release_done_17
task_release_done_17:
  %t103 = load ptr, ptr %t0
  %t104 = icmp eq ptr %t103, null
  br i1 %t104, label %task_release_done_19, label %task_release_18
task_release_18:
  call void @glitch_task_wait(ptr %t103)
  %t105 = getelementptr inbounds %glitch.task, ptr %t103, i32 0, i32 1
  %t106 = load ptr, ptr %t105
  call void @GlitchTask_Destroy(ptr %t103)
  call void @glitch_free(ptr %t103)
  br label %task_release_done_19
task_release_done_19:
  %t107 = call i64 @GlitchLiveAllocations_Load()
  %t108 = icmp ne i64 %t107, 0
  %t109 = load ptr, ptr @glitch_exception_pending
  %t110 = icmp ne ptr %t109, null
  %t111 = or i1 %t108, %t110
  %t112 = zext i1 %t111 to i32
  %t113 = call ptr @getenv(ptr @.env_report_leaks)
  %t114 = icmp ne ptr %t113, null
  br i1 %t114, label %report_leaks_20, label %main_return_21
report_leaks_20:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t107)
  br label %main_return_21
main_return_21:
  ret i32 %t112
exception_unwind:
  %t115 = load ptr, ptr %t8
  %t116 = icmp eq ptr %t115, null
  br i1 %t116, label %task_release_done_23, label %task_release_22
task_release_22:
  call void @glitch_task_wait(ptr %t115)
  %t117 = getelementptr inbounds %glitch.task, ptr %t115, i32 0, i32 1
  %t118 = load ptr, ptr %t117
  call void @GlitchTask_Destroy(ptr %t115)
  call void @glitch_free(ptr %t115)
  br label %task_release_done_23
task_release_done_23:
  %t119 = load ptr, ptr %t6
  %t120 = icmp eq ptr %t119, null
  br i1 %t120, label %task_release_done_25, label %task_release_24
task_release_24:
  call void @glitch_task_wait(ptr %t119)
  %t121 = getelementptr inbounds %glitch.task, ptr %t119, i32 0, i32 1
  %t122 = load ptr, ptr %t121
  call void @GlitchTask_Destroy(ptr %t119)
  call void @glitch_free(ptr %t119)
  br label %task_release_done_25
task_release_done_25:
  %t123 = load ptr, ptr %t4
  %t124 = icmp eq ptr %t123, null
  br i1 %t124, label %task_release_done_27, label %task_release_26
task_release_26:
  call void @glitch_task_wait(ptr %t123)
  %t125 = getelementptr inbounds %glitch.task, ptr %t123, i32 0, i32 1
  %t126 = load ptr, ptr %t125
  call void @GlitchTask_Destroy(ptr %t123)
  call void @glitch_free(ptr %t123)
  br label %task_release_done_27
task_release_done_27:
  %t127 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t127)
  %t128 = load ptr, ptr %t2
  %t129 = icmp eq ptr %t128, null
  br i1 %t129, label %task_release_done_29, label %task_release_28
task_release_28:
  call void @glitch_task_wait(ptr %t128)
  %t130 = getelementptr inbounds %glitch.task, ptr %t128, i32 0, i32 1
  %t131 = load ptr, ptr %t130
  call void @glitch_string_release(ptr %t131)
  call void @GlitchTask_Destroy(ptr %t128)
  call void @glitch_free(ptr %t128)
  br label %task_release_done_29
task_release_done_29:
  %t132 = load ptr, ptr %t0
  %t133 = icmp eq ptr %t132, null
  br i1 %t133, label %task_release_done_31, label %task_release_30
task_release_30:
  call void @glitch_task_wait(ptr %t132)
  %t134 = getelementptr inbounds %glitch.task, ptr %t132, i32 0, i32 1
  %t135 = load ptr, ptr %t134
  call void @GlitchTask_Destroy(ptr %t132)
  call void @glitch_free(ptr %t132)
  br label %task_release_done_31
task_release_done_31:
  %t136 = call i64 @GlitchLiveAllocations_Load()
  %t137 = icmp ne i64 %t136, 0
  %t138 = load ptr, ptr @glitch_exception_pending
  %t139 = icmp ne ptr %t138, null
  %t140 = or i1 %t137, %t139
  %t141 = zext i1 %t140 to i32
  %t142 = call ptr @getenv(ptr @.env_report_leaks)
  %t143 = icmp ne ptr %t142, null
  br i1 %t143, label %report_leaks_32, label %main_return_33
report_leaks_32:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t136)
  br label %main_return_33
main_return_33:
  ret i32 %t141
}

define i1 @glitch_endpoint_handlers_contains(ptr %app, ptr %method, ptr %path) {
entry:
  ret i1 false
}

define ptr @glitch_endpoint_handlers_invoke(ptr %app, ptr %method, ptr %path, ptr %body) {
entry:
  ret ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.1, i32 0, i32 2, i64 0)
}

