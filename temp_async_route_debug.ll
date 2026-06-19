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
%glitch.CancellationToken__g0__t45 = type { i8 }
%glitch.Task__g0__t46 = type { i64, ptr, i1, i1, ptr }
%glitch.ValueTask__g0__t47 = type { i64, ptr, i1, i1, ptr }
%glitch.IDbContextTransaction__g0__t48 = type { i64, ptr }
%glitch.DbContextOptions__g0__t49 = type { i64, ptr, ptr }
%glitch.SqlProvider__g0__t50 = type { i64, ptr, ptr, ptr }
%glitch.DbSet__g1__t51 = type { ptr, i32 }
%glitch.DbQuery__g1__t52 = type { ptr, ptr, i32 }
%glitch.DbContextOptionsBuilder_ApplicationDbContext__g0__t53 = type { i64, ptr }
%glitch.ChangeTracker__g0__t54 = type { i64, ptr, ptr }
%glitch.DebugView__g0__t55 = type { i64, ptr, ptr }
%glitch.DatabaseFacade__g0__t56 = type { i64, ptr, ptr }
%glitch.DatabaseTransaction__g0__t57 = type { i64, ptr }
%glitch.EntityEntry__g0__t58 = type { i64, ptr, i32 }
%glitch.DbUpdateConcurrencyException__g0__t59 = type { i64, ptr, ptr }
%glitch.DbContext__g0__t60 = type { i64, ptr, ptr, i1, ptr, ptr, ptr, ptr }
%glitch.IQueryableString__g0__t61 = type { i64, ptr, ptr, ptr, i1 }
%glitch.DbSetString__g0__t62 = type { i64, ptr, ptr, ptr }
%glitch.ModelBuilder__g0__t63 = type { i64, ptr }
%glitch.MigrationBuilder__g0__t64 = type { i64, ptr }
%glitch.EntityTypeBuilder__g1__t65 = type { i64, ptr }
%glitch.PropertyBuilder__g0__t66 = type { i64, ptr }
%glitch.IndexBuilder__g0__t67 = type { i64, ptr }
%glitch.ILogger__g0__t68 = type { i64, ptr }
%glitch.ConsoleLogger__g0__t69 = type { i64, ptr }
%glitch.ILoggerFactory__g0__t70 = type { i64, ptr }
%glitch.ILoggerProvider__g0__t71 = type { i64, ptr }
%glitch.LoggerFactory__g0__t72 = type { i64, ptr }
%glitch.SymmetricSecurityKey__g0__t73 = type { i64, ptr, ptr }
%glitch.SigningCredentials__g0__t74 = type { i64, ptr, ptr, ptr }
%glitch.TokenValidationParameters__g0__t75 = type { i64, ptr, i1, i1, i1, i1, i1, ptr, ptr, ptr, ptr }
%glitch.SecurityAlgorithms__g0__t76 = type { i64, ptr }
%glitch.JwtBearerDefaults__g0__t77 = type { i64, ptr, ptr }
%glitch.JwtBearerOptions__g0__t78 = type { i64, ptr, ptr, ptr }
%glitch.JwtBearerEvents__g0__t79 = type { i64, ptr, ptr }
%glitch.IRequest__g0__t80 = type { i64, ptr }
%glitch.IRequestHandler__g1__t81 = type { i64, ptr }
%glitch.IPipelineBehavior__g2__t82 = type { i64, ptr }
%glitch.IMediator__g0__t83 = type { i64, ptr }
%glitch.Mediator__g0__t84 = type { i64, ptr, ptr }
%glitch.MediatRServiceConfiguration__g0__t85 = type { i64, ptr }
%glitch.IMapper__g0__t86 = type { i64, ptr }
%glitch.Mapper__g0__t87 = type { i64, ptr }
%glitch.IMemberConfigurationExpression__g0__t88 = type { i64, ptr }
%glitch.MemberConfigurationExpression__g0__t89 = type { i64, ptr }
%glitch.IMappingExpression__g2__t90 = type { i64, ptr }
%glitch.MappingExpression__g2__t91 = type { i64, ptr }
%glitch.Profile__g0__t92 = type { i64, ptr }
%glitch.MapperConfigurationExpression__g0__t93 = type { i64, ptr, ptr }
%glitch.MapperConfiguration__g0__t94 = type { i64, ptr, ptr }
%glitch.ValidationFailure__g0__t95 = type { i64, ptr, ptr }
%glitch.ValidationResult__g0__t96 = type { i64, ptr, ptr }
%glitch.ValidationException__g0__t97 = type { i64, ptr, ptr, ptr }
%glitch.ValidationContext__g1__t98 = type { i64, ptr, ptr }
%glitch.IValidator__g1__t99 = type { i64, ptr }
%glitch.RuleBuilder__g2__t100 = type { i64, ptr }
%glitch.AbstractValidator__g1__t101 = type { i64, ptr }
%glitch.JsonSerializerOptions__g0__t102 = type { i64, ptr, i32 }
%glitch.JsonSerializer__g0__t103 = type { i64, ptr }
%glitch.OpenApiReference__g0__t104 = type { i64, ptr, i32, ptr }
%glitch.OpenApiSecurityScheme__g0__t105 = type { i64, ptr, i32, ptr, ptr, i32, ptr, ptr }
%glitch.OpenApiSecurityRequirement__g0__t106 = type { i64, ptr }
%glitch.OpenApiInfo__g0__t107 = type { i64, ptr, ptr, ptr }
%glitch.IServiceProvider__g0__t108 = type { i64, ptr }
%glitch.DbContextOptionsBuilder__g0__t109 = type { i64, ptr, ptr }
%glitch.LocalizationOptions__g0__t110 = type { i64, ptr, ptr }
%glitch.CorsPolicyBuilder__g0__t111 = type { i64, ptr }
%glitch.MvcConventions__g0__t112 = type { i64, ptr }
%glitch.MvcFilterCollection__g0__t113 = type { i64, ptr }
%glitch.MvcJsonOptions__g0__t114 = type { i64, ptr, ptr }
%glitch.MvcOptions__g0__t115 = type { i64, ptr, ptr, ptr, i1 }
%glitch.AuthenticationBuilder__g0__t116 = type { i64, ptr }
%glitch.SwaggerActionDescriptor__g0__t117 = type { i64, ptr, ptr }
%glitch.SwaggerGenOptions__g0__t118 = type { i64, ptr }
%glitch.ServiceProvider__g0__t119 = type { i64, ptr, ptr }
%glitch.IServiceScope__g0__t120 = type { i64, ptr, ptr }
%glitch.IServiceScopeFactory__g0__t121 = type { i64, ptr }
%glitch.ServiceScope__g0__t122 = type { i64, ptr, ptr }
%glitch.MvcBuilder__g0__t123 = type { i64, ptr }
%glitch.IServiceCollection__g0__t124 = type { i64, ptr, ptr, ptr }
%glitch.ServiceCollection__g0__t125 = type { i64, ptr, ptr, ptr }
%glitch.ConfigurationManager__g0__t126 = type { i64, ptr }
%glitch.HostEnvironment__g0__t127 = type { i64, ptr }
%glitch.LoggingBuilder__g0__t128 = type { i64, ptr }
%glitch.WebApplicationBuilder__g0__t129 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.HttpRequest__g0__t130 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.HttpResponse__g0__t131 = type { i64, ptr, i32, ptr, ptr }
%glitch.IPAddress__g0__t132 = type { i64, ptr, ptr }
%glitch.ConnectionInfo__g0__t133 = type { i64, ptr, ptr }
%glitch.HttpContext__g0__t134 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.IHttpContextAccessor__g0__t135 = type { i64, ptr, ptr }
%glitch.HttpContextAccessor__g0__t136 = type { i64, ptr, ptr }
%glitch.SwaggerOptions__g0__t137 = type { i64, ptr, ptr }
%glitch.SwaggerUiOptions__g0__t138 = type { i64, ptr }
%glitch.CorsMiddlewareOptions__g0__t139 = type { i64, ptr }
%glitch.Endpoint__g0__t140 = type { i64, ptr, ptr, ptr, ptr }
%glitch.WebApplication__g0__t141 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.StaticFileOptions__g0__t142 = type { i64, ptr, ptr, ptr }
%glitch.PhysicalFileProvider__g0__t143 = type { i64, ptr }
%glitch.ApiVersion__g0__t144 = type { i64, ptr }
%glitch.ProblemDetails__g0__t145 = type { i64, ptr, ptr, i32, ptr, ptr, ptr }
%glitch.ObjectResult__g0__t146 = type { i64, ptr, i32 }
%glitch.IFormFile__g0__t147 = type { i64, ptr, ptr, i64 }
%glitch.ActionExecutingContext__g0__t148 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.ExceptionContext__g0__t149 = type { i64, ptr, ptr, ptr, ptr, i1 }
%glitch.ModelError__g0__t150 = type { i64, ptr, ptr }
%glitch.ModelStateEntry__g0__t151 = type { i64, ptr, ptr }
%glitch.ModelStateDictionary__g0__t152 = type { i64, ptr, i1, ptr }
%glitch.ControllerBase__g0__t153 = type { i64, ptr, ptr, ptr }
%glitch.MappingExpression_TSource_TDestination___g0__t91 = type { i64, ptr }
%glitch.IEnumerator_T___g0__t33 = type { i64, ptr, ptr }
%glitch.IEnumerator_KeyValuePair_K_V____g0__t33 = type { i64, ptr, ptr }
%glitch.EntityTypeBuilder_object___g0__t65 = type { i64, ptr }
%glitch.IMappingExpression_TSource_TDestination___g0__t90 = type { i64, ptr }
%glitch.ListEnumerator_T___g0__t43 = type { i64, ptr, ptr, ptr, i32 }
%glitch.Rc_T___g0__t30 = type { i64, ptr, ptr, i32 }
%glitch.DbSet_T___g0__t51 = type { ptr, i32 }
%glitch.IRequestHandler_TRequest___g0__t81 = type { i64, ptr }
%glitch.IValidator_T___g0__t99 = type { i64, ptr }
%glitch.KeyValuePair_K_V___g0__t40 = type { ptr, ptr }
%glitch.HashSet_T___g0__t44 = type { i64, ptr, i32, ptr }
%glitch.IDictionary_K_V___g0__t38 = type { i64, ptr, i32 }
%glitch.ICollection_T___g0__t34 = type { i64, ptr, i32 }
%glitch.DbQuery_T___g0__t52 = type { ptr, ptr, i32 }
%glitch.Weak_T___g0__t31 = type { i64, ptr, ptr }
%glitch.RuleBuilder_T_TProperty___g0__t100 = type { i64, ptr }
%glitch.ValidationContext_T___g0__t98 = type { i64, ptr, ptr }
%glitch.IReadOnlyDictionary_K_V___g0__t39 = type { i64, ptr, i32 }
%glitch.AbstractValidator_T___g0__t101 = type { i64, ptr }
%glitch.EntityTypeBuilder_T___g0__t65 = type { i64, ptr }
%glitch.RuleBuilder_T_object___g0__t100 = type { i64, ptr }
%glitch.IList_T___g0__t36 = type { i64, ptr, i32, i32 }
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
%glitch_async_state_HealthAsync = type { i32, ptr }
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
declare void @GlitchTask_RunI64(ptr, ptr)
declare void @GlitchTask_RunDouble(ptr, ptr)
declare void @GlitchTask_RunPtr(ptr, ptr)
declare void @GlitchTask_Wait(ptr)
declare i1 @GlitchTask_IsCompleted(ptr)
declare void @GlitchTask_Destroy(ptr)
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
@.str.0 = private unnamed_addr global { i64, i64, [9 x i8] } { i64 1000000000, i64 8, [9 x i8] c"Assembly\00" }
@.str.1 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.2 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.3 = private unnamed_addr global { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"true\00" }
@.str.4 = private unnamed_addr global { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"false\00" }
@.str.5 = private unnamed_addr global { i64, i64, [42 x i8] } { i64 1000000000, i64 41, [42 x i8] c"Input string was not in a correct format.\00" }
@.str.6 = private unnamed_addr global { i64, i64, [30 x i8] } { i64 1000000000, i64 29, [30 x i8] c"Sequence contains no elements\00" }
@.str.7 = private unnamed_addr global { i64, i64, [39 x i8] } { i64 1000000000, i64 38, [39 x i8] c"Sequence contains no matching elements\00" }
@.str.8 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.9 = private unnamed_addr global { i64, i64, [15 x i8] } { i64 1000000000, i64 14, [15 x i8] c"select * from \00" }
@.str.10 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.11 = private unnamed_addr global { i64, i64, [9 x i8] } { i64 1000000000, i64 8, [9 x i8] c"inmemory\00" }
@.str.12 = private unnamed_addr global { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"sql\00" }
@.str.13 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.14 = private unnamed_addr global { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"sql\00" }
@.str.15 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.16 = private unnamed_addr global { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"sql\00" }
@.str.17 = private unnamed_addr global { i64, i64, [22 x i8] } { i64 1000000000, i64 21, [22 x i8] c"DbContext is disposed\00" }
@.str.18 = private unnamed_addr global { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"sql\00" }
@.str.19 = private unnamed_addr global { i64, i64, [9 x i8] } { i64 1000000000, i64 8, [9 x i8] c"tracked:\00" }
@.str.20 = private unnamed_addr global { i64, i64, [9 x i8] } { i64 1000000000, i64 8, [9 x i8] c"[Error] \00" }
@.str.21 = private unnamed_addr global { i64, i64, [11 x i8] } { i64 1000000000, i64 10, [11 x i8] c"[Warning] \00" }
@.str.22 = private unnamed_addr global { i64, i64, [8 x i8] } { i64 1000000000, i64 7, [8 x i8] c"[Info] \00" }
@.str.23 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.24 = private unnamed_addr global { i64, i64, [7 x i8] } { i64 1000000000, i64 6, [7 x i8] c"Bearer\00" }
@.str.25 = private unnamed_addr global { i64, i64, [10 x i8] } { i64 1000000000, i64 9, [10 x i8] c"inmemory:\00" }
@.str.26 = private unnamed_addr global { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c"T\00" }
@.str.27 = private unnamed_addr global { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c"T\00" }
@.str.28 = private unnamed_addr global { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c"T\00" }
@.str.29 = private unnamed_addr global { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c"T\00" }
@.str.30 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.31 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.32 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.33 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.34 = private unnamed_addr global { i64, i64, [10 x i8] } { i64 1000000000, i64 9, [10 x i8] c"127.0.0.1\00" }
@.str.35 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.36 = private unnamed_addr global { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"trace\00" }
@.str.37 = private unnamed_addr global { i64, i64, [13 x i8] } { i64 1000000000, i64 12, [13 x i8] c"Content-Type\00" }
@.str.38 = private unnamed_addr global { i64, i64, [17 x i8] } { i64 1000000000, i64 16, [17 x i8] c"application/json\00" }
@.str.39 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.40 = private unnamed_addr global { i64, i64, [8 x i8] } { i64 1000000000, i64 7, [8 x i8] c"routing\00" }
@.str.41 = private unnamed_addr global { i64, i64, [10 x i8] } { i64 1000000000, i64 9, [10 x i8] c"endpoints\00" }
@.str.42 = private unnamed_addr global { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"trace\00" }
@.str.43 = private unnamed_addr global { i64, i64, [14 x i8] } { i64 1000000000, i64 13, [14 x i8] c"json-envelope\00" }
@.str.44 = private unnamed_addr global { i64, i64, [18 x i8] } { i64 1000000000, i64 17, [18 x i8] c"https-redirection\00" }
@.str.45 = private unnamed_addr global { i64, i64, [14 x i8] } { i64 1000000000, i64 13, [14 x i8] c"authorization\00" }
@.str.46 = private unnamed_addr global { i64, i64, [15 x i8] } { i64 1000000000, i64 14, [15 x i8] c"authentication\00" }
@.str.47 = private unnamed_addr global { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"cors\00" }
@.str.48 = private unnamed_addr global { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"cors\00" }
@.str.49 = private unnamed_addr global { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"cors\00" }
@.str.50 = private unnamed_addr global { i64, i64, [12 x i8] } { i64 1000000000, i64 11, [12 x i8] c"controllers\00" }
@.str.51 = private unnamed_addr global { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"GET \00" }
@.str.52 = private unnamed_addr global { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"GET \00" }
@.str.53 = private unnamed_addr global { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"POST \00" }
@.str.54 = private unnamed_addr global { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"POST \00" }
@.str.55 = private unnamed_addr global { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c" \00" }
@.str.56 = private unnamed_addr global { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"404\00" }
@.str.57 = private unnamed_addr global { i64, i64, [14 x i8] } { i64 1000000000, i64 13, [14 x i8] c"json-envelope\00" }
@.str.58 = private unnamed_addr global { i64, i64, [11 x i8] } { i64 1000000000, i64 10, [11 x i8] c"{\22result\22:\00" }
@.str.59 = private unnamed_addr global { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c"}\00" }
@.str.60 = private unnamed_addr global { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"trace\00" }
@.str.61 = private unnamed_addr global { i64, i64, [7 x i8] } { i64 1000000000, i64 6, [7 x i8] c"trace:\00" }
@.str.62 = private unnamed_addr global { i64, i64, [25 x i8] } { i64 1000000000, i64 24, [25 x i8] c"GLITCH_HTTP_MAX_REQUESTS\00" }
@.str.63 = private unnamed_addr global { i64, i64, [17 x i8] } { i64 1000000000, i64 16, [17 x i8] c"GLITCH_HTTP_PORT\00" }
@.str.64 = private unnamed_addr global { i64, i64, [25 x i8] } { i64 1000000000, i64 24, [25 x i8] c"GLITCH_HTTP_MAX_REQUESTS\00" }
@.str.65 = private unnamed_addr global { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.66 = private unnamed_addr global { i64, i64, [10 x i8] } { i64 1000000000, i64 9, [10 x i8] c"Not Found\00" }
@.str.67 = private unnamed_addr global { i64, i64, [16 x i8] } { i64 1000000000, i64 15, [16 x i8] c"{\22status\22:\22ok\22}\00" }
define ptr @glitch_delegate_wrapper_BuildHealth_0(ptr %env) {
entry:
  %t_wrap_2 = call ptr @BuildHealth()
  ret ptr %t_wrap_2
}

define ptr @glitch_async_resume_HealthAsync(ptr %env) {
entry:
  %t0 = getelementptr inbounds %glitch_async_state_HealthAsync, ptr %env, i32 0, i32 0
  %t1 = alloca ptr
  store ptr null, ptr %t1
  %t4 = getelementptr %glitch.delegate, ptr null, i32 1
  %t5 = ptrtoint ptr %t4 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t5)
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 1
  store ptr @glitch_delegate_wrapper_BuildHealth_0, ptr %t7
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 2
  store ptr null, ptr %t8
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 3
  store ptr null, ptr %t9
  call void @glitch_delegate_retain(ptr %t3)
  %t10 = call ptr @glitch_task_run_ptr(ptr %t3)
  call void @glitch_delegate_release(ptr %t3)
  %t11 = load ptr, ptr %t1
  %t12 = icmp eq ptr %t11, null
  br i1 %t12, label %task_release_done_1, label %task_release_0
task_release_0:
  call void @glitch_task_wait(ptr %t11)
  %t13 = getelementptr inbounds %glitch.task, ptr %t11, i32 0, i32 1
  %t14 = load ptr, ptr %t13
  call void @glitch_string_release(ptr %t14)
  call void @GlitchTask_Destroy(ptr %t11)
  call void @glitch_free(ptr %t11)
  br label %task_release_done_1
task_release_done_1:
  store ptr %t10, ptr %t1
  %t15 = load ptr, ptr %t1
  store i32 1, ptr %t0
  %t16 = call ptr @glitch_task_get_result_ptr(ptr %t15)
  call void @glitch_string_retain(ptr %t16)
  %t17 = load ptr, ptr %t1
  %t18 = icmp eq ptr %t17, null
  br i1 %t18, label %task_release_done_3, label %task_release_2
task_release_2:
  call void @glitch_task_wait(ptr %t17)
  %t19 = getelementptr inbounds %glitch.task, ptr %t17, i32 0, i32 1
  %t20 = load ptr, ptr %t19
  call void @glitch_string_release(ptr %t20)
  call void @GlitchTask_Destroy(ptr %t17)
  call void @glitch_free(ptr %t17)
  br label %task_release_done_3
task_release_done_3:
  ret ptr %t16
exception_unwind:
  %t21 = load ptr, ptr %t1
  %t22 = icmp eq ptr %t21, null
  br i1 %t22, label %task_release_done_5, label %task_release_4
task_release_4:
  call void @glitch_task_wait(ptr %t21)
  %t23 = getelementptr inbounds %glitch.task, ptr %t21, i32 0, i32 1
  %t24 = load ptr, ptr %t23
  call void @glitch_string_release(ptr %t24)
  call void @GlitchTask_Destroy(ptr %t21)
  call void @glitch_free(ptr %t21)
  br label %task_release_done_5
task_release_done_5:
  ret ptr null
}

