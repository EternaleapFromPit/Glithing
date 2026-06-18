; ModuleID = 'glitching'
%glitch.array = type { i64, ptr }
%glitch.list = type { i64, i64, ptr }
%glitch.dict = type { i64, i64, ptr, ptr }
%glitch.string_node = type { i64, i64, [0 x i8] }
%glitch.delegate = type { i64, ptr, ptr, ptr }
%glitch.AssemblyName__g0__t0 = type { i64, ptr, ptr }
%glitch.Assembly__g0__t1 = type { i64, ptr, ptr }
%glitch.MemberInfo__g0__t2 = type { i64, ptr, ptr, ptr }
%glitch.MethodBase__g0__t3 = type { i64, ptr, ptr, ptr, ptr, i1 }
%glitch.MethodInfo__g0__t4 = type { i64, ptr, ptr, ptr, ptr, i1, ptr, ptr, i1, ptr }
%glitch.ConstructorInfo__g0__t5 = type { i64, ptr, ptr, ptr, ptr, i1, ptr }
%glitch.ParameterInfo__g0__t6 = type { i64, ptr, ptr, ptr, ptr, i32, i1, i1, i1 }
%glitch.FieldInfo__g0__t7 = type { i64, ptr, ptr, ptr, ptr, i1 }
%glitch.PropertyInfo__g0__t8 = type { i64, ptr, ptr, ptr, ptr, i1, i1, ptr, ptr }
%glitch.EventInfo__g0__t9 = type { i64, ptr, ptr, ptr, ptr }
%glitch.AppContext__g0__t10 = type { i64, ptr }
%glitch.Uri__g0__t11 = type { i64, ptr }
%glitch.GC__g0__t12 = type { i64, ptr }
%glitch.Buffer__g0__t13 = type { i64, ptr }
%glitch.object__g0__t14 = type { i64, ptr }
%glitch.Object__g0__t15 = type { i64, ptr }
%glitch.Exception__g0__t16 = type { i64, ptr, ptr }
%glitch.ArgumentException__g0__t17 = type { i64, ptr, ptr }
%glitch.InvalidOperationException__g0__t18 = type { i64, ptr, ptr }
%glitch.FormatException__g0__t19 = type { i64, ptr, ptr }
%glitch.DateTime__g0__t20 = type { i64, ptr, double }
%glitch.DateTimeOffset__g0__t21 = type { i64, ptr }
%glitch.TimeSpan__g0__t22 = type { i64, ptr, double }
%glitch.Enum__g0__t23 = type { i64, ptr }
%glitch.Type__g0__t24 = type { i64, ptr, ptr, ptr, i1, i1, i1, i1, i1, i1, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Array__g0__t25 = type { i64, ptr }
%glitch.string__g0__t26 = type { i64, ptr }
%glitch.String__g0__t27 = type { i64, ptr }
%glitch.bool__g0__t28 = type { i64, ptr }
%glitch.int__g0__t29 = type { i64, ptr }
%glitch.Rc__g1__t30 = type { i64, ptr, ptr, i32 }
%glitch.Weak__g1__t31 = type { i64, ptr, ptr }
%glitch.IEnumerable__g1__t32 = type { i64, ptr }
%glitch.IEnumerator__g1__t33 = type { i64, ptr, ptr }
%glitch.ICollection__g1__t34 = type { i64, ptr, i32 }
%glitch.IReadOnlyCollection__g1__t35 = type { i64, ptr, i32 }
%glitch.IList__g1__t36 = type { i64, ptr, i32, i32 }
%glitch.IReadOnlyList__g1__t37 = type { i64, ptr, i32 }
%glitch.IDictionary__g2__t38 = type { i64, ptr, i32 }
%glitch.IReadOnlyDictionary__g2__t39 = type { i64, ptr, i32 }
%glitch.KeyValuePair__g2__t40 = type { ptr, ptr }
%glitch.List__g1__t41 = type { i64, ptr, i32 }
%glitch.Dictionary__g2__t42 = type { i64, ptr, i32 }
%glitch.ListEnumerator__g1__t43 = type { i64, ptr, ptr, ptr, i32 }
%glitch.HashSet__g1__t44 = type { i64, ptr, i32, ptr }
%glitch.ValidationFailure__g0__t45 = type { i64, ptr, ptr }
%glitch.ValidationResult__g0__t46 = type { i64, ptr, ptr }
%glitch.ValidationException__g0__t47 = type { i64, ptr, ptr, ptr }
%glitch.ValidationContext__g1__t48 = type { i64, ptr, ptr }
%glitch.IValidator__g1__t49 = type { i64, ptr }
%glitch.RuleBuilder__g2__t50 = type { i64, ptr }
%glitch.AbstractValidator__g1__t51 = type { i64, ptr }
%glitch.RuleBuilder_T_object___g0__t50 = type { i64, ptr }
%glitch.IList_T___g0__t36 = type { i64, ptr, i32, i32 }
%glitch.KeyValuePair_K_V___g0__t40 = type { ptr, ptr }
%glitch.IDictionary_K_V___g0__t38 = type { i64, ptr, i32 }
%glitch.RuleBuilder_T_TProperty___g0__t50 = type { i64, ptr }
%glitch.HashSet_T___g0__t44 = type { i64, ptr, i32, ptr }
%glitch.ListEnumerator_T___g0__t43 = type { i64, ptr, ptr, ptr, i32 }
%glitch.IEnumerator_T___g0__t33 = type { i64, ptr, ptr }
%glitch.IReadOnlyDictionary_K_V___g0__t39 = type { i64, ptr, i32 }
%glitch.IValidator_T___g0__t49 = type { i64, ptr }
%glitch.Rc_T___g0__t30 = type { i64, ptr, ptr, i32 }
%glitch.Weak_T___g0__t31 = type { i64, ptr, ptr }
%glitch.ICollection_T___g0__t34 = type { i64, ptr, i32 }
%glitch.ValidationContext_T___g0__t48 = type { i64, ptr, ptr }
%glitch.AbstractValidator_T___g0__t51 = type { i64, ptr }
%glitch.IEnumerator_KeyValuePair_K_V____g0__t33 = type { i64, ptr, ptr }
declare ptr @System_String_Substring_Native(ptr, i32)
declare ptr @System_String_TrimEnd_Native(ptr, ptr)
declare ptr @System_String_ToLower_Native(ptr)
declare ptr @System_String_ToLowerInvariant_Native(ptr)
declare ptr @System_String_Replace_Native(ptr, ptr, ptr)
declare ptr @System_String_Trim_Native(ptr)
declare ptr @System_String_Split_Native(ptr, ptr)
declare i1 @System_String_Contains_Native(ptr, ptr)
declare ptr @System_String_TrimStart_Native(ptr, ptr)
declare ptr @System_Array_Empty_Native__g1()
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
@.str.0 = private unnamed_addr constant { i64, i64, [9 x i8] } { i64 1000000000, i64 8, [9 x i8] c"Assembly\00" }
@.str.1 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.2 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.3 = private unnamed_addr constant { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"true\00" }
@.str.4 = private unnamed_addr constant { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"false\00" }
@.str.5 = private unnamed_addr constant { i64, i64, [42 x i8] } { i64 1000000000, i64 41, [42 x i8] c"Input string was not in a correct format.\00" }
@.str.6 = private unnamed_addr constant { i64, i64, [30 x i8] } { i64 1000000000, i64 29, [30 x i8] c"Sequence contains no elements\00" }
@.str.7 = private unnamed_addr constant { i64, i64, [39 x i8] } { i64 1000000000, i64 38, [39 x i8] c"Sequence contains no matching elements\00" }
@.str.8 = private unnamed_addr constant { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"404\00" }

define void @glitch_destroy_ValidationException__g0__t47(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ValidationException__g0__t47, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.ValidationException__g0__t47, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t0 = icmp eq ptr %field_3, null
  br i1 %t0, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t1 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 0
  %t2 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 2
  %t3 = load i64, ptr %t1
  %t4 = load ptr, ptr %t2
  %t5 = alloca i64
  store i64 0, ptr %t5
  br label %element_drop_loop_2
element_drop_loop_2:
  %t6 = load i64, ptr %t5
  %t7 = icmp ult i64 %t6, %t3
  br i1 %t7, label %element_drop_body_3, label %element_drop_done_4
element_drop_body_3:
  %t8 = getelementptr inbounds ptr, ptr %t4, i64 %t6
  %t9 = load ptr, ptr %t8
  call void @glitch_drop_ValidationFailure__g0__t45(ptr %t9)
  %t10 = add i64 %t6, 1
  store i64 %t10, ptr %t5
  br label %element_drop_loop_2
element_drop_done_4:
  call void @glitch_free(ptr %t4)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_1
collection_release_done_1:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationException__g0__t47(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationException__g0__t47, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationException__g0__t47(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationException__g0__t47, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationException__g0__t47, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationException__g0__t47(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IReadOnlyDictionary__g2__t39(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IReadOnlyDictionary__g2__t39(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyDictionary__g2__t39, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IReadOnlyDictionary__g2__t39(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyDictionary__g2__t39, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IReadOnlyDictionary__g2__t39, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IReadOnlyDictionary__g2__t39(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MethodInfo__g0__t4(ptr %object) {
entry:
  %field_9_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 9
  %field_9 = load ptr, ptr %field_9_ptr
  %t11 = icmp eq ptr %field_9, null
  br i1 %t11, label %array_release_done_6, label %array_release_5
array_release_5:
  %t12 = getelementptr inbounds %glitch.array, ptr %field_9, i32 0, i32 0
  %t14 = getelementptr inbounds %glitch.array, ptr %field_9, i32 0, i32 1
  %t13 = load i64, ptr %t12
  %t15 = load ptr, ptr %t14
  call void @glitch_free(ptr %t15)
  call void @glitch_free(ptr %field_9)
  br label %array_release_done_6
array_release_done_6:
  %field_4_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t16 = icmp eq ptr %field_4, null
  br i1 %t16, label %array_release_done_8, label %array_release_7
array_release_7:
  %t17 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 0
  %t19 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 1
  %t18 = load i64, ptr %t17
  %t20 = load ptr, ptr %t19
  %t21 = alloca i64
  store i64 0, ptr %t21
  br label %element_drop_loop_9
element_drop_loop_9:
  %t22 = load i64, ptr %t21
  %t23 = icmp ult i64 %t22, %t18
  br i1 %t23, label %element_drop_body_10, label %element_drop_done_11
element_drop_body_10:
  %t24 = getelementptr inbounds ptr, ptr %t20, i64 %t22
  %t25 = load ptr, ptr %t24
  call void @glitch_drop_ParameterInfo__g0__t6(ptr %t25)
  %t26 = add i64 %t22, 1
  store i64 %t26, ptr %t21
  br label %element_drop_loop_9
element_drop_done_11:
  call void @glitch_free(ptr %t20)
  call void @glitch_free(ptr %field_4)
  br label %array_release_done_8
array_release_done_8:
  %field_7_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  %t27 = icmp eq ptr %field_7, null
  br i1 %t27, label %array_release_done_13, label %array_release_12
array_release_12:
  %t28 = getelementptr inbounds %glitch.array, ptr %field_7, i32 0, i32 0
  %t30 = getelementptr inbounds %glitch.array, ptr %field_7, i32 0, i32 1
  %t29 = load i64, ptr %t28
  %t31 = load ptr, ptr %t30
  call void @glitch_free(ptr %t31)
  call void @glitch_free(ptr %field_7)
  br label %array_release_done_13
array_release_done_13:
  %field_2_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MethodInfo__g0__t4(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MethodInfo__g0__t4(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MethodInfo__g0__t4(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IList_T___g0__t36(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IList_T___g0__t36(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IList_T___g0__t36, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IList_T___g0__t36(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IList_T___g0__t36, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IList_T___g0__t36, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IList_T___g0__t36(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_object__g0__t14(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_object__g0__t14(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.object__g0__t14, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_object__g0__t14(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.object__g0__t14, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.object__g0__t14, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_object__g0__t14(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_AppContext__g0__t10(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_AppContext__g0__t10(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.AppContext__g0__t10, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_AppContext__g0__t10(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.AppContext__g0__t10, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.AppContext__g0__t10, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_AppContext__g0__t10(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_TimeSpan__g0__t22(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_TimeSpan__g0__t22(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.TimeSpan__g0__t22, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_TimeSpan__g0__t22(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.TimeSpan__g0__t22, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.TimeSpan__g0__t22, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_TimeSpan__g0__t22(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IDictionary_K_V___g0__t38(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IDictionary_K_V___g0__t38(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IDictionary_K_V___g0__t38, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IDictionary_K_V___g0__t38(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IDictionary_K_V___g0__t38, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IDictionary_K_V___g0__t38, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IDictionary_K_V___g0__t38(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Rc_T___g0__t30(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Rc_T___g0__t30(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Rc_T___g0__t30(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Rc_T___g0__t30(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_EventInfo__g0__t9(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.EventInfo__g0__t9, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_EventInfo__g0__t9(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.EventInfo__g0__t9, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_EventInfo__g0__t9(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.EventInfo__g0__t9, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.EventInfo__g0__t9, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_EventInfo__g0__t9(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Weak_T___g0__t31(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Weak_T___g0__t31(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Weak_T___g0__t31, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Weak_T___g0__t31(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Weak_T___g0__t31, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Weak_T___g0__t31, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Weak_T___g0__t31(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_KeyValuePair_K_V___g0__t40(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %destroy_object
destroy_object:
  call void @glitch_free(ptr %object)
  br label %done
done:
  ret void
}

define void @glitch_destroy_HashSet_T___g0__t44(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t32 = icmp eq ptr %field_3, null
  br i1 %t32, label %collection_release_done_15, label %collection_release_14
collection_release_14:
  %t33 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 0
  %t34 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 2
  %t35 = load i64, ptr %t33
  %t36 = load ptr, ptr %t34
  call void @glitch_free(ptr %t36)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_15
collection_release_done_15:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HashSet_T___g0__t44(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HashSet_T___g0__t44(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HashSet_T___g0__t44(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_RuleBuilder_T_object___g0__t50(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_RuleBuilder_T_object___g0__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_RuleBuilder_T_object___g0__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_RuleBuilder_T_object___g0__t50(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_int__g0__t29(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_int__g0__t29(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.int__g0__t29, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_int__g0__t29(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.int__g0__t29, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.int__g0__t29, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_int__g0__t29(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Weak__g1__t31(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Weak__g1__t31(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Weak__g1__t31, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Weak__g1__t31(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Weak__g1__t31, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Weak__g1__t31, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Weak__g1__t31(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Exception__g0__t16(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Exception__g0__t16, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Exception__g0__t16(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Exception__g0__t16, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Exception__g0__t16(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Exception__g0__t16, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Exception__g0__t16, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Exception__g0__t16(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_AssemblyName__g0__t0(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.AssemblyName__g0__t0, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_AssemblyName__g0__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.AssemblyName__g0__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_AssemblyName__g0__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.AssemblyName__g0__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.AssemblyName__g0__t0, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_AssemblyName__g0__t0(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IEnumerator_KeyValuePair_K_V____g0__t33(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IEnumerator_KeyValuePair_K_V____g0__t33(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerator_KeyValuePair_K_V____g0__t33, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IEnumerator_KeyValuePair_K_V____g0__t33(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerator_KeyValuePair_K_V____g0__t33, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IEnumerator_KeyValuePair_K_V____g0__t33, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IEnumerator_KeyValuePair_K_V____g0__t33(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IValidator__g1__t49(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IValidator__g1__t49(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IValidator__g1__t49, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IValidator__g1__t49(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IValidator__g1__t49, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IValidator__g1__t49, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IValidator__g1__t49(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Array__g0__t25(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Array__g0__t25(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Array__g0__t25, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Array__g0__t25(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Array__g0__t25, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Array__g0__t25, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Array__g0__t25(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_RuleBuilder__g2__t50(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_RuleBuilder__g2__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder__g2__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_RuleBuilder__g2__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder__g2__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.RuleBuilder__g2__t50, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_RuleBuilder__g2__t50(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IList__g1__t36(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IList__g1__t36(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IList__g1__t36, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IList__g1__t36(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IList__g1__t36, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IList__g1__t36, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IList__g1__t36(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_List__g1__t41(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_List__g1__t41(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.List__g1__t41, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_List__g1__t41(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.List__g1__t41, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.List__g1__t41, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_List__g1__t41(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_GC__g0__t12(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_GC__g0__t12(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.GC__g0__t12, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_GC__g0__t12(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.GC__g0__t12, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.GC__g0__t12, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_GC__g0__t12(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DateTime__g0__t20(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DateTime__g0__t20(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DateTime__g0__t20, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DateTime__g0__t20(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DateTime__g0__t20, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DateTime__g0__t20, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DateTime__g0__t20(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ListEnumerator_T___g0__t43(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t37 = icmp eq ptr %field_3, null
  br i1 %t37, label %collection_release_done_17, label %collection_release_16
collection_release_16:
  %t38 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 0
  %t39 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 2
  %t40 = load i64, ptr %t38
  %t41 = load ptr, ptr %t39
  call void @glitch_free(ptr %t41)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_17
collection_release_done_17:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ListEnumerator_T___g0__t43(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ListEnumerator_T___g0__t43(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ListEnumerator_T___g0__t43(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Buffer__g0__t13(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Buffer__g0__t13(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Buffer__g0__t13, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Buffer__g0__t13(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Buffer__g0__t13, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Buffer__g0__t13, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Buffer__g0__t13(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IEnumerator__g1__t33(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IEnumerator__g1__t33(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerator__g1__t33, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IEnumerator__g1__t33(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerator__g1__t33, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IEnumerator__g1__t33, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IEnumerator__g1__t33(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_FormatException__g0__t19(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.FormatException__g0__t19, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_FormatException__g0__t19(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.FormatException__g0__t19, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_FormatException__g0__t19(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.FormatException__g0__t19, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.FormatException__g0__t19, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_FormatException__g0__t19(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IValidator_T___g0__t49(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IValidator_T___g0__t49(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IValidator_T___g0__t49, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IValidator_T___g0__t49(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IValidator_T___g0__t49, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IValidator_T___g0__t49, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IValidator_T___g0__t49(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ValidationFailure__g0__t45(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ValidationFailure__g0__t45, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationFailure__g0__t45(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationFailure__g0__t45, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationFailure__g0__t45(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationFailure__g0__t45, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationFailure__g0__t45, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationFailure__g0__t45(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Dictionary__g2__t42(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Dictionary__g2__t42(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Dictionary__g2__t42, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Dictionary__g2__t42(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Dictionary__g2__t42, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Dictionary__g2__t42, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Dictionary__g2__t42(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_PropertyInfo__g0__t8(ptr %object) {
entry:
  %field_8_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t8, ptr %object, i32 0, i32 8
  %field_8 = load ptr, ptr %field_8_ptr
  call void @glitch_drop_MethodInfo__g0__t4(ptr %field_8)
  %field_7_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t8, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_MethodInfo__g0__t4(ptr %field_7)
  %field_2_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t8, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_PropertyInfo__g0__t8(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t8, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_PropertyInfo__g0__t8(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t8, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t8, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_PropertyInfo__g0__t8(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ValidationResult__g0__t46(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t42 = icmp eq ptr %field_2, null
  br i1 %t42, label %collection_release_done_19, label %collection_release_18
collection_release_18:
  %t43 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t44 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t45 = load i64, ptr %t43
  %t46 = load ptr, ptr %t44
  %t47 = alloca i64
  store i64 0, ptr %t47
  br label %element_drop_loop_20
element_drop_loop_20:
  %t48 = load i64, ptr %t47
  %t49 = icmp ult i64 %t48, %t45
  br i1 %t49, label %element_drop_body_21, label %element_drop_done_22
element_drop_body_21:
  %t50 = getelementptr inbounds ptr, ptr %t46, i64 %t48
  %t51 = load ptr, ptr %t50
  call void @glitch_drop_ValidationFailure__g0__t45(ptr %t51)
  %t52 = add i64 %t48, 1
  store i64 %t52, ptr %t47
  br label %element_drop_loop_20
element_drop_done_22:
  call void @glitch_free(ptr %t46)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_19
collection_release_done_19:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationResult__g0__t46(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationResult__g0__t46(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationResult__g0__t46(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ValidationContext__g1__t48(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationContext__g1__t48(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationContext__g1__t48, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationContext__g1__t48(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationContext__g1__t48, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationContext__g1__t48, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationContext__g1__t48(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_RuleBuilder_T_TProperty___g0__t50(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_RuleBuilder_T_TProperty___g0__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_RuleBuilder_T_TProperty___g0__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_RuleBuilder_T_TProperty___g0__t50(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Type__g0__t24(ptr %object) {
entry:
  %field_17_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 17
  %field_17 = load ptr, ptr %field_17_ptr
  %t53 = icmp eq ptr %field_17, null
  br i1 %t53, label %array_release_done_24, label %array_release_23
array_release_23:
  %t54 = getelementptr inbounds %glitch.array, ptr %field_17, i32 0, i32 0
  %t56 = getelementptr inbounds %glitch.array, ptr %field_17, i32 0, i32 1
  %t55 = load i64, ptr %t54
  %t57 = load ptr, ptr %t56
  %t58 = alloca i64
  store i64 0, ptr %t58
  br label %element_drop_loop_25
element_drop_loop_25:
  %t59 = load i64, ptr %t58
  %t60 = icmp ult i64 %t59, %t55
  br i1 %t60, label %element_drop_body_26, label %element_drop_done_27
element_drop_body_26:
  %t61 = getelementptr inbounds ptr, ptr %t57, i64 %t59
  %t62 = load ptr, ptr %t61
  call void @glitch_drop_FieldInfo__g0__t7(ptr %t62)
  %t63 = add i64 %t59, 1
  store i64 %t63, ptr %t58
  br label %element_drop_loop_25
element_drop_done_27:
  call void @glitch_free(ptr %t57)
  call void @glitch_free(ptr %field_17)
  br label %array_release_done_24
array_release_done_24:
  %field_15_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 15
  %field_15 = load ptr, ptr %field_15_ptr
  %t64 = icmp eq ptr %field_15, null
  br i1 %t64, label %array_release_done_29, label %array_release_28
array_release_28:
  %t65 = getelementptr inbounds %glitch.array, ptr %field_15, i32 0, i32 0
  %t67 = getelementptr inbounds %glitch.array, ptr %field_15, i32 0, i32 1
  %t66 = load i64, ptr %t65
  %t68 = load ptr, ptr %t67
  %t69 = alloca i64
  store i64 0, ptr %t69
  br label %element_drop_loop_30
element_drop_loop_30:
  %t70 = load i64, ptr %t69
  %t71 = icmp ult i64 %t70, %t66
  br i1 %t71, label %element_drop_body_31, label %element_drop_done_32
element_drop_body_31:
  %t72 = getelementptr inbounds ptr, ptr %t68, i64 %t70
  %t73 = load ptr, ptr %t72
  call void @glitch_drop_PropertyInfo__g0__t8(ptr %t73)
  %t74 = add i64 %t70, 1
  store i64 %t74, ptr %t69
  br label %element_drop_loop_30
element_drop_done_32:
  call void @glitch_free(ptr %t68)
  call void @glitch_free(ptr %field_15)
  br label %array_release_done_29
array_release_done_29:
  %field_19_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 19
  %field_19 = load ptr, ptr %field_19_ptr
  %t75 = icmp eq ptr %field_19, null
  br i1 %t75, label %array_release_done_34, label %array_release_33
array_release_33:
  %t76 = getelementptr inbounds %glitch.array, ptr %field_19, i32 0, i32 0
  %t78 = getelementptr inbounds %glitch.array, ptr %field_19, i32 0, i32 1
  %t77 = load i64, ptr %t76
  %t79 = load ptr, ptr %t78
  %t80 = alloca i64
  store i64 0, ptr %t80
  br label %element_drop_loop_35
element_drop_loop_35:
  %t81 = load i64, ptr %t80
  %t82 = icmp ult i64 %t81, %t77
  br i1 %t82, label %element_drop_body_36, label %element_drop_done_37
element_drop_body_36:
  %t83 = getelementptr inbounds ptr, ptr %t79, i64 %t81
  %t84 = load ptr, ptr %t83
  call void @glitch_drop_Type__g0__t24(ptr %t84)
  %t85 = add i64 %t81, 1
  store i64 %t85, ptr %t80
  br label %element_drop_loop_35
element_drop_done_37:
  call void @glitch_free(ptr %t79)
  call void @glitch_free(ptr %field_19)
  br label %array_release_done_34
array_release_done_34:
  %field_3_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_18_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 18
  %field_18 = load ptr, ptr %field_18_ptr
  %t86 = icmp eq ptr %field_18, null
  br i1 %t86, label %array_release_done_39, label %array_release_38
array_release_38:
  %t87 = getelementptr inbounds %glitch.array, ptr %field_18, i32 0, i32 0
  %t89 = getelementptr inbounds %glitch.array, ptr %field_18, i32 0, i32 1
  %t88 = load i64, ptr %t87
  %t90 = load ptr, ptr %t89
  %t91 = alloca i64
  store i64 0, ptr %t91
  br label %element_drop_loop_40
element_drop_loop_40:
  %t92 = load i64, ptr %t91
  %t93 = icmp ult i64 %t92, %t88
  br i1 %t93, label %element_drop_body_41, label %element_drop_done_42
element_drop_body_41:
  %t94 = getelementptr inbounds ptr, ptr %t90, i64 %t92
  %t95 = load ptr, ptr %t94
  call void @glitch_drop_ConstructorInfo__g0__t5(ptr %t95)
  %t96 = add i64 %t92, 1
  store i64 %t96, ptr %t91
  br label %element_drop_loop_40
element_drop_done_42:
  call void @glitch_free(ptr %t90)
  call void @glitch_free(ptr %field_18)
  br label %array_release_done_39
array_release_done_39:
  %field_16_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 16
  %field_16 = load ptr, ptr %field_16_ptr
  %t97 = icmp eq ptr %field_16, null
  br i1 %t97, label %array_release_done_44, label %array_release_43
array_release_43:
  %t98 = getelementptr inbounds %glitch.array, ptr %field_16, i32 0, i32 0
  %t100 = getelementptr inbounds %glitch.array, ptr %field_16, i32 0, i32 1
  %t99 = load i64, ptr %t98
  %t101 = load ptr, ptr %t100
  %t102 = alloca i64
  store i64 0, ptr %t102
  br label %element_drop_loop_45
element_drop_loop_45:
  %t103 = load i64, ptr %t102
  %t104 = icmp ult i64 %t103, %t99
  br i1 %t104, label %element_drop_body_46, label %element_drop_done_47
element_drop_body_46:
  %t105 = getelementptr inbounds ptr, ptr %t101, i64 %t103
  %t106 = load ptr, ptr %t105
  call void @glitch_drop_MethodInfo__g0__t4(ptr %t106)
  %t107 = add i64 %t103, 1
  store i64 %t107, ptr %t102
  br label %element_drop_loop_45
element_drop_done_47:
  call void @glitch_free(ptr %t101)
  call void @glitch_free(ptr %field_16)
  br label %array_release_done_44
array_release_done_44:
  %field_10_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 10
  %field_10 = load ptr, ptr %field_10_ptr
  call void @glitch_string_release(ptr %field_10)
  %field_12_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 12
  %field_12 = load ptr, ptr %field_12_ptr
  %t108 = icmp eq ptr %field_12, null
  br i1 %t108, label %array_release_done_49, label %array_release_48
array_release_48:
  %t109 = getelementptr inbounds %glitch.array, ptr %field_12, i32 0, i32 0
  %t111 = getelementptr inbounds %glitch.array, ptr %field_12, i32 0, i32 1
  %t110 = load i64, ptr %t109
  %t112 = load ptr, ptr %t111
  %t113 = alloca i64
  store i64 0, ptr %t113
  br label %element_drop_loop_50
element_drop_loop_50:
  %t114 = load i64, ptr %t113
  %t115 = icmp ult i64 %t114, %t110
  br i1 %t115, label %element_drop_body_51, label %element_drop_done_52
element_drop_body_51:
  %t116 = getelementptr inbounds ptr, ptr %t112, i64 %t114
  %t117 = load ptr, ptr %t116
  call void @glitch_drop_Type__g0__t24(ptr %t117)
  %t118 = add i64 %t114, 1
  store i64 %t118, ptr %t113
  br label %element_drop_loop_50
element_drop_done_52:
  call void @glitch_free(ptr %t112)
  call void @glitch_free(ptr %field_12)
  br label %array_release_done_49
array_release_done_49:
  %field_11_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 11
  %field_11 = load ptr, ptr %field_11_ptr
  call void @glitch_string_release(ptr %field_11)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Type__g0__t24(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Type__g0__t24(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Type__g0__t24(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Rc__g1__t30(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Rc__g1__t30(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Rc__g1__t30, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Rc__g1__t30(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Rc__g1__t30, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Rc__g1__t30, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Rc__g1__t30(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_InvalidOperationException__g0__t18(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_InvalidOperationException__g0__t18(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_InvalidOperationException__g0__t18(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_InvalidOperationException__g0__t18(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Uri__g0__t11(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Uri__g0__t11(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Uri__g0__t11, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Uri__g0__t11(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Uri__g0__t11, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Uri__g0__t11, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Uri__g0__t11(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ListEnumerator__g1__t43(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ListEnumerator__g1__t43(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ListEnumerator__g1__t43, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ListEnumerator__g1__t43(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ListEnumerator__g1__t43, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ListEnumerator__g1__t43, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ListEnumerator__g1__t43(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IReadOnlyCollection__g1__t35(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IReadOnlyCollection__g1__t35(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyCollection__g1__t35, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IReadOnlyCollection__g1__t35(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyCollection__g1__t35, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IReadOnlyCollection__g1__t35, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IReadOnlyCollection__g1__t35(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ArgumentException__g0__t17(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ArgumentException__g0__t17, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ArgumentException__g0__t17(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ArgumentException__g0__t17, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ArgumentException__g0__t17(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ArgumentException__g0__t17, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ArgumentException__g0__t17, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ArgumentException__g0__t17(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_KeyValuePair__g2__t40(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %destroy_object
destroy_object:
  call void @glitch_free(ptr %object)
  br label %done
done:
  ret void
}

define void @glitch_destroy_HashSet__g1__t44(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.HashSet__g1__t44, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t119 = icmp eq ptr %field_3, null
  br i1 %t119, label %collection_release_done_54, label %collection_release_53
collection_release_53:
  %t120 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 0
  %t121 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 2
  %t122 = load i64, ptr %t120
  %t123 = load ptr, ptr %t121
  call void @glitch_free(ptr %t123)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_54
collection_release_done_54:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HashSet__g1__t44(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HashSet__g1__t44, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HashSet__g1__t44(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HashSet__g1__t44, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HashSet__g1__t44, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HashSet__g1__t44(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ICollection_T___g0__t34(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ICollection_T___g0__t34(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ICollection_T___g0__t34, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ICollection_T___g0__t34(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ICollection_T___g0__t34, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ICollection_T___g0__t34, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ICollection_T___g0__t34(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ValidationContext_T___g0__t48(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationContext_T___g0__t48(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationContext_T___g0__t48, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationContext_T___g0__t48(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationContext_T___g0__t48, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationContext_T___g0__t48, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationContext_T___g0__t48(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MemberInfo__g0__t2(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.MemberInfo__g0__t2, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MemberInfo__g0__t2(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MemberInfo__g0__t2, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MemberInfo__g0__t2(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MemberInfo__g0__t2, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MemberInfo__g0__t2, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MemberInfo__g0__t2(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MethodBase__g0__t3(ptr %object) {
entry:
  %field_4_ptr = getelementptr inbounds %glitch.MethodBase__g0__t3, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t124 = icmp eq ptr %field_4, null
  br i1 %t124, label %array_release_done_56, label %array_release_55
array_release_55:
  %t125 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 0
  %t127 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 1
  %t126 = load i64, ptr %t125
  %t128 = load ptr, ptr %t127
  %t129 = alloca i64
  store i64 0, ptr %t129
  br label %element_drop_loop_57
element_drop_loop_57:
  %t130 = load i64, ptr %t129
  %t131 = icmp ult i64 %t130, %t126
  br i1 %t131, label %element_drop_body_58, label %element_drop_done_59
element_drop_body_58:
  %t132 = getelementptr inbounds ptr, ptr %t128, i64 %t130
  %t133 = load ptr, ptr %t132
  call void @glitch_drop_ParameterInfo__g0__t6(ptr %t133)
  %t134 = add i64 %t130, 1
  store i64 %t134, ptr %t129
  br label %element_drop_loop_57
element_drop_done_59:
  call void @glitch_free(ptr %t128)
  call void @glitch_free(ptr %field_4)
  br label %array_release_done_56
array_release_done_56:
  %field_2_ptr = getelementptr inbounds %glitch.MethodBase__g0__t3, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MethodBase__g0__t3(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MethodBase__g0__t3, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MethodBase__g0__t3(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MethodBase__g0__t3, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MethodBase__g0__t3, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MethodBase__g0__t3(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_String__g0__t27(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_String__g0__t27(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.String__g0__t27, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_String__g0__t27(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.String__g0__t27, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.String__g0__t27, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_String__g0__t27(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_string__g0__t26(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_string__g0__t26(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.string__g0__t26, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_string__g0__t26(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.string__g0__t26, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.string__g0__t26, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_string__g0__t26(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_bool__g0__t28(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_bool__g0__t28(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.bool__g0__t28, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_bool__g0__t28(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.bool__g0__t28, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.bool__g0__t28, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_bool__g0__t28(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_FieldInfo__g0__t7(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.FieldInfo__g0__t7, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_FieldInfo__g0__t7(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.FieldInfo__g0__t7, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_FieldInfo__g0__t7(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.FieldInfo__g0__t7, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.FieldInfo__g0__t7, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_FieldInfo__g0__t7(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IReadOnlyList__g1__t37(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IReadOnlyList__g1__t37(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyList__g1__t37, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IReadOnlyList__g1__t37(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyList__g1__t37, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IReadOnlyList__g1__t37, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IReadOnlyList__g1__t37(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Object__g0__t15(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Object__g0__t15(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Object__g0__t15, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Object__g0__t15(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Object__g0__t15, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Object__g0__t15, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Object__g0__t15(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DateTimeOffset__g0__t21(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DateTimeOffset__g0__t21(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DateTimeOffset__g0__t21, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DateTimeOffset__g0__t21(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DateTimeOffset__g0__t21, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DateTimeOffset__g0__t21, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DateTimeOffset__g0__t21(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_AbstractValidator_T___g0__t51(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_AbstractValidator_T___g0__t51(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.AbstractValidator_T___g0__t51, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_AbstractValidator_T___g0__t51(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.AbstractValidator_T___g0__t51, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.AbstractValidator_T___g0__t51, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_AbstractValidator_T___g0__t51(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ConstructorInfo__g0__t5(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ConstructorInfo__g0__t5, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_6_ptr = getelementptr inbounds %glitch.ConstructorInfo__g0__t5, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  %t135 = icmp eq ptr %field_6, null
  br i1 %t135, label %array_release_done_61, label %array_release_60
array_release_60:
  %t136 = getelementptr inbounds %glitch.array, ptr %field_6, i32 0, i32 0
  %t138 = getelementptr inbounds %glitch.array, ptr %field_6, i32 0, i32 1
  %t137 = load i64, ptr %t136
  %t139 = load ptr, ptr %t138
  call void @glitch_free(ptr %t139)
  call void @glitch_free(ptr %field_6)
  br label %array_release_done_61
array_release_done_61:
  %field_4_ptr = getelementptr inbounds %glitch.ConstructorInfo__g0__t5, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t140 = icmp eq ptr %field_4, null
  br i1 %t140, label %array_release_done_63, label %array_release_62
array_release_62:
  %t141 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 0
  %t143 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 1
  %t142 = load i64, ptr %t141
  %t144 = load ptr, ptr %t143
  %t145 = alloca i64
  store i64 0, ptr %t145
  br label %element_drop_loop_64
element_drop_loop_64:
  %t146 = load i64, ptr %t145
  %t147 = icmp ult i64 %t146, %t142
  br i1 %t147, label %element_drop_body_65, label %element_drop_done_66
element_drop_body_65:
  %t148 = getelementptr inbounds ptr, ptr %t144, i64 %t146
  %t149 = load ptr, ptr %t148
  call void @glitch_drop_ParameterInfo__g0__t6(ptr %t149)
  %t150 = add i64 %t146, 1
  store i64 %t150, ptr %t145
  br label %element_drop_loop_64
element_drop_done_66:
  call void @glitch_free(ptr %t144)
  call void @glitch_free(ptr %field_4)
  br label %array_release_done_63
array_release_done_63:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ConstructorInfo__g0__t5(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ConstructorInfo__g0__t5, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ConstructorInfo__g0__t5(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ConstructorInfo__g0__t5, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ConstructorInfo__g0__t5, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ConstructorInfo__g0__t5(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IDictionary__g2__t38(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IDictionary__g2__t38(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IDictionary__g2__t38, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IDictionary__g2__t38(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IDictionary__g2__t38, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IDictionary__g2__t38, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IDictionary__g2__t38(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IEnumerable__g1__t32(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IEnumerable__g1__t32(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerable__g1__t32, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IEnumerable__g1__t32(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerable__g1__t32, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IEnumerable__g1__t32, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IEnumerable__g1__t32(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IEnumerator_T___g0__t33(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IEnumerator_T___g0__t33(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerator_T___g0__t33, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IEnumerator_T___g0__t33(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerator_T___g0__t33, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IEnumerator_T___g0__t33, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IEnumerator_T___g0__t33(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ICollection__g1__t34(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ICollection__g1__t34(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ICollection__g1__t34, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ICollection__g1__t34(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ICollection__g1__t34, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ICollection__g1__t34, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ICollection__g1__t34(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ParameterInfo__g0__t6(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ParameterInfo__g0__t6, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ParameterInfo__g0__t6(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ParameterInfo__g0__t6, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ParameterInfo__g0__t6(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ParameterInfo__g0__t6, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ParameterInfo__g0__t6, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ParameterInfo__g0__t6(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_AbstractValidator__g1__t51(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_AbstractValidator__g1__t51(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.AbstractValidator__g1__t51, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_AbstractValidator__g1__t51(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.AbstractValidator__g1__t51, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.AbstractValidator__g1__t51, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_AbstractValidator__g1__t51(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IReadOnlyDictionary_K_V___g0__t39(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IReadOnlyDictionary_K_V___g0__t39(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyDictionary_K_V___g0__t39, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IReadOnlyDictionary_K_V___g0__t39(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyDictionary_K_V___g0__t39, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IReadOnlyDictionary_K_V___g0__t39, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IReadOnlyDictionary_K_V___g0__t39(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Enum__g0__t23(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Enum__g0__t23(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Enum__g0__t23, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Enum__g0__t23(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Enum__g0__t23, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Enum__g0__t23, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Enum__g0__t23(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Assembly__g0__t1(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Assembly__g0__t1, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Assembly__g0__t1(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Assembly__g0__t1, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Assembly__g0__t1(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Assembly__g0__t1, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Assembly__g0__t1, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Assembly__g0__t1(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define ptr @Assembly__g0__t1_GetExecutingAssembly__g0() {
entry:
  %t0 = getelementptr %glitch.Assembly__g0__t1, ptr null, i32 1
  %t1 = ptrtoint ptr %t0 to i64
  %t2 = call ptr @glitch_calloc(i64 1, i64 %t1)
  %t3 = getelementptr inbounds %glitch.Assembly__g0__t1, ptr %t2, i32 0, i32 0
  store i64 1, ptr %t3
  %t4 = getelementptr inbounds %glitch.Assembly__g0__t1, ptr %t2, i32 0, i32 1
  store ptr @glitch_destroy_Assembly__g0__t1, ptr %t4
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Assembly__g0__t1_GetName__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr null, ptr %t1
  %t2 = getelementptr %glitch.AssemblyName__g0__t0, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.AssemblyName__g0__t0, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.AssemblyName__g0__t0, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_AssemblyName__g0__t0, ptr %t6
  %t7 = load ptr, ptr %t1
  call void @glitch_drop_AssemblyName__g0__t0(ptr %t7)
  store ptr %t4, ptr %t1
  %t8 = load ptr, ptr %t1
  %t9 = getelementptr inbounds %glitch.AssemblyName__g0__t0, ptr %t8, i32 0, i32 2
  %t10 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t10)
  store ptr getelementptr inbounds ({ i64, i64, [9 x i8] }, ptr @.str.0, i32 0, i32 2, i64 0), ptr %t9
  %t11 = load ptr, ptr %t1
  call void @glitch_retain_AssemblyName__g0__t0(ptr %t11)
  ret ptr %t11
exception_unwind:
  %t12 = load ptr, ptr %t1
  call void @glitch_drop_AssemblyName__g0__t0(ptr %t12)
  ret ptr null
}

define ptr @Assembly__g0__t1_GetType__g0(ptr %this, ptr %fullName) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %fullName, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = load ptr, ptr %t2
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @Assembly__g0__t1_GetTypes__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = call ptr @glitch_calloc(i64 1, i64 16)
  %t2 = call ptr @glitch_calloc(i64 0, i64 8)
  %t3 = getelementptr inbounds %glitch.array, ptr %t1, i32 0, i32 0
  store i64 0, ptr %t3
  %t4 = getelementptr inbounds %glitch.array, ptr %t1, i32 0, i32 1
  store ptr %t2, ptr %t4
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @MethodBase__g0__t3_GetParameters__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.MethodBase__g0__t3, ptr %t1, i32 0, i32 4
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @MethodInfo__g0__t4_GetGenericArguments__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %t1, i32 0, i32 9
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @Uri__g0__t11_ctor(ptr %this, ptr %uri) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %uri, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @GC__g0__t12_SuppressFinalize__g0(ptr %obj) {
entry:
  %t0 = alloca ptr
  store ptr %obj, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @Buffer__g0__t13_BlockCopy__g0(ptr %src, i32 %srcOffset, ptr %dst, i32 %dstOffset, i32 %count) {
entry:
  %t0 = alloca ptr
  store ptr %src, ptr %t0
  %t1 = alloca i32
  store i32 %srcOffset, ptr %t1
  %t2 = alloca ptr
  store ptr %dst, ptr %t2
  %t3 = alloca i32
  store i32 %dstOffset, ptr %t3
  %t4 = alloca i32
  store i32 %count, ptr %t4
  %t5 = alloca i32
  store i32 0, ptr %t5
  %t6 = trunc i64 0 to i32
  store i32 %t6, ptr %t5
  br label %while_cond_0
while_cond_0:
  %t7 = load i32, ptr %t5
  %t8 = load i32, ptr %t4
  %t9 = icmp slt i32 %t7, %t8
  br i1 %t9, label %while_body_1, label %while_end_2
while_body_1:
  %t10 = load ptr, ptr %t2
  %t11 = load i32, ptr %t3
  %t12 = load i32, ptr %t5
  %t13 = add i32 %t11, %t12
  %t14 = sext i32 %t13 to i64
  %t15 = getelementptr inbounds %glitch.array, ptr %t10, i32 0, i32 1
  %t16 = load ptr, ptr %t15
  %t17 = getelementptr inbounds i8, ptr %t16, i64 %t14
  %t18 = load ptr, ptr %t0
  %t19 = load i32, ptr %t1
  %t20 = load i32, ptr %t5
  %t21 = add i32 %t19, %t20
  %t22 = sext i32 %t21 to i64
  %t23 = getelementptr inbounds %glitch.array, ptr %t18, i32 0, i32 1
  %t24 = load ptr, ptr %t23
  %t25 = getelementptr inbounds i8, ptr %t24, i64 %t22
  %t26 = load i8, ptr %t25
  store i8 %t26, ptr %t17
  %t27 = load i32, ptr %t5
  %t28 = trunc i64 1 to i32
  %t29 = add i32 %t27, %t28
  store i32 %t29, ptr %t5
  br label %while_cond_0
while_end_2:
  ret void
exception_unwind:
  ret void
}

define i1 @object__g0__t14_Equals__g0__object_object(ptr %left, ptr %right) {
entry:
  %t0 = alloca ptr
  store ptr %left, ptr %t0
  %t1 = alloca ptr
  store ptr %right, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @object__g0__t14_Equals__g0__object(ptr %this, ptr %other) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %other, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @Object__g0__t15_Equals__g0__object_object(ptr %left, ptr %right) {
entry:
  %t0 = alloca ptr
  store ptr %left, ptr %t0
  %t1 = alloca ptr
  store ptr %right, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @Object__g0__t15_Equals__g0__object(ptr %this, ptr %other) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %other, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @Exception__g0__t16_ctor(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Exception__g0__t16, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @ArgumentException__g0__t17_ctor(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ArgumentException__g0__t17, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @InvalidOperationException__g0__t18_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.1, i32 0, i32 2, i64 0), ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @InvalidOperationException__g0__t18_ctor__string(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @FormatException__g0__t19_ctor(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.FormatException__g0__t19, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @DateTime__g0__t20_Parse__g0(ptr %s) {
entry:
  %t0 = alloca ptr
  store ptr %s, ptr %t0
  ret ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.2, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define void @DateTimeOffset__g0__t21_ctor(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret void
exception_unwind:
  ret void
}

define i64 @DateTimeOffset__g0__t21_ToUnixTimeSeconds__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret i64 0
exception_unwind:
  ret i64 0
}

define void @TimeSpan__g0__t22_ctor(ptr %this, i32 %h, i32 %m, i32 %s, i32 %ms) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %h, ptr %t1
  %t2 = alloca i32
  store i32 %m, ptr %t2
  %t3 = alloca i32
  store i32 %s, ptr %t3
  %t4 = alloca i32
  store i32 %ms, ptr %t4
  ret void
exception_unwind:
  ret void
}

define ptr @TimeSpan__g0__t22_FromMinutes__g0(double %minutes) {
entry:
  %t0 = alloca double
  store double %minutes, ptr %t0
  %t1 = getelementptr %glitch.TimeSpan__g0__t22, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.TimeSpan__g0__t22, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.TimeSpan__g0__t22, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_TimeSpan__g0__t22, ptr %t5
  %t6 = trunc i64 0 to i32
  %t7 = trunc i64 0 to i32
  %t8 = trunc i64 0 to i32
  %t9 = trunc i64 0 to i32
  call void @TimeSpan__g0__t22_ctor(ptr %t3, i32 %t6, i32 %t7, i32 %t8, i32 %t9)
  %t10 = load ptr, ptr @glitch_exception_pending
  %t11 = icmp ne ptr %t10, null
  br i1 %t11, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define i1 @Enum__g0__t23_TryParse__g1(ptr %value, ptr %result) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = alloca ptr
  store ptr %result, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define ptr @Type__g0__t24_GetGenericTypeDefinition__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr null, ptr %t1
  %t2 = getelementptr %glitch.Type__g0__t24, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_Type__g0__t24, ptr %t6
  %t7 = load ptr, ptr %t1
  call void @glitch_drop_Type__g0__t24(ptr %t7)
  store ptr %t4, ptr %t1
  %t8 = load ptr, ptr %t1
  %t9 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t8, i32 0, i32 2
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t10, i32 0, i32 2
  %t12 = load ptr, ptr %t11
  call void @glitch_string_retain(ptr %t12)
  %t13 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t13)
  store ptr %t12, ptr %t9
  %t14 = load ptr, ptr %t1
  %t15 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t14, i32 0, i32 3
  %t16 = load ptr, ptr %t0
  %t17 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t16, i32 0, i32 3
  %t18 = load ptr, ptr %t17
  call void @glitch_string_retain(ptr %t18)
  %t19 = load ptr, ptr %t15
  call void @glitch_string_release(ptr %t19)
  store ptr %t18, ptr %t15
  %t20 = load ptr, ptr %t1
  %t21 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t20, i32 0, i32 10
  %t22 = load ptr, ptr %t0
  %t23 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t22, i32 0, i32 11
  %t24 = load ptr, ptr %t23
  call void @glitch_string_retain(ptr %t24)
  %t25 = load ptr, ptr %t21
  call void @glitch_string_release(ptr %t25)
  store ptr %t24, ptr %t21
  %t26 = load ptr, ptr %t1
  %t27 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t26, i32 0, i32 11
  %t28 = load ptr, ptr %t0
  %t29 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t28, i32 0, i32 11
  %t30 = load ptr, ptr %t29
  call void @glitch_string_retain(ptr %t30)
  %t31 = load ptr, ptr %t27
  call void @glitch_string_release(ptr %t31)
  store ptr %t30, ptr %t27
  %t32 = load ptr, ptr %t1
  %t33 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t32, i32 0, i32 4
  %t34 = load ptr, ptr %t0
  %t35 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t34, i32 0, i32 4
  %t36 = load i1, ptr %t35
  store i1 %t36, ptr %t33
  %t37 = load ptr, ptr %t1
  %t38 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t37, i32 0, i32 5
  %t39 = load ptr, ptr %t0
  %t40 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t39, i32 0, i32 5
  %t41 = load i1, ptr %t40
  store i1 %t41, ptr %t38
  %t42 = load ptr, ptr %t1
  %t43 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t42, i32 0, i32 6
  %t44 = load ptr, ptr %t0
  %t45 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t44, i32 0, i32 6
  %t46 = load i1, ptr %t45
  store i1 %t46, ptr %t43
  %t47 = load ptr, ptr %t1
  %t48 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t47, i32 0, i32 7
  %t49 = load ptr, ptr %t0
  %t50 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t49, i32 0, i32 7
  %t51 = load i1, ptr %t50
  store i1 %t51, ptr %t48
  %t52 = load ptr, ptr %t1
  %t53 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t52, i32 0, i32 8
  %t54 = load ptr, ptr %t0
  %t55 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t54, i32 0, i32 8
  %t56 = load i1, ptr %t55
  store i1 %t56, ptr %t53
  %t57 = load ptr, ptr %t1
  %t58 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t57, i32 0, i32 9
  %t59 = load ptr, ptr %t0
  %t60 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t59, i32 0, i32 9
  %t61 = load i1, ptr %t60
  store i1 %t61, ptr %t58
  %t62 = load ptr, ptr %t1
  %t63 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t62, i32 0, i32 13
  %t64 = load ptr, ptr %t0
  %t65 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t64, i32 0, i32 13
  %t66 = load ptr, ptr %t65
  call void @glitch_retain_Type__g0__t24(ptr %t66)
  %t67 = load ptr, ptr %t63
  call void @glitch_drop_Type__g0__t24(ptr %t67)
  store ptr %t66, ptr %t63
  %t68 = load ptr, ptr %t1
  %t69 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t68, i32 0, i32 14
  %t70 = load ptr, ptr %t0
  %t71 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t70, i32 0, i32 14
  %t72 = load ptr, ptr %t71
  call void @glitch_retain_Type__g0__t24(ptr %t72)
  %t73 = load ptr, ptr %t69
  call void @glitch_drop_Type__g0__t24(ptr %t73)
  store ptr %t72, ptr %t69
  %t74 = load ptr, ptr %t1
  %t75 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t74, i32 0, i32 12
  store ptr null, ptr %t75
  %t76 = load ptr, ptr %t1
  %t77 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t76, i32 0, i32 15
  store ptr null, ptr %t77
  %t78 = load ptr, ptr %t1
  %t79 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t78, i32 0, i32 16
  store ptr null, ptr %t79
  %t80 = load ptr, ptr %t1
  %t81 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t80, i32 0, i32 17
  store ptr null, ptr %t81
  %t82 = load ptr, ptr %t1
  %t83 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t82, i32 0, i32 18
  store ptr null, ptr %t83
  %t84 = load ptr, ptr %t1
  %t85 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t84, i32 0, i32 19
  store ptr null, ptr %t85
  %t86 = load ptr, ptr %t1
  call void @glitch_retain_Type__g0__t24(ptr %t86)
  ret ptr %t86
exception_unwind:
  %t87 = load ptr, ptr %t1
  call void @glitch_drop_Type__g0__t24(ptr %t87)
  ret ptr null
}

define ptr @Type__g0__t24_GetGenericArguments__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t1, i32 0, i32 12
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Type__g0__t24_GetMethod__g0(ptr %this, ptr %name, ptr %types) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %types, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t5, i32 0, i32 16
  %t7 = load ptr, ptr %t6
  %t8 = icmp eq ptr %t7, null
  br i1 %t8, label %if_then_0, label %if_else_1
if_then_0:
  %t9 = load ptr, ptr %t4
  call void @glitch_drop_MethodInfo__g0__t4(ptr %t9)
  ret ptr null
if_else_1:
  br label %if_end_2
if_end_2:
  %t10 = trunc i64 0 to i32
  store i32 %t10, ptr %t3
  br label %while_cond_3
while_cond_3:
  %t11 = load i32, ptr %t3
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t12, i32 0, i32 16
  %t14 = load ptr, ptr %t13
  %t15 = getelementptr inbounds %glitch.array, ptr %t14, i32 0, i32 0
  %t16 = load i64, ptr %t15
  %t17 = trunc i64 %t16 to i32
  %t18 = icmp slt i32 %t11, %t17
  br i1 %t18, label %while_body_4, label %while_end_5
while_body_4:
  %t19 = load ptr, ptr %t0
  %t20 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t19, i32 0, i32 16
  %t21 = load ptr, ptr %t20
  %t22 = load i32, ptr %t3
  %t23 = sext i32 %t22 to i64
  %t24 = getelementptr inbounds %glitch.array, ptr %t21, i32 0, i32 1
  %t25 = load ptr, ptr %t24
  %t26 = getelementptr inbounds ptr, ptr %t25, i64 %t23
  %t27 = load ptr, ptr %t26
  call void @glitch_retain_MethodInfo__g0__t4(ptr %t27)
  %t28 = load ptr, ptr %t4
  call void @glitch_drop_MethodInfo__g0__t4(ptr %t28)
  store ptr %t27, ptr %t4
  %t29 = load ptr, ptr %t4
  %t30 = icmp ne ptr %t29, null
  %t31 = load ptr, ptr %t4
  %t32 = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %t31, i32 0, i32 2
  %t33 = load ptr, ptr %t32
  %t34 = load ptr, ptr %t1
  %t35 = icmp eq ptr %t33, %t34
  %t36 = and i1 %t30, %t35
  br i1 %t36, label %if_then_6, label %if_else_7
if_then_6:
  %t37 = load ptr, ptr %t2
  %t38 = icmp eq ptr %t37, null
  %t39 = load ptr, ptr %t4
  %t40 = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %t39, i32 0, i32 7
  %t41 = load ptr, ptr %t40
  %t42 = icmp eq ptr %t41, null
  %t43 = or i1 %t38, %t42
  %t44 = load ptr, ptr %t4
  %t45 = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %t44, i32 0, i32 7
  %t46 = load ptr, ptr %t45
  %t47 = getelementptr inbounds %glitch.array, ptr %t46, i32 0, i32 0
  %t48 = load i64, ptr %t47
  %t49 = trunc i64 %t48 to i32
  %t50 = load ptr, ptr %t2
  %t51 = getelementptr inbounds %glitch.array, ptr %t50, i32 0, i32 0
  %t52 = load i64, ptr %t51
  %t53 = trunc i64 %t52 to i32
  %t54 = icmp eq i32 %t49, %t53
  %t55 = or i1 %t43, %t54
  br i1 %t55, label %if_then_9, label %if_else_10
if_then_9:
  %t56 = load ptr, ptr %t4
  call void @glitch_retain_MethodInfo__g0__t4(ptr %t56)
  ret ptr %t56
if_else_10:
  br label %if_end_11
if_end_11:
  br label %if_end_8
if_else_7:
  br label %if_end_8
if_end_8:
  %t57 = load i32, ptr %t3
  %t58 = trunc i64 1 to i32
  %t59 = add i32 %t57, %t58
  store i32 %t59, ptr %t3
  br label %while_cond_3
while_end_5:
  %t60 = load ptr, ptr %t4
  call void @glitch_drop_MethodInfo__g0__t4(ptr %t60)
  ret ptr null
exception_unwind:
  %t61 = load ptr, ptr %t4
  call void @glitch_drop_MethodInfo__g0__t4(ptr %t61)
  ret ptr null
}

define ptr @Type__g0__t24_GetMethods__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t1, i32 0, i32 16
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Type__g0__t24_GetProperty__g0(ptr %this, ptr %name, ptr %flags) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %flags, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t5, i32 0, i32 15
  %t7 = load ptr, ptr %t6
  %t8 = icmp eq ptr %t7, null
  br i1 %t8, label %if_then_0, label %if_else_1
if_then_0:
  %t9 = load ptr, ptr %t4
  call void @glitch_drop_PropertyInfo__g0__t8(ptr %t9)
  ret ptr null
if_else_1:
  br label %if_end_2
if_end_2:
  %t10 = trunc i64 0 to i32
  store i32 %t10, ptr %t3
  br label %while_cond_3
while_cond_3:
  %t11 = load i32, ptr %t3
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t12, i32 0, i32 15
  %t14 = load ptr, ptr %t13
  %t15 = getelementptr inbounds %glitch.array, ptr %t14, i32 0, i32 0
  %t16 = load i64, ptr %t15
  %t17 = trunc i64 %t16 to i32
  %t18 = icmp slt i32 %t11, %t17
  br i1 %t18, label %while_body_4, label %while_end_5
while_body_4:
  %t19 = load ptr, ptr %t0
  %t20 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t19, i32 0, i32 15
  %t21 = load ptr, ptr %t20
  %t22 = load i32, ptr %t3
  %t23 = sext i32 %t22 to i64
  %t24 = getelementptr inbounds %glitch.array, ptr %t21, i32 0, i32 1
  %t25 = load ptr, ptr %t24
  %t26 = getelementptr inbounds ptr, ptr %t25, i64 %t23
  %t27 = load ptr, ptr %t26
  call void @glitch_retain_PropertyInfo__g0__t8(ptr %t27)
  %t28 = load ptr, ptr %t4
  call void @glitch_drop_PropertyInfo__g0__t8(ptr %t28)
  store ptr %t27, ptr %t4
  %t29 = load ptr, ptr %t4
  %t30 = icmp ne ptr %t29, null
  %t31 = load ptr, ptr %t4
  %t32 = getelementptr inbounds %glitch.PropertyInfo__g0__t8, ptr %t31, i32 0, i32 2
  %t33 = load ptr, ptr %t32
  %t34 = load ptr, ptr %t1
  %t35 = icmp eq ptr %t33, %t34
  %t36 = and i1 %t30, %t35
  br i1 %t36, label %if_then_6, label %if_else_7
if_then_6:
  %t37 = load ptr, ptr %t4
  call void @glitch_retain_PropertyInfo__g0__t8(ptr %t37)
  ret ptr %t37
if_else_7:
  br label %if_end_8
if_end_8:
  %t38 = load i32, ptr %t3
  %t39 = trunc i64 1 to i32
  %t40 = add i32 %t38, %t39
  store i32 %t40, ptr %t3
  br label %while_cond_3
while_end_5:
  %t41 = load ptr, ptr %t4
  call void @glitch_drop_PropertyInfo__g0__t8(ptr %t41)
  ret ptr null
exception_unwind:
  %t42 = load ptr, ptr %t4
  call void @glitch_drop_PropertyInfo__g0__t8(ptr %t42)
  ret ptr null
}

define ptr @Type__g0__t24_GetProperties__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t1, i32 0, i32 15
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Type__g0__t24_GetField__g0(ptr %this, ptr %name, ptr %flags) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %flags, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t5, i32 0, i32 17
  %t7 = load ptr, ptr %t6
  %t8 = icmp eq ptr %t7, null
  br i1 %t8, label %if_then_0, label %if_else_1
if_then_0:
  %t9 = load ptr, ptr %t4
  call void @glitch_drop_FieldInfo__g0__t7(ptr %t9)
  ret ptr null
if_else_1:
  br label %if_end_2
if_end_2:
  %t10 = trunc i64 0 to i32
  store i32 %t10, ptr %t3
  br label %while_cond_3
while_cond_3:
  %t11 = load i32, ptr %t3
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t12, i32 0, i32 17
  %t14 = load ptr, ptr %t13
  %t15 = getelementptr inbounds %glitch.array, ptr %t14, i32 0, i32 0
  %t16 = load i64, ptr %t15
  %t17 = trunc i64 %t16 to i32
  %t18 = icmp slt i32 %t11, %t17
  br i1 %t18, label %while_body_4, label %while_end_5
while_body_4:
  %t19 = load ptr, ptr %t0
  %t20 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t19, i32 0, i32 17
  %t21 = load ptr, ptr %t20
  %t22 = load i32, ptr %t3
  %t23 = sext i32 %t22 to i64
  %t24 = getelementptr inbounds %glitch.array, ptr %t21, i32 0, i32 1
  %t25 = load ptr, ptr %t24
  %t26 = getelementptr inbounds ptr, ptr %t25, i64 %t23
  %t27 = load ptr, ptr %t26
  call void @glitch_retain_FieldInfo__g0__t7(ptr %t27)
  %t28 = load ptr, ptr %t4
  call void @glitch_drop_FieldInfo__g0__t7(ptr %t28)
  store ptr %t27, ptr %t4
  %t29 = load ptr, ptr %t4
  %t30 = icmp ne ptr %t29, null
  %t31 = load ptr, ptr %t4
  %t32 = getelementptr inbounds %glitch.FieldInfo__g0__t7, ptr %t31, i32 0, i32 2
  %t33 = load ptr, ptr %t32
  %t34 = load ptr, ptr %t1
  %t35 = icmp eq ptr %t33, %t34
  %t36 = and i1 %t30, %t35
  br i1 %t36, label %if_then_6, label %if_else_7
if_then_6:
  %t37 = load ptr, ptr %t4
  call void @glitch_retain_FieldInfo__g0__t7(ptr %t37)
  ret ptr %t37
if_else_7:
  br label %if_end_8
if_end_8:
  %t38 = load i32, ptr %t3
  %t39 = trunc i64 1 to i32
  %t40 = add i32 %t38, %t39
  store i32 %t40, ptr %t3
  br label %while_cond_3
while_end_5:
  %t41 = load ptr, ptr %t4
  call void @glitch_drop_FieldInfo__g0__t7(ptr %t41)
  ret ptr null
exception_unwind:
  %t42 = load ptr, ptr %t4
  call void @glitch_drop_FieldInfo__g0__t7(ptr %t42)
  ret ptr null
}

define ptr @Type__g0__t24_GetFields__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t1, i32 0, i32 17
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Type__g0__t24_GetConstructors__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t1, i32 0, i32 18
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Type__g0__t24_GetInterfaces__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t1, i32 0, i32 19
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Type__g0__t24_GetElementType__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t1, i32 0, i32 14
  %t3 = load ptr, ptr %t2
  call void @glitch_retain_Type__g0__t24(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define i1 @Type__g0__t24_IsAssignableFrom__g0(ptr %this, ptr %other) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %other, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = icmp eq ptr %t2, null
  br i1 %t3, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 0
if_else_1:
  br label %if_end_2
if_end_2:
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 10
  %t6 = load ptr, ptr %t5
  %t7 = load ptr, ptr %t1
  %t8 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t7, i32 0, i32 10
  %t9 = load ptr, ptr %t8
  %t10 = icmp eq ptr %t6, %t9
  br i1 %t10, label %if_then_3, label %if_else_4
if_then_3:
  ret i1 1
if_else_4:
  br label %if_end_5
if_end_5:
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t11, i32 0, i32 13
  %t13 = load ptr, ptr %t12
  %t14 = icmp ne ptr %t13, null
  %t15 = load ptr, ptr %t0
  %t16 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t15, i32 0, i32 13
  %t17 = load ptr, ptr %t16
  %t18 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t17, i32 0, i32 10
  %t19 = load ptr, ptr %t18
  %t20 = load ptr, ptr %t1
  %t21 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t20, i32 0, i32 10
  %t22 = load ptr, ptr %t21
  %t23 = icmp eq ptr %t19, %t22
  %t24 = and i1 %t14, %t23
  br i1 %t24, label %if_then_6, label %if_else_7
if_then_6:
  ret i1 1
if_else_7:
  br label %if_end_8
if_end_8:
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @Type__g0__t24_IsInstanceOfType__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = icmp ne ptr %t2, null
  ret i1 %t3
exception_unwind:
  ret i1 0
}

define ptr @Array__g0__t25_Empty__g1() {
entry:
  %t0 = call ptr @System_Array_Empty_Native__g1()
  %t1 = load ptr, ptr @glitch_exception_pending
  %t2 = icmp ne ptr %t1, null
  br i1 %t2, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t0
exception_unwind:
  ret ptr null
}

define i1 @string__g0__t26_IsNullOrEmpty__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = icmp eq ptr %t1, null
  %t3 = load ptr, ptr %t0
  %t4 = call i64 @strlen(ptr %t3)
  %t5 = trunc i64 %t4 to i32
  %t6 = trunc i64 0 to i32
  %t7 = icmp eq i32 %t5, %t6
  %t8 = or i1 %t2, %t7
  ret i1 %t8
exception_unwind:
  ret i1 0
}

define i1 @string__g0__t26_IsNullOrWhiteSpace__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = icmp eq ptr %t1, null
  %t3 = load ptr, ptr %t0
  %t4 = call i64 @strlen(ptr %t3)
  %t5 = trunc i64 %t4 to i32
  %t6 = trunc i64 0 to i32
  %t7 = icmp eq i32 %t5, %t6
  %t8 = or i1 %t2, %t7
  ret i1 %t8
exception_unwind:
  ret i1 0
}

define ptr @string__g0__t26_ToLower__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_ToLower_Native(ptr %t1)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t26_ToLowerInvariant__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_ToLowerInvariant_Native(ptr %t1)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t26_Substring__g0(ptr %this, i32 %start) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %start, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load i32, ptr %t1
  %t4 = sext i32 %t3 to i64
  %t5 = call ptr @System_String_Substring_Native(ptr %t2, i64 %t4)
  %t6 = load ptr, ptr @glitch_exception_pending
  %t7 = icmp ne ptr %t6, null
  br i1 %t7, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t26_TrimEnd__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_TrimEnd_Native(ptr %t1, ptr null)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t26_TrimEnd__g0__string(ptr %this, ptr %chars) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %chars, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  %t4 = call ptr @System_String_TrimEnd_Native(ptr %t2, ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t26_TrimStart__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_TrimStart_Native(ptr %t1, ptr null)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t26_TrimStart__g0__string(ptr %this, ptr %chars) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %chars, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  %t4 = call ptr @System_String_TrimStart_Native(ptr %t2, ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t26_Split__g0(ptr %this, ptr %separator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %separator, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  %t4 = call ptr @System_String_Split_Native(ptr %t2, ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t26_Replace__g0(ptr %this, ptr %old, ptr %newStr) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %old, ptr %t1
  %t2 = alloca ptr
  store ptr %newStr, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  %t5 = load ptr, ptr %t2
  %t6 = call ptr @System_String_Replace_Native(ptr %t3, ptr %t4, ptr %t5)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t26_Trim__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_Trim_Native(ptr %t1)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define i1 @string__g0__t26_Contains__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  %t4 = call i1 @System_String_Contains_Native(ptr %t2, ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret i1 %t4
exception_unwind:
  ret i1 0
}

define i1 @string__g0__t26_EndsWith__g0(ptr %this, ptr %value, i32 %comparison) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = alloca i32
  store i32 %comparison, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca i32
  store i32 0, ptr %t4
  %t5 = load ptr, ptr %t1
  %t6 = icmp eq ptr %t5, null
  br i1 %t6, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 0
if_else_1:
  br label %if_end_2
if_end_2:
  %t7 = load ptr, ptr %t1
  %t8 = call i64 @strlen(ptr %t7)
  %t9 = trunc i64 %t8 to i32
  %t10 = load ptr, ptr %t0
  %t11 = call i64 @strlen(ptr %t10)
  %t12 = trunc i64 %t11 to i32
  %t13 = icmp sgt i32 %t9, %t12
  br i1 %t13, label %if_then_3, label %if_else_4
if_then_3:
  ret i1 0
if_else_4:
  br label %if_end_5
if_end_5:
  %t14 = load ptr, ptr %t0
  %t15 = call i64 @strlen(ptr %t14)
  %t16 = trunc i64 %t15 to i32
  %t17 = load ptr, ptr %t1
  %t18 = call i64 @strlen(ptr %t17)
  %t19 = trunc i64 %t18 to i32
  %t20 = sub i32 %t16, %t19
  store i32 %t20, ptr %t3
  %t21 = trunc i64 0 to i32
  store i32 %t21, ptr %t4
  br label %while_cond_6
while_cond_6:
  %t22 = load i32, ptr %t4
  %t23 = load ptr, ptr %t1
  %t24 = call i64 @strlen(ptr %t23)
  %t25 = trunc i64 %t24 to i32
  %t26 = icmp slt i32 %t22, %t25
  br i1 %t26, label %while_body_7, label %while_end_8
while_body_7:
  %t27 = load ptr, ptr %t0
  %t28 = load i32, ptr %t3
  %t29 = load i32, ptr %t4
  %t30 = add i32 %t28, %t29
  %t31 = sext i32 %t30 to i64
  %t32 = getelementptr inbounds i8, ptr %t27, i64 %t31
  %t33 = load i8, ptr %t32
  %t34 = load ptr, ptr %t1
  %t35 = load i32, ptr %t4
  %t36 = sext i32 %t35 to i64
  %t37 = getelementptr inbounds i8, ptr %t34, i64 %t36
  %t38 = load i8, ptr %t37
  %t39 = icmp ne i8 %t33, %t38
  br i1 %t39, label %if_then_9, label %if_else_10
if_then_9:
  ret i1 0
if_else_10:
  br label %if_end_11
if_end_11:
  %t40 = load i32, ptr %t4
  %t41 = trunc i64 1 to i32
  %t42 = add i32 %t40, %t41
  store i32 %t42, ptr %t4
  br label %while_cond_6
while_end_8:
  ret i1 1
exception_unwind:
  ret i1 0
}

define i1 @string__g0__t26_StartsWith__g0(ptr %this, ptr %value, i32 %comparison) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = alloca i32
  store i32 %comparison, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = icmp eq ptr %t4, null
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 0
if_else_1:
  br label %if_end_2
if_end_2:
  %t6 = load ptr, ptr %t1
  %t7 = call i64 @strlen(ptr %t6)
  %t8 = trunc i64 %t7 to i32
  %t9 = load ptr, ptr %t0
  %t10 = call i64 @strlen(ptr %t9)
  %t11 = trunc i64 %t10 to i32
  %t12 = icmp sgt i32 %t8, %t11
  br i1 %t12, label %if_then_3, label %if_else_4
if_then_3:
  ret i1 0
if_else_4:
  br label %if_end_5
if_end_5:
  %t13 = trunc i64 0 to i32
  store i32 %t13, ptr %t3
  br label %while_cond_6
while_cond_6:
  %t14 = load i32, ptr %t3
  %t15 = load ptr, ptr %t1
  %t16 = call i64 @strlen(ptr %t15)
  %t17 = trunc i64 %t16 to i32
  %t18 = icmp slt i32 %t14, %t17
  br i1 %t18, label %while_body_7, label %while_end_8
while_body_7:
  %t19 = load ptr, ptr %t0
  %t20 = load i32, ptr %t3
  %t21 = sext i32 %t20 to i64
  %t22 = getelementptr inbounds i8, ptr %t19, i64 %t21
  %t23 = load i8, ptr %t22
  %t24 = load ptr, ptr %t1
  %t25 = load i32, ptr %t3
  %t26 = sext i32 %t25 to i64
  %t27 = getelementptr inbounds i8, ptr %t24, i64 %t26
  %t28 = load i8, ptr %t27
  %t29 = icmp ne i8 %t23, %t28
  br i1 %t29, label %if_then_9, label %if_else_10
if_then_9:
  ret i1 0
if_else_10:
  br label %if_end_11
if_end_11:
  %t30 = load i32, ptr %t3
  %t31 = trunc i64 1 to i32
  %t32 = add i32 %t30, %t31
  store i32 %t32, ptr %t3
  br label %while_cond_6
while_end_8:
  ret i1 1
exception_unwind:
  ret i1 0
}

define i1 @string__g0__t26_Equals__g0(ptr %this, ptr %value, i32 %comparison) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = alloca i32
  store i32 %comparison, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = icmp eq ptr %t4, null
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 0
if_else_1:
  br label %if_end_2
if_end_2:
  %t6 = load ptr, ptr %t0
  %t7 = call i64 @strlen(ptr %t6)
  %t8 = trunc i64 %t7 to i32
  %t9 = load ptr, ptr %t1
  %t10 = call i64 @strlen(ptr %t9)
  %t11 = trunc i64 %t10 to i32
  %t12 = icmp ne i32 %t8, %t11
  br i1 %t12, label %if_then_3, label %if_else_4
if_then_3:
  ret i1 0
if_else_4:
  br label %if_end_5
if_end_5:
  %t13 = trunc i64 0 to i32
  store i32 %t13, ptr %t3
  br label %while_cond_6
while_cond_6:
  %t14 = load i32, ptr %t3
  %t15 = load ptr, ptr %t0
  %t16 = call i64 @strlen(ptr %t15)
  %t17 = trunc i64 %t16 to i32
  %t18 = icmp slt i32 %t14, %t17
  br i1 %t18, label %while_body_7, label %while_end_8
while_body_7:
  %t19 = load ptr, ptr %t0
  %t20 = load i32, ptr %t3
  %t21 = sext i32 %t20 to i64
  %t22 = getelementptr inbounds i8, ptr %t19, i64 %t21
  %t23 = load i8, ptr %t22
  %t24 = load ptr, ptr %t1
  %t25 = load i32, ptr %t3
  %t26 = sext i32 %t25 to i64
  %t27 = getelementptr inbounds i8, ptr %t24, i64 %t26
  %t28 = load i8, ptr %t27
  %t29 = icmp ne i8 %t23, %t28
  br i1 %t29, label %if_then_9, label %if_else_10
if_then_9:
  ret i1 0
if_else_10:
  br label %if_end_11
if_end_11:
  %t30 = load i32, ptr %t3
  %t31 = trunc i64 1 to i32
  %t32 = add i32 %t30, %t31
  store i32 %t32, ptr %t3
  br label %while_cond_6
while_end_8:
  ret i1 1
exception_unwind:
  ret i1 0
}

define i1 @String__g0__t27_IsNullOrEmpty__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = icmp eq ptr %t1, null
  %t3 = load ptr, ptr %t0
  %t4 = call i64 @strlen(ptr %t3)
  %t5 = trunc i64 %t4 to i32
  %t6 = trunc i64 0 to i32
  %t7 = icmp eq i32 %t5, %t6
  %t8 = or i1 %t2, %t7
  ret i1 %t8
exception_unwind:
  ret i1 0
}

define i1 @String__g0__t27_IsNullOrWhiteSpace__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = icmp eq ptr %t1, null
  %t3 = load ptr, ptr %t0
  %t4 = call i64 @strlen(ptr %t3)
  %t5 = trunc i64 %t4 to i32
  %t6 = trunc i64 0 to i32
  %t7 = icmp eq i32 %t5, %t6
  %t8 = or i1 %t2, %t7
  ret i1 %t8
exception_unwind:
  ret i1 0
}

define ptr @String__g0__t27_ToLower__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_ToLower_Native(ptr %t1)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t27_ToLowerInvariant__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_ToLowerInvariant_Native(ptr %t1)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t27_Substring__g0(ptr %this, i32 %start) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %start, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load i32, ptr %t1
  %t4 = sext i32 %t3 to i64
  %t5 = call ptr @System_String_Substring_Native(ptr %t2, i64 %t4)
  %t6 = load ptr, ptr @glitch_exception_pending
  %t7 = icmp ne ptr %t6, null
  br i1 %t7, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t27_TrimEnd__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_TrimEnd_Native(ptr %t1, ptr null)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t27_TrimEnd__g0__string(ptr %this, ptr %chars) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %chars, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  %t4 = call ptr @System_String_TrimEnd_Native(ptr %t2, ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t27_TrimStart__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_TrimStart_Native(ptr %t1, ptr null)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t27_TrimStart__g0__string(ptr %this, ptr %chars) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %chars, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  %t4 = call ptr @System_String_TrimStart_Native(ptr %t2, ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t27_Split__g0(ptr %this, ptr %separator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %separator, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  %t4 = call ptr @System_String_Split_Native(ptr %t2, ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t27_Replace__g0(ptr %this, ptr %old, ptr %newStr) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %old, ptr %t1
  %t2 = alloca ptr
  store ptr %newStr, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  %t5 = load ptr, ptr %t2
  %t6 = call ptr @System_String_Replace_Native(ptr %t3, ptr %t4, ptr %t5)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t27_Trim__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @System_String_Trim_Native(ptr %t1)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define i1 @String__g0__t27_Contains__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  %t4 = call i1 @System_String_Contains_Native(ptr %t2, ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret i1 %t4
exception_unwind:
  ret i1 0
}

define i1 @String__g0__t27_EndsWith__g0(ptr %this, ptr %value, i32 %comparison) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = alloca i32
  store i32 %comparison, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca i32
  store i32 0, ptr %t4
  %t5 = load ptr, ptr %t1
  %t6 = icmp eq ptr %t5, null
  br i1 %t6, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 0
if_else_1:
  br label %if_end_2
if_end_2:
  %t7 = load ptr, ptr %t1
  %t8 = call i64 @strlen(ptr %t7)
  %t9 = trunc i64 %t8 to i32
  %t10 = load ptr, ptr %t0
  %t11 = call i64 @strlen(ptr %t10)
  %t12 = trunc i64 %t11 to i32
  %t13 = icmp sgt i32 %t9, %t12
  br i1 %t13, label %if_then_3, label %if_else_4
if_then_3:
  ret i1 0
if_else_4:
  br label %if_end_5
if_end_5:
  %t14 = load ptr, ptr %t0
  %t15 = call i64 @strlen(ptr %t14)
  %t16 = trunc i64 %t15 to i32
  %t17 = load ptr, ptr %t1
  %t18 = call i64 @strlen(ptr %t17)
  %t19 = trunc i64 %t18 to i32
  %t20 = sub i32 %t16, %t19
  store i32 %t20, ptr %t3
  %t21 = trunc i64 0 to i32
  store i32 %t21, ptr %t4
  br label %while_cond_6
while_cond_6:
  %t22 = load i32, ptr %t4
  %t23 = load ptr, ptr %t1
  %t24 = call i64 @strlen(ptr %t23)
  %t25 = trunc i64 %t24 to i32
  %t26 = icmp slt i32 %t22, %t25
  br i1 %t26, label %while_body_7, label %while_end_8
while_body_7:
  %t27 = load ptr, ptr %t0
  %t28 = load i32, ptr %t3
  %t29 = load i32, ptr %t4
  %t30 = add i32 %t28, %t29
  %t31 = sext i32 %t30 to i64
  %t32 = getelementptr inbounds i8, ptr %t27, i64 %t31
  %t33 = load i8, ptr %t32
  %t34 = load ptr, ptr %t1
  %t35 = load i32, ptr %t4
  %t36 = sext i32 %t35 to i64
  %t37 = getelementptr inbounds i8, ptr %t34, i64 %t36
  %t38 = load i8, ptr %t37
  %t39 = icmp ne i8 %t33, %t38
  br i1 %t39, label %if_then_9, label %if_else_10
if_then_9:
  ret i1 0
if_else_10:
  br label %if_end_11
if_end_11:
  %t40 = load i32, ptr %t4
  %t41 = trunc i64 1 to i32
  %t42 = add i32 %t40, %t41
  store i32 %t42, ptr %t4
  br label %while_cond_6
while_end_8:
  ret i1 1
exception_unwind:
  ret i1 0
}

define i1 @String__g0__t27_StartsWith__g0(ptr %this, ptr %value, i32 %comparison) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = alloca i32
  store i32 %comparison, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = icmp eq ptr %t4, null
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 0
if_else_1:
  br label %if_end_2
if_end_2:
  %t6 = load ptr, ptr %t1
  %t7 = call i64 @strlen(ptr %t6)
  %t8 = trunc i64 %t7 to i32
  %t9 = load ptr, ptr %t0
  %t10 = call i64 @strlen(ptr %t9)
  %t11 = trunc i64 %t10 to i32
  %t12 = icmp sgt i32 %t8, %t11
  br i1 %t12, label %if_then_3, label %if_else_4
if_then_3:
  ret i1 0
if_else_4:
  br label %if_end_5
if_end_5:
  %t13 = trunc i64 0 to i32
  store i32 %t13, ptr %t3
  br label %while_cond_6
while_cond_6:
  %t14 = load i32, ptr %t3
  %t15 = load ptr, ptr %t1
  %t16 = call i64 @strlen(ptr %t15)
  %t17 = trunc i64 %t16 to i32
  %t18 = icmp slt i32 %t14, %t17
  br i1 %t18, label %while_body_7, label %while_end_8
while_body_7:
  %t19 = load ptr, ptr %t0
  %t20 = load i32, ptr %t3
  %t21 = sext i32 %t20 to i64
  %t22 = getelementptr inbounds i8, ptr %t19, i64 %t21
  %t23 = load i8, ptr %t22
  %t24 = load ptr, ptr %t1
  %t25 = load i32, ptr %t3
  %t26 = sext i32 %t25 to i64
  %t27 = getelementptr inbounds i8, ptr %t24, i64 %t26
  %t28 = load i8, ptr %t27
  %t29 = icmp ne i8 %t23, %t28
  br i1 %t29, label %if_then_9, label %if_else_10
if_then_9:
  ret i1 0
if_else_10:
  br label %if_end_11
if_end_11:
  %t30 = load i32, ptr %t3
  %t31 = trunc i64 1 to i32
  %t32 = add i32 %t30, %t31
  store i32 %t32, ptr %t3
  br label %while_cond_6
while_end_8:
  ret i1 1
exception_unwind:
  ret i1 0
}

define i1 @String__g0__t27_Equals__g0(ptr %this, ptr %value, i32 %comparison) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = alloca i32
  store i32 %comparison, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = icmp eq ptr %t4, null
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 0
if_else_1:
  br label %if_end_2
if_end_2:
  %t6 = load ptr, ptr %t0
  %t7 = call i64 @strlen(ptr %t6)
  %t8 = trunc i64 %t7 to i32
  %t9 = load ptr, ptr %t1
  %t10 = call i64 @strlen(ptr %t9)
  %t11 = trunc i64 %t10 to i32
  %t12 = icmp ne i32 %t8, %t11
  br i1 %t12, label %if_then_3, label %if_else_4
if_then_3:
  ret i1 0
if_else_4:
  br label %if_end_5
if_end_5:
  %t13 = trunc i64 0 to i32
  store i32 %t13, ptr %t3
  br label %while_cond_6
while_cond_6:
  %t14 = load i32, ptr %t3
  %t15 = load ptr, ptr %t0
  %t16 = call i64 @strlen(ptr %t15)
  %t17 = trunc i64 %t16 to i32
  %t18 = icmp slt i32 %t14, %t17
  br i1 %t18, label %while_body_7, label %while_end_8
while_body_7:
  %t19 = load ptr, ptr %t0
  %t20 = load i32, ptr %t3
  %t21 = sext i32 %t20 to i64
  %t22 = getelementptr inbounds i8, ptr %t19, i64 %t21
  %t23 = load i8, ptr %t22
  %t24 = load ptr, ptr %t1
  %t25 = load i32, ptr %t3
  %t26 = sext i32 %t25 to i64
  %t27 = getelementptr inbounds i8, ptr %t24, i64 %t26
  %t28 = load i8, ptr %t27
  %t29 = icmp ne i8 %t23, %t28
  br i1 %t29, label %if_then_9, label %if_else_10
if_then_9:
  ret i1 0
if_else_10:
  br label %if_end_11
if_end_11:
  %t30 = load i32, ptr %t3
  %t31 = trunc i64 1 to i32
  %t32 = add i32 %t30, %t31
  store i32 %t32, ptr %t3
  br label %while_cond_6
while_end_8:
  ret i1 1
exception_unwind:
  ret i1 0
}

define i1 @bool__g0__t28_Parse__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = icmp eq ptr %t1, getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.3, i32 0, i32 2, i64 0)
  br i1 %t2, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 1
if_else_1:
  br label %if_end_2
if_end_2:
  %t3 = load ptr, ptr %t0
  %t4 = icmp eq ptr %t3, getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.4, i32 0, i32 2, i64 0)
  br i1 %t4, label %if_then_3, label %if_else_4
if_then_3:
  ret i1 0
if_else_4:
  br label %if_end_5
if_end_5:
  %t5 = getelementptr %glitch.FormatException__g0__t19, ptr null, i32 1
  %t6 = ptrtoint ptr %t5 to i64
  %t7 = call ptr @glitch_calloc(i64 1, i64 %t6)
  %t8 = getelementptr inbounds %glitch.FormatException__g0__t19, ptr %t7, i32 0, i32 0
  store i64 1, ptr %t8
  %t9 = getelementptr inbounds %glitch.FormatException__g0__t19, ptr %t7, i32 0, i32 1
  store ptr @glitch_destroy_FormatException__g0__t19, ptr %t9
  call void @FormatException__g0__t19_ctor(ptr %t7, ptr getelementptr inbounds ({ i64, i64, [42 x i8] }, ptr @.str.5, i32 0, i32 2, i64 0))
  %t10 = load ptr, ptr @glitch_exception_pending
  %t11 = icmp ne ptr %t10, null
  br i1 %t11, label %exception_unwind, label %call_continue_6
call_continue_6:
  store ptr %t7, ptr @glitch_exception_pending
  br label %exception_unwind
exception_unwind:
  ret i1 0
}

define i32 @int__g0__t29_Parse__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret i32 0
exception_unwind:
  ret i32 0
}

define i1 @int__g0__t29_TryParse__g0(ptr %s, i32 %result) {
entry:
  %t0 = alloca ptr
  store ptr %s, ptr %t0
  %t1 = alloca i32
  store i32 %result, ptr %t1
  %t2 = load ptr, ptr %t0
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @Rc__g1__t30_ctor(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Rc__g1__t30_AddRef__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t1, i32 0, i32 3
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t3, i32 0, i32 3
  %t5 = load i32, ptr %t4
  %t6 = trunc i64 1 to i32
  %t7 = add i32 %t5, %t6
  store i32 %t7, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @Rc__g1__t30_Release__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t1, i32 0, i32 3
  %t3 = load i32, ptr %t2
  %t4 = sub i32 %t3, 1
  store i32 %t4, ptr %t2
  %t5 = trunc i64 0 to i32
  %t6 = icmp eq i32 %t4, %t5
  br i1 %t6, label %if_then_0, label %if_else_1
if_then_0:
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  ret void
exception_unwind:
  ret void
}

define ptr @Rc__g1__t30_Get__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @Weak__g1__t31_ctor(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret void
exception_unwind:
  ret void
}

define i1 @Weak__g1__t31_TryGetTarget__g0(ptr %this, ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %target, ptr %t1
  store ptr null, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @KeyValuePair__g2__t40_ctor(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.KeyValuePair_K_V___g0__t40, ptr %t3, i32 0, i32 0
  %t5 = load ptr, ptr %t1
  store ptr %t5, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.KeyValuePair_K_V___g0__t40, ptr %t6, i32 0, i32 1
  %t8 = load ptr, ptr %t2
  store ptr %t8, ptr %t7
  ret void
exception_unwind:
  ret void
}

define void @List__g1__t41_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @List__g1__t41_ToArray__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @List__g1__t41_ToList__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr null, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = call ptr @glitch_calloc(i64 1, i64 24)
  %t4 = call ptr @glitch_calloc(i64 4, i64 8)
  %t5 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 1
  store i64 4, ptr %t5
  %t6 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 2
  store ptr %t4, ptr %t6
  %t7 = load ptr, ptr %t1
  %t8 = icmp eq ptr %t7, null
  br i1 %t8, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t9 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 0
  %t10 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 2
  %t11 = load i64, ptr %t9
  %t12 = load ptr, ptr %t10
  call void @glitch_free(ptr %t12)
  call void @glitch_free(ptr %t7)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t3, ptr %t1
  %t13 = trunc i64 0 to i32
  store i32 %t13, ptr %t2
  br label %while_cond_2
while_cond_2:
  %t14 = load i32, ptr %t2
  %t15 = load ptr, ptr %t0
  %t16 = getelementptr inbounds %glitch.list, ptr %t15, i32 0, i32 0
  %t17 = load i64, ptr %t16
  %t18 = trunc i64 %t17 to i32
  %t19 = icmp slt i32 %t14, %t18
  br i1 %t19, label %while_body_3, label %while_end_4
while_body_3:
  %t20 = load ptr, ptr %t1
  %t21 = load ptr, ptr %t0
  %t22 = load i32, ptr %t2
  %t23 = sext i32 %t22 to i64
  %t24 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 2
  %t25 = load ptr, ptr %t24
  %t26 = getelementptr inbounds ptr, ptr %t25, i64 %t23
  %t27 = load ptr, ptr %t26
  %t28 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 0
  %t29 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 1
  %t30 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 2
  %t31 = load i64, ptr %t28
  %t32 = load i64, ptr %t29
  %t33 = load ptr, ptr %t30
  %t34 = icmp eq i64 %t31, %t32
  br i1 %t34, label %list_grow_5, label %list_ready_6
list_grow_5:
  %t35 = mul i64 %t32, 2
  %t36 = mul i64 %t35, 8
  %t37 = call ptr @glitch_realloc(ptr %t33, i64 %t36)
  store i64 %t35, ptr %t29
  store ptr %t37, ptr %t30
  br label %list_ready_6
list_ready_6:
  %t38 = load ptr, ptr %t30
  %t39 = getelementptr inbounds ptr, ptr %t38, i64 %t31
  store ptr %t27, ptr %t39
  %t40 = add i64 %t31, 1
  store i64 %t40, ptr %t28
  %t41 = load i32, ptr %t2
  %t42 = trunc i64 1 to i32
  %t43 = add i32 %t41, %t42
  store i32 %t43, ptr %t2
  br label %while_cond_2
while_end_4:
  %t44 = load ptr, ptr %t1
  ret ptr %t44
exception_unwind:
  %t45 = load ptr, ptr %t1
  %t46 = icmp eq ptr %t45, null
  br i1 %t46, label %collection_release_done_8, label %collection_release_7
collection_release_7:
  %t47 = getelementptr inbounds %glitch.list, ptr %t45, i32 0, i32 0
  %t48 = getelementptr inbounds %glitch.list, ptr %t45, i32 0, i32 2
  %t49 = load i64, ptr %t47
  %t50 = load ptr, ptr %t48
  call void @glitch_free(ptr %t50)
  call void @glitch_free(ptr %t45)
  br label %collection_release_done_8
collection_release_done_8:
  ret ptr null
}

define ptr @List__g1__t41_Where__g0(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  %t9 = load ptr, ptr %t2
  %t10 = icmp eq ptr %t9, null
  br i1 %t10, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 0
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  %t13 = load i64, ptr %t11
  %t14 = load ptr, ptr %t12
  call void @glitch_free(ptr %t14)
  call void @glitch_free(ptr %t9)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t5, ptr %t2
  %t15 = trunc i64 0 to i32
  store i32 %t15, ptr %t3
  br label %while_cond_2
while_cond_2:
  %t16 = load i32, ptr %t3
  %t17 = load ptr, ptr %t0
  %t18 = getelementptr inbounds %glitch.list, ptr %t17, i32 0, i32 0
  %t19 = load i64, ptr %t18
  %t20 = trunc i64 %t19 to i32
  %t21 = icmp slt i32 %t16, %t20
  br i1 %t21, label %while_body_3, label %while_end_4
while_body_3:
  %t22 = load ptr, ptr %t0
  %t23 = load i32, ptr %t3
  %t24 = sext i32 %t23 to i64
  %t25 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 2
  %t26 = load ptr, ptr %t25
  %t27 = getelementptr inbounds ptr, ptr %t26, i64 %t24
  %t28 = load ptr, ptr %t27
  store ptr %t28, ptr %t4
  %t29 = load ptr, ptr %t1
  %t30 = getelementptr inbounds %glitch.delegate, ptr %t29, i32 0, i32 1
  %t31 = load ptr, ptr %t30
  %t32 = getelementptr inbounds %glitch.delegate, ptr %t29, i32 0, i32 2
  %t33 = load ptr, ptr %t32
  %t34 = load ptr, ptr %t4
  %t35 = call ptr %t31(ptr %t33, ptr %t34)
  %t36 = icmp ne ptr %t35, null
  br i1 %t36, label %if_then_5, label %if_else_6
if_then_5:
  %t37 = load ptr, ptr %t2
  %t38 = load ptr, ptr %t4
  %t39 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 0
  %t40 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 1
  %t41 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 2
  %t42 = load i64, ptr %t39
  %t43 = load i64, ptr %t40
  %t44 = load ptr, ptr %t41
  %t45 = icmp eq i64 %t42, %t43
  br i1 %t45, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t46 = mul i64 %t43, 2
  %t47 = mul i64 %t46, 8
  %t48 = call ptr @glitch_realloc(ptr %t44, i64 %t47)
  store i64 %t46, ptr %t40
  store ptr %t48, ptr %t41
  br label %list_ready_9
list_ready_9:
  %t49 = load ptr, ptr %t41
  %t50 = getelementptr inbounds ptr, ptr %t49, i64 %t42
  store ptr %t38, ptr %t50
  %t51 = add i64 %t42, 1
  store i64 %t51, ptr %t39
  br label %if_end_7
if_else_6:
  br label %if_end_7
if_end_7:
  %t52 = load i32, ptr %t3
  %t53 = trunc i64 1 to i32
  %t54 = add i32 %t52, %t53
  store i32 %t54, ptr %t3
  br label %while_cond_2
while_end_4:
  %t55 = load ptr, ptr %t2
  ret ptr %t55
exception_unwind:
  %t56 = load ptr, ptr %t2
  %t57 = icmp eq ptr %t56, null
  br i1 %t57, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t58 = getelementptr inbounds %glitch.list, ptr %t56, i32 0, i32 0
  %t59 = getelementptr inbounds %glitch.list, ptr %t56, i32 0, i32 2
  %t60 = load i64, ptr %t58
  %t61 = load ptr, ptr %t59
  call void @glitch_free(ptr %t61)
  call void @glitch_free(ptr %t56)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr null
}

define ptr @List__g1__t41_Select__g1(ptr %this, ptr %selector) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %selector, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = call ptr @glitch_calloc(i64 1, i64 24)
  %t5 = call ptr @glitch_calloc(i64 4, i64 8)
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 1
  store i64 4, ptr %t6
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  store ptr %t5, ptr %t7
  %t8 = load ptr, ptr %t2
  %t9 = icmp eq ptr %t8, null
  br i1 %t9, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t10 = getelementptr inbounds %glitch.list, ptr %t8, i32 0, i32 0
  %t11 = getelementptr inbounds %glitch.list, ptr %t8, i32 0, i32 2
  %t12 = load i64, ptr %t10
  %t13 = load ptr, ptr %t11
  call void @glitch_free(ptr %t13)
  call void @glitch_free(ptr %t8)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t4, ptr %t2
  %t14 = trunc i64 0 to i32
  store i32 %t14, ptr %t3
  br label %while_cond_2
while_cond_2:
  %t15 = load i32, ptr %t3
  %t16 = load ptr, ptr %t0
  %t17 = getelementptr inbounds %glitch.list, ptr %t16, i32 0, i32 0
  %t18 = load i64, ptr %t17
  %t19 = trunc i64 %t18 to i32
  %t20 = icmp slt i32 %t15, %t19
  br i1 %t20, label %while_body_3, label %while_end_4
while_body_3:
  %t21 = load ptr, ptr %t2
  %t22 = load ptr, ptr %t1
  %t23 = getelementptr inbounds %glitch.delegate, ptr %t22, i32 0, i32 1
  %t24 = load ptr, ptr %t23
  %t25 = getelementptr inbounds %glitch.delegate, ptr %t22, i32 0, i32 2
  %t26 = load ptr, ptr %t25
  %t27 = load ptr, ptr %t0
  %t28 = load i32, ptr %t3
  %t29 = sext i32 %t28 to i64
  %t30 = getelementptr inbounds %glitch.list, ptr %t27, i32 0, i32 2
  %t31 = load ptr, ptr %t30
  %t32 = getelementptr inbounds ptr, ptr %t31, i64 %t29
  %t33 = load ptr, ptr %t32
  %t34 = call ptr %t24(ptr %t26, ptr %t33)
  %t35 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 0
  %t36 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 1
  %t37 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 2
  %t38 = load i64, ptr %t35
  %t39 = load i64, ptr %t36
  %t40 = load ptr, ptr %t37
  %t41 = icmp eq i64 %t38, %t39
  br i1 %t41, label %list_grow_5, label %list_ready_6
list_grow_5:
  %t42 = mul i64 %t39, 2
  %t43 = mul i64 %t42, 8
  %t44 = call ptr @glitch_realloc(ptr %t40, i64 %t43)
  store i64 %t42, ptr %t36
  store ptr %t44, ptr %t37
  br label %list_ready_6
list_ready_6:
  %t45 = load ptr, ptr %t37
  %t46 = getelementptr inbounds ptr, ptr %t45, i64 %t38
  store ptr %t34, ptr %t46
  %t47 = add i64 %t38, 1
  store i64 %t47, ptr %t35
  %t48 = load i32, ptr %t3
  %t49 = trunc i64 1 to i32
  %t50 = add i32 %t48, %t49
  store i32 %t50, ptr %t3
  br label %while_cond_2
while_end_4:
  %t51 = load ptr, ptr %t2
  ret ptr %t51
exception_unwind:
  %t52 = load ptr, ptr %t2
  %t53 = icmp eq ptr %t52, null
  br i1 %t53, label %collection_release_done_8, label %collection_release_7
collection_release_7:
  %t54 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 0
  %t55 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 2
  %t56 = load i64, ptr %t54
  %t57 = load ptr, ptr %t55
  call void @glitch_free(ptr %t57)
  call void @glitch_free(ptr %t52)
  br label %collection_release_done_8
collection_release_done_8:
  ret ptr null
}

define ptr @List__g1__t41_SelectMany__g1(ptr %this, ptr %selector) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %selector, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  %t9 = load ptr, ptr %t2
  %t10 = icmp eq ptr %t9, null
  br i1 %t10, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 0
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  %t13 = load i64, ptr %t11
  %t14 = load ptr, ptr %t12
  call void @glitch_free(ptr %t14)
  call void @glitch_free(ptr %t9)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t5, ptr %t2
  %t15 = trunc i64 0 to i32
  store i32 %t15, ptr %t3
  br label %while_cond_2
while_cond_2:
  %t16 = load i32, ptr %t3
  %t17 = load ptr, ptr %t0
  %t18 = getelementptr inbounds %glitch.list, ptr %t17, i32 0, i32 0
  %t19 = load i64, ptr %t18
  %t20 = trunc i64 %t19 to i32
  %t21 = icmp slt i32 %t16, %t20
  br i1 %t21, label %while_body_3, label %while_end_4
while_body_3:
  %t22 = load ptr, ptr %t1
  %t23 = getelementptr inbounds %glitch.delegate, ptr %t22, i32 0, i32 1
  %t24 = load ptr, ptr %t23
  %t25 = getelementptr inbounds %glitch.delegate, ptr %t22, i32 0, i32 2
  %t26 = load ptr, ptr %t25
  %t27 = load ptr, ptr %t0
  %t28 = load i32, ptr %t3
  %t29 = sext i32 %t28 to i64
  %t30 = getelementptr inbounds %glitch.list, ptr %t27, i32 0, i32 2
  %t31 = load ptr, ptr %t30
  %t32 = getelementptr inbounds ptr, ptr %t31, i64 %t29
  %t33 = load ptr, ptr %t32
  %t34 = call ptr %t24(ptr %t26, ptr %t33)
  %t35 = load i32, ptr %t3
  %t36 = trunc i64 1 to i32
  %t37 = add i32 %t35, %t36
  store i32 %t37, ptr %t3
  br label %while_cond_2
while_end_4:
  %t38 = load ptr, ptr %t2
  ret ptr %t38
exception_unwind:
  %t39 = load ptr, ptr %t2
  %t40 = icmp eq ptr %t39, null
  br i1 %t40, label %collection_release_done_6, label %collection_release_5
collection_release_5:
  %t41 = getelementptr inbounds %glitch.list, ptr %t39, i32 0, i32 0
  %t42 = getelementptr inbounds %glitch.list, ptr %t39, i32 0, i32 2
  %t43 = load i64, ptr %t41
  %t44 = load ptr, ptr %t42
  call void @glitch_free(ptr %t44)
  call void @glitch_free(ptr %t39)
  br label %collection_release_done_6
collection_release_done_6:
  ret ptr null
}

define i1 @List__g1__t41_Any__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.list, ptr %t1, i32 0, i32 0
  %t3 = load i64, ptr %t2
  %t4 = trunc i64 %t3 to i32
  %t5 = trunc i64 0 to i32
  %t6 = icmp sgt i32 %t4, %t5
  ret i1 %t6
exception_unwind:
  ret i1 0
}

define i1 @List__g1__t41_Any__g0__fn_T_ret_bool(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = trunc i64 0 to i32
  store i32 %t3, ptr %t2
  br label %while_cond_0
while_cond_0:
  %t4 = load i32, ptr %t2
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 0
  %t7 = load i64, ptr %t6
  %t8 = trunc i64 %t7 to i32
  %t9 = icmp slt i32 %t4, %t8
  br i1 %t9, label %while_body_1, label %while_end_2
while_body_1:
  %t10 = load ptr, ptr %t1
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 1
  %t12 = load ptr, ptr %t11
  %t13 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 2
  %t14 = load ptr, ptr %t13
  %t15 = load ptr, ptr %t0
  %t16 = load i32, ptr %t2
  %t17 = sext i32 %t16 to i64
  %t18 = getelementptr inbounds %glitch.list, ptr %t15, i32 0, i32 2
  %t19 = load ptr, ptr %t18
  %t20 = getelementptr inbounds ptr, ptr %t19, i64 %t17
  %t21 = load ptr, ptr %t20
  %t22 = call ptr %t12(ptr %t14, ptr %t21)
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %if_then_3, label %if_else_4
if_then_3:
  ret i1 1
if_else_4:
  br label %if_end_5
if_end_5:
  %t24 = load i32, ptr %t2
  %t25 = trunc i64 1 to i32
  %t26 = add i32 %t24, %t25
  store i32 %t26, ptr %t2
  br label %while_cond_0
while_end_2:
  ret i1 0
exception_unwind:
  ret i1 0
}

define ptr @List__g1__t41_First__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.list, ptr %t1, i32 0, i32 0
  %t3 = load i64, ptr %t2
  %t4 = trunc i64 %t3 to i32
  %t5 = trunc i64 0 to i32
  %t6 = icmp sgt i32 %t4, %t5
  br i1 %t6, label %if_then_0, label %if_else_1
if_then_0:
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  %t10 = getelementptr inbounds ptr, ptr %t9, i64 0
  %t11 = load ptr, ptr %t10
  ret ptr %t11
if_else_1:
  br label %if_end_2
if_end_2:
  %t12 = getelementptr %glitch.InvalidOperationException__g0__t18, ptr null, i32 1
  %t13 = ptrtoint ptr %t12 to i64
  %t14 = call ptr @glitch_calloc(i64 1, i64 %t13)
  %t15 = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %t14, i32 0, i32 0
  store i64 1, ptr %t15
  %t16 = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %t14, i32 0, i32 1
  store ptr @glitch_destroy_InvalidOperationException__g0__t18, ptr %t16
  call void @InvalidOperationException__g0__t18_ctor__string(ptr %t14, ptr getelementptr inbounds ({ i64, i64, [30 x i8] }, ptr @.str.6, i32 0, i32 2, i64 0))
  %t17 = load ptr, ptr @glitch_exception_pending
  %t18 = icmp ne ptr %t17, null
  br i1 %t18, label %exception_unwind, label %call_continue_3
call_continue_3:
  store ptr %t14, ptr @glitch_exception_pending
  br label %exception_unwind
exception_unwind:
  ret ptr null
}

define ptr @List__g1__t41_First__g0__fn_T_ret_bool(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = trunc i64 0 to i32
  store i32 %t4, ptr %t2
  br label %while_cond_0
while_cond_0:
  %t5 = load i32, ptr %t2
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 0
  %t8 = load i64, ptr %t7
  %t9 = trunc i64 %t8 to i32
  %t10 = icmp slt i32 %t5, %t9
  br i1 %t10, label %while_body_1, label %while_end_2
while_body_1:
  %t11 = load ptr, ptr %t0
  %t12 = load i32, ptr %t2
  %t13 = sext i32 %t12 to i64
  %t14 = getelementptr inbounds %glitch.list, ptr %t11, i32 0, i32 2
  %t15 = load ptr, ptr %t14
  %t16 = getelementptr inbounds ptr, ptr %t15, i64 %t13
  %t17 = load ptr, ptr %t16
  store ptr %t17, ptr %t3
  %t18 = load ptr, ptr %t1
  %t19 = getelementptr inbounds %glitch.delegate, ptr %t18, i32 0, i32 1
  %t20 = load ptr, ptr %t19
  %t21 = getelementptr inbounds %glitch.delegate, ptr %t18, i32 0, i32 2
  %t22 = load ptr, ptr %t21
  %t23 = load ptr, ptr %t3
  %t24 = call ptr %t20(ptr %t22, ptr %t23)
  %t25 = icmp ne ptr %t24, null
  br i1 %t25, label %if_then_3, label %if_else_4
if_then_3:
  %t26 = load ptr, ptr %t3
  ret ptr %t26
if_else_4:
  br label %if_end_5
if_end_5:
  %t27 = load i32, ptr %t2
  %t28 = trunc i64 1 to i32
  %t29 = add i32 %t27, %t28
  store i32 %t29, ptr %t2
  br label %while_cond_0
while_end_2:
  %t30 = getelementptr %glitch.InvalidOperationException__g0__t18, ptr null, i32 1
  %t31 = ptrtoint ptr %t30 to i64
  %t32 = call ptr @glitch_calloc(i64 1, i64 %t31)
  %t33 = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %t32, i32 0, i32 0
  store i64 1, ptr %t33
  %t34 = getelementptr inbounds %glitch.InvalidOperationException__g0__t18, ptr %t32, i32 0, i32 1
  store ptr @glitch_destroy_InvalidOperationException__g0__t18, ptr %t34
  call void @InvalidOperationException__g0__t18_ctor__string(ptr %t32, ptr getelementptr inbounds ({ i64, i64, [39 x i8] }, ptr @.str.7, i32 0, i32 2, i64 0))
  %t35 = load ptr, ptr @glitch_exception_pending
  %t36 = icmp ne ptr %t35, null
  br i1 %t36, label %exception_unwind, label %call_continue_6
call_continue_6:
  store ptr %t32, ptr @glitch_exception_pending
  br label %exception_unwind
exception_unwind:
  ret ptr null
}

define ptr @List__g1__t41_FirstOrDefault__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.list, ptr %t1, i32 0, i32 0
  %t3 = load i64, ptr %t2
  %t4 = trunc i64 %t3 to i32
  %t5 = trunc i64 0 to i32
  %t6 = icmp sgt i32 %t4, %t5
  br i1 %t6, label %if_then_0, label %if_else_1
if_then_0:
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  %t10 = getelementptr inbounds ptr, ptr %t9, i64 0
  %t11 = load ptr, ptr %t10
  ret ptr %t11
if_else_1:
  br label %if_end_2
if_end_2:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @List__g1__t41_FirstOrDefault__g0__fn_T_ret_bool(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = trunc i64 0 to i32
  store i32 %t4, ptr %t2
  br label %while_cond_0
while_cond_0:
  %t5 = load i32, ptr %t2
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 0
  %t8 = load i64, ptr %t7
  %t9 = trunc i64 %t8 to i32
  %t10 = icmp slt i32 %t5, %t9
  br i1 %t10, label %while_body_1, label %while_end_2
while_body_1:
  %t11 = load ptr, ptr %t0
  %t12 = load i32, ptr %t2
  %t13 = sext i32 %t12 to i64
  %t14 = getelementptr inbounds %glitch.list, ptr %t11, i32 0, i32 2
  %t15 = load ptr, ptr %t14
  %t16 = getelementptr inbounds ptr, ptr %t15, i64 %t13
  %t17 = load ptr, ptr %t16
  store ptr %t17, ptr %t3
  %t18 = load ptr, ptr %t1
  %t19 = getelementptr inbounds %glitch.delegate, ptr %t18, i32 0, i32 1
  %t20 = load ptr, ptr %t19
  %t21 = getelementptr inbounds %glitch.delegate, ptr %t18, i32 0, i32 2
  %t22 = load ptr, ptr %t21
  %t23 = load ptr, ptr %t3
  %t24 = call ptr %t20(ptr %t22, ptr %t23)
  %t25 = icmp ne ptr %t24, null
  br i1 %t25, label %if_then_3, label %if_else_4
if_then_3:
  %t26 = load ptr, ptr %t3
  ret ptr %t26
if_else_4:
  br label %if_end_5
if_end_5:
  %t27 = load i32, ptr %t2
  %t28 = trunc i64 1 to i32
  %t29 = add i32 %t27, %t28
  store i32 %t29, ptr %t2
  br label %while_cond_0
while_end_2:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @List__g1__t41_OrderBy__g0(ptr %this, ptr %keySelector) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %keySelector, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = alloca i32
  store i32 0, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = call ptr @List__g1__t41_ToList__g0(ptr %t7)
  %t9 = load ptr, ptr @glitch_exception_pending
  %t10 = icmp ne ptr %t9, null
  br i1 %t10, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t8, ptr %t2
  %t11 = trunc i64 1 to i32
  store i32 %t11, ptr %t3
  br label %while_cond_1
while_cond_1:
  %t12 = load i32, ptr %t3
  %t13 = load ptr, ptr %t2
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  %t15 = load i64, ptr %t14
  %t16 = trunc i64 %t15 to i32
  %t17 = icmp slt i32 %t12, %t16
  br i1 %t17, label %while_body_2, label %while_end_3
while_body_2:
  %t18 = load ptr, ptr %t2
  %t19 = load i32, ptr %t3
  %t20 = sext i32 %t19 to i64
  %t21 = getelementptr inbounds %glitch.list, ptr %t18, i32 0, i32 2
  %t22 = load ptr, ptr %t21
  %t23 = getelementptr inbounds ptr, ptr %t22, i64 %t20
  %t24 = load ptr, ptr %t23
  store ptr %t24, ptr %t4
  %t25 = load ptr, ptr %t1
  %t26 = getelementptr inbounds %glitch.delegate, ptr %t25, i32 0, i32 1
  %t27 = load ptr, ptr %t26
  %t28 = getelementptr inbounds %glitch.delegate, ptr %t25, i32 0, i32 2
  %t29 = load ptr, ptr %t28
  %t30 = load ptr, ptr %t4
  %t31 = call ptr %t27(ptr %t29, ptr %t30)
  %t32 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t32)
  store ptr %t31, ptr %t5
  %t33 = load i32, ptr %t3
  %t34 = trunc i64 1 to i32
  %t35 = sub i32 %t33, %t34
  store i32 %t35, ptr %t6
  br label %while_cond_4
while_cond_4:
  %t36 = load i32, ptr %t6
  %t37 = trunc i64 0 to i32
  %t38 = icmp sge i32 %t36, %t37
  %t39 = load ptr, ptr %t1
  %t40 = getelementptr inbounds %glitch.delegate, ptr %t39, i32 0, i32 1
  %t41 = load ptr, ptr %t40
  %t42 = getelementptr inbounds %glitch.delegate, ptr %t39, i32 0, i32 2
  %t43 = load ptr, ptr %t42
  %t44 = load ptr, ptr %t2
  %t45 = load i32, ptr %t6
  %t46 = sext i32 %t45 to i64
  %t47 = getelementptr inbounds %glitch.list, ptr %t44, i32 0, i32 2
  %t48 = load ptr, ptr %t47
  %t49 = getelementptr inbounds ptr, ptr %t48, i64 %t46
  %t50 = load ptr, ptr %t49
  %t51 = call ptr %t41(ptr %t43, ptr %t50)
  %t52 = load ptr, ptr %t5
  %t53 = call i32 @CompareStrings(ptr %t51, ptr %t52)
  %t54 = load ptr, ptr @glitch_exception_pending
  %t55 = icmp ne ptr %t54, null
  br i1 %t55, label %exception_unwind, label %call_continue_7
call_continue_7:
  %t56 = trunc i64 0 to i32
  %t57 = icmp sgt i32 %t53, %t56
  %t58 = and i1 %t38, %t57
  br i1 %t58, label %while_body_5, label %while_end_6
while_body_5:
  %t59 = load ptr, ptr %t2
  %t60 = load i32, ptr %t6
  %t61 = trunc i64 1 to i32
  %t62 = add i32 %t60, %t61
  %t63 = sext i32 %t62 to i64
  %t64 = getelementptr inbounds %glitch.list, ptr %t59, i32 0, i32 2
  %t65 = load ptr, ptr %t64
  %t66 = getelementptr inbounds ptr, ptr %t65, i64 %t63
  %t67 = load ptr, ptr %t2
  %t68 = load i32, ptr %t6
  %t69 = sext i32 %t68 to i64
  %t70 = getelementptr inbounds %glitch.list, ptr %t67, i32 0, i32 2
  %t71 = load ptr, ptr %t70
  %t72 = getelementptr inbounds ptr, ptr %t71, i64 %t69
  %t73 = load ptr, ptr %t72
  store ptr %t73, ptr %t66
  %t74 = load i32, ptr %t6
  %t75 = trunc i64 1 to i32
  %t76 = sub i32 %t74, %t75
  store i32 %t76, ptr %t6
  br label %while_cond_4
while_end_6:
  %t77 = load ptr, ptr %t2
  %t78 = load i32, ptr %t6
  %t79 = trunc i64 1 to i32
  %t80 = add i32 %t78, %t79
  %t81 = sext i32 %t80 to i64
  %t82 = getelementptr inbounds %glitch.list, ptr %t77, i32 0, i32 2
  %t83 = load ptr, ptr %t82
  %t84 = getelementptr inbounds ptr, ptr %t83, i64 %t81
  %t85 = load ptr, ptr %t4
  store ptr %t85, ptr %t84
  %t86 = load i32, ptr %t3
  %t87 = trunc i64 1 to i32
  %t88 = add i32 %t86, %t87
  store i32 %t88, ptr %t3
  br label %while_cond_1
while_end_3:
  %t89 = load ptr, ptr %t2
  %t90 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t90)
  ret ptr %t89
exception_unwind:
  %t91 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t91)
  %t92 = load ptr, ptr %t2
  %t93 = icmp eq ptr %t92, null
  br i1 %t93, label %collection_release_done_9, label %collection_release_8
collection_release_8:
  %t94 = getelementptr inbounds %glitch.list, ptr %t92, i32 0, i32 0
  %t95 = getelementptr inbounds %glitch.list, ptr %t92, i32 0, i32 2
  %t96 = load i64, ptr %t94
  %t97 = load ptr, ptr %t95
  call void @glitch_free(ptr %t97)
  call void @glitch_free(ptr %t92)
  br label %collection_release_done_9
collection_release_done_9:
  ret ptr null
}

define ptr @List__g1__t41_OrderByDescending__g0(ptr %this, ptr %keySelector) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %keySelector, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = alloca i32
  store i32 0, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = call ptr @List__g1__t41_ToList__g0(ptr %t7)
  %t9 = load ptr, ptr @glitch_exception_pending
  %t10 = icmp ne ptr %t9, null
  br i1 %t10, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t8, ptr %t2
  %t11 = trunc i64 1 to i32
  store i32 %t11, ptr %t3
  br label %while_cond_1
while_cond_1:
  %t12 = load i32, ptr %t3
  %t13 = load ptr, ptr %t2
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  %t15 = load i64, ptr %t14
  %t16 = trunc i64 %t15 to i32
  %t17 = icmp slt i32 %t12, %t16
  br i1 %t17, label %while_body_2, label %while_end_3
while_body_2:
  %t18 = load ptr, ptr %t2
  %t19 = load i32, ptr %t3
  %t20 = sext i32 %t19 to i64
  %t21 = getelementptr inbounds %glitch.list, ptr %t18, i32 0, i32 2
  %t22 = load ptr, ptr %t21
  %t23 = getelementptr inbounds ptr, ptr %t22, i64 %t20
  %t24 = load ptr, ptr %t23
  store ptr %t24, ptr %t4
  %t25 = load ptr, ptr %t1
  %t26 = getelementptr inbounds %glitch.delegate, ptr %t25, i32 0, i32 1
  %t27 = load ptr, ptr %t26
  %t28 = getelementptr inbounds %glitch.delegate, ptr %t25, i32 0, i32 2
  %t29 = load ptr, ptr %t28
  %t30 = load ptr, ptr %t4
  %t31 = call ptr %t27(ptr %t29, ptr %t30)
  %t32 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t32)
  store ptr %t31, ptr %t5
  %t33 = load i32, ptr %t3
  %t34 = trunc i64 1 to i32
  %t35 = sub i32 %t33, %t34
  store i32 %t35, ptr %t6
  br label %while_cond_4
while_cond_4:
  %t36 = load i32, ptr %t6
  %t37 = trunc i64 0 to i32
  %t38 = icmp sge i32 %t36, %t37
  %t39 = load ptr, ptr %t1
  %t40 = getelementptr inbounds %glitch.delegate, ptr %t39, i32 0, i32 1
  %t41 = load ptr, ptr %t40
  %t42 = getelementptr inbounds %glitch.delegate, ptr %t39, i32 0, i32 2
  %t43 = load ptr, ptr %t42
  %t44 = load ptr, ptr %t2
  %t45 = load i32, ptr %t6
  %t46 = sext i32 %t45 to i64
  %t47 = getelementptr inbounds %glitch.list, ptr %t44, i32 0, i32 2
  %t48 = load ptr, ptr %t47
  %t49 = getelementptr inbounds ptr, ptr %t48, i64 %t46
  %t50 = load ptr, ptr %t49
  %t51 = call ptr %t41(ptr %t43, ptr %t50)
  %t52 = load ptr, ptr %t5
  %t53 = call i32 @CompareStrings(ptr %t51, ptr %t52)
  %t54 = load ptr, ptr @glitch_exception_pending
  %t55 = icmp ne ptr %t54, null
  br i1 %t55, label %exception_unwind, label %call_continue_7
call_continue_7:
  %t56 = trunc i64 0 to i32
  %t57 = icmp slt i32 %t53, %t56
  %t58 = and i1 %t38, %t57
  br i1 %t58, label %while_body_5, label %while_end_6
while_body_5:
  %t59 = load ptr, ptr %t2
  %t60 = load i32, ptr %t6
  %t61 = trunc i64 1 to i32
  %t62 = add i32 %t60, %t61
  %t63 = sext i32 %t62 to i64
  %t64 = getelementptr inbounds %glitch.list, ptr %t59, i32 0, i32 2
  %t65 = load ptr, ptr %t64
  %t66 = getelementptr inbounds ptr, ptr %t65, i64 %t63
  %t67 = load ptr, ptr %t2
  %t68 = load i32, ptr %t6
  %t69 = sext i32 %t68 to i64
  %t70 = getelementptr inbounds %glitch.list, ptr %t67, i32 0, i32 2
  %t71 = load ptr, ptr %t70
  %t72 = getelementptr inbounds ptr, ptr %t71, i64 %t69
  %t73 = load ptr, ptr %t72
  store ptr %t73, ptr %t66
  %t74 = load i32, ptr %t6
  %t75 = trunc i64 1 to i32
  %t76 = sub i32 %t74, %t75
  store i32 %t76, ptr %t6
  br label %while_cond_4
while_end_6:
  %t77 = load ptr, ptr %t2
  %t78 = load i32, ptr %t6
  %t79 = trunc i64 1 to i32
  %t80 = add i32 %t78, %t79
  %t81 = sext i32 %t80 to i64
  %t82 = getelementptr inbounds %glitch.list, ptr %t77, i32 0, i32 2
  %t83 = load ptr, ptr %t82
  %t84 = getelementptr inbounds ptr, ptr %t83, i64 %t81
  %t85 = load ptr, ptr %t4
  store ptr %t85, ptr %t84
  %t86 = load i32, ptr %t3
  %t87 = trunc i64 1 to i32
  %t88 = add i32 %t86, %t87
  store i32 %t88, ptr %t3
  br label %while_cond_1
while_end_3:
  %t89 = load ptr, ptr %t2
  %t90 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t90)
  ret ptr %t89
exception_unwind:
  %t91 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t91)
  %t92 = load ptr, ptr %t2
  %t93 = icmp eq ptr %t92, null
  br i1 %t93, label %collection_release_done_9, label %collection_release_8
collection_release_8:
  %t94 = getelementptr inbounds %glitch.list, ptr %t92, i32 0, i32 0
  %t95 = getelementptr inbounds %glitch.list, ptr %t92, i32 0, i32 2
  %t96 = load i64, ptr %t94
  %t97 = load ptr, ptr %t95
  call void @glitch_free(ptr %t97)
  call void @glitch_free(ptr %t92)
  br label %collection_release_done_9
collection_release_done_9:
  ret ptr null
}

define void @List__g1__t41_Add__g0(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @List__g1__t41_Clear__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define i1 @List__g1__t41_Contains__g0(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define ptr @List__g1__t41_GetEnumerator__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.ListEnumerator_T___g0__t43, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_ListEnumerator_T___g0__t43, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t3, i32 0, i32 3
  store ptr %t6, ptr %t7
  %t8 = sub i64 0, 1
  %t9 = trunc i64 %t8 to i32
  %t10 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t3, i32 0, i32 4
  store i32 %t9, ptr %t10
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @Dictionary__g2__t42_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @Dictionary__g2__t42_Add__g0(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  ret void
exception_unwind:
  ret void
}

define i1 @Dictionary__g2__t42_ContainsKey__g0(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @Dictionary__g2__t42_Remove__g0(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @Dictionary__g2__t42_TryGetValue__g0(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  store ptr null, ptr %t2
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @Dictionary__g2__t42_Clear__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @Dictionary__g2__t42_GetEnumerator__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define i1 @ListEnumerator__g1__t43_MoveNext__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t1, i32 0, i32 4
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t3, i32 0, i32 4
  %t5 = load i32, ptr %t4
  %t6 = trunc i64 1 to i32
  %t7 = add i32 %t5, %t6
  store i32 %t7, ptr %t2
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t8, i32 0, i32 4
  %t10 = load i32, ptr %t9
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t11, i32 0, i32 3
  %t13 = load ptr, ptr %t12
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  %t15 = load i64, ptr %t14
  %t16 = trunc i64 %t15 to i32
  %t17 = icmp slt i32 %t10, %t16
  ret i1 %t17
exception_unwind:
  ret i1 0
}

define void @ListEnumerator__g1__t43_Reset__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t1, i32 0, i32 4
  %t3 = sub i64 0, 1
  %t4 = trunc i64 %t3 to i32
  store i32 %t4, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @ListEnumerator__g1__t43_Dispose__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @HashSet__g1__t44_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t1, i32 0, i32 3
  %t3 = call ptr @glitch_calloc(i64 1, i64 24)
  %t4 = call ptr @glitch_calloc(i64 4, i64 8)
  %t5 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 1
  store i64 4, ptr %t5
  %t6 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 2
  store ptr %t4, ptr %t6
  store ptr %t3, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @HashSet__g1__t44_ctor__ienumerable_T(ptr %this, ptr %values) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %values, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t3, i32 0, i32 3
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  store ptr %t5, ptr %t4
  %t9 = load ptr, ptr %t1
  %t10 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 0
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  %t12 = load i64, ptr %t10
  %t13 = load ptr, ptr %t11
  %t14 = alloca i64
  %t15 = alloca ptr
  store i64 0, ptr %t14
  br label %foreach_condition_0
foreach_condition_0:
  %t16 = load i64, ptr %t14
  %t17 = icmp ult i64 %t16, %t12
  br i1 %t17, label %foreach_body_1, label %foreach_end_3
foreach_body_1:
  %t18 = getelementptr inbounds ptr, ptr %t13, i64 %t16
  %t19 = load ptr, ptr %t18
  store ptr %t19, ptr %t15
  %t20 = load ptr, ptr %t0
  %t21 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t20, i32 0, i32 3
  %t22 = load ptr, ptr %t21
  %t23 = load ptr, ptr %t15
  %t24 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 0
  %t25 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 2
  %t26 = load i64, ptr %t24
  %t27 = load ptr, ptr %t25
  %t28 = alloca i64
  %t29 = alloca i1
  store i64 0, ptr %t28
  store i1 false, ptr %t29
  br label %list_contains_loop_4
list_contains_loop_4:
  %t30 = load i64, ptr %t28
  %t31 = icmp ult i64 %t30, %t26
  br i1 %t31, label %list_contains_next_6, label %list_contains_done_7
list_contains_next_6:
  %t32 = getelementptr inbounds ptr, ptr %t27, i64 %t30
  %t33 = load ptr, ptr %t32
  %t34 = icmp eq ptr %t33, %t23
  br i1 %t34, label %list_contains_found_5, label %list_contains_advance_8
list_contains_advance_8:
  %t35 = add i64 %t30, 1
  store i64 %t35, ptr %t28
  br label %list_contains_loop_4
list_contains_found_5:
  store i1 true, ptr %t29
  br label %list_contains_done_7
list_contains_done_7:
  %t36 = load i1, ptr %t29
  %t37 = xor i1 %t36, true
  br i1 %t37, label %if_then_9, label %if_else_10
if_then_9:
  %t38 = load ptr, ptr %t0
  %t39 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t38, i32 0, i32 3
  %t40 = load ptr, ptr %t39
  %t41 = load ptr, ptr %t15
  %t42 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 0
  %t43 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 1
  %t44 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 2
  %t45 = load i64, ptr %t42
  %t46 = load i64, ptr %t43
  %t47 = load ptr, ptr %t44
  %t48 = icmp eq i64 %t45, %t46
  br i1 %t48, label %list_grow_12, label %list_ready_13
list_grow_12:
  %t49 = mul i64 %t46, 2
  %t50 = mul i64 %t49, 8
  %t51 = call ptr @glitch_realloc(ptr %t47, i64 %t50)
  store i64 %t49, ptr %t43
  store ptr %t51, ptr %t44
  br label %list_ready_13
list_ready_13:
  %t52 = load ptr, ptr %t44
  %t53 = getelementptr inbounds ptr, ptr %t52, i64 %t45
  store ptr %t41, ptr %t53
  %t54 = add i64 %t45, 1
  store i64 %t54, ptr %t42
  br label %if_end_11
if_else_10:
  br label %if_end_11
if_end_11:
  br label %foreach_advance_2
foreach_advance_2:
  %t55 = load i64, ptr %t14
  %t56 = add i64 %t55, 1
  store i64 %t56, ptr %t14
  br label %foreach_condition_0
foreach_end_3:
  ret void
exception_unwind:
  ret void
}

define void @HashSet__g1__t44_Add__g0(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t2, i32 0, i32 3
  %t4 = load ptr, ptr %t3
  %t5 = load ptr, ptr %t1
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 0
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  %t8 = load i64, ptr %t6
  %t9 = load ptr, ptr %t7
  %t10 = alloca i64
  %t11 = alloca i1
  store i64 0, ptr %t10
  store i1 false, ptr %t11
  br label %list_contains_loop_0
list_contains_loop_0:
  %t12 = load i64, ptr %t10
  %t13 = icmp ult i64 %t12, %t8
  br i1 %t13, label %list_contains_next_2, label %list_contains_done_3
list_contains_next_2:
  %t14 = getelementptr inbounds ptr, ptr %t9, i64 %t12
  %t15 = load ptr, ptr %t14
  %t16 = icmp eq ptr %t15, %t5
  br i1 %t16, label %list_contains_found_1, label %list_contains_advance_4
list_contains_advance_4:
  %t17 = add i64 %t12, 1
  store i64 %t17, ptr %t10
  br label %list_contains_loop_0
list_contains_found_1:
  store i1 true, ptr %t11
  br label %list_contains_done_3
list_contains_done_3:
  %t18 = load i1, ptr %t11
  %t19 = xor i1 %t18, true
  br i1 %t19, label %if_then_5, label %if_else_6
if_then_5:
  %t20 = load ptr, ptr %t0
  %t21 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t20, i32 0, i32 3
  %t22 = load ptr, ptr %t21
  %t23 = load ptr, ptr %t1
  %t24 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 0
  %t25 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 1
  %t26 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 2
  %t27 = load i64, ptr %t24
  %t28 = load i64, ptr %t25
  %t29 = load ptr, ptr %t26
  %t30 = icmp eq i64 %t27, %t28
  br i1 %t30, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t31 = mul i64 %t28, 2
  %t32 = mul i64 %t31, 8
  %t33 = call ptr @glitch_realloc(ptr %t29, i64 %t32)
  store i64 %t31, ptr %t25
  store ptr %t33, ptr %t26
  br label %list_ready_9
list_ready_9:
  %t34 = load ptr, ptr %t26
  %t35 = getelementptr inbounds ptr, ptr %t34, i64 %t27
  store ptr %t23, ptr %t35
  %t36 = add i64 %t27, 1
  store i64 %t36, ptr %t24
  br label %if_end_7
if_else_6:
  br label %if_end_7
if_end_7:
  ret void
exception_unwind:
  ret void
}

define i1 @HashSet__g1__t44_Contains__g0(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t2, i32 0, i32 3
  %t4 = load ptr, ptr %t3
  %t5 = load ptr, ptr %t1
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 0
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  %t8 = load i64, ptr %t6
  %t9 = load ptr, ptr %t7
  %t10 = alloca i64
  %t11 = alloca i1
  store i64 0, ptr %t10
  store i1 false, ptr %t11
  br label %list_contains_loop_0
list_contains_loop_0:
  %t12 = load i64, ptr %t10
  %t13 = icmp ult i64 %t12, %t8
  br i1 %t13, label %list_contains_next_2, label %list_contains_done_3
list_contains_next_2:
  %t14 = getelementptr inbounds ptr, ptr %t9, i64 %t12
  %t15 = load ptr, ptr %t14
  %t16 = icmp eq ptr %t15, %t5
  br i1 %t16, label %list_contains_found_1, label %list_contains_advance_4
list_contains_advance_4:
  %t17 = add i64 %t12, 1
  store i64 %t17, ptr %t10
  br label %list_contains_loop_0
list_contains_found_1:
  store i1 true, ptr %t11
  br label %list_contains_done_3
list_contains_done_3:
  %t18 = load i1, ptr %t11
  ret i1 %t18
exception_unwind:
  ret i1 0
}

define void @HashSet__g1__t44_Clear__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t1, i32 0, i32 3
  %t3 = load ptr, ptr %t2
  %t4 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 0
  store i64 0, ptr %t4
  ret void
exception_unwind:
  ret void
}

define ptr @HashSet__g1__t44_GetEnumerator__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t1, i32 0, i32 3
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @List__g1__t41_GetEnumerator__g0(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_IEnumerator_T___g0__t33(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @ValidationResult__g0__t46_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %t1, i32 0, i32 2
  %t3 = call ptr @glitch_calloc(i64 1, i64 24)
  %t4 = call ptr @glitch_calloc(i64 4, i64 8)
  %t5 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 1
  store i64 4, ptr %t5
  %t6 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 2
  store ptr %t4, ptr %t6
  store ptr %t3, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @ValidationResult__g0__t46_ctor__list_ValidationFailure(ptr %this, ptr %errors) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %errors, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @ValidationException__g0__t47_ctor(ptr %this, ptr %errors) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %errors, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ValidationException__g0__t47, ptr %t2, i32 0, i32 3
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @ValidationContext__g1__t48_ctor(ptr %this, ptr %instanceToValidate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %instanceToValidate, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ValidationContext_T___g0__t48, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @RuleBuilder__g2__t50_NotNull__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_NotEmpty__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_SetValidator__g0(ptr %this, ptr %validator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %validator, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_EmailAddress__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_MinimumLength__g0(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_MaximumLength__g0(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Length__g0(ptr %this, i32 %min, i32 %max) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %min, ptr %t1
  %t2 = alloca i32
  store i32 %max, ptr %t2
  %t3 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Matches__g0(ptr %this, ptr %pattern) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %pattern, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Equal__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_NotEqual__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Must__g0(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_WithMessage__g0(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t51_RuleFor__g0(ptr %this, ptr %expression) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %expression, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_object___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_object___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t51_RuleForEach__g0(ptr %this, ptr %expression) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %expression, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_object___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_object___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t51_Validate__g0(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = getelementptr %glitch.ValidationResult__g0__t46, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_ValidationResult__g0__t46, ptr %t6
  call void @ValidationResult__g0__t46_ctor__empty(ptr %t4)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @make_owned__g1(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @make_shared__g1(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @make_borrow__g1(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @make_view__g1(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define i32 @CompareStrings(ptr %left, ptr %right) {
entry:
  %t0 = alloca ptr
  store ptr %left, ptr %t0
  %t1 = alloca ptr
  store ptr %right, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  %t5 = icmp eq ptr %t3, %t4
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  %t6 = trunc i64 0 to i32
  ret i32 %t6
if_else_1:
  br label %if_end_2
if_end_2:
  %t7 = load ptr, ptr %t0
  %t8 = icmp eq ptr %t7, null
  br i1 %t8, label %if_then_3, label %if_else_4
if_then_3:
  %t9 = sub i64 0, 1
  %t10 = trunc i64 %t9 to i32
  ret i32 %t10
if_else_4:
  br label %if_end_5
if_end_5:
  %t11 = load ptr, ptr %t1
  %t12 = icmp eq ptr %t11, null
  br i1 %t12, label %if_then_6, label %if_else_7
if_then_6:
  %t13 = trunc i64 1 to i32
  ret i32 %t13
if_else_7:
  br label %if_end_8
if_end_8:
  %t14 = trunc i64 0 to i32
  store i32 %t14, ptr %t2
  br label %while_cond_9
while_cond_9:
  %t15 = load i32, ptr %t2
  %t16 = load ptr, ptr %t0
  %t17 = call i64 @strlen(ptr %t16)
  %t18 = trunc i64 %t17 to i32
  %t19 = icmp slt i32 %t15, %t18
  %t20 = load i32, ptr %t2
  %t21 = load ptr, ptr %t1
  %t22 = call i64 @strlen(ptr %t21)
  %t23 = trunc i64 %t22 to i32
  %t24 = icmp slt i32 %t20, %t23
  %t25 = and i1 %t19, %t24
  br i1 %t25, label %while_body_10, label %while_end_11
while_body_10:
  %t26 = load ptr, ptr %t0
  %t27 = load i32, ptr %t2
  %t28 = sext i32 %t27 to i64
  %t29 = getelementptr inbounds i8, ptr %t26, i64 %t28
  %t30 = load i8, ptr %t29
  %t31 = load ptr, ptr %t1
  %t32 = load i32, ptr %t2
  %t33 = sext i32 %t32 to i64
  %t34 = getelementptr inbounds i8, ptr %t31, i64 %t33
  %t35 = load i8, ptr %t34
  %t36 = icmp slt i8 %t30, %t35
  br i1 %t36, label %if_then_12, label %if_else_13
if_then_12:
  %t37 = sub i64 0, 1
  %t38 = trunc i64 %t37 to i32
  ret i32 %t38
if_else_13:
  br label %if_end_14
if_end_14:
  %t39 = load ptr, ptr %t0
  %t40 = load i32, ptr %t2
  %t41 = sext i32 %t40 to i64
  %t42 = getelementptr inbounds i8, ptr %t39, i64 %t41
  %t43 = load i8, ptr %t42
  %t44 = load ptr, ptr %t1
  %t45 = load i32, ptr %t2
  %t46 = sext i32 %t45 to i64
  %t47 = getelementptr inbounds i8, ptr %t44, i64 %t46
  %t48 = load i8, ptr %t47
  %t49 = icmp sgt i8 %t43, %t48
  br i1 %t49, label %if_then_15, label %if_else_16
if_then_15:
  %t50 = trunc i64 1 to i32
  ret i32 %t50
if_else_16:
  br label %if_end_17
if_end_17:
  %t51 = load i32, ptr %t2
  %t52 = trunc i64 1 to i32
  %t53 = add i32 %t51, %t52
  store i32 %t53, ptr %t2
  br label %while_cond_9
while_end_11:
  %t54 = load ptr, ptr %t0
  %t55 = call i64 @strlen(ptr %t54)
  %t56 = trunc i64 %t55 to i32
  %t57 = load ptr, ptr %t1
  %t58 = call i64 @strlen(ptr %t57)
  %t59 = trunc i64 %t58 to i32
  %t60 = icmp slt i32 %t56, %t59
  br i1 %t60, label %if_then_18, label %if_else_19
if_then_18:
  %t61 = sub i64 0, 1
  %t62 = trunc i64 %t61 to i32
  ret i32 %t62
if_else_19:
  br label %if_end_20
if_end_20:
  %t63 = load ptr, ptr %t0
  %t64 = call i64 @strlen(ptr %t63)
  %t65 = trunc i64 %t64 to i32
  %t66 = load ptr, ptr %t1
  %t67 = call i64 @strlen(ptr %t66)
  %t68 = trunc i64 %t67 to i32
  %t69 = icmp sgt i32 %t65, %t68
  br i1 %t69, label %if_then_21, label %if_else_22
if_then_21:
  %t70 = trunc i64 1 to i32
  ret i32 %t70
if_else_22:
  br label %if_end_23
if_end_23:
  %t71 = trunc i64 0 to i32
  ret i32 %t71
exception_unwind:
  ret i32 0
}

define i32 @main() {
entry:
  %t0 = alloca ptr
  store ptr null, ptr %t0
  %t1 = alloca ptr
  store ptr null, ptr %t1
  %t2 = call ptr @glitch_calloc(i64 1, i64 24)
  %t3 = call ptr @glitch_calloc(i64 4, i64 8)
  %t4 = getelementptr inbounds %glitch.list, ptr %t2, i32 0, i32 1
  store i64 4, ptr %t4
  %t5 = getelementptr inbounds %glitch.list, ptr %t2, i32 0, i32 2
  store ptr %t3, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = icmp eq ptr %t6, null
  br i1 %t7, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t8 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 0
  %t9 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 2
  %t10 = load i64, ptr %t8
  %t11 = load ptr, ptr %t9
  %t12 = alloca i64
  store i64 0, ptr %t12
  br label %element_drop_loop_2
element_drop_loop_2:
  %t13 = load i64, ptr %t12
  %t14 = icmp ult i64 %t13, %t10
  br i1 %t14, label %element_drop_body_3, label %element_drop_done_4
element_drop_body_3:
  %t15 = getelementptr inbounds ptr, ptr %t11, i64 %t13
  %t16 = load ptr, ptr %t15
  call void @glitch_drop_ValidationFailure__g0__t45(ptr %t16)
  %t17 = add i64 %t13, 1
  store i64 %t17, ptr %t12
  br label %element_drop_loop_2
element_drop_done_4:
  call void @glitch_free(ptr %t11)
  call void @glitch_free(ptr %t6)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t2, ptr %t0
  %t18 = getelementptr %glitch.ValidationException__g0__t47, ptr null, i32 1
  %t19 = ptrtoint ptr %t18 to i64
  %t20 = call ptr @glitch_calloc(i64 1, i64 %t19)
  %t21 = getelementptr inbounds %glitch.ValidationException__g0__t47, ptr %t20, i32 0, i32 0
  store i64 1, ptr %t21
  %t22 = getelementptr inbounds %glitch.ValidationException__g0__t47, ptr %t20, i32 0, i32 1
  store ptr @glitch_destroy_ValidationException__g0__t47, ptr %t22
  %t23 = load ptr, ptr %t0
  call void @ValidationException__g0__t47_ctor(ptr %t20, ptr %t23)
  %t24 = load ptr, ptr @glitch_exception_pending
  %t25 = icmp ne ptr %t24, null
  br i1 %t25, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t26 = load ptr, ptr %t1
  call void @glitch_drop_ValidationException__g0__t47(ptr %t26)
  store ptr %t20, ptr %t1
  %t27 = load ptr, ptr %t1
  call void @glitch_drop_ValidationException__g0__t47(ptr %t27)
  %t28 = load ptr, ptr %t0
  %t29 = icmp eq ptr %t28, null
  br i1 %t29, label %collection_release_done_7, label %collection_release_6
collection_release_6:
  %t30 = getelementptr inbounds %glitch.list, ptr %t28, i32 0, i32 0
  %t31 = getelementptr inbounds %glitch.list, ptr %t28, i32 0, i32 2
  %t32 = load i64, ptr %t30
  %t33 = load ptr, ptr %t31
  %t34 = alloca i64
  store i64 0, ptr %t34
  br label %element_drop_loop_8
element_drop_loop_8:
  %t35 = load i64, ptr %t34
  %t36 = icmp ult i64 %t35, %t32
  br i1 %t36, label %element_drop_body_9, label %element_drop_done_10
element_drop_body_9:
  %t37 = getelementptr inbounds ptr, ptr %t33, i64 %t35
  %t38 = load ptr, ptr %t37
  call void @glitch_drop_ValidationFailure__g0__t45(ptr %t38)
  %t39 = add i64 %t35, 1
  store i64 %t39, ptr %t34
  br label %element_drop_loop_8
element_drop_done_10:
  call void @glitch_free(ptr %t33)
  call void @glitch_free(ptr %t28)
  br label %collection_release_done_7
collection_release_done_7:
  %t40 = call i64 @GlitchLiveAllocations_Load()
  %t41 = icmp ne i64 %t40, 0
  %t42 = load ptr, ptr @glitch_exception_pending
  %t43 = icmp ne ptr %t42, null
  %t44 = or i1 %t41, %t43
  %t45 = zext i1 %t44 to i32
  %t46 = call ptr @getenv(ptr @.env_report_leaks)
  %t47 = icmp ne ptr %t46, null
  br i1 %t47, label %report_leaks_11, label %main_return_12
report_leaks_11:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t40)
  br label %main_return_12
main_return_12:
  ret i32 %t45
exception_unwind:
  %t48 = load ptr, ptr %t1
  call void @glitch_drop_ValidationException__g0__t47(ptr %t48)
  %t49 = load ptr, ptr %t0
  %t50 = icmp eq ptr %t49, null
  br i1 %t50, label %collection_release_done_14, label %collection_release_13
collection_release_13:
  %t51 = getelementptr inbounds %glitch.list, ptr %t49, i32 0, i32 0
  %t52 = getelementptr inbounds %glitch.list, ptr %t49, i32 0, i32 2
  %t53 = load i64, ptr %t51
  %t54 = load ptr, ptr %t52
  %t55 = alloca i64
  store i64 0, ptr %t55
  br label %element_drop_loop_15
element_drop_loop_15:
  %t56 = load i64, ptr %t55
  %t57 = icmp ult i64 %t56, %t53
  br i1 %t57, label %element_drop_body_16, label %element_drop_done_17
element_drop_body_16:
  %t58 = getelementptr inbounds ptr, ptr %t54, i64 %t56
  %t59 = load ptr, ptr %t58
  call void @glitch_drop_ValidationFailure__g0__t45(ptr %t59)
  %t60 = add i64 %t56, 1
  store i64 %t60, ptr %t55
  br label %element_drop_loop_15
element_drop_done_17:
  call void @glitch_free(ptr %t54)
  call void @glitch_free(ptr %t49)
  br label %collection_release_done_14
collection_release_done_14:
  %t61 = call i64 @GlitchLiveAllocations_Load()
  %t62 = icmp ne i64 %t61, 0
  %t63 = load ptr, ptr @glitch_exception_pending
  %t64 = icmp ne ptr %t63, null
  %t65 = or i1 %t62, %t64
  %t66 = zext i1 %t65 to i32
  %t67 = call ptr @getenv(ptr @.env_report_leaks)
  %t68 = icmp ne ptr %t67, null
  br i1 %t68, label %report_leaks_18, label %main_return_19
report_leaks_18:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t61)
  br label %main_return_19
main_return_19:
  ret i32 %t66
}

define ptr @RuleBuilder__g2__t50_NotNull__g0__owner_T_object__g0__t14(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_NotEmpty__g0__owner_T_object__g0__t14(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_SetValidator__g0__owner_T_object__g0__t14(ptr %this, ptr %validator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %validator, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_EmailAddress__g0__owner_T_object__g0__t14(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_MinimumLength__g0__owner_T_object__g0__t14(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_MaximumLength__g0__owner_T_object__g0__t14(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Length__g0__owner_T_object__g0__t14(ptr %this, i32 %min, i32 %max) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %min, ptr %t1
  %t2 = alloca i32
  store i32 %max, ptr %t2
  %t3 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Matches__g0__owner_T_object__g0__t14(ptr %this, ptr %pattern) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %pattern, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Equal__g0__owner_T_object__g0__t14(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_NotEqual__g0__owner_T_object__g0__t14(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Must__g0__owner_T_object__g0__t14(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_WithMessage__g0__owner_T_object__g0__t14(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define i1 @IList__g1__t36_Contains__g0__owner_T(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @KeyValuePair__g2__t40_ctor__owner_K_V(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.KeyValuePair_K_V___g0__t40, ptr %t3, i32 0, i32 0
  %t5 = load ptr, ptr %t1
  store ptr %t5, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.KeyValuePair_K_V___g0__t40, ptr %t6, i32 0, i32 1
  %t8 = load ptr, ptr %t2
  store ptr %t8, ptr %t7
  ret void
exception_unwind:
  ret void
}

define void @IDictionary__g2__t38_Add__g0__owner_K_V(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  ret void
exception_unwind:
  ret void
}

define i1 @IDictionary__g2__t38_ContainsKey__g0__owner_K_V(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @IDictionary__g2__t38_Remove__g0__owner_K_V(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @IDictionary__g2__t38_TryGetValue__g0__owner_K_V(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @IDictionary__g2__t38_Clear__g0__owner_K_V(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @RuleBuilder__g2__t50_NotNull__g0__owner_T_TProperty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_NotEmpty__g0__owner_T_TProperty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_SetValidator__g0__owner_T_TProperty(ptr %this, ptr %validator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %validator, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_EmailAddress__g0__owner_T_TProperty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_MinimumLength__g0__owner_T_TProperty(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_MaximumLength__g0__owner_T_TProperty(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Length__g0__owner_T_TProperty(ptr %this, i32 %min, i32 %max) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %min, ptr %t1
  %t2 = alloca i32
  store i32 %max, ptr %t2
  %t3 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Matches__g0__owner_T_TProperty(ptr %this, ptr %pattern) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %pattern, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Equal__g0__owner_T_TProperty(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_NotEqual__g0__owner_T_TProperty(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_Must__g0__owner_T_TProperty(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t50_WithMessage__g0__owner_T_TProperty(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @HashSet__g1__t44_ctor__empty__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t1, i32 0, i32 3
  %t3 = call ptr @glitch_calloc(i64 1, i64 24)
  %t4 = call ptr @glitch_calloc(i64 4, i64 8)
  %t5 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 1
  store i64 4, ptr %t5
  %t6 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 2
  store ptr %t4, ptr %t6
  store ptr %t3, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @HashSet__g1__t44_ctor__ienumerable_T__owner_T(ptr %this, ptr %values) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %values, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t3, i32 0, i32 3
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  store ptr %t5, ptr %t4
  %t9 = load ptr, ptr %t1
  %t10 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 0
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  %t12 = load i64, ptr %t10
  %t13 = load ptr, ptr %t11
  %t14 = alloca i64
  %t15 = alloca ptr
  store i64 0, ptr %t14
  br label %foreach_condition_0
foreach_condition_0:
  %t16 = load i64, ptr %t14
  %t17 = icmp ult i64 %t16, %t12
  br i1 %t17, label %foreach_body_1, label %foreach_end_3
foreach_body_1:
  %t18 = getelementptr inbounds ptr, ptr %t13, i64 %t16
  %t19 = load ptr, ptr %t18
  store ptr %t19, ptr %t15
  %t20 = load ptr, ptr %t0
  %t21 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t20, i32 0, i32 3
  %t22 = load ptr, ptr %t21
  %t23 = load ptr, ptr %t15
  %t24 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 0
  %t25 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 2
  %t26 = load i64, ptr %t24
  %t27 = load ptr, ptr %t25
  %t28 = alloca i64
  %t29 = alloca i1
  store i64 0, ptr %t28
  store i1 false, ptr %t29
  br label %list_contains_loop_4
list_contains_loop_4:
  %t30 = load i64, ptr %t28
  %t31 = icmp ult i64 %t30, %t26
  br i1 %t31, label %list_contains_next_6, label %list_contains_done_7
list_contains_next_6:
  %t32 = getelementptr inbounds ptr, ptr %t27, i64 %t30
  %t33 = load ptr, ptr %t32
  %t34 = icmp eq ptr %t33, %t23
  br i1 %t34, label %list_contains_found_5, label %list_contains_advance_8
list_contains_advance_8:
  %t35 = add i64 %t30, 1
  store i64 %t35, ptr %t28
  br label %list_contains_loop_4
list_contains_found_5:
  store i1 true, ptr %t29
  br label %list_contains_done_7
list_contains_done_7:
  %t36 = load i1, ptr %t29
  %t37 = xor i1 %t36, true
  br i1 %t37, label %if_then_9, label %if_else_10
if_then_9:
  %t38 = load ptr, ptr %t0
  %t39 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t38, i32 0, i32 3
  %t40 = load ptr, ptr %t39
  %t41 = load ptr, ptr %t15
  %t42 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 0
  %t43 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 1
  %t44 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 2
  %t45 = load i64, ptr %t42
  %t46 = load i64, ptr %t43
  %t47 = load ptr, ptr %t44
  %t48 = icmp eq i64 %t45, %t46
  br i1 %t48, label %list_grow_12, label %list_ready_13
list_grow_12:
  %t49 = mul i64 %t46, 2
  %t50 = mul i64 %t49, 8
  %t51 = call ptr @glitch_realloc(ptr %t47, i64 %t50)
  store i64 %t49, ptr %t43
  store ptr %t51, ptr %t44
  br label %list_ready_13
list_ready_13:
  %t52 = load ptr, ptr %t44
  %t53 = getelementptr inbounds ptr, ptr %t52, i64 %t45
  store ptr %t41, ptr %t53
  %t54 = add i64 %t45, 1
  store i64 %t54, ptr %t42
  br label %if_end_11
if_else_10:
  br label %if_end_11
if_end_11:
  br label %foreach_advance_2
foreach_advance_2:
  %t55 = load i64, ptr %t14
  %t56 = add i64 %t55, 1
  store i64 %t56, ptr %t14
  br label %foreach_condition_0
foreach_end_3:
  ret void
exception_unwind:
  ret void
}

define void @HashSet__g1__t44_Add__g0__owner_T(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t2, i32 0, i32 3
  %t4 = load ptr, ptr %t3
  %t5 = load ptr, ptr %t1
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 0
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  %t8 = load i64, ptr %t6
  %t9 = load ptr, ptr %t7
  %t10 = alloca i64
  %t11 = alloca i1
  store i64 0, ptr %t10
  store i1 false, ptr %t11
  br label %list_contains_loop_0
list_contains_loop_0:
  %t12 = load i64, ptr %t10
  %t13 = icmp ult i64 %t12, %t8
  br i1 %t13, label %list_contains_next_2, label %list_contains_done_3
list_contains_next_2:
  %t14 = getelementptr inbounds ptr, ptr %t9, i64 %t12
  %t15 = load ptr, ptr %t14
  %t16 = icmp eq ptr %t15, %t5
  br i1 %t16, label %list_contains_found_1, label %list_contains_advance_4
list_contains_advance_4:
  %t17 = add i64 %t12, 1
  store i64 %t17, ptr %t10
  br label %list_contains_loop_0
list_contains_found_1:
  store i1 true, ptr %t11
  br label %list_contains_done_3
list_contains_done_3:
  %t18 = load i1, ptr %t11
  %t19 = xor i1 %t18, true
  br i1 %t19, label %if_then_5, label %if_else_6
if_then_5:
  %t20 = load ptr, ptr %t0
  %t21 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t20, i32 0, i32 3
  %t22 = load ptr, ptr %t21
  %t23 = load ptr, ptr %t1
  %t24 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 0
  %t25 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 1
  %t26 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 2
  %t27 = load i64, ptr %t24
  %t28 = load i64, ptr %t25
  %t29 = load ptr, ptr %t26
  %t30 = icmp eq i64 %t27, %t28
  br i1 %t30, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t31 = mul i64 %t28, 2
  %t32 = mul i64 %t31, 8
  %t33 = call ptr @glitch_realloc(ptr %t29, i64 %t32)
  store i64 %t31, ptr %t25
  store ptr %t33, ptr %t26
  br label %list_ready_9
list_ready_9:
  %t34 = load ptr, ptr %t26
  %t35 = getelementptr inbounds ptr, ptr %t34, i64 %t27
  store ptr %t23, ptr %t35
  %t36 = add i64 %t27, 1
  store i64 %t36, ptr %t24
  br label %if_end_7
if_else_6:
  br label %if_end_7
if_end_7:
  ret void
exception_unwind:
  ret void
}

define i1 @HashSet__g1__t44_Contains__g0__owner_T(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t2, i32 0, i32 3
  %t4 = load ptr, ptr %t3
  %t5 = load ptr, ptr %t1
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 0
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  %t8 = load i64, ptr %t6
  %t9 = load ptr, ptr %t7
  %t10 = alloca i64
  %t11 = alloca i1
  store i64 0, ptr %t10
  store i1 false, ptr %t11
  br label %list_contains_loop_0
list_contains_loop_0:
  %t12 = load i64, ptr %t10
  %t13 = icmp ult i64 %t12, %t8
  br i1 %t13, label %list_contains_next_2, label %list_contains_done_3
list_contains_next_2:
  %t14 = getelementptr inbounds ptr, ptr %t9, i64 %t12
  %t15 = load ptr, ptr %t14
  %t16 = icmp eq ptr %t15, %t5
  br i1 %t16, label %list_contains_found_1, label %list_contains_advance_4
list_contains_advance_4:
  %t17 = add i64 %t12, 1
  store i64 %t17, ptr %t10
  br label %list_contains_loop_0
list_contains_found_1:
  store i1 true, ptr %t11
  br label %list_contains_done_3
list_contains_done_3:
  %t18 = load i1, ptr %t11
  ret i1 %t18
exception_unwind:
  ret i1 0
}

define void @HashSet__g1__t44_Clear__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t1, i32 0, i32 3
  %t3 = load ptr, ptr %t2
  %t4 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 0
  store i64 0, ptr %t4
  ret void
exception_unwind:
  ret void
}

define ptr @HashSet__g1__t44_GetEnumerator__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %t1, i32 0, i32 3
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @List__g1__t41_GetEnumerator__g0(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_IEnumerator_T___g0__t33(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define i1 @ListEnumerator__g1__t43_MoveNext__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t1, i32 0, i32 4
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t3, i32 0, i32 4
  %t5 = load i32, ptr %t4
  %t6 = trunc i64 1 to i32
  %t7 = add i32 %t5, %t6
  store i32 %t7, ptr %t2
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t8, i32 0, i32 4
  %t10 = load i32, ptr %t9
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t11, i32 0, i32 3
  %t13 = load ptr, ptr %t12
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  %t15 = load i64, ptr %t14
  %t16 = trunc i64 %t15 to i32
  %t17 = icmp slt i32 %t10, %t16
  ret i1 %t17
exception_unwind:
  ret i1 0
}

define void @ListEnumerator__g1__t43_Reset__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %t1, i32 0, i32 4
  %t3 = sub i64 0, 1
  %t4 = trunc i64 %t3 to i32
  store i32 %t4, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @ListEnumerator__g1__t43_Dispose__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define i1 @IEnumerator__g1__t33_MoveNext__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @IEnumerator__g1__t33_Reset__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IEnumerator__g1__t33_Dispose__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define i1 @IReadOnlyDictionary__g2__t39_ContainsKey__g0__owner_K_V(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @IReadOnlyDictionary__g2__t39_TryGetValue__g0__owner_K_V(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  ret i1 0
exception_unwind:
  ret i1 0
}

define ptr @IValidator__g1__t49_Validate__g0__owner_T(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Rc__g1__t30_ctor__owner_T(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Rc__g1__t30_AddRef__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t1, i32 0, i32 3
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t3, i32 0, i32 3
  %t5 = load i32, ptr %t4
  %t6 = trunc i64 1 to i32
  %t7 = add i32 %t5, %t6
  store i32 %t7, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @Rc__g1__t30_Release__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t1, i32 0, i32 3
  %t3 = load i32, ptr %t2
  %t4 = sub i32 %t3, 1
  store i32 %t4, ptr %t2
  %t5 = trunc i64 0 to i32
  %t6 = icmp eq i32 %t4, %t5
  br i1 %t6, label %if_then_0, label %if_else_1
if_then_0:
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  ret void
exception_unwind:
  ret void
}

define ptr @Rc__g1__t30_Get__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Rc_T___g0__t30, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @Weak__g1__t31_ctor__owner_T(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret void
exception_unwind:
  ret void
}

define i1 @Weak__g1__t31_TryGetTarget__g0__owner_T(ptr %this, ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %target, ptr %t1
  store ptr null, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @ICollection__g1__t34_Add__g0__owner_T(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ICollection__g1__t34_Clear__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ValidationContext__g1__t48_ctor__owner_T(ptr %this, ptr %instanceToValidate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %instanceToValidate, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ValidationContext_T___g0__t48, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @AbstractValidator__g1__t51_RuleFor__g0__owner_T(ptr %this, ptr %expression) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %expression, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_object___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_object___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t51_RuleForEach__g0__owner_T(ptr %this, ptr %expression) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %expression, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_object___g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_object___g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t51_Validate__g0__owner_T(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = getelementptr %glitch.ValidationResult__g0__t46, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.ValidationResult__g0__t46, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_ValidationResult__g0__t46, ptr %t6
  call void @ValidationResult__g0__t46_ctor__empty(ptr %t4)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define i1 @IEnumerator__g1__t33_MoveNext__g0__owner_KeyValuePair__g2__t40(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @IEnumerator__g1__t33_Reset__g0__owner_KeyValuePair__g2__t40(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IEnumerator__g1__t33_Dispose__g0__owner_KeyValuePair__g2__t40(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define i1 @glitch_endpoint_handlers_contains(ptr %app, ptr %method, ptr %path) {
entry:
  ret i1 false
}

define ptr @glitch_endpoint_handlers_invoke(ptr %app, ptr %method, ptr %path, ptr %body) {
entry:
  ret ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.8, i32 0, i32 2, i64 0)
}