define void @glitch_async_destroy_HealthAsync(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.68 = private unnamed_addr global { i64, i64, [8 x i8] } { i64 1000000000, i64 7, [8 x i8] c"/health\00" }
define ptr @glitch_delegate_wrapper_HealthAsync_1(ptr %env) {
entry:
  %t_wrap_10 = call ptr @HealthAsync()
  ret ptr %t_wrap_10
}

@.str.69 = private unnamed_addr global { i64, i64, [20 x i8] } { i64 1000000000, i64 19, [20 x i8] c"501 Not Implemented\00" }
@.str.70 = private unnamed_addr global { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"GET\00" }
@.str.71 = private unnamed_addr global { i64, i64, [8 x i8] } { i64 1000000000, i64 7, [8 x i8] c"/health\00" }
@.str.72 = private unnamed_addr global { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"404\00" }

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

define void @glitch_destroy_JwtBearerEvents__g0__t79(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.JwtBearerEvents__g0__t79, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_object__g0__t14(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_JwtBearerEvents__g0__t79(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.JwtBearerEvents__g0__t79, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_JwtBearerEvents__g0__t79(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.JwtBearerEvents__g0__t79, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.JwtBearerEvents__g0__t79, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_JwtBearerEvents__g0__t79(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_LoggingBuilder__g0__t128(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_LoggingBuilder__g0__t128(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.LoggingBuilder__g0__t128, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_LoggingBuilder__g0__t128(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.LoggingBuilder__g0__t128, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.LoggingBuilder__g0__t128, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_LoggingBuilder__g0__t128(ptr %object)
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

define void @glitch_destroy_IPAddress__g0__t132(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IPAddress__g0__t132, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IPAddress__g0__t132(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IPAddress__g0__t132, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IPAddress__g0__t132(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IPAddress__g0__t132, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IPAddress__g0__t132, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IPAddress__g0__t132(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_SwaggerOptions__g0__t137(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.SwaggerOptions__g0__t137, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SwaggerOptions__g0__t137(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SwaggerOptions__g0__t137, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SwaggerOptions__g0__t137(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SwaggerOptions__g0__t137, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SwaggerOptions__g0__t137, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SwaggerOptions__g0__t137(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_StaticFileOptions__g0__t142(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.StaticFileOptions__g0__t142, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_object__g0__t14(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.StaticFileOptions__g0__t142, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_StaticFileOptions__g0__t142(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.StaticFileOptions__g0__t142, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_StaticFileOptions__g0__t142(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.StaticFileOptions__g0__t142, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.StaticFileOptions__g0__t142, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_StaticFileOptions__g0__t142(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_EntityTypeBuilder_object___g0__t65(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_EntityTypeBuilder_object___g0__t65(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.EntityTypeBuilder_object___g0__t65, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_EntityTypeBuilder_object___g0__t65(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.EntityTypeBuilder_object___g0__t65, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.EntityTypeBuilder_object___g0__t65, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_EntityTypeBuilder_object___g0__t65(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IValidator_T___g0__t99(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IValidator_T___g0__t99(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IValidator_T___g0__t99, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IValidator_T___g0__t99(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IValidator_T___g0__t99, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IValidator_T___g0__t99, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IValidator_T___g0__t99(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HashSet_T___g0__t44(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.HashSet_T___g0__t44, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t0 = icmp eq ptr %field_3, null
  br i1 %t0, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t1 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 0
  %t2 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 2
  %t3 = load i64, ptr %t1
  %t4 = load ptr, ptr %t2
  call void @glitch_free(ptr %t4)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_1
collection_release_done_1:
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

define void @glitch_destroy_SqlProvider__g0__t50(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SqlProvider__g0__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SqlProvider__g0__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SqlProvider__g0__t50(ptr %object)
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

define void @glitch_destroy_HttpContext__g0__t134(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_HttpRequest__g0__t130(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_HttpResponse__g0__t131(ptr %field_3)
  %field_4_ptr = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_ConnectionInfo__g0__t133(ptr %field_4)
  %field_5_ptr = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HttpContext__g0__t134(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HttpContext__g0__t134(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HttpContext__g0__t134(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MvcFilterCollection__g0__t113(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MvcFilterCollection__g0__t113(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MvcFilterCollection__g0__t113, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MvcFilterCollection__g0__t113(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MvcFilterCollection__g0__t113, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MvcFilterCollection__g0__t113, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MvcFilterCollection__g0__t113(ptr %object)
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

define void @glitch_destroy_JwtBearerDefaults__g0__t77(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t77, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_JwtBearerDefaults__g0__t77(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t77, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_JwtBearerDefaults__g0__t77(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t77, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t77, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_JwtBearerDefaults__g0__t77(ptr %object)
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

define void @glitch_destroy_Mediator__g0__t84(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Mediator__g0__t84, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_IServiceProvider__g0__t108(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Mediator__g0__t84(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Mediator__g0__t84, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Mediator__g0__t84(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Mediator__g0__t84, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Mediator__g0__t84, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Mediator__g0__t84(ptr %object)
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

define void @glitch_destroy_LocalizationOptions__g0__t110(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.LocalizationOptions__g0__t110, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_LocalizationOptions__g0__t110(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.LocalizationOptions__g0__t110, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_LocalizationOptions__g0__t110(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.LocalizationOptions__g0__t110, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.LocalizationOptions__g0__t110, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_LocalizationOptions__g0__t110(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbContextOptionsBuilder_ApplicationDbContext__g0__t53(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbContextOptionsBuilder_ApplicationDbContext__g0__t53(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder_ApplicationDbContext__g0__t53, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbContextOptionsBuilder_ApplicationDbContext__g0__t53(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder_ApplicationDbContext__g0__t53, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder_ApplicationDbContext__g0__t53, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbContextOptionsBuilder_ApplicationDbContext__g0__t53(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ModelBuilder__g0__t63(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ModelBuilder__g0__t63(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ModelBuilder__g0__t63, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ModelBuilder__g0__t63(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ModelBuilder__g0__t63, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ModelBuilder__g0__t63, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ModelBuilder__g0__t63(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Task__g0__t46(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Task__g0__t46(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Task__g0__t46, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Task__g0__t46(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Task__g0__t46, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Task__g0__t46, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Task__g0__t46(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ExceptionContext__g0__t149(ptr %object) {
entry:
  %field_4_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t149, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_object__g0__t14(ptr %field_4)
  %field_3_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t149, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_HttpContext__g0__t134(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t149, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_object__g0__t14(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ExceptionContext__g0__t149(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t149, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ExceptionContext__g0__t149(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t149, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t149, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ExceptionContext__g0__t149(ptr %object)
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

define void @glitch_destroy_OpenApiSecurityScheme__g0__t105(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.OpenApiSecurityScheme__g0__t105, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_7_ptr = getelementptr inbounds %glitch.OpenApiSecurityScheme__g0__t105, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_OpenApiReference__g0__t104(ptr %field_7)
  %field_4_ptr = getelementptr inbounds %glitch.OpenApiSecurityScheme__g0__t105, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_6_ptr = getelementptr inbounds %glitch.OpenApiSecurityScheme__g0__t105, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_string_release(ptr %field_6)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_OpenApiSecurityScheme__g0__t105(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.OpenApiSecurityScheme__g0__t105, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_OpenApiSecurityScheme__g0__t105(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.OpenApiSecurityScheme__g0__t105, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.OpenApiSecurityScheme__g0__t105, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_OpenApiSecurityScheme__g0__t105(ptr %object)
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

define void @glitch_destroy_DatabaseTransaction__g0__t57(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DatabaseTransaction__g0__t57(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DatabaseTransaction__g0__t57, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DatabaseTransaction__g0__t57(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DatabaseTransaction__g0__t57, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DatabaseTransaction__g0__t57, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DatabaseTransaction__g0__t57(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IServiceScopeFactory__g0__t121(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IServiceScopeFactory__g0__t121(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IServiceScopeFactory__g0__t121, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IServiceScopeFactory__g0__t121(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IServiceScopeFactory__g0__t121, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IServiceScopeFactory__g0__t121, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IServiceScopeFactory__g0__t121(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ConstructorInfo__g0__t5(ptr %object) {
entry:
  %field_4_ptr = getelementptr inbounds %glitch.ConstructorInfo__g0__t5, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t5 = icmp eq ptr %field_4, null
  br i1 %t5, label %array_release_done_3, label %array_release_2
array_release_2:
  %t6 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 0
  %t8 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 1
  %t7 = load i64, ptr %t6
  %t9 = load ptr, ptr %t8
  %t10 = alloca i64
  store i64 0, ptr %t10
  br label %element_drop_loop_4
element_drop_loop_4:
  %t11 = load i64, ptr %t10
  %t12 = icmp ult i64 %t11, %t7
  br i1 %t12, label %element_drop_body_5, label %element_drop_done_6
element_drop_body_5:
  %t13 = getelementptr inbounds ptr, ptr %t9, i64 %t11
  %t14 = load ptr, ptr %t13
  call void @glitch_drop_ParameterInfo__g0__t6(ptr %t14)
  %t15 = add i64 %t11, 1
  store i64 %t15, ptr %t10
  br label %element_drop_loop_4
element_drop_done_6:
  call void @glitch_free(ptr %t9)
  call void @glitch_free(ptr %field_4)
  br label %array_release_done_3
array_release_done_3:
  %field_2_ptr = getelementptr inbounds %glitch.ConstructorInfo__g0__t5, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_6_ptr = getelementptr inbounds %glitch.ConstructorInfo__g0__t5, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  %t16 = icmp eq ptr %field_6, null
  br i1 %t16, label %array_release_done_8, label %array_release_7
array_release_7:
  %t17 = getelementptr inbounds %glitch.array, ptr %field_6, i32 0, i32 0
  %t19 = getelementptr inbounds %glitch.array, ptr %field_6, i32 0, i32 1
  %t18 = load i64, ptr %t17
  %t20 = load ptr, ptr %t19
  call void @glitch_free(ptr %t20)
  call void @glitch_free(ptr %field_6)
  br label %array_release_done_8
array_release_done_8:
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

define void @glitch_destroy_SwaggerUiOptions__g0__t138(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SwaggerUiOptions__g0__t138(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SwaggerUiOptions__g0__t138, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SwaggerUiOptions__g0__t138(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SwaggerUiOptions__g0__t138, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SwaggerUiOptions__g0__t138, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SwaggerUiOptions__g0__t138(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ApiVersion__g0__t144(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ApiVersion__g0__t144(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ApiVersion__g0__t144, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ApiVersion__g0__t144(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ApiVersion__g0__t144, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ApiVersion__g0__t144, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ApiVersion__g0__t144(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IFormFile__g0__t147(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IFormFile__g0__t147, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IFormFile__g0__t147(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IFormFile__g0__t147, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IFormFile__g0__t147(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IFormFile__g0__t147, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IFormFile__g0__t147, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IFormFile__g0__t147(ptr %object)
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

define void @glitch_destroy_JsonSerializerOptions__g0__t102(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_JsonSerializerOptions__g0__t102(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.JsonSerializerOptions__g0__t102, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_JsonSerializerOptions__g0__t102(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.JsonSerializerOptions__g0__t102, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.JsonSerializerOptions__g0__t102, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_JsonSerializerOptions__g0__t102(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MvcOptions__g0__t115(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.MvcOptions__g0__t115, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_MvcFilterCollection__g0__t113(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.MvcOptions__g0__t115, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_MvcConventions__g0__t112(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MvcOptions__g0__t115(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MvcOptions__g0__t115, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MvcOptions__g0__t115(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MvcOptions__g0__t115, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MvcOptions__g0__t115, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MvcOptions__g0__t115(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ServiceScope__g0__t122(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ServiceScope__g0__t122, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_IServiceProvider__g0__t108(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ServiceScope__g0__t122(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ServiceScope__g0__t122, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ServiceScope__g0__t122(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ServiceScope__g0__t122, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ServiceScope__g0__t122, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ServiceScope__g0__t122(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ModelStateDictionary__g0__t152(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t21 = icmp eq ptr %field_3, null
  br i1 %t21, label %collection_release_done_10, label %collection_release_9
collection_release_9:
  %t22 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 0
  %t23 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 2
  %t24 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 3
  %t25 = load i64, ptr %t22
  %t26 = load ptr, ptr %t23
  %t27 = load ptr, ptr %t24
  %t28 = alloca i64
  store i64 0, ptr %t28
  br label %element_drop_loop_11
element_drop_loop_11:
  %t29 = load i64, ptr %t28
  %t30 = icmp ult i64 %t29, %t25
  br i1 %t30, label %element_drop_body_12, label %element_drop_done_13
element_drop_body_12:
  %t31 = getelementptr inbounds ptr, ptr %t26, i64 %t29
  %t32 = load ptr, ptr %t31
  call void @glitch_string_release(ptr %t32)
  %t33 = add i64 %t29, 1
  store i64 %t33, ptr %t28
  br label %element_drop_loop_11
element_drop_done_13:
  %t34 = alloca i64
  store i64 0, ptr %t34
  br label %element_drop_loop_14
element_drop_loop_14:
  %t35 = load i64, ptr %t34
  %t36 = icmp ult i64 %t35, %t25
  br i1 %t36, label %element_drop_body_15, label %element_drop_done_16
element_drop_body_15:
  %t37 = getelementptr inbounds ptr, ptr %t27, i64 %t35
  %t38 = load ptr, ptr %t37
  call void @glitch_drop_ModelStateEntry__g0__t151(ptr %t38)
  %t39 = add i64 %t35, 1
  store i64 %t39, ptr %t34
  br label %element_drop_loop_14
element_drop_done_16:
  call void @glitch_free(ptr %t26)
  call void @glitch_free(ptr %t27)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_10
collection_release_done_10:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ModelStateDictionary__g0__t152(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ModelStateDictionary__g0__t152(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ModelStateDictionary__g0__t152(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_RuleBuilder_T_TProperty___g0__t100(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_RuleBuilder_T_TProperty___g0__t100(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_RuleBuilder_T_TProperty___g0__t100(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_RuleBuilder_T_TProperty___g0__t100(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbSet__g1__t51(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %destroy_object
destroy_object:
  %field_0_ptr = getelementptr inbounds %glitch.DbSet__g1__t51, ptr %object, i32 0, i32 0
  %field_0 = load ptr, ptr %field_0_ptr
  %t40 = icmp eq ptr %field_0, null
  br i1 %t40, label %collection_release_done_18, label %collection_release_17
collection_release_17:
  %t41 = getelementptr inbounds %glitch.list, ptr %field_0, i32 0, i32 0
  %t42 = getelementptr inbounds %glitch.list, ptr %field_0, i32 0, i32 2
  %t43 = load i64, ptr %t41
  %t44 = load ptr, ptr %t42
  call void @glitch_free(ptr %t44)
  call void @glitch_free(ptr %field_0)
  br label %collection_release_done_18
collection_release_done_18:
  call void @glitch_free(ptr %object)
  br label %done
done:
  ret void
}

define void @glitch_destroy_ValidationResult__g0__t96(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t45 = icmp eq ptr %field_2, null
  br i1 %t45, label %collection_release_done_20, label %collection_release_19
collection_release_19:
  %t46 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t47 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t48 = load i64, ptr %t46
  %t49 = load ptr, ptr %t47
  %t50 = alloca i64
  store i64 0, ptr %t50
  br label %element_drop_loop_21
element_drop_loop_21:
  %t51 = load i64, ptr %t50
  %t52 = icmp ult i64 %t51, %t48
  br i1 %t52, label %element_drop_body_22, label %element_drop_done_23
element_drop_body_22:
  %t53 = getelementptr inbounds ptr, ptr %t49, i64 %t51
  %t54 = load ptr, ptr %t53
  call void @glitch_drop_ValidationFailure__g0__t95(ptr %t54)
  %t55 = add i64 %t51, 1
  store i64 %t55, ptr %t50
  br label %element_drop_loop_21
element_drop_done_23:
  call void @glitch_free(ptr %t49)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_20
collection_release_done_20:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationResult__g0__t96(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationResult__g0__t96(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationResult__g0__t96(ptr %object)
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

define void @glitch_destroy_MapperConfiguration__g0__t94(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.MapperConfiguration__g0__t94, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_Mapper__g0__t87(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MapperConfiguration__g0__t94(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MapperConfiguration__g0__t94, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MapperConfiguration__g0__t94(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MapperConfiguration__g0__t94, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MapperConfiguration__g0__t94, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MapperConfiguration__g0__t94(ptr %object)
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

define void @glitch_destroy_MappingExpression__g2__t91(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MappingExpression__g2__t91(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MappingExpression__g2__t91, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MappingExpression__g2__t91(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MappingExpression__g2__t91, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MappingExpression__g2__t91, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MappingExpression__g2__t91(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ServiceCollection__g0__t125(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ServiceCollection__g0__t125, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_object__g0__t14(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.ServiceCollection__g0__t125, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ServiceCollection__g0__t125(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ServiceCollection__g0__t125, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ServiceCollection__g0__t125(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ServiceCollection__g0__t125, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ServiceCollection__g0__t125, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ServiceCollection__g0__t125(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_AuthenticationBuilder__g0__t116(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_AuthenticationBuilder__g0__t116(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.AuthenticationBuilder__g0__t116, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_AuthenticationBuilder__g0__t116(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.AuthenticationBuilder__g0__t116, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.AuthenticationBuilder__g0__t116, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_AuthenticationBuilder__g0__t116(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DatabaseFacade__g0__t56(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DatabaseFacade__g0__t56(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DatabaseFacade__g0__t56(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DatabaseFacade__g0__t56(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HostEnvironment__g0__t127(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HostEnvironment__g0__t127(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HostEnvironment__g0__t127, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HostEnvironment__g0__t127(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HostEnvironment__g0__t127, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HostEnvironment__g0__t127, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HostEnvironment__g0__t127(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbSet_T___g0__t51(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %destroy_object
destroy_object:
  %field_0_ptr = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %object, i32 0, i32 0
  %field_0 = load ptr, ptr %field_0_ptr
  %t56 = icmp eq ptr %field_0, null
  br i1 %t56, label %collection_release_done_25, label %collection_release_24
collection_release_24:
  %t57 = getelementptr inbounds %glitch.list, ptr %field_0, i32 0, i32 0
  %t58 = getelementptr inbounds %glitch.list, ptr %field_0, i32 0, i32 2
  %t59 = load i64, ptr %t57
  %t60 = load ptr, ptr %t58
  call void @glitch_free(ptr %t60)
  call void @glitch_free(ptr %field_0)
  br label %collection_release_done_25
collection_release_done_25:
  call void @glitch_free(ptr %object)
  br label %done
done:
  ret void
}

define void @glitch_destroy_ValidationFailure__g0__t95(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ValidationFailure__g0__t95, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationFailure__g0__t95(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationFailure__g0__t95, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationFailure__g0__t95(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationFailure__g0__t95, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationFailure__g0__t95, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationFailure__g0__t95(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IDbContextTransaction__g0__t48(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IDbContextTransaction__g0__t48(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IDbContextTransaction__g0__t48, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IDbContextTransaction__g0__t48(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IDbContextTransaction__g0__t48, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IDbContextTransaction__g0__t48, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IDbContextTransaction__g0__t48(ptr %object)
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

define void @glitch_destroy_OpenApiSecurityRequirement__g0__t106(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_OpenApiSecurityRequirement__g0__t106(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.OpenApiSecurityRequirement__g0__t106, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_OpenApiSecurityRequirement__g0__t106(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.OpenApiSecurityRequirement__g0__t106, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.OpenApiSecurityRequirement__g0__t106, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_OpenApiSecurityRequirement__g0__t106(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_OpenApiReference__g0__t104(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.OpenApiReference__g0__t104, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_OpenApiReference__g0__t104(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.OpenApiReference__g0__t104, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_OpenApiReference__g0__t104(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.OpenApiReference__g0__t104, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.OpenApiReference__g0__t104, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_OpenApiReference__g0__t104(ptr %object)
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

define void @glitch_destroy_MediatRServiceConfiguration__g0__t85(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MediatRServiceConfiguration__g0__t85(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MediatRServiceConfiguration__g0__t85, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MediatRServiceConfiguration__g0__t85(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MediatRServiceConfiguration__g0__t85, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MediatRServiceConfiguration__g0__t85, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MediatRServiceConfiguration__g0__t85(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Endpoint__g0__t140(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Endpoint__g0__t140, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.Endpoint__g0__t140, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_4_ptr = getelementptr inbounds %glitch.Endpoint__g0__t140, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Endpoint__g0__t140(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Endpoint__g0__t140, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Endpoint__g0__t140(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Endpoint__g0__t140, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Endpoint__g0__t140, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Endpoint__g0__t140(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ControllerBase__g0__t153(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t153, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_HttpContext__g0__t134(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t153, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ModelStateDictionary__g0__t152(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ControllerBase__g0__t153(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t153, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ControllerBase__g0__t153(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t153, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t153, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ControllerBase__g0__t153(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ObjectResult__g0__t146(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ObjectResult__g0__t146(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ObjectResult__g0__t146, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ObjectResult__g0__t146(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ObjectResult__g0__t146, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ObjectResult__g0__t146, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ObjectResult__g0__t146(ptr %object)
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

define void @glitch_destroy_AbstractValidator_T___g0__t101(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_AbstractValidator_T___g0__t101(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.AbstractValidator_T___g0__t101, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_AbstractValidator_T___g0__t101(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.AbstractValidator_T___g0__t101, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.AbstractValidator_T___g0__t101, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_AbstractValidator_T___g0__t101(ptr %object)
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

define void @glitch_destroy_ILoggerProvider__g0__t71(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ILoggerProvider__g0__t71(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ILoggerProvider__g0__t71, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ILoggerProvider__g0__t71(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ILoggerProvider__g0__t71, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ILoggerProvider__g0__t71, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ILoggerProvider__g0__t71(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HashSet__g1__t44(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.HashSet__g1__t44, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t61 = icmp eq ptr %field_3, null
  br i1 %t61, label %collection_release_done_27, label %collection_release_26
collection_release_26:
  %t62 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 0
  %t63 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 2
  %t64 = load i64, ptr %t62
  %t65 = load ptr, ptr %t63
  call void @glitch_free(ptr %t65)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_27
collection_release_done_27:
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

define void @glitch_destroy_IServiceProvider__g0__t108(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IServiceProvider__g0__t108(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IServiceProvider__g0__t108, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IServiceProvider__g0__t108(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IServiceProvider__g0__t108, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IServiceProvider__g0__t108, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IServiceProvider__g0__t108(ptr %object)
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

define void @glitch_destroy_ValueTask__g0__t47(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValueTask__g0__t47(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValueTask__g0__t47, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValueTask__g0__t47(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValueTask__g0__t47, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValueTask__g0__t47, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValueTask__g0__t47(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_SecurityAlgorithms__g0__t76(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SecurityAlgorithms__g0__t76(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SecurityAlgorithms__g0__t76, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SecurityAlgorithms__g0__t76(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SecurityAlgorithms__g0__t76, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SecurityAlgorithms__g0__t76, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SecurityAlgorithms__g0__t76(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IMappingExpression__g2__t90(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IMappingExpression__g2__t90(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IMappingExpression__g2__t90, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IMappingExpression__g2__t90(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IMappingExpression__g2__t90, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IMappingExpression__g2__t90, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IMappingExpression__g2__t90(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ServiceProvider__g0__t119(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_object__g0__t14(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ServiceProvider__g0__t119(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ServiceProvider__g0__t119(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ServiceProvider__g0__t119(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IndexBuilder__g0__t67(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IndexBuilder__g0__t67(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IndexBuilder__g0__t67, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IndexBuilder__g0__t67(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IndexBuilder__g0__t67, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IndexBuilder__g0__t67, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IndexBuilder__g0__t67(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IHttpContextAccessor__g0__t135(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IHttpContextAccessor__g0__t135, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_HttpContext__g0__t134(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IHttpContextAccessor__g0__t135(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IHttpContextAccessor__g0__t135, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IHttpContextAccessor__g0__t135(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IHttpContextAccessor__g0__t135, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IHttpContextAccessor__g0__t135, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IHttpContextAccessor__g0__t135(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ModelError__g0__t150(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ModelError__g0__t150, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ModelError__g0__t150(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ModelError__g0__t150, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ModelError__g0__t150(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ModelError__g0__t150, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ModelError__g0__t150, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ModelError__g0__t150(ptr %object)
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

define void @glitch_destroy_EntityTypeBuilder__g1__t65(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_EntityTypeBuilder__g1__t65(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.EntityTypeBuilder__g1__t65, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_EntityTypeBuilder__g1__t65(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.EntityTypeBuilder__g1__t65, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.EntityTypeBuilder__g1__t65, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_EntityTypeBuilder__g1__t65(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MapperConfigurationExpression__g0__t93(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t93, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t66 = icmp eq ptr %field_2, null
  br i1 %t66, label %collection_release_done_29, label %collection_release_28
collection_release_28:
  %t67 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t68 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t69 = load i64, ptr %t67
  %t70 = load ptr, ptr %t68
  %t71 = alloca i64
  store i64 0, ptr %t71
  br label %element_drop_loop_30
element_drop_loop_30:
  %t72 = load i64, ptr %t71
  %t73 = icmp ult i64 %t72, %t69
  br i1 %t73, label %element_drop_body_31, label %element_drop_done_32
element_drop_body_31:
  %t74 = getelementptr inbounds ptr, ptr %t70, i64 %t72
  %t75 = load ptr, ptr %t74
  call void @glitch_drop_Profile__g0__t92(ptr %t75)
  %t76 = add i64 %t72, 1
  store i64 %t76, ptr %t71
  br label %element_drop_loop_30
element_drop_done_32:
  call void @glitch_free(ptr %t70)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_29
collection_release_done_29:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MapperConfigurationExpression__g0__t93(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t93, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MapperConfigurationExpression__g0__t93(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t93, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t93, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MapperConfigurationExpression__g0__t93(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ConnectionInfo__g0__t133(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ConnectionInfo__g0__t133, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_IPAddress__g0__t132(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ConnectionInfo__g0__t133(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ConnectionInfo__g0__t133, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ConnectionInfo__g0__t133(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ConnectionInfo__g0__t133, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ConnectionInfo__g0__t133, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ConnectionInfo__g0__t133(ptr %object)
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

define void @glitch_destroy_OpenApiInfo__g0__t107(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.OpenApiInfo__g0__t107, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.OpenApiInfo__g0__t107, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_OpenApiInfo__g0__t107(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.OpenApiInfo__g0__t107, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_OpenApiInfo__g0__t107(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.OpenApiInfo__g0__t107, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.OpenApiInfo__g0__t107, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_OpenApiInfo__g0__t107(ptr %object)
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

define void @glitch_destroy_WebApplication__g0__t141(ptr %object) {
entry:
  %field_7_ptr = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_HostEnvironment__g0__t127(ptr %field_7)
  %field_3_ptr = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t77 = icmp eq ptr %field_3, null
  br i1 %t77, label %collection_release_done_34, label %collection_release_33
collection_release_33:
  %t78 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 0
  %t79 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 2
  %t80 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 3
  %t81 = load i64, ptr %t78
  %t82 = load ptr, ptr %t79
  %t83 = load ptr, ptr %t80
  %t84 = alloca i64
  store i64 0, ptr %t84
  br label %element_drop_loop_35
element_drop_loop_35:
  %t85 = load i64, ptr %t84
  %t86 = icmp ult i64 %t85, %t81
  br i1 %t86, label %element_drop_body_36, label %element_drop_done_37
element_drop_body_36:
  %t87 = getelementptr inbounds ptr, ptr %t82, i64 %t85
  %t88 = load ptr, ptr %t87
  call void @glitch_string_release(ptr %t88)
  %t89 = add i64 %t85, 1
  store i64 %t89, ptr %t84
  br label %element_drop_loop_35
element_drop_done_37:
  %t90 = alloca i64
  store i64 0, ptr %t90
  br label %element_drop_loop_38
element_drop_loop_38:
  %t91 = load i64, ptr %t90
  %t92 = icmp ult i64 %t91, %t81
  br i1 %t92, label %element_drop_body_39, label %element_drop_done_40
element_drop_body_39:
  %t93 = getelementptr inbounds ptr, ptr %t83, i64 %t91
  %t94 = load ptr, ptr %t93
  call void @glitch_string_release(ptr %t94)
  %t95 = add i64 %t91, 1
  store i64 %t95, ptr %t90
  br label %element_drop_loop_38
element_drop_done_40:
  call void @glitch_free(ptr %t82)
  call void @glitch_free(ptr %t83)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_34
collection_release_done_34:
  %field_4_ptr = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t96 = icmp eq ptr %field_4, null
  br i1 %t96, label %collection_release_done_42, label %collection_release_41
collection_release_41:
  %t97 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 0
  %t98 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 2
  %t99 = load i64, ptr %t97
  %t100 = load ptr, ptr %t98
  %t101 = alloca i64
  store i64 0, ptr %t101
  br label %element_drop_loop_43
element_drop_loop_43:
  %t102 = load i64, ptr %t101
  %t103 = icmp ult i64 %t102, %t99
  br i1 %t103, label %element_drop_body_44, label %element_drop_done_45
element_drop_body_44:
  %t104 = getelementptr inbounds ptr, ptr %t100, i64 %t102
  %t105 = load ptr, ptr %t104
  call void @glitch_string_release(ptr %t105)
  %t106 = add i64 %t102, 1
  store i64 %t106, ptr %t101
  br label %element_drop_loop_43
element_drop_done_45:
  call void @glitch_free(ptr %t100)
  call void @glitch_free(ptr %field_4)
  br label %collection_release_done_42
collection_release_done_42:
  %field_2_ptr = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t107 = icmp eq ptr %field_2, null
  br i1 %t107, label %collection_release_done_47, label %collection_release_46
collection_release_46:
  %t108 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t109 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t110 = load i64, ptr %t108
  %t111 = load ptr, ptr %t109
  %t112 = alloca i64
  store i64 0, ptr %t112
  br label %element_drop_loop_48
element_drop_loop_48:
  %t113 = load i64, ptr %t112
  %t114 = icmp ult i64 %t113, %t110
  br i1 %t114, label %element_drop_body_49, label %element_drop_done_50
element_drop_body_49:
  %t115 = getelementptr inbounds ptr, ptr %t111, i64 %t113
  %t116 = load ptr, ptr %t115
  call void @glitch_string_release(ptr %t116)
  %t117 = add i64 %t113, 1
  store i64 %t117, ptr %t112
  br label %element_drop_loop_48
element_drop_done_50:
  call void @glitch_free(ptr %t111)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_47
collection_release_done_47:
  %field_5_ptr = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_ConfigurationManager__g0__t126(ptr %field_5)
  %field_6_ptr = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_ServiceProvider__g0__t119(ptr %field_6)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_WebApplication__g0__t141(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_WebApplication__g0__t141(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_WebApplication__g0__t141(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_SigningCredentials__g0__t74(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.SigningCredentials__g0__t74, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_SymmetricSecurityKey__g0__t73(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.SigningCredentials__g0__t74, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SigningCredentials__g0__t74(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SigningCredentials__g0__t74, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SigningCredentials__g0__t74(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SigningCredentials__g0__t74, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SigningCredentials__g0__t74, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SigningCredentials__g0__t74(ptr %object)
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

define void @glitch_destroy_SwaggerActionDescriptor__g0__t117(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.SwaggerActionDescriptor__g0__t117, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SwaggerActionDescriptor__g0__t117(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SwaggerActionDescriptor__g0__t117, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SwaggerActionDescriptor__g0__t117(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SwaggerActionDescriptor__g0__t117, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SwaggerActionDescriptor__g0__t117, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SwaggerActionDescriptor__g0__t117(ptr %object)
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

define void @glitch_destroy_PropertyInfo__g0__t8(ptr %object) {
entry:
  %field_7_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t8, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_MethodInfo__g0__t4(ptr %field_7)
  %field_8_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t8, ptr %object, i32 0, i32 8
  %field_8 = load ptr, ptr %field_8_ptr
  call void @glitch_drop_MethodInfo__g0__t4(ptr %field_8)
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

define void @glitch_destroy_LoggerFactory__g0__t72(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_LoggerFactory__g0__t72(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.LoggerFactory__g0__t72, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_LoggerFactory__g0__t72(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.LoggerFactory__g0__t72, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.LoggerFactory__g0__t72, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_LoggerFactory__g0__t72(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MappingExpression_TSource_TDestination___g0__t91(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MappingExpression_TSource_TDestination___g0__t91(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MappingExpression_TSource_TDestination___g0__t91(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MappingExpression_TSource_TDestination___g0__t91(ptr %object)
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

define void @glitch_destroy_ListEnumerator_T___g0__t43(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.ListEnumerator_T___g0__t43, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t118 = icmp eq ptr %field_3, null
  br i1 %t118, label %collection_release_done_52, label %collection_release_51
collection_release_51:
  %t119 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 0
  %t120 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 2
  %t121 = load i64, ptr %t119
  %t122 = load ptr, ptr %t120
  call void @glitch_free(ptr %t122)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_52
collection_release_done_52:
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

define void @glitch_drop_DbQuery__g1__t52(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %destroy_object
destroy_object:
  %field_0_ptr = getelementptr inbounds %glitch.DbQuery__g1__t52, ptr %object, i32 0, i32 0
  %field_0 = load ptr, ptr %field_0_ptr
  %t123 = icmp eq ptr %field_0, null
  br i1 %t123, label %collection_release_done_54, label %collection_release_53
collection_release_53:
  %t124 = getelementptr inbounds %glitch.list, ptr %field_0, i32 0, i32 0
  %t125 = getelementptr inbounds %glitch.list, ptr %field_0, i32 0, i32 2
  %t126 = load i64, ptr %t124
  %t127 = load ptr, ptr %t125
  call void @glitch_free(ptr %t127)
  call void @glitch_free(ptr %field_0)
  br label %collection_release_done_54
collection_release_done_54:
  %field_1_ptr = getelementptr inbounds %glitch.DbQuery__g1__t52, ptr %object, i32 0, i32 1
  %field_1 = load ptr, ptr %field_1_ptr
  call void @glitch_delegate_release(ptr %field_1)
  call void @glitch_free(ptr %object)
  br label %done
done:
  ret void
}

define void @glitch_destroy_IRequestHandler_TRequest___g0__t81(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IRequestHandler_TRequest___g0__t81(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IRequestHandler_TRequest___g0__t81, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IRequestHandler_TRequest___g0__t81(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IRequestHandler_TRequest___g0__t81, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IRequestHandler_TRequest___g0__t81, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IRequestHandler_TRequest___g0__t81(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Type__g0__t24(ptr %object) {
entry:
  %field_19_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 19
  %field_19 = load ptr, ptr %field_19_ptr
  %t128 = icmp eq ptr %field_19, null
  br i1 %t128, label %array_release_done_56, label %array_release_55
array_release_55:
  %t129 = getelementptr inbounds %glitch.array, ptr %field_19, i32 0, i32 0
  %t131 = getelementptr inbounds %glitch.array, ptr %field_19, i32 0, i32 1
  %t130 = load i64, ptr %t129
  %t132 = load ptr, ptr %t131
  %t133 = alloca i64
  store i64 0, ptr %t133
  br label %element_drop_loop_57
element_drop_loop_57:
  %t134 = load i64, ptr %t133
  %t135 = icmp ult i64 %t134, %t130
  br i1 %t135, label %element_drop_body_58, label %element_drop_done_59
element_drop_body_58:
  %t136 = getelementptr inbounds ptr, ptr %t132, i64 %t134
  %t137 = load ptr, ptr %t136
  call void @glitch_drop_Type__g0__t24(ptr %t137)
  %t138 = add i64 %t134, 1
  store i64 %t138, ptr %t133
  br label %element_drop_loop_57
element_drop_done_59:
  call void @glitch_free(ptr %t132)
  call void @glitch_free(ptr %field_19)
  br label %array_release_done_56
array_release_done_56:
  %field_17_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 17
  %field_17 = load ptr, ptr %field_17_ptr
  %t139 = icmp eq ptr %field_17, null
  br i1 %t139, label %array_release_done_61, label %array_release_60
array_release_60:
  %t140 = getelementptr inbounds %glitch.array, ptr %field_17, i32 0, i32 0
  %t142 = getelementptr inbounds %glitch.array, ptr %field_17, i32 0, i32 1
  %t141 = load i64, ptr %t140
  %t143 = load ptr, ptr %t142
  %t144 = alloca i64
  store i64 0, ptr %t144
  br label %element_drop_loop_62
element_drop_loop_62:
  %t145 = load i64, ptr %t144
  %t146 = icmp ult i64 %t145, %t141
  br i1 %t146, label %element_drop_body_63, label %element_drop_done_64
element_drop_body_63:
  %t147 = getelementptr inbounds ptr, ptr %t143, i64 %t145
  %t148 = load ptr, ptr %t147
  call void @glitch_drop_FieldInfo__g0__t7(ptr %t148)
  %t149 = add i64 %t145, 1
  store i64 %t149, ptr %t144
  br label %element_drop_loop_62
element_drop_done_64:
  call void @glitch_free(ptr %t143)
  call void @glitch_free(ptr %field_17)
  br label %array_release_done_61
array_release_done_61:
  %field_18_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 18
  %field_18 = load ptr, ptr %field_18_ptr
  %t150 = icmp eq ptr %field_18, null
  br i1 %t150, label %array_release_done_66, label %array_release_65
array_release_65:
  %t151 = getelementptr inbounds %glitch.array, ptr %field_18, i32 0, i32 0
  %t153 = getelementptr inbounds %glitch.array, ptr %field_18, i32 0, i32 1
  %t152 = load i64, ptr %t151
  %t154 = load ptr, ptr %t153
  %t155 = alloca i64
  store i64 0, ptr %t155
  br label %element_drop_loop_67
element_drop_loop_67:
  %t156 = load i64, ptr %t155
  %t157 = icmp ult i64 %t156, %t152
  br i1 %t157, label %element_drop_body_68, label %element_drop_done_69
element_drop_body_68:
  %t158 = getelementptr inbounds ptr, ptr %t154, i64 %t156
  %t159 = load ptr, ptr %t158
  call void @glitch_drop_ConstructorInfo__g0__t5(ptr %t159)
  %t160 = add i64 %t156, 1
  store i64 %t160, ptr %t155
  br label %element_drop_loop_67
element_drop_done_69:
  call void @glitch_free(ptr %t154)
  call void @glitch_free(ptr %field_18)
  br label %array_release_done_66
array_release_done_66:
  %field_10_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 10
  %field_10 = load ptr, ptr %field_10_ptr
  call void @glitch_string_release(ptr %field_10)
  %field_12_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 12
  %field_12 = load ptr, ptr %field_12_ptr
  %t161 = icmp eq ptr %field_12, null
  br i1 %t161, label %array_release_done_71, label %array_release_70
array_release_70:
  %t162 = getelementptr inbounds %glitch.array, ptr %field_12, i32 0, i32 0
  %t164 = getelementptr inbounds %glitch.array, ptr %field_12, i32 0, i32 1
  %t163 = load i64, ptr %t162
  %t165 = load ptr, ptr %t164
  %t166 = alloca i64
  store i64 0, ptr %t166
  br label %element_drop_loop_72
element_drop_loop_72:
  %t167 = load i64, ptr %t166
  %t168 = icmp ult i64 %t167, %t163
  br i1 %t168, label %element_drop_body_73, label %element_drop_done_74
element_drop_body_73:
  %t169 = getelementptr inbounds ptr, ptr %t165, i64 %t167
  %t170 = load ptr, ptr %t169
  call void @glitch_drop_Type__g0__t24(ptr %t170)
  %t171 = add i64 %t167, 1
  store i64 %t171, ptr %t166
  br label %element_drop_loop_72
element_drop_done_74:
  call void @glitch_free(ptr %t165)
  call void @glitch_free(ptr %field_12)
  br label %array_release_done_71
array_release_done_71:
  %field_3_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_11_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 11
  %field_11 = load ptr, ptr %field_11_ptr
  call void @glitch_string_release(ptr %field_11)
  %field_15_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 15
  %field_15 = load ptr, ptr %field_15_ptr
  %t172 = icmp eq ptr %field_15, null
  br i1 %t172, label %array_release_done_76, label %array_release_75
array_release_75:
  %t173 = getelementptr inbounds %glitch.array, ptr %field_15, i32 0, i32 0
  %t175 = getelementptr inbounds %glitch.array, ptr %field_15, i32 0, i32 1
  %t174 = load i64, ptr %t173
  %t176 = load ptr, ptr %t175
  %t177 = alloca i64
  store i64 0, ptr %t177
  br label %element_drop_loop_77
element_drop_loop_77:
  %t178 = load i64, ptr %t177
  %t179 = icmp ult i64 %t178, %t174
  br i1 %t179, label %element_drop_body_78, label %element_drop_done_79
element_drop_body_78:
  %t180 = getelementptr inbounds ptr, ptr %t176, i64 %t178
  %t181 = load ptr, ptr %t180
  call void @glitch_drop_PropertyInfo__g0__t8(ptr %t181)
  %t182 = add i64 %t178, 1
  store i64 %t182, ptr %t177
  br label %element_drop_loop_77
element_drop_done_79:
  call void @glitch_free(ptr %t176)
  call void @glitch_free(ptr %field_15)
  br label %array_release_done_76
array_release_done_76:
  %field_2_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_16_ptr = getelementptr inbounds %glitch.Type__g0__t24, ptr %object, i32 0, i32 16
  %field_16 = load ptr, ptr %field_16_ptr
  %t183 = icmp eq ptr %field_16, null
  br i1 %t183, label %array_release_done_81, label %array_release_80
array_release_80:
  %t184 = getelementptr inbounds %glitch.array, ptr %field_16, i32 0, i32 0
  %t186 = getelementptr inbounds %glitch.array, ptr %field_16, i32 0, i32 1
  %t185 = load i64, ptr %t184
  %t187 = load ptr, ptr %t186
  %t188 = alloca i64
  store i64 0, ptr %t188
  br label %element_drop_loop_82
element_drop_loop_82:
  %t189 = load i64, ptr %t188
  %t190 = icmp ult i64 %t189, %t185
  br i1 %t190, label %element_drop_body_83, label %element_drop_done_84
element_drop_body_83:
  %t191 = getelementptr inbounds ptr, ptr %t187, i64 %t189
  %t192 = load ptr, ptr %t191
  call void @glitch_drop_MethodInfo__g0__t4(ptr %t192)
  %t193 = add i64 %t189, 1
  store i64 %t193, ptr %t188
  br label %element_drop_loop_82
element_drop_done_84:
  call void @glitch_free(ptr %t187)
  call void @glitch_free(ptr %field_16)
  br label %array_release_done_81
array_release_done_81:
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

define void @glitch_destroy_PhysicalFileProvider__g0__t143(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_PhysicalFileProvider__g0__t143(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.PhysicalFileProvider__g0__t143, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_PhysicalFileProvider__g0__t143(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.PhysicalFileProvider__g0__t143, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.PhysicalFileProvider__g0__t143, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_PhysicalFileProvider__g0__t143(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_RuleBuilder__g2__t100(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_RuleBuilder__g2__t100(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder__g2__t100, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_RuleBuilder__g2__t100(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder__g2__t100, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.RuleBuilder__g2__t100, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_RuleBuilder__g2__t100(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HttpRequest__g0__t130(ptr %object) {
entry:
  %field_5_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  %field_4_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t194 = icmp eq ptr %field_4, null
  br i1 %t194, label %collection_release_done_86, label %collection_release_85
collection_release_85:
  %t195 = getelementptr inbounds %glitch.dict, ptr %field_4, i32 0, i32 0
  %t196 = getelementptr inbounds %glitch.dict, ptr %field_4, i32 0, i32 2
  %t197 = getelementptr inbounds %glitch.dict, ptr %field_4, i32 0, i32 3
  %t198 = load i64, ptr %t195
  %t199 = load ptr, ptr %t196
  %t200 = load ptr, ptr %t197
  %t201 = alloca i64
  store i64 0, ptr %t201
  br label %element_drop_loop_87
element_drop_loop_87:
  %t202 = load i64, ptr %t201
  %t203 = icmp ult i64 %t202, %t198
  br i1 %t203, label %element_drop_body_88, label %element_drop_done_89
element_drop_body_88:
  %t204 = getelementptr inbounds ptr, ptr %t199, i64 %t202
  %t205 = load ptr, ptr %t204
  call void @glitch_string_release(ptr %t205)
  %t206 = add i64 %t202, 1
  store i64 %t206, ptr %t201
  br label %element_drop_loop_87
element_drop_done_89:
  %t207 = alloca i64
  store i64 0, ptr %t207
  br label %element_drop_loop_90
element_drop_loop_90:
  %t208 = load i64, ptr %t207
  %t209 = icmp ult i64 %t208, %t198
  br i1 %t209, label %element_drop_body_91, label %element_drop_done_92
element_drop_body_91:
  %t210 = getelementptr inbounds ptr, ptr %t200, i64 %t208
  %t211 = load ptr, ptr %t210
  call void @glitch_string_release(ptr %t211)
  %t212 = add i64 %t208, 1
  store i64 %t212, ptr %t207
  br label %element_drop_loop_90
element_drop_done_92:
  call void @glitch_free(ptr %t199)
  call void @glitch_free(ptr %t200)
  call void @glitch_free(ptr %field_4)
  br label %collection_release_done_86
collection_release_done_86:
  %field_3_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HttpRequest__g0__t130(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HttpRequest__g0__t130(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HttpRequest__g0__t130(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ChangeTracker__g0__t54(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_DebugView__g0__t55(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ChangeTracker__g0__t54(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ChangeTracker__g0__t54(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ChangeTracker__g0__t54(ptr %object)
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

define void @glitch_destroy_ConsoleLogger__g0__t69(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ConsoleLogger__g0__t69(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ConsoleLogger__g0__t69, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ConsoleLogger__g0__t69(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ConsoleLogger__g0__t69, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ConsoleLogger__g0__t69, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ConsoleLogger__g0__t69(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_JwtBearerOptions__g0__t78(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.JwtBearerOptions__g0__t78, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_JwtBearerEvents__g0__t79(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.JwtBearerOptions__g0__t78, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_TokenValidationParameters__g0__t75(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_JwtBearerOptions__g0__t78(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.JwtBearerOptions__g0__t78, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_JwtBearerOptions__g0__t78(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.JwtBearerOptions__g0__t78, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.JwtBearerOptions__g0__t78, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_JwtBearerOptions__g0__t78(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IValidator__g1__t99(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IValidator__g1__t99(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IValidator__g1__t99, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IValidator__g1__t99(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IValidator__g1__t99, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IValidator__g1__t99, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IValidator__g1__t99(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MigrationBuilder__g0__t64(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MigrationBuilder__g0__t64(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MigrationBuilder__g0__t64, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MigrationBuilder__g0__t64(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MigrationBuilder__g0__t64, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MigrationBuilder__g0__t64, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MigrationBuilder__g0__t64(ptr %object)
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

define void @glitch_destroy_EntityTypeBuilder_T___g0__t65(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_EntityTypeBuilder_T___g0__t65(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.EntityTypeBuilder_T___g0__t65, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_EntityTypeBuilder_T___g0__t65(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.EntityTypeBuilder_T___g0__t65, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.EntityTypeBuilder_T___g0__t65, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_EntityTypeBuilder_T___g0__t65(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbSetString__g0__t62(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbSetString__g0__t62(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbSetString__g0__t62(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbSetString__g0__t62(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IMediator__g0__t83(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IMediator__g0__t83(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IMediator__g0__t83, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IMediator__g0__t83(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IMediator__g0__t83, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IMediator__g0__t83, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IMediator__g0__t83(ptr %object)
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

define void @glitch_destroy_ValidationException__g0__t97(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.ValidationException__g0__t97, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t213 = icmp eq ptr %field_3, null
  br i1 %t213, label %collection_release_done_94, label %collection_release_93
collection_release_93:
  %t214 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 0
  %t215 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 2
  %t216 = load i64, ptr %t214
  %t217 = load ptr, ptr %t215
  %t218 = alloca i64
  store i64 0, ptr %t218
  br label %element_drop_loop_95
element_drop_loop_95:
  %t219 = load i64, ptr %t218
  %t220 = icmp ult i64 %t219, %t216
  br i1 %t220, label %element_drop_body_96, label %element_drop_done_97
element_drop_body_96:
  %t221 = getelementptr inbounds ptr, ptr %t217, i64 %t219
  %t222 = load ptr, ptr %t221
  call void @glitch_drop_ValidationFailure__g0__t95(ptr %t222)
  %t223 = add i64 %t219, 1
  store i64 %t223, ptr %t218
  br label %element_drop_loop_95
element_drop_done_97:
  call void @glitch_free(ptr %t217)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_94
collection_release_done_94:
  %field_2_ptr = getelementptr inbounds %glitch.ValidationException__g0__t97, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationException__g0__t97(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationException__g0__t97, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationException__g0__t97(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationException__g0__t97, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationException__g0__t97, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationException__g0__t97(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HttpResponse__g0__t131(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t224 = icmp eq ptr %field_3, null
  br i1 %t224, label %collection_release_done_99, label %collection_release_98
collection_release_98:
  %t225 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 0
  %t226 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 2
  %t227 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 3
  %t228 = load i64, ptr %t225
  %t229 = load ptr, ptr %t226
  %t230 = load ptr, ptr %t227
  %t231 = alloca i64
  store i64 0, ptr %t231
  br label %element_drop_loop_100
element_drop_loop_100:
  %t232 = load i64, ptr %t231
  %t233 = icmp ult i64 %t232, %t228
  br i1 %t233, label %element_drop_body_101, label %element_drop_done_102
element_drop_body_101:
  %t234 = getelementptr inbounds ptr, ptr %t229, i64 %t232
  %t235 = load ptr, ptr %t234
  call void @glitch_string_release(ptr %t235)
  %t236 = add i64 %t232, 1
  store i64 %t236, ptr %t231
  br label %element_drop_loop_100
element_drop_done_102:
  %t237 = alloca i64
  store i64 0, ptr %t237
  br label %element_drop_loop_103
element_drop_loop_103:
  %t238 = load i64, ptr %t237
  %t239 = icmp ult i64 %t238, %t228
  br i1 %t239, label %element_drop_body_104, label %element_drop_done_105
element_drop_body_104:
  %t240 = getelementptr inbounds ptr, ptr %t230, i64 %t238
  %t241 = load ptr, ptr %t240
  call void @glitch_string_release(ptr %t241)
  %t242 = add i64 %t238, 1
  store i64 %t242, ptr %t237
  br label %element_drop_loop_103
element_drop_done_105:
  call void @glitch_free(ptr %t229)
  call void @glitch_free(ptr %t230)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_99
collection_release_done_99:
  %field_4_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HttpResponse__g0__t131(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HttpResponse__g0__t131(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HttpResponse__g0__t131(ptr %object)
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

define void @glitch_destroy_Mapper__g0__t87(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Mapper__g0__t87(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Mapper__g0__t87, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Mapper__g0__t87(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Mapper__g0__t87, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Mapper__g0__t87, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Mapper__g0__t87(ptr %object)
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

define void @glitch_destroy_MethodInfo__g0__t4(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_7_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  %t243 = icmp eq ptr %field_7, null
  br i1 %t243, label %array_release_done_107, label %array_release_106
array_release_106:
  %t244 = getelementptr inbounds %glitch.array, ptr %field_7, i32 0, i32 0
  %t246 = getelementptr inbounds %glitch.array, ptr %field_7, i32 0, i32 1
  %t245 = load i64, ptr %t244
  %t247 = load ptr, ptr %t246
  call void @glitch_free(ptr %t247)
  call void @glitch_free(ptr %field_7)
  br label %array_release_done_107
array_release_done_107:
  %field_4_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t248 = icmp eq ptr %field_4, null
  br i1 %t248, label %array_release_done_109, label %array_release_108
array_release_108:
  %t249 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 0
  %t251 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 1
  %t250 = load i64, ptr %t249
  %t252 = load ptr, ptr %t251
  %t253 = alloca i64
  store i64 0, ptr %t253
  br label %element_drop_loop_110
element_drop_loop_110:
  %t254 = load i64, ptr %t253
  %t255 = icmp ult i64 %t254, %t250
  br i1 %t255, label %element_drop_body_111, label %element_drop_done_112
element_drop_body_111:
  %t256 = getelementptr inbounds ptr, ptr %t252, i64 %t254
  %t257 = load ptr, ptr %t256
  call void @glitch_drop_ParameterInfo__g0__t6(ptr %t257)
  %t258 = add i64 %t254, 1
  store i64 %t258, ptr %t253
  br label %element_drop_loop_110
element_drop_done_112:
  call void @glitch_free(ptr %t252)
  call void @glitch_free(ptr %field_4)
  br label %array_release_done_109
array_release_done_109:
  %field_9_ptr = getelementptr inbounds %glitch.MethodInfo__g0__t4, ptr %object, i32 0, i32 9
  %field_9 = load ptr, ptr %field_9_ptr
  %t259 = icmp eq ptr %field_9, null
  br i1 %t259, label %array_release_done_114, label %array_release_113
array_release_113:
  %t260 = getelementptr inbounds %glitch.array, ptr %field_9, i32 0, i32 0
  %t262 = getelementptr inbounds %glitch.array, ptr %field_9, i32 0, i32 1
  %t261 = load i64, ptr %t260
  %t263 = load ptr, ptr %t262
  call void @glitch_free(ptr %t263)
  call void @glitch_free(ptr %field_9)
  br label %array_release_done_114
array_release_done_114:
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

define void @glitch_destroy_MemberConfigurationExpression__g0__t89(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MemberConfigurationExpression__g0__t89(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MemberConfigurationExpression__g0__t89, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MemberConfigurationExpression__g0__t89(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MemberConfigurationExpression__g0__t89, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MemberConfigurationExpression__g0__t89, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MemberConfigurationExpression__g0__t89(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MethodBase__g0__t3(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.MethodBase__g0__t3, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_4_ptr = getelementptr inbounds %glitch.MethodBase__g0__t3, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t264 = icmp eq ptr %field_4, null
  br i1 %t264, label %array_release_done_116, label %array_release_115
array_release_115:
  %t265 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 0
  %t267 = getelementptr inbounds %glitch.array, ptr %field_4, i32 0, i32 1
  %t266 = load i64, ptr %t265
  %t268 = load ptr, ptr %t267
  %t269 = alloca i64
  store i64 0, ptr %t269
  br label %element_drop_loop_117
element_drop_loop_117:
  %t270 = load i64, ptr %t269
  %t271 = icmp ult i64 %t270, %t266
  br i1 %t271, label %element_drop_body_118, label %element_drop_done_119
element_drop_body_118:
  %t272 = getelementptr inbounds ptr, ptr %t268, i64 %t270
  %t273 = load ptr, ptr %t272
  call void @glitch_drop_ParameterInfo__g0__t6(ptr %t273)
  %t274 = add i64 %t270, 1
  store i64 %t274, ptr %t269
  br label %element_drop_loop_117
element_drop_done_119:
  call void @glitch_free(ptr %t268)
  call void @glitch_free(ptr %field_4)
  br label %array_release_done_116
array_release_done_116:
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

define void @glitch_destroy_IRequest__g0__t80(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IRequest__g0__t80(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IRequest__g0__t80, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IRequest__g0__t80(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IRequest__g0__t80, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IRequest__g0__t80, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IRequest__g0__t80(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ValidationContext__g1__t98(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationContext__g1__t98(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationContext__g1__t98, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationContext__g1__t98(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationContext__g1__t98, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationContext__g1__t98, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationContext__g1__t98(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IRequestHandler__g1__t81(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IRequestHandler__g1__t81(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IRequestHandler__g1__t81, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IRequestHandler__g1__t81(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IRequestHandler__g1__t81, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IRequestHandler__g1__t81, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IRequestHandler__g1__t81(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_SymmetricSecurityKey__g0__t73(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.SymmetricSecurityKey__g0__t73, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SymmetricSecurityKey__g0__t73(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SymmetricSecurityKey__g0__t73, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SymmetricSecurityKey__g0__t73(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SymmetricSecurityKey__g0__t73, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SymmetricSecurityKey__g0__t73, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SymmetricSecurityKey__g0__t73(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ConfigurationManager__g0__t126(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ConfigurationManager__g0__t126(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ConfigurationManager__g0__t126, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ConfigurationManager__g0__t126(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ConfigurationManager__g0__t126, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ConfigurationManager__g0__t126, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ConfigurationManager__g0__t126(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DebugView__g0__t55(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.DebugView__g0__t55, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DebugView__g0__t55(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DebugView__g0__t55, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DebugView__g0__t55(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DebugView__g0__t55, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DebugView__g0__t55, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DebugView__g0__t55(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IServiceScope__g0__t120(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IServiceScope__g0__t120, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_IServiceProvider__g0__t108(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IServiceScope__g0__t120(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IServiceScope__g0__t120, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IServiceScope__g0__t120(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IServiceScope__g0__t120, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IServiceScope__g0__t120, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IServiceScope__g0__t120(ptr %object)
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

define void @glitch_destroy_ValidationContext_T___g0__t98(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValidationContext_T___g0__t98(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValidationContext_T___g0__t98, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValidationContext_T___g0__t98(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValidationContext_T___g0__t98, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValidationContext_T___g0__t98, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValidationContext_T___g0__t98(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_CorsPolicyBuilder__g0__t111(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_CorsPolicyBuilder__g0__t111(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.CorsPolicyBuilder__g0__t111, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_CorsPolicyBuilder__g0__t111(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.CorsPolicyBuilder__g0__t111, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.CorsPolicyBuilder__g0__t111, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_CorsPolicyBuilder__g0__t111(ptr %object)
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

define void @glitch_destroy_AbstractValidator__g1__t101(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_AbstractValidator__g1__t101(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.AbstractValidator__g1__t101, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_AbstractValidator__g1__t101(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.AbstractValidator__g1__t101, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.AbstractValidator__g1__t101, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_AbstractValidator__g1__t101(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ILogger__g0__t68(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ILogger__g0__t68(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ILogger__g0__t68, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ILogger__g0__t68(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ILogger__g0__t68, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ILogger__g0__t68, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ILogger__g0__t68(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ProblemDetails__g0__t145(ptr %object) {
entry:
  %field_4_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t145, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_6_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t145, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  %t275 = icmp eq ptr %field_6, null
  br i1 %t275, label %collection_release_done_121, label %collection_release_120
collection_release_120:
  %t276 = getelementptr inbounds %glitch.dict, ptr %field_6, i32 0, i32 0
  %t277 = getelementptr inbounds %glitch.dict, ptr %field_6, i32 0, i32 2
  %t278 = getelementptr inbounds %glitch.dict, ptr %field_6, i32 0, i32 3
  %t279 = load i64, ptr %t276
  %t280 = load ptr, ptr %t277
  %t281 = load ptr, ptr %t278
  %t282 = alloca i64
  store i64 0, ptr %t282
  br label %element_drop_loop_122
element_drop_loop_122:
  %t283 = load i64, ptr %t282
  %t284 = icmp ult i64 %t283, %t279
  br i1 %t284, label %element_drop_body_123, label %element_drop_done_124
element_drop_body_123:
  %t285 = getelementptr inbounds ptr, ptr %t280, i64 %t283
  %t286 = load ptr, ptr %t285
  call void @glitch_string_release(ptr %t286)
  %t287 = add i64 %t283, 1
  store i64 %t287, ptr %t282
  br label %element_drop_loop_122
element_drop_done_124:
  %t288 = alloca i64
  store i64 0, ptr %t288
  br label %element_drop_loop_125
element_drop_loop_125:
  %t289 = load i64, ptr %t288
  %t290 = icmp ult i64 %t289, %t279
  br i1 %t290, label %element_drop_body_126, label %element_drop_done_127
element_drop_body_126:
  %t291 = getelementptr inbounds ptr, ptr %t281, i64 %t289
  %t292 = load ptr, ptr %t291
  call void @glitch_drop_object__g0__t14(ptr %t292)
  %t293 = add i64 %t289, 1
  store i64 %t293, ptr %t288
  br label %element_drop_loop_125
element_drop_done_127:
  call void @glitch_free(ptr %t280)
  call void @glitch_free(ptr %t281)
  call void @glitch_free(ptr %field_6)
  br label %collection_release_done_121
collection_release_done_121:
  %field_5_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t145, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  %field_2_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t145, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ProblemDetails__g0__t145(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t145, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ProblemDetails__g0__t145(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t145, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t145, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ProblemDetails__g0__t145(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IServiceCollection__g0__t124(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_object__g0__t14(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IServiceCollection__g0__t124(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IServiceCollection__g0__t124(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IServiceCollection__g0__t124(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ActionExecutingContext__g0__t148(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t148, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_ModelStateDictionary__g0__t152(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t148, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t294 = icmp eq ptr %field_2, null
  br i1 %t294, label %collection_release_done_129, label %collection_release_128
collection_release_128:
  %t295 = getelementptr inbounds %glitch.dict, ptr %field_2, i32 0, i32 0
  %t296 = getelementptr inbounds %glitch.dict, ptr %field_2, i32 0, i32 2
  %t297 = getelementptr inbounds %glitch.dict, ptr %field_2, i32 0, i32 3
  %t298 = load i64, ptr %t295
  %t299 = load ptr, ptr %t296
  %t300 = load ptr, ptr %t297
  %t301 = alloca i64
  store i64 0, ptr %t301
  br label %element_drop_loop_130
element_drop_loop_130:
  %t302 = load i64, ptr %t301
  %t303 = icmp ult i64 %t302, %t298
  br i1 %t303, label %element_drop_body_131, label %element_drop_done_132
element_drop_body_131:
  %t304 = getelementptr inbounds ptr, ptr %t299, i64 %t302
  %t305 = load ptr, ptr %t304
  call void @glitch_string_release(ptr %t305)
  %t306 = add i64 %t302, 1
  store i64 %t306, ptr %t301
  br label %element_drop_loop_130
element_drop_done_132:
  %t307 = alloca i64
  store i64 0, ptr %t307
  br label %element_drop_loop_133
element_drop_loop_133:
  %t308 = load i64, ptr %t307
  %t309 = icmp ult i64 %t308, %t298
  br i1 %t309, label %element_drop_body_134, label %element_drop_done_135
element_drop_body_134:
  %t310 = getelementptr inbounds ptr, ptr %t300, i64 %t308
  %t311 = load ptr, ptr %t310
  call void @glitch_drop_object__g0__t14(ptr %t311)
  %t312 = add i64 %t308, 1
  store i64 %t312, ptr %t307
  br label %element_drop_loop_133
element_drop_done_135:
  call void @glitch_free(ptr %t299)
  call void @glitch_free(ptr %t300)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_129
collection_release_done_129:
  %field_5_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t148, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_object__g0__t14(ptr %field_5)
  %field_4_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t148, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_HttpContext__g0__t134(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ActionExecutingContext__g0__t148(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t148, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ActionExecutingContext__g0__t148(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t148, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t148, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ActionExecutingContext__g0__t148(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbQuery_T___g0__t52(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %destroy_object
destroy_object:
  %field_0_ptr = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %object, i32 0, i32 0
  %field_0 = load ptr, ptr %field_0_ptr
  %t313 = icmp eq ptr %field_0, null
  br i1 %t313, label %collection_release_done_137, label %collection_release_136
collection_release_136:
  %t314 = getelementptr inbounds %glitch.list, ptr %field_0, i32 0, i32 0
  %t315 = getelementptr inbounds %glitch.list, ptr %field_0, i32 0, i32 2
  %t316 = load i64, ptr %t314
  %t317 = load ptr, ptr %t315
  call void @glitch_free(ptr %t317)
  call void @glitch_free(ptr %field_0)
  br label %collection_release_done_137
collection_release_done_137:
  %field_1_ptr = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %object, i32 0, i32 1
  %field_1 = load ptr, ptr %field_1_ptr
  call void @glitch_delegate_release(ptr %field_1)
  call void @glitch_free(ptr %object)
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

define void @glitch_destroy_CorsMiddlewareOptions__g0__t139(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_CorsMiddlewareOptions__g0__t139(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.CorsMiddlewareOptions__g0__t139, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_CorsMiddlewareOptions__g0__t139(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.CorsMiddlewareOptions__g0__t139, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.CorsMiddlewareOptions__g0__t139, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_CorsMiddlewareOptions__g0__t139(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbUpdateConcurrencyException__g0__t59(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.DbUpdateConcurrencyException__g0__t59, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbUpdateConcurrencyException__g0__t59(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbUpdateConcurrencyException__g0__t59, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbUpdateConcurrencyException__g0__t59(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbUpdateConcurrencyException__g0__t59, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbUpdateConcurrencyException__g0__t59, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbUpdateConcurrencyException__g0__t59(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IMapper__g0__t86(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IMapper__g0__t86(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IMapper__g0__t86, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IMapper__g0__t86(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IMapper__g0__t86, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IMapper__g0__t86, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IMapper__g0__t86(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbContextOptionsBuilder__g0__t109(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_DbContextOptions__g0__t49(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbContextOptionsBuilder__g0__t109(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbContextOptionsBuilder__g0__t109(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbContextOptionsBuilder__g0__t109(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IPipelineBehavior__g2__t82(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IPipelineBehavior__g2__t82(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IPipelineBehavior__g2__t82, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IPipelineBehavior__g2__t82(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IPipelineBehavior__g2__t82, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IPipelineBehavior__g2__t82, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IPipelineBehavior__g2__t82(ptr %object)
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

define void @glitch_destroy_EntityEntry__g0__t58(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_EntityEntry__g0__t58(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.EntityEntry__g0__t58, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_EntityEntry__g0__t58(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.EntityEntry__g0__t58, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.EntityEntry__g0__t58, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_EntityEntry__g0__t58(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Profile__g0__t92(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Profile__g0__t92(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Profile__g0__t92, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Profile__g0__t92(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Profile__g0__t92, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Profile__g0__t92, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Profile__g0__t92(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MvcJsonOptions__g0__t114(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.MvcJsonOptions__g0__t114, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_JsonSerializerOptions__g0__t102(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MvcJsonOptions__g0__t114(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MvcJsonOptions__g0__t114, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MvcJsonOptions__g0__t114(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MvcJsonOptions__g0__t114, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MvcJsonOptions__g0__t114, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MvcJsonOptions__g0__t114(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_RuleBuilder_T_object___g0__t100(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_RuleBuilder_T_object___g0__t100(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_RuleBuilder_T_object___g0__t100(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_RuleBuilder_T_object___g0__t100(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IMappingExpression_TSource_TDestination___g0__t90(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IMappingExpression_TSource_TDestination___g0__t90(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IMappingExpression_TSource_TDestination___g0__t90, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IMappingExpression_TSource_TDestination___g0__t90(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IMappingExpression_TSource_TDestination___g0__t90, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IMappingExpression_TSource_TDestination___g0__t90, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IMappingExpression_TSource_TDestination___g0__t90(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_CancellationToken__g0__t45(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %destroy_object
destroy_object:
  call void @glitch_free(ptr %object)
  br label %done
done:
  ret void
}

define void @glitch_destroy_TokenValidationParameters__g0__t75(ptr %object) {
entry:
  %field_7_ptr = getelementptr inbounds %glitch.TokenValidationParameters__g0__t75, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_string_release(ptr %field_7)
  %field_9_ptr = getelementptr inbounds %glitch.TokenValidationParameters__g0__t75, ptr %object, i32 0, i32 9
  %field_9 = load ptr, ptr %field_9_ptr
  call void @glitch_drop_SymmetricSecurityKey__g0__t73(ptr %field_9)
  %field_8_ptr = getelementptr inbounds %glitch.TokenValidationParameters__g0__t75, ptr %object, i32 0, i32 8
  %field_8 = load ptr, ptr %field_8_ptr
  call void @glitch_string_release(ptr %field_8)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_TokenValidationParameters__g0__t75(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.TokenValidationParameters__g0__t75, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_TokenValidationParameters__g0__t75(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.TokenValidationParameters__g0__t75, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.TokenValidationParameters__g0__t75, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_TokenValidationParameters__g0__t75(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_JsonSerializer__g0__t103(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_JsonSerializer__g0__t103(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.JsonSerializer__g0__t103, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_JsonSerializer__g0__t103(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.JsonSerializer__g0__t103, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.JsonSerializer__g0__t103, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_JsonSerializer__g0__t103(ptr %object)
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

define void @glitch_destroy_DbContextOptions__g0__t49(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbContextOptions__g0__t49(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbContextOptions__g0__t49(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbContextOptions__g0__t49(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IMemberConfigurationExpression__g0__t88(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IMemberConfigurationExpression__g0__t88(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IMemberConfigurationExpression__g0__t88, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IMemberConfigurationExpression__g0__t88(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IMemberConfigurationExpression__g0__t88, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IMemberConfigurationExpression__g0__t88, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IMemberConfigurationExpression__g0__t88(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbContext__g0__t60(ptr %object) {
entry:
  %field_7_ptr = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_ChangeTracker__g0__t54(ptr %field_7)
  %field_4_ptr = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t318 = icmp eq ptr %field_4, null
  br i1 %t318, label %collection_release_done_139, label %collection_release_138
collection_release_138:
  %t319 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 0
  %t320 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 2
  %t321 = load i64, ptr %t319
  %t322 = load ptr, ptr %t320
  %t323 = alloca i64
  store i64 0, ptr %t323
  br label %element_drop_loop_140
element_drop_loop_140:
  %t324 = load i64, ptr %t323
  %t325 = icmp ult i64 %t324, %t321
  br i1 %t325, label %element_drop_body_141, label %element_drop_done_142
element_drop_body_141:
  %t326 = getelementptr inbounds ptr, ptr %t322, i64 %t324
  %t327 = load ptr, ptr %t326
  call void @glitch_string_release(ptr %t327)
  %t328 = add i64 %t324, 1
  store i64 %t328, ptr %t323
  br label %element_drop_loop_140
element_drop_done_142:
  call void @glitch_free(ptr %t322)
  call void @glitch_free(ptr %field_4)
  br label %collection_release_done_139
collection_release_done_139:
  %field_5_ptr = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_SqlProvider__g0__t50(ptr %field_5)
  %field_6_ptr = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_DatabaseFacade__g0__t56(ptr %field_6)
  %field_2_ptr = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbContext__g0__t60(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbContext__g0__t60(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbContext__g0__t60(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_WebApplicationBuilder__g0__t129(ptr %object) {
entry:
  %field_5_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_LoggingBuilder__g0__t128(ptr %field_5)
  %field_4_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_HostEnvironment__g0__t127(ptr %field_4)
  %field_2_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ServiceCollection__g0__t125(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_ConfigurationManager__g0__t126(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_WebApplicationBuilder__g0__t129(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_WebApplicationBuilder__g0__t129(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_WebApplicationBuilder__g0__t129(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ModelStateEntry__g0__t151(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ModelStateEntry__g0__t151, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t329 = icmp eq ptr %field_2, null
  br i1 %t329, label %collection_release_done_144, label %collection_release_143
collection_release_143:
  %t330 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t331 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t332 = load i64, ptr %t330
  %t333 = load ptr, ptr %t331
  %t334 = alloca i64
  store i64 0, ptr %t334
  br label %element_drop_loop_145
element_drop_loop_145:
  %t335 = load i64, ptr %t334
  %t336 = icmp ult i64 %t335, %t332
  br i1 %t336, label %element_drop_body_146, label %element_drop_done_147
element_drop_body_146:
  %t337 = getelementptr inbounds ptr, ptr %t333, i64 %t335
  %t338 = load ptr, ptr %t337
  call void @glitch_drop_ModelError__g0__t150(ptr %t338)
  %t339 = add i64 %t335, 1
  store i64 %t339, ptr %t334
  br label %element_drop_loop_145
element_drop_done_147:
  call void @glitch_free(ptr %t333)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_144
collection_release_done_144:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ModelStateEntry__g0__t151(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ModelStateEntry__g0__t151, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ModelStateEntry__g0__t151(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ModelStateEntry__g0__t151, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ModelStateEntry__g0__t151, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ModelStateEntry__g0__t151(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IQueryableString__g0__t61(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IQueryableString__g0__t61(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IQueryableString__g0__t61(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IQueryableString__g0__t61(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MvcConventions__g0__t112(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MvcConventions__g0__t112(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MvcConventions__g0__t112, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MvcConventions__g0__t112(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MvcConventions__g0__t112, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MvcConventions__g0__t112, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MvcConventions__g0__t112(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MvcBuilder__g0__t123(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MvcBuilder__g0__t123(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MvcBuilder__g0__t123(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MvcBuilder__g0__t123(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HttpContextAccessor__g0__t136(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.HttpContextAccessor__g0__t136, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_HttpContext__g0__t134(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HttpContextAccessor__g0__t136(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HttpContextAccessor__g0__t136, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HttpContextAccessor__g0__t136(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HttpContextAccessor__g0__t136, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HttpContextAccessor__g0__t136, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HttpContextAccessor__g0__t136(ptr %object)
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

define void @glitch_destroy_SwaggerGenOptions__g0__t118(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SwaggerGenOptions__g0__t118(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SwaggerGenOptions__g0__t118, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SwaggerGenOptions__g0__t118(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SwaggerGenOptions__g0__t118, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SwaggerGenOptions__g0__t118, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SwaggerGenOptions__g0__t118(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ILoggerFactory__g0__t70(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ILoggerFactory__g0__t70(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ILoggerFactory__g0__t70, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ILoggerFactory__g0__t70(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ILoggerFactory__g0__t70, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ILoggerFactory__g0__t70, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ILoggerFactory__g0__t70(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_PropertyBuilder__g0__t66(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_PropertyBuilder__g0__t66(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.PropertyBuilder__g0__t66, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_PropertyBuilder__g0__t66(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.PropertyBuilder__g0__t66, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.PropertyBuilder__g0__t66, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_PropertyBuilder__g0__t66(ptr %object)
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

define ptr @Task__g0__t46_CompletedTask__g0() {
entry:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t46_Run__g0__fn(ptr %function) {
entry:
  %t0 = alloca ptr
  store ptr %function, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  %t3 = load ptr, ptr %t2
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = call ptr %t3(ptr %t5)
  %t7 = call ptr @Task__g0__t46_CompletedTask__g0()
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t7
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t46_Run__g1__fn_ret_T(ptr %function) {
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

define void @Task__g0__t46_Wait__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @Task__g0__t46_GetAwaiter__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  store ptr null, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define void @Task__g0__t46_GetResult__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @Task__g0__t46_WhenAll__g0__array_task_void(ptr %tasks) {
entry:
  %t0 = alloca ptr
  store ptr %tasks, ptr %t0
  %t1 = call ptr @Task__g0__t46_CompletedTask__g0()
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t46_WhenAll__g1__array_task_T(ptr %tasks) {
entry:
  %t0 = alloca ptr
  store ptr %tasks, ptr %t0
  %t1 = call ptr @Task__g0__t46_CompletedTask__g0()
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t46_FromResult__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t46_WhenAll__g0__task_T_task_T(ptr %first, ptr %second) {
entry:
  %t0 = alloca ptr
  store ptr %first, ptr %t0
  %t1 = alloca ptr
  store ptr %second, ptr %t1
  %t2 = call ptr @Task__g0__t46_CompletedTask__g0()
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define void @ValueTask__g0__t47_Wait__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @ValueTask__g0__t47_GetAwaiter__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  store ptr null, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @ValueTask__g0__t47_FromResult__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ValueTask__g0__t47_GetResult__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ValueTask__g0__t47, ptr %t1, i32 0, i32 4
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @ValueTask__g0__t47_AsTask__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ValueTask__g0__t47, ptr %t1, i32 0, i32 4
  %t3 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @DbContextOptions__g0__t49_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.8, i32 0, i32 2, i64 0), ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @SqlProvider__g0__t50_ctor(ptr %this, ptr %name, ptr %connectionString) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %connectionString, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t9)
  %t10 = load ptr, ptr %t8
  call void @glitch_string_release(ptr %t10)
  store ptr %t9, ptr %t8
  ret void
exception_unwind:
  ret void
}

define ptr @SqlProvider__g0__t50_BuildSelectAll__g0(ptr %this, ptr %table) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %table, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [15 x i8] }, ptr @.str.9, i32 0, i32 2, i64 0), ptr %t2
  %t4 = load ptr, ptr %t2
  %t5 = load ptr, ptr %t1
  %t6 = call ptr @glitch_string_concat(ptr %t4, ptr %t5)
  %t7 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t7)
  ret ptr %t6
exception_unwind:
  %t8 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t8)
  ret ptr null
}

define ptr @DbSet__g1__t51_AsQueryable__g0(ptr %this) {
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
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t13, i32 0, i32 0
  %t15 = load ptr, ptr %t14
  %t16 = icmp ne ptr %t15, null
  br i1 %t16, label %if_then_2, label %if_else_3
if_then_2:
  %t17 = trunc i64 0 to i32
  store i32 %t17, ptr %t2
  br label %while_cond_5
while_cond_5:
  %t18 = load i32, ptr %t2
  %t19 = icmp slt i32 %t18, 0
  br i1 %t19, label %while_body_6, label %while_end_7
while_body_6:
  %t20 = load ptr, ptr %t1
  %t21 = load ptr, ptr %t0
  %t22 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t21, i32 0, i32 0
  %t23 = load ptr, ptr %t22
  %t24 = load i32, ptr %t2
  %t25 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 0
  %t26 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 1
  %t27 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 2
  %t28 = load i64, ptr %t25
  %t29 = load i64, ptr %t26
  %t30 = load ptr, ptr %t27
  %t31 = icmp eq i64 %t28, %t29
  br i1 %t31, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t32 = mul i64 %t29, 2
  %t33 = mul i64 %t32, 8
  %t34 = call ptr @glitch_realloc(ptr %t30, i64 %t33)
  store i64 %t32, ptr %t26
  store ptr %t34, ptr %t27
  br label %list_ready_9
list_ready_9:
  %t35 = load ptr, ptr %t27
  %t36 = getelementptr inbounds ptr, ptr %t35, i64 %t28
  store ptr null, ptr %t36
  %t37 = add i64 %t28, 1
  store i64 %t37, ptr %t25
  %t38 = load i32, ptr %t2
  %t39 = trunc i64 1 to i32
  %t40 = add i32 %t38, %t39
  store i32 %t40, ptr %t2
  br label %while_cond_5
while_end_7:
  br label %if_end_4
if_else_3:
  br label %if_end_4
if_end_4:
  %t41 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t42 = ptrtoint ptr %t41 to i64
  %t43 = call ptr @glitch_calloc(i64 1, i64 %t42)
  %t44 = load ptr, ptr %t1
  %t45 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t43, i32 0, i32 0
  store ptr %t44, ptr %t45
  %t46 = load ptr, ptr %t1
  %t47 = icmp eq ptr %t46, null
  br i1 %t47, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t48 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 0
  %t49 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 2
  %t50 = load i64, ptr %t48
  %t51 = load ptr, ptr %t49
  call void @glitch_free(ptr %t51)
  call void @glitch_free(ptr %t46)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr %t43
exception_unwind:
  %t52 = load ptr, ptr %t1
  %t53 = icmp eq ptr %t52, null
  br i1 %t53, label %collection_release_done_13, label %collection_release_12
collection_release_12:
  %t54 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 0
  %t55 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 2
  %t56 = load i64, ptr %t54
  %t57 = load ptr, ptr %t55
  call void @glitch_free(ptr %t57)
  call void @glitch_free(ptr %t52)
  br label %collection_release_done_13
collection_release_done_13:
  ret ptr null
}

define ptr @DbSet__g1__t51_AsNoTracking__g0(ptr %this) {
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
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t13, i32 0, i32 0
  %t15 = load ptr, ptr %t14
  %t16 = icmp ne ptr %t15, null
  br i1 %t16, label %if_then_2, label %if_else_3
if_then_2:
  %t17 = trunc i64 0 to i32
  store i32 %t17, ptr %t2
  br label %while_cond_5
while_cond_5:
  %t18 = load i32, ptr %t2
  %t19 = icmp slt i32 %t18, 0
  br i1 %t19, label %while_body_6, label %while_end_7
while_body_6:
  %t20 = load ptr, ptr %t1
  %t21 = load ptr, ptr %t0
  %t22 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t21, i32 0, i32 0
  %t23 = load ptr, ptr %t22
  %t24 = load i32, ptr %t2
  %t25 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 0
  %t26 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 1
  %t27 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 2
  %t28 = load i64, ptr %t25
  %t29 = load i64, ptr %t26
  %t30 = load ptr, ptr %t27
  %t31 = icmp eq i64 %t28, %t29
  br i1 %t31, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t32 = mul i64 %t29, 2
  %t33 = mul i64 %t32, 8
  %t34 = call ptr @glitch_realloc(ptr %t30, i64 %t33)
  store i64 %t32, ptr %t26
  store ptr %t34, ptr %t27
  br label %list_ready_9
list_ready_9:
  %t35 = load ptr, ptr %t27
  %t36 = getelementptr inbounds ptr, ptr %t35, i64 %t28
  store ptr null, ptr %t36
  %t37 = add i64 %t28, 1
  store i64 %t37, ptr %t25
  %t38 = load i32, ptr %t2
  %t39 = trunc i64 1 to i32
  %t40 = add i32 %t38, %t39
  store i32 %t40, ptr %t2
  br label %while_cond_5
while_end_7:
  br label %if_end_4
if_else_3:
  br label %if_end_4
if_end_4:
  %t41 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t42 = ptrtoint ptr %t41 to i64
  %t43 = call ptr @glitch_calloc(i64 1, i64 %t42)
  %t44 = load ptr, ptr %t1
  %t45 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t43, i32 0, i32 0
  store ptr %t44, ptr %t45
  %t46 = load ptr, ptr %t1
  %t47 = icmp eq ptr %t46, null
  br i1 %t47, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t48 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 0
  %t49 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 2
  %t50 = load i64, ptr %t48
  %t51 = load ptr, ptr %t49
  call void @glitch_free(ptr %t51)
  call void @glitch_free(ptr %t46)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr %t43
exception_unwind:
  %t52 = load ptr, ptr %t1
  %t53 = icmp eq ptr %t52, null
  br i1 %t53, label %collection_release_done_13, label %collection_release_12
collection_release_12:
  %t54 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 0
  %t55 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 2
  %t56 = load i64, ptr %t54
  %t57 = load ptr, ptr %t55
  call void @glitch_free(ptr %t57)
  call void @glitch_free(ptr %t52)
  br label %collection_release_done_13
collection_release_done_13:
  ret ptr null
}

define ptr @DbSet__g1__t51_Where__g0(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
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
  %t14 = load ptr, ptr %t0
  %t15 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t14, i32 0, i32 0
  %t16 = load ptr, ptr %t15
  %t17 = icmp ne ptr %t16, null
  br i1 %t17, label %if_then_2, label %if_else_3
if_then_2:
  %t18 = trunc i64 0 to i32
  store i32 %t18, ptr %t3
  br label %while_cond_5
while_cond_5:
  %t19 = load i32, ptr %t3
  %t20 = icmp slt i32 %t19, 0
  br i1 %t20, label %while_body_6, label %while_end_7
while_body_6:
  %t21 = load ptr, ptr %t2
  %t22 = load ptr, ptr %t0
  %t23 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t22, i32 0, i32 0
  %t24 = load ptr, ptr %t23
  %t25 = load i32, ptr %t3
  %t26 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 0
  %t27 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 1
  %t28 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 2
  %t29 = load i64, ptr %t26
  %t30 = load i64, ptr %t27
  %t31 = load ptr, ptr %t28
  %t32 = icmp eq i64 %t29, %t30
  br i1 %t32, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t33 = mul i64 %t30, 2
  %t34 = mul i64 %t33, 8
  %t35 = call ptr @glitch_realloc(ptr %t31, i64 %t34)
  store i64 %t33, ptr %t27
  store ptr %t35, ptr %t28
  br label %list_ready_9
list_ready_9:
  %t36 = load ptr, ptr %t28
  %t37 = getelementptr inbounds ptr, ptr %t36, i64 %t29
  store ptr null, ptr %t37
  %t38 = add i64 %t29, 1
  store i64 %t38, ptr %t26
  %t39 = load i32, ptr %t3
  %t40 = trunc i64 1 to i32
  %t41 = add i32 %t39, %t40
  store i32 %t41, ptr %t3
  br label %while_cond_5
while_end_7:
  br label %if_end_4
if_else_3:
  br label %if_end_4
if_end_4:
  %t42 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t43 = ptrtoint ptr %t42 to i64
  %t44 = call ptr @glitch_calloc(i64 1, i64 %t43)
  %t45 = load ptr, ptr %t2
  %t46 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t44, i32 0, i32 0
  store ptr %t45, ptr %t46
  %t47 = load ptr, ptr %t1
  call void @glitch_delegate_retain(ptr %t47)
  %t48 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t44, i32 0, i32 1
  store ptr %t47, ptr %t48
  %t49 = load ptr, ptr %t2
  %t50 = icmp eq ptr %t49, null
  br i1 %t50, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t51 = getelementptr inbounds %glitch.list, ptr %t49, i32 0, i32 0
  %t52 = getelementptr inbounds %glitch.list, ptr %t49, i32 0, i32 2
  %t53 = load i64, ptr %t51
  %t54 = load ptr, ptr %t52
  call void @glitch_free(ptr %t54)
  call void @glitch_free(ptr %t49)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr %t44
exception_unwind:
  %t55 = load ptr, ptr %t2
  %t56 = icmp eq ptr %t55, null
  br i1 %t56, label %collection_release_done_13, label %collection_release_12
collection_release_12:
  %t57 = getelementptr inbounds %glitch.list, ptr %t55, i32 0, i32 0
  %t58 = getelementptr inbounds %glitch.list, ptr %t55, i32 0, i32 2
  %t59 = load i64, ptr %t57
  %t60 = load ptr, ptr %t58
  call void @glitch_free(ptr %t60)
  call void @glitch_free(ptr %t55)
  br label %collection_release_done_13
collection_release_done_13:
  ret ptr null
}

define void @DbSet__g1__t51_Add__g0(ptr %this, ptr %entity) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t3, i32 0, i32 0
  %t5 = load ptr, ptr %t4
  %t6 = icmp eq ptr %t5, null
  br i1 %t6, label %if_then_0, label %if_else_1
if_then_0:
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t7, i32 0, i32 0
  %t9 = call ptr @glitch_calloc(i64 1, i64 24)
  %t10 = call ptr @glitch_calloc(i64 4, i64 8)
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 1
  store i64 4, ptr %t11
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  store ptr %t10, ptr %t12
  store ptr %t9, ptr %t8
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t13, i32 0, i32 0
  %t15 = load ptr, ptr %t14
  store ptr %t15, ptr %t2
  %t16 = load ptr, ptr %t2
  %t17 = load ptr, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @DbSet__g1__t51_Clear__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t1, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  %t4 = icmp eq ptr %t3, null
  br i1 %t4, label %if_then_0, label %if_else_1
if_then_0:
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t5, i32 0, i32 0
  %t7 = call ptr @glitch_calloc(i64 1, i64 24)
  %t8 = call ptr @glitch_calloc(i64 4, i64 8)
  %t9 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 1
  store i64 4, ptr %t9
  %t10 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 2
  store ptr %t8, ptr %t10
  store ptr %t7, ptr %t6
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t11, i32 0, i32 0
  %t13 = load ptr, ptr %t12
  ret void
exception_unwind:
  ret void
}

define ptr @DbSet__g1__t51_AddAsync__g0__T_CancellationToken(ptr %this, ptr %entity, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AddAsync__g0__T(ptr %this, ptr %entity) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AddRangeAsync__g0__ienumerable_T_CancellationToken(ptr %this, ptr %entities, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entities, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 0
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  %t7 = load i64, ptr %t5
  %t8 = load ptr, ptr %t6
  %t9 = alloca i64
  %t10 = alloca ptr
  store i64 0, ptr %t9
  br label %foreach_condition_0
foreach_condition_0:
  %t11 = load i64, ptr %t9
  %t12 = icmp ult i64 %t11, %t7
  br i1 %t12, label %foreach_body_1, label %foreach_end_3
foreach_body_1:
  %t13 = getelementptr inbounds ptr, ptr %t8, i64 %t11
  %t14 = load ptr, ptr %t13
  store ptr %t14, ptr %t10
  %t15 = load ptr, ptr %t0
  %t16 = load ptr, ptr %t10
  br label %foreach_advance_2
foreach_advance_2:
  %t17 = load i64, ptr %t9
  %t18 = add i64 %t17, 1
  store i64 %t18, ptr %t9
  br label %foreach_condition_0
foreach_end_3:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AddRangeAsync__g0__list_T_CancellationToken(ptr %this, ptr %entities, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entities, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = trunc i64 0 to i32
  store i32 %t4, ptr %t3
  br label %while_cond_0
while_cond_0:
  %t5 = load i32, ptr %t3
  %t6 = load ptr, ptr %t1
  %t7 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 0
  %t8 = load i64, ptr %t7
  %t9 = trunc i64 %t8 to i32
  %t10 = icmp slt i32 %t5, %t9
  br i1 %t10, label %while_body_1, label %while_end_2
while_body_1:
  %t11 = load ptr, ptr %t0
  %t12 = load ptr, ptr %t1
  %t13 = load i32, ptr %t3
  %t14 = sext i32 %t13 to i64
  %t15 = getelementptr inbounds %glitch.list, ptr %t12, i32 0, i32 2
  %t16 = load ptr, ptr %t15
  %t17 = getelementptr inbounds ptr, ptr %t16, i64 %t14
  %t18 = load ptr, ptr %t17
  %t19 = load i32, ptr %t3
  %t20 = trunc i64 1 to i32
  %t21 = add i32 %t19, %t20
  store i32 %t21, ptr %t3
  br label %while_cond_0
while_end_2:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AnyAsync__g0__CancellationToken(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_drop_object__g0__t14(ptr null)
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AnyAsync__g0__fn_T_ret_bool_CancellationToken(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  call void @glitch_drop_object__g0__t14(ptr null)
  %t5 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FirstAsync__g0__CancellationToken(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_drop_object__g0__t14(ptr null)
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FirstAsync__g0__fn_T_ret_bool_CancellationToken(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  call void @glitch_drop_object__g0__t14(ptr null)
  %t5 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FirstOrDefaultAsync__g0__CancellationToken(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @FirstOrDefaultAsync__object(ptr %t2)
  %t4 = load ptr, ptr @glitch_exception_pending
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FirstOrDefaultAsync__g0__fn_T_ret_bool_CancellationToken(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @FirstOrDefaultAsync__object(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_SingleOrDefaultAsync__g0__CancellationToken(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @SingleOrDefaultAsync__object(ptr %t2)
  %t4 = load ptr, ptr @glitch_exception_pending
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_SingleOrDefaultAsync__g0__fn_T_ret_bool_CancellationToken(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @SingleOrDefaultAsync__object(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_ToListAsync__g0(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_drop_object__g0__t14(ptr null)
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FindAsync__g0(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_Snapshot__g0(ptr %this) {
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
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t13, i32 0, i32 0
  %t15 = load ptr, ptr %t14
  %t16 = icmp eq ptr %t15, null
  br i1 %t16, label %if_then_2, label %if_else_3
if_then_2:
  %t17 = load ptr, ptr %t1
  ret ptr %t17
if_else_3:
  br label %if_end_4
if_end_4:
  %t18 = trunc i64 0 to i32
  store i32 %t18, ptr %t2
  br label %while_cond_5
while_cond_5:
  %t19 = load i32, ptr %t2
  %t20 = icmp slt i32 %t19, 0
  br i1 %t20, label %while_body_6, label %while_end_7
while_body_6:
  %t21 = load ptr, ptr %t1
  %t22 = load ptr, ptr %t0
  %t23 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t22, i32 0, i32 0
  %t24 = load ptr, ptr %t23
  %t25 = load i32, ptr %t2
  %t26 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 0
  %t27 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 1
  %t28 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 2
  %t29 = load i64, ptr %t26
  %t30 = load i64, ptr %t27
  %t31 = load ptr, ptr %t28
  %t32 = icmp eq i64 %t29, %t30
  br i1 %t32, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t33 = mul i64 %t30, 2
  %t34 = mul i64 %t33, 8
  %t35 = call ptr @glitch_realloc(ptr %t31, i64 %t34)
  store i64 %t33, ptr %t27
  store ptr %t35, ptr %t28
  br label %list_ready_9
list_ready_9:
  %t36 = load ptr, ptr %t28
  %t37 = getelementptr inbounds ptr, ptr %t36, i64 %t29
  store ptr null, ptr %t37
  %t38 = add i64 %t29, 1
  store i64 %t38, ptr %t26
  %t39 = load i32, ptr %t2
  %t40 = trunc i64 1 to i32
  %t41 = add i32 %t39, %t40
  store i32 %t41, ptr %t2
  br label %while_cond_5
while_end_7:
  %t42 = load ptr, ptr %t1
  ret ptr %t42
exception_unwind:
  %t43 = load ptr, ptr %t1
  %t44 = icmp eq ptr %t43, null
  br i1 %t44, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t45 = getelementptr inbounds %glitch.list, ptr %t43, i32 0, i32 0
  %t46 = getelementptr inbounds %glitch.list, ptr %t43, i32 0, i32 2
  %t47 = load i64, ptr %t45
  %t48 = load ptr, ptr %t46
  call void @glitch_free(ptr %t48)
  call void @glitch_free(ptr %t43)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr null
}

define ptr @DbQuery__g1__t52_RowsSnapshot__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t1, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define i1 @DbQuery__g1__t52_Matches__g0(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t2, i32 0, i32 1
  %t4 = load ptr, ptr %t3
  %t5 = icmp ne ptr %t4, null
  %t6 = load ptr, ptr %t0
  %t7 = load ptr, ptr %t1
  %t8 = icmp ne ptr null, null
  %t9 = xor i1 %t8, true
  %t10 = and i1 %t5, %t9
  br i1 %t10, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 0
if_else_1:
  br label %if_end_2
if_end_2:
  ret i1 1
exception_unwind:
  ret i1 0
}

define ptr @DbQuery__g1__t52_AsQueryable__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = call ptr @glitch_calloc(i64 1, i64 24)
  %t5 = call ptr @glitch_calloc(i64 4, i64 8)
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 1
  store i64 4, ptr %t6
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  store ptr %t5, ptr %t7
  %t8 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t3, i32 0, i32 0
  store ptr %t4, ptr %t8
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t9, i32 0, i32 1
  %t11 = load ptr, ptr %t10
  call void @glitch_delegate_retain(ptr %t11)
  %t12 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t3, i32 0, i32 1
  store ptr %t11, ptr %t12
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_AsNoTracking__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = call ptr @glitch_calloc(i64 1, i64 24)
  %t5 = call ptr @glitch_calloc(i64 4, i64 8)
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 1
  store i64 4, ptr %t6
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  store ptr %t5, ptr %t7
  %t8 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t3, i32 0, i32 0
  store ptr %t4, ptr %t8
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t9, i32 0, i32 1
  %t11 = load ptr, ptr %t10
  call void @glitch_delegate_retain(ptr %t11)
  %t12 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t3, i32 0, i32 1
  store ptr %t11, ptr %t12
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_Include__g0(ptr %this, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  %t9 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 0
  store ptr %t5, ptr %t9
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t10, i32 0, i32 1
  %t12 = load ptr, ptr %t11
  call void @glitch_delegate_retain(ptr %t12)
  %t13 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 1
  store ptr %t12, ptr %t13
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_ThenInclude__g0(ptr %this, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  %t9 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 0
  store ptr %t5, ptr %t9
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t10, i32 0, i32 1
  %t12 = load ptr, ptr %t11
  call void @glitch_delegate_retain(ptr %t12)
  %t13 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 1
  store ptr %t12, ptr %t13
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_Where__g0(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  %t9 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 0
  store ptr %t5, ptr %t9
  %t10 = load ptr, ptr %t1
  call void @glitch_delegate_retain(ptr %t10)
  %t11 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 1
  store ptr %t10, ptr %t11
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_AnyAsync__g0(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = trunc i64 0 to i32
  store i32 %t4, ptr %t2
  %t5 = load ptr, ptr %t0
  %t6 = call ptr @DbQuery__g1__t52_RowsSnapshot__g0(ptr %t5)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t6, ptr %t3
  br label %while_cond_1
while_cond_1:
  %t9 = load i32, ptr %t2
  %t10 = load ptr, ptr %t3
  %t11 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 0
  %t12 = load i64, ptr %t11
  %t13 = trunc i64 %t12 to i32
  %t14 = icmp slt i32 %t9, %t13
  br i1 %t14, label %while_body_2, label %while_end_3
while_body_2:
  %t15 = load ptr, ptr %t0
  %t16 = load ptr, ptr %t3
  %t17 = load i32, ptr %t2
  %t18 = sext i32 %t17 to i64
  %t19 = getelementptr inbounds %glitch.list, ptr %t16, i32 0, i32 2
  %t20 = load ptr, ptr %t19
  %t21 = getelementptr inbounds ptr, ptr %t20, i64 %t18
  %t22 = load ptr, ptr %t21
  %t23 = icmp ne ptr null, null
  br i1 %t23, label %if_then_4, label %if_else_5
if_then_4:
  %t24 = load ptr, ptr %t3
  %t25 = icmp eq ptr %t24, null
  br i1 %t25, label %collection_release_done_8, label %collection_release_7
collection_release_7:
  %t26 = getelementptr inbounds %glitch.list, ptr %t24, i32 0, i32 0
  %t27 = getelementptr inbounds %glitch.list, ptr %t24, i32 0, i32 2
  %t28 = load i64, ptr %t26
  %t29 = load ptr, ptr %t27
  call void @glitch_free(ptr %t29)
  call void @glitch_free(ptr %t24)
  br label %collection_release_done_8
collection_release_done_8:
  ret ptr null
if_else_5:
  br label %if_end_6
if_end_6:
  %t30 = load i32, ptr %t2
  %t31 = trunc i64 1 to i32
  %t32 = add i32 %t30, %t31
  store i32 %t32, ptr %t2
  br label %while_cond_1
while_end_3:
  %t33 = load ptr, ptr %t3
  %t34 = icmp eq ptr %t33, null
  br i1 %t34, label %collection_release_done_10, label %collection_release_9
collection_release_9:
  %t35 = getelementptr inbounds %glitch.list, ptr %t33, i32 0, i32 0
  %t36 = getelementptr inbounds %glitch.list, ptr %t33, i32 0, i32 2
  %t37 = load i64, ptr %t35
  %t38 = load ptr, ptr %t36
  call void @glitch_free(ptr %t38)
  call void @glitch_free(ptr %t33)
  br label %collection_release_done_10
collection_release_done_10:
  ret ptr null
exception_unwind:
  %t39 = load ptr, ptr %t3
  %t40 = icmp eq ptr %t39, null
  br i1 %t40, label %collection_release_done_12, label %collection_release_11
collection_release_11:
  %t41 = getelementptr inbounds %glitch.list, ptr %t39, i32 0, i32 0
  %t42 = getelementptr inbounds %glitch.list, ptr %t39, i32 0, i32 2
  %t43 = load i64, ptr %t41
  %t44 = load ptr, ptr %t42
  call void @glitch_free(ptr %t44)
  call void @glitch_free(ptr %t39)
  br label %collection_release_done_12
collection_release_done_12:
  ret ptr null
}

define ptr @DbQuery__g1__t52_FirstAsync__g0__CancellationToken(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = trunc i64 0 to i32
  store i32 %t5, ptr %t2
  %t6 = load ptr, ptr %t0
  %t7 = call ptr @DbQuery__g1__t52_RowsSnapshot__g0(ptr %t6)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t7, ptr %t3
  br label %while_cond_1
while_cond_1:
  %t10 = load i32, ptr %t2
  %t11 = load ptr, ptr %t3
  %t12 = getelementptr inbounds %glitch.list, ptr %t11, i32 0, i32 0
  %t13 = load i64, ptr %t12
  %t14 = trunc i64 %t13 to i32
  %t15 = icmp slt i32 %t10, %t14
  br i1 %t15, label %while_body_2, label %while_end_3
while_body_2:
  %t16 = load ptr, ptr %t3
  %t17 = load i32, ptr %t2
  %t18 = sext i32 %t17 to i64
  %t19 = getelementptr inbounds %glitch.list, ptr %t16, i32 0, i32 2
  %t20 = load ptr, ptr %t19
  %t21 = getelementptr inbounds ptr, ptr %t20, i64 %t18
  %t22 = load ptr, ptr %t21
  store ptr %t22, ptr %t4
  %t23 = load ptr, ptr %t0
  %t24 = load ptr, ptr %t4
  %t25 = icmp ne ptr null, null
  br i1 %t25, label %if_then_4, label %if_else_5
if_then_4:
  %t26 = load ptr, ptr %t4
  %t27 = load ptr, ptr %t3
  %t28 = icmp eq ptr %t27, null
  br i1 %t28, label %collection_release_done_8, label %collection_release_7
collection_release_7:
  %t29 = getelementptr inbounds %glitch.list, ptr %t27, i32 0, i32 0
  %t30 = getelementptr inbounds %glitch.list, ptr %t27, i32 0, i32 2
  %t31 = load i64, ptr %t29
  %t32 = load ptr, ptr %t30
  call void @glitch_free(ptr %t32)
  call void @glitch_free(ptr %t27)
  br label %collection_release_done_8
collection_release_done_8:
  ret ptr null
if_else_5:
  br label %if_end_6
if_end_6:
  %t33 = load i32, ptr %t2
  %t34 = trunc i64 1 to i32
  %t35 = add i32 %t33, %t34
  store i32 %t35, ptr %t2
  br label %while_cond_1
while_end_3:
  %t36 = load ptr, ptr %t3
  %t37 = icmp eq ptr %t36, null
  br i1 %t37, label %collection_release_done_10, label %collection_release_9
collection_release_9:
  %t38 = getelementptr inbounds %glitch.list, ptr %t36, i32 0, i32 0
  %t39 = getelementptr inbounds %glitch.list, ptr %t36, i32 0, i32 2
  %t40 = load i64, ptr %t38
  %t41 = load ptr, ptr %t39
  call void @glitch_free(ptr %t41)
  call void @glitch_free(ptr %t36)
  br label %collection_release_done_10
collection_release_done_10:
  ret ptr null
exception_unwind:
  %t42 = load ptr, ptr %t3
  %t43 = icmp eq ptr %t42, null
  br i1 %t43, label %collection_release_done_12, label %collection_release_11
collection_release_11:
  %t44 = getelementptr inbounds %glitch.list, ptr %t42, i32 0, i32 0
  %t45 = getelementptr inbounds %glitch.list, ptr %t42, i32 0, i32 2
  %t46 = load i64, ptr %t44
  %t47 = load ptr, ptr %t45
  call void @glitch_free(ptr %t47)
  call void @glitch_free(ptr %t42)
  br label %collection_release_done_12
collection_release_done_12:
  ret ptr null
}

define ptr @DbQuery__g1__t52_FirstAsync__g0__fn_T_ret_bool_CancellationToken(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  call void @glitch_drop_object__g0__t14(ptr null)
  %t5 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_FirstOrDefaultAsync__g0__CancellationToken(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_FirstOrDefaultAsync__g0__fn_T_ret_bool_CancellationToken(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @FirstOrDefaultAsync__object(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_SingleOrDefaultAsync__g0__CancellationToken(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca i1
  store i1 0, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = alloca ptr
  store ptr null, ptr %t6
  %t7 = trunc i64 0 to i32
  store i32 %t7, ptr %t2
  store ptr null, ptr %t3
  store i1 0, ptr %t4
  %t8 = load ptr, ptr %t0
  %t9 = call ptr @DbQuery__g1__t52_RowsSnapshot__g0(ptr %t8)
  %t10 = load ptr, ptr @glitch_exception_pending
  %t11 = icmp ne ptr %t10, null
  br i1 %t11, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t9, ptr %t5
  br label %while_cond_1
while_cond_1:
  %t12 = load i32, ptr %t2
  %t13 = load ptr, ptr %t5
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  %t15 = load i64, ptr %t14
  %t16 = trunc i64 %t15 to i32
  %t17 = icmp slt i32 %t12, %t16
  br i1 %t17, label %while_body_2, label %while_end_3
while_body_2:
  %t18 = load ptr, ptr %t5
  %t19 = load i32, ptr %t2
  %t20 = sext i32 %t19 to i64
  %t21 = getelementptr inbounds %glitch.list, ptr %t18, i32 0, i32 2
  %t22 = load ptr, ptr %t21
  %t23 = getelementptr inbounds ptr, ptr %t22, i64 %t20
  %t24 = load ptr, ptr %t23
  store ptr %t24, ptr %t6
  %t25 = load ptr, ptr %t0
  %t26 = load ptr, ptr %t6
  %t27 = icmp ne ptr null, null
  br i1 %t27, label %if_then_4, label %if_else_5
if_then_4:
  %t28 = load i1, ptr %t4
  br i1 %t28, label %if_then_7, label %if_else_8
if_then_7:
  %t29 = load ptr, ptr %t5
  %t30 = icmp eq ptr %t29, null
  br i1 %t30, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t31 = getelementptr inbounds %glitch.list, ptr %t29, i32 0, i32 0
  %t32 = getelementptr inbounds %glitch.list, ptr %t29, i32 0, i32 2
  %t33 = load i64, ptr %t31
  %t34 = load ptr, ptr %t32
  call void @glitch_free(ptr %t34)
  call void @glitch_free(ptr %t29)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr null
if_else_8:
  br label %if_end_9
if_end_9:
  %t35 = load ptr, ptr %t6
  store ptr %t35, ptr %t3
  store i1 1, ptr %t4
  br label %if_end_6
if_else_5:
  br label %if_end_6
if_end_6:
  %t36 = load i32, ptr %t2
  %t37 = trunc i64 1 to i32
  %t38 = add i32 %t36, %t37
  store i32 %t38, ptr %t2
  br label %while_cond_1
while_end_3:
  %t39 = load ptr, ptr %t3
  %t40 = load ptr, ptr %t5
  %t41 = icmp eq ptr %t40, null
  br i1 %t41, label %collection_release_done_13, label %collection_release_12
collection_release_12:
  %t42 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 0
  %t43 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 2
  %t44 = load i64, ptr %t42
  %t45 = load ptr, ptr %t43
  call void @glitch_free(ptr %t45)
  call void @glitch_free(ptr %t40)
  br label %collection_release_done_13
collection_release_done_13:
  ret ptr null
exception_unwind:
  %t46 = load ptr, ptr %t5
  %t47 = icmp eq ptr %t46, null
  br i1 %t47, label %collection_release_done_15, label %collection_release_14
collection_release_14:
  %t48 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 0
  %t49 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 2
  %t50 = load i64, ptr %t48
  %t51 = load ptr, ptr %t49
  call void @glitch_free(ptr %t51)
  call void @glitch_free(ptr %t46)
  br label %collection_release_done_15
collection_release_done_15:
  ret ptr null
}

define ptr @DbQuery__g1__t52_SingleOrDefaultAsync__g0__fn_T_ret_bool_CancellationToken(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @SingleOrDefaultAsync__object(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_ToListAsync__g0(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = call ptr @glitch_calloc(i64 1, i64 24)
  %t7 = call ptr @glitch_calloc(i64 4, i64 8)
  %t8 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 1
  store i64 4, ptr %t8
  %t9 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 2
  store ptr %t7, ptr %t9
  %t10 = load ptr, ptr %t2
  %t11 = icmp eq ptr %t10, null
  br i1 %t11, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t12 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 0
  %t13 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 2
  %t14 = load i64, ptr %t12
  %t15 = load ptr, ptr %t13
  call void @glitch_free(ptr %t15)
  call void @glitch_free(ptr %t10)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t6, ptr %t2
  %t16 = trunc i64 0 to i32
  store i32 %t16, ptr %t3
  %t17 = load ptr, ptr %t0
  %t18 = call ptr @DbQuery__g1__t52_RowsSnapshot__g0(ptr %t17)
  %t19 = load ptr, ptr @glitch_exception_pending
  %t20 = icmp ne ptr %t19, null
  br i1 %t20, label %exception_unwind, label %call_continue_2
call_continue_2:
  store ptr %t18, ptr %t4
  br label %while_cond_3
while_cond_3:
  %t21 = load i32, ptr %t3
  %t22 = load ptr, ptr %t4
  %t23 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 0
  %t24 = load i64, ptr %t23
  %t25 = trunc i64 %t24 to i32
  %t26 = icmp slt i32 %t21, %t25
  br i1 %t26, label %while_body_4, label %while_end_5
while_body_4:
  %t27 = load ptr, ptr %t4
  %t28 = load i32, ptr %t3
  %t29 = sext i32 %t28 to i64
  %t30 = getelementptr inbounds %glitch.list, ptr %t27, i32 0, i32 2
  %t31 = load ptr, ptr %t30
  %t32 = getelementptr inbounds ptr, ptr %t31, i64 %t29
  %t33 = load ptr, ptr %t32
  store ptr %t33, ptr %t5
  %t34 = load ptr, ptr %t0
  %t35 = load ptr, ptr %t5
  %t36 = icmp ne ptr null, null
  br i1 %t36, label %if_then_6, label %if_else_7
if_then_6:
  %t37 = load ptr, ptr %t2
  %t38 = load ptr, ptr %t5
  %t39 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 0
  %t40 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 1
  %t41 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 2
  %t42 = load i64, ptr %t39
  %t43 = load i64, ptr %t40
  %t44 = load ptr, ptr %t41
  %t45 = icmp eq i64 %t42, %t43
  br i1 %t45, label %list_grow_9, label %list_ready_10
list_grow_9:
  %t46 = mul i64 %t43, 2
  %t47 = mul i64 %t46, 8
  %t48 = call ptr @glitch_realloc(ptr %t44, i64 %t47)
  store i64 %t46, ptr %t40
  store ptr %t48, ptr %t41
  br label %list_ready_10
list_ready_10:
  %t49 = load ptr, ptr %t41
  %t50 = getelementptr inbounds ptr, ptr %t49, i64 %t42
  store ptr %t38, ptr %t50
  %t51 = add i64 %t42, 1
  store i64 %t51, ptr %t39
  br label %if_end_8
if_else_7:
  br label %if_end_8
if_end_8:
  %t52 = load i32, ptr %t3
  %t53 = trunc i64 1 to i32
  %t54 = add i32 %t52, %t53
  store i32 %t54, ptr %t3
  br label %while_cond_3
while_end_5:
  %t55 = load ptr, ptr %t2
  %t56 = load ptr, ptr %t4
  %t57 = icmp eq ptr %t56, null
  br i1 %t57, label %collection_release_done_12, label %collection_release_11
collection_release_11:
  %t58 = getelementptr inbounds %glitch.list, ptr %t56, i32 0, i32 0
  %t59 = getelementptr inbounds %glitch.list, ptr %t56, i32 0, i32 2
  %t60 = load i64, ptr %t58
  %t61 = load ptr, ptr %t59
  call void @glitch_free(ptr %t61)
  call void @glitch_free(ptr %t56)
  br label %collection_release_done_12
collection_release_done_12:
  %t62 = load ptr, ptr %t2
  %t63 = icmp eq ptr %t62, null
  br i1 %t63, label %collection_release_done_14, label %collection_release_13
collection_release_13:
  %t64 = getelementptr inbounds %glitch.list, ptr %t62, i32 0, i32 0
  %t65 = getelementptr inbounds %glitch.list, ptr %t62, i32 0, i32 2
  %t66 = load i64, ptr %t64
  %t67 = load ptr, ptr %t65
  call void @glitch_free(ptr %t67)
  call void @glitch_free(ptr %t62)
  br label %collection_release_done_14
collection_release_done_14:
  ret ptr null
exception_unwind:
  %t68 = load ptr, ptr %t4
  %t69 = icmp eq ptr %t68, null
  br i1 %t69, label %collection_release_done_16, label %collection_release_15
collection_release_15:
  %t70 = getelementptr inbounds %glitch.list, ptr %t68, i32 0, i32 0
  %t71 = getelementptr inbounds %glitch.list, ptr %t68, i32 0, i32 2
  %t72 = load i64, ptr %t70
  %t73 = load ptr, ptr %t71
  call void @glitch_free(ptr %t73)
  call void @glitch_free(ptr %t68)
  br label %collection_release_done_16
collection_release_done_16:
  %t74 = load ptr, ptr %t2
  %t75 = icmp eq ptr %t74, null
  br i1 %t75, label %collection_release_done_18, label %collection_release_17
collection_release_17:
  %t76 = getelementptr inbounds %glitch.list, ptr %t74, i32 0, i32 0
  %t77 = getelementptr inbounds %glitch.list, ptr %t74, i32 0, i32 2
  %t78 = load i64, ptr %t76
  %t79 = load ptr, ptr %t77
  call void @glitch_free(ptr %t79)
  call void @glitch_free(ptr %t74)
  br label %collection_release_done_18
collection_release_done_18:
  ret ptr null
}

define void @DbContextOptionsBuilder_ApplicationDbContext__g0__t53_UseSqlServer__g0(ptr %this, ptr %connectionString, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = alloca ptr
  store ptr %configure, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @ChangeTracker__g0__t54_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.DebugView__g0__t55, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.DebugView__g0__t55, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.DebugView__g0__t55, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_DebugView__g0__t55, ptr %t7
  %t8 = load ptr, ptr %t2
  call void @glitch_drop_DebugView__g0__t55(ptr %t8)
  store ptr %t5, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @ChangeTracker__g0__t54_DetectChanges__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ChangeTracker__g0__t54_Clear__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @DatabaseFacade__g0__t56_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.10, i32 0, i32 2, i64 0), ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @DatabaseFacade__g0__t56_ctor__string(ptr %this, ptr %connectionString) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define i1 @DatabaseFacade__g0__t56_EnsureCreated__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret i1 1
exception_unwind:
  ret i1 0
}

define i1 @DatabaseFacade__g0__t56_EnsureDeleted__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @DatabaseFacade__g0__t56_IsInMemory__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  %t4 = icmp eq ptr %t3, getelementptr inbounds ({ i64, i64, [9 x i8] }, ptr @.str.11, i32 0, i32 2, i64 0)
  ret i1 %t4
exception_unwind:
  ret i1 0
}

define ptr @DatabaseFacade__g0__t56_BeginTransaction__g0(ptr %this, ptr %isolationLevel) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %isolationLevel, ptr %t1
  %t2 = getelementptr %glitch.DatabaseTransaction__g0__t57, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.DatabaseTransaction__g0__t57, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.DatabaseTransaction__g0__t57, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_DatabaseTransaction__g0__t57, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @DatabaseFacade__g0__t56_Migrate__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define i32 @DatabaseFacade__g0__t56_ExecuteSqlRaw__g0(ptr %this, ptr %sql, ptr %parameters) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %sql, ptr %t1
  %t2 = alloca ptr
  store ptr %parameters, ptr %t2
  %t3 = trunc i64 0 to i32
  ret i32 %t3
exception_unwind:
  ret i32 0
}

define void @DatabaseTransaction__g0__t57_Commit__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @DatabaseTransaction__g0__t57_Rollback__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @DatabaseTransaction__g0__t57_Dispose__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t60_ctor__string(ptr %this, ptr %connectionString) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t6, i32 0, i32 3
  store i1 0, ptr %t7
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t8, i32 0, i32 4
  %t10 = call ptr @glitch_calloc(i64 1, i64 24)
  %t11 = call ptr @glitch_calloc(i64 4, i64 8)
  %t12 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 1
  store i64 4, ptr %t12
  %t13 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 2
  store ptr %t11, ptr %t13
  store ptr %t10, ptr %t9
  %t14 = load ptr, ptr %t0
  %t15 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t14, i32 0, i32 5
  %t16 = getelementptr %glitch.SqlProvider__g0__t50, ptr null, i32 1
  %t17 = ptrtoint ptr %t16 to i64
  %t18 = call ptr @glitch_calloc(i64 1, i64 %t17)
  %t19 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t18, i32 0, i32 0
  store i64 1, ptr %t19
  %t20 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t18, i32 0, i32 1
  store ptr @glitch_destroy_SqlProvider__g0__t50, ptr %t20
  %t21 = load ptr, ptr %t1
  call void @SqlProvider__g0__t50_ctor(ptr %t18, ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.12, i32 0, i32 2, i64 0), ptr %t21)
  %t22 = load ptr, ptr @glitch_exception_pending
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t24 = load ptr, ptr %t15
  call void @glitch_drop_SqlProvider__g0__t50(ptr %t24)
  store ptr %t18, ptr %t15
  %t25 = load ptr, ptr %t0
  %t26 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t25, i32 0, i32 6
  %t27 = getelementptr %glitch.DatabaseFacade__g0__t56, ptr null, i32 1
  %t28 = ptrtoint ptr %t27 to i64
  %t29 = call ptr @glitch_calloc(i64 1, i64 %t28)
  %t30 = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %t29, i32 0, i32 0
  store i64 1, ptr %t30
  %t31 = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %t29, i32 0, i32 1
  store ptr @glitch_destroy_DatabaseFacade__g0__t56, ptr %t31
  %t32 = load ptr, ptr %t1
  call void @DatabaseFacade__g0__t56_ctor__string(ptr %t29, ptr %t32)
  %t33 = load ptr, ptr @glitch_exception_pending
  %t34 = icmp ne ptr %t33, null
  br i1 %t34, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t35 = load ptr, ptr %t26
  call void @glitch_drop_DatabaseFacade__g0__t56(ptr %t35)
  store ptr %t29, ptr %t26
  %t36 = load ptr, ptr %t0
  %t37 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t36, i32 0, i32 7
  %t38 = getelementptr %glitch.ChangeTracker__g0__t54, ptr null, i32 1
  %t39 = ptrtoint ptr %t38 to i64
  %t40 = call ptr @glitch_calloc(i64 1, i64 %t39)
  %t41 = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %t40, i32 0, i32 0
  store i64 1, ptr %t41
  %t42 = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %t40, i32 0, i32 1
  store ptr @glitch_destroy_ChangeTracker__g0__t54, ptr %t42
  call void @ChangeTracker__g0__t54_ctor(ptr %t40)
  %t43 = load ptr, ptr @glitch_exception_pending
  %t44 = icmp ne ptr %t43, null
  br i1 %t44, label %exception_unwind, label %call_continue_2
call_continue_2:
  %t45 = load ptr, ptr %t37
  call void @glitch_drop_ChangeTracker__g0__t54(ptr %t45)
  store ptr %t40, ptr %t37
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t60_ctor__object(ptr %this, ptr %options) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %options, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t4)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.13, i32 0, i32 2, i64 0), ptr %t3
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t5, i32 0, i32 3
  store i1 0, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t7, i32 0, i32 4
  %t9 = call ptr @glitch_calloc(i64 1, i64 24)
  %t10 = call ptr @glitch_calloc(i64 4, i64 8)
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 1
  store i64 4, ptr %t11
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  store ptr %t10, ptr %t12
  store ptr %t9, ptr %t8
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t13, i32 0, i32 5
  %t15 = getelementptr %glitch.SqlProvider__g0__t50, ptr null, i32 1
  %t16 = ptrtoint ptr %t15 to i64
  %t17 = call ptr @glitch_calloc(i64 1, i64 %t16)
  %t18 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t17, i32 0, i32 0
  store i64 1, ptr %t18
  %t19 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t17, i32 0, i32 1
  store ptr @glitch_destroy_SqlProvider__g0__t50, ptr %t19
  call void @SqlProvider__g0__t50_ctor(ptr %t17, ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.14, i32 0, i32 2, i64 0), ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.15, i32 0, i32 2, i64 0))
  %t20 = load ptr, ptr @glitch_exception_pending
  %t21 = icmp ne ptr %t20, null
  br i1 %t21, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t22 = load ptr, ptr %t14
  call void @glitch_drop_SqlProvider__g0__t50(ptr %t22)
  store ptr %t17, ptr %t14
  %t23 = load ptr, ptr %t0
  %t24 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t23, i32 0, i32 6
  %t25 = getelementptr %glitch.DatabaseFacade__g0__t56, ptr null, i32 1
  %t26 = ptrtoint ptr %t25 to i64
  %t27 = call ptr @glitch_calloc(i64 1, i64 %t26)
  %t28 = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %t27, i32 0, i32 0
  store i64 1, ptr %t28
  %t29 = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %t27, i32 0, i32 1
  store ptr @glitch_destroy_DatabaseFacade__g0__t56, ptr %t29
  call void @DatabaseFacade__g0__t56_ctor__empty(ptr %t27)
  %t30 = load ptr, ptr @glitch_exception_pending
  %t31 = icmp ne ptr %t30, null
  br i1 %t31, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t32 = load ptr, ptr %t24
  call void @glitch_drop_DatabaseFacade__g0__t56(ptr %t32)
  store ptr %t27, ptr %t24
  %t33 = load ptr, ptr %t0
  %t34 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t33, i32 0, i32 7
  %t35 = getelementptr %glitch.ChangeTracker__g0__t54, ptr null, i32 1
  %t36 = ptrtoint ptr %t35 to i64
  %t37 = call ptr @glitch_calloc(i64 1, i64 %t36)
  %t38 = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %t37, i32 0, i32 0
  store i64 1, ptr %t38
  %t39 = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %t37, i32 0, i32 1
  store ptr @glitch_destroy_ChangeTracker__g0__t54, ptr %t39
  call void @ChangeTracker__g0__t54_ctor(ptr %t37)
  %t40 = load ptr, ptr @glitch_exception_pending
  %t41 = icmp ne ptr %t40, null
  br i1 %t41, label %exception_unwind, label %call_continue_2
call_continue_2:
  %t42 = load ptr, ptr %t34
  call void @glitch_drop_ChangeTracker__g0__t54(ptr %t42)
  store ptr %t37, ptr %t34
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t60_ctor__DbContextOptions(ptr %this, ptr %options) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %options, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t5
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t3
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t8, i32 0, i32 3
  store i1 0, ptr %t9
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t10, i32 0, i32 4
  %t12 = call ptr @glitch_calloc(i64 1, i64 24)
  %t13 = call ptr @glitch_calloc(i64 4, i64 8)
  %t14 = getelementptr inbounds %glitch.list, ptr %t12, i32 0, i32 1
  store i64 4, ptr %t14
  %t15 = getelementptr inbounds %glitch.list, ptr %t12, i32 0, i32 2
  store ptr %t13, ptr %t15
  store ptr %t12, ptr %t11
  %t16 = load ptr, ptr %t0
  %t17 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t16, i32 0, i32 5
  %t18 = getelementptr %glitch.SqlProvider__g0__t50, ptr null, i32 1
  %t19 = ptrtoint ptr %t18 to i64
  %t20 = call ptr @glitch_calloc(i64 1, i64 %t19)
  %t21 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t20, i32 0, i32 0
  store i64 1, ptr %t21
  %t22 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t20, i32 0, i32 1
  store ptr @glitch_destroy_SqlProvider__g0__t50, ptr %t22
  %t23 = load ptr, ptr %t0
  %t24 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t23, i32 0, i32 2
  %t25 = load ptr, ptr %t24
  call void @SqlProvider__g0__t50_ctor(ptr %t20, ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.16, i32 0, i32 2, i64 0), ptr %t25)
  %t26 = load ptr, ptr @glitch_exception_pending
  %t27 = icmp ne ptr %t26, null
  br i1 %t27, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t28 = load ptr, ptr %t17
  call void @glitch_drop_SqlProvider__g0__t50(ptr %t28)
  store ptr %t20, ptr %t17
  %t29 = load ptr, ptr %t0
  %t30 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t29, i32 0, i32 6
  %t31 = getelementptr %glitch.DatabaseFacade__g0__t56, ptr null, i32 1
  %t32 = ptrtoint ptr %t31 to i64
  %t33 = call ptr @glitch_calloc(i64 1, i64 %t32)
  %t34 = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %t33, i32 0, i32 0
  store i64 1, ptr %t34
  %t35 = getelementptr inbounds %glitch.DatabaseFacade__g0__t56, ptr %t33, i32 0, i32 1
  store ptr @glitch_destroy_DatabaseFacade__g0__t56, ptr %t35
  %t36 = load ptr, ptr %t0
  %t37 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t36, i32 0, i32 2
  %t38 = load ptr, ptr %t37
  call void @DatabaseFacade__g0__t56_ctor__string(ptr %t33, ptr %t38)
  %t39 = load ptr, ptr @glitch_exception_pending
  %t40 = icmp ne ptr %t39, null
  br i1 %t40, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t41 = load ptr, ptr %t30
  call void @glitch_drop_DatabaseFacade__g0__t56(ptr %t41)
  store ptr %t33, ptr %t30
  %t42 = load ptr, ptr %t0
  %t43 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t42, i32 0, i32 7
  %t44 = getelementptr %glitch.ChangeTracker__g0__t54, ptr null, i32 1
  %t45 = ptrtoint ptr %t44 to i64
  %t46 = call ptr @glitch_calloc(i64 1, i64 %t45)
  %t47 = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %t46, i32 0, i32 0
  store i64 1, ptr %t47
  %t48 = getelementptr inbounds %glitch.ChangeTracker__g0__t54, ptr %t46, i32 0, i32 1
  store ptr @glitch_destroy_ChangeTracker__g0__t54, ptr %t48
  call void @ChangeTracker__g0__t54_ctor(ptr %t46)
  %t49 = load ptr, ptr @glitch_exception_pending
  %t50 = icmp ne ptr %t49, null
  br i1 %t50, label %exception_unwind, label %call_continue_2
call_continue_2:
  %t51 = load ptr, ptr %t43
  call void @glitch_drop_ChangeTracker__g0__t54(ptr %t51)
  store ptr %t46, ptr %t43
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t60_EnsureNotDisposed__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t1, i32 0, i32 3
  %t3 = load i1, ptr %t2
  br i1 %t3, label %if_then_0, label %if_else_1
if_then_0:
  store ptr getelementptr inbounds ({ i64, i64, [22 x i8] }, ptr @.str.17, i32 0, i32 2, i64 0), ptr @glitch_exception_pending
  br label %exception_unwind
if_else_1:
  br label %if_end_2
if_end_2:
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t60_Track__g0(ptr %this, ptr %entityKey) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entityKey, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t60_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t5, i32 0, i32 4
  %t7 = load ptr, ptr %t6
  %t8 = load ptr, ptr %t1
  %t9 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 0
  %t10 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 1
  %t11 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 2
  %t12 = load i64, ptr %t9
  %t13 = load i64, ptr %t10
  %t14 = load ptr, ptr %t11
  %t15 = icmp eq i64 %t12, %t13
  br i1 %t15, label %list_grow_1, label %list_ready_2
list_grow_1:
  %t16 = mul i64 %t13, 2
  %t17 = mul i64 %t16, 8
  %t18 = call ptr @glitch_realloc(ptr %t14, i64 %t17)
  store i64 %t16, ptr %t10
  store ptr %t18, ptr %t11
  br label %list_ready_2
list_ready_2:
  %t19 = load ptr, ptr %t11
  %t20 = getelementptr inbounds ptr, ptr %t19, i64 %t12
  call void @glitch_string_retain(ptr %t8)
  store ptr %t8, ptr %t20
  %t21 = add i64 %t12, 1
  store i64 %t21, ptr %t9
  ret void
exception_unwind:
  ret void
}

define i32 @DbContext__g0__t60_get_TrackedCount__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t1, i32 0, i32 4
  %t3 = load ptr, ptr %t2
  %t4 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 0
  %t5 = load i64, ptr %t4
  %t6 = trunc i64 %t5 to i32
  ret i32 %t6
exception_unwind:
  ret i32 0
}

define i32 @DbContext__g0__t60_SaveChanges__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 0, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t60_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t5, i32 0, i32 4
  %t7 = load ptr, ptr %t6
  %t8 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 0
  %t9 = load i64, ptr %t8
  %t10 = trunc i64 %t9 to i32
  store i32 %t10, ptr %t1
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t11, i32 0, i32 4
  %t13 = load ptr, ptr %t12
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  store i64 0, ptr %t14
  %t15 = load i32, ptr %t1
  ret i32 %t15
exception_unwind:
  ret i32 0
}

define ptr @DbContext__g0__t60_SaveChangesAsync__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 0, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t60_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t5, i32 0, i32 4
  %t7 = load ptr, ptr %t6
  %t8 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 0
  %t9 = load i64, ptr %t8
  %t10 = trunc i64 %t9 to i32
  store i32 %t10, ptr %t1
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t11, i32 0, i32 4
  %t13 = load ptr, ptr %t12
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  store i64 0, ptr %t14
  %t15 = load i32, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @DbContext__g0__t60_Add__g0(ptr %this, ptr %entityKey) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entityKey, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  call void @DbContext__g0__t60_Track__g0(ptr %t2, ptr %t3)
  %t4 = load ptr, ptr @glitch_exception_pending
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t60_Update__g0(ptr %this, ptr %entityKey) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entityKey, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  call void @DbContext__g0__t60_Track__g0(ptr %t2, ptr %t3)
  %t4 = load ptr, ptr @glitch_exception_pending
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t60_Remove__g0(ptr %this, ptr %entityKey) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entityKey, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t60_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t60_Dispose__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t1, i32 0, i32 3
  store i1 1, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t3, i32 0, i32 4
  %t5 = load ptr, ptr %t4
  %t6 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 0
  store i64 0, ptr %t6
  ret void
exception_unwind:
  ret void
}

define ptr @DbContext__g0__t60_Set__g1(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.DbSet_T___g0__t51, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbContext__g0__t60_Entry__g0(ptr %this, ptr %entity) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = getelementptr %glitch.EntityEntry__g0__t58, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.EntityEntry__g0__t58, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.EntityEntry__g0__t58, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_EntityEntry__g0__t58, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @IQueryableString__g0__t61_ctor(ptr %this, ptr %connectionString, ptr %table, i1 %tracking) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = alloca ptr
  store ptr %table, ptr %t2
  %t3 = alloca i1
  store i1 %tracking, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t12, i32 0, i32 4
  %t14 = load i1, ptr %t3
  store i1 %t14, ptr %t13
  ret void
exception_unwind:
  ret void
}

define ptr @IQueryableString__g0__t61_AsNoTracking__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.IQueryableString__g0__t61, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_IQueryableString__g0__t61, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t9, i32 0, i32 3
  %t11 = load ptr, ptr %t10
  call void @IQueryableString__g0__t61_ctor(ptr %t3, ptr %t8, ptr %t11, i1 0)
  %t12 = load ptr, ptr @glitch_exception_pending
  %t13 = icmp ne ptr %t12, null
  br i1 %t13, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @IQueryableString__g0__t61_ToQueryString__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr null, ptr %t1
  %t2 = getelementptr %glitch.SqlProvider__g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.SqlProvider__g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_SqlProvider__g0__t50, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  call void @SqlProvider__g0__t50_ctor(ptr %t4, ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.18, i32 0, i32 2, i64 0), ptr %t9)
  %t10 = load ptr, ptr @glitch_exception_pending
  %t11 = icmp ne ptr %t10, null
  br i1 %t11, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t12 = load ptr, ptr %t1
  call void @glitch_drop_SqlProvider__g0__t50(ptr %t12)
  store ptr %t4, ptr %t1
  %t13 = load ptr, ptr %t1
  %t14 = load ptr, ptr %t0
  %t15 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t14, i32 0, i32 3
  %t16 = load ptr, ptr %t15
  %t17 = call ptr @SqlProvider__g0__t50_BuildSelectAll__g0(ptr %t13, ptr %t16)
  %t18 = load ptr, ptr @glitch_exception_pending
  %t19 = icmp ne ptr %t18, null
  br i1 %t19, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t20 = load ptr, ptr %t1
  call void @glitch_drop_SqlProvider__g0__t50(ptr %t20)
  ret ptr %t17
exception_unwind:
  %t21 = load ptr, ptr %t1
  call void @glitch_drop_SqlProvider__g0__t50(ptr %t21)
  ret ptr null
}

define ptr @IQueryableString__g0__t61_ToList__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr null, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = call ptr @glitch_calloc(i64 1, i64 24)
  %t5 = call ptr @glitch_calloc(i64 4, i64 8)
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 1
  store i64 4, ptr %t6
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  store ptr %t5, ptr %t7
  %t8 = load ptr, ptr %t1
  %t9 = icmp eq ptr %t8, null
  br i1 %t9, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t10 = getelementptr inbounds %glitch.list, ptr %t8, i32 0, i32 0
  %t11 = getelementptr inbounds %glitch.list, ptr %t8, i32 0, i32 2
  %t12 = load i64, ptr %t10
  %t13 = load ptr, ptr %t11
  %t14 = alloca i64
  store i64 0, ptr %t14
  br label %element_drop_loop_2
element_drop_loop_2:
  %t15 = load i64, ptr %t14
  %t16 = icmp ult i64 %t15, %t12
  br i1 %t16, label %element_drop_body_3, label %element_drop_done_4
element_drop_body_3:
  %t17 = getelementptr inbounds ptr, ptr %t13, i64 %t15
  %t18 = load ptr, ptr %t17
  call void @glitch_string_release(ptr %t18)
  %t19 = add i64 %t15, 1
  store i64 %t19, ptr %t14
  br label %element_drop_loop_2
element_drop_done_4:
  call void @glitch_free(ptr %t13)
  call void @glitch_free(ptr %t8)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t4, ptr %t1
  %t20 = load ptr, ptr %t0
  %t21 = call ptr @IQueryableString__g0__t61_ToQueryString__g0(ptr %t20)
  %t22 = load ptr, ptr @glitch_exception_pending
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t24 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t24)
  store ptr %t21, ptr %t2
  %t25 = load ptr, ptr %t0
  %t26 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t25, i32 0, i32 4
  %t27 = load i1, ptr %t26
  br i1 %t27, label %if_then_6, label %if_else_7
if_then_6:
  %t28 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t28)
  store ptr getelementptr inbounds ({ i64, i64, [9 x i8] }, ptr @.str.19, i32 0, i32 2, i64 0), ptr %t3
  %t29 = load ptr, ptr %t1
  %t30 = load ptr, ptr %t3
  %t31 = load ptr, ptr %t2
  %t32 = call ptr @glitch_string_concat(ptr %t30, ptr %t31)
  %t33 = getelementptr inbounds %glitch.list, ptr %t29, i32 0, i32 0
  %t34 = getelementptr inbounds %glitch.list, ptr %t29, i32 0, i32 1
  %t35 = getelementptr inbounds %glitch.list, ptr %t29, i32 0, i32 2
  %t36 = load i64, ptr %t33
  %t37 = load i64, ptr %t34
  %t38 = load ptr, ptr %t35
  %t39 = icmp eq i64 %t36, %t37
  br i1 %t39, label %list_grow_9, label %list_ready_10
list_grow_9:
  %t40 = mul i64 %t37, 2
  %t41 = mul i64 %t40, 8
  %t42 = call ptr @glitch_realloc(ptr %t38, i64 %t41)
  store i64 %t40, ptr %t34
  store ptr %t42, ptr %t35
  br label %list_ready_10
list_ready_10:
  %t43 = load ptr, ptr %t35
  %t44 = getelementptr inbounds ptr, ptr %t43, i64 %t36
  store ptr %t32, ptr %t44
  %t45 = add i64 %t36, 1
  store i64 %t45, ptr %t33
  br label %if_end_8
if_else_7:
  %t46 = load ptr, ptr %t1
  %t47 = load ptr, ptr %t2
  %t48 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 0
  %t49 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 1
  %t50 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 2
  %t51 = load i64, ptr %t48
  %t52 = load i64, ptr %t49
  %t53 = load ptr, ptr %t50
  %t54 = icmp eq i64 %t51, %t52
  br i1 %t54, label %list_grow_11, label %list_ready_12
list_grow_11:
  %t55 = mul i64 %t52, 2
  %t56 = mul i64 %t55, 8
  %t57 = call ptr @glitch_realloc(ptr %t53, i64 %t56)
  store i64 %t55, ptr %t49
  store ptr %t57, ptr %t50
  br label %list_ready_12
list_ready_12:
  %t58 = load ptr, ptr %t50
  %t59 = getelementptr inbounds ptr, ptr %t58, i64 %t51
  call void @glitch_string_retain(ptr %t47)
  store ptr %t47, ptr %t59
  %t60 = add i64 %t51, 1
  store i64 %t60, ptr %t48
  br label %if_end_8
if_end_8:
  %t61 = load ptr, ptr %t1
  %t62 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t62)
  %t63 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t63)
  ret ptr %t61
exception_unwind:
  %t64 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t64)
  %t65 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t65)
  %t66 = load ptr, ptr %t1
  %t67 = icmp eq ptr %t66, null
  br i1 %t67, label %collection_release_done_14, label %collection_release_13
collection_release_13:
  %t68 = getelementptr inbounds %glitch.list, ptr %t66, i32 0, i32 0
  %t69 = getelementptr inbounds %glitch.list, ptr %t66, i32 0, i32 2
  %t70 = load i64, ptr %t68
  %t71 = load ptr, ptr %t69
  %t72 = alloca i64
  store i64 0, ptr %t72
  br label %element_drop_loop_15
element_drop_loop_15:
  %t73 = load i64, ptr %t72
  %t74 = icmp ult i64 %t73, %t70
  br i1 %t74, label %element_drop_body_16, label %element_drop_done_17
element_drop_body_16:
  %t75 = getelementptr inbounds ptr, ptr %t71, i64 %t73
  %t76 = load ptr, ptr %t75
  call void @glitch_string_release(ptr %t76)
  %t77 = add i64 %t73, 1
  store i64 %t77, ptr %t72
  br label %element_drop_loop_15
element_drop_done_17:
  call void @glitch_free(ptr %t71)
  call void @glitch_free(ptr %t66)
  br label %collection_release_done_14
collection_release_done_14:
  ret ptr null
}

define void @DbSetString__g0__t62_ctor(ptr %this, ptr %connectionString, ptr %table) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = alloca ptr
  store ptr %table, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t9)
  %t10 = load ptr, ptr %t8
  call void @glitch_string_release(ptr %t10)
  store ptr %t9, ptr %t8
  ret void
exception_unwind:
  ret void
}

define ptr @DbSetString__g0__t62_AsQueryable__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.IQueryableString__g0__t61, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_IQueryableString__g0__t61, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %t9, i32 0, i32 3
  %t11 = load ptr, ptr %t10
  call void @IQueryableString__g0__t61_ctor(ptr %t3, ptr %t8, ptr %t11, i1 1)
  %t12 = load ptr, ptr @glitch_exception_pending
  %t13 = icmp ne ptr %t12, null
  br i1 %t13, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbSetString__g0__t62_AsNoTracking__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.IQueryableString__g0__t61, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.IQueryableString__g0__t61, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_IQueryableString__g0__t61, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %t9, i32 0, i32 3
  %t11 = load ptr, ptr %t10
  call void @IQueryableString__g0__t61_ctor(ptr %t3, ptr %t8, ptr %t11, i1 0)
  %t12 = load ptr, ptr @glitch_exception_pending
  %t13 = icmp ne ptr %t12, null
  br i1 %t13, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbSetString__g0__t62_ToList__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @DbSetString__g0__t62_AsQueryable__g0(ptr %t1)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = call ptr @IQueryableString__g0__t61_ToList__g0(ptr %t2)
  %t6 = load ptr, ptr @glitch_exception_pending
  %t7 = icmp ne ptr %t6, null
  br i1 %t7, label %exception_unwind, label %call_continue_1
call_continue_1:
  call void @glitch_drop_IQueryableString__g0__t61(ptr %t2)
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @ModelBuilder__g0__t63_Entity__g1__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.EntityTypeBuilder_T___g0__t65, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.EntityTypeBuilder_T___g0__t65, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.EntityTypeBuilder_T___g0__t65, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_EntityTypeBuilder_T___g0__t65, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @ModelBuilder__g0__t63_Entity__g0__string(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = getelementptr %glitch.EntityTypeBuilder_object___g0__t65, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.EntityTypeBuilder_object___g0__t65, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.EntityTypeBuilder_object___g0__t65, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_EntityTypeBuilder_object___g0__t65, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @ModelBuilder__g0__t63_Entity__g0__string_object(ptr %this, ptr %name, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %configure, ptr %t2
  %t3 = getelementptr %glitch.EntityTypeBuilder_object___g0__t65, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.EntityTypeBuilder_object___g0__t65, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.EntityTypeBuilder_object___g0__t65, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_EntityTypeBuilder_object___g0__t65, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define void @ConsoleLogger__g0__t69_LogError__g0__string(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [9 x i8] }, ptr @.str.20, i32 0, i32 2, i64 0), ptr %t2)
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t3)
  ret void
exception_unwind:
  ret void
}

define void @ConsoleLogger__g0__t69_LogWarning__g0__string(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [11 x i8] }, ptr @.str.21, i32 0, i32 2, i64 0), ptr %t2)
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t3)
  ret void
exception_unwind:
  ret void
}

define void @ConsoleLogger__g0__t69_LogInformation__g0__string(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.22, i32 0, i32 2, i64 0), ptr %t2)
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t3)
  ret void
exception_unwind:
  ret void
}

define void @LoggerFactory__g0__t72_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @LoggerFactory__g0__t72_CreateLogger__g0(ptr %this, ptr %categoryName) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %categoryName, ptr %t1
  %t2 = getelementptr %glitch.ConsoleLogger__g0__t69, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.ConsoleLogger__g0__t69, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.ConsoleLogger__g0__t69, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_ConsoleLogger__g0__t69, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @LoggerFactory__g0__t72_AddProvider__g0(ptr %this, ptr %provider) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %provider, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @LoggerFactory__g0__t72_AddSerilog__g0(ptr %this, ptr %logger) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %logger, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @SymmetricSecurityKey__g0__t73_ctor__string(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.SymmetricSecurityKey__g0__t73, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @SymmetricSecurityKey__g0__t73_ctor__array_byte(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.SymmetricSecurityKey__g0__t73, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t4)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.23, i32 0, i32 2, i64 0), ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @SigningCredentials__g0__t74_ctor(ptr %this, ptr %key, ptr %algorithm) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %algorithm, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.SigningCredentials__g0__t74, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  call void @glitch_retain_SymmetricSecurityKey__g0__t73(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_drop_SymmetricSecurityKey__g0__t73(ptr %t6)
  store ptr %t5, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.SigningCredentials__g0__t74, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t9)
  %t10 = load ptr, ptr %t8
  call void @glitch_string_release(ptr %t10)
  store ptr %t9, ptr %t8
  ret void
exception_unwind:
  ret void
}

define void @JwtBearerDefaults__g0__t77_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t77, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [7 x i8] }, ptr @.str.24, i32 0, i32 2, i64 0), ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @Mediator__g0__t84_ctor(ptr %this, ptr %provider) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %provider, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Mediator__g0__t84, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_IServiceProvider__g0__t108(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_IServiceProvider__g0__t108(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Mediator__g0__t84_GetProvider__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Mediator__g0__t84, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_retain_IServiceProvider__g0__t108(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Mediator__g0__t84_Send__g1__IRequest_TResponse_(ptr %this, ptr %request) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %request, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Mediator__g0__t84_Send__g0__IRequest(ptr %this, ptr %request) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %request, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @MediatRServiceConfiguration__g0__t85_RegisterServicesFromAssembly__g0__Assembly(ptr %this, ptr %assembly) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %assembly, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @MediatRServiceConfiguration__g0__t85_RegisterServicesFromAssembly__g0__object(ptr %this, ptr %assembly) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %assembly, ptr %t1
  ret void
exception_unwind:
  ret void
}

define ptr @Mapper__g0__t87_Map__g2(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @MemberConfigurationExpression__g0__t89_MapFrom__g0(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @MemberConfigurationExpression__g0__t89_Ignore__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @MappingExpression__g2__t91_ReverseMap__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_MappingExpression_TSource_TDestination___g0__t91, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @MappingExpression__g2__t91_ForMember__g0(ptr %this, ptr %dest, ptr %opt) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %dest, ptr %t1
  %t2 = alloca ptr
  store ptr %opt, ptr %t2
  %t3 = getelementptr %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_MappingExpression_TSource_TDestination___g0__t91, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @MappingExpression__g2__t91_IgnoreAllMembers__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_MappingExpression_TSource_TDestination___g0__t91, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Profile__g0__t92_CreateMap__g2(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_MappingExpression_TSource_TDestination___g0__t91, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @MapperConfigurationExpression__g0__t93_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t93, ptr %t1, i32 0, i32 2
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

define void @MapperConfigurationExpression__g0__t93_AddProfile__g0(ptr %this, ptr %profile) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %profile, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t93, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  %t5 = load ptr, ptr %t1
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 0
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 1
  %t8 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  %t9 = load i64, ptr %t6
  %t10 = load i64, ptr %t7
  %t11 = load ptr, ptr %t8
  %t12 = icmp eq i64 %t9, %t10
  br i1 %t12, label %list_grow_0, label %list_ready_1
list_grow_0:
  %t13 = mul i64 %t10, 2
  %t14 = mul i64 %t13, 8
  %t15 = call ptr @glitch_realloc(ptr %t11, i64 %t14)
  store i64 %t13, ptr %t7
  store ptr %t15, ptr %t8
  br label %list_ready_1
list_ready_1:
  %t16 = load ptr, ptr %t8
  %t17 = getelementptr inbounds ptr, ptr %t16, i64 %t9
  call void @glitch_retain_Profile__g0__t92(ptr %t5)
  store ptr %t5, ptr %t17
  %t18 = add i64 %t9, 1
  store i64 %t18, ptr %t6
  ret void
exception_unwind:
  ret void
}

define void @MapperConfigurationExpression__g0__t93_AddMaps__g0(ptr %this, ptr %marker) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %marker, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @MapperConfiguration__g0__t94_ctor(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.MapperConfiguration__g0__t94, ptr %t2, i32 0, i32 2
  %t4 = getelementptr %glitch.Mapper__g0__t87, ptr null, i32 1
  %t5 = ptrtoint ptr %t4 to i64
  %t6 = call ptr @glitch_calloc(i64 1, i64 %t5)
  %t7 = getelementptr inbounds %glitch.Mapper__g0__t87, ptr %t6, i32 0, i32 0
  store i64 1, ptr %t7
  %t8 = getelementptr inbounds %glitch.Mapper__g0__t87, ptr %t6, i32 0, i32 1
  store ptr @glitch_destroy_Mapper__g0__t87, ptr %t8
  %t9 = load ptr, ptr %t3
  call void @glitch_drop_Mapper__g0__t87(ptr %t9)
  store ptr %t6, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @MapperConfiguration__g0__t94_CreateMapper__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.MapperConfiguration__g0__t94, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_retain_Mapper__g0__t87(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @ValidationResult__g0__t96_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %t1, i32 0, i32 2
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

define void @ValidationResult__g0__t96_ctor__list_ValidationFailure(ptr %this, ptr %errors) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %errors, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @ValidationException__g0__t97_ctor__list_ValidationFailure(ptr %this, ptr %errors) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %errors, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ValidationException__g0__t97, ptr %t2, i32 0, i32 3
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @ValidationException__g0__t97_ctor__ienumerable_ValidationFailure(ptr %this, ptr %errors) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %errors, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.ValidationException__g0__t97, ptr %t3, i32 0, i32 3
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
  %t21 = getelementptr inbounds %glitch.ValidationException__g0__t97, ptr %t20, i32 0, i32 3
  %t22 = load ptr, ptr %t21
  %t23 = load ptr, ptr %t15
  %t24 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 0
  %t25 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 1
  %t26 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 2
  %t27 = load i64, ptr %t24
  %t28 = load i64, ptr %t25
  %t29 = load ptr, ptr %t26
  %t30 = icmp eq i64 %t27, %t28
  br i1 %t30, label %list_grow_4, label %list_ready_5
list_grow_4:
  %t31 = mul i64 %t28, 2
  %t32 = mul i64 %t31, 8
  %t33 = call ptr @glitch_realloc(ptr %t29, i64 %t32)
  store i64 %t31, ptr %t25
  store ptr %t33, ptr %t26
  br label %list_ready_5
list_ready_5:
  %t34 = load ptr, ptr %t26
  %t35 = getelementptr inbounds ptr, ptr %t34, i64 %t27
  call void @glitch_retain_ValidationFailure__g0__t95(ptr %t23)
  store ptr %t23, ptr %t35
  %t36 = add i64 %t27, 1
  store i64 %t36, ptr %t24
  br label %foreach_advance_2
foreach_advance_2:
  %t37 = load i64, ptr %t14
  %t38 = add i64 %t37, 1
  store i64 %t38, ptr %t14
  br label %foreach_condition_0
foreach_end_3:
  ret void
exception_unwind:
  ret void
}

define void @ValidationException__g0__t97_ctor__object(ptr %this, ptr %errors) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %errors, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ValidationException__g0__t97, ptr %t2, i32 0, i32 3
  %t4 = call ptr @glitch_calloc(i64 1, i64 24)
  %t5 = call ptr @glitch_calloc(i64 4, i64 8)
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 1
  store i64 4, ptr %t6
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  store ptr %t5, ptr %t7
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @ValidationContext__g1__t98_ctor(ptr %this, ptr %instanceToValidate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %instanceToValidate, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ValidationContext_T___g0__t98, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @RuleBuilder__g2__t100_NotNull__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_NotEmpty__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_SetValidator__g0(ptr %this, ptr %validator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %validator, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_EmailAddress__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_MinimumLength__g0(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_MaximumLength__g0(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Length__g0(ptr %this, i32 %min, i32 %max) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %min, ptr %t1
  %t2 = alloca i32
  store i32 %max, ptr %t2
  %t3 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Matches__g0(ptr %this, ptr %pattern) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %pattern, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Equal__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_NotEqual__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Must__g0(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_WithMessage__g0(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t101_RuleFor__g0(ptr %this, ptr %expression) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %expression, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_object___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_object___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t101_RuleForEach__g0(ptr %this, ptr %expression) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %expression, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_object___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_object___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t101_Validate__g0(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = getelementptr %glitch.ValidationResult__g0__t96, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_ValidationResult__g0__t96, ptr %t6
  call void @ValidationResult__g0__t96_ctor__empty(ptr %t4)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @JsonSerializer__g0__t103_Serialize__g0(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @JsonSerializer__g0__t103_Deserialize__g0(ptr %json) {
entry:
  %t0 = alloca ptr
  store ptr %json, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @OpenApiSecurityRequirement__g0__t106_Add__g0(ptr %this, ptr %scheme, ptr %scopes) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %scheme, ptr %t1
  %t2 = alloca ptr
  store ptr %scopes, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @DbContextOptionsBuilder__g0__t109_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.DbContextOptions__g0__t49, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_DbContextOptions__g0__t49, ptr %t7
  call void @DbContextOptions__g0__t49_ctor(ptr %t5)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = load ptr, ptr %t2
  call void @glitch_drop_DbContextOptions__g0__t49(ptr %t10)
  store ptr %t5, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @DbContextOptionsBuilder__g0__t109_UseSqlite__g0(ptr %this, ptr %connectionString) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %t8, i32 0, i32 2
  %t10 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t11)
  store ptr %t10, ptr %t9
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  ret void
exception_unwind:
  ret void
}

define void @DbContextOptionsBuilder__g0__t109_UseSqlServer__g0(ptr %this, ptr %connectionString) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %t8, i32 0, i32 2
  %t10 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t11)
  store ptr %t10, ptr %t9
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  ret void
exception_unwind:
  ret void
}

define void @DbContextOptionsBuilder__g0__t109_UseInMemoryDatabase__g0(ptr %this, ptr %connectionString) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.DbContextOptionsBuilder__g0__t109, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = getelementptr inbounds %glitch.DbContextOptions__g0__t49, ptr %t8, i32 0, i32 2
  %t10 = load ptr, ptr %t1
  %t11 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [10 x i8] }, ptr @.str.25, i32 0, i32 2, i64 0), ptr %t10)
  %t12 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t12)
  store ptr %t11, ptr %t9
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  ret void
exception_unwind:
  ret void
}

define ptr @CorsPolicyBuilder__g0__t111_AllowAnyOrigin__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.CorsPolicyBuilder__g0__t111, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.CorsPolicyBuilder__g0__t111, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.CorsPolicyBuilder__g0__t111, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_CorsPolicyBuilder__g0__t111, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @CorsPolicyBuilder__g0__t111_AllowAnyHeader__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.CorsPolicyBuilder__g0__t111, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.CorsPolicyBuilder__g0__t111, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.CorsPolicyBuilder__g0__t111, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_CorsPolicyBuilder__g0__t111, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @CorsPolicyBuilder__g0__t111_AllowAnyMethod__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.CorsPolicyBuilder__g0__t111, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.CorsPolicyBuilder__g0__t111, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.CorsPolicyBuilder__g0__t111, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_CorsPolicyBuilder__g0__t111, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @MvcConventions__g0__t112_Add__g0(ptr %this, ptr %convention) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %convention, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @MvcFilterCollection__g0__t113_Add__g1(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @MvcJsonOptions__g0__t114_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.MvcJsonOptions__g0__t114, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.JsonSerializerOptions__g0__t102, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.JsonSerializerOptions__g0__t102, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.JsonSerializerOptions__g0__t102, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_JsonSerializerOptions__g0__t102, ptr %t7
  %t8 = load ptr, ptr %t2
  call void @glitch_drop_JsonSerializerOptions__g0__t102(ptr %t8)
  store ptr %t5, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @MvcOptions__g0__t115_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.MvcOptions__g0__t115, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.MvcConventions__g0__t112, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.MvcConventions__g0__t112, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.MvcConventions__g0__t112, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_MvcConventions__g0__t112, ptr %t7
  %t8 = load ptr, ptr %t2
  call void @glitch_drop_MvcConventions__g0__t112(ptr %t8)
  store ptr %t5, ptr %t2
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.MvcOptions__g0__t115, ptr %t9, i32 0, i32 3
  %t11 = getelementptr %glitch.MvcFilterCollection__g0__t113, ptr null, i32 1
  %t12 = ptrtoint ptr %t11 to i64
  %t13 = call ptr @glitch_calloc(i64 1, i64 %t12)
  %t14 = getelementptr inbounds %glitch.MvcFilterCollection__g0__t113, ptr %t13, i32 0, i32 0
  store i64 1, ptr %t14
  %t15 = getelementptr inbounds %glitch.MvcFilterCollection__g0__t113, ptr %t13, i32 0, i32 1
  store ptr @glitch_destroy_MvcFilterCollection__g0__t113, ptr %t15
  %t16 = load ptr, ptr %t10
  call void @glitch_drop_MvcFilterCollection__g0__t113(ptr %t16)
  store ptr %t13, ptr %t10
  %t17 = load ptr, ptr %t0
  %t18 = getelementptr inbounds %glitch.MvcOptions__g0__t115, ptr %t17, i32 0, i32 4
  store i1 1, ptr %t18
  ret void
exception_unwind:
  ret void
}

define ptr @AuthenticationBuilder__g0__t116_AddJwtBearer__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = getelementptr %glitch.AuthenticationBuilder__g0__t116, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.AuthenticationBuilder__g0__t116, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.AuthenticationBuilder__g0__t116, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_AuthenticationBuilder__g0__t116, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @SwaggerGenOptions__g0__t118_AddSecurityDefinition__g0(ptr %this, ptr %name, ptr %scheme) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %scheme, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @SwaggerGenOptions__g0__t118_SupportNonNullableReferenceTypes__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @SwaggerGenOptions__g0__t118_AddSecurityRequirement__g0(ptr %this, ptr %requirement) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %requirement, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @SwaggerGenOptions__g0__t118_SwaggerDoc__g0(ptr %this, ptr %documentName, ptr %info) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %documentName, ptr %t1
  %t2 = alloca ptr
  store ptr %info, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @SwaggerGenOptions__g0__t118_CustomSchemaIds__g0(ptr %this, ptr %selector) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %selector, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @SwaggerGenOptions__g0__t118_DocInclusionPredicate__g0(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @SwaggerGenOptions__g0__t118_TagActionsBy__g0(ptr %this, ptr %selector) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %selector, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ServiceProvider__g0__t119_ctor(ptr %this, ptr %singleton) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %singleton, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_object__g0__t14(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_object__g0__t14(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @ServiceProvider__g0__t119_GetRequiredService__g1__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr %glitch.Type__g0__t24, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_Type__g0__t24, ptr %t6
  %t7 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 10
  store ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.26, i32 0, i32 2, i64 0), ptr %t7
  %t8 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 11
  store ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.27, i32 0, i32 2, i64 0), ptr %t8
  %t9 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 4
  store i1 0, ptr %t9
  %t10 = call ptr @glitch_calloc(i64 1, i64 16)
  %t11 = call ptr @glitch_calloc(i64 0, i64 8)
  %t12 = getelementptr inbounds %glitch.array, ptr %t10, i32 0, i32 0
  store i64 0, ptr %t12
  %t13 = getelementptr inbounds %glitch.array, ptr %t10, i32 0, i32 1
  store ptr %t11, ptr %t13
  %t14 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 12
  store ptr %t10, ptr %t14
  %t15 = call ptr @glitch_calloc(i64 1, i64 16)
  %t16 = call ptr @glitch_calloc(i64 0, i64 8)
  %t17 = getelementptr inbounds %glitch.array, ptr %t15, i32 0, i32 0
  store i64 0, ptr %t17
  %t18 = getelementptr inbounds %glitch.array, ptr %t15, i32 0, i32 1
  store ptr %t16, ptr %t18
  %t19 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 15
  store ptr %t15, ptr %t19
  call void @glitch_drop_Type__g0__t24(ptr %t4)
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ServiceProvider__g0__t119_GetRequiredService__g0__string(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @ServiceProvider__g0__t119_GetService__g1(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr %glitch.Type__g0__t24, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_Type__g0__t24, ptr %t6
  %t7 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 10
  store ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.28, i32 0, i32 2, i64 0), ptr %t7
  %t8 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 11
  store ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.29, i32 0, i32 2, i64 0), ptr %t8
  %t9 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 4
  store i1 0, ptr %t9
  %t10 = call ptr @glitch_calloc(i64 1, i64 16)
  %t11 = call ptr @glitch_calloc(i64 0, i64 8)
  %t12 = getelementptr inbounds %glitch.array, ptr %t10, i32 0, i32 0
  store i64 0, ptr %t12
  %t13 = getelementptr inbounds %glitch.array, ptr %t10, i32 0, i32 1
  store ptr %t11, ptr %t13
  %t14 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 12
  store ptr %t10, ptr %t14
  %t15 = call ptr @glitch_calloc(i64 1, i64 16)
  %t16 = call ptr @glitch_calloc(i64 0, i64 8)
  %t17 = getelementptr inbounds %glitch.array, ptr %t15, i32 0, i32 0
  store i64 0, ptr %t17
  %t18 = getelementptr inbounds %glitch.array, ptr %t15, i32 0, i32 1
  store ptr %t16, ptr %t18
  %t19 = getelementptr inbounds %glitch.Type__g0__t24, ptr %t4, i32 0, i32 15
  store ptr %t15, ptr %t19
  call void @glitch_drop_Type__g0__t24(ptr %t4)
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ServiceProvider__g0__t119_CreateScope__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.ServiceScope__g0__t122, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.ServiceScope__g0__t122, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.ServiceScope__g0__t122, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_ServiceScope__g0__t122, ptr %t5
  %t6 = load ptr, ptr %t0
  call void @ServiceScope__g0__t122_ctor(ptr %t3, ptr %t6)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @ServiceScope__g0__t122_ctor(ptr %this, ptr %provider) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %provider, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ServiceScope__g0__t122, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr null, ptr %t1
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_IServiceProvider__g0__t108(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @MvcBuilder__g0__t123_AddJsonOptions__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = getelementptr %glitch.MvcBuilder__g0__t123, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t123, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @MvcBuilder__g0__t123_ConfigureApiBehaviorOptions__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = getelementptr %glitch.MvcBuilder__g0__t123, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t123, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @MvcBuilder__g0__t123_AddJwtBearer__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = getelementptr %glitch.MvcBuilder__g0__t123, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t123, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @MvcBuilder__g0__t123_AddAuthentication__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.MvcBuilder__g0__t123, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t123, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @MvcBuilder__g0__t123_AddAuthentication__g0__string(ptr %this, ptr %scheme) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %scheme, ptr %t1
  %t2 = getelementptr %glitch.MvcBuilder__g0__t123, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t123, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @IServiceCollection__g0__t124_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %t1, i32 0, i32 3
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.30, i32 0, i32 2, i64 0), ptr %t2
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %t4, i32 0, i32 2
  call void @glitch_retain_object__g0__t14(ptr null)
  %t6 = load ptr, ptr %t5
  call void @glitch_drop_object__g0__t14(ptr %t6)
  store ptr null, ptr %t5
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddSingleton__g1__T(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_object__g0__t14(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_object__g0__t14(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddSingleton__g0__string_string(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %t3, i32 0, i32 3
  %t5 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t4
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddTransient__g2__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddTransient__g0__object_object(ptr %this, ptr %serviceType, ptr %implementationType) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %serviceType, ptr %t1
  %t2 = alloca ptr
  store ptr %implementationType, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddScoped__g0__object_object(ptr %this, ptr %serviceType, ptr %implementationType) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %serviceType, ptr %t1
  %t2 = alloca ptr
  store ptr %implementationType, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddScoped__g2__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_Configure__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddApiVersioning__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddVersionedApiExplorer__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddSwaggerGen__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddMemoryCache__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddHttpContextAccessor__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddEndpointsApiExplorer__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddLibraryApiVersionConfiguration__g0(ptr %this, ptr %version) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %version, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddAndConfigureSwagger__g0(ptr %this, ptr %configuration, ptr %path, i1 %enabled) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configuration, ptr %t1
  %t2 = alloca ptr
  store ptr %path, ptr %t2
  %t3 = alloca i1
  store i1 %enabled, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddOptions__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddLogging__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddMediatR__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddValidatorsFromAssemblyContaining__g1(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddDbContext__g1(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddDbContextPool__g1(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddLocalization__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddCors__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddCors__g0__fn_CorsPolicyBuilder(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddAutoMapper__g0__fn_MapperConfigurationExpression(ptr %this, ptr %marker) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %marker, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddAutoMapper__g0__MapperConfigurationExpression(ptr %this, ptr %marker) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %marker, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddRepositories__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @IServiceCollection__g0__t124_AddDataServices__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @IServiceCollection__g0__t124_AddMvc__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = getelementptr %glitch.MvcBuilder__g0__t123, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t123, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @IServiceCollection__g0__t124_AddControllers__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = getelementptr %glitch.MvcBuilder__g0__t123, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t123, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t123, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @IServiceCollection__g0__t124_AddAuthentication__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.AuthenticationBuilder__g0__t116, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.AuthenticationBuilder__g0__t116, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.AuthenticationBuilder__g0__t116, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_AuthenticationBuilder__g0__t116, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @IServiceCollection__g0__t124_AddAuthentication__g0__string(ptr %this, ptr %scheme) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %scheme, ptr %t1
  %t2 = getelementptr %glitch.AuthenticationBuilder__g0__t116, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.AuthenticationBuilder__g0__t116, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.AuthenticationBuilder__g0__t116, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_AuthenticationBuilder__g0__t116, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @IServiceCollection__g0__t124_BuildServiceProvider__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.ServiceProvider__g0__t119, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_ServiceProvider__g0__t119, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.IServiceCollection__g0__t124, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  call void @ServiceProvider__g0__t119_ctor(ptr %t3, ptr %t8)
  %t9 = load ptr, ptr @glitch_exception_pending
  %t10 = icmp ne ptr %t9, null
  br i1 %t10, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @ServiceCollection__g0__t125_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ServiceCollection__g0__t125, ptr %t1, i32 0, i32 3
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.31, i32 0, i32 2, i64 0), ptr %t2
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.ServiceCollection__g0__t125, ptr %t4, i32 0, i32 2
  call void @glitch_retain_object__g0__t14(ptr null)
  %t6 = load ptr, ptr %t5
  call void @glitch_drop_object__g0__t14(ptr %t6)
  store ptr null, ptr %t5
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t125_AddSwaggerGen__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define ptr @ConfigurationManager__g0__t126_GetConnectionString__g0(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  ret ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.32, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define ptr @ConfigurationManager__g0__t126_GetSection__g0(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = getelementptr %glitch.ConfigurationManager__g0__t126, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.ConfigurationManager__g0__t126, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.ConfigurationManager__g0__t126, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_ConfigurationManager__g0__t126, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @ConfigurationManager__g0__t126_Get__g0__string(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  ret ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.33, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define ptr @ConfigurationManager__g0__t126_Get__g1__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ConfigurationManager__g0__t126_GetValue__g1(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define i1 @HostEnvironment__g0__t127_IsDevelopment__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @LoggingBuilder__g0__t128_ClearProviders__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @LoggingBuilder__g0__t128_AddSerilog__g0(ptr %this, ptr %logger, i1 %dispose) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %logger, ptr %t1
  %t2 = alloca i1
  store i1 %dispose, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @WebApplicationBuilder__g0__t129_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.ServiceCollection__g0__t125, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.ServiceCollection__g0__t125, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.ServiceCollection__g0__t125, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_ServiceCollection__g0__t125, ptr %t7
  call void @ServiceCollection__g0__t125_ctor(ptr %t5)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = load ptr, ptr %t2
  call void @glitch_drop_ServiceCollection__g0__t125(ptr %t10)
  store ptr %t5, ptr %t2
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %t11, i32 0, i32 3
  %t13 = getelementptr %glitch.ConfigurationManager__g0__t126, ptr null, i32 1
  %t14 = ptrtoint ptr %t13 to i64
  %t15 = call ptr @glitch_calloc(i64 1, i64 %t14)
  %t16 = getelementptr inbounds %glitch.ConfigurationManager__g0__t126, ptr %t15, i32 0, i32 0
  store i64 1, ptr %t16
  %t17 = getelementptr inbounds %glitch.ConfigurationManager__g0__t126, ptr %t15, i32 0, i32 1
  store ptr @glitch_destroy_ConfigurationManager__g0__t126, ptr %t17
  %t18 = load ptr, ptr %t12
  call void @glitch_drop_ConfigurationManager__g0__t126(ptr %t18)
  store ptr %t15, ptr %t12
  %t19 = load ptr, ptr %t0
  %t20 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %t19, i32 0, i32 4
  %t21 = getelementptr %glitch.HostEnvironment__g0__t127, ptr null, i32 1
  %t22 = ptrtoint ptr %t21 to i64
  %t23 = call ptr @glitch_calloc(i64 1, i64 %t22)
  %t24 = getelementptr inbounds %glitch.HostEnvironment__g0__t127, ptr %t23, i32 0, i32 0
  store i64 1, ptr %t24
  %t25 = getelementptr inbounds %glitch.HostEnvironment__g0__t127, ptr %t23, i32 0, i32 1
  store ptr @glitch_destroy_HostEnvironment__g0__t127, ptr %t25
  %t26 = load ptr, ptr %t20
  call void @glitch_drop_HostEnvironment__g0__t127(ptr %t26)
  store ptr %t23, ptr %t20
  %t27 = load ptr, ptr %t0
  %t28 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %t27, i32 0, i32 5
  %t29 = getelementptr %glitch.LoggingBuilder__g0__t128, ptr null, i32 1
  %t30 = ptrtoint ptr %t29 to i64
  %t31 = call ptr @glitch_calloc(i64 1, i64 %t30)
  %t32 = getelementptr inbounds %glitch.LoggingBuilder__g0__t128, ptr %t31, i32 0, i32 0
  store i64 1, ptr %t32
  %t33 = getelementptr inbounds %glitch.LoggingBuilder__g0__t128, ptr %t31, i32 0, i32 1
  store ptr @glitch_destroy_LoggingBuilder__g0__t128, ptr %t33
  %t34 = load ptr, ptr %t28
  call void @glitch_drop_LoggingBuilder__g0__t128(ptr %t34)
  store ptr %t31, ptr %t28
  ret void
exception_unwind:
  ret void
}

define ptr @WebApplicationBuilder__g0__t129_Build__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.WebApplication__g0__t141, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_WebApplication__g0__t141, ptr %t5
  call void @WebApplication__g0__t141_ctor(ptr %t3)
  %t6 = load ptr, ptr @glitch_exception_pending
  %t7 = icmp ne ptr %t6, null
  br i1 %t7, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @WebApplicationBuilder__g0__t129_ConfigureSerilog__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @HttpRequest__g0__t130_ctor(ptr %this, ptr %method, ptr %path, ptr %body) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %method, ptr %t1
  %t2 = alloca ptr
  store ptr %path, ptr %t2
  %t3 = alloca ptr
  store ptr %body, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %t12, i32 0, i32 4
  %t14 = call ptr @glitch_calloc(i64 1, i64 32)
  %t15 = call ptr @glitch_calloc(i64 4, i64 8)
  %t16 = call ptr @glitch_calloc(i64 4, i64 8)
  %t17 = getelementptr inbounds %glitch.dict, ptr %t14, i32 0, i32 1
  store i64 4, ptr %t17
  %t18 = getelementptr inbounds %glitch.dict, ptr %t14, i32 0, i32 2
  store ptr %t15, ptr %t18
  %t19 = getelementptr inbounds %glitch.dict, ptr %t14, i32 0, i32 3
  store ptr %t16, ptr %t19
  store ptr %t14, ptr %t13
  %t20 = load ptr, ptr %t0
  %t21 = getelementptr inbounds %glitch.HttpRequest__g0__t130, ptr %t20, i32 0, i32 5
  %t22 = load ptr, ptr %t3
  call void @glitch_string_retain(ptr %t22)
  %t23 = load ptr, ptr %t21
  call void @glitch_string_release(ptr %t23)
  store ptr %t22, ptr %t21
  ret void
exception_unwind:
  ret void
}

define void @HttpResponse__g0__t131_ctor(ptr %this, i32 %statusCode, ptr %body) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %statusCode, ptr %t1
  %t2 = alloca ptr
  store ptr %body, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %t3, i32 0, i32 2
  %t5 = load i32, ptr %t1
  store i32 %t5, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %t6, i32 0, i32 3
  %t8 = call ptr @glitch_calloc(i64 1, i64 32)
  %t9 = call ptr @glitch_calloc(i64 4, i64 8)
  %t10 = call ptr @glitch_calloc(i64 4, i64 8)
  %t11 = getelementptr inbounds %glitch.dict, ptr %t8, i32 0, i32 1
  store i64 4, ptr %t11
  %t12 = getelementptr inbounds %glitch.dict, ptr %t8, i32 0, i32 2
  store ptr %t9, ptr %t12
  %t13 = getelementptr inbounds %glitch.dict, ptr %t8, i32 0, i32 3
  store ptr %t10, ptr %t13
  store ptr %t8, ptr %t7
  %t14 = load ptr, ptr %t0
  %t15 = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %t14, i32 0, i32 4
  %t16 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t16)
  %t17 = load ptr, ptr %t15
  call void @glitch_string_release(ptr %t17)
  store ptr %t16, ptr %t15
  ret void
exception_unwind:
  ret void
}

define void @IPAddress__g0__t132_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.IPAddress__g0__t132, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [10 x i8] }, ptr @.str.34, i32 0, i32 2, i64 0), ptr %t2
  ret void
exception_unwind:
  ret void
}

define ptr @IPAddress__g0__t132_ToString__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.IPAddress__g0__t132, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @ConnectionInfo__g0__t133_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ConnectionInfo__g0__t133, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.IPAddress__g0__t132, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.IPAddress__g0__t132, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.IPAddress__g0__t132, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_IPAddress__g0__t132, ptr %t7
  call void @IPAddress__g0__t132_ctor(ptr %t5)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = load ptr, ptr %t2
  call void @glitch_drop_IPAddress__g0__t132(ptr %t10)
  store ptr %t5, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @HttpContext__g0__t134_ctor(ptr %this, ptr %request) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %request, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr null, ptr %t1
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_HttpRequest__g0__t130(ptr %t5)
  store ptr %t4, ptr %t3
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %t6, i32 0, i32 3
  %t8 = getelementptr %glitch.HttpResponse__g0__t131, ptr null, i32 1
  %t9 = ptrtoint ptr %t8 to i64
  %t10 = call ptr @glitch_calloc(i64 1, i64 %t9)
  %t11 = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %t10, i32 0, i32 0
  store i64 1, ptr %t11
  %t12 = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %t10, i32 0, i32 1
  store ptr @glitch_destroy_HttpResponse__g0__t131, ptr %t12
  %t13 = trunc i64 200 to i32
  call void @HttpResponse__g0__t131_ctor(ptr %t10, i32 %t13, ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.35, i32 0, i32 2, i64 0))
  %t14 = load ptr, ptr @glitch_exception_pending
  %t15 = icmp ne ptr %t14, null
  br i1 %t15, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t16 = load ptr, ptr %t7
  call void @glitch_drop_HttpResponse__g0__t131(ptr %t16)
  store ptr %t10, ptr %t7
  %t17 = load ptr, ptr %t0
  %t18 = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %t17, i32 0, i32 4
  %t19 = getelementptr %glitch.ConnectionInfo__g0__t133, ptr null, i32 1
  %t20 = ptrtoint ptr %t19 to i64
  %t21 = call ptr @glitch_calloc(i64 1, i64 %t20)
  %t22 = getelementptr inbounds %glitch.ConnectionInfo__g0__t133, ptr %t21, i32 0, i32 0
  store i64 1, ptr %t22
  %t23 = getelementptr inbounds %glitch.ConnectionInfo__g0__t133, ptr %t21, i32 0, i32 1
  store ptr @glitch_destroy_ConnectionInfo__g0__t133, ptr %t23
  call void @ConnectionInfo__g0__t133_ctor(ptr %t21)
  %t24 = load ptr, ptr @glitch_exception_pending
  %t25 = icmp ne ptr %t24, null
  br i1 %t25, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t26 = load ptr, ptr %t18
  call void @glitch_drop_ConnectionInfo__g0__t133(ptr %t26)
  store ptr %t21, ptr %t18
  %t27 = load ptr, ptr %t0
  %t28 = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %t27, i32 0, i32 5
  %t29 = load ptr, ptr %t28
  call void @glitch_string_release(ptr %t29)
  store ptr getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.36, i32 0, i32 2, i64 0), ptr %t28
  ret void
exception_unwind:
  ret void
}

define void @HttpContext__g0__t134_WriteJson__g0(ptr %this, ptr %text) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %text, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %t2, i32 0, i32 3
  %t4 = load ptr, ptr %t3
  %t5 = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %t4, i32 0, i32 4
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.HttpContext__g0__t134, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t9
  %t11 = getelementptr inbounds %glitch.HttpResponse__g0__t131, ptr %t10, i32 0, i32 3
  %t12 = load ptr, ptr %t11
  %t13 = getelementptr inbounds %glitch.dict, ptr %t12, i32 0, i32 0
  %t14 = getelementptr inbounds %glitch.dict, ptr %t12, i32 0, i32 1
  %t15 = getelementptr inbounds %glitch.dict, ptr %t12, i32 0, i32 2
  %t16 = getelementptr inbounds %glitch.dict, ptr %t12, i32 0, i32 3
  %t17 = load i64, ptr %t13
  %t18 = load i64, ptr %t14
  %t19 = load ptr, ptr %t15
  %t20 = load ptr, ptr %t16
  %t21 = icmp eq i64 %t17, %t18
  br i1 %t21, label %dict_grow_0, label %dict_ready_1
dict_grow_0:
  %t22 = mul i64 %t18, 2
  %t23 = mul i64 %t22, 8
  %t24 = mul i64 %t22, 8
  %t25 = call ptr @glitch_realloc(ptr %t19, i64 %t23)
  %t26 = call ptr @glitch_realloc(ptr %t20, i64 %t24)
  store i64 %t22, ptr %t14
  store ptr %t25, ptr %t15
  store ptr %t26, ptr %t16
  br label %dict_ready_1
dict_ready_1:
  %t27 = load ptr, ptr %t15
  %t28 = load ptr, ptr %t16
  %t29 = getelementptr inbounds ptr, ptr %t27, i64 %t17
  %t30 = getelementptr inbounds ptr, ptr %t28, i64 %t17
  store ptr getelementptr inbounds ({ i64, i64, [13 x i8] }, ptr @.str.37, i32 0, i32 2, i64 0), ptr %t29
  store ptr getelementptr inbounds ({ i64, i64, [17 x i8] }, ptr @.str.38, i32 0, i32 2, i64 0), ptr %t30
  %t31 = add i64 %t17, 1
  store i64 %t31, ptr %t13
  ret void
exception_unwind:
  ret void
}

define void @SwaggerUiOptions__g0__t138_SwaggerEndpoint__g0(ptr %this, ptr %url, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %url, ptr %t1
  %t2 = alloca ptr
  store ptr %name, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @Endpoint__g0__t140_ctor(ptr %this, ptr %method, ptr %path, ptr %text) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %method, ptr %t1
  %t2 = alloca ptr
  store ptr %path, ptr %t2
  %t3 = alloca ptr
  store ptr %text, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Endpoint__g0__t140, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.Endpoint__g0__t140, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Endpoint__g0__t140, ptr %t12, i32 0, i32 4
  %t14 = load ptr, ptr %t3
  call void @glitch_string_retain(ptr %t14)
  %t15 = load ptr, ptr %t13
  call void @glitch_string_release(ptr %t15)
  store ptr %t14, ptr %t13
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t1, i32 0, i32 2
  %t3 = call ptr @glitch_calloc(i64 1, i64 24)
  %t4 = call ptr @glitch_calloc(i64 4, i64 8)
  %t5 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 1
  store i64 4, ptr %t5
  %t6 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 2
  store ptr %t4, ptr %t6
  store ptr %t3, ptr %t2
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t7, i32 0, i32 3
  %t9 = call ptr @glitch_calloc(i64 1, i64 32)
  %t10 = call ptr @glitch_calloc(i64 4, i64 8)
  %t11 = call ptr @glitch_calloc(i64 4, i64 8)
  %t12 = getelementptr inbounds %glitch.dict, ptr %t9, i32 0, i32 1
  store i64 4, ptr %t12
  %t13 = getelementptr inbounds %glitch.dict, ptr %t9, i32 0, i32 2
  store ptr %t10, ptr %t13
  %t14 = getelementptr inbounds %glitch.dict, ptr %t9, i32 0, i32 3
  store ptr %t11, ptr %t14
  store ptr %t9, ptr %t8
  %t15 = load ptr, ptr %t0
  %t16 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t15, i32 0, i32 4
  %t17 = call ptr @glitch_calloc(i64 1, i64 24)
  %t18 = call ptr @glitch_calloc(i64 4, i64 8)
  %t19 = getelementptr inbounds %glitch.list, ptr %t17, i32 0, i32 1
  store i64 4, ptr %t19
  %t20 = getelementptr inbounds %glitch.list, ptr %t17, i32 0, i32 2
  store ptr %t18, ptr %t20
  store ptr %t17, ptr %t16
  %t21 = load ptr, ptr %t0
  %t22 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t21, i32 0, i32 5
  %t23 = getelementptr %glitch.ConfigurationManager__g0__t126, ptr null, i32 1
  %t24 = ptrtoint ptr %t23 to i64
  %t25 = call ptr @glitch_calloc(i64 1, i64 %t24)
  %t26 = getelementptr inbounds %glitch.ConfigurationManager__g0__t126, ptr %t25, i32 0, i32 0
  store i64 1, ptr %t26
  %t27 = getelementptr inbounds %glitch.ConfigurationManager__g0__t126, ptr %t25, i32 0, i32 1
  store ptr @glitch_destroy_ConfigurationManager__g0__t126, ptr %t27
  %t28 = load ptr, ptr %t22
  call void @glitch_drop_ConfigurationManager__g0__t126(ptr %t28)
  store ptr %t25, ptr %t22
  %t29 = load ptr, ptr %t0
  %t30 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t29, i32 0, i32 6
  %t31 = getelementptr %glitch.ServiceProvider__g0__t119, ptr null, i32 1
  %t32 = ptrtoint ptr %t31 to i64
  %t33 = call ptr @glitch_calloc(i64 1, i64 %t32)
  %t34 = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %t33, i32 0, i32 0
  store i64 1, ptr %t34
  %t35 = getelementptr inbounds %glitch.ServiceProvider__g0__t119, ptr %t33, i32 0, i32 1
  store ptr @glitch_destroy_ServiceProvider__g0__t119, ptr %t35
  call void @ServiceProvider__g0__t119_ctor(ptr %t33, ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.39, i32 0, i32 2, i64 0))
  %t36 = load ptr, ptr @glitch_exception_pending
  %t37 = icmp ne ptr %t36, null
  br i1 %t37, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t38 = load ptr, ptr %t30
  call void @glitch_drop_ServiceProvider__g0__t119(ptr %t38)
  store ptr %t33, ptr %t30
  %t39 = load ptr, ptr %t0
  %t40 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t39, i32 0, i32 7
  %t41 = getelementptr %glitch.HostEnvironment__g0__t127, ptr null, i32 1
  %t42 = ptrtoint ptr %t41 to i64
  %t43 = call ptr @glitch_calloc(i64 1, i64 %t42)
  %t44 = getelementptr inbounds %glitch.HostEnvironment__g0__t127, ptr %t43, i32 0, i32 0
  store i64 1, ptr %t44
  %t45 = getelementptr inbounds %glitch.HostEnvironment__g0__t127, ptr %t43, i32 0, i32 1
  store ptr @glitch_destroy_HostEnvironment__g0__t127, ptr %t45
  %t46 = load ptr, ptr %t40
  call void @glitch_drop_HostEnvironment__g0__t127(ptr %t46)
  store ptr %t43, ptr %t40
  %t47 = load ptr, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_Use__g0(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t2, i32 0, i32 4
  %t4 = load ptr, ptr %t3
  %t5 = load ptr, ptr %t1
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 0
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 1
  %t8 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  %t9 = load i64, ptr %t6
  %t10 = load i64, ptr %t7
  %t11 = load ptr, ptr %t8
  %t12 = icmp eq i64 %t9, %t10
  br i1 %t12, label %list_grow_0, label %list_ready_1
list_grow_0:
  %t13 = mul i64 %t10, 2
  %t14 = mul i64 %t13, 8
  %t15 = call ptr @glitch_realloc(ptr %t11, i64 %t14)
  store i64 %t13, ptr %t7
  store ptr %t15, ptr %t8
  br label %list_ready_1
list_ready_1:
  %t16 = load ptr, ptr %t8
  %t17 = getelementptr inbounds ptr, ptr %t16, i64 %t9
  call void @glitch_string_retain(ptr %t5)
  store ptr %t5, ptr %t17
  %t18 = add i64 %t9, 1
  store i64 %t18, ptr %t6
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseRouting__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.40, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseEndpoints__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [10 x i8] }, ptr @.str.41, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseTrace__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.42, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseJsonEnvelope__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [14 x i8] }, ptr @.str.43, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseHttpsRedirection__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [18 x i8] }, ptr @.str.44, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseAuthorization__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [14 x i8] }, ptr @.str.45, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseAuthentication__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [15 x i8] }, ptr @.str.46, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseCors__g0__string(ptr %this, ptr %policy) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %policy, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t2, ptr getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.47, i32 0, i32 2, i64 0))
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseCors__g0__fn_CorsPolicyBuilder(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t2, ptr getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.48, i32 0, i32 2, i64 0))
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseCors__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.49, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseMiddleware__g1__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseMiddleware__g1__object(ptr %this, ptr %param) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %param, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseSwagger__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseSwagger__g0__fn_SwaggerOptions(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseSwaggerUI__g0__fn_SwaggerUiOptions(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseSwaggerUI__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseStaticFiles__g0__object(ptr %this, ptr %options) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %options, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_UseStaticFiles__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_MapControllers__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t141_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [12 x i8] }, ptr @.str.50, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_MapGet__g1__string_fn_ret_T(ptr %this, ptr %path, ptr %handler) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = alloca ptr
  store ptr %handler, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.51, i32 0, i32 2, i64 0), ptr %t4)
  %t6 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t3
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  %t10 = load ptr, ptr %t3
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 0
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 1
  %t13 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  %t14 = load i64, ptr %t11
  %t15 = load i64, ptr %t12
  %t16 = load ptr, ptr %t13
  %t17 = icmp eq i64 %t14, %t15
  br i1 %t17, label %list_grow_0, label %list_ready_1
list_grow_0:
  %t18 = mul i64 %t15, 2
  %t19 = mul i64 %t18, 8
  %t20 = call ptr @glitch_realloc(ptr %t16, i64 %t19)
  store i64 %t18, ptr %t12
  store ptr %t20, ptr %t13
  br label %list_ready_1
list_ready_1:
  %t21 = load ptr, ptr %t13
  %t22 = getelementptr inbounds ptr, ptr %t21, i64 %t14
  call void @glitch_string_retain(ptr %t10)
  store ptr %t10, ptr %t22
  %t23 = add i64 %t14, 1
  store i64 %t23, ptr %t11
  %t24 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t24)
  ret void
exception_unwind:
  %t25 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t25)
  ret void
}

define void @WebApplication__g0__t141_MapGet__g0__string_string(ptr %this, ptr %path, ptr %text) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = alloca ptr
  store ptr %text, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.52, i32 0, i32 2, i64 0), ptr %t4)
  %t6 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t3
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  %t10 = load ptr, ptr %t3
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 0
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 1
  %t13 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  %t14 = load i64, ptr %t11
  %t15 = load i64, ptr %t12
  %t16 = load ptr, ptr %t13
  %t17 = icmp eq i64 %t14, %t15
  br i1 %t17, label %list_grow_0, label %list_ready_1
list_grow_0:
  %t18 = mul i64 %t15, 2
  %t19 = mul i64 %t18, 8
  %t20 = call ptr @glitch_realloc(ptr %t16, i64 %t19)
  store i64 %t18, ptr %t12
  store ptr %t20, ptr %t13
  br label %list_ready_1
list_ready_1:
  %t21 = load ptr, ptr %t13
  %t22 = getelementptr inbounds ptr, ptr %t21, i64 %t14
  call void @glitch_string_retain(ptr %t10)
  store ptr %t10, ptr %t22
  %t23 = add i64 %t14, 1
  store i64 %t23, ptr %t11
  %t24 = load ptr, ptr %t0
  %t25 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t24, i32 0, i32 3
  %t26 = load ptr, ptr %t25
  %t27 = load ptr, ptr %t3
  %t28 = load ptr, ptr %t2
  %t29 = getelementptr inbounds %glitch.dict, ptr %t26, i32 0, i32 0
  %t30 = getelementptr inbounds %glitch.dict, ptr %t26, i32 0, i32 1
  %t31 = getelementptr inbounds %glitch.dict, ptr %t26, i32 0, i32 2
  %t32 = getelementptr inbounds %glitch.dict, ptr %t26, i32 0, i32 3
  %t33 = load i64, ptr %t29
  %t34 = load i64, ptr %t30
  %t35 = load ptr, ptr %t31
  %t36 = load ptr, ptr %t32
  %t37 = icmp eq i64 %t33, %t34
  br i1 %t37, label %dict_grow_2, label %dict_ready_3
dict_grow_2:
  %t38 = mul i64 %t34, 2
  %t39 = mul i64 %t38, 8
  %t40 = mul i64 %t38, 8
  %t41 = call ptr @glitch_realloc(ptr %t35, i64 %t39)
  %t42 = call ptr @glitch_realloc(ptr %t36, i64 %t40)
  store i64 %t38, ptr %t30
  store ptr %t41, ptr %t31
  store ptr %t42, ptr %t32
  br label %dict_ready_3
dict_ready_3:
  %t43 = load ptr, ptr %t31
  %t44 = load ptr, ptr %t32
  %t45 = getelementptr inbounds ptr, ptr %t43, i64 %t33
  %t46 = getelementptr inbounds ptr, ptr %t44, i64 %t33
  call void @glitch_string_retain(ptr %t27)
  store ptr %t27, ptr %t45
  call void @glitch_string_retain(ptr %t28)
  store ptr %t28, ptr %t46
  %t47 = add i64 %t33, 1
  store i64 %t47, ptr %t29
  %t48 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t48)
  ret void
exception_unwind:
  %t49 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t49)
  ret void
}

define void @WebApplication__g0__t141_MapPost__g1__string_fn_ret_T(ptr %this, ptr %path, ptr %handler) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = alloca ptr
  store ptr %handler, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.53, i32 0, i32 2, i64 0), ptr %t4)
  %t6 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t3
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  %t10 = load ptr, ptr %t3
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 0
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 1
  %t13 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  %t14 = load i64, ptr %t11
  %t15 = load i64, ptr %t12
  %t16 = load ptr, ptr %t13
  %t17 = icmp eq i64 %t14, %t15
  br i1 %t17, label %list_grow_0, label %list_ready_1
list_grow_0:
  %t18 = mul i64 %t15, 2
  %t19 = mul i64 %t18, 8
  %t20 = call ptr @glitch_realloc(ptr %t16, i64 %t19)
  store i64 %t18, ptr %t12
  store ptr %t20, ptr %t13
  br label %list_ready_1
list_ready_1:
  %t21 = load ptr, ptr %t13
  %t22 = getelementptr inbounds ptr, ptr %t21, i64 %t14
  call void @glitch_string_retain(ptr %t10)
  store ptr %t10, ptr %t22
  %t23 = add i64 %t14, 1
  store i64 %t23, ptr %t11
  %t24 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t24)
  ret void
exception_unwind:
  %t25 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t25)
  ret void
}

define void @WebApplication__g0__t141_MapPost__g0__string_string(ptr %this, ptr %path, ptr %text) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = alloca ptr
  store ptr %text, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.54, i32 0, i32 2, i64 0), ptr %t4)
  %t6 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t3
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  %t10 = load ptr, ptr %t3
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 0
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 1
  %t13 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  %t14 = load i64, ptr %t11
  %t15 = load i64, ptr %t12
  %t16 = load ptr, ptr %t13
  %t17 = icmp eq i64 %t14, %t15
  br i1 %t17, label %list_grow_0, label %list_ready_1
list_grow_0:
  %t18 = mul i64 %t15, 2
  %t19 = mul i64 %t18, 8
  %t20 = call ptr @glitch_realloc(ptr %t16, i64 %t19)
  store i64 %t18, ptr %t12
  store ptr %t20, ptr %t13
  br label %list_ready_1
list_ready_1:
  %t21 = load ptr, ptr %t13
  %t22 = getelementptr inbounds ptr, ptr %t21, i64 %t14
  call void @glitch_string_retain(ptr %t10)
  store ptr %t10, ptr %t22
  %t23 = add i64 %t14, 1
  store i64 %t23, ptr %t11
  %t24 = load ptr, ptr %t0
  %t25 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t24, i32 0, i32 3
  %t26 = load ptr, ptr %t25
  %t27 = load ptr, ptr %t3
  %t28 = load ptr, ptr %t2
  %t29 = getelementptr inbounds %glitch.dict, ptr %t26, i32 0, i32 0
  %t30 = getelementptr inbounds %glitch.dict, ptr %t26, i32 0, i32 1
  %t31 = getelementptr inbounds %glitch.dict, ptr %t26, i32 0, i32 2
  %t32 = getelementptr inbounds %glitch.dict, ptr %t26, i32 0, i32 3
  %t33 = load i64, ptr %t29
  %t34 = load i64, ptr %t30
  %t35 = load ptr, ptr %t31
  %t36 = load ptr, ptr %t32
  %t37 = icmp eq i64 %t33, %t34
  br i1 %t37, label %dict_grow_2, label %dict_ready_3
dict_grow_2:
  %t38 = mul i64 %t34, 2
  %t39 = mul i64 %t38, 8
  %t40 = mul i64 %t38, 8
  %t41 = call ptr @glitch_realloc(ptr %t35, i64 %t39)
  %t42 = call ptr @glitch_realloc(ptr %t36, i64 %t40)
  store i64 %t38, ptr %t30
  store ptr %t41, ptr %t31
  store ptr %t42, ptr %t32
  br label %dict_ready_3
dict_ready_3:
  %t43 = load ptr, ptr %t31
  %t44 = load ptr, ptr %t32
  %t45 = getelementptr inbounds ptr, ptr %t43, i64 %t33
  %t46 = getelementptr inbounds ptr, ptr %t44, i64 %t33
  call void @glitch_string_retain(ptr %t27)
  store ptr %t27, ptr %t45
  call void @glitch_string_retain(ptr %t28)
  store ptr %t28, ptr %t46
  %t47 = add i64 %t33, 1
  store i64 %t47, ptr %t29
  %t48 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t48)
  ret void
exception_unwind:
  %t49 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t49)
  ret void
}

define ptr @WebApplication__g0__t141_Handle__g0(ptr %this, ptr %method, ptr %path, ptr %body) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %method, ptr %t1
  %t2 = alloca ptr
  store ptr %path, ptr %t2
  %t3 = alloca ptr
  store ptr %body, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = alloca ptr
  store ptr null, ptr %t6
  %t7 = load ptr, ptr %t1
  %t8 = call ptr @glitch_string_concat(ptr %t7, ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.55, i32 0, i32 2, i64 0))
  %t9 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t9)
  store ptr %t8, ptr %t4
  %t10 = load ptr, ptr %t4
  %t11 = load ptr, ptr %t2
  %t12 = call ptr @glitch_string_concat(ptr %t10, ptr %t11)
  %t13 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t13)
  store ptr %t12, ptr %t5
  %t14 = load ptr, ptr %t0
  %t15 = load ptr, ptr %t1
  %t16 = load ptr, ptr %t2
  %t17 = call i1 @glitch_endpoint_handlers_contains(ptr %t14, ptr %t15, ptr %t16)
  br i1 %t17, label %if_then_0, label %if_else_1
if_then_0:
  %t18 = load ptr, ptr %t0
  %t19 = load ptr, ptr %t1
  %t20 = load ptr, ptr %t2
  %t21 = load ptr, ptr %t3
  %t22 = call ptr @glitch_endpoint_handlers_invoke(ptr %t18, ptr %t19, ptr %t20, ptr %t21)
  %t23 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t23)
  store ptr %t22, ptr %t6
  %t24 = load ptr, ptr %t0
  %t25 = load ptr, ptr %t6
  %t26 = call ptr @WebApplication__g0__t141_ApplyMiddleware__g0(ptr %t24, ptr %t25)
  %t27 = load ptr, ptr @glitch_exception_pending
  %t28 = icmp ne ptr %t27, null
  br i1 %t28, label %exception_unwind, label %call_continue_3
call_continue_3:
  %t29 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t29)
  %t30 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t30)
  %t31 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t31)
  ret ptr %t26
if_else_1:
  %t32 = load ptr, ptr %t0
  %t33 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t32, i32 0, i32 3
  %t34 = load ptr, ptr %t33
  %t35 = load ptr, ptr %t5
  %t36 = getelementptr inbounds %glitch.dict, ptr %t34, i32 0, i32 0
  %t37 = getelementptr inbounds %glitch.dict, ptr %t34, i32 0, i32 2
  %t38 = load i64, ptr %t36
  %t39 = load ptr, ptr %t37
  %t40 = alloca i64
  %t41 = alloca i1
  store i64 0, ptr %t40
  store i1 false, ptr %t41
  br label %dict_find_loop_4
dict_find_loop_4:
  %t42 = load i64, ptr %t40
  %t43 = icmp ult i64 %t42, %t38
  br i1 %t43, label %dict_find_inspect_5, label %dict_find_done_8
dict_find_inspect_5:
  %t44 = getelementptr inbounds ptr, ptr %t39, i64 %t42
  %t45 = load ptr, ptr %t44
  %t50 = call i32 @strcmp(ptr %t45, ptr %t35)
  %t46 = icmp eq i32 %t50, 0
  br i1 %t46, label %dict_find_found_6, label %dict_find_advance_7
dict_find_advance_7:
  %t47 = add i64 %t42, 1
  store i64 %t47, ptr %t40
  br label %dict_find_loop_4
dict_find_found_6:
  store i1 true, ptr %t41
  br label %dict_find_done_8
dict_find_done_8:
  %t48 = load i64, ptr %t40
  %t49 = load i1, ptr %t41
  br i1 %t49, label %if_then_9, label %if_else_10
if_then_9:
  %t51 = load ptr, ptr %t0
  %t52 = load ptr, ptr %t0
  %t53 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t52, i32 0, i32 3
  %t54 = load ptr, ptr %t53
  %t55 = load ptr, ptr %t5
  %t56 = getelementptr inbounds %glitch.dict, ptr %t54, i32 0, i32 0
  %t57 = getelementptr inbounds %glitch.dict, ptr %t54, i32 0, i32 2
  %t58 = load i64, ptr %t56
  %t59 = load ptr, ptr %t57
  %t60 = alloca i64
  %t61 = alloca i1
  store i64 0, ptr %t60
  store i1 false, ptr %t61
  br label %dict_find_loop_12
dict_find_loop_12:
  %t62 = load i64, ptr %t60
  %t63 = icmp ult i64 %t62, %t58
  br i1 %t63, label %dict_find_inspect_13, label %dict_find_done_16
dict_find_inspect_13:
  %t64 = getelementptr inbounds ptr, ptr %t59, i64 %t62
  %t65 = load ptr, ptr %t64
  %t70 = call i32 @strcmp(ptr %t65, ptr %t55)
  %t66 = icmp eq i32 %t70, 0
  br i1 %t66, label %dict_find_found_14, label %dict_find_advance_15
dict_find_advance_15:
  %t67 = add i64 %t62, 1
  store i64 %t67, ptr %t60
  br label %dict_find_loop_12
dict_find_found_14:
  store i1 true, ptr %t61
  br label %dict_find_done_16
dict_find_done_16:
  %t68 = load i64, ptr %t60
  %t69 = load i1, ptr %t61
  %t75 = alloca ptr
  br i1 %t69, label %dict_index_load_18, label %dict_index_default_17
dict_index_load_18:
  %t71 = getelementptr inbounds %glitch.dict, ptr %t54, i32 0, i32 3
  %t72 = load ptr, ptr %t71
  %t73 = getelementptr inbounds ptr, ptr %t72, i64 %t68
  %t74 = load ptr, ptr %t73
  store ptr %t74, ptr %t75
  br label %dict_index_done_19
dict_index_default_17:
  store ptr null, ptr %t75
  br label %dict_index_done_19
dict_index_done_19:
  %t76 = load ptr, ptr %t75
  %t77 = call ptr @WebApplication__g0__t141_ApplyMiddleware__g0(ptr %t51, ptr %t76)
  %t78 = load ptr, ptr @glitch_exception_pending
  %t79 = icmp ne ptr %t78, null
  br i1 %t79, label %exception_unwind, label %call_continue_20
call_continue_20:
  %t80 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t80)
  %t81 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t81)
  %t82 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t82)
  ret ptr %t77
if_else_10:
  %t83 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t83)
  %t84 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t84)
  %t85 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t85)
  ret ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.56, i32 0, i32 2, i64 0)
if_end_11:
  br label %if_end_2
if_end_2:
  %t86 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t86)
  %t87 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t87)
  %t88 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t88)
  ret ptr null
exception_unwind:
  %t89 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t89)
  %t90 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t90)
  %t91 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t91)
  ret ptr null
}

define ptr @WebApplication__g0__t141_ApplyMiddleware__g0(ptr %this, ptr %text) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %text, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = load ptr, ptr %t0
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t2
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t8, i32 0, i32 4
  %t10 = load ptr, ptr %t9
  %t11 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 0
  %t12 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 2
  %t13 = load i64, ptr %t11
  %t14 = load ptr, ptr %t12
  %t15 = alloca i64
  %t16 = alloca i1
  store i64 0, ptr %t15
  store i1 false, ptr %t16
  br label %list_contains_loop_0
list_contains_loop_0:
  %t17 = load i64, ptr %t15
  %t18 = icmp ult i64 %t17, %t13
  br i1 %t18, label %list_contains_next_2, label %list_contains_done_3
list_contains_next_2:
  %t19 = getelementptr inbounds ptr, ptr %t14, i64 %t17
  %t20 = load ptr, ptr %t19
  %t24 = call i32 @strcmp(ptr %t20, ptr getelementptr inbounds ({ i64, i64, [14 x i8] }, ptr @.str.57, i32 0, i32 2, i64 0))
  %t21 = icmp eq i32 %t24, 0
  br i1 %t21, label %list_contains_found_1, label %list_contains_advance_4
list_contains_advance_4:
  %t22 = add i64 %t17, 1
  store i64 %t22, ptr %t15
  br label %list_contains_loop_0
list_contains_found_1:
  store i1 true, ptr %t16
  br label %list_contains_done_3
list_contains_done_3:
  %t23 = load i1, ptr %t16
  br i1 %t23, label %if_then_5, label %if_else_6
if_then_5:
  %t25 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t25)
  store ptr getelementptr inbounds ({ i64, i64, [11 x i8] }, ptr @.str.58, i32 0, i32 2, i64 0), ptr %t3
  %t26 = load ptr, ptr %t3
  %t27 = load ptr, ptr %t2
  %t28 = call ptr @glitch_string_concat(ptr %t26, ptr %t27)
  %t29 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t29)
  store ptr %t28, ptr %t4
  %t30 = load ptr, ptr %t4
  %t31 = call ptr @glitch_string_concat(ptr %t30, ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.59, i32 0, i32 2, i64 0))
  %t32 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t32)
  %t33 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t33)
  %t34 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t34)
  ret ptr %t31
if_else_6:
  br label %if_end_7
if_end_7:
  %t35 = load ptr, ptr %t0
  %t36 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t35, i32 0, i32 4
  %t37 = load ptr, ptr %t36
  %t38 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 0
  %t39 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 2
  %t40 = load i64, ptr %t38
  %t41 = load ptr, ptr %t39
  %t42 = alloca i64
  %t43 = alloca i1
  store i64 0, ptr %t42
  store i1 false, ptr %t43
  br label %list_contains_loop_8
list_contains_loop_8:
  %t44 = load i64, ptr %t42
  %t45 = icmp ult i64 %t44, %t40
  br i1 %t45, label %list_contains_next_10, label %list_contains_done_11
list_contains_next_10:
  %t46 = getelementptr inbounds ptr, ptr %t41, i64 %t44
  %t47 = load ptr, ptr %t46
  %t51 = call i32 @strcmp(ptr %t47, ptr getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.60, i32 0, i32 2, i64 0))
  %t48 = icmp eq i32 %t51, 0
  br i1 %t48, label %list_contains_found_9, label %list_contains_advance_12
list_contains_advance_12:
  %t49 = add i64 %t44, 1
  store i64 %t49, ptr %t42
  br label %list_contains_loop_8
list_contains_found_9:
  store i1 true, ptr %t43
  br label %list_contains_done_11
list_contains_done_11:
  %t50 = load i1, ptr %t43
  br i1 %t50, label %if_then_13, label %if_else_14
if_then_13:
  %t52 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t52)
  store ptr getelementptr inbounds ({ i64, i64, [7 x i8] }, ptr @.str.61, i32 0, i32 2, i64 0), ptr %t3
  %t53 = load ptr, ptr %t3
  %t54 = load ptr, ptr %t2
  %t55 = call ptr @glitch_string_concat(ptr %t53, ptr %t54)
  %t56 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t56)
  %t57 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t57)
  %t58 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t58)
  ret ptr %t55
if_else_14:
  br label %if_end_15
if_end_15:
  %t59 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t59)
  %t60 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t60)
  %t61 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t61)
  ret ptr %t59
exception_unwind:
  %t62 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t62)
  %t63 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t63)
  %t64 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t64)
  ret ptr null
}

define void @WebApplication__g0__t141_Run__g0__int(ptr %this, i32 %port) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %port, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load i32, ptr %t1
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [25 x i8] }, ptr @.str.62, i32 0, i32 2, i64 0))
  call void @GlitchRestHost_Run(ptr %t2, i32 %t3, i32 0, ptr @WebApplication_Handle, ptr @glitch_string_release)
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_Run__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [17 x i8] }, ptr @.str.63, i32 0, i32 2, i64 0))
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [25 x i8] }, ptr @.str.64, i32 0, i32 2, i64 0))
  call void @GlitchRestHost_Run(ptr %t1, i32 0, i32 0, ptr @WebApplication_Handle, ptr @glitch_string_release)
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t141_RunOnce__g0(ptr %this, i32 %port) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %port, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load i32, ptr %t1
  %t4 = trunc i64 1 to i32
  call void @GlitchRestHost_Run(ptr %t2, i32 %t3, i32 %t4, ptr @WebApplication_Handle, ptr @glitch_string_release)
  ret void
exception_unwind:
  ret void
}

define void @PhysicalFileProvider__g0__t143_ctor(ptr %this, ptr %path) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ApiVersion__g0__t144_ctor(ptr %this, i32 %major, i32 %minor, ptr %status) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %major, ptr %t1
  %t2 = alloca i32
  store i32 %minor, ptr %t2
  %t3 = alloca ptr
  store ptr %status, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @ProblemDetails__g0__t145_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ProblemDetails__g0__t145, ptr %t1, i32 0, i32 6
  %t3 = call ptr @glitch_calloc(i64 1, i64 32)
  %t4 = call ptr @glitch_calloc(i64 4, i64 8)
  %t5 = call ptr @glitch_calloc(i64 4, i64 8)
  %t6 = getelementptr inbounds %glitch.dict, ptr %t3, i32 0, i32 1
  store i64 4, ptr %t6
  %t7 = getelementptr inbounds %glitch.dict, ptr %t3, i32 0, i32 2
  store ptr %t4, ptr %t7
  %t8 = getelementptr inbounds %glitch.dict, ptr %t3, i32 0, i32 3
  store ptr %t5, ptr %t8
  store ptr %t3, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @ObjectResult__g0__t146_ctor(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ModelStateEntry__g0__t151_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ModelStateEntry__g0__t151, ptr %t1, i32 0, i32 2
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

define void @ModelStateDictionary__g0__t152_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %t1, i32 0, i32 2
  store i1 1, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %t3, i32 0, i32 3
  %t5 = call ptr @glitch_calloc(i64 1, i64 32)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = call ptr @glitch_calloc(i64 4, i64 8)
  %t8 = getelementptr inbounds %glitch.dict, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t8
  %t9 = getelementptr inbounds %glitch.dict, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t9
  %t10 = getelementptr inbounds %glitch.dict, ptr %t5, i32 0, i32 3
  store ptr %t7, ptr %t10
  store ptr %t5, ptr %t4
  ret void
exception_unwind:
  ret void
}

define void @ModelStateDictionary__g0__t152_AddModelError__g0(ptr %this, ptr %key, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %message, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %t4, i32 0, i32 2
  store i1 0, ptr %t5
  call void @glitch_retain_ModelStateEntry__g0__t151(ptr null)
  %t6 = load ptr, ptr %t3
  call void @glitch_drop_ModelStateEntry__g0__t151(ptr %t6)
  store ptr null, ptr %t3
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t8
  %t10 = load ptr, ptr %t1
  %t11 = getelementptr inbounds %glitch.dict, ptr %t9, i32 0, i32 0
  %t12 = getelementptr inbounds %glitch.dict, ptr %t9, i32 0, i32 2
  %t13 = load i64, ptr %t11
  %t14 = load ptr, ptr %t12
  %t15 = alloca i64
  %t16 = alloca i1
  store i64 0, ptr %t15
  store i1 false, ptr %t16
  br label %dict_find_loop_0
dict_find_loop_0:
  %t17 = load i64, ptr %t15
  %t18 = icmp ult i64 %t17, %t13
  br i1 %t18, label %dict_find_inspect_1, label %dict_find_done_4
dict_find_inspect_1:
  %t19 = getelementptr inbounds ptr, ptr %t14, i64 %t17
  %t20 = load ptr, ptr %t19
  %t25 = call i32 @strcmp(ptr %t20, ptr %t10)
  %t21 = icmp eq i32 %t25, 0
  br i1 %t21, label %dict_find_found_2, label %dict_find_advance_3
dict_find_advance_3:
  %t22 = add i64 %t17, 1
  store i64 %t22, ptr %t15
  br label %dict_find_loop_0
dict_find_found_2:
  store i1 true, ptr %t16
  br label %dict_find_done_4
dict_find_done_4:
  %t23 = load i64, ptr %t15
  %t24 = load i1, ptr %t16
  store ptr null, ptr %t3
  br i1 %t24, label %dict_tryget_load_5, label %dict_tryget_done_6

dict_tryget_load_5:
  %t26 = getelementptr inbounds %glitch.dict, ptr %t9, i32 0, i32 3
  %t27 = load ptr, ptr %t26
  %t28 = getelementptr inbounds ptr, ptr %t27, i64 %t23
  %t29 = load ptr, ptr %t28
  store ptr %t29, ptr %t3
  br label %dict_tryget_done_6

dict_tryget_done_6:
  %t30 = xor i1 %t24, true
  br i1 %t30, label %if_then_7, label %if_else_8
if_then_7:
  %t31 = getelementptr %glitch.ModelStateEntry__g0__t151, ptr null, i32 1
  %t32 = ptrtoint ptr %t31 to i64
  %t33 = call ptr @glitch_calloc(i64 1, i64 %t32)
  %t34 = getelementptr inbounds %glitch.ModelStateEntry__g0__t151, ptr %t33, i32 0, i32 0
  store i64 1, ptr %t34
  %t35 = getelementptr inbounds %glitch.ModelStateEntry__g0__t151, ptr %t33, i32 0, i32 1
  store ptr @glitch_destroy_ModelStateEntry__g0__t151, ptr %t35
  call void @ModelStateEntry__g0__t151_ctor(ptr %t33)
  %t36 = load ptr, ptr @glitch_exception_pending
  %t37 = icmp ne ptr %t36, null
  br i1 %t37, label %exception_unwind, label %call_continue_10
call_continue_10:
  %t38 = load ptr, ptr %t3
  call void @glitch_drop_ModelStateEntry__g0__t151(ptr %t38)
  store ptr %t33, ptr %t3
  %t39 = load ptr, ptr %t0
  %t40 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %t39, i32 0, i32 3
  %t41 = load ptr, ptr %t40
  %t42 = load ptr, ptr %t1
  %t43 = load ptr, ptr %t3
  %t44 = getelementptr inbounds %glitch.dict, ptr %t41, i32 0, i32 0
  %t45 = getelementptr inbounds %glitch.dict, ptr %t41, i32 0, i32 1
  %t46 = getelementptr inbounds %glitch.dict, ptr %t41, i32 0, i32 2
  %t47 = getelementptr inbounds %glitch.dict, ptr %t41, i32 0, i32 3
  %t48 = load i64, ptr %t44
  %t49 = load i64, ptr %t45
  %t50 = load ptr, ptr %t46
  %t51 = load ptr, ptr %t47
  %t52 = icmp eq i64 %t48, %t49
  br i1 %t52, label %dict_grow_11, label %dict_ready_12
dict_grow_11:
  %t53 = mul i64 %t49, 2
  %t54 = mul i64 %t53, 8
  %t55 = mul i64 %t53, 8
  %t56 = call ptr @glitch_realloc(ptr %t50, i64 %t54)
  %t57 = call ptr @glitch_realloc(ptr %t51, i64 %t55)
  store i64 %t53, ptr %t45
  store ptr %t56, ptr %t46
  store ptr %t57, ptr %t47
  br label %dict_ready_12
dict_ready_12:
  %t58 = load ptr, ptr %t46
  %t59 = load ptr, ptr %t47
  %t60 = getelementptr inbounds ptr, ptr %t58, i64 %t48
  %t61 = getelementptr inbounds ptr, ptr %t59, i64 %t48
  call void @glitch_string_retain(ptr %t42)
  store ptr %t42, ptr %t60
  call void @glitch_retain_ModelStateEntry__g0__t151(ptr %t43)
  store ptr %t43, ptr %t61
  %t62 = add i64 %t48, 1
  store i64 %t62, ptr %t44
  br label %if_end_9
if_else_8:
  br label %if_end_9
if_end_9:
  %t63 = load ptr, ptr %t3
  %t64 = getelementptr inbounds %glitch.ModelStateEntry__g0__t151, ptr %t63, i32 0, i32 2
  %t65 = load ptr, ptr %t64
  %t66 = getelementptr %glitch.ModelError__g0__t150, ptr null, i32 1
  %t67 = ptrtoint ptr %t66 to i64
  %t68 = call ptr @glitch_calloc(i64 1, i64 %t67)
  %t69 = getelementptr inbounds %glitch.ModelError__g0__t150, ptr %t68, i32 0, i32 0
  store i64 1, ptr %t69
  %t70 = getelementptr inbounds %glitch.ModelError__g0__t150, ptr %t68, i32 0, i32 1
  store ptr @glitch_destroy_ModelError__g0__t150, ptr %t70
  %t71 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t71)
  %t72 = getelementptr inbounds %glitch.ModelError__g0__t150, ptr %t68, i32 0, i32 2
  store ptr %t71, ptr %t72
  %t73 = getelementptr inbounds %glitch.list, ptr %t65, i32 0, i32 0
  %t74 = getelementptr inbounds %glitch.list, ptr %t65, i32 0, i32 1
  %t75 = getelementptr inbounds %glitch.list, ptr %t65, i32 0, i32 2
  %t76 = load i64, ptr %t73
  %t77 = load i64, ptr %t74
  %t78 = load ptr, ptr %t75
  %t79 = icmp eq i64 %t76, %t77
  br i1 %t79, label %list_grow_13, label %list_ready_14
list_grow_13:
  %t80 = mul i64 %t77, 2
  %t81 = mul i64 %t80, 8
  %t82 = call ptr @glitch_realloc(ptr %t78, i64 %t81)
  store i64 %t80, ptr %t74
  store ptr %t82, ptr %t75
  br label %list_ready_14
list_ready_14:
  %t83 = load ptr, ptr %t75
  %t84 = getelementptr inbounds ptr, ptr %t83, i64 %t76
  store ptr %t68, ptr %t84
  %t85 = add i64 %t76, 1
  store i64 %t85, ptr %t73
  %t86 = load ptr, ptr %t3
  call void @glitch_drop_ModelStateEntry__g0__t151(ptr %t86)
  ret void
exception_unwind:
  %t87 = load ptr, ptr %t3
  call void @glitch_drop_ModelStateEntry__g0__t151(ptr %t87)
  ret void
}

define ptr @ModelStateDictionary__g0__t152_ToDictionary__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %t1, i32 0, i32 3
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @ControllerBase__g0__t153_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ControllerBase__g0__t153, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.ModelStateDictionary__g0__t152, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t152, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_ModelStateDictionary__g0__t152, ptr %t7
  call void @ModelStateDictionary__g0__t152_ctor(ptr %t5)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = load ptr, ptr %t2
  call void @glitch_drop_ModelStateDictionary__g0__t152(ptr %t10)
  store ptr %t5, ptr %t2
  ret void
exception_unwind:
  ret void
}

define ptr @ControllerBase__g0__t153_Ok__g0__object(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t1
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @ControllerBase__g0__t153_Ok__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.65, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define ptr @ControllerBase__g0__t153_BadRequest__g0(ptr %this, ptr %error) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %error, ptr %t1
  %t2 = load ptr, ptr %t1
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @ControllerBase__g0__t153_NotFound__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr getelementptr inbounds ({ i64, i64, [10 x i8] }, ptr @.str.66, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define ptr @ControllerBase__g0__t153_StatusCode__g0(ptr %this, i32 %code, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %code, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  %t3 = load ptr, ptr %t2
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
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

define ptr @SetString(ptr %context, ptr %table) {
entry:
  %t0 = alloca ptr
  store ptr %context, ptr %t0
  %t1 = alloca ptr
  store ptr %table, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t60_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = getelementptr %glitch.DbSetString__g0__t62, ptr null, i32 1
  %t6 = ptrtoint ptr %t5 to i64
  %t7 = call ptr @glitch_calloc(i64 1, i64 %t6)
  %t8 = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %t7, i32 0, i32 0
  store i64 1, ptr %t8
  %t9 = getelementptr inbounds %glitch.DbSetString__g0__t62, ptr %t7, i32 0, i32 1
  store ptr @glitch_destroy_DbSetString__g0__t62, ptr %t9
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.DbContext__g0__t60, ptr %t10, i32 0, i32 2
  %t12 = load ptr, ptr %t11
  %t13 = load ptr, ptr %t1
  call void @DbSetString__g0__t62_ctor(ptr %t7, ptr %t12, ptr %t13)
  %t14 = load ptr, ptr @glitch_exception_pending
  %t15 = icmp ne ptr %t14, null
  br i1 %t15, label %exception_unwind, label %call_continue_1
call_continue_1:
  ret ptr %t7
exception_unwind:
  ret ptr null
}

define ptr @EnableRetryOnFailure(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @CommandTimeout(ptr %target, i32 %timeout) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca i32
  store i32 %timeout, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @HasDefaultSchema(ptr %target, ptr %schema) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %schema, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Entity__object_string(ptr %target, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Entity__object_string_object(ptr %target, ptr %name, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %configure, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @HasAnnotation(ptr %target, ptr %name, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @UseIdentityColumns(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @EnsureSchema(ptr %target, ptr %schema) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %schema, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @CreateTable__object_string_object(ptr %target, ptr %name, ptr %columns) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %columns, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @CreateTable__object_string_object_object(ptr %target, ptr %name, ptr %columns, ptr %constraints) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %columns, ptr %t2
  %t3 = alloca ptr
  store ptr %constraints, ptr %t3
  %t4 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @CreateTable__object_string_object_object_string(ptr %target, ptr %name, ptr %columns, ptr %constraints, ptr %schema) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %columns, ptr %t2
  %t3 = alloca ptr
  store ptr %constraints, ptr %t3
  %t4 = alloca ptr
  store ptr %schema, ptr %t4
  %t5 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t5)
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @CreateTable__object_string_string_object_object(ptr %target, ptr %name, ptr %schema, ptr %columns, ptr %constraints) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %schema, ptr %t2
  %t3 = alloca ptr
  store ptr %columns, ptr %t3
  %t4 = alloca ptr
  store ptr %constraints, ptr %t4
  %t5 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t5)
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @CreateIndex__object_string_object(ptr %target, ptr %name, ptr %columns) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %columns, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @CreateIndex__object_string_string_object(ptr %target, ptr %name, ptr %table, ptr %columns) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %table, ptr %t2
  %t3 = alloca ptr
  store ptr %columns, ptr %t3
  %t4 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @CreateIndex__object_string_string_object_bool(ptr %target, ptr %name, ptr %table, ptr %columns, i1 %unique) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %table, ptr %t2
  %t3 = alloca ptr
  store ptr %columns, ptr %t3
  %t4 = alloca i1
  store i1 %unique, ptr %t4
  %t5 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t5)
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @CreateIndex__object_string_string_string_object(ptr %target, ptr %name, ptr %schema, ptr %table, ptr %columns) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %schema, ptr %t2
  %t3 = alloca ptr
  store ptr %table, ptr %t3
  %t4 = alloca ptr
  store ptr %columns, ptr %t4
  %t5 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t5)
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @CreateIndex__object_string_string_string_object_bool(ptr %target, ptr %name, ptr %schema, ptr %table, ptr %columns, i1 %unique) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %schema, ptr %t2
  %t3 = alloca ptr
  store ptr %table, ptr %t3
  %t4 = alloca ptr
  store ptr %columns, ptr %t4
  %t5 = alloca i1
  store i1 %unique, ptr %t5
  %t6 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t6)
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @CreateIndex__object_string_string_object_string_bool(ptr %target, ptr %name, ptr %table, ptr %columns, ptr %schema, i1 %unique) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %table, ptr %t2
  %t3 = alloca ptr
  store ptr %columns, ptr %t3
  %t4 = alloca ptr
  store ptr %schema, ptr %t4
  %t5 = alloca i1
  store i1 %unique, ptr %t5
  %t6 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t6)
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @DropTable__object_string(ptr %target, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @DropTable__object_string_string(ptr %target, ptr %name, ptr %schema) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %schema, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__g1__object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t6)
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__g1__object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t7)
  ret ptr %t7
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__g1__object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = alloca ptr
  store ptr %arg7, ptr %t7
  %t8 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t8)
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__g1__object_object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7, ptr %arg8) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = alloca ptr
  store ptr %arg7, ptr %t7
  %t8 = alloca ptr
  store ptr %arg8, ptr %t8
  %t9 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t9)
  ret ptr %t9
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__g1__object_object_object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7, ptr %arg8, ptr %arg9) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = alloca ptr
  store ptr %arg7, ptr %t7
  %t8 = alloca ptr
  store ptr %arg8, ptr %t8
  %t9 = alloca ptr
  store ptr %arg9, ptr %t9
  %t10 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t10)
  ret ptr %t10
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__g1__object_object_object_object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7, ptr %arg8, ptr %arg9, ptr %arg10) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = alloca ptr
  store ptr %arg7, ptr %t7
  %t8 = alloca ptr
  store ptr %arg8, ptr %t8
  %t9 = alloca ptr
  store ptr %arg9, ptr %t9
  %t10 = alloca ptr
  store ptr %arg10, ptr %t10
  %t11 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t11)
  ret ptr %t11
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__object_string_object(ptr %target, ptr %name, ptr %column) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %column, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DropIndex__object_string(ptr %target, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @DropIndex__object_string_string(ptr %target, ptr %name, ptr %schema) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %schema, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DropIndex__object_string_string_string(ptr %target, ptr %name, ptr %schema, ptr %table) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %schema, ptr %t2
  %t3 = alloca ptr
  store ptr %table, ptr %t3
  %t4 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__g1__object_object_object(ptr %target, ptr %arg1, ptr %arg2) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__g1__object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__g1__object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t5)
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__g1__object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t6)
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__g1__object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t7)
  ret ptr %t7
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__g1__object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = alloca ptr
  store ptr %arg7, ptr %t7
  %t8 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t8)
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__object_string_object(ptr %target, ptr %name, ptr %column) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %column, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DropColumn__object_string(ptr %target, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @DropColumn__object_string_string(ptr %target, ptr %name, ptr %schema) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %schema, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DropColumn__object_string_string_string(ptr %target, ptr %name, ptr %schema, ptr %table) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %schema, ptr %t2
  %t3 = alloca ptr
  store ptr %table, ptr %t3
  %t4 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RenameColumn__object_string_string(ptr %target, ptr %name, ptr %newName) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %newName, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RenameColumn__object_string_string_string_string(ptr %target, ptr %table, ptr %name, ptr %newName, ptr %schema) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %table, ptr %t1
  %t2 = alloca ptr
  store ptr %name, ptr %t2
  %t3 = alloca ptr
  store ptr %newName, ptr %t3
  %t4 = alloca ptr
  store ptr %schema, ptr %t4
  %t5 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t5)
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @RenameColumn__object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t5)
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @RenameColumn__object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t6)
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @AddForeignKey__object_object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7, ptr %arg8) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = alloca ptr
  store ptr %arg7, ptr %t7
  %t8 = alloca ptr
  store ptr %arg8, ptr %t8
  %t9 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t9)
  ret ptr %t9
exception_unwind:
  ret ptr null
}

define ptr @AddForeignKey__object_object_object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7, ptr %arg8, ptr %arg9) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = alloca ptr
  store ptr %arg7, ptr %t7
  %t8 = alloca ptr
  store ptr %arg8, ptr %t8
  %t9 = alloca ptr
  store ptr %arg9, ptr %t9
  %t10 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t10)
  ret ptr %t10
exception_unwind:
  ret ptr null
}

define ptr @AddForeignKey__object_object_object_object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7, ptr %arg8, ptr %arg9, ptr %arg10) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = alloca ptr
  store ptr %arg3, ptr %t3
  %t4 = alloca ptr
  store ptr %arg4, ptr %t4
  %t5 = alloca ptr
  store ptr %arg5, ptr %t5
  %t6 = alloca ptr
  store ptr %arg6, ptr %t6
  %t7 = alloca ptr
  store ptr %arg7, ptr %t7
  %t8 = alloca ptr
  store ptr %arg8, ptr %t8
  %t9 = alloca ptr
  store ptr %arg9, ptr %t9
  %t10 = alloca ptr
  store ptr %arg10, ptr %t10
  %t11 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t11)
  ret ptr %t11
exception_unwind:
  ret ptr null
}

define ptr @AddForeignKey__object_string_string_string_string_string(ptr %target, ptr %name, ptr %table, ptr %column, ptr %principalTable, ptr %principalColumn) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %table, ptr %t2
  %t3 = alloca ptr
  store ptr %column, ptr %t3
  %t4 = alloca ptr
  store ptr %principalTable, ptr %t4
  %t5 = alloca ptr
  store ptr %principalColumn, ptr %t5
  %t6 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t6)
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @DropForeignKey__object_string_string(ptr %target, ptr %name, ptr %table) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %table, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DropForeignKey__object_string_string_string(ptr %target, ptr %name, ptr %table, ptr %schema) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %table, ptr %t2
  %t3 = alloca ptr
  store ptr %schema, ptr %t3
  %t4 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @Annotation(ptr %target, ptr %name, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Column(ptr %target, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @HasDefaultValueSql(ptr %target, ptr %sql) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %sql, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Property(ptr %target, ptr %prop) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %prop, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @HasConversion(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @IsUnique(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @HasIndex(ptr %target, ptr %index) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %index, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @UsingEntity(ptr %target, ptr %entity) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @WithMany(ptr %target, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @HasMany(ptr %target, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @OnDelete(ptr %target, ptr %behavior) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %behavior, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @HasForeignKey(ptr %target, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @HasOne(ptr %target, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @AsQueryable(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @IgnoreQueryFilters(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @Where(ptr %target, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Take(ptr %target, i32 %count) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca i32
  store i32 %count, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Skip(ptr %target, i32 %count) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca i32
  store i32 %count, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @ToList(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @Include(ptr %target, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @ThenInclude(ptr %target, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @HasDefaultValue(ptr %target, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @HasColumnType(ptr %target, ptr %type) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %type, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Any__object(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @Any__object_object(ptr %target, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @ToDictionary__object_object(ptr %target, ptr %keySelector) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %keySelector, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @ToDictionary__object_object_object(ptr %target, ptr %keySelector, ptr %elementSelector) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %keySelector, ptr %t1
  %t2 = alloca ptr
  store ptr %elementSelector, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @ToArray(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @Select(ptr %target, ptr %selector) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %selector, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @FirstOrDefault__object(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @FirstOrDefault__object_object(ptr %target, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @FirstOrDefaultAsync__object(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @FirstOrDefaultAsync__object_object(ptr %target, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @SingleOrDefaultAsync__object(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @SingleOrDefaultAsync__object_object(ptr %target, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @SaveChangesAsync__overload() {
entry:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @SaveChangesAsync__object(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @AddRangeAsync__object_object(ptr %target, ptr %entities) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %entities, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @AddRangeAsync__object_object_object(ptr %target, ptr %entities, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %entities, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @AddAsync(ptr %target, ptr %entity) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @AsNoTrackingWithIdentityResolution(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @FromSqlRaw(ptr %target, ptr %sql, ptr %parameters) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %sql, ptr %t1
  %t2 = alloca ptr
  store ptr %parameters, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @ExecuteSqlRaw(ptr %target, ptr %sql, ptr %parameters) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %sql, ptr %t1
  %t2 = alloca ptr
  store ptr %parameters, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @SystemTextJson_SerializeString(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @SystemTextJson_DeserializeString(ptr %json) {
entry:
  %t0 = alloca ptr
  store ptr %json, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @JsonSerializer_SerializeString(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @JsonSerializer_DeserializeString(ptr %json) {
entry:
  %t0 = alloca ptr
  store ptr %json, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Serialize(ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %value, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Deserialize(ptr %json) {
entry:
  %t0 = alloca ptr
  store ptr %json, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @CreateBuilder(ptr %args) {
entry:
  %t0 = alloca ptr
  store ptr %args, ptr %t0
  %t1 = getelementptr %glitch.WebApplicationBuilder__g0__t129, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t129, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_WebApplicationBuilder__g0__t129, ptr %t5
  call void @WebApplicationBuilder__g0__t129_ctor(ptr %t3)
  %t6 = load ptr, ptr @glitch_exception_pending
  %t7 = icmp ne ptr %t6, null
  br i1 %t7, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @BuildHealth() {
entry:
  ret ptr getelementptr inbounds ({ i64, i64, [16 x i8] }, ptr @.str.67, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define ptr @HealthAsync() {
entry:
  %t0 = getelementptr %glitch_async_state_HealthAsync, ptr null, i32 1
  %t1 = ptrtoint ptr %t0 to i64
  %t2 = call ptr @glitch_calloc(i64 1, i64 %t1)
  %t3 = getelementptr inbounds %glitch_async_state_HealthAsync, ptr %t2, i32 0, i32 0
  store i32 0, ptr %t3
  %t4 = getelementptr inbounds %glitch_async_state_HealthAsync, ptr %t2, i32 0, i32 1
  store ptr null, ptr %t4
  %t5 = getelementptr %glitch.delegate, ptr null, i32 1
  %t6 = ptrtoint ptr %t5 to i64
  %t7 = call ptr @glitch_calloc(i64 1, i64 %t6)
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t7, i32 0, i32 0
  store i64 1, ptr %t8
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t7, i32 0, i32 1
  store ptr @glitch_async_resume_HealthAsync, ptr %t9
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t7, i32 0, i32 2
  store ptr %t2, ptr %t10
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t7, i32 0, i32 3
  store ptr @glitch_async_destroy_HealthAsync, ptr %t11
  %t12 = call ptr @glitch_task_run_ptr(ptr %t7)
  ret ptr %t12
}

define i32 @main() {
entry:
  %t0 = alloca ptr
  store ptr null, ptr %t0
  %t1 = getelementptr %glitch.WebApplication__g0__t141, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.WebApplication__g0__t141, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_WebApplication__g0__t141, ptr %t5
  call void @WebApplication__g0__t141_ctor(ptr %t3)
  %t6 = load ptr, ptr @glitch_exception_pending
  %t7 = icmp ne ptr %t6, null
  br i1 %t7, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t8 = load ptr, ptr %t0
  call void @glitch_drop_WebApplication__g0__t141(ptr %t8)
  store ptr %t3, ptr %t0
  %t9 = load ptr, ptr %t0
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.68, i32 0, i32 2, i64 0))
  %t12 = getelementptr %glitch.delegate, ptr null, i32 1
  %t13 = ptrtoint ptr %t12 to i64
  %t11 = call ptr @glitch_calloc(i64 1, i64 %t13)
  %t14 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 0
  store i64 1, ptr %t14
  %t15 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 1
  store ptr @glitch_delegate_wrapper_HealthAsync_1, ptr %t15
  %t16 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 2
  store ptr null, ptr %t16
  %t17 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 3
  store ptr null, ptr %t17
  call void @glitch_delegate_release(ptr %t11)
  %t18 = load ptr, ptr %t0
  %t19 = trunc i64 5111 to i32
  call void @WebApplication__g0__t141_RunOnce__g0(ptr %t18, i32 %t19)
  %t20 = load ptr, ptr @glitch_exception_pending
  %t21 = icmp ne ptr %t20, null
  br i1 %t21, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t22 = load ptr, ptr %t0
  call void @glitch_drop_WebApplication__g0__t141(ptr %t22)
  %t23 = call i64 @GlitchLiveAllocations_Load()
  %t24 = icmp ne i64 %t23, 0
  %t25 = load ptr, ptr @glitch_exception_pending
  %t26 = icmp ne ptr %t25, null
  %t27 = or i1 %t24, %t26
  %t28 = zext i1 %t27 to i32
  %t29 = call ptr @getenv(ptr @.env_report_leaks)
  %t30 = icmp ne ptr %t29, null
  br i1 %t30, label %report_leaks_2, label %main_return_3
report_leaks_2:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t23)
  br label %main_return_3
main_return_3:
  ret i32 %t28
exception_unwind:
  %t31 = load ptr, ptr %t0
  call void @glitch_drop_WebApplication__g0__t141(ptr %t31)
  %t32 = call i64 @GlitchLiveAllocations_Load()
  %t33 = icmp ne i64 %t32, 0
  %t34 = load ptr, ptr @glitch_exception_pending
  %t35 = icmp ne ptr %t34, null
  %t36 = or i1 %t33, %t35
  %t37 = zext i1 %t36 to i32
  %t38 = call ptr @getenv(ptr @.env_report_leaks)
  %t39 = icmp ne ptr %t38, null
  br i1 %t39, label %report_leaks_4, label %main_return_5
report_leaks_4:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t32)
  br label %main_return_5
main_return_5:
  ret i32 %t37
}

define ptr @MappingExpression__g2__t91_ReverseMap__g0__owner_TSource_TDestination(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_MappingExpression_TSource_TDestination___g0__t91, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @MappingExpression__g2__t91_ForMember__g0__owner_TSource_TDestination(ptr %this, ptr %dest, ptr %opt) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %dest, ptr %t1
  %t2 = alloca ptr
  store ptr %opt, ptr %t2
  %t3 = getelementptr %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_MappingExpression_TSource_TDestination___g0__t91, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @MappingExpression__g2__t91_IgnoreAllMembers__g0__owner_TSource_TDestination(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.MappingExpression_TSource_TDestination___g0__t91, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_MappingExpression_TSource_TDestination___g0__t91, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
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

define ptr @IMappingExpression__g2__t90_ReverseMap__g0__owner_TSource_TDestination(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @IMappingExpression__g2__t90_ForMember__g0__owner_TSource_TDestination(ptr %this, ptr %dest, ptr %opt) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %dest, ptr %t1
  %t2 = alloca ptr
  store ptr %opt, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @IMappingExpression__g2__t90_IgnoreAllMembers__g0__owner_TSource_TDestination(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
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

define ptr @DbSet__g1__t51_AsQueryable__g0__owner_T(ptr %this) {
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
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t13, i32 0, i32 0
  %t15 = load ptr, ptr %t14
  %t16 = icmp ne ptr %t15, null
  br i1 %t16, label %if_then_2, label %if_else_3
if_then_2:
  %t17 = trunc i64 0 to i32
  store i32 %t17, ptr %t2
  br label %while_cond_5
while_cond_5:
  %t18 = load i32, ptr %t2
  %t19 = icmp slt i32 %t18, 0
  br i1 %t19, label %while_body_6, label %while_end_7
while_body_6:
  %t20 = load ptr, ptr %t1
  %t21 = load ptr, ptr %t0
  %t22 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t21, i32 0, i32 0
  %t23 = load ptr, ptr %t22
  %t24 = load i32, ptr %t2
  %t25 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 0
  %t26 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 1
  %t27 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 2
  %t28 = load i64, ptr %t25
  %t29 = load i64, ptr %t26
  %t30 = load ptr, ptr %t27
  %t31 = icmp eq i64 %t28, %t29
  br i1 %t31, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t32 = mul i64 %t29, 2
  %t33 = mul i64 %t32, 8
  %t34 = call ptr @glitch_realloc(ptr %t30, i64 %t33)
  store i64 %t32, ptr %t26
  store ptr %t34, ptr %t27
  br label %list_ready_9
list_ready_9:
  %t35 = load ptr, ptr %t27
  %t36 = getelementptr inbounds ptr, ptr %t35, i64 %t28
  store ptr null, ptr %t36
  %t37 = add i64 %t28, 1
  store i64 %t37, ptr %t25
  %t38 = load i32, ptr %t2
  %t39 = trunc i64 1 to i32
  %t40 = add i32 %t38, %t39
  store i32 %t40, ptr %t2
  br label %while_cond_5
while_end_7:
  br label %if_end_4
if_else_3:
  br label %if_end_4
if_end_4:
  %t41 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t42 = ptrtoint ptr %t41 to i64
  %t43 = call ptr @glitch_calloc(i64 1, i64 %t42)
  %t44 = load ptr, ptr %t1
  %t45 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t43, i32 0, i32 0
  store ptr %t44, ptr %t45
  %t46 = load ptr, ptr %t1
  %t47 = icmp eq ptr %t46, null
  br i1 %t47, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t48 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 0
  %t49 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 2
  %t50 = load i64, ptr %t48
  %t51 = load ptr, ptr %t49
  call void @glitch_free(ptr %t51)
  call void @glitch_free(ptr %t46)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr %t43
exception_unwind:
  %t52 = load ptr, ptr %t1
  %t53 = icmp eq ptr %t52, null
  br i1 %t53, label %collection_release_done_13, label %collection_release_12
collection_release_12:
  %t54 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 0
  %t55 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 2
  %t56 = load i64, ptr %t54
  %t57 = load ptr, ptr %t55
  call void @glitch_free(ptr %t57)
  call void @glitch_free(ptr %t52)
  br label %collection_release_done_13
collection_release_done_13:
  ret ptr null
}

define ptr @DbSet__g1__t51_AsNoTracking__g0__owner_T(ptr %this) {
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
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t13, i32 0, i32 0
  %t15 = load ptr, ptr %t14
  %t16 = icmp ne ptr %t15, null
  br i1 %t16, label %if_then_2, label %if_else_3
if_then_2:
  %t17 = trunc i64 0 to i32
  store i32 %t17, ptr %t2
  br label %while_cond_5
while_cond_5:
  %t18 = load i32, ptr %t2
  %t19 = icmp slt i32 %t18, 0
  br i1 %t19, label %while_body_6, label %while_end_7
while_body_6:
  %t20 = load ptr, ptr %t1
  %t21 = load ptr, ptr %t0
  %t22 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t21, i32 0, i32 0
  %t23 = load ptr, ptr %t22
  %t24 = load i32, ptr %t2
  %t25 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 0
  %t26 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 1
  %t27 = getelementptr inbounds %glitch.list, ptr %t20, i32 0, i32 2
  %t28 = load i64, ptr %t25
  %t29 = load i64, ptr %t26
  %t30 = load ptr, ptr %t27
  %t31 = icmp eq i64 %t28, %t29
  br i1 %t31, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t32 = mul i64 %t29, 2
  %t33 = mul i64 %t32, 8
  %t34 = call ptr @glitch_realloc(ptr %t30, i64 %t33)
  store i64 %t32, ptr %t26
  store ptr %t34, ptr %t27
  br label %list_ready_9
list_ready_9:
  %t35 = load ptr, ptr %t27
  %t36 = getelementptr inbounds ptr, ptr %t35, i64 %t28
  store ptr null, ptr %t36
  %t37 = add i64 %t28, 1
  store i64 %t37, ptr %t25
  %t38 = load i32, ptr %t2
  %t39 = trunc i64 1 to i32
  %t40 = add i32 %t38, %t39
  store i32 %t40, ptr %t2
  br label %while_cond_5
while_end_7:
  br label %if_end_4
if_else_3:
  br label %if_end_4
if_end_4:
  %t41 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t42 = ptrtoint ptr %t41 to i64
  %t43 = call ptr @glitch_calloc(i64 1, i64 %t42)
  %t44 = load ptr, ptr %t1
  %t45 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t43, i32 0, i32 0
  store ptr %t44, ptr %t45
  %t46 = load ptr, ptr %t1
  %t47 = icmp eq ptr %t46, null
  br i1 %t47, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t48 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 0
  %t49 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 2
  %t50 = load i64, ptr %t48
  %t51 = load ptr, ptr %t49
  call void @glitch_free(ptr %t51)
  call void @glitch_free(ptr %t46)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr %t43
exception_unwind:
  %t52 = load ptr, ptr %t1
  %t53 = icmp eq ptr %t52, null
  br i1 %t53, label %collection_release_done_13, label %collection_release_12
collection_release_12:
  %t54 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 0
  %t55 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 2
  %t56 = load i64, ptr %t54
  %t57 = load ptr, ptr %t55
  call void @glitch_free(ptr %t57)
  call void @glitch_free(ptr %t52)
  br label %collection_release_done_13
collection_release_done_13:
  ret ptr null
}

define ptr @DbSet__g1__t51_Where__g0__owner_T(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
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
  %t14 = load ptr, ptr %t0
  %t15 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t14, i32 0, i32 0
  %t16 = load ptr, ptr %t15
  %t17 = icmp ne ptr %t16, null
  br i1 %t17, label %if_then_2, label %if_else_3
if_then_2:
  %t18 = trunc i64 0 to i32
  store i32 %t18, ptr %t3
  br label %while_cond_5
while_cond_5:
  %t19 = load i32, ptr %t3
  %t20 = icmp slt i32 %t19, 0
  br i1 %t20, label %while_body_6, label %while_end_7
while_body_6:
  %t21 = load ptr, ptr %t2
  %t22 = load ptr, ptr %t0
  %t23 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t22, i32 0, i32 0
  %t24 = load ptr, ptr %t23
  %t25 = load i32, ptr %t3
  %t26 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 0
  %t27 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 1
  %t28 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 2
  %t29 = load i64, ptr %t26
  %t30 = load i64, ptr %t27
  %t31 = load ptr, ptr %t28
  %t32 = icmp eq i64 %t29, %t30
  br i1 %t32, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t33 = mul i64 %t30, 2
  %t34 = mul i64 %t33, 8
  %t35 = call ptr @glitch_realloc(ptr %t31, i64 %t34)
  store i64 %t33, ptr %t27
  store ptr %t35, ptr %t28
  br label %list_ready_9
list_ready_9:
  %t36 = load ptr, ptr %t28
  %t37 = getelementptr inbounds ptr, ptr %t36, i64 %t29
  store ptr null, ptr %t37
  %t38 = add i64 %t29, 1
  store i64 %t38, ptr %t26
  %t39 = load i32, ptr %t3
  %t40 = trunc i64 1 to i32
  %t41 = add i32 %t39, %t40
  store i32 %t41, ptr %t3
  br label %while_cond_5
while_end_7:
  br label %if_end_4
if_else_3:
  br label %if_end_4
if_end_4:
  %t42 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t43 = ptrtoint ptr %t42 to i64
  %t44 = call ptr @glitch_calloc(i64 1, i64 %t43)
  %t45 = load ptr, ptr %t2
  %t46 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t44, i32 0, i32 0
  store ptr %t45, ptr %t46
  %t47 = load ptr, ptr %t1
  call void @glitch_delegate_retain(ptr %t47)
  %t48 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t44, i32 0, i32 1
  store ptr %t47, ptr %t48
  %t49 = load ptr, ptr %t2
  %t50 = icmp eq ptr %t49, null
  br i1 %t50, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t51 = getelementptr inbounds %glitch.list, ptr %t49, i32 0, i32 0
  %t52 = getelementptr inbounds %glitch.list, ptr %t49, i32 0, i32 2
  %t53 = load i64, ptr %t51
  %t54 = load ptr, ptr %t52
  call void @glitch_free(ptr %t54)
  call void @glitch_free(ptr %t49)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr %t44
exception_unwind:
  %t55 = load ptr, ptr %t2
  %t56 = icmp eq ptr %t55, null
  br i1 %t56, label %collection_release_done_13, label %collection_release_12
collection_release_12:
  %t57 = getelementptr inbounds %glitch.list, ptr %t55, i32 0, i32 0
  %t58 = getelementptr inbounds %glitch.list, ptr %t55, i32 0, i32 2
  %t59 = load i64, ptr %t57
  %t60 = load ptr, ptr %t58
  call void @glitch_free(ptr %t60)
  call void @glitch_free(ptr %t55)
  br label %collection_release_done_13
collection_release_done_13:
  ret ptr null
}

define void @DbSet__g1__t51_Add__g0__owner_T(ptr %this, ptr %entity) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t3, i32 0, i32 0
  %t5 = load ptr, ptr %t4
  %t6 = icmp eq ptr %t5, null
  br i1 %t6, label %if_then_0, label %if_else_1
if_then_0:
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t7, i32 0, i32 0
  %t9 = call ptr @glitch_calloc(i64 1, i64 24)
  %t10 = call ptr @glitch_calloc(i64 4, i64 8)
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 1
  store i64 4, ptr %t11
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  store ptr %t10, ptr %t12
  store ptr %t9, ptr %t8
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t13, i32 0, i32 0
  %t15 = load ptr, ptr %t14
  store ptr %t15, ptr %t2
  %t16 = load ptr, ptr %t2
  %t17 = load ptr, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @DbSet__g1__t51_Clear__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t1, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  %t4 = icmp eq ptr %t3, null
  br i1 %t4, label %if_then_0, label %if_else_1
if_then_0:
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t5, i32 0, i32 0
  %t7 = call ptr @glitch_calloc(i64 1, i64 24)
  %t8 = call ptr @glitch_calloc(i64 4, i64 8)
  %t9 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 1
  store i64 4, ptr %t9
  %t10 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 2
  store ptr %t8, ptr %t10
  store ptr %t7, ptr %t6
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t11, i32 0, i32 0
  %t13 = load ptr, ptr %t12
  ret void
exception_unwind:
  ret void
}

define ptr @DbSet__g1__t51_AddAsync__g0__T_CancellationToken__owner_T(ptr %this, ptr %entity, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AddAsync__g0__T__owner_T(ptr %this, ptr %entity) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AddRangeAsync__g0__ienumerable_T_CancellationToken__owner_T(ptr %this, ptr %entities, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entities, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 0
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  %t7 = load i64, ptr %t5
  %t8 = load ptr, ptr %t6
  %t9 = alloca i64
  %t10 = alloca ptr
  store i64 0, ptr %t9
  br label %foreach_condition_0
foreach_condition_0:
  %t11 = load i64, ptr %t9
  %t12 = icmp ult i64 %t11, %t7
  br i1 %t12, label %foreach_body_1, label %foreach_end_3
foreach_body_1:
  %t13 = getelementptr inbounds ptr, ptr %t8, i64 %t11
  %t14 = load ptr, ptr %t13
  store ptr %t14, ptr %t10
  %t15 = load ptr, ptr %t0
  %t16 = load ptr, ptr %t10
  br label %foreach_advance_2
foreach_advance_2:
  %t17 = load i64, ptr %t9
  %t18 = add i64 %t17, 1
  store i64 %t18, ptr %t9
  br label %foreach_condition_0
foreach_end_3:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AddRangeAsync__g0__list_T_CancellationToken__owner_T(ptr %this, ptr %entities, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entities, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = trunc i64 0 to i32
  store i32 %t4, ptr %t3
  br label %while_cond_0
while_cond_0:
  %t5 = load i32, ptr %t3
  %t6 = load ptr, ptr %t1
  %t7 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 0
  %t8 = load i64, ptr %t7
  %t9 = trunc i64 %t8 to i32
  %t10 = icmp slt i32 %t5, %t9
  br i1 %t10, label %while_body_1, label %while_end_2
while_body_1:
  %t11 = load ptr, ptr %t0
  %t12 = load ptr, ptr %t1
  %t13 = load i32, ptr %t3
  %t14 = sext i32 %t13 to i64
  %t15 = getelementptr inbounds %glitch.list, ptr %t12, i32 0, i32 2
  %t16 = load ptr, ptr %t15
  %t17 = getelementptr inbounds ptr, ptr %t16, i64 %t14
  %t18 = load ptr, ptr %t17
  %t19 = load i32, ptr %t3
  %t20 = trunc i64 1 to i32
  %t21 = add i32 %t19, %t20
  store i32 %t21, ptr %t3
  br label %while_cond_0
while_end_2:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AnyAsync__g0__CancellationToken__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_drop_object__g0__t14(ptr null)
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_AnyAsync__g0__fn_T_ret_bool_CancellationToken__owner_T(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  call void @glitch_drop_object__g0__t14(ptr null)
  %t5 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FirstAsync__g0__CancellationToken__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_drop_object__g0__t14(ptr null)
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FirstAsync__g0__fn_T_ret_bool_CancellationToken__owner_T(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  call void @glitch_drop_object__g0__t14(ptr null)
  %t5 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FirstOrDefaultAsync__g0__CancellationToken__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @FirstOrDefaultAsync__object(ptr %t2)
  %t4 = load ptr, ptr @glitch_exception_pending
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FirstOrDefaultAsync__g0__fn_T_ret_bool_CancellationToken__owner_T(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @FirstOrDefaultAsync__object(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_SingleOrDefaultAsync__g0__CancellationToken__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @SingleOrDefaultAsync__object(ptr %t2)
  %t4 = load ptr, ptr @glitch_exception_pending
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_SingleOrDefaultAsync__g0__fn_T_ret_bool_CancellationToken__owner_T(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @SingleOrDefaultAsync__object(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_ToListAsync__g0__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_drop_object__g0__t14(ptr null)
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_FindAsync__g0__owner_T(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbSet__g1__t51_Snapshot__g0__owner_T(ptr %this) {
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
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t13, i32 0, i32 0
  %t15 = load ptr, ptr %t14
  %t16 = icmp eq ptr %t15, null
  br i1 %t16, label %if_then_2, label %if_else_3
if_then_2:
  %t17 = load ptr, ptr %t1
  ret ptr %t17
if_else_3:
  br label %if_end_4
if_end_4:
  %t18 = trunc i64 0 to i32
  store i32 %t18, ptr %t2
  br label %while_cond_5
while_cond_5:
  %t19 = load i32, ptr %t2
  %t20 = icmp slt i32 %t19, 0
  br i1 %t20, label %while_body_6, label %while_end_7
while_body_6:
  %t21 = load ptr, ptr %t1
  %t22 = load ptr, ptr %t0
  %t23 = getelementptr inbounds %glitch.DbSet_T___g0__t51, ptr %t22, i32 0, i32 0
  %t24 = load ptr, ptr %t23
  %t25 = load i32, ptr %t2
  %t26 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 0
  %t27 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 1
  %t28 = getelementptr inbounds %glitch.list, ptr %t21, i32 0, i32 2
  %t29 = load i64, ptr %t26
  %t30 = load i64, ptr %t27
  %t31 = load ptr, ptr %t28
  %t32 = icmp eq i64 %t29, %t30
  br i1 %t32, label %list_grow_8, label %list_ready_9
list_grow_8:
  %t33 = mul i64 %t30, 2
  %t34 = mul i64 %t33, 8
  %t35 = call ptr @glitch_realloc(ptr %t31, i64 %t34)
  store i64 %t33, ptr %t27
  store ptr %t35, ptr %t28
  br label %list_ready_9
list_ready_9:
  %t36 = load ptr, ptr %t28
  %t37 = getelementptr inbounds ptr, ptr %t36, i64 %t29
  store ptr null, ptr %t37
  %t38 = add i64 %t29, 1
  store i64 %t38, ptr %t26
  %t39 = load i32, ptr %t2
  %t40 = trunc i64 1 to i32
  %t41 = add i32 %t39, %t40
  store i32 %t41, ptr %t2
  br label %while_cond_5
while_end_7:
  %t42 = load ptr, ptr %t1
  ret ptr %t42
exception_unwind:
  %t43 = load ptr, ptr %t1
  %t44 = icmp eq ptr %t43, null
  br i1 %t44, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t45 = getelementptr inbounds %glitch.list, ptr %t43, i32 0, i32 0
  %t46 = getelementptr inbounds %glitch.list, ptr %t43, i32 0, i32 2
  %t47 = load i64, ptr %t45
  %t48 = load ptr, ptr %t46
  call void @glitch_free(ptr %t48)
  call void @glitch_free(ptr %t43)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr null
}

define ptr @IRequestHandler__g1__t81_Handle__g0__TRequest_CancellationToken__owner_TRequest(ptr %this, ptr %request, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %request, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @IValidator__g1__t99_Validate__g0__owner_T(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
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

define ptr @DbQuery__g1__t52_RowsSnapshot__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t1, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define i1 @DbQuery__g1__t52_Matches__g0__owner_T(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t2, i32 0, i32 1
  %t4 = load ptr, ptr %t3
  %t5 = icmp ne ptr %t4, null
  %t6 = load ptr, ptr %t0
  %t7 = load ptr, ptr %t1
  %t8 = icmp ne ptr null, null
  %t9 = xor i1 %t8, true
  %t10 = and i1 %t5, %t9
  br i1 %t10, label %if_then_0, label %if_else_1
if_then_0:
  ret i1 0
if_else_1:
  br label %if_end_2
if_end_2:
  ret i1 1
exception_unwind:
  ret i1 0
}

define ptr @DbQuery__g1__t52_AsQueryable__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = call ptr @glitch_calloc(i64 1, i64 24)
  %t5 = call ptr @glitch_calloc(i64 4, i64 8)
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 1
  store i64 4, ptr %t6
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  store ptr %t5, ptr %t7
  %t8 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t3, i32 0, i32 0
  store ptr %t4, ptr %t8
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t9, i32 0, i32 1
  %t11 = load ptr, ptr %t10
  call void @glitch_delegate_retain(ptr %t11)
  %t12 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t3, i32 0, i32 1
  store ptr %t11, ptr %t12
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_AsNoTracking__g0__owner_T(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = call ptr @glitch_calloc(i64 1, i64 24)
  %t5 = call ptr @glitch_calloc(i64 4, i64 8)
  %t6 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 1
  store i64 4, ptr %t6
  %t7 = getelementptr inbounds %glitch.list, ptr %t4, i32 0, i32 2
  store ptr %t5, ptr %t7
  %t8 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t3, i32 0, i32 0
  store ptr %t4, ptr %t8
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t9, i32 0, i32 1
  %t11 = load ptr, ptr %t10
  call void @glitch_delegate_retain(ptr %t11)
  %t12 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t3, i32 0, i32 1
  store ptr %t11, ptr %t12
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_Include__g0__owner_T(ptr %this, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  %t9 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 0
  store ptr %t5, ptr %t9
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t10, i32 0, i32 1
  %t12 = load ptr, ptr %t11
  call void @glitch_delegate_retain(ptr %t12)
  %t13 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 1
  store ptr %t12, ptr %t13
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_ThenInclude__g0__owner_T(ptr %this, ptr %navigation) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %navigation, ptr %t1
  %t2 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  %t9 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 0
  store ptr %t5, ptr %t9
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t10, i32 0, i32 1
  %t12 = load ptr, ptr %t11
  call void @glitch_delegate_retain(ptr %t12)
  %t13 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 1
  store ptr %t12, ptr %t13
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_Where__g0__owner_T(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = getelementptr %glitch.DbQuery_T___g0__t52, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = call ptr @glitch_calloc(i64 1, i64 24)
  %t6 = call ptr @glitch_calloc(i64 4, i64 8)
  %t7 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 1
  store i64 4, ptr %t7
  %t8 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 2
  store ptr %t6, ptr %t8
  %t9 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 0
  store ptr %t5, ptr %t9
  %t10 = load ptr, ptr %t1
  call void @glitch_delegate_retain(ptr %t10)
  %t11 = getelementptr inbounds %glitch.DbQuery_T___g0__t52, ptr %t4, i32 0, i32 1
  store ptr %t10, ptr %t11
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_AnyAsync__g0__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = trunc i64 0 to i32
  store i32 %t4, ptr %t2
  %t5 = load ptr, ptr %t0
  %t6 = call ptr @DbQuery__g1__t52_RowsSnapshot__g0(ptr %t5)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t6, ptr %t3
  br label %while_cond_1
while_cond_1:
  %t9 = load i32, ptr %t2
  %t10 = load ptr, ptr %t3
  %t11 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 0
  %t12 = load i64, ptr %t11
  %t13 = trunc i64 %t12 to i32
  %t14 = icmp slt i32 %t9, %t13
  br i1 %t14, label %while_body_2, label %while_end_3
while_body_2:
  %t15 = load ptr, ptr %t0
  %t16 = load ptr, ptr %t3
  %t17 = load i32, ptr %t2
  %t18 = sext i32 %t17 to i64
  %t19 = getelementptr inbounds %glitch.list, ptr %t16, i32 0, i32 2
  %t20 = load ptr, ptr %t19
  %t21 = getelementptr inbounds ptr, ptr %t20, i64 %t18
  %t22 = load ptr, ptr %t21
  %t23 = icmp ne ptr null, null
  br i1 %t23, label %if_then_4, label %if_else_5
if_then_4:
  %t24 = load ptr, ptr %t3
  %t25 = icmp eq ptr %t24, null
  br i1 %t25, label %collection_release_done_8, label %collection_release_7
collection_release_7:
  %t26 = getelementptr inbounds %glitch.list, ptr %t24, i32 0, i32 0
  %t27 = getelementptr inbounds %glitch.list, ptr %t24, i32 0, i32 2
  %t28 = load i64, ptr %t26
  %t29 = load ptr, ptr %t27
  call void @glitch_free(ptr %t29)
  call void @glitch_free(ptr %t24)
  br label %collection_release_done_8
collection_release_done_8:
  ret ptr null
if_else_5:
  br label %if_end_6
if_end_6:
  %t30 = load i32, ptr %t2
  %t31 = trunc i64 1 to i32
  %t32 = add i32 %t30, %t31
  store i32 %t32, ptr %t2
  br label %while_cond_1
while_end_3:
  %t33 = load ptr, ptr %t3
  %t34 = icmp eq ptr %t33, null
  br i1 %t34, label %collection_release_done_10, label %collection_release_9
collection_release_9:
  %t35 = getelementptr inbounds %glitch.list, ptr %t33, i32 0, i32 0
  %t36 = getelementptr inbounds %glitch.list, ptr %t33, i32 0, i32 2
  %t37 = load i64, ptr %t35
  %t38 = load ptr, ptr %t36
  call void @glitch_free(ptr %t38)
  call void @glitch_free(ptr %t33)
  br label %collection_release_done_10
collection_release_done_10:
  ret ptr null
exception_unwind:
  %t39 = load ptr, ptr %t3
  %t40 = icmp eq ptr %t39, null
  br i1 %t40, label %collection_release_done_12, label %collection_release_11
collection_release_11:
  %t41 = getelementptr inbounds %glitch.list, ptr %t39, i32 0, i32 0
  %t42 = getelementptr inbounds %glitch.list, ptr %t39, i32 0, i32 2
  %t43 = load i64, ptr %t41
  %t44 = load ptr, ptr %t42
  call void @glitch_free(ptr %t44)
  call void @glitch_free(ptr %t39)
  br label %collection_release_done_12
collection_release_done_12:
  ret ptr null
}

define ptr @DbQuery__g1__t52_FirstAsync__g0__CancellationToken__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = trunc i64 0 to i32
  store i32 %t5, ptr %t2
  %t6 = load ptr, ptr %t0
  %t7 = call ptr @DbQuery__g1__t52_RowsSnapshot__g0(ptr %t6)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t7, ptr %t3
  br label %while_cond_1
while_cond_1:
  %t10 = load i32, ptr %t2
  %t11 = load ptr, ptr %t3
  %t12 = getelementptr inbounds %glitch.list, ptr %t11, i32 0, i32 0
  %t13 = load i64, ptr %t12
  %t14 = trunc i64 %t13 to i32
  %t15 = icmp slt i32 %t10, %t14
  br i1 %t15, label %while_body_2, label %while_end_3
while_body_2:
  %t16 = load ptr, ptr %t3
  %t17 = load i32, ptr %t2
  %t18 = sext i32 %t17 to i64
  %t19 = getelementptr inbounds %glitch.list, ptr %t16, i32 0, i32 2
  %t20 = load ptr, ptr %t19
  %t21 = getelementptr inbounds ptr, ptr %t20, i64 %t18
  %t22 = load ptr, ptr %t21
  store ptr %t22, ptr %t4
  %t23 = load ptr, ptr %t0
  %t24 = load ptr, ptr %t4
  %t25 = icmp ne ptr null, null
  br i1 %t25, label %if_then_4, label %if_else_5
if_then_4:
  %t26 = load ptr, ptr %t4
  %t27 = load ptr, ptr %t3
  %t28 = icmp eq ptr %t27, null
  br i1 %t28, label %collection_release_done_8, label %collection_release_7
collection_release_7:
  %t29 = getelementptr inbounds %glitch.list, ptr %t27, i32 0, i32 0
  %t30 = getelementptr inbounds %glitch.list, ptr %t27, i32 0, i32 2
  %t31 = load i64, ptr %t29
  %t32 = load ptr, ptr %t30
  call void @glitch_free(ptr %t32)
  call void @glitch_free(ptr %t27)
  br label %collection_release_done_8
collection_release_done_8:
  ret ptr null
if_else_5:
  br label %if_end_6
if_end_6:
  %t33 = load i32, ptr %t2
  %t34 = trunc i64 1 to i32
  %t35 = add i32 %t33, %t34
  store i32 %t35, ptr %t2
  br label %while_cond_1
while_end_3:
  %t36 = load ptr, ptr %t3
  %t37 = icmp eq ptr %t36, null
  br i1 %t37, label %collection_release_done_10, label %collection_release_9
collection_release_9:
  %t38 = getelementptr inbounds %glitch.list, ptr %t36, i32 0, i32 0
  %t39 = getelementptr inbounds %glitch.list, ptr %t36, i32 0, i32 2
  %t40 = load i64, ptr %t38
  %t41 = load ptr, ptr %t39
  call void @glitch_free(ptr %t41)
  call void @glitch_free(ptr %t36)
  br label %collection_release_done_10
collection_release_done_10:
  ret ptr null
exception_unwind:
  %t42 = load ptr, ptr %t3
  %t43 = icmp eq ptr %t42, null
  br i1 %t43, label %collection_release_done_12, label %collection_release_11
collection_release_11:
  %t44 = getelementptr inbounds %glitch.list, ptr %t42, i32 0, i32 0
  %t45 = getelementptr inbounds %glitch.list, ptr %t42, i32 0, i32 2
  %t46 = load i64, ptr %t44
  %t47 = load ptr, ptr %t45
  call void @glitch_free(ptr %t47)
  call void @glitch_free(ptr %t42)
  br label %collection_release_done_12
collection_release_done_12:
  ret ptr null
}

define ptr @DbQuery__g1__t52_FirstAsync__g0__fn_T_ret_bool_CancellationToken__owner_T(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = load ptr, ptr %t1
  call void @glitch_drop_object__g0__t14(ptr null)
  %t5 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_FirstOrDefaultAsync__g0__CancellationToken__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_FirstOrDefaultAsync__g0__fn_T_ret_bool_CancellationToken__owner_T(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @FirstOrDefaultAsync__object(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_SingleOrDefaultAsync__g0__CancellationToken__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca i1
  store i1 0, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = alloca ptr
  store ptr null, ptr %t6
  %t7 = trunc i64 0 to i32
  store i32 %t7, ptr %t2
  store ptr null, ptr %t3
  store i1 0, ptr %t4
  %t8 = load ptr, ptr %t0
  %t9 = call ptr @DbQuery__g1__t52_RowsSnapshot__g0(ptr %t8)
  %t10 = load ptr, ptr @glitch_exception_pending
  %t11 = icmp ne ptr %t10, null
  br i1 %t11, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t9, ptr %t5
  br label %while_cond_1
while_cond_1:
  %t12 = load i32, ptr %t2
  %t13 = load ptr, ptr %t5
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  %t15 = load i64, ptr %t14
  %t16 = trunc i64 %t15 to i32
  %t17 = icmp slt i32 %t12, %t16
  br i1 %t17, label %while_body_2, label %while_end_3
while_body_2:
  %t18 = load ptr, ptr %t5
  %t19 = load i32, ptr %t2
  %t20 = sext i32 %t19 to i64
  %t21 = getelementptr inbounds %glitch.list, ptr %t18, i32 0, i32 2
  %t22 = load ptr, ptr %t21
  %t23 = getelementptr inbounds ptr, ptr %t22, i64 %t20
  %t24 = load ptr, ptr %t23
  store ptr %t24, ptr %t6
  %t25 = load ptr, ptr %t0
  %t26 = load ptr, ptr %t6
  %t27 = icmp ne ptr null, null
  br i1 %t27, label %if_then_4, label %if_else_5
if_then_4:
  %t28 = load i1, ptr %t4
  br i1 %t28, label %if_then_7, label %if_else_8
if_then_7:
  %t29 = load ptr, ptr %t5
  %t30 = icmp eq ptr %t29, null
  br i1 %t30, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t31 = getelementptr inbounds %glitch.list, ptr %t29, i32 0, i32 0
  %t32 = getelementptr inbounds %glitch.list, ptr %t29, i32 0, i32 2
  %t33 = load i64, ptr %t31
  %t34 = load ptr, ptr %t32
  call void @glitch_free(ptr %t34)
  call void @glitch_free(ptr %t29)
  br label %collection_release_done_11
collection_release_done_11:
  ret ptr null
if_else_8:
  br label %if_end_9
if_end_9:
  %t35 = load ptr, ptr %t6
  store ptr %t35, ptr %t3
  store i1 1, ptr %t4
  br label %if_end_6
if_else_5:
  br label %if_end_6
if_end_6:
  %t36 = load i32, ptr %t2
  %t37 = trunc i64 1 to i32
  %t38 = add i32 %t36, %t37
  store i32 %t38, ptr %t2
  br label %while_cond_1
while_end_3:
  %t39 = load ptr, ptr %t3
  %t40 = load ptr, ptr %t5
  %t41 = icmp eq ptr %t40, null
  br i1 %t41, label %collection_release_done_13, label %collection_release_12
collection_release_12:
  %t42 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 0
  %t43 = getelementptr inbounds %glitch.list, ptr %t40, i32 0, i32 2
  %t44 = load i64, ptr %t42
  %t45 = load ptr, ptr %t43
  call void @glitch_free(ptr %t45)
  call void @glitch_free(ptr %t40)
  br label %collection_release_done_13
collection_release_done_13:
  ret ptr null
exception_unwind:
  %t46 = load ptr, ptr %t5
  %t47 = icmp eq ptr %t46, null
  br i1 %t47, label %collection_release_done_15, label %collection_release_14
collection_release_14:
  %t48 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 0
  %t49 = getelementptr inbounds %glitch.list, ptr %t46, i32 0, i32 2
  %t50 = load i64, ptr %t48
  %t51 = load ptr, ptr %t49
  call void @glitch_free(ptr %t51)
  call void @glitch_free(ptr %t46)
  br label %collection_release_done_15
collection_release_done_15:
  ret ptr null
}

define ptr @DbQuery__g1__t52_SingleOrDefaultAsync__g0__fn_T_ret_bool_CancellationToken__owner_T(ptr %this, ptr %predicate, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @SingleOrDefaultAsync__object(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_retain_object__g0__t14(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @DbQuery__g1__t52_ToListAsync__g0__owner_T(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca i32
  store i32 0, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = call ptr @glitch_calloc(i64 1, i64 24)
  %t7 = call ptr @glitch_calloc(i64 4, i64 8)
  %t8 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 1
  store i64 4, ptr %t8
  %t9 = getelementptr inbounds %glitch.list, ptr %t6, i32 0, i32 2
  store ptr %t7, ptr %t9
  %t10 = load ptr, ptr %t2
  %t11 = icmp eq ptr %t10, null
  br i1 %t11, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t12 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 0
  %t13 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 2
  %t14 = load i64, ptr %t12
  %t15 = load ptr, ptr %t13
  call void @glitch_free(ptr %t15)
  call void @glitch_free(ptr %t10)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t6, ptr %t2
  %t16 = trunc i64 0 to i32
  store i32 %t16, ptr %t3
  %t17 = load ptr, ptr %t0
  %t18 = call ptr @DbQuery__g1__t52_RowsSnapshot__g0(ptr %t17)
  %t19 = load ptr, ptr @glitch_exception_pending
  %t20 = icmp ne ptr %t19, null
  br i1 %t20, label %exception_unwind, label %call_continue_2
call_continue_2:
  store ptr %t18, ptr %t4
  br label %while_cond_3
while_cond_3:
  %t21 = load i32, ptr %t3
  %t22 = load ptr, ptr %t4
  %t23 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 0
  %t24 = load i64, ptr %t23
  %t25 = trunc i64 %t24 to i32
  %t26 = icmp slt i32 %t21, %t25
  br i1 %t26, label %while_body_4, label %while_end_5
while_body_4:
  %t27 = load ptr, ptr %t4
  %t28 = load i32, ptr %t3
  %t29 = sext i32 %t28 to i64
  %t30 = getelementptr inbounds %glitch.list, ptr %t27, i32 0, i32 2
  %t31 = load ptr, ptr %t30
  %t32 = getelementptr inbounds ptr, ptr %t31, i64 %t29
  %t33 = load ptr, ptr %t32
  store ptr %t33, ptr %t5
  %t34 = load ptr, ptr %t0
  %t35 = load ptr, ptr %t5
  %t36 = icmp ne ptr null, null
  br i1 %t36, label %if_then_6, label %if_else_7
if_then_6:
  %t37 = load ptr, ptr %t2
  %t38 = load ptr, ptr %t5
  %t39 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 0
  %t40 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 1
  %t41 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 2
  %t42 = load i64, ptr %t39
  %t43 = load i64, ptr %t40
  %t44 = load ptr, ptr %t41
  %t45 = icmp eq i64 %t42, %t43
  br i1 %t45, label %list_grow_9, label %list_ready_10
list_grow_9:
  %t46 = mul i64 %t43, 2
  %t47 = mul i64 %t46, 8
  %t48 = call ptr @glitch_realloc(ptr %t44, i64 %t47)
  store i64 %t46, ptr %t40
  store ptr %t48, ptr %t41
  br label %list_ready_10
list_ready_10:
  %t49 = load ptr, ptr %t41
  %t50 = getelementptr inbounds ptr, ptr %t49, i64 %t42
  store ptr %t38, ptr %t50
  %t51 = add i64 %t42, 1
  store i64 %t51, ptr %t39
  br label %if_end_8
if_else_7:
  br label %if_end_8
if_end_8:
  %t52 = load i32, ptr %t3
  %t53 = trunc i64 1 to i32
  %t54 = add i32 %t52, %t53
  store i32 %t54, ptr %t3
  br label %while_cond_3
while_end_5:
  %t55 = load ptr, ptr %t2
  %t56 = load ptr, ptr %t4
  %t57 = icmp eq ptr %t56, null
  br i1 %t57, label %collection_release_done_12, label %collection_release_11
collection_release_11:
  %t58 = getelementptr inbounds %glitch.list, ptr %t56, i32 0, i32 0
  %t59 = getelementptr inbounds %glitch.list, ptr %t56, i32 0, i32 2
  %t60 = load i64, ptr %t58
  %t61 = load ptr, ptr %t59
  call void @glitch_free(ptr %t61)
  call void @glitch_free(ptr %t56)
  br label %collection_release_done_12
collection_release_done_12:
  %t62 = load ptr, ptr %t2
  %t63 = icmp eq ptr %t62, null
  br i1 %t63, label %collection_release_done_14, label %collection_release_13
collection_release_13:
  %t64 = getelementptr inbounds %glitch.list, ptr %t62, i32 0, i32 0
  %t65 = getelementptr inbounds %glitch.list, ptr %t62, i32 0, i32 2
  %t66 = load i64, ptr %t64
  %t67 = load ptr, ptr %t65
  call void @glitch_free(ptr %t67)
  call void @glitch_free(ptr %t62)
  br label %collection_release_done_14
collection_release_done_14:
  ret ptr null
exception_unwind:
  %t68 = load ptr, ptr %t4
  %t69 = icmp eq ptr %t68, null
  br i1 %t69, label %collection_release_done_16, label %collection_release_15
collection_release_15:
  %t70 = getelementptr inbounds %glitch.list, ptr %t68, i32 0, i32 0
  %t71 = getelementptr inbounds %glitch.list, ptr %t68, i32 0, i32 2
  %t72 = load i64, ptr %t70
  %t73 = load ptr, ptr %t71
  call void @glitch_free(ptr %t73)
  call void @glitch_free(ptr %t68)
  br label %collection_release_done_16
collection_release_done_16:
  %t74 = load ptr, ptr %t2
  %t75 = icmp eq ptr %t74, null
  br i1 %t75, label %collection_release_done_18, label %collection_release_17
collection_release_17:
  %t76 = getelementptr inbounds %glitch.list, ptr %t74, i32 0, i32 0
  %t77 = getelementptr inbounds %glitch.list, ptr %t74, i32 0, i32 2
  %t78 = load i64, ptr %t76
  %t79 = load ptr, ptr %t77
  call void @glitch_free(ptr %t79)
  call void @glitch_free(ptr %t74)
  br label %collection_release_done_18
collection_release_done_18:
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

define ptr @RuleBuilder__g2__t100_NotNull__g0__owner_T_TProperty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_NotEmpty__g0__owner_T_TProperty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_SetValidator__g0__owner_T_TProperty(ptr %this, ptr %validator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %validator, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_EmailAddress__g0__owner_T_TProperty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_MinimumLength__g0__owner_T_TProperty(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_MaximumLength__g0__owner_T_TProperty(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Length__g0__owner_T_TProperty(ptr %this, i32 %min, i32 %max) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %min, ptr %t1
  %t2 = alloca i32
  store i32 %max, ptr %t2
  %t3 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Matches__g0__owner_T_TProperty(ptr %this, ptr %pattern) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %pattern, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Equal__g0__owner_T_TProperty(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_NotEqual__g0__owner_T_TProperty(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Must__g0__owner_T_TProperty(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_WithMessage__g0__owner_T_TProperty(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @ValidationContext__g1__t98_ctor__owner_T(ptr %this, ptr %instanceToValidate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %instanceToValidate, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ValidationContext_T___g0__t98, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
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

define ptr @AbstractValidator__g1__t101_RuleFor__g0__owner_T(ptr %this, ptr %expression) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %expression, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_object___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_object___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t101_RuleForEach__g0__owner_T(ptr %this, ptr %expression) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %expression, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_object___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_object___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_object___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AbstractValidator__g1__t101_Validate__g0__owner_T(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = getelementptr %glitch.ValidationResult__g0__t96, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.ValidationResult__g0__t96, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_ValidationResult__g0__t96, ptr %t6
  call void @ValidationResult__g0__t96_ctor__empty(ptr %t4)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_NotNull__g0__owner_T_object__g0__t14(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_NotEmpty__g0__owner_T_object__g0__t14(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_SetValidator__g0__owner_T_object__g0__t14(ptr %this, ptr %validator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %validator, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_EmailAddress__g0__owner_T_object__g0__t14(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_MinimumLength__g0__owner_T_object__g0__t14(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_MaximumLength__g0__owner_T_object__g0__t14(ptr %this, i32 %length) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %length, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Length__g0__owner_T_object__g0__t14(ptr %this, i32 %min, i32 %max) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %min, ptr %t1
  %t2 = alloca i32
  store i32 %max, ptr %t2
  %t3 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Matches__g0__owner_T_object__g0__t14(ptr %this, ptr %pattern) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %pattern, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Equal__g0__owner_T_object__g0__t14(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_NotEqual__g0__owner_T_object__g0__t14(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_Must__g0__owner_T_object__g0__t14(ptr %this, ptr %predicate) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %predicate, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @RuleBuilder__g2__t100_WithMessage__g0__owner_T_object__g0__t14(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = getelementptr %glitch.RuleBuilder_T_TProperty___g0__t100, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.RuleBuilder_T_TProperty___g0__t100, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_RuleBuilder_T_TProperty___g0__t100, ptr %t6
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

define ptr @WebApplication_Handle(ptr %self, ptr %method, ptr %path, ptr %body) {
entry:
  %result = call ptr @WebApplication__g0__t141_Handle__g0(ptr %self, ptr %method, ptr %path, ptr %body)
  ret ptr %result
}

define ptr @glitch_endpoint_handler_0(ptr %path, ptr %body) {
entry:
  %result = call ptr @HealthAsync()
  %t2 = call ptr @glitch_task_get_result_ptr(ptr %result)
  call void @glitch_string_retain(ptr %t2)
  %t3 = icmp eq ptr %result, null
  br i1 %t3, label %task_release_done_1, label %task_release_0
task_release_0:
  call void @glitch_task_wait(ptr %result)
  %t4 = getelementptr inbounds %glitch.task, ptr %result, i32 0, i32 1
  %t5 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t5)
  call void @GlitchTask_Destroy(ptr %result)
  call void @glitch_free(ptr %result)
  br label %task_release_done_1
task_release_done_1:
  ret ptr %t2
}

define i1 @glitch_endpoint_handlers_contains(ptr %app, ptr %method, ptr %path) {
entry:
  %contains_method_cmp_0 = call i32 @strcmp(ptr %method, ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.70, i32 0, i32 2, i64 0))
  %contains_method_match_0 = icmp eq i32 %contains_method_cmp_0, 0
  br i1 %contains_method_match_0, label %contains_path_0, label %endpoint_not_found
contains_path_0:
  %contains_path_match_0 = call i1 @glitch_route_match(ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.71, i32 0, i32 2, i64 0), ptr %path)
  br i1 %contains_path_match_0, label %endpoint_found, label %endpoint_not_found
endpoint_found:
  ret i1 true
endpoint_not_found:
  ret i1 false
}

define ptr @glitch_endpoint_handlers_invoke(ptr %app, ptr %method, ptr %path, ptr %body) {
entry:
  %invoke_method_cmp_0 = call i32 @strcmp(ptr %method, ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.70, i32 0, i32 2, i64 0))
  %invoke_method_match_0 = icmp eq i32 %invoke_method_cmp_0, 0
  br i1 %invoke_method_match_0, label %invoke_path_0, label %invoke_not_found
invoke_path_0:
  %invoke_path_match_0 = call i1 @glitch_route_match(ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.71, i32 0, i32 2, i64 0), ptr %path)
  br i1 %invoke_path_match_0, label %invoke_handler_0, label %invoke_not_found
invoke_handler_0:
  %invoke_result_0 = call ptr @glitch_endpoint_handler_0(ptr %path, ptr %body)
  ret ptr %invoke_result_0
invoke_not_found:
  ret ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.72, i32 0, i32 2, i64 0)
}

