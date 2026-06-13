; ModuleID = 'glitching'
%glitch.array = type { i64, ptr }
%glitch.list = type { i64, i64, ptr }
%glitch.dict = type { i64, i64, ptr, ptr }
%glitch.string_node = type { i64, i64, [0 x i8] }
%glitch.delegate = type { i64, ptr, ptr, ptr }
%glitch.IEnumerable__g1__t0 = type { i64, ptr }
%glitch.IEnumerator__g1__t1 = type { i64, ptr, ptr }
%glitch.ICollection__g1__t2 = type { i64, ptr, i32 }
%glitch.IReadOnlyCollection__g1__t3 = type { i64, ptr, i32 }
%glitch.IList__g1__t4 = type { i64, ptr }
%glitch.IReadOnlyList__g1__t5 = type { i64, ptr }
%glitch.IDictionary__g2__t6 = type { i64, ptr, i32 }
%glitch.IReadOnlyDictionary__g2__t7 = type { i64, ptr, i32 }
%glitch.KeyValuePair__g2__t8 = type { ptr, ptr }
%glitch.List__g1__t9 = type { i64, ptr, i32 }
%glitch.Dictionary__g2__t10 = type { i64, ptr, i32 }
%glitch.AppContext__g0__t11 = type { i64, ptr, ptr }
%glitch.Uri__g0__t12 = type { i64, ptr }
%glitch.GC__g0__t13 = type { i64, ptr }
%glitch.Exception__g0__t14 = type { i64, ptr, ptr }
%glitch.ArgumentException__g0__t15 = type { i64, ptr, ptr }
%glitch.FormatException__g0__t16 = type { i64, ptr, ptr }
%glitch.DateTime__g0__t17 = type { i64, ptr, double, ptr }
%glitch.TimeSpan__g0__t18 = type { i64, ptr, double }
%glitch.Enum__g0__t19 = type { i64, ptr }
%glitch.Type__g0__t20 = type { i64, ptr, ptr, i1 }
%glitch.PropertyInfo__g0__t21 = type { i64, ptr, ptr, ptr }
%glitch.string__g0__t22 = type { i64, ptr }
%glitch.String__g0__t23 = type { i64, ptr }
%glitch.bool__g0__t24 = type { i64, ptr }
%glitch.int__g0__t25 = type { i64, ptr }
%glitch.Enumerable__g0__t26 = type { i64, ptr }
%glitch.Task__g0__t27 = type { i64, ptr, ptr, ptr }
%glitch.ValueTask__g0__t28 = type { i64, ptr, ptr }
%glitch.DbContextOptions__g1__t29 = type { i64, ptr, ptr }
%glitch.SqlProvider__g0__t30 = type { i64, ptr, ptr, ptr }
%glitch.DbSet__g1__t31 = type { i64, ptr }
%glitch.DbContextOptionsBuilder_ApplicationDbContext__g0__t32 = type { i64, ptr }
%glitch.ChangeTracker__g0__t33 = type { i64, ptr, ptr }
%glitch.DebugView__g0__t34 = type { i64, ptr, ptr }
%glitch.DatabaseFacade__g0__t35 = type { i64, ptr }
%glitch.EntityEntry__g0__t36 = type { i64, ptr, i32 }
%glitch.DbUpdateConcurrencyException__g0__t37 = type { i64, ptr, ptr }
%glitch.DbContext__g0__t38 = type { i64, ptr, ptr, i1, ptr, ptr, ptr, ptr }
%glitch.IQueryableString__g0__t39 = type { i64, ptr, ptr, ptr, i1 }
%glitch.DbSetString__g0__t40 = type { i64, ptr, ptr, ptr }
%glitch.ModelBuilder__g0__t41 = type { i64, ptr }
%glitch.MigrationBuilder__g0__t42 = type { i64, ptr }
%glitch.EntityTypeBuilder__g1__t43 = type { i64, ptr }
%glitch.PropertyBuilder__g0__t44 = type { i64, ptr }
%glitch.IndexBuilder__g0__t45 = type { i64, ptr }
%glitch.IServiceProvider__g0__t46 = type { i64, ptr }
%glitch.ServiceProvider__g0__t47 = type { i64, ptr, ptr }
%glitch.IServiceScope__g0__t48 = type { i64, ptr, ptr }
%glitch.ServiceScope__g0__t49 = type { i64, ptr, ptr }
%glitch.MvcBuilder__g0__t50 = type { i64, ptr }
%glitch.IServiceCollection__g0__t51 = type { i64, ptr }
%glitch.ServiceCollection__g0__t52 = type { i64, ptr, ptr }
%glitch.ConfigurationManager__g0__t53 = type { i64, ptr }
%glitch.HostEnvironment__g0__t54 = type { i64, ptr }
%glitch.LoggingBuilder__g0__t55 = type { i64, ptr }
%glitch.WebApplicationBuilder__g0__t56 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.HttpRequest__g0__t57 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.HttpResponse__g0__t58 = type { i64, ptr, i32, ptr, ptr }
%glitch.IPAddress__g0__t59 = type { i64, ptr, ptr }
%glitch.ConnectionInfo__g0__t60 = type { i64, ptr, ptr }
%glitch.HttpContext__g0__t61 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.IHttpContextAccessor__g0__t62 = type { i64, ptr, ptr }
%glitch.HttpContextAccessor__g0__t63 = type { i64, ptr, ptr }
%glitch.Endpoint__g0__t64 = type { i64, ptr, ptr, ptr, ptr }
%glitch.WebApplication__g0__t65 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.StaticFileOptions__g0__t66 = type { i64, ptr, ptr, ptr }
%glitch.PhysicalFileProvider__g0__t67 = type { i64, ptr }
%glitch.ApiVersion__g0__t68 = type { i64, ptr }
%glitch.ProblemDetails__g0__t69 = type { i64, ptr, ptr, i32, ptr, ptr, ptr }
%glitch.ObjectResult__g0__t70 = type { i64, ptr, i32 }
%glitch.JwtBearerDefaults__g0__t71 = type { i64, ptr, ptr }
%glitch.IFormFile__g0__t72 = type { i64, ptr, ptr, i64 }
%glitch.ActionExecutingContext__g0__t73 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.ExceptionContext__g0__t74 = type { i64, ptr, ptr, ptr, ptr, i1 }
%glitch.ModelError__g0__t75 = type { i64, ptr, ptr }
%glitch.ModelStateEntry__g0__t76 = type { i64, ptr, ptr }
%glitch.ModelStateDictionary__g0__t77 = type { i64, ptr, i1 }
%glitch.ControllerBase__g0__t78 = type { i64, ptr, ptr, ptr }
%glitch.IMapper__g0__t79 = type { i64, ptr }
%glitch.Mapper__g0__t80 = type { i64, ptr }
%glitch.IMemberConfigurationExpression__g0__t81 = type { i64, ptr }
%glitch.MemberConfigurationExpression__g0__t82 = type { i64, ptr }
%glitch.IMappingExpression__g2__t83 = type { i64, ptr }
%glitch.MappingExpression__g2__t84 = type { i64, ptr }
%glitch.Profile__g0__t85 = type { i64, ptr, ptr, ptr, ptr, i1 }
%glitch.MapperConfigurationExpression__g0__t86 = type { i64, ptr, ptr }
%glitch.MapperConfiguration__g0__t87 = type { i64, ptr, ptr }
%glitch.JsonSerializer__g0__t88 = type { i64, ptr }
%glitch.ILogger__g0__t89 = type { i64, ptr }
%glitch.ConsoleLogger__g0__t90 = type { i64, ptr }
%glitch.ILoggerFactory__g0__t91 = type { i64, ptr }
%glitch.ILoggerProvider__g0__t92 = type { i64, ptr }
%glitch.LoggerFactory__g0__t93 = type { i64, ptr }
%glitch.Path__g0__t94 = type { i64, ptr }
%glitch.Directory__g0__t95 = type { i64, ptr }
%glitch.FileStream__g0__t96 = type { i64, ptr }
%glitch.File__g0__t97 = type { i64, ptr }
%glitch.SqlColumn__g0__t98 = type { i64, ptr, i32, ptr, ptr, i1 }
%glitch.ColumnOptions__g0__t99 = type { i64, ptr, ptr }
%glitch.MSSqlServerSinkOptions__g0__t100 = type { i64, ptr, ptr, ptr, i1, i32, ptr, ptr, ptr }
%glitch.Conduit_Domain_Article__g0__t101 = type { i64, ptr, i32, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Domain_ArticleFavorite__g0__t102 = type { i64, ptr, i32, ptr, i32, ptr }
%glitch.Conduit_Domain_ArticleTag__g0__t103 = type { i64, ptr, i32, ptr, ptr, ptr }
%glitch.Conduit_Domain_Comment__g0__t104 = type { i64, ptr, i32, ptr, ptr, i32, ptr, i32, ptr, ptr }
%glitch.Conduit_Domain_FollowedPeople__g0__t105 = type { i64, ptr, i32, ptr, i32, ptr }
%glitch.Conduit_Domain_Person__g0__t106 = type { i64, ptr, i32, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Domain_Tag__g0__t107 = type { i64, ptr, ptr, ptr }
%glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Articles_ArticleExtensions__g0__t109 = type { i64, ptr }
%glitch.Conduit_Features_Articles_ArticlesController__g0__t110 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Articles_ArticlesEnvelope__g0__t111 = type { i64, ptr, ptr, i32 }
%glitch.Conduit_Features_Articles_Create__g0__t112 = type { i64, ptr }
%glitch.Conduit_Features_Articles_ArticleData__g0__t113 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Articles_ArticleDataValidator__g0__t114 = type { i64, ptr }
%glitch.Conduit_Features_Articles_Command__g0__t115 = type { i64, ptr, ptr, ptr, ptr, i32, ptr, ptr }
%glitch.Conduit_Features_Articles_CommandValidator__g0__t116 = type { i64, ptr }
%glitch.Conduit_Features_Articles_Handler__g0__t117 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Articles_Delete__g0__t118 = type { i64, ptr }
%glitch.Conduit_Features_Articles_QueryHandler__g0__t119 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Articles_Details__g0__t120 = type { i64, ptr }
%glitch.Conduit_Features_Articles_Query__g0__t121 = type { i64, ptr, ptr, ptr, ptr, ptr, i32, i32, i1, ptr }
%glitch.Conduit_Features_Articles_QueryValidator__g0__t122 = type { i64, ptr }
%glitch.Conduit_Features_Articles_Edit__g0__t123 = type { i64, ptr }
%glitch.Conduit_Features_Articles_Model__g0__t124 = type { i64, ptr, ptr, ptr }
%glitch.Conduit_Features_Articles_List__g0__t125 = type { i64, ptr, i32 }
%glitch.Conduit_Features_Comments_CommentEnvelope__g0__t126 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Comments_CommentsController__g0__t127 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Comments_CommentsEnvelope__g0__t128 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Comments_Create__g0__t129 = type { i64, ptr }
%glitch.Conduit_Features_Comments_CommentData__g0__t130 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Comments_Command__g0__t131 = type { i64, ptr, ptr, ptr, ptr, i32, ptr, ptr }
%glitch.Conduit_Features_Comments_Model__g0__t132 = type { i64, ptr, ptr, ptr }
%glitch.Conduit_Features_Comments_CommandValidator__g0__t133 = type { i64, ptr }
%glitch.Conduit_Features_Comments_Handler__g0__t134 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Comments_Delete__g0__t135 = type { i64, ptr }
%glitch.Conduit_Features_Comments_QueryHandler__g0__t136 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Comments_List__g0__t137 = type { i64, ptr, i32 }
%glitch.Conduit_Features_Comments_Query__g0__t138 = type { i64, ptr, ptr, ptr, ptr, ptr, i32, i32, i1, ptr }
%glitch.Conduit_Features_Favorites_Add__g0__t139 = type { i64, ptr }
%glitch.Conduit_Features_Favorites_Command__g0__t140 = type { i64, ptr, ptr, ptr, ptr, i32, ptr, ptr }
%glitch.Conduit_Features_Favorites_CommandValidator__g0__t141 = type { i64, ptr }
%glitch.Conduit_Features_Favorites_QueryHandler__g0__t142 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Favorites_Delete__g0__t143 = type { i64, ptr }
%glitch.Conduit_Features_Favorites_FavoritesController__g0__t144 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Followers_Add__g0__t145 = type { i64, ptr }
%glitch.Conduit_Features_Followers_Command__g0__t146 = type { i64, ptr, ptr, ptr, ptr, i32, ptr, ptr }
%glitch.Conduit_Features_Followers_CommandValidator__g0__t147 = type { i64, ptr }
%glitch.Conduit_Features_Followers_QueryHandler__g0__t148 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Followers_Delete__g0__t149 = type { i64, ptr }
%glitch.Conduit_Features_Followers_FollowersController__g0__t150 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Profiles_Details__g0__t151 = type { i64, ptr }
%glitch.Conduit_Features_Profiles_Query__g0__t152 = type { i64, ptr, ptr, ptr, ptr, ptr, i32, i32, i1, ptr }
%glitch.Conduit_Features_Profiles_QueryValidator__g0__t153 = type { i64, ptr }
%glitch.Conduit_Features_Profiles_QueryHandler__g0__t154 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Profiles_IProfileReader__g0__t155 = type { i64, ptr }
%glitch.Conduit_Features_Profiles_MappingProfile__g0__t156 = type { i64, ptr, ptr, ptr, ptr, i1 }
%glitch.Conduit_Features_Profiles_Profile__g0__t157 = type { i64, ptr, ptr, ptr, ptr, i1 }
%glitch.Conduit_Features_Profiles_ProfileEnvelope__g0__t158 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Profiles_ProfileReader__g0__t159 = type { i64, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Profiles_ProfilesController__g0__t160 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Tags_List__g0__t161 = type { i64, ptr, i32 }
%glitch.Conduit_Features_Tags_Query__g0__t162 = type { i64, ptr, ptr, ptr, ptr, ptr, i32, i32, i1, ptr }
%glitch.Conduit_Features_Tags_QueryHandler__g0__t163 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Tags_TagsController__g0__t164 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Tags_TagsEnvelope__g0__t165 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Users_Create__g0__t166 = type { i64, ptr }
%glitch.Conduit_Features_Users_UserData__g0__t167 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Users_Command__g0__t168 = type { i64, ptr, ptr, ptr, ptr, i32, ptr, ptr }
%glitch.Conduit_Features_Users_CommandValidator__g0__t169 = type { i64, ptr }
%glitch.Conduit_Features_Users_Handler__g0__t170 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Users_Details__g0__t171 = type { i64, ptr }
%glitch.Conduit_Features_Users_Query__g0__t172 = type { i64, ptr, ptr, ptr, ptr, ptr, i32, i32, i1, ptr }
%glitch.Conduit_Features_Users_QueryValidator__g0__t173 = type { i64, ptr }
%glitch.Conduit_Features_Users_QueryHandler__g0__t174 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Users_Edit__g0__t175 = type { i64, ptr }
%glitch.Conduit_Features_Users_Login__g0__t176 = type { i64, ptr }
%glitch.Conduit_Features_Users_MappingProfile__g0__t177 = type { i64, ptr, ptr, ptr, ptr, i1 }
%glitch.Conduit_Features_Users_User__g0__t178 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Features_Users_UserEnvelope__g0__t179 = type { i64, ptr, ptr }
%glitch.Conduit_Features_Users_UserController__g0__t180 = type { i64, ptr, ptr, ptr }
%glitch.Conduit_Features_Users_UsersController__g0__t181 = type { i64, ptr, ptr }
%glitch.Conduit_Infrastructure_ConduitContext__g0__t182 = type { i64, ptr, ptr, i1, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Infrastructure_CurrentUserAccessor__g0__t183 = type { i64, ptr, ptr }
%glitch.Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184 = type { i64, ptr, ptr }
%glitch.Conduit_Infrastructure_Errors_Constants__g0__t185 = type { i64, ptr, ptr, ptr, ptr }
%glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186 = type { i64, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Infrastructure_Errors_RestException__g0__t187 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Infrastructure_GroupByApiRootConvention__g0__t188 = type { i64, ptr }
%glitch.Conduit_Infrastructure_ICurrentUserAccessor__g0__t189 = type { i64, ptr }
%glitch.Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190 = type { i64, ptr }
%glitch.Conduit_Infrastructure_Security_IPasswordHasher__g0__t191 = type { i64, ptr }
%glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192 = type { i64, ptr, ptr, ptr, ptr, ptr, ptr, ptr }
%glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193 = type { i64, ptr, ptr, ptr }
%glitch.Conduit_Infrastructure_Security_PasswordHasher__g0__t194 = type { i64, ptr, ptr }
%glitch.Conduit_Infrastructure_Slug__g0__t195 = type { i64, ptr }
%glitch.Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196 = type { i64, ptr, ptr, ptr }
%glitch.Conduit_Infrastructure_ValidatorActionFilter__g0__t197 = type { i64, ptr }
%glitch.Conduit_ServicesExtensions__g0__t198 = type { i64, ptr }
%glitch.lambda.9.env = type { ptr }
%glitch.lambda.10.env = type { ptr }
%glitch.lambda.11.env = type { ptr }
%glitch.lambda.12.env = type { ptr }
%glitch.lambda.13.env = type { ptr }
%glitch.lambda.14.env = type { ptr }
%glitch.lambda.17.env = type { ptr }
%glitch.lambda.18.env = type { ptr }
%glitch.lambda.19.env = type { ptr }
%glitch.lambda.20.env = type { ptr }
%glitch.lambda.21.env = type { ptr }
%glitch.lambda.23.env = type { ptr }
%glitch.lambda.24.env = type { ptr }
%glitch.lambda.25.env = type { ptr, ptr }
%glitch.lambda.26.env = type { ptr }
%glitch.lambda.28.env = type { ptr }
%glitch.lambda.29.env = type { ptr }
%glitch.lambda.30.env = type { ptr, ptr }
%glitch.lambda.32.env = type { ptr }
%glitch.lambda.33.env = type { ptr }
%glitch.lambda.34.env = type { ptr }
%glitch.lambda.40.env = type { ptr }
%glitch.lambda.41.env = type { ptr }
%glitch.lambda.43.env = type { ptr }
%glitch.lambda.49.env = type { ptr }
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
  %live = load i64, ptr @glitch_live_allocations
  %next = add i64 %live, 1
  store i64 %next, ptr @glitch_live_allocations
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
  %live = load i64, ptr @glitch_live_allocations
  %next = add i64 %live, 1
  store i64 %next, ptr @glitch_live_allocations
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
  %live = load i64, ptr @glitch_live_allocations
  %next = sub i64 %live, 1
  store i64 %next, ptr @glitch_live_allocations
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
@.fmt_i64 = private unnamed_addr constant [6 x i8] c"%lld\0A\00"
@.fmt_i32 = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@.fmt_double = private unnamed_addr constant [4 x i8] c"%f\0A\00"
@.fmt_str = private unnamed_addr constant [4 x i8] c"%s\0A\00"
@.fmt_json_i64 = private unnamed_addr constant [5 x i8] c"%lld\00"
@.fmt_json_double = private unnamed_addr constant [6 x i8] c"%.17g\00"
@.json_true = private unnamed_addr constant [5 x i8] c"true\00"
@.json_false = private unnamed_addr constant [6 x i8] c"false\00"
@.env_report_leaks = private unnamed_addr constant [20 x i8] c"GLITCH_REPORT_LEAKS\00"
@.str.0 = private unnamed_addr constant { i64, i64, [30 x i8] } { i64 1000000000, i64 29, [30 x i8] c"Sequence contains no elements\00" }
@.str.1 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.2 = private unnamed_addr constant { i64, i64, [15 x i8] } { i64 1000000000, i64 14, [15 x i8] c"select * from \00" }
@.str.3 = private unnamed_addr constant { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"sql\00" }
@.str.4 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.5 = private unnamed_addr constant { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"sql\00" }
@.str.6 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.7 = private unnamed_addr constant { i64, i64, [22 x i8] } { i64 1000000000, i64 21, [22 x i8] c"DbContext is disposed\00" }
@.str.8 = private unnamed_addr constant { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"sql\00" }
@.str.9 = private unnamed_addr constant { i64, i64, [9 x i8] } { i64 1000000000, i64 8, [9 x i8] c"tracked:\00" }
@.str.10 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.11 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.12 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.13 = private unnamed_addr constant { i64, i64, [10 x i8] } { i64 1000000000, i64 9, [10 x i8] c"127.0.0.1\00" }
@.str.14 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.15 = private unnamed_addr constant { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"trace\00" }
@.str.16 = private unnamed_addr constant { i64, i64, [13 x i8] } { i64 1000000000, i64 12, [13 x i8] c"Content-Type\00" }
@.str.17 = private unnamed_addr constant { i64, i64, [17 x i8] } { i64 1000000000, i64 16, [17 x i8] c"application/json\00" }
@.str.18 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.19 = private unnamed_addr constant { i64, i64, [8 x i8] } { i64 1000000000, i64 7, [8 x i8] c"routing\00" }
@.str.20 = private unnamed_addr constant { i64, i64, [10 x i8] } { i64 1000000000, i64 9, [10 x i8] c"endpoints\00" }
@.str.21 = private unnamed_addr constant { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"trace\00" }
@.str.22 = private unnamed_addr constant { i64, i64, [14 x i8] } { i64 1000000000, i64 13, [14 x i8] c"json-envelope\00" }
@.str.23 = private unnamed_addr constant { i64, i64, [18 x i8] } { i64 1000000000, i64 17, [18 x i8] c"https-redirection\00" }
@.str.24 = private unnamed_addr constant { i64, i64, [14 x i8] } { i64 1000000000, i64 13, [14 x i8] c"authorization\00" }
@.str.25 = private unnamed_addr constant { i64, i64, [15 x i8] } { i64 1000000000, i64 14, [15 x i8] c"authentication\00" }
@.str.26 = private unnamed_addr constant { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"cors\00" }
@.str.27 = private unnamed_addr constant { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"cors\00" }
@.str.28 = private unnamed_addr constant { i64, i64, [12 x i8] } { i64 1000000000, i64 11, [12 x i8] c"controllers\00" }
@.str.29 = private unnamed_addr constant { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c"GET \00" }
@.str.30 = private unnamed_addr constant { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"POST \00" }
@.str.31 = private unnamed_addr constant { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c" \00" }
@.str.32 = private unnamed_addr constant { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"404\00" }
@.str.33 = private unnamed_addr constant { i64, i64, [14 x i8] } { i64 1000000000, i64 13, [14 x i8] c"json-envelope\00" }
@.str.34 = private unnamed_addr constant { i64, i64, [11 x i8] } { i64 1000000000, i64 10, [11 x i8] c"{\22result\22:\00" }
@.str.35 = private unnamed_addr constant { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c"}\00" }
@.str.36 = private unnamed_addr constant { i64, i64, [6 x i8] } { i64 1000000000, i64 5, [6 x i8] c"trace\00" }
@.str.37 = private unnamed_addr constant { i64, i64, [7 x i8] } { i64 1000000000, i64 6, [7 x i8] c"trace:\00" }
@.str.38 = private unnamed_addr constant { i64, i64, [7 x i8] } { i64 1000000000, i64 6, [7 x i8] c"Bearer\00" }
@.str.39 = private unnamed_addr constant { i64, i64, [9 x i8] } { i64 1000000000, i64 8, [9 x i8] c"[Error] \00" }
@.str.40 = private unnamed_addr constant { i64, i64, [11 x i8] } { i64 1000000000, i64 10, [11 x i8] c"[Warning] \00" }
@.str.41 = private unnamed_addr constant { i64, i64, [8 x i8] } { i64 1000000000, i64 7, [8 x i8] c"[Info] \00" }
@.str.42 = private unnamed_addr constant { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c"/\00" }
@.str.43 = private unnamed_addr constant { i64, i64, [5 x i8] } { i64 1000000000, i64 4, [5 x i8] c".jpg\00" }
define ptr @glitch_lambda_0(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = icmp ne ptr null, null
  %t2 = inttoptr i1 %t1 to ptr
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_0_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_1(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_1_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_2(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_2_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_3(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_3_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_4(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_4_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_5(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_5_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_6(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_6_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_7(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_7_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_8(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_8_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_9(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.9.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t4, i32 0, i32 3
  %t6 = load ptr, ptr %t5
  %t7 = call ptr @Conduit_Infrastructure_CurrentUserAccessor__g0__t183_GetCurrentUsername__g0(ptr %t6)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = icmp eq ptr null, %t7
  %t11 = inttoptr i1 %t10 to ptr
  ret ptr %t11
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_9_destroy(ptr %env) {
entry:
  %t25 = getelementptr inbounds %glitch.lambda.9.env, ptr %env, i32 0, i32 0
  %t26 = load ptr, ptr %t25
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_10(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.10.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = getelementptr %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr null, i32 1
  %t5 = ptrtoint ptr %t4 to i64
  %t6 = call ptr @glitch_calloc(i64 1, i64 %t5)
  %t7 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t6, i32 0, i32 0
  store i64 1, ptr %t7
  %t8 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t6, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Domain_ArticleTag__g0__t103, ptr %t8
  %t9 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Domain_Article__g0__t101(ptr %t9)
  %t10 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t6, i32 0, i32 3
  store ptr %t9, ptr %t10
  %t11 = load ptr, ptr %t0
  call void @glitch_retain_Conduit_Domain_Tag__g0__t107(ptr %t11)
  %t12 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t6, i32 0, i32 5
  store ptr %t11, ptr %t12
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_10_destroy(ptr %env) {
entry:
  %t164 = getelementptr inbounds %glitch.lambda.10.env, ptr %env, i32 0, i32 0
  %t165 = load ptr, ptr %t164
  call void @glitch_drop_Article(ptr %t165)
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_11(ptr %env, ptr %t) {
entry:
  %t0 = alloca ptr
  store ptr %t, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.11.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = icmp eq ptr null, %t4
  %t6 = inttoptr i1 %t5 to ptr
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_11_destroy(ptr %env) {
entry:
  %t53 = getelementptr inbounds %glitch.lambda.11.env, ptr %env, i32 0, i32 0
  %t54 = load ptr, ptr %t53
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_12(ptr %env, ptr %t) {
entry:
  %t0 = alloca ptr
  store ptr %t, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.12.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t0
  %t5 = icmp eq ptr %t4, null
  %t6 = inttoptr i1 %t5 to ptr
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_12_destroy(ptr %env) {
entry:
  %t48 = getelementptr inbounds %glitch.lambda.12.env, ptr %env, i32 0, i32 0
  %t49 = load ptr, ptr %t48
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_13(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.13.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t4, i32 0, i32 3
  %t6 = load ptr, ptr %t5
  %t7 = icmp eq ptr null, %t6
  %t8 = inttoptr i1 %t7 to ptr
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_13_destroy(ptr %env) {
entry:
  %t16 = getelementptr inbounds %glitch.lambda.13.env, ptr %env, i32 0, i32 0
  %t17 = load ptr, ptr %t16
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_14(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.14.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t5
  %t7 = icmp eq ptr null, %t6
  %t8 = inttoptr i1 %t7 to ptr
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_14_destroy(ptr %env) {
entry:
  %t16 = getelementptr inbounds %glitch.lambda.14.env, ptr %env, i32 0, i32 0
  %t17 = load ptr, ptr %t16
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_15(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_15_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_16(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_16_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_17(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.17.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t4, i32 0, i32 3
  %t6 = load ptr, ptr %t5
  %t7 = icmp eq ptr null, %t6
  %t8 = inttoptr i1 %t7 to ptr
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_17_destroy(ptr %env) {
entry:
  %t18 = getelementptr inbounds %glitch.lambda.17.env, ptr %env, i32 0, i32 0
  %t19 = load ptr, ptr %t18
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_18(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.18.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t4, i32 0, i32 3
  %t6 = load ptr, ptr %t5
  %t7 = call ptr @Conduit_Infrastructure_CurrentUserAccessor__g0__t183_GetCurrentUsername__g0(ptr %t6)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = icmp eq ptr null, %t7
  %t11 = inttoptr i1 %t10 to ptr
  ret ptr %t11
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_18_destroy(ptr %env) {
entry:
  %t53 = getelementptr inbounds %glitch.lambda.18.env, ptr %env, i32 0, i32 0
  %t54 = load ptr, ptr %t53
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_19(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.19.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t4, i32 0, i32 3
  %t6 = load ptr, ptr %t5
  %t7 = icmp eq ptr null, %t6
  %t8 = inttoptr i1 %t7 to ptr
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_19_destroy(ptr %env) {
entry:
  %t17 = getelementptr inbounds %glitch.lambda.19.env, ptr %env, i32 0, i32 0
  %t18 = load ptr, ptr %t17
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_20(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.20.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t4, i32 0, i32 5
  %t6 = load i32, ptr %t5
  %t7 = icmp eq ptr null, null
  %t8 = inttoptr i1 %t7 to ptr
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_20_destroy(ptr %env) {
entry:
  %t48 = getelementptr inbounds %glitch.lambda.20.env, ptr %env, i32 0, i32 0
  %t49 = load ptr, ptr %t48
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_21(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.21.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t5
  %t7 = icmp eq ptr null, %t6
  %t8 = inttoptr i1 %t7 to ptr
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_21_destroy(ptr %env) {
entry:
  %t16 = getelementptr inbounds %glitch.lambda.21.env, ptr %env, i32 0, i32 0
  %t17 = load ptr, ptr %t16
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_22(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_22_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_23(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.23.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t4, i32 0, i32 3
  %t6 = load ptr, ptr %t5
  %t7 = icmp eq ptr null, %t6
  %t8 = inttoptr i1 %t7 to ptr
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_23_destroy(ptr %env) {
entry:
  %t18 = getelementptr inbounds %glitch.lambda.23.env, ptr %env, i32 0, i32 0
  %t19 = load ptr, ptr %t18
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_24(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.24.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t4, i32 0, i32 3
  %t6 = load ptr, ptr %t5
  %t7 = call ptr @Conduit_Infrastructure_CurrentUserAccessor__g0__t183_GetCurrentUsername__g0(ptr %t6)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = icmp eq ptr null, %t7
  %t11 = inttoptr i1 %t10 to ptr
  ret ptr %t11
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_24_destroy(ptr %env) {
entry:
  %t48 = getelementptr inbounds %glitch.lambda.24.env, ptr %env, i32 0, i32 0
  %t49 = load ptr, ptr %t48
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_25(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.25.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = alloca ptr
  %t5 = getelementptr inbounds %glitch.lambda.25.env, ptr %env, i32 0, i32 1
  %t6 = load ptr, ptr %t5
  store ptr %t6, ptr %t4
  %t7 = icmp eq ptr null, null
  %t8 = icmp eq ptr null, null
  %t9 = and i1 %t7, %t8
  %t10 = inttoptr i1 %t9 to ptr
  ret ptr %t10
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_25_destroy(ptr %env) {
entry:
  %t80 = getelementptr inbounds %glitch.lambda.25.env, ptr %env, i32 0, i32 0
  %t81 = load ptr, ptr %t80
  %t82 = getelementptr inbounds %glitch.lambda.25.env, ptr %env, i32 0, i32 1
  %t83 = load ptr, ptr %t82
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_26(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.26.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = icmp eq ptr null, null
  %t5 = inttoptr i1 %t4 to ptr
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_26_destroy(ptr %env) {
entry:
  %t125 = getelementptr inbounds %glitch.lambda.26.env, ptr %env, i32 0, i32 0
  %t126 = load ptr, ptr %t125
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_27(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_27_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_28(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.28.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t4, i32 0, i32 6
  %t6 = load ptr, ptr %t5
  %t7 = icmp eq ptr null, %t6
  %t8 = inttoptr i1 %t7 to ptr
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_28_destroy(ptr %env) {
entry:
  %t18 = getelementptr inbounds %glitch.lambda.28.env, ptr %env, i32 0, i32 0
  %t19 = load ptr, ptr %t18
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_29(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.29.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t4, i32 0, i32 3
  %t6 = load ptr, ptr %t5
  %t7 = call ptr @Conduit_Infrastructure_CurrentUserAccessor__g0__t183_GetCurrentUsername__g0(ptr %t6)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = icmp eq ptr null, %t7
  %t11 = inttoptr i1 %t10 to ptr
  ret ptr %t11
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_29_destroy(ptr %env) {
entry:
  %t48 = getelementptr inbounds %glitch.lambda.29.env, ptr %env, i32 0, i32 0
  %t49 = load ptr, ptr %t48
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_30(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.30.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = alloca ptr
  %t5 = getelementptr inbounds %glitch.lambda.30.env, ptr %env, i32 0, i32 1
  %t6 = load ptr, ptr %t5
  store ptr %t6, ptr %t4
  %t7 = icmp eq ptr null, null
  %t8 = icmp eq ptr null, null
  %t9 = and i1 %t7, %t8
  %t10 = inttoptr i1 %t9 to ptr
  ret ptr %t10
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_30_destroy(ptr %env) {
entry:
  %t80 = getelementptr inbounds %glitch.lambda.30.env, ptr %env, i32 0, i32 0
  %t81 = load ptr, ptr %t80
  %t82 = getelementptr inbounds %glitch.lambda.30.env, ptr %env, i32 0, i32 1
  %t83 = load ptr, ptr %t82
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_31(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_31_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_32(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.32.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = icmp eq ptr null, %t4
  %t6 = inttoptr i1 %t5 to ptr
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_32_destroy(ptr %env) {
entry:
  %t25 = getelementptr inbounds %glitch.lambda.32.env, ptr %env, i32 0, i32 0
  %t26 = load ptr, ptr %t25
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_33(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.33.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = icmp eq ptr null, %t4
  %t6 = inttoptr i1 %t5 to ptr
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_33_destroy(ptr %env) {
entry:
  %t75 = getelementptr inbounds %glitch.lambda.33.env, ptr %env, i32 0, i32 0
  %t76 = load ptr, ptr %t75
  call void @glitch_string_release(ptr %t76)
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_34(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.34.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = icmp eq ptr null, null
  %t5 = inttoptr i1 %t4 to ptr
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_34_destroy(ptr %env) {
entry:
  %t105 = getelementptr inbounds %glitch.lambda.34.env, ptr %env, i32 0, i32 0
  %t106 = load ptr, ptr %t105
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_35(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_35_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_36(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t2 = icmp eq ptr null, null
  %t1 = alloca ptr
  br i1 %t2, label %coalesce_right_1, label %coalesce_left_0
coalesce_left_0:
  store ptr null, ptr %t1
  br label %coalesce_end_2
coalesce_right_1:
  store ptr null, ptr %t1
  br label %coalesce_end_2
coalesce_end_2:
  %t3 = load ptr, ptr %t1
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_36_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_37(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_37_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_38(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_38_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_39(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_39_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_40(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.40.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t4, i32 0, i32 7
  %t6 = load ptr, ptr %t5
  %t7 = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = icmp eq ptr null, %t8
  %t10 = inttoptr i1 %t9 to ptr
  ret ptr %t10
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_40_destroy(ptr %env) {
entry:
  %t23 = getelementptr inbounds %glitch.lambda.40.env, ptr %env, i32 0, i32 0
  %t24 = load ptr, ptr %t23
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_41(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.41.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t4, i32 0, i32 7
  %t6 = load ptr, ptr %t5
  %t7 = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %t6, i32 0, i32 3
  %t8 = load ptr, ptr %t7
  %t9 = icmp eq ptr null, %t8
  %t10 = inttoptr i1 %t9 to ptr
  ret ptr %t10
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_41_destroy(ptr %env) {
entry:
  %t57 = getelementptr inbounds %glitch.lambda.41.env, ptr %env, i32 0, i32 0
  %t58 = load ptr, ptr %t57
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_42(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_42_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_43(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.43.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t1
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t4, i32 0, i32 9
  %t6 = load ptr, ptr %t5
  %t7 = icmp eq ptr null, %t6
  %t8 = inttoptr i1 %t7 to ptr
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_43_destroy(ptr %env) {
entry:
  %t17 = getelementptr inbounds %glitch.lambda.43.env, ptr %env, i32 0, i32 0
  %t18 = load ptr, ptr %t17
  call void @glitch_free(ptr %env)
  ret void
}

@.str.44 = private unnamed_addr constant { i64, i64, [10 x i8] } { i64 1000000000, i64 9, [10 x i8] c"<unknown>\00" }
define ptr @glitch_lambda_44(ptr %env, ptr %b) {
entry:
  %t0 = alloca ptr
  store ptr %b, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_44_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_45(ptr %env, ptr %b) {
entry:
  %t0 = alloca ptr
  store ptr %b, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_45_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_46(ptr %env, ptr %b) {
entry:
  %t0 = alloca ptr
  store ptr %b, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_46_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.45 = private unnamed_addr constant { i64, i64, [8 x i8] } { i64 1000000000, i64 7, [8 x i8] c"rethrow\00" }
define ptr @glitch_lambda_47(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = icmp eq ptr null, null
  %t2 = inttoptr i1 %t1 to ptr
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_47_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.46 = private unnamed_addr constant { i64, i64, [8 x i8] } { i64 1000000000, i64 7, [8 x i8] c"rethrow\00" }
@.str.47 = private unnamed_addr constant { i64, i64, [20 x i8] } { i64 1000000000, i64 19, [20 x i8] c"Unhandled Exception\00" }
@.str.48 = private unnamed_addr constant { i64, i64, [17 x i8] } { i64 1000000000, i64 16, [17 x i8] c"application/json\00" }
@.str.49 = private unnamed_addr constant { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c"/\00" }
@.str.50 = private unnamed_addr constant { i64, i64, [8 x i8] } { i64 1000000000, i64 7, [8 x i8] c"default\00" }
define ptr @glitch_lambda_48(ptr %env) {
entry:
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_48_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.51 = private unnamed_addr constant { i64, i64, [1 x i8] } { i64 1000000000, i64 0, [1 x i8] c"\00" }
@.str.52 = private unnamed_addr constant { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c" \00" }
@.str.53 = private unnamed_addr constant { i64, i64, [2 x i8] } { i64 1000000000, i64 1, [2 x i8] c"-\00" }
define ptr @glitch_lambda_49(ptr %env, ptr %v) {
entry:
  %t0 = alloca ptr
  store ptr %v, ptr %t0
  %t1 = alloca ptr
  %t2 = getelementptr inbounds %glitch.lambda.49.env, ptr %env, i32 0, i32 0
  %t3 = load ptr, ptr %t2
  store ptr %t3, ptr %t1
  %t4 = load ptr, ptr %t0
  %t5 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_49_destroy(ptr %env) {
entry:
  %t22 = getelementptr inbounds %glitch.lambda.49.env, ptr %env, i32 0, i32 0
  %t23 = load ptr, ptr %t22
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_50(ptr %env, ptr %result) {
entry:
  %t0 = alloca ptr
  store ptr %result, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_50_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_51(ptr %env, ptr %f) {
entry:
  %t0 = alloca ptr
  store ptr %f, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = icmp ne ptr %t1, null
  %t3 = inttoptr i1 %t2 to ptr
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_51_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.54 = private unnamed_addr constant { i64, i64, [17 x i8] } { i64 1000000000, i64 16, [17 x i8] c"application/json\00" }
define ptr @glitch_lambda_52(ptr %env, ptr %cfg) {
entry:
  %t0 = alloca ptr
  store ptr %cfg, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_52_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_53(ptr %env, ptr %cfg) {
entry:
  %t0 = alloca ptr
  store ptr %cfg, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_53_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.55 = private unnamed_addr constant { i64, i64, [7 x i8] } { i64 1000000000, i64 6, [7 x i8] c"issuer\00" }
@.str.56 = private unnamed_addr constant { i64, i64, [9 x i8] } { i64 1000000000, i64 8, [9 x i8] c"audience\00" }
define ptr @glitch_lambda_54(ptr %env, ptr %options) {
entry:
  %t0 = alloca ptr
  store ptr %options, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_54_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_55(ptr %env, ptr %options) {
entry:
  %t0 = alloca ptr
  store ptr %options, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_55_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.57 = private unnamed_addr constant { i64, i64, [77 x i8] } { i64 1000000000, i64 76, [77 x i8] c"{Timestamp:HH:mm:ss} [{Level}] {SourceContext} {Message}{NewLine}{Exception}\00" }
@.str.58 = private unnamed_addr constant { i64, i64, [22 x i8] } { i64 1000000000, i64 21, [22 x i8] c"Filename=realworld.db\00" }
@.str.59 = private unnamed_addr constant { i64, i64, [7 x i8] } { i64 1000000000, i64 6, [7 x i8] c"sqlite\00" }
define ptr @glitch_lambda_56(ptr %env, ptr %options) {
entry:
  %t0 = alloca ptr
  store ptr %options, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_56_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.60 = private unnamed_addr constant { i64, i64, [10 x i8] } { i64 1000000000, i64 9, [10 x i8] c"Resources\00" }
define ptr @glitch_lambda_57(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_57_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_58(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_58_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_59(ptr %env, ptr %opt) {
entry:
  %t0 = alloca ptr
  store ptr %opt, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_59_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_60(ptr %env, ptr %opt) {
entry:
  %t0 = alloca ptr
  store ptr %opt, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_60_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

define ptr @glitch_lambda_61(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_61_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.61 = private unnamed_addr constant { i64, i64, [36 x i8] } { i64 1000000000, i64 35, [36 x i8] c"swagger/{documentName}/swagger.json\00" }
define ptr @glitch_lambda_62(ptr %env, ptr %c) {
entry:
  %t0 = alloca ptr
  store ptr %c, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_62_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.62 = private unnamed_addr constant { i64, i64, [25 x i8] } { i64 1000000000, i64 24, [25 x i8] c"/swagger/v1/swagger.json\00" }
@.str.63 = private unnamed_addr constant { i64, i64, [17 x i8] } { i64 1000000000, i64 16, [17 x i8] c"RealWorld API V1\00" }
define ptr @glitch_lambda_63(ptr %env, ptr %x) {
entry:
  %t0 = alloca ptr
  store ptr %x, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [25 x i8] }, ptr @.str.62, i32 0, i32 2, i64 0))
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [17 x i8] }, ptr @.str.63, i32 0, i32 2, i64 0))
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @glitch_lambda_63_destroy(ptr %env) {
entry:
  call void @glitch_free(ptr %env)
  ret void
}

@.str.64 = private unnamed_addr constant { i64, i64, [4 x i8] } { i64 1000000000, i64 3, [4 x i8] c"404\00" }

define void @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_Errors_RestException__g0__t187(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_Errors_RestException__g0__t187(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_ServicesExtensions__g0__t198(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_ServicesExtensions__g0__t198(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_ServicesExtensions__g0__t198, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_ServicesExtensions__g0__t198(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_ServicesExtensions__g0__t198, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_ServicesExtensions__g0__t198, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_ServicesExtensions__g0__t198(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ServiceScope__g0__t49(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ServiceScope__g0__t49, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_IServiceProvider(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ServiceScope__g0__t49(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ServiceScope__g0__t49, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ServiceScope__g0__t49(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ServiceScope__g0__t49, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ServiceScope__g0__t49, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ServiceScope__g0__t49(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_GC__g0__t13(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_GC__g0__t13(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.GC__g0__t13, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_GC__g0__t13(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.GC__g0__t13, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.GC__g0__t13, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_GC__g0__t13(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IEnumerator__g1__t1(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IEnumerator__g1__t1(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerator__g1__t1, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IEnumerator__g1__t1(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerator__g1__t1, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IEnumerator__g1__t1, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IEnumerator__g1__t1(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_UserEnvelope__g0__t179(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_User(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_UserEnvelope__g0__t179(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_UserEnvelope__g0__t179(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_UserEnvelope__g0__t179(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_JwtIssuerOptions(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ObjectResult__g0__t70(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ObjectResult__g0__t70(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ObjectResult__g0__t70, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ObjectResult__g0__t70(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ObjectResult__g0__t70, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ObjectResult__g0__t70, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ObjectResult__g0__t70(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Exception__g0__t14(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Exception__g0__t14, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Exception__g0__t14(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Exception__g0__t14, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Exception__g0__t14(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Exception__g0__t14, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Exception__g0__t14, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Exception__g0__t14(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_MappingProfile__g0__t177(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_MappingProfile__g0__t177, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_MappingProfile__g0__t177, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_MappingProfile__g0__t177, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_MappingProfile__g0__t177(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_MappingProfile__g0__t177, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_MappingProfile__g0__t177(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_MappingProfile__g0__t177, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_MappingProfile__g0__t177, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_MappingProfile__g0__t177(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Directory__g0__t95(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Directory__g0__t95(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Directory__g0__t95, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Directory__g0__t95(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Directory__g0__t95, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Directory__g0__t95, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Directory__g0__t95(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Comments_CommentEnvelope__g0__t126(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentEnvelope__g0__t126, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_Comment(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Comments_CommentEnvelope__g0__t126(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentEnvelope__g0__t126, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Comments_CommentEnvelope__g0__t126(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentEnvelope__g0__t126, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentEnvelope__g0__t126, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Comments_CommentEnvelope__g0__t126(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Enum__g0__t19(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Enum__g0__t19(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Enum__g0__t19, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Enum__g0__t19(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Enum__g0__t19, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Enum__g0__t19, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Enum__g0__t19(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Domain_Tag__g0__t107(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %object, i32 0, i32 3
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
  call void @glitch_drop_Conduit_Domain_ArticleTag__g0__t103(ptr %t9)
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

define void @glitch_retain_Conduit_Domain_Tag__g0__t107(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Domain_Tag__g0__t107(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Domain_Tag__g0__t107(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Followers_FollowersController__g0__t150(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Followers_FollowersController__g0__t150(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Followers_FollowersController__g0__t150, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Followers_FollowersController__g0__t150(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Followers_FollowersController__g0__t150, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Followers_FollowersController__g0__t150, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Followers_FollowersController__g0__t150(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Articles_ArticlesEnvelope__g0__t111(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesEnvelope__g0__t111, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t11 = icmp eq ptr %field_2, null
  br i1 %t11, label %collection_release_done_6, label %collection_release_5
collection_release_5:
  %t12 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t13 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t14 = load i64, ptr %t12
  %t15 = load ptr, ptr %t13
  %t16 = alloca i64
  store i64 0, ptr %t16
  br label %element_drop_loop_7
element_drop_loop_7:
  %t17 = load i64, ptr %t16
  %t18 = icmp ult i64 %t17, %t14
  br i1 %t18, label %element_drop_body_8, label %element_drop_done_9
element_drop_body_8:
  %t19 = getelementptr inbounds ptr, ptr %t15, i64 %t17
  %t20 = load ptr, ptr %t19
  call void @glitch_drop_Conduit_Domain_Article__g0__t101(ptr %t20)
  %t21 = add i64 %t17, 1
  store i64 %t21, ptr %t16
  br label %element_drop_loop_7
element_drop_done_9:
  call void @glitch_free(ptr %t15)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_6
collection_release_done_6:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Articles_ArticlesEnvelope__g0__t111(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesEnvelope__g0__t111, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Articles_ArticlesEnvelope__g0__t111(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesEnvelope__g0__t111, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesEnvelope__g0__t111, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Articles_ArticlesEnvelope__g0__t111(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Articles_ArticleDataValidator__g0__t114(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Articles_ArticleDataValidator__g0__t114(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleDataValidator__g0__t114, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Articles_ArticleDataValidator__g0__t114(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleDataValidator__g0__t114, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleDataValidator__g0__t114, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Articles_ArticleDataValidator__g0__t114(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbContext__g0__t38(ptr %object) {
entry:
  %field_4_ptr = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t22 = icmp eq ptr %field_4, null
  br i1 %t22, label %collection_release_done_11, label %collection_release_10
collection_release_10:
  %t23 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 0
  %t24 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 2
  %t25 = load i64, ptr %t23
  %t26 = load ptr, ptr %t24
  %t27 = alloca i64
  store i64 0, ptr %t27
  br label %element_drop_loop_12
element_drop_loop_12:
  %t28 = load i64, ptr %t27
  %t29 = icmp ult i64 %t28, %t25
  br i1 %t29, label %element_drop_body_13, label %element_drop_done_14
element_drop_body_13:
  %t30 = getelementptr inbounds ptr, ptr %t26, i64 %t28
  %t31 = load ptr, ptr %t30
  call void @glitch_string_release(ptr %t31)
  %t32 = add i64 %t28, 1
  store i64 %t32, ptr %t27
  br label %element_drop_loop_12
element_drop_done_14:
  call void @glitch_free(ptr %t26)
  call void @glitch_free(ptr %field_4)
  br label %collection_release_done_11
collection_release_done_11:
  %field_5_ptr = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_SqlProvider(ptr %field_5)
  %field_6_ptr = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_DatabaseFacade(ptr %field_6)
  %field_7_ptr = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_ChangeTracker(ptr %field_7)
  %field_2_ptr = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbContext__g0__t38(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbContext__g0__t38(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbContext__g0__t38(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Endpoint__g0__t64(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Endpoint__g0__t64, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.Endpoint__g0__t64, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_4_ptr = getelementptr inbounds %glitch.Endpoint__g0__t64, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Endpoint__g0__t64(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Endpoint__g0__t64, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Endpoint__g0__t64(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Endpoint__g0__t64, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Endpoint__g0__t64, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Endpoint__g0__t64(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ArgumentException__g0__t15(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ArgumentException__g0__t15, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ArgumentException__g0__t15(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ArgumentException__g0__t15, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ArgumentException__g0__t15(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ArgumentException__g0__t15, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ArgumentException__g0__t15, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ArgumentException__g0__t15(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Comments_CommentData__g0__t130(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentData__g0__t130, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Comments_CommentData__g0__t130(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentData__g0__t130, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Comments_CommentData__g0__t130(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentData__g0__t130, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentData__g0__t130, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Comments_CommentData__g0__t130(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186(ptr %object) {
entry:
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_ILogger(ptr %field_4)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_delegate_release(ptr %field_5)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_CurrentUserAccessor__g0__t183(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_CurrentUserAccessor__g0__t183, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_IHttpContextAccessor(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_CurrentUserAccessor__g0__t183(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_CurrentUserAccessor__g0__t183, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_CurrentUserAccessor__g0__t183(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_CurrentUserAccessor__g0__t183, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_CurrentUserAccessor__g0__t183, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_CurrentUserAccessor__g0__t183(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ActionExecutingContext__g0__t73(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_ModelStateDictionary(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t33 = icmp eq ptr %field_2, null
  br i1 %t33, label %collection_release_done_16, label %collection_release_15
collection_release_15:
  %t34 = getelementptr inbounds %glitch.dict, ptr %field_2, i32 0, i32 0
  %t35 = getelementptr inbounds %glitch.dict, ptr %field_2, i32 0, i32 2
  %t36 = getelementptr inbounds %glitch.dict, ptr %field_2, i32 0, i32 3
  %t37 = load i64, ptr %t34
  %t38 = load ptr, ptr %t35
  %t39 = load ptr, ptr %t36
  %t40 = alloca i64
  store i64 0, ptr %t40
  br label %element_drop_loop_17
element_drop_loop_17:
  %t41 = load i64, ptr %t40
  %t42 = icmp ult i64 %t41, %t37
  br i1 %t42, label %element_drop_body_18, label %element_drop_done_19
element_drop_body_18:
  %t43 = getelementptr inbounds ptr, ptr %t38, i64 %t41
  %t44 = load ptr, ptr %t43
  call void @glitch_string_release(ptr %t44)
  %t45 = add i64 %t41, 1
  store i64 %t45, ptr %t40
  br label %element_drop_loop_17
element_drop_done_19:
  call void @glitch_free(ptr %t38)
  call void @glitch_free(ptr %t39)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_16
collection_release_done_16:
  %field_4_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_HttpContext(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ActionExecutingContext__g0__t73(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ActionExecutingContext__g0__t73(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ActionExecutingContext__g0__t73(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_string__g0__t22(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_string__g0__t22(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.string__g0__t22, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_string__g0__t22(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.string__g0__t22, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.string__g0__t22, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_string__g0__t22(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DatabaseFacade__g0__t35(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DatabaseFacade__g0__t35(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DatabaseFacade__g0__t35, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DatabaseFacade__g0__t35(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DatabaseFacade__g0__t35, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DatabaseFacade__g0__t35, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DatabaseFacade__g0__t35(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ConfigurationManager__g0__t53(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ConfigurationManager__g0__t53(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ConfigurationManager__g0__t53, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ConfigurationManager__g0__t53(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ConfigurationManager__g0__t53, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ConfigurationManager__g0__t53, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ConfigurationManager__g0__t53(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MappingExpression__g2__t84(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MappingExpression__g2__t84(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MappingExpression__g2__t84, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MappingExpression__g2__t84(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MappingExpression__g2__t84, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MappingExpression__g2__t84, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MappingExpression__g2__t84(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Dictionary__g2__t10(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Dictionary__g2__t10(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Dictionary__g2__t10, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Dictionary__g2__t10(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Dictionary__g2__t10, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Dictionary__g2__t10, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Dictionary__g2__t10(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_Slug__g0__t195(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_Slug__g0__t195(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Slug__g0__t195, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_Slug__g0__t195(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Slug__g0__t195, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Slug__g0__t195, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_Slug__g0__t195(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ModelStateEntry__g0__t76(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ModelStateEntry__g0__t76, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t46 = icmp eq ptr %field_2, null
  br i1 %t46, label %collection_release_done_21, label %collection_release_20
collection_release_20:
  %t47 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t48 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t49 = load i64, ptr %t47
  %t50 = load ptr, ptr %t48
  %t51 = alloca i64
  store i64 0, ptr %t51
  br label %element_drop_loop_22
element_drop_loop_22:
  %t52 = load i64, ptr %t51
  %t53 = icmp ult i64 %t52, %t49
  br i1 %t53, label %element_drop_body_23, label %element_drop_done_24
element_drop_body_23:
  %t54 = getelementptr inbounds ptr, ptr %t50, i64 %t52
  %t55 = load ptr, ptr %t54
  call void @glitch_drop_ModelError__g0__t75(ptr %t55)
  %t56 = add i64 %t52, 1
  store i64 %t56, ptr %t51
  br label %element_drop_loop_22
element_drop_done_24:
  call void @glitch_free(ptr %t50)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_21
collection_release_done_21:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ModelStateEntry__g0__t76(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ModelStateEntry__g0__t76, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ModelStateEntry__g0__t76(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ModelStateEntry__g0__t76, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ModelStateEntry__g0__t76, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ModelStateEntry__g0__t76(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ModelError__g0__t75(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ModelError__g0__t75, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ModelError__g0__t75(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ModelError__g0__t75, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ModelError__g0__t75(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ModelError__g0__t75, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ModelError__g0__t75, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ModelError__g0__t75(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DateTime__g0__t17(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.DateTime__g0__t17, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DateTime__g0__t17(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DateTime__g0__t17, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DateTime__g0__t17(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DateTime__g0__t17, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DateTime__g0__t17, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DateTime__g0__t17(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ConnectionInfo__g0__t60(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ConnectionInfo__g0__t60, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_IPAddress(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ConnectionInfo__g0__t60(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ConnectionInfo__g0__t60, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ConnectionInfo__g0__t60(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ConnectionInfo__g0__t60, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ConnectionInfo__g0__t60, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ConnectionInfo__g0__t60(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Articles_ArticleData__g0__t113(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  %t57 = icmp eq ptr %field_5, null
  br i1 %t57, label %array_release_done_26, label %array_release_25
array_release_25:
  %t58 = getelementptr inbounds %glitch.array, ptr %field_5, i32 0, i32 0
  %t60 = getelementptr inbounds %glitch.array, ptr %field_5, i32 0, i32 1
  %t59 = load i64, ptr %t58
  %t61 = load ptr, ptr %t60
  %t62 = alloca i64
  store i64 0, ptr %t62
  br label %element_drop_loop_27
element_drop_loop_27:
  %t63 = load i64, ptr %t62
  %t64 = icmp ult i64 %t63, %t59
  br i1 %t64, label %element_drop_body_28, label %element_drop_done_29
element_drop_body_28:
  %t65 = getelementptr inbounds ptr, ptr %t61, i64 %t63
  %t66 = load ptr, ptr %t65
  call void @glitch_string_release(ptr %t66)
  %t67 = add i64 %t63, 1
  store i64 %t67, ptr %t62
  br label %element_drop_loop_27
element_drop_done_29:
  call void @glitch_free(ptr %t61)
  call void @glitch_free(ptr %field_5)
  br label %array_release_done_26
array_release_done_26:
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Articles_ArticleData__g0__t113(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Articles_ArticleData__g0__t113(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Articles_ArticleData__g0__t113(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HttpContext__g0__t61(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_HttpResponse(ptr %field_3)
  %field_5_ptr = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  %field_4_ptr = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_ConnectionInfo(ptr %field_4)
  %field_2_ptr = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_HttpRequest(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HttpContext__g0__t61(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HttpContext__g0__t61(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HttpContext__g0__t61(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_PropertyInfo__g0__t21(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t21, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t21, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_Type(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_PropertyInfo__g0__t21(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t21, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_PropertyInfo__g0__t21(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t21, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.PropertyInfo__g0__t21, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_PropertyInfo__g0__t21(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IFormFile__g0__t72(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IFormFile__g0__t72, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IFormFile__g0__t72(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IFormFile__g0__t72, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IFormFile__g0__t72(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IFormFile__g0__t72, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IFormFile__g0__t72, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IFormFile__g0__t72(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Comments_CommentsController__g0__t127(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Comments_CommentsController__g0__t127(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsController__g0__t127, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Comments_CommentsController__g0__t127(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsController__g0__t127, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsController__g0__t127, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Comments_CommentsController__g0__t127(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ModelStateDictionary__g0__t77(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ModelStateDictionary__g0__t77(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ModelStateDictionary__g0__t77, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ModelStateDictionary__g0__t77(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ModelStateDictionary__g0__t77, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ModelStateDictionary__g0__t77, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ModelStateDictionary__g0__t77(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbSetString__g0__t40(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbSetString__g0__t40(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbSetString__g0__t40(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbSetString__g0__t40(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Domain_FollowedPeople__g0__t105(ptr %object) {
entry:
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_Person(ptr %field_5)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_Person(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Domain_FollowedPeople__g0__t105(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Domain_FollowedPeople__g0__t105(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Domain_FollowedPeople__g0__t105(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_CommandValidator__g0__t169(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_CommandValidator__g0__t169(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_CommandValidator__g0__t169, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_CommandValidator__g0__t169(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_CommandValidator__g0__t169, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_CommandValidator__g0__t169, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_CommandValidator__g0__t169(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HttpContextAccessor__g0__t63(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.HttpContextAccessor__g0__t63, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_HttpContext(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HttpContextAccessor__g0__t63(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HttpContextAccessor__g0__t63, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HttpContextAccessor__g0__t63(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HttpContextAccessor__g0__t63, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HttpContextAccessor__g0__t63, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HttpContextAccessor__g0__t63(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Articles_ArticlesController__g0__t110(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Articles_ArticlesController__g0__t110(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Articles_ArticlesController__g0__t110(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Articles_ArticlesController__g0__t110(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_Query__g0__t172(ptr %object) {
entry:
  %field_9_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %object, i32 0, i32 9
  %field_9 = load ptr, ptr %field_9_ptr
  call void @glitch_string_release(ptr %field_9)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_Query__g0__t172(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_Query__g0__t172(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_Query__g0__t172(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HttpResponse__g0__t58(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t68 = icmp eq ptr %field_3, null
  br i1 %t68, label %collection_release_done_31, label %collection_release_30
collection_release_30:
  %t69 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 0
  %t70 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 2
  %t71 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 3
  %t72 = load i64, ptr %t69
  %t73 = load ptr, ptr %t70
  %t74 = load ptr, ptr %t71
  %t75 = alloca i64
  store i64 0, ptr %t75
  br label %element_drop_loop_32
element_drop_loop_32:
  %t76 = load i64, ptr %t75
  %t77 = icmp ult i64 %t76, %t72
  br i1 %t77, label %element_drop_body_33, label %element_drop_done_34
element_drop_body_33:
  %t78 = getelementptr inbounds ptr, ptr %t73, i64 %t76
  %t79 = load ptr, ptr %t78
  call void @glitch_string_release(ptr %t79)
  %t80 = add i64 %t76, 1
  store i64 %t80, ptr %t75
  br label %element_drop_loop_32
element_drop_done_34:
  %t81 = alloca i64
  store i64 0, ptr %t81
  br label %element_drop_loop_35
element_drop_loop_35:
  %t82 = load i64, ptr %t81
  %t83 = icmp ult i64 %t82, %t72
  br i1 %t83, label %element_drop_body_36, label %element_drop_done_37
element_drop_body_36:
  %t84 = getelementptr inbounds ptr, ptr %t74, i64 %t82
  %t85 = load ptr, ptr %t84
  call void @glitch_string_release(ptr %t85)
  %t86 = add i64 %t82, 1
  store i64 %t86, ptr %t81
  br label %element_drop_loop_35
element_drop_done_37:
  call void @glitch_free(ptr %t73)
  call void @glitch_free(ptr %t74)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_31
collection_release_done_31:
  %field_4_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HttpResponse__g0__t58(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HttpResponse__g0__t58(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HttpResponse__g0__t58(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_UsersController__g0__t181(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_UsersController__g0__t181(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UsersController__g0__t181, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_UsersController__g0__t181(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UsersController__g0__t181, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UsersController__g0__t181, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_UsersController__g0__t181(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Domain_ArticleFavorite__g0__t102(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_Article(ptr %field_3)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_Person(ptr %field_5)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Domain_ArticleFavorite__g0__t102(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Domain_ArticleFavorite__g0__t102(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Domain_ArticleFavorite__g0__t102(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IList__g1__t4(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IList__g1__t4(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IList__g1__t4, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IList__g1__t4(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IList__g1__t4, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IList__g1__t4, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IList__g1__t4(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Articles_ArticleExtensions__g0__t109(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Articles_ArticleExtensions__g0__t109(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleExtensions__g0__t109, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Articles_ArticleExtensions__g0__t109(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleExtensions__g0__t109, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleExtensions__g0__t109, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Articles_ArticleExtensions__g0__t109(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_Details__g0__t171(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_Details__g0__t171(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Details__g0__t171, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_Details__g0__t171(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Details__g0__t171, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Details__g0__t171, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_Details__g0__t171(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_KeyValuePair__g2__t8(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %destroy_object
destroy_object:
  call void @glitch_free(ptr %object)
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Comments_Model__g0__t132(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_Model__g0__t132, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ArticleData(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_Model__g0__t132, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_CommentData(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Comments_Model__g0__t132(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_Model__g0__t132, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Comments_Model__g0__t132(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_Model__g0__t132, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_Model__g0__t132, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Comments_Model__g0__t132(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_GroupByApiRootConvention__g0__t188(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_GroupByApiRootConvention__g0__t188(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_GroupByApiRootConvention__g0__t188, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_GroupByApiRootConvention__g0__t188(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_GroupByApiRootConvention__g0__t188, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_GroupByApiRootConvention__g0__t188, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_GroupByApiRootConvention__g0__t188(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MapperConfiguration__g0__t87(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.MapperConfiguration__g0__t87, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_Mapper(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MapperConfiguration__g0__t87(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MapperConfiguration__g0__t87, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MapperConfiguration__g0__t87(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MapperConfiguration__g0__t87, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MapperConfiguration__g0__t87, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MapperConfiguration__g0__t87(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Profiles_IProfileReader__g0__t155(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Profiles_IProfileReader__g0__t155(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_IProfileReader__g0__t155, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Profiles_IProfileReader__g0__t155(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_IProfileReader__g0__t155, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_IProfileReader__g0__t155, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Profiles_IProfileReader__g0__t155(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t87 = icmp eq ptr %field_3, null
  br i1 %t87, label %collection_release_done_39, label %collection_release_38
collection_release_38:
  %t88 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 0
  %t89 = getelementptr inbounds %glitch.list, ptr %field_3, i32 0, i32 2
  %t90 = load i64, ptr %t88
  %t91 = load ptr, ptr %t89
  call void @glitch_free(ptr %t91)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_39
collection_release_done_39:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MapperConfigurationExpression__g0__t86(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t86, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t92 = icmp eq ptr %field_2, null
  br i1 %t92, label %collection_release_done_41, label %collection_release_40
collection_release_40:
  %t93 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t94 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t95 = load i64, ptr %t93
  %t96 = load ptr, ptr %t94
  %t97 = alloca i64
  store i64 0, ptr %t97
  br label %element_drop_loop_42
element_drop_loop_42:
  %t98 = load i64, ptr %t97
  %t99 = icmp ult i64 %t98, %t95
  br i1 %t99, label %element_drop_body_43, label %element_drop_done_44
element_drop_body_43:
  %t100 = getelementptr inbounds ptr, ptr %t96, i64 %t98
  %t101 = load ptr, ptr %t100
  call void @glitch_drop_Conduit_Features_Profiles_Profile__g0__t157(ptr %t101)
  %t102 = add i64 %t98, 1
  store i64 %t102, ptr %t97
  br label %element_drop_loop_42
element_drop_done_44:
  call void @glitch_free(ptr %t96)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_41
collection_release_done_41:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MapperConfigurationExpression__g0__t86(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t86, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MapperConfigurationExpression__g0__t86(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t86, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t86, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MapperConfigurationExpression__g0__t86(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_Edit__g0__t175(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_Edit__g0__t175(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Edit__g0__t175, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_Edit__g0__t175(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Edit__g0__t175, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Edit__g0__t175, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_Edit__g0__t175(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ControllerBase__g0__t78(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t78, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_HttpContext(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t78, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ModelStateDictionary(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ControllerBase__g0__t78(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t78, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ControllerBase__g0__t78(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t78, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ControllerBase__g0__t78, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ControllerBase__g0__t78(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IMapper__g0__t79(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IMapper__g0__t79(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IMapper__g0__t79, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IMapper__g0__t79(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IMapper__g0__t79, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IMapper__g0__t79, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IMapper__g0__t79(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Followers_Delete__g0__t149(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Followers_Delete__g0__t149(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Followers_Delete__g0__t149, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Followers_Delete__g0__t149(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Followers_Delete__g0__t149, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Followers_Delete__g0__t149, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Followers_Delete__g0__t149(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IDictionary__g2__t6(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IDictionary__g2__t6(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IDictionary__g2__t6, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IDictionary__g2__t6(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IDictionary__g2__t6, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IDictionary__g2__t6, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IDictionary__g2__t6(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_LoggingBuilder__g0__t55(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_LoggingBuilder__g0__t55(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.LoggingBuilder__g0__t55, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_LoggingBuilder__g0__t55(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.LoggingBuilder__g0__t55, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.LoggingBuilder__g0__t55, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_LoggingBuilder__g0__t55(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Uri__g0__t12(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Uri__g0__t12(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Uri__g0__t12, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Uri__g0__t12(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Uri__g0__t12, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Uri__g0__t12, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Uri__g0__t12(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Domain_ArticleTag__g0__t103(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_Article(ptr %field_3)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_Tag(ptr %field_5)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Domain_ArticleTag__g0__t103(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Domain_ArticleTag__g0__t103(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Domain_ArticleTag__g0__t103(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IPAddress__g0__t59(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IPAddress__g0__t59, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IPAddress__g0__t59(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IPAddress__g0__t59, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IPAddress__g0__t59(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IPAddress__g0__t59, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IPAddress__g0__t59, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IPAddress__g0__t59(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IEnumerable__g1__t0(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IEnumerable__g1__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerable__g1__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IEnumerable__g1__t0(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IEnumerable__g1__t0, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IEnumerable__g1__t0, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IEnumerable__g1__t0(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_FileStream__g0__t96(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_FileStream__g0__t96(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.FileStream__g0__t96, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_FileStream__g0__t96(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.FileStream__g0__t96, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.FileStream__g0__t96, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_FileStream__g0__t96(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IReadOnlyCollection__g1__t3(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IReadOnlyCollection__g1__t3(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyCollection__g1__t3, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IReadOnlyCollection__g1__t3(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyCollection__g1__t3, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IReadOnlyCollection__g1__t3, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IReadOnlyCollection__g1__t3(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ApiVersion__g0__t68(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ApiVersion__g0__t68(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ApiVersion__g0__t68, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ApiVersion__g0__t68(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ApiVersion__g0__t68, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ApiVersion__g0__t68, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ApiVersion__g0__t68(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_Login__g0__t176(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_Login__g0__t176(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Login__g0__t176, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_Login__g0__t176(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Login__g0__t176, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Login__g0__t176, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_Login__g0__t176(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_Command__g0__t168(ptr %object) {
entry:
  %field_7_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_UserData(ptr %field_7)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_Model(ptr %field_4)
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ArticleData(ptr %field_2)
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_string_release(ptr %field_6)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_Command__g0__t168(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_Command__g0__t168(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_Command__g0__t168(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_WebApplicationBuilder__g0__t56(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_ConfigurationManager(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ServiceCollection(ptr %field_2)
  %field_5_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_LoggingBuilder(ptr %field_5)
  %field_4_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_HostEnvironment(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_WebApplicationBuilder__g0__t56(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_WebApplicationBuilder__g0__t56(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_WebApplicationBuilder__g0__t56(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_bool__g0__t24(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_bool__g0__t24(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.bool__g0__t24, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_bool__g0__t24(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.bool__g0__t24, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.bool__g0__t24, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_bool__g0__t24(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_WebApplication__g0__t65(ptr %object) {
entry:
  %field_4_ptr = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t103 = icmp eq ptr %field_4, null
  br i1 %t103, label %collection_release_done_46, label %collection_release_45
collection_release_45:
  %t104 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 0
  %t105 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 2
  %t106 = load i64, ptr %t104
  %t107 = load ptr, ptr %t105
  %t108 = alloca i64
  store i64 0, ptr %t108
  br label %element_drop_loop_47
element_drop_loop_47:
  %t109 = load i64, ptr %t108
  %t110 = icmp ult i64 %t109, %t106
  br i1 %t110, label %element_drop_body_48, label %element_drop_done_49
element_drop_body_48:
  %t111 = getelementptr inbounds ptr, ptr %t107, i64 %t109
  %t112 = load ptr, ptr %t111
  call void @glitch_string_release(ptr %t112)
  %t113 = add i64 %t109, 1
  store i64 %t113, ptr %t108
  br label %element_drop_loop_47
element_drop_done_49:
  call void @glitch_free(ptr %t107)
  call void @glitch_free(ptr %field_4)
  br label %collection_release_done_46
collection_release_done_46:
  %field_2_ptr = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t114 = icmp eq ptr %field_2, null
  br i1 %t114, label %collection_release_done_51, label %collection_release_50
collection_release_50:
  %t115 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t116 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t117 = load i64, ptr %t115
  %t118 = load ptr, ptr %t116
  %t119 = alloca i64
  store i64 0, ptr %t119
  br label %element_drop_loop_52
element_drop_loop_52:
  %t120 = load i64, ptr %t119
  %t121 = icmp ult i64 %t120, %t117
  br i1 %t121, label %element_drop_body_53, label %element_drop_done_54
element_drop_body_53:
  %t122 = getelementptr inbounds ptr, ptr %t118, i64 %t120
  %t123 = load ptr, ptr %t122
  call void @glitch_string_release(ptr %t123)
  %t124 = add i64 %t120, 1
  store i64 %t124, ptr %t119
  br label %element_drop_loop_52
element_drop_done_54:
  call void @glitch_free(ptr %t118)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_51
collection_release_done_51:
  %field_5_ptr = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_ConfigurationManager(ptr %field_5)
  %field_6_ptr = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_ServiceProvider(ptr %field_6)
  %field_7_ptr = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_HostEnvironment(ptr %field_7)
  %field_3_ptr = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  %t125 = icmp eq ptr %field_3, null
  br i1 %t125, label %collection_release_done_56, label %collection_release_55
collection_release_55:
  %t126 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 0
  %t127 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 2
  %t128 = getelementptr inbounds %glitch.dict, ptr %field_3, i32 0, i32 3
  %t129 = load i64, ptr %t126
  %t130 = load ptr, ptr %t127
  %t131 = load ptr, ptr %t128
  %t132 = alloca i64
  store i64 0, ptr %t132
  br label %element_drop_loop_57
element_drop_loop_57:
  %t133 = load i64, ptr %t132
  %t134 = icmp ult i64 %t133, %t129
  br i1 %t134, label %element_drop_body_58, label %element_drop_done_59
element_drop_body_58:
  %t135 = getelementptr inbounds ptr, ptr %t130, i64 %t133
  %t136 = load ptr, ptr %t135
  call void @glitch_string_release(ptr %t136)
  %t137 = add i64 %t133, 1
  store i64 %t137, ptr %t132
  br label %element_drop_loop_57
element_drop_done_59:
  %t138 = alloca i64
  store i64 0, ptr %t138
  br label %element_drop_loop_60
element_drop_loop_60:
  %t139 = load i64, ptr %t138
  %t140 = icmp ult i64 %t139, %t129
  br i1 %t140, label %element_drop_body_61, label %element_drop_done_62
element_drop_body_61:
  %t141 = getelementptr inbounds ptr, ptr %t131, i64 %t139
  %t142 = load ptr, ptr %t141
  call void @glitch_string_release(ptr %t142)
  %t143 = add i64 %t139, 1
  store i64 %t143, ptr %t138
  br label %element_drop_loop_60
element_drop_done_62:
  call void @glitch_free(ptr %t130)
  call void @glitch_free(ptr %t131)
  call void @glitch_free(ptr %field_3)
  br label %collection_release_done_56
collection_release_done_56:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_WebApplication__g0__t65(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_WebApplication__g0__t65(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_WebApplication__g0__t65(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IndexBuilder__g0__t45(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IndexBuilder__g0__t45(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IndexBuilder__g0__t45, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IndexBuilder__g0__t45(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IndexBuilder__g0__t45, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IndexBuilder__g0__t45, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IndexBuilder__g0__t45(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ServiceCollection__g0__t52(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ServiceCollection__g0__t52(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ServiceCollection__g0__t52(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ServiceCollection__g0__t52(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbSet__g1__t31(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbSet__g1__t31(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbSet__g1__t31, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbSet__g1__t31(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbSet__g1__t31, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbSet__g1__t31, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbSet__g1__t31(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Enumerable__g0__t26(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Enumerable__g0__t26(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Enumerable__g0__t26, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Enumerable__g0__t26(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Enumerable__g0__t26, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Enumerable__g0__t26, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Enumerable__g0__t26(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_String__g0__t23(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_String__g0__t23(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.String__g0__t23, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_String__g0__t23(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.String__g0__t23, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.String__g0__t23, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_String__g0__t23(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_JwtBearerDefaults__g0__t71(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t71, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_JwtBearerDefaults__g0__t71(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t71, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_JwtBearerDefaults__g0__t71(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t71, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t71, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_JwtBearerDefaults__g0__t71(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Profiles_ProfilesController__g0__t160(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Profiles_ProfilesController__g0__t160(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfilesController__g0__t160, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Profiles_ProfilesController__g0__t160(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfilesController__g0__t160, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfilesController__g0__t160, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Profiles_ProfilesController__g0__t160(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ConduitContext(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HostEnvironment__g0__t54(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HostEnvironment__g0__t54(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HostEnvironment__g0__t54, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HostEnvironment__g0__t54(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HostEnvironment__g0__t54, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HostEnvironment__g0__t54, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HostEnvironment__g0__t54(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_12_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 12
  %field_12 = load ptr, ptr %field_12_ptr
  call void @glitch_drop_DbSet(ptr %field_12)
  %field_7_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_ChangeTracker(ptr %field_7)
  %field_10_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 10
  %field_10 = load ptr, ptr %field_10_ptr
  call void @glitch_drop_DbSet(ptr %field_10)
  %field_11_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 11
  %field_11 = load ptr, ptr %field_11_ptr
  call void @glitch_drop_DbSet(ptr %field_11)
  %field_15_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 15
  %field_15 = load ptr, ptr %field_15_ptr
  call void @glitch_drop_DbSet(ptr %field_15)
  %field_14_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 14
  %field_14 = load ptr, ptr %field_14_ptr
  call void @glitch_drop_DbSet(ptr %field_14)
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_DatabaseFacade(ptr %field_6)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t144 = icmp eq ptr %field_4, null
  br i1 %t144, label %collection_release_done_64, label %collection_release_63
collection_release_63:
  %t145 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 0
  %t146 = getelementptr inbounds %glitch.list, ptr %field_4, i32 0, i32 2
  %t147 = load i64, ptr %t145
  %t148 = load ptr, ptr %t146
  %t149 = alloca i64
  store i64 0, ptr %t149
  br label %element_drop_loop_65
element_drop_loop_65:
  %t150 = load i64, ptr %t149
  %t151 = icmp ult i64 %t150, %t147
  br i1 %t151, label %element_drop_body_66, label %element_drop_done_67
element_drop_body_66:
  %t152 = getelementptr inbounds ptr, ptr %t148, i64 %t150
  %t153 = load ptr, ptr %t152
  call void @glitch_string_release(ptr %t153)
  %t154 = add i64 %t150, 1
  store i64 %t154, ptr %t149
  br label %element_drop_loop_65
element_drop_done_67:
  call void @glitch_free(ptr %t148)
  call void @glitch_free(ptr %field_4)
  br label %collection_release_done_64
collection_release_done_64:
  %field_8_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 8
  %field_8 = load ptr, ptr %field_8_ptr
  call void @glitch_drop_DbContextOptions(ptr %field_8)
  %field_16_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 16
  %field_16 = load ptr, ptr %field_16_ptr
  call void @glitch_drop_DbSet(ptr %field_16)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_SqlProvider(ptr %field_5)
  %field_13_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 13
  %field_13 = load ptr, ptr %field_13_ptr
  call void @glitch_drop_DbSet(ptr %field_13)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ValueTask__g0__t28(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ValueTask__g0__t28(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ValueTask__g0__t28, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ValueTask__g0__t28(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ValueTask__g0__t28, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ValueTask__g0__t28, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ValueTask__g0__t28(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbContextOptionsBuilder_ApplicationDbContext__g0__t32(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbContextOptionsBuilder_ApplicationDbContext__g0__t32(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder_ApplicationDbContext__g0__t32, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbContextOptionsBuilder_ApplicationDbContext__g0__t32(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder_ApplicationDbContext__g0__t32, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbContextOptionsBuilder_ApplicationDbContext__g0__t32, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbContextOptionsBuilder_ApplicationDbContext__g0__t32(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_Handler__g0__t170(ptr %object) {
entry:
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_IMapper(ptr %field_6)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_IPasswordHasher(ptr %field_4)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_IJwtTokenGenerator(ptr %field_5)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_ICurrentUserAccessor(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ConduitContext(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_Handler__g0__t170(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_Handler__g0__t170(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_Handler__g0__t170(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ILogger__g0__t89(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ILogger__g0__t89(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ILogger__g0__t89, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ILogger__g0__t89(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ILogger__g0__t89, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ILogger__g0__t89, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ILogger__g0__t89(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Comments_CommentsEnvelope__g0__t128(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsEnvelope__g0__t128, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t155 = icmp eq ptr %field_2, null
  br i1 %t155, label %collection_release_done_69, label %collection_release_68
collection_release_68:
  %t156 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t157 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t158 = load i64, ptr %t156
  %t159 = load ptr, ptr %t157
  %t160 = alloca i64
  store i64 0, ptr %t160
  br label %element_drop_loop_70
element_drop_loop_70:
  %t161 = load i64, ptr %t160
  %t162 = icmp ult i64 %t161, %t158
  br i1 %t162, label %element_drop_body_71, label %element_drop_done_72
element_drop_body_71:
  %t163 = getelementptr inbounds ptr, ptr %t159, i64 %t161
  %t164 = load ptr, ptr %t163
  call void @glitch_drop_Conduit_Domain_Comment__g0__t104(ptr %t164)
  %t165 = add i64 %t161, 1
  store i64 %t165, ptr %t160
  br label %element_drop_loop_70
element_drop_done_72:
  call void @glitch_free(ptr %t159)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_69
collection_release_done_69:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Comments_CommentsEnvelope__g0__t128(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsEnvelope__g0__t128, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Comments_CommentsEnvelope__g0__t128(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsEnvelope__g0__t128, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsEnvelope__g0__t128, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Comments_CommentsEnvelope__g0__t128(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbContextOptions__g1__t29(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.DbContextOptions__g1__t29, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbContextOptions__g1__t29(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptions__g1__t29, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbContextOptions__g1__t29(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbContextOptions__g1__t29, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbContextOptions__g1__t29, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbContextOptions__g1__t29(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_File__g0__t97(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_File__g0__t97(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.File__g0__t97, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_File__g0__t97(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.File__g0__t97, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.File__g0__t97, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_File__g0__t97(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_Security_IPasswordHasher__g0__t191(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_Security_IPasswordHasher__g0__t191(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_IPasswordHasher__g0__t191, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_Security_IPasswordHasher__g0__t191(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_IPasswordHasher__g0__t191, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_IPasswordHasher__g0__t191, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_Security_IPasswordHasher__g0__t191(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_Create__g0__t166(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_Create__g0__t166(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Create__g0__t166, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_Create__g0__t166(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Create__g0__t166, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_Create__g0__t166, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_Create__g0__t166(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Domain_Person__g0__t106(ptr %object) {
entry:
  %field_11_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 11
  %field_11 = load ptr, ptr %field_11_ptr
  %t166 = icmp eq ptr %field_11, null
  br i1 %t166, label %array_release_done_74, label %array_release_73
array_release_73:
  %t167 = getelementptr inbounds %glitch.array, ptr %field_11, i32 0, i32 0
  %t169 = getelementptr inbounds %glitch.array, ptr %field_11, i32 0, i32 1
  %t168 = load i64, ptr %t167
  %t170 = load ptr, ptr %t169
  call void @glitch_free(ptr %t170)
  call void @glitch_free(ptr %field_11)
  br label %array_release_done_74
array_release_done_74:
  %field_9_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 9
  %field_9 = load ptr, ptr %field_9_ptr
  %t171 = icmp eq ptr %field_9, null
  br i1 %t171, label %collection_release_done_76, label %collection_release_75
collection_release_75:
  %t172 = getelementptr inbounds %glitch.list, ptr %field_9, i32 0, i32 0
  %t173 = getelementptr inbounds %glitch.list, ptr %field_9, i32 0, i32 2
  %t174 = load i64, ptr %t172
  %t175 = load ptr, ptr %t173
  %t176 = alloca i64
  store i64 0, ptr %t176
  br label %element_drop_loop_77
element_drop_loop_77:
  %t177 = load i64, ptr %t176
  %t178 = icmp ult i64 %t177, %t174
  br i1 %t178, label %element_drop_body_78, label %element_drop_done_79
element_drop_body_78:
  %t179 = getelementptr inbounds ptr, ptr %t175, i64 %t177
  %t180 = load ptr, ptr %t179
  call void @glitch_drop_Conduit_Domain_FollowedPeople__g0__t105(ptr %t180)
  %t181 = add i64 %t177, 1
  store i64 %t181, ptr %t176
  br label %element_drop_loop_77
element_drop_done_79:
  call void @glitch_free(ptr %t175)
  call void @glitch_free(ptr %field_9)
  br label %collection_release_done_76
collection_release_done_76:
  %field_7_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  %t182 = icmp eq ptr %field_7, null
  br i1 %t182, label %collection_release_done_81, label %collection_release_80
collection_release_80:
  %t183 = getelementptr inbounds %glitch.list, ptr %field_7, i32 0, i32 0
  %t184 = getelementptr inbounds %glitch.list, ptr %field_7, i32 0, i32 2
  %t185 = load i64, ptr %t183
  %t186 = load ptr, ptr %t184
  %t187 = alloca i64
  store i64 0, ptr %t187
  br label %element_drop_loop_82
element_drop_loop_82:
  %t188 = load i64, ptr %t187
  %t189 = icmp ult i64 %t188, %t185
  br i1 %t189, label %element_drop_body_83, label %element_drop_done_84
element_drop_body_83:
  %t190 = getelementptr inbounds ptr, ptr %t186, i64 %t188
  %t191 = load ptr, ptr %t190
  call void @glitch_drop_Conduit_Domain_ArticleFavorite__g0__t102(ptr %t191)
  %t192 = add i64 %t188, 1
  store i64 %t192, ptr %t187
  br label %element_drop_loop_82
element_drop_done_84:
  call void @glitch_free(ptr %t186)
  call void @glitch_free(ptr %field_7)
  br label %collection_release_done_81
collection_release_done_81:
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_10_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 10
  %field_10 = load ptr, ptr %field_10_ptr
  %t193 = icmp eq ptr %field_10, null
  br i1 %t193, label %array_release_done_86, label %array_release_85
array_release_85:
  %t194 = getelementptr inbounds %glitch.array, ptr %field_10, i32 0, i32 0
  %t196 = getelementptr inbounds %glitch.array, ptr %field_10, i32 0, i32 1
  %t195 = load i64, ptr %t194
  %t197 = load ptr, ptr %t196
  call void @glitch_free(ptr %t197)
  call void @glitch_free(ptr %field_10)
  br label %array_release_done_86
array_release_done_86:
  %field_8_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 8
  %field_8 = load ptr, ptr %field_8_ptr
  %t198 = icmp eq ptr %field_8, null
  br i1 %t198, label %collection_release_done_88, label %collection_release_87
collection_release_87:
  %t199 = getelementptr inbounds %glitch.list, ptr %field_8, i32 0, i32 0
  %t200 = getelementptr inbounds %glitch.list, ptr %field_8, i32 0, i32 2
  %t201 = load i64, ptr %t199
  %t202 = load ptr, ptr %t200
  %t203 = alloca i64
  store i64 0, ptr %t203
  br label %element_drop_loop_89
element_drop_loop_89:
  %t204 = load i64, ptr %t203
  %t205 = icmp ult i64 %t204, %t201
  br i1 %t205, label %element_drop_body_90, label %element_drop_done_91
element_drop_body_90:
  %t206 = getelementptr inbounds ptr, ptr %t202, i64 %t204
  %t207 = load ptr, ptr %t206
  call void @glitch_drop_Conduit_Domain_FollowedPeople__g0__t105(ptr %t207)
  %t208 = add i64 %t204, 1
  store i64 %t208, ptr %t203
  br label %element_drop_loop_89
element_drop_done_91:
  call void @glitch_free(ptr %t202)
  call void @glitch_free(ptr %field_8)
  br label %collection_release_done_88
collection_release_done_88:
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_string_release(ptr %field_6)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Domain_Person__g0__t106(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Domain_Person__g0__t106(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Domain_Person__g0__t106(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MigrationBuilder__g0__t42(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MigrationBuilder__g0__t42(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MigrationBuilder__g0__t42, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MigrationBuilder__g0__t42(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MigrationBuilder__g0__t42, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MigrationBuilder__g0__t42, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MigrationBuilder__g0__t42(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Articles_ArticleEnvelope__g0__t108(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_Article(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Articles_ArticleEnvelope__g0__t108(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Articles_ArticleEnvelope__g0__t108(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Articles_ArticleEnvelope__g0__t108(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_int__g0__t25(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_int__g0__t25(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.int__g0__t25, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_int__g0__t25(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.int__g0__t25, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.int__g0__t25, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_int__g0__t25(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_UserData__g0__t167(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_string_release(ptr %field_6)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_UserData__g0__t167(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_UserData__g0__t167(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_UserData__g0__t167(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MvcBuilder__g0__t50(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MvcBuilder__g0__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MvcBuilder__g0__t50(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MvcBuilder__g0__t50(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ICurrentUserAccessor__g0__t189, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ICurrentUserAccessor__g0__t189, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ICurrentUserAccessor__g0__t189, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_SqlColumn__g0__t98(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.SqlColumn__g0__t98, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_4_ptr = getelementptr inbounds %glitch.SqlColumn__g0__t98, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SqlColumn__g0__t98(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SqlColumn__g0__t98, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SqlColumn__g0__t98(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SqlColumn__g0__t98, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SqlColumn__g0__t98, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SqlColumn__g0__t98(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_QueryValidator__g0__t173(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_QueryValidator__g0__t173(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryValidator__g0__t173, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_QueryValidator__g0__t173(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryValidator__g0__t173, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryValidator__g0__t173, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_QueryValidator__g0__t173(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IHttpContextAccessor__g0__t62(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IHttpContextAccessor__g0__t62, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_HttpContext(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IHttpContextAccessor__g0__t62(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IHttpContextAccessor__g0__t62, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IHttpContextAccessor__g0__t62(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IHttpContextAccessor__g0__t62, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IHttpContextAccessor__g0__t62, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IHttpContextAccessor__g0__t62(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IReadOnlyDictionary__g2__t7(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IReadOnlyDictionary__g2__t7(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyDictionary__g2__t7, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IReadOnlyDictionary__g2__t7(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyDictionary__g2__t7, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IReadOnlyDictionary__g2__t7, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IReadOnlyDictionary__g2__t7(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DbUpdateConcurrencyException__g0__t37(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.DbUpdateConcurrencyException__g0__t37, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DbUpdateConcurrencyException__g0__t37(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DbUpdateConcurrencyException__g0__t37, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DbUpdateConcurrencyException__g0__t37(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DbUpdateConcurrencyException__g0__t37, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DbUpdateConcurrencyException__g0__t37, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DbUpdateConcurrencyException__g0__t37(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Mapper__g0__t80(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Mapper__g0__t80(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Mapper__g0__t80, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Mapper__g0__t80(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Mapper__g0__t80, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Mapper__g0__t80, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Mapper__g0__t80(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IReadOnlyList__g1__t5(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IReadOnlyList__g1__t5(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyList__g1__t5, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IReadOnlyList__g1__t5(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IReadOnlyList__g1__t5, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IReadOnlyList__g1__t5, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IReadOnlyList__g1__t5(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_EntityEntry__g0__t36(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_EntityEntry__g0__t36(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.EntityEntry__g0__t36, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_EntityEntry__g0__t36(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.EntityEntry__g0__t36, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.EntityEntry__g0__t36, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_EntityEntry__g0__t36(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Profiles_Profile__g0__t157(ptr %object) {
entry:
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_Profile__g0__t157, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_Profile__g0__t157, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_Profile__g0__t157, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Profiles_Profile__g0__t157(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_Profile__g0__t157, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Profiles_Profile__g0__t157(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_Profile__g0__t157, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_Profile__g0__t157, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Profiles_Profile__g0__t157(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ILoggerFactory__g0__t91(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ILoggerFactory__g0__t91(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ILoggerFactory__g0__t91, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ILoggerFactory__g0__t91(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ILoggerFactory__g0__t91, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ILoggerFactory__g0__t91, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ILoggerFactory__g0__t91(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_HttpRequest__g0__t57(ptr %object) {
entry:
  %field_5_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  %field_2_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_4_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  %t209 = icmp eq ptr %field_4, null
  br i1 %t209, label %collection_release_done_93, label %collection_release_92
collection_release_92:
  %t210 = getelementptr inbounds %glitch.dict, ptr %field_4, i32 0, i32 0
  %t211 = getelementptr inbounds %glitch.dict, ptr %field_4, i32 0, i32 2
  %t212 = getelementptr inbounds %glitch.dict, ptr %field_4, i32 0, i32 3
  %t213 = load i64, ptr %t210
  %t214 = load ptr, ptr %t211
  %t215 = load ptr, ptr %t212
  %t216 = alloca i64
  store i64 0, ptr %t216
  br label %element_drop_loop_94
element_drop_loop_94:
  %t217 = load i64, ptr %t216
  %t218 = icmp ult i64 %t217, %t213
  br i1 %t218, label %element_drop_body_95, label %element_drop_done_96
element_drop_body_95:
  %t219 = getelementptr inbounds ptr, ptr %t214, i64 %t217
  %t220 = load ptr, ptr %t219
  call void @glitch_string_release(ptr %t220)
  %t221 = add i64 %t217, 1
  store i64 %t221, ptr %t216
  br label %element_drop_loop_94
element_drop_done_96:
  %t222 = alloca i64
  store i64 0, ptr %t222
  br label %element_drop_loop_97
element_drop_loop_97:
  %t223 = load i64, ptr %t222
  %t224 = icmp ult i64 %t223, %t213
  br i1 %t224, label %element_drop_body_98, label %element_drop_done_99
element_drop_body_98:
  %t225 = getelementptr inbounds ptr, ptr %t215, i64 %t223
  %t226 = load ptr, ptr %t225
  call void @glitch_string_release(ptr %t226)
  %t227 = add i64 %t223, 1
  store i64 %t227, ptr %t222
  br label %element_drop_loop_97
element_drop_done_99:
  call void @glitch_free(ptr %t214)
  call void @glitch_free(ptr %t215)
  call void @glitch_free(ptr %field_4)
  br label %collection_release_done_93
collection_release_done_93:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_HttpRequest__g0__t57(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_HttpRequest__g0__t57(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_HttpRequest__g0__t57(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ProblemDetails__g0__t69(ptr %object) {
entry:
  %field_6_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t69, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  %t228 = icmp eq ptr %field_6, null
  br i1 %t228, label %collection_release_done_101, label %collection_release_100
collection_release_100:
  %t229 = getelementptr inbounds %glitch.dict, ptr %field_6, i32 0, i32 0
  %t230 = getelementptr inbounds %glitch.dict, ptr %field_6, i32 0, i32 2
  %t231 = getelementptr inbounds %glitch.dict, ptr %field_6, i32 0, i32 3
  %t232 = load i64, ptr %t229
  %t233 = load ptr, ptr %t230
  %t234 = load ptr, ptr %t231
  %t235 = alloca i64
  store i64 0, ptr %t235
  br label %element_drop_loop_102
element_drop_loop_102:
  %t236 = load i64, ptr %t235
  %t237 = icmp ult i64 %t236, %t232
  br i1 %t237, label %element_drop_body_103, label %element_drop_done_104
element_drop_body_103:
  %t238 = getelementptr inbounds ptr, ptr %t233, i64 %t236
  %t239 = load ptr, ptr %t238
  call void @glitch_string_release(ptr %t239)
  %t240 = add i64 %t236, 1
  store i64 %t240, ptr %t235
  br label %element_drop_loop_102
element_drop_done_104:
  call void @glitch_free(ptr %t233)
  call void @glitch_free(ptr %t234)
  call void @glitch_free(ptr %field_6)
  br label %collection_release_done_101
collection_release_done_101:
  %field_4_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t69, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_5_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t69, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  %field_2_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t69, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ProblemDetails__g0__t69(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t69, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ProblemDetails__g0__t69(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t69, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ProblemDetails__g0__t69, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ProblemDetails__g0__t69(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ColumnOptions__g0__t99(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ColumnOptions__g0__t99, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t241 = icmp eq ptr %field_2, null
  br i1 %t241, label %collection_release_done_106, label %collection_release_105
collection_release_105:
  %t242 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t243 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t244 = load i64, ptr %t242
  %t245 = load ptr, ptr %t243
  %t246 = alloca i64
  store i64 0, ptr %t246
  br label %element_drop_loop_107
element_drop_loop_107:
  %t247 = load i64, ptr %t246
  %t248 = icmp ult i64 %t247, %t244
  br i1 %t248, label %element_drop_body_108, label %element_drop_done_109
element_drop_body_108:
  %t249 = getelementptr inbounds ptr, ptr %t245, i64 %t247
  %t250 = load ptr, ptr %t249
  call void @glitch_drop_SqlColumn__g0__t98(ptr %t250)
  %t251 = add i64 %t247, 1
  store i64 %t251, ptr %t246
  br label %element_drop_loop_107
element_drop_done_109:
  call void @glitch_free(ptr %t245)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_106
collection_release_done_106:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ColumnOptions__g0__t99(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ColumnOptions__g0__t99, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ColumnOptions__g0__t99(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ColumnOptions__g0__t99, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ColumnOptions__g0__t99, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ColumnOptions__g0__t99(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Favorites_FavoritesController__g0__t144(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Favorites_FavoritesController__g0__t144(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Favorites_FavoritesController__g0__t144, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Favorites_FavoritesController__g0__t144(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Favorites_FavoritesController__g0__t144, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Favorites_FavoritesController__g0__t144, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Favorites_FavoritesController__g0__t144(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_TimeSpan(ptr %field_6)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_TimeSpan__g0__t18(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_TimeSpan__g0__t18(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.TimeSpan__g0__t18, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_TimeSpan__g0__t18(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.TimeSpan__g0__t18, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.TimeSpan__g0__t18, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_TimeSpan__g0__t18(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IMemberConfigurationExpression__g0__t81(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IMemberConfigurationExpression__g0__t81(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IMemberConfigurationExpression__g0__t81, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IMemberConfigurationExpression__g0__t81(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IMemberConfigurationExpression__g0__t81, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IMemberConfigurationExpression__g0__t81, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IMemberConfigurationExpression__g0__t81(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Task__g0__t27(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Task__g0__t27(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Task__g0__t27, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Task__g0__t27(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Task__g0__t27, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Task__g0__t27, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Task__g0__t27(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Profiles_ProfileEnvelope__g0__t158(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileEnvelope__g0__t158, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_Profile(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Profiles_ProfileEnvelope__g0__t158(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileEnvelope__g0__t158, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Profiles_ProfileEnvelope__g0__t158(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileEnvelope__g0__t158, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileEnvelope__g0__t158, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Profiles_ProfileEnvelope__g0__t158(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_EntityTypeBuilder__g1__t43(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_EntityTypeBuilder__g1__t43(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.EntityTypeBuilder__g1__t43, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_EntityTypeBuilder__g1__t43(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.EntityTypeBuilder__g1__t43, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.EntityTypeBuilder__g1__t43, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_EntityTypeBuilder__g1__t43(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_JsonSerializer__g0__t88(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_JsonSerializer__g0__t88(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.JsonSerializer__g0__t88, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_JsonSerializer__g0__t88(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.JsonSerializer__g0__t88, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.JsonSerializer__g0__t88, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_JsonSerializer__g0__t88(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_UserController__g0__t180(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserController__g0__t180, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_ICurrentUserAccessor(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_UserController__g0__t180(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserController__g0__t180, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_UserController__g0__t180(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserController__g0__t180, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_UserController__g0__t180, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_UserController__g0__t180(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_AppContext__g0__t11(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.AppContext__g0__t11, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_AppContext__g0__t11(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.AppContext__g0__t11, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_AppContext__g0__t11(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.AppContext__g0__t11, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.AppContext__g0__t11, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_AppContext__g0__t11(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_Errors_Constants__g0__t185(ptr %object) {
entry:
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_Errors_Constants__g0__t185(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_Errors_Constants__g0__t185(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_Errors_Constants__g0__t185(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_Security_PasswordHasher__g0__t194(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_Security_PasswordHasher__g0__t194(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_PasswordHasher__g0__t194, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_Security_PasswordHasher__g0__t194(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_PasswordHasher__g0__t194, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_PasswordHasher__g0__t194, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_Security_PasswordHasher__g0__t194(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Infrastructure_ValidatorActionFilter__g0__t197(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Infrastructure_ValidatorActionFilter__g0__t197(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ValidatorActionFilter__g0__t197, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Infrastructure_ValidatorActionFilter__g0__t197(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ValidatorActionFilter__g0__t197, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Infrastructure_ValidatorActionFilter__g0__t197, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Infrastructure_ValidatorActionFilter__g0__t197(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_PhysicalFileProvider__g0__t67(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_PhysicalFileProvider__g0__t67(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.PhysicalFileProvider__g0__t67, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_PhysicalFileProvider__g0__t67(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.PhysicalFileProvider__g0__t67, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.PhysicalFileProvider__g0__t67, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_PhysicalFileProvider__g0__t67(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_User__g0__t178(ptr %object) {
entry:
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_User__g0__t178, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_string_release(ptr %field_6)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_User__g0__t178, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_User__g0__t178, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_User__g0__t178, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_User__g0__t178, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_User__g0__t178(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_User__g0__t178, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_User__g0__t178(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_User__g0__t178, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_User__g0__t178, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_User__g0__t178(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IServiceCollection__g0__t51(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IServiceCollection__g0__t51(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IServiceCollection__g0__t51, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IServiceCollection__g0__t51(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IServiceCollection__g0__t51, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IServiceCollection__g0__t51, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IServiceCollection__g0__t51(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IServiceScope__g0__t48(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IServiceScope__g0__t48, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_IServiceProvider(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IServiceScope__g0__t48(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IServiceScope__g0__t48, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IServiceScope__g0__t48(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IServiceScope__g0__t48, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IServiceScope__g0__t48, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IServiceScope__g0__t48(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_LoggerFactory__g0__t93(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_LoggerFactory__g0__t93(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.LoggerFactory__g0__t93, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_LoggerFactory__g0__t93(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.LoggerFactory__g0__t93, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.LoggerFactory__g0__t93, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_LoggerFactory__g0__t93(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ICollection__g1__t2(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ICollection__g1__t2(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ICollection__g1__t2, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ICollection__g1__t2(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ICollection__g1__t2, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ICollection__g1__t2, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ICollection__g1__t2(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IMappingExpression__g2__t83(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IMappingExpression__g2__t83(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IMappingExpression__g2__t83, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IMappingExpression__g2__t83(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IMappingExpression__g2__t83, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IMappingExpression__g2__t83, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IMappingExpression__g2__t83(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ConsoleLogger__g0__t90(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ConsoleLogger__g0__t90(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ConsoleLogger__g0__t90, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ConsoleLogger__g0__t90(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ConsoleLogger__g0__t90, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ConsoleLogger__g0__t90, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ConsoleLogger__g0__t90(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Path__g0__t94(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Path__g0__t94(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Path__g0__t94, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Path__g0__t94(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Path__g0__t94, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Path__g0__t94, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Path__g0__t94(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Domain_Article__g0__t101(ptr %object) {
entry:
  %field_7_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_drop_Person(ptr %field_7)
  %field_10_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 10
  %field_10 = load ptr, ptr %field_10_ptr
  %t252 = icmp eq ptr %field_10, null
  br i1 %t252, label %collection_release_done_111, label %collection_release_110
collection_release_110:
  %t253 = getelementptr inbounds %glitch.list, ptr %field_10, i32 0, i32 0
  %t254 = getelementptr inbounds %glitch.list, ptr %field_10, i32 0, i32 2
  %t255 = load i64, ptr %t253
  %t256 = load ptr, ptr %t254
  %t257 = alloca i64
  store i64 0, ptr %t257
  br label %element_drop_loop_112
element_drop_loop_112:
  %t258 = load i64, ptr %t257
  %t259 = icmp ult i64 %t258, %t255
  br i1 %t259, label %element_drop_body_113, label %element_drop_done_114
element_drop_body_113:
  %t260 = getelementptr inbounds ptr, ptr %t256, i64 %t258
  %t261 = load ptr, ptr %t260
  call void @glitch_drop_Conduit_Domain_ArticleFavorite__g0__t102(ptr %t261)
  %t262 = add i64 %t258, 1
  store i64 %t262, ptr %t257
  br label %element_drop_loop_112
element_drop_done_114:
  call void @glitch_free(ptr %t256)
  call void @glitch_free(ptr %field_10)
  br label %collection_release_done_111
collection_release_done_111:
  %field_12_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 12
  %field_12 = load ptr, ptr %field_12_ptr
  call void @glitch_string_release(ptr %field_12)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_string_release(ptr %field_4)
  %field_8_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 8
  %field_8 = load ptr, ptr %field_8_ptr
  %t263 = icmp eq ptr %field_8, null
  br i1 %t263, label %collection_release_done_116, label %collection_release_115
collection_release_115:
  %t264 = getelementptr inbounds %glitch.list, ptr %field_8, i32 0, i32 0
  %t265 = getelementptr inbounds %glitch.list, ptr %field_8, i32 0, i32 2
  %t266 = load i64, ptr %t264
  %t267 = load ptr, ptr %t265
  %t268 = alloca i64
  store i64 0, ptr %t268
  br label %element_drop_loop_117
element_drop_loop_117:
  %t269 = load i64, ptr %t268
  %t270 = icmp ult i64 %t269, %t266
  br i1 %t270, label %element_drop_body_118, label %element_drop_done_119
element_drop_body_118:
  %t271 = getelementptr inbounds ptr, ptr %t267, i64 %t269
  %t272 = load ptr, ptr %t271
  call void @glitch_drop_Conduit_Domain_Comment__g0__t104(ptr %t272)
  %t273 = add i64 %t269, 1
  store i64 %t273, ptr %t268
  br label %element_drop_loop_117
element_drop_done_119:
  call void @glitch_free(ptr %t267)
  call void @glitch_free(ptr %field_8)
  br label %collection_release_done_116
collection_release_done_116:
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_string_release(ptr %field_5)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_string_release(ptr %field_6)
  %field_9_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 9
  %field_9 = load ptr, ptr %field_9_ptr
  %t274 = icmp eq ptr %field_9, null
  br i1 %t274, label %collection_release_done_121, label %collection_release_120
collection_release_120:
  %t275 = getelementptr inbounds %glitch.list, ptr %field_9, i32 0, i32 0
  %t276 = getelementptr inbounds %glitch.list, ptr %field_9, i32 0, i32 2
  %t277 = load i64, ptr %t275
  %t278 = load ptr, ptr %t276
  %t279 = alloca i64
  store i64 0, ptr %t279
  br label %element_drop_loop_122
element_drop_loop_122:
  %t280 = load i64, ptr %t279
  %t281 = icmp ult i64 %t280, %t277
  br i1 %t281, label %element_drop_body_123, label %element_drop_done_124
element_drop_body_123:
  %t282 = getelementptr inbounds ptr, ptr %t278, i64 %t280
  %t283 = load ptr, ptr %t282
  call void @glitch_drop_Conduit_Domain_ArticleTag__g0__t103(ptr %t283)
  %t284 = add i64 %t280, 1
  store i64 %t284, ptr %t279
  br label %element_drop_loop_122
element_drop_done_124:
  call void @glitch_free(ptr %t278)
  call void @glitch_free(ptr %field_9)
  br label %collection_release_done_121
collection_release_done_121:
  %field_11_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 11
  %field_11 = load ptr, ptr %field_11_ptr
  call void @glitch_string_release(ptr %field_11)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Domain_Article__g0__t101(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Domain_Article__g0__t101(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Domain_Article__g0__t101(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Domain_Comment__g0__t104(ptr %object) {
entry:
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_Article(ptr %field_6)
  %field_9_ptr = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %object, i32 0, i32 9
  %field_9 = load ptr, ptr %field_9_ptr
  call void @glitch_string_release(ptr %field_9)
  %field_8_ptr = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %object, i32 0, i32 8
  %field_8 = load ptr, ptr %field_8_ptr
  call void @glitch_string_release(ptr %field_8)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_Person(ptr %field_4)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Domain_Comment__g0__t104(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Domain_Comment__g0__t104(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Domain_Comment__g0__t104(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_StaticFileOptions__g0__t66(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.StaticFileOptions__g0__t66, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_StaticFileOptions__g0__t66(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.StaticFileOptions__g0__t66, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_StaticFileOptions__g0__t66(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.StaticFileOptions__g0__t66, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.StaticFileOptions__g0__t66, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_StaticFileOptions__g0__t66(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MemberConfigurationExpression__g0__t82(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MemberConfigurationExpression__g0__t82(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MemberConfigurationExpression__g0__t82, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MemberConfigurationExpression__g0__t82(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MemberConfigurationExpression__g0__t82, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MemberConfigurationExpression__g0__t82, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MemberConfigurationExpression__g0__t82(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_FormatException__g0__t16(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.FormatException__g0__t16, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_FormatException__g0__t16(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.FormatException__g0__t16, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_FormatException__g0__t16(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.FormatException__g0__t16, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.FormatException__g0__t16, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_FormatException__g0__t16(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IServiceProvider__g0__t46(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IServiceProvider__g0__t46(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IServiceProvider__g0__t46, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IServiceProvider__g0__t46(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IServiceProvider__g0__t46, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IServiceProvider__g0__t46, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IServiceProvider__g0__t46(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Profiles_ProfileReader__g0__t159(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ConduitContext(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_ICurrentUserAccessor(ptr %field_3)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_IMapper(ptr %field_4)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Profiles_ProfileReader__g0__t159(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Profiles_ProfileReader__g0__t159(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Profiles_ProfileReader__g0__t159(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ChangeTracker__g0__t33(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ChangeTracker__g0__t33, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_DebugView(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ChangeTracker__g0__t33(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ChangeTracker__g0__t33, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ChangeTracker__g0__t33(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ChangeTracker__g0__t33, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ChangeTracker__g0__t33, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ChangeTracker__g0__t33(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Users_QueryHandler__g0__t174(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_drop_ConduitContext(ptr %field_2)
  %field_5_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %object, i32 0, i32 5
  %field_5 = load ptr, ptr %field_5_ptr
  call void @glitch_drop_IJwtTokenGenerator(ptr %field_5)
  %field_3_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_ICurrentUserAccessor(ptr %field_3)
  %field_4_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %object, i32 0, i32 4
  %field_4 = load ptr, ptr %field_4_ptr
  call void @glitch_drop_IProfileReader(ptr %field_4)
  %field_6_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_IMapper(ptr %field_6)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Users_QueryHandler__g0__t174(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Users_QueryHandler__g0__t174(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Users_QueryHandler__g0__t174(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Type__g0__t20(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Type__g0__t20, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t285 = icmp eq ptr %field_2, null
  br i1 %t285, label %collection_release_done_126, label %collection_release_125
collection_release_125:
  %t286 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t287 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t288 = load i64, ptr %t286
  %t289 = load ptr, ptr %t287
  %t290 = alloca i64
  store i64 0, ptr %t290
  br label %element_drop_loop_127
element_drop_loop_127:
  %t291 = load i64, ptr %t290
  %t292 = icmp ult i64 %t291, %t288
  br i1 %t292, label %element_drop_body_128, label %element_drop_done_129
element_drop_body_128:
  %t293 = getelementptr inbounds ptr, ptr %t289, i64 %t291
  %t294 = load ptr, ptr %t293
  call void @glitch_drop_Type__g0__t20(ptr %t294)
  %t295 = add i64 %t291, 1
  store i64 %t295, ptr %t290
  br label %element_drop_loop_127
element_drop_done_129:
  call void @glitch_free(ptr %t289)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_126
collection_release_done_126:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Type__g0__t20(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Type__g0__t20, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Type__g0__t20(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Type__g0__t20, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Type__g0__t20, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Type__g0__t20(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Followers_Add__g0__t145(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Followers_Add__g0__t145(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Followers_Add__g0__t145, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Followers_Add__g0__t145(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Followers_Add__g0__t145, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Followers_Add__g0__t145, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Followers_Add__g0__t145(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ModelBuilder__g0__t41(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ModelBuilder__g0__t41(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ModelBuilder__g0__t41, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ModelBuilder__g0__t41(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ModelBuilder__g0__t41, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ModelBuilder__g0__t41, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ModelBuilder__g0__t41(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_DebugView__g0__t34(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.DebugView__g0__t34, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_DebugView__g0__t34(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.DebugView__g0__t34, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_DebugView__g0__t34(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.DebugView__g0__t34, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.DebugView__g0__t34, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_DebugView__g0__t34(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_SqlProvider__g0__t30(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_SqlProvider__g0__t30(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_SqlProvider__g0__t30(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_SqlProvider__g0__t30(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_MSSqlServerSinkOptions__g0__t100(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.MSSqlServerSinkOptions__g0__t100, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  %field_6_ptr = getelementptr inbounds %glitch.MSSqlServerSinkOptions__g0__t100, ptr %object, i32 0, i32 6
  %field_6 = load ptr, ptr %field_6_ptr
  call void @glitch_drop_TimeSpan(ptr %field_6)
  %field_7_ptr = getelementptr inbounds %glitch.MSSqlServerSinkOptions__g0__t100, ptr %object, i32 0, i32 7
  %field_7 = load ptr, ptr %field_7_ptr
  call void @glitch_string_release(ptr %field_7)
  %field_8_ptr = getelementptr inbounds %glitch.MSSqlServerSinkOptions__g0__t100, ptr %object, i32 0, i32 8
  %field_8 = load ptr, ptr %field_8_ptr
  call void @glitch_drop_ColumnOptions(ptr %field_8)
  %field_2_ptr = getelementptr inbounds %glitch.MSSqlServerSinkOptions__g0__t100, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_MSSqlServerSinkOptions__g0__t100(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.MSSqlServerSinkOptions__g0__t100, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_MSSqlServerSinkOptions__g0__t100(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.MSSqlServerSinkOptions__g0__t100, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.MSSqlServerSinkOptions__g0__t100, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_MSSqlServerSinkOptions__g0__t100(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_PropertyBuilder__g0__t44(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_PropertyBuilder__g0__t44(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.PropertyBuilder__g0__t44, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_PropertyBuilder__g0__t44(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.PropertyBuilder__g0__t44, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.PropertyBuilder__g0__t44, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_PropertyBuilder__g0__t44(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ServiceProvider__g0__t47(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ServiceProvider__g0__t47(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ServiceProvider__g0__t47(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ServiceProvider__g0__t47(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ILoggerProvider__g0__t92(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ILoggerProvider__g0__t92(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ILoggerProvider__g0__t92, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ILoggerProvider__g0__t92(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ILoggerProvider__g0__t92, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ILoggerProvider__g0__t92, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ILoggerProvider__g0__t92(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_ExceptionContext__g0__t74(ptr %object) {
entry:
  %field_3_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t74, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_drop_HttpContext(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_ExceptionContext__g0__t74(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t74, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_ExceptionContext__g0__t74(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t74, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.ExceptionContext__g0__t74, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_ExceptionContext__g0__t74(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Tags_List__g0__t161(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Tags_List__g0__t161(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_List__g0__t161, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Tags_List__g0__t161(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_List__g0__t161, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_List__g0__t161, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Tags_List__g0__t161(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_IQueryableString__g0__t39(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  call void @glitch_string_release(ptr %field_2)
  %field_3_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %object, i32 0, i32 3
  %field_3 = load ptr, ptr %field_3_ptr
  call void @glitch_string_release(ptr %field_3)
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_IQueryableString__g0__t39(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_IQueryableString__g0__t39(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_IQueryableString__g0__t39(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Tags_TagsController__g0__t164(ptr %object) {
entry:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Tags_TagsController__g0__t164(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsController__g0__t164, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Tags_TagsController__g0__t164(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsController__g0__t164, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsController__g0__t164, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Tags_TagsController__g0__t164(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_destroy_Conduit_Features_Tags_TagsEnvelope__g0__t165(ptr %object) {
entry:
  %field_2_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsEnvelope__g0__t165, ptr %object, i32 0, i32 2
  %field_2 = load ptr, ptr %field_2_ptr
  %t296 = icmp eq ptr %field_2, null
  br i1 %t296, label %collection_release_done_131, label %collection_release_130
collection_release_130:
  %t297 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 0
  %t298 = getelementptr inbounds %glitch.list, ptr %field_2, i32 0, i32 2
  %t299 = load i64, ptr %t297
  %t300 = load ptr, ptr %t298
  %t301 = alloca i64
  store i64 0, ptr %t301
  br label %element_drop_loop_132
element_drop_loop_132:
  %t302 = load i64, ptr %t301
  %t303 = icmp ult i64 %t302, %t299
  br i1 %t303, label %element_drop_body_133, label %element_drop_done_134
element_drop_body_133:
  %t304 = getelementptr inbounds ptr, ptr %t300, i64 %t302
  %t305 = load ptr, ptr %t304
  call void @glitch_string_release(ptr %t305)
  %t306 = add i64 %t302, 1
  store i64 %t306, ptr %t301
  br label %element_drop_loop_132
element_drop_done_134:
  call void @glitch_free(ptr %t300)
  call void @glitch_free(ptr %field_2)
  br label %collection_release_done_131
collection_release_done_131:
  call void @glitch_free(ptr %object)
  ret void
}

define void @glitch_retain_Conduit_Features_Tags_TagsEnvelope__g0__t165(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %retain
retain:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsEnvelope__g0__t165, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = add i64 %rc, 1
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @glitch_drop_Conduit_Features_Tags_TagsEnvelope__g0__t165(ptr %object) {
entry:
  %is_null = icmp eq ptr %object, null
  br i1 %is_null, label %done, label %release
release:
  %rc_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsEnvelope__g0__t165, ptr %object, i32 0, i32 0
  %rc = load i64, ptr %rc_ptr
  %next = sub i64 %rc, 1
  %destroy = icmp eq i64 %next, 0
  br i1 %destroy, label %destroy_object, label %keep
destroy_object:
  %drop_ptr_ptr = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsEnvelope__g0__t165, ptr %object, i32 0, i32 1
  %drop_ptr = load ptr, ptr %drop_ptr_ptr
  %has_dynamic_drop = icmp ne ptr %drop_ptr, null
  br i1 %has_dynamic_drop, label %dynamic_drop, label %static_drop
dynamic_drop:
  call void %drop_ptr(ptr %object)
  br label %done
static_drop:
  call void @glitch_destroy_Conduit_Features_Tags_TagsEnvelope__g0__t165(ptr %object)
  br label %done
keep:
  store i64 %next, ptr %rc_ptr
  br label %done
done:
  ret void
}

define void @KeyValuePair__g2__t8_ctor(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.KeyValuePair__g2__t8, ptr %t3, i32 0, i32 0
  %t5 = load ptr, ptr %t1
  store ptr %t5, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.KeyValuePair__g2__t8, ptr %t6, i32 0, i32 1
  %t8 = load ptr, ptr %t2
  store ptr %t8, ptr %t7
  ret void
exception_unwind:
  ret void
}

define void @List__g1__t9_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @List__g1__t9_ToArray__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @List__g1__t9_Add__g0(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @List__g1__t9_Clear__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define i1 @List__g1__t9_Contains__g0(ptr %this, ptr %item) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %item, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define ptr @List__g1__t9_GetEnumerator__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Dictionary__g2__t10_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @Dictionary__g2__t10_Add__g0(ptr %this, ptr %key, ptr %value) {
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

define i1 @Dictionary__g2__t10_ContainsKey__g0(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @Dictionary__g2__t10_Remove__g0(ptr %this, ptr %key) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @Dictionary__g2__t10_TryGetValue__g0(ptr %this, ptr %key, ptr %value) {
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

define void @Dictionary__g2__t10_Clear__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @Dictionary__g2__t10_GetEnumerator__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Uri__g0__t12_ctor(ptr %this, ptr %uri) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %uri, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @GC__g0__t13_SuppressFinalize__g0(ptr %this, ptr %obj) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %obj, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @Exception__g0__t14_ctor(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Exception__g0__t14, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @ArgumentException__g0__t15_ctor(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ArgumentException__g0__t15, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @FormatException__g0__t16_ctor(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.FormatException__g0__t16, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @DateTime__g0__t17_Parse__g0(ptr %this, ptr %s) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %s, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @TimeSpan__g0__t18_ctor(ptr %this, i32 %h, i32 %m, i32 %s, i32 %ms) {
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

define ptr @TimeSpan__g0__t18_FromMinutes__g0(ptr %this, double %minutes) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca double
  store double %minutes, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define i1 @Enum__g0__t19_TryParse__g1(ptr %this, ptr %value, ptr %result) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = alloca ptr
  store ptr %result, ptr %t2
  ret i1 0
exception_unwind:
  ret i1 0
}

define ptr @Type__g0__t20_GetGenericTypeDefinition__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Type__g0__t20_GetGenericArguments__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Type__g0__t20_GetProperty__g0(ptr %this, ptr %name, ptr %flags) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %flags, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Type__g0__t20_GetProperties__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define i1 @string__g0__t22_IsNullOrEmpty__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @string__g0__t22_IsNullOrWhiteSpace__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define ptr @string__g0__t22_ToLower__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_string__g0__t22(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t22_Substring__g0(ptr %this, i32 %start) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %start, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_string__g0__t22(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t22_Split__g0(ptr %this, ptr %separator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %separator, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @string__g0__t22_Replace__g0(ptr %this, ptr %old, ptr %newStr) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %old, ptr %t1
  %t2 = alloca ptr
  store ptr %newStr, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_string__g0__t22(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define i1 @String__g0__t23_IsNullOrEmpty__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i1 @String__g0__t23_IsNullOrWhiteSpace__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define ptr @String__g0__t23_ToLower__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @glitch_retain_String__g0__t23(ptr %t1)
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t23_Substring__g0(ptr %this, i32 %start) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %start, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @glitch_retain_String__g0__t23(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t23_Split__g0(ptr %this, ptr %separator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %separator, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @String__g0__t23_Replace__g0(ptr %this, ptr %old, ptr %newStr) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %old, ptr %t1
  %t2 = alloca ptr
  store ptr %newStr, ptr %t2
  %t3 = load ptr, ptr %t0
  call void @glitch_retain_String__g0__t23(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define i1 @bool__g0__t24_Parse__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret i1 0
exception_unwind:
  ret i1 0
}

define i32 @int__g0__t25_Parse__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = trunc i64 0 to i32
  ret i32 %t2
exception_unwind:
  ret i32 0
}

define i1 @int__g0__t25_TryParse__g0(ptr %this, ptr %s, i32 %result) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %s, ptr %t1
  %t2 = alloca i32
  store i32 %result, ptr %t2
  %t3 = trunc i64 0 to i32
  store i32 %t3, ptr %t2
  ret i1 0
exception_unwind:
  ret i1 0
}

define i32 @Enumerable__g0__t26_Count__g1(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
  %t2 = alloca i32
  store i32 0, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = trunc i64 0 to i32
  store i32 %t4, ptr %t2
  %t5 = load ptr, ptr %t1
  store ptr null, ptr %t3
  br label %while_cond_0
while_cond_0:
  %t6 = load ptr, ptr %t3
  %t7 = icmp ne ptr null, null
  br i1 %t7, label %while_body_1, label %while_end_2
while_body_1:
  %t8 = load i32, ptr %t2
  %t9 = trunc i64 1 to i32
  %t10 = add i32 %t8, %t9
  store i32 %t10, ptr %t2
  br label %while_cond_0
while_end_2:
  %t11 = load i32, ptr %t2
  ret i32 %t11
exception_unwind:
  ret i32 0
}

define ptr @Enumerable__g0__t26_First__g1(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t1
  store ptr null, ptr %t2
  %t4 = load ptr, ptr %t2
  %t5 = icmp ne ptr null, null
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  ret ptr null
if_else_1:
  br label %if_end_2
if_end_2:
  store ptr null, ptr @glitch_exception_pending
  br label %exception_unwind
exception_unwind:
  ret ptr null
}

define ptr @Enumerable__g0__t26_FirstOrDefault__g1(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t1
  store ptr null, ptr %t2
  %t4 = load ptr, ptr %t2
  %t5 = icmp ne ptr null, null
  br i1 %t5, label %if_then_0, label %if_else_1
if_then_0:
  ret ptr null
if_else_1:
  br label %if_end_2
if_end_2:
  ret ptr null
exception_unwind:
  ret ptr null
}

define i1 @Enumerable__g0__t26_Any__g1(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t1
  store ptr null, ptr %t2
  %t4 = load ptr, ptr %t2
  ret i1 0
exception_unwind:
  ret i1 0
}

define ptr @Enumerable__g0__t26_ToList__g1(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
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
  %t14 = load ptr, ptr %t1
  store ptr null, ptr %t3
  br label %while_cond_2
while_cond_2:
  %t15 = load ptr, ptr %t3
  %t16 = icmp ne ptr null, null
  br i1 %t16, label %while_body_3, label %while_end_4
while_body_3:
  %t17 = load ptr, ptr %t2
  %t18 = getelementptr inbounds %glitch.list, ptr %t17, i32 0, i32 0
  %t19 = getelementptr inbounds %glitch.list, ptr %t17, i32 0, i32 1
  %t20 = getelementptr inbounds %glitch.list, ptr %t17, i32 0, i32 2
  %t21 = load i64, ptr %t18
  %t22 = load i64, ptr %t19
  %t23 = load ptr, ptr %t20
  %t24 = icmp eq i64 %t21, %t22
  br i1 %t24, label %list_grow_5, label %list_ready_6
list_grow_5:
  %t25 = mul i64 %t22, 2
  %t26 = mul i64 %t25, 8
  %t27 = call ptr @glitch_realloc(ptr %t23, i64 %t26)
  store i64 %t25, ptr %t19
  store ptr %t27, ptr %t20
  br label %list_ready_6
list_ready_6:
  %t28 = load ptr, ptr %t20
  %t29 = getelementptr inbounds ptr, ptr %t28, i64 %t21
  store ptr null, ptr %t29
  %t30 = add i64 %t21, 1
  store i64 %t30, ptr %t18
  br label %while_cond_2
while_end_4:
  %t31 = load ptr, ptr %t2
  ret ptr %t31
exception_unwind:
  %t32 = load ptr, ptr %t2
  %t33 = icmp eq ptr %t32, null
  br i1 %t33, label %collection_release_done_8, label %collection_release_7
collection_release_7:
  %t34 = getelementptr inbounds %glitch.list, ptr %t32, i32 0, i32 0
  %t35 = getelementptr inbounds %glitch.list, ptr %t32, i32 0, i32 2
  %t36 = load i64, ptr %t34
  %t37 = load ptr, ptr %t35
  call void @glitch_free(ptr %t37)
  call void @glitch_free(ptr %t32)
  br label %collection_release_done_8
collection_release_done_8:
  ret ptr null
}

define ptr @Enumerable__g0__t26_ToArray__g1(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t1
  %t4 = call ptr @ToList(ptr %t3)
  %t5 = load ptr, ptr @glitch_exception_pending
  %t6 = icmp ne ptr %t5, null
  br i1 %t6, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t4, ptr %t2
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t27_CompletedTask__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t27_Run__g0__fn(ptr %this, ptr %function) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %function, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = getelementptr inbounds %glitch.delegate, ptr %t2, i32 0, i32 1
  %t4 = load ptr, ptr %t3
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t2, i32 0, i32 2
  %t6 = load ptr, ptr %t5
  %t7 = call ptr %t4(ptr %t6)
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.Task__g0__t27, ptr %t8, i32 0, i32 2
  %t10 = load ptr, ptr %t9
  ret ptr %t10
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t27_Run__g1__fn(ptr %this, ptr %function) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %function, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = getelementptr inbounds %glitch.delegate, ptr %t2, i32 0, i32 1
  %t4 = load ptr, ptr %t3
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t2, i32 0, i32 2
  %t6 = load ptr, ptr %t5
  %t7 = call ptr %t4(ptr %t6)
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t27_FromResult__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Task__g0__t27_WhenAll__g0(ptr %this, ptr %first, ptr %second) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %first, ptr %t1
  %t2 = alloca ptr
  store ptr %second, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ValueTask__g0__t28_FromResult__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ValueTask__g0__t28_AsTask__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ValueTask__g0__t28, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  %t4 = call ptr @glitch_task_from_result_ptr(ptr %t3)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @DbContextOptions__g1__t29_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbContextOptions__g1__t29, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.1, i32 0, i32 2, i64 0), ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @SqlProvider__g0__t30_ctor(ptr %this, ptr %name, ptr %connectionString) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %connectionString, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t9)
  %t10 = load ptr, ptr %t8
  call void @glitch_string_release(ptr %t10)
  store ptr %t9, ptr %t8
  ret void
exception_unwind:
  ret void
}

define ptr @SqlProvider__g0__t30_BuildSelectAll__g0(ptr %this, ptr %table) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %table, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [15 x i8] }, ptr @.str.2, i32 0, i32 2, i64 0), ptr %t2
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

define void @DbContextOptionsBuilder_ApplicationDbContext__g0__t32_UseSqlServer__g0(ptr %this, ptr %connectionString, ptr %configure) {
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

define void @ChangeTracker__g0__t33_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ChangeTracker__g0__t33, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.DebugView__g0__t34, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.DebugView__g0__t34, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.DebugView__g0__t34, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_DebugView__g0__t34, ptr %t7
  %t8 = load ptr, ptr %t2
  call void @glitch_drop_DebugView__g0__t34(ptr %t8)
  store ptr %t5, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @ChangeTracker__g0__t33_DetectChanges__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ChangeTracker__g0__t33_Clear__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define i1 @DatabaseFacade__g0__t35_EnsureDeleted__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @DatabaseFacade__g0__t35_Migrate__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define i32 @DatabaseFacade__g0__t35_ExecuteSqlRaw__g0(ptr %this, ptr %sql, ptr %parameters) {
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

define void @DbContext__g0__t38_ctor__string(ptr %this, ptr %connectionString) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t6, i32 0, i32 3
  store i1 0, ptr %t7
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t8, i32 0, i32 4
  %t10 = call ptr @glitch_calloc(i64 1, i64 24)
  %t11 = call ptr @glitch_calloc(i64 4, i64 8)
  %t12 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 1
  store i64 4, ptr %t12
  %t13 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 2
  store ptr %t11, ptr %t13
  store ptr %t10, ptr %t9
  %t14 = load ptr, ptr %t0
  %t15 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t14, i32 0, i32 5
  %t16 = getelementptr %glitch.SqlProvider__g0__t30, ptr null, i32 1
  %t17 = ptrtoint ptr %t16 to i64
  %t18 = call ptr @glitch_calloc(i64 1, i64 %t17)
  %t19 = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %t18, i32 0, i32 0
  store i64 1, ptr %t19
  %t20 = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %t18, i32 0, i32 1
  store ptr @glitch_destroy_SqlProvider__g0__t30, ptr %t20
  %t21 = load ptr, ptr %t1
  call void @SqlProvider__g0__t30_ctor(ptr %t18, ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.3, i32 0, i32 2, i64 0), ptr %t21)
  %t22 = load ptr, ptr @glitch_exception_pending
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t24 = load ptr, ptr %t15
  call void @glitch_drop_SqlProvider__g0__t30(ptr %t24)
  store ptr %t18, ptr %t15
  %t25 = load ptr, ptr %t0
  %t26 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t25, i32 0, i32 6
  %t27 = getelementptr %glitch.DatabaseFacade__g0__t35, ptr null, i32 1
  %t28 = ptrtoint ptr %t27 to i64
  %t29 = call ptr @glitch_calloc(i64 1, i64 %t28)
  %t30 = getelementptr inbounds %glitch.DatabaseFacade__g0__t35, ptr %t29, i32 0, i32 0
  store i64 1, ptr %t30
  %t31 = getelementptr inbounds %glitch.DatabaseFacade__g0__t35, ptr %t29, i32 0, i32 1
  store ptr @glitch_destroy_DatabaseFacade__g0__t35, ptr %t31
  %t32 = load ptr, ptr %t26
  call void @glitch_drop_DatabaseFacade__g0__t35(ptr %t32)
  store ptr %t29, ptr %t26
  %t33 = load ptr, ptr %t0
  %t34 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t33, i32 0, i32 7
  %t35 = getelementptr %glitch.ChangeTracker__g0__t33, ptr null, i32 1
  %t36 = ptrtoint ptr %t35 to i64
  %t37 = call ptr @glitch_calloc(i64 1, i64 %t36)
  %t38 = getelementptr inbounds %glitch.ChangeTracker__g0__t33, ptr %t37, i32 0, i32 0
  store i64 1, ptr %t38
  %t39 = getelementptr inbounds %glitch.ChangeTracker__g0__t33, ptr %t37, i32 0, i32 1
  store ptr @glitch_destroy_ChangeTracker__g0__t33, ptr %t39
  call void @ChangeTracker__g0__t33_ctor(ptr %t37)
  %t40 = load ptr, ptr @glitch_exception_pending
  %t41 = icmp ne ptr %t40, null
  br i1 %t41, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t42 = load ptr, ptr %t34
  call void @glitch_drop_ChangeTracker__g0__t33(ptr %t42)
  store ptr %t37, ptr %t34
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t38_ctor__object(ptr %this, ptr %options) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %options, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t4)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.4, i32 0, i32 2, i64 0), ptr %t3
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t5, i32 0, i32 3
  store i1 0, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t7, i32 0, i32 4
  %t9 = call ptr @glitch_calloc(i64 1, i64 24)
  %t10 = call ptr @glitch_calloc(i64 4, i64 8)
  %t11 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 1
  store i64 4, ptr %t11
  %t12 = getelementptr inbounds %glitch.list, ptr %t9, i32 0, i32 2
  store ptr %t10, ptr %t12
  store ptr %t9, ptr %t8
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t13, i32 0, i32 5
  %t15 = getelementptr %glitch.SqlProvider__g0__t30, ptr null, i32 1
  %t16 = ptrtoint ptr %t15 to i64
  %t17 = call ptr @glitch_calloc(i64 1, i64 %t16)
  %t18 = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %t17, i32 0, i32 0
  store i64 1, ptr %t18
  %t19 = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %t17, i32 0, i32 1
  store ptr @glitch_destroy_SqlProvider__g0__t30, ptr %t19
  call void @SqlProvider__g0__t30_ctor(ptr %t17, ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.5, i32 0, i32 2, i64 0), ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.6, i32 0, i32 2, i64 0))
  %t20 = load ptr, ptr @glitch_exception_pending
  %t21 = icmp ne ptr %t20, null
  br i1 %t21, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t22 = load ptr, ptr %t14
  call void @glitch_drop_SqlProvider__g0__t30(ptr %t22)
  store ptr %t17, ptr %t14
  %t23 = load ptr, ptr %t0
  %t24 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t23, i32 0, i32 6
  %t25 = getelementptr %glitch.DatabaseFacade__g0__t35, ptr null, i32 1
  %t26 = ptrtoint ptr %t25 to i64
  %t27 = call ptr @glitch_calloc(i64 1, i64 %t26)
  %t28 = getelementptr inbounds %glitch.DatabaseFacade__g0__t35, ptr %t27, i32 0, i32 0
  store i64 1, ptr %t28
  %t29 = getelementptr inbounds %glitch.DatabaseFacade__g0__t35, ptr %t27, i32 0, i32 1
  store ptr @glitch_destroy_DatabaseFacade__g0__t35, ptr %t29
  %t30 = load ptr, ptr %t24
  call void @glitch_drop_DatabaseFacade__g0__t35(ptr %t30)
  store ptr %t27, ptr %t24
  %t31 = load ptr, ptr %t0
  %t32 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t31, i32 0, i32 7
  %t33 = getelementptr %glitch.ChangeTracker__g0__t33, ptr null, i32 1
  %t34 = ptrtoint ptr %t33 to i64
  %t35 = call ptr @glitch_calloc(i64 1, i64 %t34)
  %t36 = getelementptr inbounds %glitch.ChangeTracker__g0__t33, ptr %t35, i32 0, i32 0
  store i64 1, ptr %t36
  %t37 = getelementptr inbounds %glitch.ChangeTracker__g0__t33, ptr %t35, i32 0, i32 1
  store ptr @glitch_destroy_ChangeTracker__g0__t33, ptr %t37
  call void @ChangeTracker__g0__t33_ctor(ptr %t35)
  %t38 = load ptr, ptr @glitch_exception_pending
  %t39 = icmp ne ptr %t38, null
  br i1 %t39, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t40 = load ptr, ptr %t32
  call void @glitch_drop_ChangeTracker__g0__t33(ptr %t40)
  store ptr %t35, ptr %t32
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t38_EnsureNotDisposed__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t1, i32 0, i32 3
  %t3 = load i1, ptr %t2
  br i1 %t3, label %if_then_0, label %if_else_1
if_then_0:
  store ptr getelementptr inbounds ({ i64, i64, [22 x i8] }, ptr @.str.7, i32 0, i32 2, i64 0), ptr @glitch_exception_pending
  br label %exception_unwind
if_else_1:
  br label %if_end_2
if_end_2:
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t38_Track__g0(ptr %this, ptr %entityKey) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entityKey, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t38_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t5, i32 0, i32 4
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

define i32 @DbContext__g0__t38_get_TrackedCount__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t1, i32 0, i32 4
  %t3 = load ptr, ptr %t2
  %t4 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 0
  %t5 = load i64, ptr %t4
  %t6 = trunc i64 %t5 to i32
  ret i32 %t6
exception_unwind:
  ret i32 0
}

define i32 @DbContext__g0__t38_SaveChanges__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 0, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t38_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t5, i32 0, i32 4
  %t7 = load ptr, ptr %t6
  %t8 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 0
  %t9 = load i64, ptr %t8
  %t10 = trunc i64 %t9 to i32
  store i32 %t10, ptr %t1
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t11, i32 0, i32 4
  %t13 = load ptr, ptr %t12
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  store i64 0, ptr %t14
  %t15 = load i32, ptr %t1
  ret i32 %t15
exception_unwind:
  ret i32 0
}

define ptr @DbContext__g0__t38_SaveChangesAsync__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 0, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t38_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t5, i32 0, i32 4
  %t7 = load ptr, ptr %t6
  %t8 = getelementptr inbounds %glitch.list, ptr %t7, i32 0, i32 0
  %t9 = load i64, ptr %t8
  %t10 = trunc i64 %t9 to i32
  store i32 %t10, ptr %t1
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t11, i32 0, i32 4
  %t13 = load ptr, ptr %t12
  %t14 = getelementptr inbounds %glitch.list, ptr %t13, i32 0, i32 0
  store i64 0, ptr %t14
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @DbContext__g0__t38_Add__g0(ptr %this, ptr %entityKey) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entityKey, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  call void @DbContext__g0__t38_Track__g0(ptr %t2, ptr %t3)
  %t4 = load ptr, ptr @glitch_exception_pending
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t38_Update__g0(ptr %this, ptr %entityKey) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entityKey, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  call void @DbContext__g0__t38_Track__g0(ptr %t2, ptr %t3)
  %t4 = load ptr, ptr @glitch_exception_pending
  %t5 = icmp ne ptr %t4, null
  br i1 %t5, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t38_Remove__g0(ptr %this, ptr %entityKey) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entityKey, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t38_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @DbContext__g0__t38_Dispose__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t1, i32 0, i32 3
  store i1 1, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t3, i32 0, i32 4
  %t5 = load ptr, ptr %t4
  %t6 = getelementptr inbounds %glitch.list, ptr %t5, i32 0, i32 0
  store i64 0, ptr %t6
  ret void
exception_unwind:
  ret void
}

define ptr @DbContext__g0__t38_Set__g1(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @DbContext__g0__t38_Entry__g0(ptr %this, ptr %entity) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %entity, ptr %t1
  %t2 = getelementptr %glitch.EntityEntry__g0__t36, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.EntityEntry__g0__t36, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.EntityEntry__g0__t36, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_EntityEntry__g0__t36, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @IQueryableString__g0__t39_ctor(ptr %this, ptr %connectionString, ptr %table, i1 %tracking) {
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
  %t5 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t12, i32 0, i32 4
  %t14 = load i1, ptr %t3
  store i1 %t14, ptr %t13
  ret void
exception_unwind:
  ret void
}

define ptr @IQueryableString__g0__t39_AsNoTracking__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.IQueryableString__g0__t39, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_IQueryableString__g0__t39, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t9, i32 0, i32 3
  %t11 = load ptr, ptr %t10
  call void @IQueryableString__g0__t39_ctor(ptr %t3, ptr %t8, ptr %t11, i1 0)
  %t12 = load ptr, ptr @glitch_exception_pending
  %t13 = icmp ne ptr %t12, null
  br i1 %t13, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @IQueryableString__g0__t39_ToQueryString__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr null, ptr %t1
  %t2 = getelementptr %glitch.SqlProvider__g0__t30, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.SqlProvider__g0__t30, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_SqlProvider__g0__t30, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  call void @SqlProvider__g0__t30_ctor(ptr %t4, ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.8, i32 0, i32 2, i64 0), ptr %t9)
  %t10 = load ptr, ptr @glitch_exception_pending
  %t11 = icmp ne ptr %t10, null
  br i1 %t11, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t12 = load ptr, ptr %t1
  call void @glitch_drop_SqlProvider__g0__t30(ptr %t12)
  store ptr %t4, ptr %t1
  %t13 = load ptr, ptr %t1
  %t14 = load ptr, ptr %t0
  %t15 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t14, i32 0, i32 3
  %t16 = load ptr, ptr %t15
  %t17 = call ptr @SqlProvider__g0__t30_BuildSelectAll__g0(ptr %t13, ptr %t16)
  %t18 = load ptr, ptr @glitch_exception_pending
  %t19 = icmp ne ptr %t18, null
  br i1 %t19, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t20 = load ptr, ptr %t1
  call void @glitch_drop_SqlProvider__g0__t30(ptr %t20)
  ret ptr %t17
exception_unwind:
  %t21 = load ptr, ptr %t1
  call void @glitch_drop_SqlProvider__g0__t30(ptr %t21)
  ret ptr null
}

define ptr @IQueryableString__g0__t39_ToList__g0(ptr %this) {
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
  %t21 = call ptr @IQueryableString__g0__t39_ToQueryString__g0(ptr %t20)
  %t22 = load ptr, ptr @glitch_exception_pending
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t24 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t24)
  store ptr %t21, ptr %t2
  %t25 = load ptr, ptr %t0
  %t26 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t25, i32 0, i32 4
  %t27 = load i1, ptr %t26
  br i1 %t27, label %if_then_6, label %if_else_7
if_then_6:
  %t28 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t28)
  store ptr getelementptr inbounds ({ i64, i64, [9 x i8] }, ptr @.str.9, i32 0, i32 2, i64 0), ptr %t3
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

define void @DbSetString__g0__t40_ctor(ptr %this, ptr %connectionString, ptr %table) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %connectionString, ptr %t1
  %t2 = alloca ptr
  store ptr %table, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t9)
  %t10 = load ptr, ptr %t8
  call void @glitch_string_release(ptr %t10)
  store ptr %t9, ptr %t8
  ret void
exception_unwind:
  ret void
}

define ptr @DbSetString__g0__t40_AsQueryable__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.IQueryableString__g0__t39, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_IQueryableString__g0__t39, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %t9, i32 0, i32 3
  %t11 = load ptr, ptr %t10
  call void @IQueryableString__g0__t39_ctor(ptr %t3, ptr %t8, ptr %t11, i1 1)
  %t12 = load ptr, ptr @glitch_exception_pending
  %t13 = icmp ne ptr %t12, null
  br i1 %t13, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbSetString__g0__t40_AsNoTracking__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.IQueryableString__g0__t39, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.IQueryableString__g0__t39, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_IQueryableString__g0__t39, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %t9, i32 0, i32 3
  %t11 = load ptr, ptr %t10
  call void @IQueryableString__g0__t39_ctor(ptr %t3, ptr %t8, ptr %t11, i1 0)
  %t12 = load ptr, ptr @glitch_exception_pending
  %t13 = icmp ne ptr %t12, null
  br i1 %t13, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @DbSetString__g0__t40_ToList__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = call ptr @DbSetString__g0__t40_AsQueryable__g0(ptr %t1)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = call ptr @IQueryableString__g0__t39_ToList__g0(ptr %t2)
  %t6 = load ptr, ptr @glitch_exception_pending
  %t7 = icmp ne ptr %t6, null
  br i1 %t7, label %exception_unwind, label %call_continue_1
call_continue_1:
  call void @glitch_drop_IQueryableString__g0__t39(ptr %t2)
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @ModelBuilder__g0__t41_Entity__g1__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ModelBuilder__g0__t41_Entity__g0__string(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ModelBuilder__g0__t41_Entity__g0__string_object(ptr %this, ptr %name, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = alloca ptr
  store ptr %configure, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @ServiceProvider__g0__t47_ctor(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @ServiceProvider__g0__t47_GetRequiredService__g1__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ServiceProvider__g0__t47_GetRequiredService__g0__string(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  call void @glitch_string_retain(ptr %t4)
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @ServiceProvider__g0__t47_GetService__g1(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ServiceProvider__g0__t47_CreateScope__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.ServiceScope__g0__t49, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.ServiceScope__g0__t49, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.ServiceScope__g0__t49, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_ServiceScope__g0__t49, ptr %t5
  %t6 = load ptr, ptr %t0
  call void @ServiceScope__g0__t49_ctor(ptr %t3, ptr %t6)
  %t7 = load ptr, ptr @glitch_exception_pending
  %t8 = icmp ne ptr %t7, null
  br i1 %t8, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @ServiceScope__g0__t49_ctor(ptr %this, ptr %provider) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %provider, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ServiceScope__g0__t49, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr null, ptr %t1
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_IServiceProvider__g0__t46(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @MvcBuilder__g0__t50_AddJsonOptions__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = getelementptr %glitch.MvcBuilder__g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @MvcBuilder__g0__t50_ConfigureApiBehaviorOptions__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = getelementptr %glitch.MvcBuilder__g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @MvcBuilder__g0__t50_AddJwtBearer__g0(ptr %this, ptr %scheme, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %scheme, ptr %t1
  %t2 = alloca ptr
  store ptr %configure, ptr %t2
  %t3 = getelementptr %glitch.MvcBuilder__g0__t50, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t50, ptr %t7
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @MvcBuilder__g0__t50_AddAuthentication__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.MvcBuilder__g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @ServiceCollection__g0__t52_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.10, i32 0, i32 2, i64 0), ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddSingleton__g0__object(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddSingleton__g0__string(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddSingleton__g0__string_string(ptr %this, ptr %key, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t4
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddTransient__g2(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddScoped__g0__object_object(ptr %this, ptr %serviceType, ptr %implementationType) {
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

define void @ServiceCollection__g0__t52_AddScoped__g2__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_Configure__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddApiVersioning__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddVersionedApiExplorer__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddSwaggerGen__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddMemoryCache__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddHttpContextAccessor__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddEndpointsApiExplorer__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddLibraryApiVersionConfiguration__g0(ptr %this, ptr %version) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %version, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddAndConfigureSwagger__g0(ptr %this, ptr %configuration, ptr %path, i1 %enabled) {
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

define void @ServiceCollection__g0__t52_AddDbContextPool__g1(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddAutoMapper__g0(ptr %this, ptr %marker) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %marker, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddRepositories__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddDataServices__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ServiceCollection__g0__t52_AddCors__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define ptr @ServiceCollection__g0__t52_AddControllers__g0(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = getelementptr %glitch.MvcBuilder__g0__t50, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t50, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @ServiceCollection__g0__t52_AddAuthentication__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.MvcBuilder__g0__t50, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.MvcBuilder__g0__t50, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_MvcBuilder__g0__t50, ptr %t5
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @ServiceCollection__g0__t52_BuildServiceProvider__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.ServiceProvider__g0__t47, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_ServiceProvider__g0__t47, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  call void @ServiceProvider__g0__t47_ctor(ptr %t3, ptr %t8)
  %t9 = load ptr, ptr @glitch_exception_pending
  %t10 = icmp ne ptr %t9, null
  br i1 %t10, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @ConfigurationManager__g0__t53_GetConnectionString__g0(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  ret ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.11, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define ptr @ConfigurationManager__g0__t53_GetSection__g0(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = getelementptr %glitch.ConfigurationManager__g0__t53, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.ConfigurationManager__g0__t53, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.ConfigurationManager__g0__t53, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_ConfigurationManager__g0__t53, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @ConfigurationManager__g0__t53_Get__g0__string(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  ret ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.12, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define ptr @ConfigurationManager__g0__t53_Get__g1__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ConfigurationManager__g0__t53_GetValue__g1(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define i1 @HostEnvironment__g0__t54_IsDevelopment__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret i1 0
exception_unwind:
  ret i1 0
}

define void @LoggingBuilder__g0__t55_ClearProviders__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @LoggingBuilder__g0__t55_AddSerilog__g0(ptr %this, ptr %logger, i1 %dispose) {
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

define void @WebApplicationBuilder__g0__t56_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.ServiceCollection__g0__t52, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.ServiceCollection__g0__t52, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_ServiceCollection__g0__t52, ptr %t7
  call void @ServiceCollection__g0__t52_ctor(ptr %t5)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = load ptr, ptr %t2
  call void @glitch_drop_ServiceCollection__g0__t52(ptr %t10)
  store ptr %t5, ptr %t2
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t11, i32 0, i32 3
  %t13 = getelementptr %glitch.ConfigurationManager__g0__t53, ptr null, i32 1
  %t14 = ptrtoint ptr %t13 to i64
  %t15 = call ptr @glitch_calloc(i64 1, i64 %t14)
  %t16 = getelementptr inbounds %glitch.ConfigurationManager__g0__t53, ptr %t15, i32 0, i32 0
  store i64 1, ptr %t16
  %t17 = getelementptr inbounds %glitch.ConfigurationManager__g0__t53, ptr %t15, i32 0, i32 1
  store ptr @glitch_destroy_ConfigurationManager__g0__t53, ptr %t17
  %t18 = load ptr, ptr %t12
  call void @glitch_drop_ConfigurationManager__g0__t53(ptr %t18)
  store ptr %t15, ptr %t12
  %t19 = load ptr, ptr %t0
  %t20 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t19, i32 0, i32 4
  %t21 = getelementptr %glitch.HostEnvironment__g0__t54, ptr null, i32 1
  %t22 = ptrtoint ptr %t21 to i64
  %t23 = call ptr @glitch_calloc(i64 1, i64 %t22)
  %t24 = getelementptr inbounds %glitch.HostEnvironment__g0__t54, ptr %t23, i32 0, i32 0
  store i64 1, ptr %t24
  %t25 = getelementptr inbounds %glitch.HostEnvironment__g0__t54, ptr %t23, i32 0, i32 1
  store ptr @glitch_destroy_HostEnvironment__g0__t54, ptr %t25
  %t26 = load ptr, ptr %t20
  call void @glitch_drop_HostEnvironment__g0__t54(ptr %t26)
  store ptr %t23, ptr %t20
  %t27 = load ptr, ptr %t0
  %t28 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t27, i32 0, i32 5
  %t29 = getelementptr %glitch.LoggingBuilder__g0__t55, ptr null, i32 1
  %t30 = ptrtoint ptr %t29 to i64
  %t31 = call ptr @glitch_calloc(i64 1, i64 %t30)
  %t32 = getelementptr inbounds %glitch.LoggingBuilder__g0__t55, ptr %t31, i32 0, i32 0
  store i64 1, ptr %t32
  %t33 = getelementptr inbounds %glitch.LoggingBuilder__g0__t55, ptr %t31, i32 0, i32 1
  store ptr @glitch_destroy_LoggingBuilder__g0__t55, ptr %t33
  %t34 = load ptr, ptr %t28
  call void @glitch_drop_LoggingBuilder__g0__t55(ptr %t34)
  store ptr %t31, ptr %t28
  ret void
exception_unwind:
  ret void
}

define ptr @WebApplicationBuilder__g0__t56_Build__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr %glitch.WebApplication__g0__t65, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_WebApplication__g0__t65, ptr %t5
  call void @WebApplication__g0__t65_ctor(ptr %t3)
  %t6 = load ptr, ptr @glitch_exception_pending
  %t7 = icmp ne ptr %t6, null
  br i1 %t7, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @WebApplicationBuilder__g0__t56_ConfigureSerilog__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @HttpRequest__g0__t57_ctor(ptr %this, ptr %method, ptr %path, ptr %body) {
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
  %t5 = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %t12, i32 0, i32 4
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
  %t21 = getelementptr inbounds %glitch.HttpRequest__g0__t57, ptr %t20, i32 0, i32 5
  %t22 = load ptr, ptr %t3
  call void @glitch_string_retain(ptr %t22)
  %t23 = load ptr, ptr %t21
  call void @glitch_string_release(ptr %t23)
  store ptr %t22, ptr %t21
  ret void
exception_unwind:
  ret void
}

define void @HttpResponse__g0__t58_ctor(ptr %this, i32 %statusCode, ptr %body) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %statusCode, ptr %t1
  %t2 = alloca ptr
  store ptr %body, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t3, i32 0, i32 2
  %t5 = load i32, ptr %t1
  store i32 %t5, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t6, i32 0, i32 3
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
  %t15 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t14, i32 0, i32 4
  %t16 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t16)
  %t17 = load ptr, ptr %t15
  call void @glitch_string_release(ptr %t17)
  store ptr %t16, ptr %t15
  ret void
exception_unwind:
  ret void
}

define void @IPAddress__g0__t59_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.IPAddress__g0__t59, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [10 x i8] }, ptr @.str.13, i32 0, i32 2, i64 0), ptr %t2
  ret void
exception_unwind:
  ret void
}

define ptr @IPAddress__g0__t59_ToString__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.IPAddress__g0__t59, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define void @ConnectionInfo__g0__t60_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ConnectionInfo__g0__t60, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.IPAddress__g0__t59, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.IPAddress__g0__t59, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.IPAddress__g0__t59, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_IPAddress__g0__t59, ptr %t7
  call void @IPAddress__g0__t59_ctor(ptr %t5)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t10 = load ptr, ptr %t2
  call void @glitch_drop_IPAddress__g0__t59(ptr %t10)
  store ptr %t5, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @HttpContext__g0__t61_ctor(ptr %this, ptr %request) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %request, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr null, ptr %t1
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_HttpRequest__g0__t57(ptr %t5)
  store ptr %t4, ptr %t3
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t6, i32 0, i32 3
  %t8 = getelementptr %glitch.HttpResponse__g0__t58, ptr null, i32 1
  %t9 = ptrtoint ptr %t8 to i64
  %t10 = call ptr @glitch_calloc(i64 1, i64 %t9)
  %t11 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t10, i32 0, i32 0
  store i64 1, ptr %t11
  %t12 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t10, i32 0, i32 1
  store ptr @glitch_destroy_HttpResponse__g0__t58, ptr %t12
  %t13 = trunc i64 200 to i32
  call void @HttpResponse__g0__t58_ctor(ptr %t10, i32 %t13, ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.14, i32 0, i32 2, i64 0))
  %t14 = load ptr, ptr @glitch_exception_pending
  %t15 = icmp ne ptr %t14, null
  br i1 %t15, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t16 = load ptr, ptr %t7
  call void @glitch_drop_HttpResponse__g0__t58(ptr %t16)
  store ptr %t10, ptr %t7
  %t17 = load ptr, ptr %t0
  %t18 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t17, i32 0, i32 4
  %t19 = getelementptr %glitch.ConnectionInfo__g0__t60, ptr null, i32 1
  %t20 = ptrtoint ptr %t19 to i64
  %t21 = call ptr @glitch_calloc(i64 1, i64 %t20)
  %t22 = getelementptr inbounds %glitch.ConnectionInfo__g0__t60, ptr %t21, i32 0, i32 0
  store i64 1, ptr %t22
  %t23 = getelementptr inbounds %glitch.ConnectionInfo__g0__t60, ptr %t21, i32 0, i32 1
  store ptr @glitch_destroy_ConnectionInfo__g0__t60, ptr %t23
  call void @ConnectionInfo__g0__t60_ctor(ptr %t21)
  %t24 = load ptr, ptr @glitch_exception_pending
  %t25 = icmp ne ptr %t24, null
  br i1 %t25, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t26 = load ptr, ptr %t18
  call void @glitch_drop_ConnectionInfo__g0__t60(ptr %t26)
  store ptr %t21, ptr %t18
  %t27 = load ptr, ptr %t0
  %t28 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t27, i32 0, i32 5
  %t29 = load ptr, ptr %t28
  call void @glitch_string_release(ptr %t29)
  store ptr getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.15, i32 0, i32 2, i64 0), ptr %t28
  ret void
exception_unwind:
  ret void
}

define void @HttpContext__g0__t61_WriteJson__g0(ptr %this, ptr %text) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %text, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t2, i32 0, i32 3
  %t4 = load ptr, ptr %t3
  %t5 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t4, i32 0, i32 4
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t9
  %t11 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t10, i32 0, i32 3
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
  store ptr getelementptr inbounds ({ i64, i64, [13 x i8] }, ptr @.str.16, i32 0, i32 2, i64 0), ptr %t29
  store ptr getelementptr inbounds ({ i64, i64, [17 x i8] }, ptr @.str.17, i32 0, i32 2, i64 0), ptr %t30
  %t31 = add i64 %t17, 1
  store i64 %t31, ptr %t13
  ret void
exception_unwind:
  ret void
}

define void @Endpoint__g0__t64_ctor(ptr %this, ptr %method, ptr %path, ptr %text) {
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
  %t5 = getelementptr inbounds %glitch.Endpoint__g0__t64, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.Endpoint__g0__t64, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Endpoint__g0__t64, ptr %t12, i32 0, i32 4
  %t14 = load ptr, ptr %t3
  call void @glitch_string_retain(ptr %t14)
  %t15 = load ptr, ptr %t13
  call void @glitch_string_release(ptr %t15)
  store ptr %t14, ptr %t13
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t1, i32 0, i32 2
  %t3 = call ptr @glitch_calloc(i64 1, i64 24)
  %t4 = call ptr @glitch_calloc(i64 4, i64 8)
  %t5 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 1
  store i64 4, ptr %t5
  %t6 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 2
  store ptr %t4, ptr %t6
  store ptr %t3, ptr %t2
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t7, i32 0, i32 3
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
  %t16 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t15, i32 0, i32 4
  %t17 = call ptr @glitch_calloc(i64 1, i64 24)
  %t18 = call ptr @glitch_calloc(i64 4, i64 8)
  %t19 = getelementptr inbounds %glitch.list, ptr %t17, i32 0, i32 1
  store i64 4, ptr %t19
  %t20 = getelementptr inbounds %glitch.list, ptr %t17, i32 0, i32 2
  store ptr %t18, ptr %t20
  store ptr %t17, ptr %t16
  %t21 = load ptr, ptr %t0
  %t22 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t21, i32 0, i32 5
  %t23 = getelementptr %glitch.ConfigurationManager__g0__t53, ptr null, i32 1
  %t24 = ptrtoint ptr %t23 to i64
  %t25 = call ptr @glitch_calloc(i64 1, i64 %t24)
  %t26 = getelementptr inbounds %glitch.ConfigurationManager__g0__t53, ptr %t25, i32 0, i32 0
  store i64 1, ptr %t26
  %t27 = getelementptr inbounds %glitch.ConfigurationManager__g0__t53, ptr %t25, i32 0, i32 1
  store ptr @glitch_destroy_ConfigurationManager__g0__t53, ptr %t27
  %t28 = load ptr, ptr %t22
  call void @glitch_drop_ConfigurationManager__g0__t53(ptr %t28)
  store ptr %t25, ptr %t22
  %t29 = load ptr, ptr %t0
  %t30 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t29, i32 0, i32 6
  %t31 = getelementptr %glitch.ServiceProvider__g0__t47, ptr null, i32 1
  %t32 = ptrtoint ptr %t31 to i64
  %t33 = call ptr @glitch_calloc(i64 1, i64 %t32)
  %t34 = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %t33, i32 0, i32 0
  store i64 1, ptr %t34
  %t35 = getelementptr inbounds %glitch.ServiceProvider__g0__t47, ptr %t33, i32 0, i32 1
  store ptr @glitch_destroy_ServiceProvider__g0__t47, ptr %t35
  call void @ServiceProvider__g0__t47_ctor(ptr %t33, ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.18, i32 0, i32 2, i64 0))
  %t36 = load ptr, ptr @glitch_exception_pending
  %t37 = icmp ne ptr %t36, null
  br i1 %t37, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t38 = load ptr, ptr %t30
  call void @glitch_drop_ServiceProvider__g0__t47(ptr %t38)
  store ptr %t33, ptr %t30
  %t39 = load ptr, ptr %t0
  %t40 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t39, i32 0, i32 7
  %t41 = getelementptr %glitch.HostEnvironment__g0__t54, ptr null, i32 1
  %t42 = ptrtoint ptr %t41 to i64
  %t43 = call ptr @glitch_calloc(i64 1, i64 %t42)
  %t44 = getelementptr inbounds %glitch.HostEnvironment__g0__t54, ptr %t43, i32 0, i32 0
  store i64 1, ptr %t44
  %t45 = getelementptr inbounds %glitch.HostEnvironment__g0__t54, ptr %t43, i32 0, i32 1
  store ptr @glitch_destroy_HostEnvironment__g0__t54, ptr %t45
  %t46 = load ptr, ptr %t40
  call void @glitch_drop_HostEnvironment__g0__t54(ptr %t46)
  store ptr %t43, ptr %t40
  %t47 = load ptr, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_Use__g0(ptr %this, ptr %name) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %name, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t2, i32 0, i32 4
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

define void @WebApplication__g0__t65_UseRouting__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.19, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseEndpoints__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [10 x i8] }, ptr @.str.20, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseTrace__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.21, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseJsonEnvelope__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [14 x i8] }, ptr @.str.22, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseHttpsRedirection__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [18 x i8] }, ptr @.str.23, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseAuthorization__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [14 x i8] }, ptr @.str.24, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseAuthentication__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [15 x i8] }, ptr @.str.25, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseCors__g0__string(ptr %this, ptr %policy) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %policy, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t2, ptr getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.26, i32 0, i32 2, i64 0))
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseCors__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.27, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseMiddleware__g1__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseMiddleware__g1__object(ptr %this, ptr %param) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %param, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseSwagger__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseSwaggerUI__g0__object(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseSwaggerUI__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseStaticFiles__g0__object(ptr %this, ptr %options) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %options, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_UseStaticFiles__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_MapControllers__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  call void @WebApplication__g0__t65_Use__g0(ptr %t1, ptr getelementptr inbounds ({ i64, i64, [12 x i8] }, ptr @.str.28, i32 0, i32 2, i64 0))
  %t2 = load ptr, ptr @glitch_exception_pending
  %t3 = icmp ne ptr %t2, null
  br i1 %t3, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_MapGet__g0(ptr %this, ptr %path, ptr %text) {
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
  %t5 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.29, i32 0, i32 2, i64 0), ptr %t4)
  %t6 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t3
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t7, i32 0, i32 2
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
  %t25 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t24, i32 0, i32 3
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

define void @WebApplication__g0__t65_MapPost__g0(ptr %this, ptr %path, ptr %text) {
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
  %t5 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.30, i32 0, i32 2, i64 0), ptr %t4)
  %t6 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t6)
  store ptr %t5, ptr %t3
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t7, i32 0, i32 2
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
  %t25 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t24, i32 0, i32 3
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

define ptr @WebApplication__g0__t65_Handle__g0(ptr %this, ptr %method, ptr %path, ptr %body) {
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
  %t8 = call ptr @glitch_string_concat(ptr %t7, ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.31, i32 0, i32 2, i64 0))
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
  %t26 = call ptr @WebApplication__g0__t65_ApplyMiddleware__g0(ptr %t24, ptr %t25)
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
  %t33 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t32, i32 0, i32 3
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
  %t53 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t52, i32 0, i32 3
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
  %t77 = call ptr @WebApplication__g0__t65_ApplyMiddleware__g0(ptr %t51, ptr %t76)
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
  ret ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.32, i32 0, i32 2, i64 0)
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

define ptr @WebApplication__g0__t65_ApplyMiddleware__g0(ptr %this, ptr %text) {
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
  %t7 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t2
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t8, i32 0, i32 4
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
  %t24 = call i32 @strcmp(ptr %t20, ptr getelementptr inbounds ({ i64, i64, [14 x i8] }, ptr @.str.33, i32 0, i32 2, i64 0))
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
  store ptr getelementptr inbounds ({ i64, i64, [11 x i8] }, ptr @.str.34, i32 0, i32 2, i64 0), ptr %t3
  %t26 = load ptr, ptr %t3
  %t27 = load ptr, ptr %t2
  %t28 = call ptr @glitch_string_concat(ptr %t26, ptr %t27)
  %t29 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t29)
  store ptr %t28, ptr %t4
  %t30 = load ptr, ptr %t4
  %t31 = call ptr @glitch_string_concat(ptr %t30, ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.35, i32 0, i32 2, i64 0))
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
  %t36 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t35, i32 0, i32 4
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
  %t51 = call i32 @strcmp(ptr %t47, ptr getelementptr inbounds ({ i64, i64, [6 x i8] }, ptr @.str.36, i32 0, i32 2, i64 0))
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
  store ptr getelementptr inbounds ({ i64, i64, [7 x i8] }, ptr @.str.37, i32 0, i32 2, i64 0), ptr %t3
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

define void @WebApplication__g0__t65_Run__g0__int(ptr %this, i32 %port) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %port, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load i32, ptr %t1
  %t4 = trunc i64 0 to i32
  call void @GlitchRestHost_Run(ptr %t2, i32 %t3, i32 %t4, ptr @WebApplication_Handle, ptr @glitch_string_release)
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_Run__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = trunc i64 5000 to i32
  %t3 = trunc i64 0 to i32
  call void @GlitchRestHost_Run(ptr %t1, i32 %t2, i32 %t3, ptr @WebApplication_Handle, ptr @glitch_string_release)
  ret void
exception_unwind:
  ret void
}

define void @WebApplication__g0__t65_RunOnce__g0(ptr %this, i32 %port) {
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

define void @PhysicalFileProvider__g0__t67_ctor(ptr %this, ptr %path) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @ApiVersion__g0__t68_ctor(ptr %this, i32 %major, i32 %minor, ptr %status) {
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

define void @ProblemDetails__g0__t69_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ProblemDetails__g0__t69, ptr %t1, i32 0, i32 6
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

define void @ObjectResult__g0__t70_ctor(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @JwtBearerDefaults__g0__t71_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t71, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_string_release(ptr %t3)
  store ptr getelementptr inbounds ({ i64, i64, [7 x i8] }, ptr @.str.38, i32 0, i32 2, i64 0), ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @ModelStateDictionary__g0__t77_AddModelError__g0(ptr %this, ptr %key, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %key, ptr %t1
  %t2 = alloca ptr
  store ptr %message, ptr %t2
  ret void
exception_unwind:
  ret void
}

define ptr @ModelStateDictionary__g0__t77_ToDictionary__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = call ptr @glitch_calloc(i64 1, i64 32)
  %t2 = call ptr @glitch_calloc(i64 4, i64 8)
  %t3 = call ptr @glitch_calloc(i64 4, i64 8)
  %t4 = getelementptr inbounds %glitch.dict, ptr %t1, i32 0, i32 1
  store i64 4, ptr %t4
  %t5 = getelementptr inbounds %glitch.dict, ptr %t1, i32 0, i32 2
  store ptr %t2, ptr %t5
  %t6 = getelementptr inbounds %glitch.dict, ptr %t1, i32 0, i32 3
  store ptr %t3, ptr %t6
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define void @ControllerBase__g0__t78_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.ControllerBase__g0__t78, ptr %t1, i32 0, i32 2
  %t3 = getelementptr %glitch.ModelStateDictionary__g0__t77, ptr null, i32 1
  %t4 = ptrtoint ptr %t3 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t4)
  %t6 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t77, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t77, ptr %t5, i32 0, i32 1
  store ptr @glitch_destroy_ModelStateDictionary__g0__t77, ptr %t7
  %t8 = load ptr, ptr %t2
  call void @glitch_drop_ModelStateDictionary__g0__t77(ptr %t8)
  store ptr %t5, ptr %t2
  ret void
exception_unwind:
  ret void
}

define ptr @ControllerBase__g0__t78_Ok__g0__object(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t1
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @ControllerBase__g0__t78_Ok__g0__overload(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ControllerBase__g0__t78_BadRequest__g0(ptr %this, ptr %error) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %error, ptr %t1
  %t2 = load ptr, ptr %t1
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @ControllerBase__g0__t78_NotFound__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @ControllerBase__g0__t78_StatusCode__g0(ptr %this, i32 %code, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca i32
  store i32 %code, ptr %t1
  %t2 = alloca ptr
  store ptr %value, ptr %t2
  %t3 = load ptr, ptr %t2
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @Mapper__g0__t80_Map__g1(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @MemberConfigurationExpression__g0__t82_MapFrom__g0(ptr %this, ptr %source) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %source, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @MemberConfigurationExpression__g0__t82_Ignore__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @MappingExpression__g2__t84_ReverseMap__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @MappingExpression__g2__t84_ForMember__g0(ptr %this, ptr %dest, ptr %opt) {
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

define ptr @MappingExpression__g2__t84_IgnoreAllMembers__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Profile__g0__t85_CreateMap__g2(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @MapperConfigurationExpression__g0__t86_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t86, ptr %t1, i32 0, i32 2
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

define void @MapperConfigurationExpression__g0__t86_AddProfile__g0(ptr %this, ptr %profile) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %profile, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.MapperConfigurationExpression__g0__t86, ptr %t2, i32 0, i32 2
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
  call void @glitch_retain_Conduit_Features_Profiles_Profile__g0__t157(ptr %t5)
  store ptr %t5, ptr %t17
  %t18 = add i64 %t9, 1
  store i64 %t18, ptr %t6
  ret void
exception_unwind:
  ret void
}

define void @MapperConfiguration__g0__t87_ctor(ptr %this, ptr %configure) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %configure, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.MapperConfiguration__g0__t87, ptr %t2, i32 0, i32 2
  %t4 = getelementptr %glitch.Mapper__g0__t80, ptr null, i32 1
  %t5 = ptrtoint ptr %t4 to i64
  %t6 = call ptr @glitch_calloc(i64 1, i64 %t5)
  %t7 = getelementptr inbounds %glitch.Mapper__g0__t80, ptr %t6, i32 0, i32 0
  store i64 1, ptr %t7
  %t8 = getelementptr inbounds %glitch.Mapper__g0__t80, ptr %t6, i32 0, i32 1
  store ptr @glitch_destroy_Mapper__g0__t80, ptr %t8
  %t9 = load ptr, ptr %t3
  call void @glitch_drop_Mapper__g0__t80(ptr %t9)
  store ptr %t6, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @MapperConfiguration__g0__t87_CreateMapper__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.MapperConfiguration__g0__t87, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  call void @glitch_retain_Mapper__g0__t80(ptr %t3)
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @JsonSerializer__g0__t88_Serialize__g0(ptr %this, ptr %value) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %value, ptr %t1
  %t2 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @JsonSerializer__g0__t88_Deserialize__g0(ptr %this, ptr %json) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %json, ptr %t1
  %t2 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @ConsoleLogger__g0__t90_LogError__g0__string(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [9 x i8] }, ptr @.str.39, i32 0, i32 2, i64 0), ptr %t2)
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t3)
  ret void
exception_unwind:
  ret void
}

define void @ConsoleLogger__g0__t90_LogWarning__g0__string(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [11 x i8] }, ptr @.str.40, i32 0, i32 2, i64 0), ptr %t2)
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t3)
  ret void
exception_unwind:
  ret void
}

define void @ConsoleLogger__g0__t90_LogInformation__g0__string(ptr %this, ptr %message) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @glitch_string_concat(ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.41, i32 0, i32 2, i64 0), ptr %t2)
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([4 x i8], ptr @.fmt_str, i64 0, i64 0), ptr %t3)
  ret void
exception_unwind:
  ret void
}

define void @LoggerFactory__g0__t93_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @LoggerFactory__g0__t93_CreateLogger__g0(ptr %this, ptr %categoryName) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %categoryName, ptr %t1
  %t2 = getelementptr %glitch.ConsoleLogger__g0__t90, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t5 = getelementptr inbounds %glitch.ConsoleLogger__g0__t90, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t5
  %t6 = getelementptr inbounds %glitch.ConsoleLogger__g0__t90, ptr %t4, i32 0, i32 1
  store ptr @glitch_destroy_ConsoleLogger__g0__t90, ptr %t6
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define void @LoggerFactory__g0__t93_AddProvider__g0(ptr %this, ptr %provider) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %provider, ptr %t1
  ret void
exception_unwind:
  ret void
}

define ptr @Path__g0__t94_Combine__g0(ptr %this, ptr %path1, ptr %path2) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path1, ptr %t1
  %t2 = alloca ptr
  store ptr %path2, ptr %t2
  %t3 = load ptr, ptr %t1
  %t4 = call ptr @glitch_string_concat(ptr %t3, ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.42, i32 0, i32 2, i64 0))
  %t5 = load ptr, ptr %t2
  %t6 = call ptr @glitch_string_concat(ptr %t4, ptr %t5)
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @Path__g0__t94_GetExtension__g0(ptr %this, ptr %path) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  ret ptr getelementptr inbounds ({ i64, i64, [5 x i8] }, ptr @.str.43, i32 0, i32 2, i64 0)
exception_unwind:
  ret ptr null
}

define ptr @Path__g0__t94_GetFileName__g0(ptr %this, ptr %path) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Directory__g0__t95_GetCurrentDirectory__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Directory__g0__t95_CreateDirectory__g0(ptr %this, ptr %path) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = load ptr, ptr %t1
  ret void
exception_unwind:
  ret void
}

define ptr @Directory__g0__t95_GetFiles__g0(ptr %this, ptr %path) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @FileStream__g0__t96_ctor(ptr %this, ptr %path, i32 %mode) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = alloca i32
  store i32 %mode, ptr %t2
  ret void
exception_unwind:
  ret void
}

define void @FileStream__g0__t96_Dispose__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define ptr @File__g0__t97_ReadAllText__g0(ptr %this, ptr %path) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @File__g0__t97_WriteAllText__g0(ptr %this, ptr %path, ptr %contents) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = alloca ptr
  store ptr %contents, ptr %t2
  %t3 = load ptr, ptr %t1
  %t4 = load ptr, ptr %t2
  ret void
exception_unwind:
  ret void
}

define i1 @File__g0__t97_Exists__g0(ptr %this, ptr %path) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %path, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = icmp ne ptr null, null
  ret i1 %t3
exception_unwind:
  ret i1 0
}

define void @SqlColumn__g0__t98_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @ColumnOptions__g0__t99_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @MSSqlServerSinkOptions__g0__t100_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define i1 @Conduit_Domain_Article__g0__t101_get_Favorited__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t1, i32 0, i32 10
  %t3 = load ptr, ptr %t2
  %t4 = getelementptr inbounds %glitch.list, ptr %t3, i32 0, i32 0
  %t5 = load i64, ptr %t4
  %t6 = trunc i64 %t5 to i32
  %t7 = trunc i64 0 to i32
  %t8 = icmp ne i32 %t6, %t7
  ret i1 %t8
exception_unwind:
  ret i1 0
}

define i32 @Conduit_Domain_Article__g0__t101_get_FavoritesCount__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t1, i32 0, i32 10
  %t3 = load ptr, ptr %t2
  %t4 = icmp eq ptr %t3, null
  %t5 = alloca i32
  br i1 %t4, label %conditional_true_0, label %conditional_false_1
conditional_true_0:
  store i32 0, ptr %t5
  br label %conditional_end_2
conditional_false_1:
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t6, i32 0, i32 10
  %t8 = load ptr, ptr %t7
  %t9 = getelementptr inbounds %glitch.list, ptr %t8, i32 0, i32 0
  %t10 = load i64, ptr %t9
  %t11 = trunc i64 %t10 to i32
  store i32 %t11, ptr %t5
  br label %conditional_end_2
conditional_end_2:
  %t12 = load i32, ptr %t5
  %t14 = icmp eq i32 %t12, 0
  %t13 = alloca i32
  br i1 %t14, label %coalesce_right_4, label %coalesce_left_3
coalesce_left_3:
  store i32 %t12, ptr %t13
  br label %coalesce_end_5
coalesce_right_4:
  %t15 = trunc i64 0 to i32
  store i32 %t15, ptr %t13
  br label %coalesce_end_5
coalesce_end_5:
  %t16 = load i32, ptr %t13
  ret i32 %t16
exception_unwind:
  ret i32 0
}

define ptr @Conduit_Domain_Article__g0__t101_get_TagList__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t1, i32 0, i32 9
  %t3 = load ptr, ptr %t2
  %t5 = getelementptr %glitch.delegate, ptr null, i32 1
  %t6 = ptrtoint ptr %t5 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t6)
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t7
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 1
  store ptr @glitch_lambda_0, ptr %t8
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 2
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 3
  store ptr null, ptr %t9
  store ptr null, ptr %t10
  call void @glitch_delegate_release(ptr %t4)
  %t12 = getelementptr %glitch.delegate, ptr null, i32 1
  %t13 = ptrtoint ptr %t12 to i64
  %t11 = call ptr @glitch_calloc(i64 1, i64 %t13)
  %t14 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 0
  store i64 1, ptr %t14
  %t15 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 1
  store ptr @glitch_lambda_1, ptr %t15
  %t16 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 2
  %t17 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 3
  store ptr null, ptr %t16
  store ptr null, ptr %t17
  call void @glitch_delegate_release(ptr %t11)
  %t18 = call ptr @ToList(ptr null)
  %t19 = load ptr, ptr @glitch_exception_pending
  %t20 = icmp ne ptr %t19, null
  br i1 %t20, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t18
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Articles_ArticleEnvelope__g0__t108_ctor(ptr %this, ptr %Article) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Article, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t2, i32 0, i32 2
  call void @glitch_retain_Conduit_Domain_Article__g0__t101(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Domain_Article__g0__t101(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Articles_ArticleExtensions__g0__t109_GetAllData__g0(ptr %this, ptr %articles) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %articles, ptr %t1
  %t2 = load ptr, ptr %t1
  %t4 = getelementptr %glitch.delegate, ptr null, i32 1
  %t5 = ptrtoint ptr %t4 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t5)
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 1
  store ptr @glitch_lambda_2, ptr %t7
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 2
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 3
  store ptr null, ptr %t8
  store ptr null, ptr %t9
  %t10 = call ptr @Include(ptr %t2, ptr %t3)
  %t11 = load ptr, ptr @glitch_exception_pending
  %t12 = icmp ne ptr %t11, null
  br i1 %t12, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t14 = getelementptr %glitch.delegate, ptr null, i32 1
  %t15 = ptrtoint ptr %t14 to i64
  %t13 = call ptr @glitch_calloc(i64 1, i64 %t15)
  %t16 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 0
  store i64 1, ptr %t16
  %t17 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 1
  store ptr @glitch_lambda_3, ptr %t17
  %t18 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 2
  %t19 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 3
  store ptr null, ptr %t18
  store ptr null, ptr %t19
  call void @glitch_delegate_release(ptr %t13)
  %t21 = getelementptr %glitch.delegate, ptr null, i32 1
  %t22 = ptrtoint ptr %t21 to i64
  %t20 = call ptr @glitch_calloc(i64 1, i64 %t22)
  %t23 = getelementptr inbounds %glitch.delegate, ptr %t20, i32 0, i32 0
  store i64 1, ptr %t23
  %t24 = getelementptr inbounds %glitch.delegate, ptr %t20, i32 0, i32 1
  store ptr @glitch_lambda_4, ptr %t24
  %t25 = getelementptr inbounds %glitch.delegate, ptr %t20, i32 0, i32 2
  %t26 = getelementptr inbounds %glitch.delegate, ptr %t20, i32 0, i32 3
  store ptr null, ptr %t25
  store ptr null, ptr %t26
  call void @glitch_delegate_release(ptr %t20)
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Articles_ArticlesController__g0__t110_ctor(ptr %this, ptr %mediator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %mediator, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Articles_ArticlesController__g0__t110_Get__g0__string_string_string_int_int_CancellationToken(ptr %this, ptr %tag, ptr %author, ptr %favorited, i32 %limit, i32 %offset, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %tag, ptr %t1
  %t2 = alloca ptr
  store ptr %author, ptr %t2
  %t3 = alloca ptr
  store ptr %favorited, ptr %t3
  %t4 = alloca i32
  store i32 %limit, ptr %t4
  %t5 = alloca i32
  store i32 %offset, ptr %t5
  %t6 = alloca ptr
  store ptr %cancellationToken, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  %t10 = load ptr, ptr %t1
  %t11 = load ptr, ptr %t2
  %t12 = load ptr, ptr %t3
  %t13 = load i32, ptr %t4
  %t14 = load i32, ptr %t5
  %t15 = load ptr, ptr %t6
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Articles_ArticlesController__g0__t110_GetFeed__g0(ptr %this, ptr %tag, ptr %author, ptr %favorited, i32 %limit, i32 %offset, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %tag, ptr %t1
  %t2 = alloca ptr
  store ptr %author, ptr %t2
  %t3 = alloca ptr
  store ptr %favorited, ptr %t3
  %t4 = alloca i32
  store i32 %limit, ptr %t4
  %t5 = alloca i32
  store i32 %offset, ptr %t5
  %t6 = alloca ptr
  store ptr %cancellationToken, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %t7, i32 0, i32 2
  %t9 = load ptr, ptr %t8
  %t10 = load ptr, ptr %t1
  %t11 = load ptr, ptr %t2
  %t12 = load ptr, ptr %t3
  %t13 = load i32, ptr %t4
  %t14 = load i32, ptr %t5
  %t15 = load ptr, ptr %t6
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Articles_ArticlesController__g0__t110_Get__g0__string_CancellationToken(ptr %this, ptr %slug, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %slug, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Articles_ArticlesController__g0__t110_Create__g0(ptr %this, ptr %command, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %command, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Articles_ArticlesController__g0__t110_Edit__g0(ptr %this, ptr %slug, ptr %model, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %slug, ptr %t1
  %t2 = alloca ptr
  store ptr %model, ptr %t2
  %t3 = alloca ptr
  store ptr %cancellationToken, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t5
  %t7 = load ptr, ptr %t2
  %t8 = load ptr, ptr %t1
  %t9 = load ptr, ptr %t3
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Articles_ArticlesController__g0__t110_Delete__g0(ptr %this, ptr %slug, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %slug, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticlesController__g0__t110, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Articles_ArticleData__g0__t113_ctor(ptr %this, ptr %Title, ptr %Description, ptr %Body, ptr %TagList) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Title, ptr %t1
  %t2 = alloca ptr
  store ptr %Description, ptr %t2
  %t3 = alloca ptr
  store ptr %Body, ptr %t3
  %t4 = alloca ptr
  store ptr %TagList, ptr %t4
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %t5, i32 0, i32 2
  %t7 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t7)
  %t8 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t8)
  store ptr %t7, ptr %t6
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %t9, i32 0, i32 3
  %t11 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t11)
  %t12 = load ptr, ptr %t10
  call void @glitch_string_release(ptr %t12)
  store ptr %t11, ptr %t10
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %t13, i32 0, i32 4
  %t15 = load ptr, ptr %t3
  call void @glitch_string_retain(ptr %t15)
  %t16 = load ptr, ptr %t14
  call void @glitch_string_release(ptr %t16)
  store ptr %t15, ptr %t14
  %t17 = load ptr, ptr %t0
  %t18 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %t17, i32 0, i32 5
  %t19 = load ptr, ptr %t4
  store ptr %t19, ptr %t18
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_ArticleDataValidator__g0__t114_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_5, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  call void @glitch_delegate_release(ptr %t1)
  %t9 = getelementptr %glitch.delegate, ptr null, i32 1
  %t10 = ptrtoint ptr %t9 to i64
  %t8 = call ptr @glitch_calloc(i64 1, i64 %t10)
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t8, i32 0, i32 0
  store i64 1, ptr %t11
  %t12 = getelementptr inbounds %glitch.delegate, ptr %t8, i32 0, i32 1
  store ptr @glitch_lambda_6, ptr %t12
  %t13 = getelementptr inbounds %glitch.delegate, ptr %t8, i32 0, i32 2
  %t14 = getelementptr inbounds %glitch.delegate, ptr %t8, i32 0, i32 3
  store ptr null, ptr %t13
  store ptr null, ptr %t14
  call void @glitch_delegate_release(ptr %t8)
  %t16 = getelementptr %glitch.delegate, ptr null, i32 1
  %t17 = ptrtoint ptr %t16 to i64
  %t15 = call ptr @glitch_calloc(i64 1, i64 %t17)
  %t18 = getelementptr inbounds %glitch.delegate, ptr %t15, i32 0, i32 0
  store i64 1, ptr %t18
  %t19 = getelementptr inbounds %glitch.delegate, ptr %t15, i32 0, i32 1
  store ptr @glitch_lambda_7, ptr %t19
  %t20 = getelementptr inbounds %glitch.delegate, ptr %t15, i32 0, i32 2
  %t21 = getelementptr inbounds %glitch.delegate, ptr %t15, i32 0, i32 3
  store ptr null, ptr %t20
  store ptr null, ptr %t21
  call void @glitch_delegate_release(ptr %t15)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_Command__g0__t115_ctor__ArticleData(ptr %this, ptr %Article) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Article, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t2, i32 0, i32 2
  call void @glitch_retain_Conduit_Features_Articles_ArticleData__g0__t113(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Features_Articles_ArticleData__g0__t113(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_Command__g0__t115_ctor__string(ptr %this, ptr %Slug) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Slug, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t2, i32 0, i32 3
  call void @glitch_string_retain(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_Command__g0__t115_ctor__Model_string(ptr %this, ptr %Model, ptr %Slug) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Model, ptr %t1
  %t2 = alloca ptr
  store ptr %Slug, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t3, i32 0, i32 4
  call void @glitch_retain_Conduit_Features_Comments_Model__g0__t132(ptr null)
  %t5 = load ptr, ptr %t4
  call void @glitch_drop_Conduit_Features_Comments_Model__g0__t132(ptr %t5)
  store ptr null, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t6, i32 0, i32 3
  call void @glitch_string_retain(ptr null)
  %t8 = load ptr, ptr %t7
  call void @glitch_string_release(ptr %t8)
  store ptr null, ptr %t7
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_CommandValidator__g0__t116_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_8, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  call void @glitch_delegate_release(ptr %t1)
  %t8 = getelementptr %glitch.Conduit_Features_Articles_ArticleDataValidator__g0__t114, ptr null, i32 1
  %t9 = ptrtoint ptr %t8 to i64
  %t10 = call ptr @glitch_calloc(i64 1, i64 %t9)
  %t11 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleDataValidator__g0__t114, ptr %t10, i32 0, i32 0
  store i64 1, ptr %t11
  %t12 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleDataValidator__g0__t114, ptr %t10, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Articles_ArticleDataValidator__g0__t114, ptr %t12
  call void @Conduit_Features_Articles_ArticleDataValidator__g0__t114_ctor(ptr %t10)
  %t13 = load ptr, ptr @glitch_exception_pending
  %t14 = icmp ne ptr %t13, null
  br i1 %t14, label %exception_unwind, label %call_continue_0
call_continue_0:
  call void @glitch_drop_Conduit_Features_Articles_ArticleDataValidator__g0__t114(ptr %t10)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_Handler__g0__t117_ctor__ConduitContext_ICurrentUserAccessor(ptr %this, ptr %context, ptr %currentUserAccessor) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %currentUserAccessor, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t6)
  store ptr %t5, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t9)
  %t10 = load ptr, ptr %t8
  call void @glitch_drop_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t10)
  store ptr %t9, ptr %t8
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_Handler__g0__t117_ctor__ConduitContext(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Articles_Handler__g0__t117_Handle__g0__Command_CancellationToken(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = alloca ptr
  store ptr null, ptr %t6
  %t7 = alloca ptr
  store ptr null, ptr %t7
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t8, i32 0, i32 2
  %t10 = load ptr, ptr %t9
  %t11 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t10, i32 0, i32 12
  %t12 = load ptr, ptr %t11
  %t14 = getelementptr %glitch.delegate, ptr null, i32 1
  %t15 = ptrtoint ptr %t14 to i64
  %t13 = call ptr @glitch_calloc(i64 1, i64 %t15)
  %t16 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 0
  store i64 1, ptr %t16
  %t17 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 1
  store ptr @glitch_lambda_9, ptr %t17
  %t18 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 2
  %t19 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 3
  %t20 = getelementptr %glitch.lambda.9.env, ptr null, i32 1
  %t21 = ptrtoint ptr %t20 to i64
  %t22 = call ptr @glitch_calloc(i64 1, i64 %t21)
  %t23 = load ptr, ptr %t0
  %t24 = getelementptr inbounds %glitch.lambda.9.env, ptr %t22, i32 0, i32 0
  store ptr %t23, ptr %t24
  store ptr %t22, ptr %t18
  store ptr @glitch_lambda_9_destroy, ptr %t19
  call void @glitch_delegate_release(ptr %t13)
  %t27 = load ptr, ptr %t2
  %t28 = call ptr @glitch_task_get_result_ptr(ptr null)
  store ptr %t28, ptr %t3
  %t29 = call ptr @glitch_calloc(i64 1, i64 24)
  %t30 = call ptr @glitch_calloc(i64 4, i64 8)
  %t31 = getelementptr inbounds %glitch.list, ptr %t29, i32 0, i32 1
  store i64 4, ptr %t31
  %t32 = getelementptr inbounds %glitch.list, ptr %t29, i32 0, i32 2
  store ptr %t30, ptr %t32
  %t33 = load ptr, ptr %t4
  %t34 = icmp eq ptr %t33, null
  br i1 %t34, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t35 = getelementptr inbounds %glitch.list, ptr %t33, i32 0, i32 0
  %t36 = getelementptr inbounds %glitch.list, ptr %t33, i32 0, i32 2
  %t37 = load i64, ptr %t35
  %t38 = load ptr, ptr %t36
  %t39 = alloca i64
  store i64 0, ptr %t39
  br label %element_drop_loop_2
element_drop_loop_2:
  %t40 = load i64, ptr %t39
  %t41 = icmp ult i64 %t40, %t37
  br i1 %t41, label %element_drop_body_3, label %element_drop_done_4
element_drop_body_3:
  %t42 = getelementptr inbounds ptr, ptr %t38, i64 %t40
  %t43 = load ptr, ptr %t42
  call void @glitch_drop_Conduit_Domain_Tag__g0__t107(ptr %t43)
  %t44 = add i64 %t40, 1
  store i64 %t44, ptr %t39
  br label %element_drop_loop_2
element_drop_done_4:
  call void @glitch_free(ptr %t38)
  call void @glitch_free(ptr %t33)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t29, ptr %t4
  %t45 = load ptr, ptr %t1
  %t46 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t45, i32 0, i32 2
  %t47 = load ptr, ptr %t46
  %t48 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %t47, i32 0, i32 5
  %t49 = load ptr, ptr %t48
  %t51 = icmp eq ptr %t49, null
  %t50 = alloca ptr
  br i1 %t51, label %coalesce_right_6, label %coalesce_left_5
coalesce_left_5:
  store ptr %t49, ptr %t50
  br label %coalesce_end_7
coalesce_right_6:
  store ptr null, ptr %t50
  br label %coalesce_end_7
coalesce_end_7:
  %t52 = load ptr, ptr %t50
  %t53 = getelementptr inbounds %glitch.array, ptr %t52, i32 0, i32 0
  %t54 = getelementptr inbounds %glitch.array, ptr %t52, i32 0, i32 1
  %t55 = load i64, ptr %t53
  %t56 = load ptr, ptr %t54
  %t57 = alloca i64
  %t58 = alloca ptr
  store i64 0, ptr %t57
  br label %foreach_condition_8
foreach_condition_8:
  %t59 = load i64, ptr %t57
  %t60 = icmp ult i64 %t59, %t55
  br i1 %t60, label %foreach_body_9, label %foreach_end_11
foreach_body_9:
  %t61 = getelementptr inbounds ptr, ptr %t56, i64 %t59
  %t62 = load ptr, ptr %t61
  store ptr %t62, ptr %t58
  %t63 = load ptr, ptr %t0
  %t64 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t63, i32 0, i32 2
  %t65 = load ptr, ptr %t64
  %t66 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t65, i32 0, i32 13
  %t67 = load ptr, ptr %t66
  %t68 = load ptr, ptr %t58
  %t69 = call ptr @glitch_task_get_result_ptr(ptr null)
  store ptr %t69, ptr %t6
  %t70 = load ptr, ptr %t6
  %t71 = icmp eq ptr %t70, null
  br i1 %t71, label %if_then_12, label %if_else_13
if_then_12:
  %t72 = getelementptr %glitch.Conduit_Domain_Tag__g0__t107, ptr null, i32 1
  %t73 = ptrtoint ptr %t72 to i64
  %t74 = call ptr @glitch_calloc(i64 1, i64 %t73)
  %t75 = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %t74, i32 0, i32 0
  store i64 1, ptr %t75
  %t76 = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %t74, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Domain_Tag__g0__t107, ptr %t76
  %t77 = load ptr, ptr %t58
  call void @glitch_string_retain(ptr %t77)
  %t78 = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %t74, i32 0, i32 2
  store ptr %t77, ptr %t78
  store ptr %t74, ptr %t6
  %t79 = load ptr, ptr %t6
  %t80 = load ptr, ptr %t2
  %t81 = call ptr @AddAsync(ptr %t79, ptr %t80)
  %t82 = load ptr, ptr @glitch_exception_pending
  %t83 = icmp ne ptr %t82, null
  br i1 %t83, label %exception_unwind, label %call_continue_15
call_continue_15:
  %t84 = call ptr @glitch_task_get_result_ptr(ptr %t81)
  %t85 = load ptr, ptr %t2
  %t86 = call ptr @SaveChangesAsync__object(ptr %t85)
  %t87 = load ptr, ptr @glitch_exception_pending
  %t88 = icmp ne ptr %t87, null
  br i1 %t88, label %exception_unwind, label %call_continue_16
call_continue_16:
  %t89 = call ptr @glitch_task_get_result_ptr(ptr %t86)
  br label %if_end_14
if_else_13:
  br label %if_end_14
if_end_14:
  %t90 = load ptr, ptr %t4
  %t91 = load ptr, ptr %t6
  %t92 = getelementptr inbounds %glitch.list, ptr %t90, i32 0, i32 0
  %t93 = getelementptr inbounds %glitch.list, ptr %t90, i32 0, i32 1
  %t94 = getelementptr inbounds %glitch.list, ptr %t90, i32 0, i32 2
  %t95 = load i64, ptr %t92
  %t96 = load i64, ptr %t93
  %t97 = load ptr, ptr %t94
  %t98 = icmp eq i64 %t95, %t96
  br i1 %t98, label %list_grow_17, label %list_ready_18
list_grow_17:
  %t99 = mul i64 %t96, 2
  %t100 = mul i64 %t99, 8
  %t101 = call ptr @glitch_realloc(ptr %t97, i64 %t100)
  store i64 %t99, ptr %t93
  store ptr %t101, ptr %t94
  br label %list_ready_18
list_ready_18:
  %t102 = load ptr, ptr %t94
  %t103 = getelementptr inbounds ptr, ptr %t102, i64 %t95
  call void @glitch_retain_Conduit_Domain_Tag__g0__t107(ptr %t91)
  store ptr %t91, ptr %t103
  %t104 = add i64 %t95, 1
  store i64 %t104, ptr %t92
  br label %foreach_advance_10
foreach_advance_10:
  %t105 = load i64, ptr %t57
  %t106 = add i64 %t105, 1
  store i64 %t106, ptr %t57
  br label %foreach_condition_8
foreach_end_11:
  %t107 = getelementptr %glitch.Conduit_Domain_Article__g0__t101, ptr null, i32 1
  %t108 = ptrtoint ptr %t107 to i64
  %t109 = call ptr @glitch_calloc(i64 1, i64 %t108)
  %t110 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t109, i32 0, i32 0
  store i64 1, ptr %t110
  %t111 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t109, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Domain_Article__g0__t101, ptr %t111
  %t112 = load ptr, ptr %t3
  call void @glitch_retain_Conduit_Domain_Person__g0__t106(ptr %t112)
  %t113 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t109, i32 0, i32 7
  store ptr %t112, ptr %t113
  %t114 = load ptr, ptr %t1
  %t115 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t114, i32 0, i32 2
  %t116 = load ptr, ptr %t115
  %t117 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %t116, i32 0, i32 4
  %t118 = load ptr, ptr %t117
  call void @glitch_string_retain(ptr %t118)
  %t119 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t109, i32 0, i32 6
  store ptr %t118, ptr %t119
  %t120 = getelementptr inbounds %glitch.DateTime__g0__t17, ptr null, i32 0, i32 3
  %t121 = load ptr, ptr %t120
  call void @glitch_string_retain(ptr %t121)
  %t122 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t109, i32 0, i32 11
  store ptr %t121, ptr %t122
  %t123 = getelementptr inbounds %glitch.DateTime__g0__t17, ptr null, i32 0, i32 3
  %t124 = load ptr, ptr %t123
  call void @glitch_string_retain(ptr %t124)
  %t125 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t109, i32 0, i32 12
  store ptr %t124, ptr %t125
  %t126 = load ptr, ptr %t1
  %t127 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t126, i32 0, i32 2
  %t128 = load ptr, ptr %t127
  %t129 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %t128, i32 0, i32 3
  %t130 = load ptr, ptr %t129
  call void @glitch_string_retain(ptr %t130)
  %t131 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t109, i32 0, i32 5
  store ptr %t130, ptr %t131
  %t132 = load ptr, ptr %t1
  %t133 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t132, i32 0, i32 2
  %t134 = load ptr, ptr %t133
  %t135 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %t134, i32 0, i32 2
  %t136 = load ptr, ptr %t135
  call void @glitch_string_retain(ptr %t136)
  %t137 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t109, i32 0, i32 4
  store ptr %t136, ptr %t137
  %t138 = load ptr, ptr %t1
  %t139 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t138, i32 0, i32 2
  %t140 = load ptr, ptr %t139
  %t141 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleData__g0__t113, ptr %t140, i32 0, i32 2
  %t142 = load ptr, ptr %t141
  %t143 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t109, i32 0, i32 3
  store ptr null, ptr %t143
  %t144 = load ptr, ptr %t7
  call void @glitch_drop_Conduit_Domain_Article__g0__t101(ptr %t144)
  store ptr %t109, ptr %t7
  %t145 = load ptr, ptr %t7
  %t146 = load ptr, ptr %t2
  %t147 = call ptr @AddAsync(ptr %t145, ptr %t146)
  %t148 = load ptr, ptr @glitch_exception_pending
  %t149 = icmp ne ptr %t148, null
  br i1 %t149, label %exception_unwind, label %call_continue_19
call_continue_19:
  %t150 = call ptr @glitch_task_get_result_ptr(ptr %t147)
  %t151 = load ptr, ptr %t4
  %t153 = getelementptr %glitch.delegate, ptr null, i32 1
  %t154 = ptrtoint ptr %t153 to i64
  %t152 = call ptr @glitch_calloc(i64 1, i64 %t154)
  %t155 = getelementptr inbounds %glitch.delegate, ptr %t152, i32 0, i32 0
  store i64 1, ptr %t155
  %t156 = getelementptr inbounds %glitch.delegate, ptr %t152, i32 0, i32 1
  store ptr @glitch_lambda_10, ptr %t156
  %t157 = getelementptr inbounds %glitch.delegate, ptr %t152, i32 0, i32 2
  %t158 = getelementptr inbounds %glitch.delegate, ptr %t152, i32 0, i32 3
  %t159 = getelementptr %glitch.lambda.10.env, ptr null, i32 1
  %t160 = ptrtoint ptr %t159 to i64
  %t161 = call ptr @glitch_calloc(i64 1, i64 %t160)
  %t162 = load ptr, ptr %t7
  call void @glitch_retain_Article(ptr %t162)
  %t163 = getelementptr inbounds %glitch.lambda.10.env, ptr %t161, i32 0, i32 0
  store ptr %t162, ptr %t163
  store ptr %t161, ptr %t157
  store ptr @glitch_lambda_10_destroy, ptr %t158
  call void @glitch_delegate_release(ptr %t152)
  %t166 = load ptr, ptr %t2
  %t167 = call ptr @AddRangeAsync(ptr null, ptr %t166)
  %t168 = load ptr, ptr @glitch_exception_pending
  %t169 = icmp ne ptr %t168, null
  br i1 %t169, label %exception_unwind, label %call_continue_20
call_continue_20:
  %t170 = call ptr @glitch_task_get_result_ptr(ptr %t167)
  %t171 = load ptr, ptr %t2
  %t172 = call ptr @SaveChangesAsync__object(ptr %t171)
  %t173 = load ptr, ptr @glitch_exception_pending
  %t174 = icmp ne ptr %t173, null
  br i1 %t174, label %exception_unwind, label %call_continue_21
call_continue_21:
  %t175 = call ptr @glitch_task_get_result_ptr(ptr %t172)
  %t176 = getelementptr %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr null, i32 1
  %t177 = ptrtoint ptr %t176 to i64
  %t178 = call ptr @glitch_calloc(i64 1, i64 %t177)
  %t179 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t178, i32 0, i32 0
  store i64 1, ptr %t179
  %t180 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t178, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t180
  %t181 = load ptr, ptr %t7
  call void @Conduit_Features_Articles_ArticleEnvelope__g0__t108_ctor(ptr %t178, ptr %t181)
  %t182 = load ptr, ptr @glitch_exception_pending
  %t183 = icmp ne ptr %t182, null
  br i1 %t183, label %exception_unwind, label %call_continue_22
call_continue_22:
  %t184 = load ptr, ptr %t7
  call void @glitch_drop_Conduit_Domain_Article__g0__t101(ptr %t184)
  %t185 = load ptr, ptr %t4
  %t186 = icmp eq ptr %t185, null
  br i1 %t186, label %collection_release_done_24, label %collection_release_23
collection_release_23:
  %t187 = getelementptr inbounds %glitch.list, ptr %t185, i32 0, i32 0
  %t188 = getelementptr inbounds %glitch.list, ptr %t185, i32 0, i32 2
  %t189 = load i64, ptr %t187
  %t190 = load ptr, ptr %t188
  %t191 = alloca i64
  store i64 0, ptr %t191
  br label %element_drop_loop_25
element_drop_loop_25:
  %t192 = load i64, ptr %t191
  %t193 = icmp ult i64 %t192, %t189
  br i1 %t193, label %element_drop_body_26, label %element_drop_done_27
element_drop_body_26:
  %t194 = getelementptr inbounds ptr, ptr %t190, i64 %t192
  %t195 = load ptr, ptr %t194
  call void @glitch_drop_Conduit_Domain_Tag__g0__t107(ptr %t195)
  %t196 = add i64 %t192, 1
  store i64 %t196, ptr %t191
  br label %element_drop_loop_25
element_drop_done_27:
  call void @glitch_free(ptr %t190)
  call void @glitch_free(ptr %t185)
  br label %collection_release_done_24
collection_release_done_24:
  ret ptr %t178
exception_unwind:
  %t197 = load ptr, ptr %t7
  call void @glitch_drop_Conduit_Domain_Article__g0__t101(ptr %t197)
  %t198 = load ptr, ptr %t4
  %t199 = icmp eq ptr %t198, null
  br i1 %t199, label %collection_release_done_29, label %collection_release_28
collection_release_28:
  %t200 = getelementptr inbounds %glitch.list, ptr %t198, i32 0, i32 0
  %t201 = getelementptr inbounds %glitch.list, ptr %t198, i32 0, i32 2
  %t202 = load i64, ptr %t200
  %t203 = load ptr, ptr %t201
  %t204 = alloca i64
  store i64 0, ptr %t204
  br label %element_drop_loop_30
element_drop_loop_30:
  %t205 = load i64, ptr %t204
  %t206 = icmp ult i64 %t205, %t202
  br i1 %t206, label %element_drop_body_31, label %element_drop_done_32
element_drop_body_31:
  %t207 = getelementptr inbounds ptr, ptr %t203, i64 %t205
  %t208 = load ptr, ptr %t207
  call void @glitch_drop_Conduit_Domain_Tag__g0__t107(ptr %t208)
  %t209 = add i64 %t205, 1
  store i64 %t209, ptr %t204
  br label %element_drop_loop_30
element_drop_done_32:
  call void @glitch_free(ptr %t203)
  call void @glitch_free(ptr %t198)
  br label %collection_release_done_29
collection_release_done_29:
  ret ptr null
}

define ptr @Conduit_Features_Articles_Handler__g0__t117_GetArticleTagsToCreate__g0(ptr %this, ptr %article, ptr %articleTagList) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %article, ptr %t1
  %t2 = alloca ptr
  store ptr %articleTagList, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
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
  %t10 = load ptr, ptr %t3
  %t11 = icmp eq ptr %t10, null
  br i1 %t11, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t12 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 0
  %t13 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 2
  %t14 = load i64, ptr %t12
  %t15 = load ptr, ptr %t13
  %t16 = alloca i64
  store i64 0, ptr %t16
  br label %element_drop_loop_2
element_drop_loop_2:
  %t17 = load i64, ptr %t16
  %t18 = icmp ult i64 %t17, %t14
  br i1 %t18, label %element_drop_body_3, label %element_drop_done_4
element_drop_body_3:
  %t19 = getelementptr inbounds ptr, ptr %t15, i64 %t17
  %t20 = load ptr, ptr %t19
  call void @glitch_drop_Conduit_Domain_ArticleTag__g0__t103(ptr %t20)
  %t21 = add i64 %t17, 1
  store i64 %t21, ptr %t16
  br label %element_drop_loop_2
element_drop_done_4:
  call void @glitch_free(ptr %t15)
  call void @glitch_free(ptr %t10)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t6, ptr %t3
  %t22 = load ptr, ptr %t2
  %t23 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 0
  %t24 = getelementptr inbounds %glitch.list, ptr %t22, i32 0, i32 2
  %t25 = load i64, ptr %t23
  %t26 = load ptr, ptr %t24
  %t27 = alloca i64
  %t28 = alloca ptr
  store i64 0, ptr %t27
  br label %foreach_condition_5
foreach_condition_5:
  %t29 = load i64, ptr %t27
  %t30 = icmp ult i64 %t29, %t25
  br i1 %t30, label %foreach_body_6, label %foreach_end_8
foreach_body_6:
  %t31 = getelementptr inbounds ptr, ptr %t26, i64 %t29
  %t32 = load ptr, ptr %t31
  store ptr %t32, ptr %t28
  %t33 = load ptr, ptr %t1
  %t34 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t33, i32 0, i32 9
  %t35 = load ptr, ptr %t34
  %t36 = icmp eq ptr %t35, null
  %t37 = alloca ptr
  br i1 %t36, label %conditional_true_9, label %conditional_false_10
conditional_true_9:
  store ptr null, ptr %t37
  br label %conditional_end_11
conditional_false_10:
  %t38 = load ptr, ptr %t1
  %t39 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t38, i32 0, i32 9
  %t40 = load ptr, ptr %t39
  %t42 = getelementptr %glitch.delegate, ptr null, i32 1
  %t43 = ptrtoint ptr %t42 to i64
  %t41 = call ptr @glitch_calloc(i64 1, i64 %t43)
  %t44 = getelementptr inbounds %glitch.delegate, ptr %t41, i32 0, i32 0
  store i64 1, ptr %t44
  %t45 = getelementptr inbounds %glitch.delegate, ptr %t41, i32 0, i32 1
  store ptr @glitch_lambda_11, ptr %t45
  %t46 = getelementptr inbounds %glitch.delegate, ptr %t41, i32 0, i32 2
  %t47 = getelementptr inbounds %glitch.delegate, ptr %t41, i32 0, i32 3
  %t48 = getelementptr %glitch.lambda.11.env, ptr null, i32 1
  %t49 = ptrtoint ptr %t48 to i64
  %t50 = call ptr @glitch_calloc(i64 1, i64 %t49)
  %t51 = load ptr, ptr %t28
  %t52 = getelementptr inbounds %glitch.lambda.11.env, ptr %t50, i32 0, i32 0
  store ptr %t51, ptr %t52
  store ptr %t50, ptr %t46
  store ptr @glitch_lambda_11_destroy, ptr %t47
  call void @glitch_delegate_release(ptr %t41)
  store ptr null, ptr %t37
  br label %conditional_end_11
conditional_end_11:
  %t55 = load ptr, ptr %t37
  store ptr %t55, ptr %t5
  %t56 = load ptr, ptr %t5
  %t57 = icmp eq ptr %t56, null
  br i1 %t57, label %if_then_12, label %if_else_13
if_then_12:
  %t58 = getelementptr %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr null, i32 1
  %t59 = ptrtoint ptr %t58 to i64
  %t60 = call ptr @glitch_calloc(i64 1, i64 %t59)
  %t61 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t60, i32 0, i32 0
  store i64 1, ptr %t61
  %t62 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t60, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Domain_ArticleTag__g0__t103, ptr %t62
  %t63 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Domain_Article__g0__t101(ptr %t63)
  %t64 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t60, i32 0, i32 3
  store ptr %t63, ptr %t64
  %t65 = load ptr, ptr %t1
  %t66 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t65, i32 0, i32 2
  %t67 = load i32, ptr %t66
  %t68 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t60, i32 0, i32 2
  store i32 %t67, ptr %t68
  %t69 = getelementptr %glitch.Conduit_Domain_Tag__g0__t107, ptr null, i32 1
  %t70 = ptrtoint ptr %t69 to i64
  %t71 = call ptr @glitch_calloc(i64 1, i64 %t70)
  %t72 = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %t71, i32 0, i32 0
  store i64 1, ptr %t72
  %t73 = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %t71, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Domain_Tag__g0__t107, ptr %t73
  %t74 = load ptr, ptr %t28
  call void @glitch_string_retain(ptr %t74)
  %t75 = getelementptr inbounds %glitch.Conduit_Domain_Tag__g0__t107, ptr %t71, i32 0, i32 2
  store ptr %t74, ptr %t75
  %t76 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t60, i32 0, i32 5
  store ptr %t71, ptr %t76
  %t77 = load ptr, ptr %t28
  call void @glitch_string_retain(ptr %t77)
  %t78 = getelementptr inbounds %glitch.Conduit_Domain_ArticleTag__g0__t103, ptr %t60, i32 0, i32 4
  store ptr %t77, ptr %t78
  store ptr %t60, ptr %t5
  %t79 = load ptr, ptr %t3
  %t80 = load ptr, ptr %t5
  %t81 = getelementptr inbounds %glitch.list, ptr %t79, i32 0, i32 0
  %t82 = getelementptr inbounds %glitch.list, ptr %t79, i32 0, i32 1
  %t83 = getelementptr inbounds %glitch.list, ptr %t79, i32 0, i32 2
  %t84 = load i64, ptr %t81
  %t85 = load i64, ptr %t82
  %t86 = load ptr, ptr %t83
  %t87 = icmp eq i64 %t84, %t85
  br i1 %t87, label %list_grow_15, label %list_ready_16
list_grow_15:
  %t88 = mul i64 %t85, 2
  %t89 = mul i64 %t88, 8
  %t90 = call ptr @glitch_realloc(ptr %t86, i64 %t89)
  store i64 %t88, ptr %t82
  store ptr %t90, ptr %t83
  br label %list_ready_16
list_ready_16:
  %t91 = load ptr, ptr %t83
  %t92 = getelementptr inbounds ptr, ptr %t91, i64 %t84
  call void @glitch_retain_Conduit_Domain_ArticleTag__g0__t103(ptr %t80)
  store ptr %t80, ptr %t92
  %t93 = add i64 %t84, 1
  store i64 %t93, ptr %t81
  br label %if_end_14
if_else_13:
  br label %if_end_14
if_end_14:
  br label %foreach_advance_7
foreach_advance_7:
  %t94 = load i64, ptr %t27
  %t95 = add i64 %t94, 1
  store i64 %t95, ptr %t27
  br label %foreach_condition_5
foreach_end_8:
  %t96 = load ptr, ptr %t3
  ret ptr %t96
exception_unwind:
  %t97 = load ptr, ptr %t3
  %t98 = icmp eq ptr %t97, null
  br i1 %t98, label %collection_release_done_18, label %collection_release_17
collection_release_17:
  %t99 = getelementptr inbounds %glitch.list, ptr %t97, i32 0, i32 0
  %t100 = getelementptr inbounds %glitch.list, ptr %t97, i32 0, i32 2
  %t101 = load i64, ptr %t99
  %t102 = load ptr, ptr %t100
  %t103 = alloca i64
  store i64 0, ptr %t103
  br label %element_drop_loop_19
element_drop_loop_19:
  %t104 = load i64, ptr %t103
  %t105 = icmp ult i64 %t104, %t101
  br i1 %t105, label %element_drop_body_20, label %element_drop_done_21
element_drop_body_20:
  %t106 = getelementptr inbounds ptr, ptr %t102, i64 %t104
  %t107 = load ptr, ptr %t106
  call void @glitch_drop_Conduit_Domain_ArticleTag__g0__t103(ptr %t107)
  %t108 = add i64 %t104, 1
  store i64 %t108, ptr %t103
  br label %element_drop_loop_19
element_drop_done_21:
  call void @glitch_free(ptr %t102)
  call void @glitch_free(ptr %t97)
  br label %collection_release_done_18
collection_release_done_18:
  ret ptr null
}

define ptr @Conduit_Features_Articles_Handler__g0__t117_GetArticleTagsToDelete__g0(ptr %this, ptr %article, ptr %articleTagList) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %article, ptr %t1
  %t2 = alloca ptr
  store ptr %articleTagList, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
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
  %t10 = load ptr, ptr %t3
  %t11 = icmp eq ptr %t10, null
  br i1 %t11, label %collection_release_done_1, label %collection_release_0
collection_release_0:
  %t12 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 0
  %t13 = getelementptr inbounds %glitch.list, ptr %t10, i32 0, i32 2
  %t14 = load i64, ptr %t12
  %t15 = load ptr, ptr %t13
  %t16 = alloca i64
  store i64 0, ptr %t16
  br label %element_drop_loop_2
element_drop_loop_2:
  %t17 = load i64, ptr %t16
  %t18 = icmp ult i64 %t17, %t14
  br i1 %t18, label %element_drop_body_3, label %element_drop_done_4
element_drop_body_3:
  %t19 = getelementptr inbounds ptr, ptr %t15, i64 %t17
  %t20 = load ptr, ptr %t19
  call void @glitch_drop_Conduit_Domain_ArticleTag__g0__t103(ptr %t20)
  %t21 = add i64 %t17, 1
  store i64 %t21, ptr %t16
  br label %element_drop_loop_2
element_drop_done_4:
  call void @glitch_free(ptr %t15)
  call void @glitch_free(ptr %t10)
  br label %collection_release_done_1
collection_release_done_1:
  store ptr %t6, ptr %t3
  %t22 = load ptr, ptr %t1
  %t23 = getelementptr inbounds %glitch.Conduit_Domain_Article__g0__t101, ptr %t22, i32 0, i32 9
  %t24 = load ptr, ptr %t23
  %t25 = getelementptr inbounds %glitch.list, ptr %t24, i32 0, i32 0
  %t26 = getelementptr inbounds %glitch.list, ptr %t24, i32 0, i32 2
  %t27 = load i64, ptr %t25
  %t28 = load ptr, ptr %t26
  %t29 = alloca i64
  %t30 = alloca ptr
  store i64 0, ptr %t29
  br label %foreach_condition_5
foreach_condition_5:
  %t31 = load i64, ptr %t29
  %t32 = icmp ult i64 %t31, %t27
  br i1 %t32, label %foreach_body_6, label %foreach_end_8
foreach_body_6:
  %t33 = getelementptr inbounds ptr, ptr %t28, i64 %t31
  %t34 = load ptr, ptr %t33
  store ptr %t34, ptr %t30
  %t35 = load ptr, ptr %t2
  %t37 = getelementptr %glitch.delegate, ptr null, i32 1
  %t38 = ptrtoint ptr %t37 to i64
  %t36 = call ptr @glitch_calloc(i64 1, i64 %t38)
  %t39 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 0
  store i64 1, ptr %t39
  %t40 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 1
  store ptr @glitch_lambda_12, ptr %t40
  %t41 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 2
  %t42 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 3
  %t43 = getelementptr %glitch.lambda.12.env, ptr null, i32 1
  %t44 = ptrtoint ptr %t43 to i64
  %t45 = call ptr @glitch_calloc(i64 1, i64 %t44)
  %t46 = load ptr, ptr %t30
  %t47 = getelementptr inbounds %glitch.lambda.12.env, ptr %t45, i32 0, i32 0
  store ptr %t46, ptr %t47
  store ptr %t45, ptr %t41
  store ptr @glitch_lambda_12_destroy, ptr %t42
  call void @glitch_delegate_release(ptr %t36)
  store ptr null, ptr %t5
  %t50 = load ptr, ptr %t5
  %t51 = icmp eq ptr %t50, null
  br i1 %t51, label %if_then_9, label %if_else_10
if_then_9:
  %t52 = load ptr, ptr %t3
  %t53 = load ptr, ptr %t30
  %t54 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 0
  %t55 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 1
  %t56 = getelementptr inbounds %glitch.list, ptr %t52, i32 0, i32 2
  %t57 = load i64, ptr %t54
  %t58 = load i64, ptr %t55
  %t59 = load ptr, ptr %t56
  %t60 = icmp eq i64 %t57, %t58
  br i1 %t60, label %list_grow_12, label %list_ready_13
list_grow_12:
  %t61 = mul i64 %t58, 2
  %t62 = mul i64 %t61, 8
  %t63 = call ptr @glitch_realloc(ptr %t59, i64 %t62)
  store i64 %t61, ptr %t55
  store ptr %t63, ptr %t56
  br label %list_ready_13
list_ready_13:
  %t64 = load ptr, ptr %t56
  %t65 = getelementptr inbounds ptr, ptr %t64, i64 %t57
  call void @glitch_retain_Conduit_Domain_ArticleTag__g0__t103(ptr %t53)
  store ptr %t53, ptr %t65
  %t66 = add i64 %t57, 1
  store i64 %t66, ptr %t54
  br label %if_end_11
if_else_10:
  br label %if_end_11
if_end_11:
  br label %foreach_advance_7
foreach_advance_7:
  %t67 = load i64, ptr %t29
  %t68 = add i64 %t67, 1
  store i64 %t68, ptr %t29
  br label %foreach_condition_5
foreach_end_8:
  %t69 = load ptr, ptr %t3
  ret ptr %t69
exception_unwind:
  %t70 = load ptr, ptr %t3
  %t71 = icmp eq ptr %t70, null
  br i1 %t71, label %collection_release_done_15, label %collection_release_14
collection_release_14:
  %t72 = getelementptr inbounds %glitch.list, ptr %t70, i32 0, i32 0
  %t73 = getelementptr inbounds %glitch.list, ptr %t70, i32 0, i32 2
  %t74 = load i64, ptr %t72
  %t75 = load ptr, ptr %t73
  %t76 = alloca i64
  store i64 0, ptr %t76
  br label %element_drop_loop_16
element_drop_loop_16:
  %t77 = load i64, ptr %t76
  %t78 = icmp ult i64 %t77, %t74
  br i1 %t78, label %element_drop_body_17, label %element_drop_done_18
element_drop_body_17:
  %t79 = getelementptr inbounds ptr, ptr %t75, i64 %t77
  %t80 = load ptr, ptr %t79
  call void @glitch_drop_Conduit_Domain_ArticleTag__g0__t103(ptr %t80)
  %t81 = add i64 %t77, 1
  store i64 %t81, ptr %t76
  br label %element_drop_loop_16
element_drop_done_18:
  call void @glitch_free(ptr %t75)
  call void @glitch_free(ptr %t70)
  br label %collection_release_done_15
collection_release_done_15:
  ret ptr null
}

define void @Conduit_Features_Articles_QueryHandler__g0__t119_ctor__ConduitContext(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_QueryHandler__g0__t119_ctor__ConduitContext_ICurrentUserAccessor(ptr %this, ptr %context, ptr %currentUserAccessor) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %currentUserAccessor, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t6)
  store ptr %t5, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t9)
  %t10 = load ptr, ptr %t8
  call void @glitch_drop_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t10)
  store ptr %t9, ptr %t8
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Articles_QueryHandler__g0__t119_Handle__g0__Command_CancellationToken(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t5 = getelementptr %glitch.delegate, ptr null, i32 1
  %t6 = ptrtoint ptr %t5 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t6)
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t7
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 1
  store ptr @glitch_lambda_13, ptr %t8
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 2
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 3
  %t11 = getelementptr %glitch.lambda.13.env, ptr null, i32 1
  %t12 = ptrtoint ptr %t11 to i64
  %t13 = call ptr @glitch_calloc(i64 1, i64 %t12)
  %t14 = load ptr, ptr %t1
  %t15 = getelementptr inbounds %glitch.lambda.13.env, ptr %t13, i32 0, i32 0
  store ptr %t14, ptr %t15
  store ptr %t13, ptr %t9
  store ptr @glitch_lambda_13_destroy, ptr %t10
  %t18 = load ptr, ptr %t2
  %t19 = call ptr @FirstOrDefaultAsync__object_object(ptr %t4, ptr %t18)
  %t20 = load ptr, ptr @glitch_exception_pending
  %t21 = icmp ne ptr %t20, null
  br i1 %t21, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t22 = call ptr @glitch_task_get_result_ptr(ptr %t19)
  %t24 = icmp eq ptr %t22, null
  %t23 = alloca ptr
  br i1 %t24, label %coalesce_right_2, label %coalesce_left_1
coalesce_left_1:
  store ptr %t22, ptr %t23
  br label %coalesce_end_3
coalesce_right_2:
  %t25 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t26 = ptrtoint ptr %t25 to i64
  %t27 = call ptr @glitch_calloc(i64 1, i64 %t26)
  %t28 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t27, i32 0, i32 0
  store i64 1, ptr %t28
  %t29 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t27, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t29
  %t30 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t31 = load ptr, ptr %t30
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t27, ptr null, ptr null)
  %t32 = load ptr, ptr @glitch_exception_pending
  %t33 = icmp ne ptr %t32, null
  br i1 %t33, label %exception_unwind, label %call_continue_4
call_continue_4:
  store ptr %t27, ptr @glitch_exception_pending
  br label %exception_unwind
coalesce_end_3:
  %t34 = load ptr, ptr %t23
  store ptr %t34, ptr %t3
  %t35 = load ptr, ptr %t0
  %t36 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t35, i32 0, i32 2
  %t37 = load ptr, ptr %t36
  %t38 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t37, i32 0, i32 10
  %t39 = load ptr, ptr %t38
  %t40 = load ptr, ptr %t3
  %t41 = load ptr, ptr %t2
  %t42 = call ptr @SaveChangesAsync__object(ptr %t41)
  %t43 = load ptr, ptr @glitch_exception_pending
  %t44 = icmp ne ptr %t43, null
  br i1 %t44, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t45 = call ptr @glitch_task_get_result_ptr(ptr %t42)
  %t46 = call ptr @glitch_task_from_result_ptr(ptr null)
  %t47 = call ptr @glitch_task_get_result_ptr(ptr %t46)
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Articles_QueryHandler__g0__t119_Handle__g0__Query_CancellationToken(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t5 = getelementptr %glitch.delegate, ptr null, i32 1
  %t6 = ptrtoint ptr %t5 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t6)
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t7
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 1
  store ptr @glitch_lambda_14, ptr %t8
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 2
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 3
  %t11 = getelementptr %glitch.lambda.14.env, ptr null, i32 1
  %t12 = ptrtoint ptr %t11 to i64
  %t13 = call ptr @glitch_calloc(i64 1, i64 %t12)
  %t14 = load ptr, ptr %t1
  %t15 = getelementptr inbounds %glitch.lambda.14.env, ptr %t13, i32 0, i32 0
  store ptr %t14, ptr %t15
  store ptr %t13, ptr %t9
  store ptr @glitch_lambda_14_destroy, ptr %t10
  %t18 = load ptr, ptr %t2
  %t19 = call ptr @FirstOrDefaultAsync__object_object(ptr %t4, ptr %t18)
  %t20 = load ptr, ptr @glitch_exception_pending
  %t21 = icmp ne ptr %t20, null
  br i1 %t21, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t22 = call ptr @glitch_task_get_result_ptr(ptr %t19)
  store ptr %t22, ptr %t3
  %t23 = load ptr, ptr %t3
  %t24 = icmp eq ptr %t23, null
  br i1 %t24, label %if_then_1, label %if_else_2
if_then_1:
  %t25 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t26 = ptrtoint ptr %t25 to i64
  %t27 = call ptr @glitch_calloc(i64 1, i64 %t26)
  %t28 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t27, i32 0, i32 0
  store i64 1, ptr %t28
  %t29 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t27, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t29
  %t30 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t31 = load ptr, ptr %t30
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t27, ptr null, ptr null)
  %t32 = load ptr, ptr @glitch_exception_pending
  %t33 = icmp ne ptr %t32, null
  br i1 %t33, label %exception_unwind, label %call_continue_4
call_continue_4:
  store ptr %t27, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_2:
  br label %if_end_3
if_end_3:
  %t34 = getelementptr %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr null, i32 1
  %t35 = ptrtoint ptr %t34 to i64
  %t36 = call ptr @glitch_calloc(i64 1, i64 %t35)
  %t37 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t36, i32 0, i32 0
  store i64 1, ptr %t37
  %t38 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t36, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t38
  %t39 = load ptr, ptr %t3
  call void @Conduit_Features_Articles_ArticleEnvelope__g0__t108_ctor(ptr %t36, ptr %t39)
  %t40 = load ptr, ptr @glitch_exception_pending
  %t41 = icmp ne ptr %t40, null
  br i1 %t41, label %exception_unwind, label %call_continue_5
call_continue_5:
  ret ptr %t36
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Articles_Query__g0__t121_ctor__string(ptr %this, ptr %Slug) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Slug, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t2, i32 0, i32 2
  call void @glitch_string_retain(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_Query__g0__t121_ctor__string_string_string_int_int_bool(ptr %this, ptr %Tag, ptr %Author, ptr %FavoritedUsername, i32 %Limit, i32 %Offset, i1 %IsFeed) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Tag, ptr %t1
  %t2 = alloca ptr
  store ptr %Author, ptr %t2
  %t3 = alloca ptr
  store ptr %FavoritedUsername, ptr %t3
  %t4 = alloca i32
  store i32 %Limit, ptr %t4
  %t5 = alloca i32
  store i32 %Offset, ptr %t5
  %t6 = alloca i1
  store i1 %IsFeed, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t7, i32 0, i32 3
  call void @glitch_string_retain(ptr null)
  %t9 = load ptr, ptr %t8
  call void @glitch_string_release(ptr %t9)
  store ptr null, ptr %t8
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t10, i32 0, i32 4
  %t12 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t12)
  %t13 = load ptr, ptr %t11
  call void @glitch_string_release(ptr %t13)
  store ptr %t12, ptr %t11
  %t14 = load ptr, ptr %t0
  %t15 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t14, i32 0, i32 5
  %t16 = load ptr, ptr %t3
  call void @glitch_string_retain(ptr %t16)
  %t17 = load ptr, ptr %t15
  call void @glitch_string_release(ptr %t17)
  store ptr %t16, ptr %t15
  %t18 = load ptr, ptr %t0
  %t19 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t18, i32 0, i32 6
  %t20 = load i32, ptr %t4
  store i32 %t20, ptr %t19
  %t21 = load ptr, ptr %t0
  %t22 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t21, i32 0, i32 7
  %t23 = load i32, ptr %t5
  store i32 %t23, ptr %t22
  %t24 = load ptr, ptr %t0
  %t25 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t24, i32 0, i32 8
  %t26 = load i1, ptr %t6
  store i1 %t26, ptr %t25
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_QueryValidator__g0__t122_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_15, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  call void @glitch_delegate_release(ptr %t1)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Articles_Model__g0__t124_ctor(ptr %this, ptr %Article) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Article, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Comments_Model__g0__t132, ptr %t2, i32 0, i32 2
  call void @glitch_retain_Conduit_Features_Articles_ArticleData__g0__t113(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Features_Articles_ArticleData__g0__t113(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Comments_CommentEnvelope__g0__t126_ctor(ptr %this, ptr %Comment) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Comment, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentEnvelope__g0__t126, ptr %t2, i32 0, i32 2
  call void @glitch_retain_Conduit_Domain_Comment__g0__t104(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Domain_Comment__g0__t104(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Comments_CommentsController__g0__t127_ctor(ptr %this, ptr %mediator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %mediator, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsController__g0__t127, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Comments_CommentsController__g0__t127_Create__g0(ptr %this, ptr %slug, ptr %model, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %slug, ptr %t1
  %t2 = alloca ptr
  store ptr %model, ptr %t2
  %t3 = alloca ptr
  store ptr %cancellationToken, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsController__g0__t127, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t5
  %t7 = load ptr, ptr %t2
  %t8 = load ptr, ptr %t1
  %t9 = load ptr, ptr %t3
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Comments_CommentsController__g0__t127_Get__g0(ptr %this, ptr %slug, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %slug, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsController__g0__t127, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Comments_CommentsController__g0__t127_Delete__g0(ptr %this, ptr %slug, i32 %id, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %slug, ptr %t1
  %t2 = alloca i32
  store i32 %id, ptr %t2
  %t3 = alloca ptr
  store ptr %cancellationToken, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsController__g0__t127, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t5
  %t7 = load ptr, ptr %t1
  %t8 = load i32, ptr %t2
  %t9 = load ptr, ptr %t3
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Comments_CommentsEnvelope__g0__t128_ctor(ptr %this, ptr %Comments) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Comments, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsEnvelope__g0__t128, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Comments_CommentData__g0__t130_ctor(ptr %this, ptr %Body) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Body, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentData__g0__t130, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Comments_Command__g0__t131_ctor__Model_string(ptr %this, ptr %Model, ptr %Slug) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Model, ptr %t1
  %t2 = alloca ptr
  store ptr %Slug, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t3, i32 0, i32 4
  call void @glitch_retain_Conduit_Features_Comments_Model__g0__t132(ptr null)
  %t5 = load ptr, ptr %t4
  call void @glitch_drop_Conduit_Features_Comments_Model__g0__t132(ptr %t5)
  store ptr null, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t6, i32 0, i32 3
  call void @glitch_string_retain(ptr null)
  %t8 = load ptr, ptr %t7
  call void @glitch_string_release(ptr %t8)
  store ptr null, ptr %t7
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Comments_Command__g0__t131_ctor__string_int(ptr %this, ptr %Slug, i32 %Id) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Slug, ptr %t1
  %t2 = alloca i32
  store i32 %Id, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t3, i32 0, i32 3
  call void @glitch_string_retain(ptr null)
  %t5 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t5)
  store ptr null, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t6, i32 0, i32 5
  %t8 = load i32, ptr %t2
  store i32 %t8, ptr %t7
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Comments_Model__g0__t132_ctor(ptr %this, ptr %Comment) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Comment, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Comments_Model__g0__t132, ptr %t2, i32 0, i32 3
  call void @glitch_retain_Conduit_Features_Comments_CommentData__g0__t130(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Features_Comments_CommentData__g0__t130(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Comments_CommandValidator__g0__t133_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_16, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  call void @glitch_delegate_release(ptr %t1)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Comments_Handler__g0__t134_ctor(ptr %this, ptr %context, ptr %currentUserAccessor) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %currentUserAccessor, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t6)
  store ptr %t5, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t9)
  %t10 = load ptr, ptr %t8
  call void @glitch_drop_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t10)
  store ptr %t9, ptr %t8
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Comments_Handler__g0__t134_Handle__g0(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t7 = getelementptr %glitch.delegate, ptr null, i32 1
  %t8 = ptrtoint ptr %t7 to i64
  %t6 = call ptr @glitch_calloc(i64 1, i64 %t8)
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 0
  store i64 1, ptr %t9
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 1
  store ptr @glitch_lambda_17, ptr %t10
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 2
  %t12 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 3
  %t13 = getelementptr %glitch.lambda.17.env, ptr null, i32 1
  %t14 = ptrtoint ptr %t13 to i64
  %t15 = call ptr @glitch_calloc(i64 1, i64 %t14)
  %t16 = load ptr, ptr %t1
  %t17 = getelementptr inbounds %glitch.lambda.17.env, ptr %t15, i32 0, i32 0
  store ptr %t16, ptr %t17
  store ptr %t15, ptr %t11
  store ptr @glitch_lambda_17_destroy, ptr %t12
  %t20 = load ptr, ptr %t2
  %t21 = call ptr @FirstOrDefaultAsync__object_object(ptr %t6, ptr %t20)
  %t22 = load ptr, ptr @glitch_exception_pending
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t24 = call ptr @glitch_task_get_result_ptr(ptr %t21)
  store ptr %t24, ptr %t3
  %t25 = load ptr, ptr %t3
  %t26 = icmp eq ptr %t25, null
  br i1 %t26, label %if_then_1, label %if_else_2
if_then_1:
  %t27 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t28 = ptrtoint ptr %t27 to i64
  %t29 = call ptr @glitch_calloc(i64 1, i64 %t28)
  %t30 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t29, i32 0, i32 0
  store i64 1, ptr %t30
  %t31 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t29, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t31
  %t32 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t33 = load ptr, ptr %t32
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t29, ptr null, ptr null)
  %t34 = load ptr, ptr @glitch_exception_pending
  %t35 = icmp ne ptr %t34, null
  br i1 %t35, label %exception_unwind, label %call_continue_4
call_continue_4:
  store ptr %t29, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_2:
  br label %if_end_3
if_end_3:
  %t36 = load ptr, ptr %t0
  %t37 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t36, i32 0, i32 2
  %t38 = load ptr, ptr %t37
  %t39 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t38, i32 0, i32 12
  %t40 = load ptr, ptr %t39
  %t42 = getelementptr %glitch.delegate, ptr null, i32 1
  %t43 = ptrtoint ptr %t42 to i64
  %t41 = call ptr @glitch_calloc(i64 1, i64 %t43)
  %t44 = getelementptr inbounds %glitch.delegate, ptr %t41, i32 0, i32 0
  store i64 1, ptr %t44
  %t45 = getelementptr inbounds %glitch.delegate, ptr %t41, i32 0, i32 1
  store ptr @glitch_lambda_18, ptr %t45
  %t46 = getelementptr inbounds %glitch.delegate, ptr %t41, i32 0, i32 2
  %t47 = getelementptr inbounds %glitch.delegate, ptr %t41, i32 0, i32 3
  %t48 = getelementptr %glitch.lambda.18.env, ptr null, i32 1
  %t49 = ptrtoint ptr %t48 to i64
  %t50 = call ptr @glitch_calloc(i64 1, i64 %t49)
  %t51 = load ptr, ptr %t0
  %t52 = getelementptr inbounds %glitch.lambda.18.env, ptr %t50, i32 0, i32 0
  store ptr %t51, ptr %t52
  store ptr %t50, ptr %t46
  store ptr @glitch_lambda_18_destroy, ptr %t47
  call void @glitch_delegate_release(ptr %t41)
  %t55 = load ptr, ptr %t2
  %t56 = call ptr @glitch_task_get_result_ptr(ptr null)
  store ptr %t56, ptr %t4
  %t57 = getelementptr %glitch.Conduit_Domain_Comment__g0__t104, ptr null, i32 1
  %t58 = ptrtoint ptr %t57 to i64
  %t59 = call ptr @glitch_calloc(i64 1, i64 %t58)
  %t60 = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %t59, i32 0, i32 0
  store i64 1, ptr %t60
  %t61 = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %t59, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Domain_Comment__g0__t104, ptr %t61
  %t62 = load ptr, ptr %t4
  call void @glitch_retain_Conduit_Domain_Person__g0__t106(ptr %t62)
  %t63 = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %t59, i32 0, i32 4
  store ptr %t62, ptr %t63
  %t64 = load ptr, ptr %t1
  %t65 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t64, i32 0, i32 4
  %t66 = load ptr, ptr %t65
  %t67 = getelementptr inbounds %glitch.Conduit_Features_Comments_Model__g0__t132, ptr %t66, i32 0, i32 3
  %t68 = load ptr, ptr %t67
  %t69 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentData__g0__t130, ptr %t68, i32 0, i32 2
  %t70 = load ptr, ptr %t69
  %t72 = icmp eq ptr %t70, null
  %t71 = alloca ptr
  br i1 %t72, label %coalesce_right_6, label %coalesce_left_5
coalesce_left_5:
  store ptr %t70, ptr %t71
  br label %coalesce_end_7
coalesce_right_6:
  store ptr null, ptr %t71
  br label %coalesce_end_7
coalesce_end_7:
  %t73 = load ptr, ptr %t71
  %t74 = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %t59, i32 0, i32 3
  store ptr %t73, ptr %t74
  %t75 = getelementptr inbounds %glitch.DateTime__g0__t17, ptr null, i32 0, i32 3
  %t76 = load ptr, ptr %t75
  call void @glitch_string_retain(ptr %t76)
  %t77 = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %t59, i32 0, i32 8
  store ptr %t76, ptr %t77
  %t78 = getelementptr inbounds %glitch.DateTime__g0__t17, ptr null, i32 0, i32 3
  %t79 = load ptr, ptr %t78
  call void @glitch_string_retain(ptr %t79)
  %t80 = getelementptr inbounds %glitch.Conduit_Domain_Comment__g0__t104, ptr %t59, i32 0, i32 9
  store ptr %t79, ptr %t80
  %t81 = load ptr, ptr %t5
  call void @glitch_drop_Conduit_Domain_Comment__g0__t104(ptr %t81)
  store ptr %t59, ptr %t5
  %t82 = load ptr, ptr %t5
  %t83 = load ptr, ptr %t2
  %t84 = call ptr @AddAsync(ptr %t82, ptr %t83)
  %t85 = load ptr, ptr @glitch_exception_pending
  %t86 = icmp ne ptr %t85, null
  br i1 %t86, label %exception_unwind, label %call_continue_8
call_continue_8:
  %t87 = call ptr @glitch_task_get_result_ptr(ptr %t84)
  %t88 = load ptr, ptr %t5
  %t89 = load ptr, ptr %t2
  %t90 = call ptr @SaveChangesAsync__object(ptr %t89)
  %t91 = load ptr, ptr @glitch_exception_pending
  %t92 = icmp ne ptr %t91, null
  br i1 %t92, label %exception_unwind, label %call_continue_9
call_continue_9:
  %t93 = call ptr @glitch_task_get_result_ptr(ptr %t90)
  %t94 = getelementptr %glitch.Conduit_Features_Comments_CommentEnvelope__g0__t126, ptr null, i32 1
  %t95 = ptrtoint ptr %t94 to i64
  %t96 = call ptr @glitch_calloc(i64 1, i64 %t95)
  %t97 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentEnvelope__g0__t126, ptr %t96, i32 0, i32 0
  store i64 1, ptr %t97
  %t98 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentEnvelope__g0__t126, ptr %t96, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Comments_CommentEnvelope__g0__t126, ptr %t98
  %t99 = load ptr, ptr %t5
  call void @Conduit_Features_Comments_CommentEnvelope__g0__t126_ctor(ptr %t96, ptr %t99)
  %t100 = load ptr, ptr @glitch_exception_pending
  %t101 = icmp ne ptr %t100, null
  br i1 %t101, label %exception_unwind, label %call_continue_10
call_continue_10:
  %t102 = load ptr, ptr %t5
  call void @glitch_drop_Conduit_Domain_Comment__g0__t104(ptr %t102)
  ret ptr %t96
exception_unwind:
  %t103 = load ptr, ptr %t5
  call void @glitch_drop_Conduit_Domain_Comment__g0__t104(ptr %t103)
  ret ptr null
}

define void @Conduit_Features_Comments_QueryHandler__g0__t136_ctor__ConduitContext(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Comments_QueryHandler__g0__t136_Handle__g0__Command_CancellationToken(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t6 = getelementptr %glitch.delegate, ptr null, i32 1
  %t7 = ptrtoint ptr %t6 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t7)
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t8
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t5, i32 0, i32 1
  store ptr @glitch_lambda_19, ptr %t9
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t5, i32 0, i32 2
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t5, i32 0, i32 3
  %t12 = getelementptr %glitch.lambda.19.env, ptr null, i32 1
  %t13 = ptrtoint ptr %t12 to i64
  %t14 = call ptr @glitch_calloc(i64 1, i64 %t13)
  %t15 = load ptr, ptr %t1
  %t16 = getelementptr inbounds %glitch.lambda.19.env, ptr %t14, i32 0, i32 0
  store ptr %t15, ptr %t16
  store ptr %t14, ptr %t10
  store ptr @glitch_lambda_19_destroy, ptr %t11
  %t19 = load ptr, ptr %t2
  %t20 = call ptr @FirstOrDefaultAsync__object_object(ptr %t5, ptr %t19)
  %t21 = load ptr, ptr @glitch_exception_pending
  %t22 = icmp ne ptr %t21, null
  br i1 %t22, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t23 = call ptr @glitch_task_get_result_ptr(ptr %t20)
  %t25 = icmp eq ptr %t23, null
  %t24 = alloca ptr
  br i1 %t25, label %coalesce_right_2, label %coalesce_left_1
coalesce_left_1:
  store ptr %t23, ptr %t24
  br label %coalesce_end_3
coalesce_right_2:
  %t26 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t27 = ptrtoint ptr %t26 to i64
  %t28 = call ptr @glitch_calloc(i64 1, i64 %t27)
  %t29 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t28, i32 0, i32 0
  store i64 1, ptr %t29
  %t30 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t28, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t30
  %t31 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t32 = load ptr, ptr %t31
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t28, ptr null, ptr null)
  %t33 = load ptr, ptr @glitch_exception_pending
  %t34 = icmp ne ptr %t33, null
  br i1 %t34, label %exception_unwind, label %call_continue_4
call_continue_4:
  store ptr %t28, ptr @glitch_exception_pending
  br label %exception_unwind
coalesce_end_3:
  %t35 = load ptr, ptr %t24
  store ptr %t35, ptr %t3
  %t37 = getelementptr %glitch.delegate, ptr null, i32 1
  %t38 = ptrtoint ptr %t37 to i64
  %t36 = call ptr @glitch_calloc(i64 1, i64 %t38)
  %t39 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 0
  store i64 1, ptr %t39
  %t40 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 1
  store ptr @glitch_lambda_20, ptr %t40
  %t41 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 2
  %t42 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 3
  %t43 = getelementptr %glitch.lambda.20.env, ptr null, i32 1
  %t44 = ptrtoint ptr %t43 to i64
  %t45 = call ptr @glitch_calloc(i64 1, i64 %t44)
  %t46 = load ptr, ptr %t1
  %t47 = getelementptr inbounds %glitch.lambda.20.env, ptr %t45, i32 0, i32 0
  store ptr %t46, ptr %t47
  store ptr %t45, ptr %t41
  store ptr @glitch_lambda_20_destroy, ptr %t42
  %t50 = call ptr @FirstOrDefault__object(ptr %t36)
  %t51 = load ptr, ptr @glitch_exception_pending
  %t52 = icmp ne ptr %t51, null
  br i1 %t52, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t54 = icmp eq ptr %t50, null
  %t53 = alloca ptr
  br i1 %t54, label %coalesce_right_7, label %coalesce_left_6
coalesce_left_6:
  store ptr %t50, ptr %t53
  br label %coalesce_end_8
coalesce_right_7:
  %t55 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t56 = ptrtoint ptr %t55 to i64
  %t57 = call ptr @glitch_calloc(i64 1, i64 %t56)
  %t58 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t57, i32 0, i32 0
  store i64 1, ptr %t58
  %t59 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t57, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t59
  %t60 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t61 = load ptr, ptr %t60
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t57, ptr null, ptr null)
  %t62 = load ptr, ptr @glitch_exception_pending
  %t63 = icmp ne ptr %t62, null
  br i1 %t63, label %exception_unwind, label %call_continue_9
call_continue_9:
  store ptr %t57, ptr @glitch_exception_pending
  br label %exception_unwind
coalesce_end_8:
  %t64 = load ptr, ptr %t53
  store ptr %t64, ptr %t4
  %t65 = load ptr, ptr %t0
  %t66 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t65, i32 0, i32 2
  %t67 = load ptr, ptr %t66
  %t68 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t67, i32 0, i32 11
  %t69 = load ptr, ptr %t68
  %t70 = load ptr, ptr %t4
  %t71 = load ptr, ptr %t2
  %t72 = call ptr @SaveChangesAsync__object(ptr %t71)
  %t73 = load ptr, ptr @glitch_exception_pending
  %t74 = icmp ne ptr %t73, null
  br i1 %t74, label %exception_unwind, label %call_continue_10
call_continue_10:
  %t75 = call ptr @glitch_task_get_result_ptr(ptr %t72)
  %t76 = call ptr @glitch_task_from_result_ptr(ptr null)
  %t77 = call ptr @glitch_task_get_result_ptr(ptr %t76)
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Comments_QueryHandler__g0__t136_Handle__g0__Query_CancellationToken(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t5 = getelementptr %glitch.delegate, ptr null, i32 1
  %t6 = ptrtoint ptr %t5 to i64
  %t4 = call ptr @glitch_calloc(i64 1, i64 %t6)
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 0
  store i64 1, ptr %t7
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 1
  store ptr @glitch_lambda_21, ptr %t8
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 2
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t4, i32 0, i32 3
  %t11 = getelementptr %glitch.lambda.21.env, ptr null, i32 1
  %t12 = ptrtoint ptr %t11 to i64
  %t13 = call ptr @glitch_calloc(i64 1, i64 %t12)
  %t14 = load ptr, ptr %t1
  %t15 = getelementptr inbounds %glitch.lambda.21.env, ptr %t13, i32 0, i32 0
  store ptr %t14, ptr %t15
  store ptr %t13, ptr %t9
  store ptr @glitch_lambda_21_destroy, ptr %t10
  %t18 = load ptr, ptr %t2
  %t19 = call ptr @FirstOrDefaultAsync__object_object(ptr %t4, ptr %t18)
  %t20 = load ptr, ptr @glitch_exception_pending
  %t21 = icmp ne ptr %t20, null
  br i1 %t21, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t22 = call ptr @glitch_task_get_result_ptr(ptr %t19)
  store ptr %t22, ptr %t3
  %t23 = load ptr, ptr %t3
  %t24 = icmp eq ptr %t23, null
  br i1 %t24, label %if_then_1, label %if_else_2
if_then_1:
  %t25 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t26 = ptrtoint ptr %t25 to i64
  %t27 = call ptr @glitch_calloc(i64 1, i64 %t26)
  %t28 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t27, i32 0, i32 0
  store i64 1, ptr %t28
  %t29 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t27, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t29
  %t30 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t31 = load ptr, ptr %t30
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t27, ptr null, ptr null)
  %t32 = load ptr, ptr @glitch_exception_pending
  %t33 = icmp ne ptr %t32, null
  br i1 %t33, label %exception_unwind, label %call_continue_4
call_continue_4:
  store ptr %t27, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_2:
  br label %if_end_3
if_end_3:
  %t34 = getelementptr %glitch.Conduit_Features_Comments_CommentsEnvelope__g0__t128, ptr null, i32 1
  %t35 = ptrtoint ptr %t34 to i64
  %t36 = call ptr @glitch_calloc(i64 1, i64 %t35)
  %t37 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsEnvelope__g0__t128, ptr %t36, i32 0, i32 0
  store i64 1, ptr %t37
  %t38 = getelementptr inbounds %glitch.Conduit_Features_Comments_CommentsEnvelope__g0__t128, ptr %t36, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Comments_CommentsEnvelope__g0__t128, ptr %t38
  call void @Conduit_Features_Comments_CommentsEnvelope__g0__t128_ctor(ptr %t36, ptr null)
  %t39 = load ptr, ptr @glitch_exception_pending
  %t40 = icmp ne ptr %t39, null
  br i1 %t40, label %exception_unwind, label %call_continue_5
call_continue_5:
  ret ptr %t36
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Comments_Query__g0__t138_ctor(ptr %this, ptr %Slug) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Slug, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t2, i32 0, i32 2
  call void @glitch_string_retain(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Favorites_Command__g0__t140_ctor__string(ptr %this, ptr %Slug) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Slug, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t2, i32 0, i32 3
  call void @glitch_string_retain(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Favorites_CommandValidator__g0__t141_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_22, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  call void @glitch_delegate_release(ptr %t1)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Favorites_QueryHandler__g0__t142_ctor__ConduitContext_ICurrentUserAccessor(ptr %this, ptr %context, ptr %currentUserAccessor) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %currentUserAccessor, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t5)
  %t6 = load ptr, ptr %t4
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t6)
  store ptr %t5, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t9)
  %t10 = load ptr, ptr %t8
  call void @glitch_drop_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t10)
  store ptr %t9, ptr %t8
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Favorites_QueryHandler__g0__t142_Handle__g0__Command_CancellationToken(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t7 = getelementptr %glitch.delegate, ptr null, i32 1
  %t8 = ptrtoint ptr %t7 to i64
  %t6 = call ptr @glitch_calloc(i64 1, i64 %t8)
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 0
  store i64 1, ptr %t9
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 1
  store ptr @glitch_lambda_23, ptr %t10
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 2
  %t12 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 3
  %t13 = getelementptr %glitch.lambda.23.env, ptr null, i32 1
  %t14 = ptrtoint ptr %t13 to i64
  %t15 = call ptr @glitch_calloc(i64 1, i64 %t14)
  %t16 = load ptr, ptr %t1
  %t17 = getelementptr inbounds %glitch.lambda.23.env, ptr %t15, i32 0, i32 0
  store ptr %t16, ptr %t17
  store ptr %t15, ptr %t11
  store ptr @glitch_lambda_23_destroy, ptr %t12
  %t20 = load ptr, ptr %t2
  %t21 = call ptr @FirstOrDefaultAsync__object_object(ptr %t6, ptr %t20)
  %t22 = load ptr, ptr @glitch_exception_pending
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t24 = call ptr @glitch_task_get_result_ptr(ptr %t21)
  store ptr %t24, ptr %t3
  %t25 = load ptr, ptr %t3
  %t26 = icmp eq ptr %t25, null
  br i1 %t26, label %if_then_1, label %if_else_2
if_then_1:
  %t27 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t28 = ptrtoint ptr %t27 to i64
  %t29 = call ptr @glitch_calloc(i64 1, i64 %t28)
  %t30 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t29, i32 0, i32 0
  store i64 1, ptr %t30
  %t31 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t29, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t31
  %t32 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t33 = load ptr, ptr %t32
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t29, ptr null, ptr null)
  %t34 = load ptr, ptr @glitch_exception_pending
  %t35 = icmp ne ptr %t34, null
  br i1 %t35, label %exception_unwind, label %call_continue_4
call_continue_4:
  store ptr %t29, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_2:
  br label %if_end_3
if_end_3:
  %t37 = getelementptr %glitch.delegate, ptr null, i32 1
  %t38 = ptrtoint ptr %t37 to i64
  %t36 = call ptr @glitch_calloc(i64 1, i64 %t38)
  %t39 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 0
  store i64 1, ptr %t39
  %t40 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 1
  store ptr @glitch_lambda_24, ptr %t40
  %t41 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 2
  %t42 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 3
  %t43 = getelementptr %glitch.lambda.24.env, ptr null, i32 1
  %t44 = ptrtoint ptr %t43 to i64
  %t45 = call ptr @glitch_calloc(i64 1, i64 %t44)
  %t46 = load ptr, ptr %t0
  %t47 = getelementptr inbounds %glitch.lambda.24.env, ptr %t45, i32 0, i32 0
  store ptr %t46, ptr %t47
  store ptr %t45, ptr %t41
  store ptr @glitch_lambda_24_destroy, ptr %t42
  %t50 = load ptr, ptr %t2
  %t51 = call ptr @FirstOrDefaultAsync__object_object(ptr %t36, ptr %t50)
  %t52 = load ptr, ptr @glitch_exception_pending
  %t53 = icmp ne ptr %t52, null
  br i1 %t53, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t54 = call ptr @glitch_task_get_result_ptr(ptr %t51)
  store ptr %t54, ptr %t4
  %t55 = load ptr, ptr %t4
  %t56 = icmp eq ptr %t55, null
  br i1 %t56, label %if_then_6, label %if_else_7
if_then_6:
  %t57 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t58 = ptrtoint ptr %t57 to i64
  %t59 = call ptr @glitch_calloc(i64 1, i64 %t58)
  %t60 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t59, i32 0, i32 0
  store i64 1, ptr %t60
  %t61 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t59, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t61
  %t62 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t63 = load ptr, ptr %t62
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t59, ptr null, ptr null)
  %t64 = load ptr, ptr @glitch_exception_pending
  %t65 = icmp ne ptr %t64, null
  br i1 %t65, label %exception_unwind, label %call_continue_9
call_continue_9:
  store ptr %t59, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_7:
  br label %if_end_8
if_end_8:
  %t67 = getelementptr %glitch.delegate, ptr null, i32 1
  %t68 = ptrtoint ptr %t67 to i64
  %t66 = call ptr @glitch_calloc(i64 1, i64 %t68)
  %t69 = getelementptr inbounds %glitch.delegate, ptr %t66, i32 0, i32 0
  store i64 1, ptr %t69
  %t70 = getelementptr inbounds %glitch.delegate, ptr %t66, i32 0, i32 1
  store ptr @glitch_lambda_25, ptr %t70
  %t71 = getelementptr inbounds %glitch.delegate, ptr %t66, i32 0, i32 2
  %t72 = getelementptr inbounds %glitch.delegate, ptr %t66, i32 0, i32 3
  %t73 = getelementptr %glitch.lambda.25.env, ptr null, i32 1
  %t74 = ptrtoint ptr %t73 to i64
  %t75 = call ptr @glitch_calloc(i64 1, i64 %t74)
  %t76 = load ptr, ptr %t3
  %t77 = getelementptr inbounds %glitch.lambda.25.env, ptr %t75, i32 0, i32 0
  store ptr %t76, ptr %t77
  %t78 = load ptr, ptr %t4
  %t79 = getelementptr inbounds %glitch.lambda.25.env, ptr %t75, i32 0, i32 1
  store ptr %t78, ptr %t79
  store ptr %t75, ptr %t71
  store ptr @glitch_lambda_25_destroy, ptr %t72
  %t84 = load ptr, ptr %t2
  %t85 = call ptr @FirstOrDefaultAsync__object_object(ptr %t66, ptr %t84)
  %t86 = load ptr, ptr @glitch_exception_pending
  %t87 = icmp ne ptr %t86, null
  br i1 %t87, label %exception_unwind, label %call_continue_10
call_continue_10:
  %t88 = call ptr @glitch_task_get_result_ptr(ptr %t85)
  store ptr %t88, ptr %t5
  %t89 = load ptr, ptr %t5
  %t90 = icmp eq ptr %t89, null
  br i1 %t90, label %if_then_11, label %if_else_12
if_then_11:
  %t91 = getelementptr %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr null, i32 1
  %t92 = ptrtoint ptr %t91 to i64
  %t93 = call ptr @glitch_calloc(i64 1, i64 %t92)
  %t94 = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %t93, i32 0, i32 0
  store i64 1, ptr %t94
  %t95 = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %t93, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Domain_ArticleFavorite__g0__t102, ptr %t95
  %t96 = load ptr, ptr %t3
  call void @glitch_retain_Conduit_Domain_Article__g0__t101(ptr %t96)
  %t97 = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %t93, i32 0, i32 3
  store ptr %t96, ptr %t97
  %t98 = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %t93, i32 0, i32 2
  store i32 0, ptr %t98
  %t99 = load ptr, ptr %t4
  call void @glitch_retain_Conduit_Domain_Person__g0__t106(ptr %t99)
  %t100 = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %t93, i32 0, i32 5
  store ptr %t99, ptr %t100
  %t101 = getelementptr inbounds %glitch.Conduit_Domain_ArticleFavorite__g0__t102, ptr %t93, i32 0, i32 4
  store i32 0, ptr %t101
  store ptr %t93, ptr %t5
  %t102 = load ptr, ptr %t5
  %t103 = load ptr, ptr %t2
  %t104 = call ptr @AddAsync(ptr %t102, ptr %t103)
  %t105 = load ptr, ptr @glitch_exception_pending
  %t106 = icmp ne ptr %t105, null
  br i1 %t106, label %exception_unwind, label %call_continue_14
call_continue_14:
  %t107 = call ptr @glitch_task_get_result_ptr(ptr %t104)
  %t108 = load ptr, ptr %t2
  %t109 = call ptr @SaveChangesAsync__object(ptr %t108)
  %t110 = load ptr, ptr @glitch_exception_pending
  %t111 = icmp ne ptr %t110, null
  br i1 %t111, label %exception_unwind, label %call_continue_15
call_continue_15:
  %t112 = call ptr @glitch_task_get_result_ptr(ptr %t109)
  br label %if_end_13
if_else_12:
  br label %if_end_13
if_end_13:
  %t114 = getelementptr %glitch.delegate, ptr null, i32 1
  %t115 = ptrtoint ptr %t114 to i64
  %t113 = call ptr @glitch_calloc(i64 1, i64 %t115)
  %t116 = getelementptr inbounds %glitch.delegate, ptr %t113, i32 0, i32 0
  store i64 1, ptr %t116
  %t117 = getelementptr inbounds %glitch.delegate, ptr %t113, i32 0, i32 1
  store ptr @glitch_lambda_26, ptr %t117
  %t118 = getelementptr inbounds %glitch.delegate, ptr %t113, i32 0, i32 2
  %t119 = getelementptr inbounds %glitch.delegate, ptr %t113, i32 0, i32 3
  %t120 = getelementptr %glitch.lambda.26.env, ptr null, i32 1
  %t121 = ptrtoint ptr %t120 to i64
  %t122 = call ptr @glitch_calloc(i64 1, i64 %t121)
  %t123 = load ptr, ptr %t3
  %t124 = getelementptr inbounds %glitch.lambda.26.env, ptr %t122, i32 0, i32 0
  store ptr %t123, ptr %t124
  store ptr %t122, ptr %t118
  store ptr @glitch_lambda_26_destroy, ptr %t119
  %t127 = load ptr, ptr %t2
  %t128 = call ptr @FirstOrDefaultAsync__object_object(ptr %t113, ptr %t127)
  %t129 = load ptr, ptr @glitch_exception_pending
  %t130 = icmp ne ptr %t129, null
  br i1 %t130, label %exception_unwind, label %call_continue_16
call_continue_16:
  %t131 = call ptr @glitch_task_get_result_ptr(ptr %t128)
  store ptr %t131, ptr %t3
  %t132 = load ptr, ptr %t3
  %t133 = icmp eq ptr %t132, null
  br i1 %t133, label %if_then_17, label %if_else_18
if_then_17:
  %t134 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t135 = ptrtoint ptr %t134 to i64
  %t136 = call ptr @glitch_calloc(i64 1, i64 %t135)
  %t137 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t136, i32 0, i32 0
  store i64 1, ptr %t137
  %t138 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t136, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t138
  %t139 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t140 = load ptr, ptr %t139
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t136, ptr null, ptr null)
  %t141 = load ptr, ptr @glitch_exception_pending
  %t142 = icmp ne ptr %t141, null
  br i1 %t142, label %exception_unwind, label %call_continue_20
call_continue_20:
  store ptr %t136, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_18:
  br label %if_end_19
if_end_19:
  %t143 = getelementptr %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr null, i32 1
  %t144 = ptrtoint ptr %t143 to i64
  %t145 = call ptr @glitch_calloc(i64 1, i64 %t144)
  %t146 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t145, i32 0, i32 0
  store i64 1, ptr %t146
  %t147 = getelementptr inbounds %glitch.Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t145, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Articles_ArticleEnvelope__g0__t108, ptr %t147
  %t148 = load ptr, ptr %t3
  call void @Conduit_Features_Articles_ArticleEnvelope__g0__t108_ctor(ptr %t145, ptr %t148)
  %t149 = load ptr, ptr @glitch_exception_pending
  %t150 = icmp ne ptr %t149, null
  br i1 %t150, label %exception_unwind, label %call_continue_21
call_continue_21:
  ret ptr %t145
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Favorites_FavoritesController__g0__t144_ctor(ptr %this, ptr %mediator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %mediator, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Favorites_FavoritesController__g0__t144, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Favorites_FavoritesController__g0__t144_FavoriteAdd__g0(ptr %this, ptr %slug, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %slug, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Favorites_FavoritesController__g0__t144, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Favorites_FavoritesController__g0__t144_FavoriteDelete__g0(ptr %this, ptr %slug, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %slug, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Favorites_FavoritesController__g0__t144, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Followers_Command__g0__t146_ctor__string(ptr %this, ptr %Username) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Username, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t2, i32 0, i32 6
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Followers_CommandValidator__g0__t147_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_27, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  call void @glitch_delegate_release(ptr %t1)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Followers_QueryHandler__g0__t148_ctor__ConduitContext_ICurrentUserAccessor_IProfileReader(ptr %this, ptr %context, ptr %currentUserAccessor, ptr %profileReader) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %currentUserAccessor, ptr %t2
  %t3 = alloca ptr
  store ptr %profileReader, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_drop_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t12, i32 0, i32 4
  %t14 = load ptr, ptr %t3
  call void @glitch_retain_Conduit_Features_Profiles_IProfileReader__g0__t155(ptr %t14)
  %t15 = load ptr, ptr %t13
  call void @glitch_drop_Conduit_Features_Profiles_IProfileReader__g0__t155(ptr %t15)
  store ptr %t14, ptr %t13
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Followers_QueryHandler__g0__t148_Handle__g0__Command_CancellationToken(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t7 = getelementptr %glitch.delegate, ptr null, i32 1
  %t8 = ptrtoint ptr %t7 to i64
  %t6 = call ptr @glitch_calloc(i64 1, i64 %t8)
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 0
  store i64 1, ptr %t9
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 1
  store ptr @glitch_lambda_28, ptr %t10
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 2
  %t12 = getelementptr inbounds %glitch.delegate, ptr %t6, i32 0, i32 3
  %t13 = getelementptr %glitch.lambda.28.env, ptr null, i32 1
  %t14 = ptrtoint ptr %t13 to i64
  %t15 = call ptr @glitch_calloc(i64 1, i64 %t14)
  %t16 = load ptr, ptr %t1
  %t17 = getelementptr inbounds %glitch.lambda.28.env, ptr %t15, i32 0, i32 0
  store ptr %t16, ptr %t17
  store ptr %t15, ptr %t11
  store ptr @glitch_lambda_28_destroy, ptr %t12
  %t20 = load ptr, ptr %t2
  %t21 = call ptr @FirstOrDefaultAsync__object_object(ptr %t6, ptr %t20)
  %t22 = load ptr, ptr @glitch_exception_pending
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t24 = call ptr @glitch_task_get_result_ptr(ptr %t21)
  store ptr %t24, ptr %t3
  %t25 = load ptr, ptr %t3
  %t26 = icmp eq ptr %t25, null
  br i1 %t26, label %if_then_1, label %if_else_2
if_then_1:
  %t27 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t28 = ptrtoint ptr %t27 to i64
  %t29 = call ptr @glitch_calloc(i64 1, i64 %t28)
  %t30 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t29, i32 0, i32 0
  store i64 1, ptr %t30
  %t31 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t29, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t31
  %t32 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t33 = load ptr, ptr %t32
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t29, ptr null, ptr null)
  %t34 = load ptr, ptr @glitch_exception_pending
  %t35 = icmp ne ptr %t34, null
  br i1 %t35, label %exception_unwind, label %call_continue_4
call_continue_4:
  store ptr %t29, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_2:
  br label %if_end_3
if_end_3:
  %t37 = getelementptr %glitch.delegate, ptr null, i32 1
  %t38 = ptrtoint ptr %t37 to i64
  %t36 = call ptr @glitch_calloc(i64 1, i64 %t38)
  %t39 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 0
  store i64 1, ptr %t39
  %t40 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 1
  store ptr @glitch_lambda_29, ptr %t40
  %t41 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 2
  %t42 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 3
  %t43 = getelementptr %glitch.lambda.29.env, ptr null, i32 1
  %t44 = ptrtoint ptr %t43 to i64
  %t45 = call ptr @glitch_calloc(i64 1, i64 %t44)
  %t46 = load ptr, ptr %t0
  %t47 = getelementptr inbounds %glitch.lambda.29.env, ptr %t45, i32 0, i32 0
  store ptr %t46, ptr %t47
  store ptr %t45, ptr %t41
  store ptr @glitch_lambda_29_destroy, ptr %t42
  %t50 = load ptr, ptr %t2
  %t51 = call ptr @FirstOrDefaultAsync__object_object(ptr %t36, ptr %t50)
  %t52 = load ptr, ptr @glitch_exception_pending
  %t53 = icmp ne ptr %t52, null
  br i1 %t53, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t54 = call ptr @glitch_task_get_result_ptr(ptr %t51)
  store ptr %t54, ptr %t4
  %t55 = load ptr, ptr %t4
  %t56 = icmp eq ptr %t55, null
  br i1 %t56, label %if_then_6, label %if_else_7
if_then_6:
  %t57 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t58 = ptrtoint ptr %t57 to i64
  %t59 = call ptr @glitch_calloc(i64 1, i64 %t58)
  %t60 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t59, i32 0, i32 0
  store i64 1, ptr %t60
  %t61 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t59, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t61
  %t62 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t63 = load ptr, ptr %t62
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t59, ptr null, ptr null)
  %t64 = load ptr, ptr @glitch_exception_pending
  %t65 = icmp ne ptr %t64, null
  br i1 %t65, label %exception_unwind, label %call_continue_9
call_continue_9:
  store ptr %t59, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_7:
  br label %if_end_8
if_end_8:
  %t67 = getelementptr %glitch.delegate, ptr null, i32 1
  %t68 = ptrtoint ptr %t67 to i64
  %t66 = call ptr @glitch_calloc(i64 1, i64 %t68)
  %t69 = getelementptr inbounds %glitch.delegate, ptr %t66, i32 0, i32 0
  store i64 1, ptr %t69
  %t70 = getelementptr inbounds %glitch.delegate, ptr %t66, i32 0, i32 1
  store ptr @glitch_lambda_30, ptr %t70
  %t71 = getelementptr inbounds %glitch.delegate, ptr %t66, i32 0, i32 2
  %t72 = getelementptr inbounds %glitch.delegate, ptr %t66, i32 0, i32 3
  %t73 = getelementptr %glitch.lambda.30.env, ptr null, i32 1
  %t74 = ptrtoint ptr %t73 to i64
  %t75 = call ptr @glitch_calloc(i64 1, i64 %t74)
  %t76 = load ptr, ptr %t4
  %t77 = getelementptr inbounds %glitch.lambda.30.env, ptr %t75, i32 0, i32 0
  store ptr %t76, ptr %t77
  %t78 = load ptr, ptr %t3
  %t79 = getelementptr inbounds %glitch.lambda.30.env, ptr %t75, i32 0, i32 1
  store ptr %t78, ptr %t79
  store ptr %t75, ptr %t71
  store ptr @glitch_lambda_30_destroy, ptr %t72
  %t84 = load ptr, ptr %t2
  %t85 = call ptr @FirstOrDefaultAsync__object_object(ptr %t66, ptr %t84)
  %t86 = load ptr, ptr @glitch_exception_pending
  %t87 = icmp ne ptr %t86, null
  br i1 %t87, label %exception_unwind, label %call_continue_10
call_continue_10:
  %t88 = call ptr @glitch_task_get_result_ptr(ptr %t85)
  store ptr %t88, ptr %t5
  %t89 = load ptr, ptr %t5
  %t90 = icmp eq ptr %t89, null
  br i1 %t90, label %if_then_11, label %if_else_12
if_then_11:
  %t91 = getelementptr %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr null, i32 1
  %t92 = ptrtoint ptr %t91 to i64
  %t93 = call ptr @glitch_calloc(i64 1, i64 %t92)
  %t94 = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %t93, i32 0, i32 0
  store i64 1, ptr %t94
  %t95 = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %t93, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Domain_FollowedPeople__g0__t105, ptr %t95
  %t96 = load ptr, ptr %t4
  call void @glitch_retain_Conduit_Domain_Person__g0__t106(ptr %t96)
  %t97 = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %t93, i32 0, i32 3
  store ptr %t96, ptr %t97
  %t98 = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %t93, i32 0, i32 2
  store i32 0, ptr %t98
  %t99 = load ptr, ptr %t3
  call void @glitch_retain_Conduit_Domain_Person__g0__t106(ptr %t99)
  %t100 = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %t93, i32 0, i32 5
  store ptr %t99, ptr %t100
  %t101 = getelementptr inbounds %glitch.Conduit_Domain_FollowedPeople__g0__t105, ptr %t93, i32 0, i32 4
  store i32 0, ptr %t101
  store ptr %t93, ptr %t5
  %t102 = load ptr, ptr %t5
  %t103 = load ptr, ptr %t2
  %t104 = call ptr @AddAsync(ptr %t102, ptr %t103)
  %t105 = load ptr, ptr @glitch_exception_pending
  %t106 = icmp ne ptr %t105, null
  br i1 %t106, label %exception_unwind, label %call_continue_14
call_continue_14:
  %t107 = call ptr @glitch_task_get_result_ptr(ptr %t104)
  %t108 = load ptr, ptr %t2
  %t109 = call ptr @SaveChangesAsync__object(ptr %t108)
  %t110 = load ptr, ptr @glitch_exception_pending
  %t111 = icmp ne ptr %t110, null
  br i1 %t111, label %exception_unwind, label %call_continue_15
call_continue_15:
  %t112 = call ptr @glitch_task_get_result_ptr(ptr %t109)
  br label %if_end_13
if_else_12:
  br label %if_end_13
if_end_13:
  %t113 = load ptr, ptr %t0
  %t114 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t113, i32 0, i32 4
  %t115 = load ptr, ptr %t114
  %t116 = load ptr, ptr %t1
  %t117 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t116, i32 0, i32 6
  %t118 = load ptr, ptr %t117
  %t119 = load ptr, ptr %t2
  %t120 = call ptr @Conduit_Features_Profiles_ProfileReader__g0__t159_ReadProfile__g0(ptr %t115, ptr %t118, ptr %t119)
  %t121 = load ptr, ptr @glitch_exception_pending
  %t122 = icmp ne ptr %t121, null
  br i1 %t122, label %exception_unwind, label %call_continue_16
call_continue_16:
  %t123 = call ptr @glitch_task_get_result_ptr(ptr %t120)
  call void @glitch_retain_Conduit_Features_Profiles_ProfileEnvelope__g0__t158(ptr %t123)
  ret ptr %t123
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Followers_FollowersController__g0__t150_ctor(ptr %this, ptr %mediator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %mediator, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Followers_FollowersController__g0__t150, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Followers_FollowersController__g0__t150_Follow__g0(ptr %this, ptr %username, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %username, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Followers_FollowersController__g0__t150, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Followers_FollowersController__g0__t150_Unfollow__g0(ptr %this, ptr %username, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %username, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Followers_FollowersController__g0__t150, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Profiles_Query__g0__t152_ctor(ptr %this, ptr %Username) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Username, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t2, i32 0, i32 9
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Profiles_QueryValidator__g0__t153_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_31, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  call void @glitch_delegate_release(ptr %t1)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Profiles_QueryHandler__g0__t154_ctor(ptr %this, ptr %profileReader) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %profileReader, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t2, i32 0, i32 4
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Features_Profiles_IProfileReader__g0__t155(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Features_Profiles_IProfileReader__g0__t155(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Profiles_QueryHandler__g0__t154_Handle__g0(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t3, i32 0, i32 4
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t6, i32 0, i32 9
  %t8 = load ptr, ptr %t7
  %t9 = load ptr, ptr %t2
  %t10 = call ptr @Conduit_Features_Profiles_ProfileReader__g0__t159_ReadProfile__g0(ptr %t5, ptr %t8, ptr %t9)
  %t11 = load ptr, ptr @glitch_exception_pending
  %t12 = icmp ne ptr %t11, null
  br i1 %t12, label %exception_unwind, label %call_continue_0
call_continue_0:
  ret ptr %t10
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Profiles_MappingProfile__g0__t156_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Profiles_ProfileEnvelope__g0__t158_ctor(ptr %this, ptr %Profile) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Profile, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileEnvelope__g0__t158, ptr %t2, i32 0, i32 2
  call void @glitch_retain_Conduit_Features_Profiles_Profile__g0__t157(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Features_Profiles_Profile__g0__t157(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Profiles_ProfileReader__g0__t159_ctor(ptr %this, ptr %context, ptr %currentUserAccessor, ptr %mapper) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %currentUserAccessor, ptr %t2
  %t3 = alloca ptr
  store ptr %mapper, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_drop_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %t12, i32 0, i32 4
  %t14 = load ptr, ptr %t3
  call void @glitch_retain_IMapper__g0__t79(ptr %t14)
  %t15 = load ptr, ptr %t13
  call void @glitch_drop_IMapper__g0__t79(ptr %t15)
  store ptr %t14, ptr %t13
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Profiles_ProfileReader__g0__t159_ReadProfile__g0(ptr %this, ptr %username, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %username, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = alloca ptr
  store ptr null, ptr %t6
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t8
  %t10 = call ptr @Conduit_Infrastructure_CurrentUserAccessor__g0__t183_GetCurrentUsername__g0(ptr %t9)
  %t11 = load ptr, ptr @glitch_exception_pending
  %t12 = icmp ne ptr %t11, null
  br i1 %t12, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t10, ptr %t3
  %t14 = getelementptr %glitch.delegate, ptr null, i32 1
  %t15 = ptrtoint ptr %t14 to i64
  %t13 = call ptr @glitch_calloc(i64 1, i64 %t15)
  %t16 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 0
  store i64 1, ptr %t16
  %t17 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 1
  store ptr @glitch_lambda_32, ptr %t17
  %t18 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 2
  %t19 = getelementptr inbounds %glitch.delegate, ptr %t13, i32 0, i32 3
  %t20 = getelementptr %glitch.lambda.32.env, ptr null, i32 1
  %t21 = ptrtoint ptr %t20 to i64
  %t22 = call ptr @glitch_calloc(i64 1, i64 %t21)
  %t23 = load ptr, ptr %t1
  %t24 = getelementptr inbounds %glitch.lambda.32.env, ptr %t22, i32 0, i32 0
  store ptr %t23, ptr %t24
  store ptr %t22, ptr %t18
  store ptr @glitch_lambda_32_destroy, ptr %t19
  %t27 = load ptr, ptr %t2
  %t28 = call ptr @FirstOrDefaultAsync__object_object(ptr %t13, ptr %t27)
  %t29 = load ptr, ptr @glitch_exception_pending
  %t30 = icmp ne ptr %t29, null
  br i1 %t30, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t31 = call ptr @glitch_task_get_result_ptr(ptr %t28)
  store ptr %t31, ptr %t4
  %t32 = load ptr, ptr %t4
  %t33 = icmp eq ptr %t32, null
  br i1 %t33, label %if_then_2, label %if_else_3
if_then_2:
  %t34 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t35 = ptrtoint ptr %t34 to i64
  %t36 = call ptr @glitch_calloc(i64 1, i64 %t35)
  %t37 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t36, i32 0, i32 0
  store i64 1, ptr %t37
  %t38 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t36, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t38
  %t39 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t40 = load ptr, ptr %t39
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t36, ptr null, ptr null)
  %t41 = load ptr, ptr @glitch_exception_pending
  %t42 = icmp ne ptr %t41, null
  br i1 %t42, label %exception_unwind, label %call_continue_5
call_continue_5:
  store ptr %t36, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_3:
  br label %if_end_4
if_end_4:
  %t43 = load ptr, ptr %t4
  %t44 = icmp eq ptr %t43, null
  br i1 %t44, label %if_then_6, label %if_else_7
if_then_6:
  %t45 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t46 = ptrtoint ptr %t45 to i64
  %t47 = call ptr @glitch_calloc(i64 1, i64 %t46)
  %t48 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t47, i32 0, i32 0
  store i64 1, ptr %t48
  %t49 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t47, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t49
  %t50 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t51 = load ptr, ptr %t50
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t47, ptr null, ptr null)
  %t52 = load ptr, ptr @glitch_exception_pending
  %t53 = icmp ne ptr %t52, null
  br i1 %t53, label %exception_unwind, label %call_continue_9
call_continue_9:
  store ptr %t47, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_7:
  br label %if_end_8
if_end_8:
  %t54 = load ptr, ptr %t0
  %t55 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileReader__g0__t159, ptr %t54, i32 0, i32 4
  %t56 = load ptr, ptr %t55
  %t57 = load ptr, ptr %t4
  %t58 = call ptr @Mapper__g0__t80_Map__g1(ptr %t56, ptr %t57)
  %t59 = load ptr, ptr @glitch_exception_pending
  %t60 = icmp ne ptr %t59, null
  br i1 %t60, label %exception_unwind, label %call_continue_10
call_continue_10:
  store ptr %t58, ptr %t5
  %t61 = load ptr, ptr %t3
  %t62 = icmp ne ptr %t61, null
  br i1 %t62, label %if_then_11, label %if_else_12
if_then_11:
  %t64 = getelementptr %glitch.delegate, ptr null, i32 1
  %t65 = ptrtoint ptr %t64 to i64
  %t63 = call ptr @glitch_calloc(i64 1, i64 %t65)
  %t66 = getelementptr inbounds %glitch.delegate, ptr %t63, i32 0, i32 0
  store i64 1, ptr %t66
  %t67 = getelementptr inbounds %glitch.delegate, ptr %t63, i32 0, i32 1
  store ptr @glitch_lambda_33, ptr %t67
  %t68 = getelementptr inbounds %glitch.delegate, ptr %t63, i32 0, i32 2
  %t69 = getelementptr inbounds %glitch.delegate, ptr %t63, i32 0, i32 3
  %t70 = getelementptr %glitch.lambda.33.env, ptr null, i32 1
  %t71 = ptrtoint ptr %t70 to i64
  %t72 = call ptr @glitch_calloc(i64 1, i64 %t71)
  %t73 = load ptr, ptr %t3
  call void @glitch_string_retain(ptr %t73)
  %t74 = getelementptr inbounds %glitch.lambda.33.env, ptr %t72, i32 0, i32 0
  store ptr %t73, ptr %t74
  store ptr %t72, ptr %t68
  store ptr @glitch_lambda_33_destroy, ptr %t69
  %t77 = load ptr, ptr %t2
  %t78 = call ptr @FirstOrDefaultAsync__object_object(ptr %t63, ptr %t77)
  %t79 = load ptr, ptr @glitch_exception_pending
  %t80 = icmp ne ptr %t79, null
  br i1 %t80, label %exception_unwind, label %call_continue_14
call_continue_14:
  %t81 = call ptr @glitch_task_get_result_ptr(ptr %t78)
  store ptr %t81, ptr %t6
  %t82 = load ptr, ptr %t6
  %t83 = icmp eq ptr %t82, null
  br i1 %t83, label %if_then_15, label %if_else_16
if_then_15:
  %t84 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t85 = ptrtoint ptr %t84 to i64
  %t86 = call ptr @glitch_calloc(i64 1, i64 %t85)
  %t87 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t86, i32 0, i32 0
  store i64 1, ptr %t87
  %t88 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t86, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t88
  %t89 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t90 = load ptr, ptr %t89
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t86, ptr null, ptr null)
  %t91 = load ptr, ptr @glitch_exception_pending
  %t92 = icmp ne ptr %t91, null
  br i1 %t92, label %exception_unwind, label %call_continue_18
call_continue_18:
  store ptr %t86, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_16:
  br label %if_end_17
if_end_17:
  %t94 = getelementptr %glitch.delegate, ptr null, i32 1
  %t95 = ptrtoint ptr %t94 to i64
  %t93 = call ptr @glitch_calloc(i64 1, i64 %t95)
  %t96 = getelementptr inbounds %glitch.delegate, ptr %t93, i32 0, i32 0
  store i64 1, ptr %t96
  %t97 = getelementptr inbounds %glitch.delegate, ptr %t93, i32 0, i32 1
  store ptr @glitch_lambda_34, ptr %t97
  %t98 = getelementptr inbounds %glitch.delegate, ptr %t93, i32 0, i32 2
  %t99 = getelementptr inbounds %glitch.delegate, ptr %t93, i32 0, i32 3
  %t100 = getelementptr %glitch.lambda.34.env, ptr null, i32 1
  %t101 = ptrtoint ptr %t100 to i64
  %t102 = call ptr @glitch_calloc(i64 1, i64 %t101)
  %t103 = load ptr, ptr %t4
  %t104 = getelementptr inbounds %glitch.lambda.34.env, ptr %t102, i32 0, i32 0
  store ptr %t103, ptr %t104
  store ptr %t102, ptr %t98
  store ptr @glitch_lambda_34_destroy, ptr %t99
  %t107 = call ptr @Any__object(ptr %t93)
  %t108 = load ptr, ptr @glitch_exception_pending
  %t109 = icmp ne ptr %t108, null
  br i1 %t109, label %exception_unwind, label %call_continue_19
call_continue_19:
  %t110 = icmp ne ptr %t107, null
  br i1 %t110, label %if_then_20, label %if_else_21
if_then_20:
  br label %if_end_22
if_else_21:
  br label %if_end_22
if_end_22:
  br label %if_end_13
if_else_12:
  br label %if_end_13
if_end_13:
  %t111 = getelementptr %glitch.Conduit_Features_Profiles_ProfileEnvelope__g0__t158, ptr null, i32 1
  %t112 = ptrtoint ptr %t111 to i64
  %t113 = call ptr @glitch_calloc(i64 1, i64 %t112)
  %t114 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileEnvelope__g0__t158, ptr %t113, i32 0, i32 0
  store i64 1, ptr %t114
  %t115 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfileEnvelope__g0__t158, ptr %t113, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Profiles_ProfileEnvelope__g0__t158, ptr %t115
  %t116 = load ptr, ptr %t5
  call void @Conduit_Features_Profiles_ProfileEnvelope__g0__t158_ctor(ptr %t113, ptr %t116)
  %t117 = load ptr, ptr @glitch_exception_pending
  %t118 = icmp ne ptr %t117, null
  br i1 %t118, label %exception_unwind, label %call_continue_23
call_continue_23:
  %t119 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t119)
  ret ptr %t113
exception_unwind:
  %t120 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t120)
  ret ptr null
}

define void @Conduit_Features_Profiles_ProfilesController__g0__t160_ctor(ptr %this, ptr %mediator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %mediator, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfilesController__g0__t160, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Profiles_ProfilesController__g0__t160_Get__g0(ptr %this, ptr %username, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %username, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Profiles_ProfilesController__g0__t160, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Tags_QueryHandler__g0__t163_ctor(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Tags_QueryHandler__g0__t163_Handle__g0(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t5
  %t7 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t6, i32 0, i32 13
  %t8 = load ptr, ptr %t7
  %t10 = getelementptr %glitch.delegate, ptr null, i32 1
  %t11 = ptrtoint ptr %t10 to i64
  %t9 = call ptr @glitch_calloc(i64 1, i64 %t11)
  %t12 = getelementptr inbounds %glitch.delegate, ptr %t9, i32 0, i32 0
  store i64 1, ptr %t12
  %t13 = getelementptr inbounds %glitch.delegate, ptr %t9, i32 0, i32 1
  store ptr @glitch_lambda_35, ptr %t13
  %t14 = getelementptr inbounds %glitch.delegate, ptr %t9, i32 0, i32 2
  %t15 = getelementptr inbounds %glitch.delegate, ptr %t9, i32 0, i32 3
  store ptr null, ptr %t14
  store ptr null, ptr %t15
  call void @glitch_delegate_release(ptr %t9)
  %t16 = load ptr, ptr %t2
  %t17 = call ptr @glitch_task_get_result_ptr(ptr null)
  store ptr %t17, ptr %t3
  %t18 = getelementptr %glitch.Conduit_Features_Tags_TagsEnvelope__g0__t165, ptr null, i32 1
  %t19 = ptrtoint ptr %t18 to i64
  %t20 = call ptr @glitch_calloc(i64 1, i64 %t19)
  %t21 = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsEnvelope__g0__t165, ptr %t20, i32 0, i32 0
  store i64 1, ptr %t21
  %t22 = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsEnvelope__g0__t165, ptr %t20, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Tags_TagsEnvelope__g0__t165, ptr %t22
  %t23 = load ptr, ptr %t3
  %t24 = icmp eq ptr %t23, null
  %t25 = alloca ptr
  br i1 %t24, label %conditional_true_0, label %conditional_false_1
conditional_true_0:
  store ptr null, ptr %t25
  br label %conditional_end_2
conditional_false_1:
  %t26 = load ptr, ptr %t3
  %t28 = getelementptr %glitch.delegate, ptr null, i32 1
  %t29 = ptrtoint ptr %t28 to i64
  %t27 = call ptr @glitch_calloc(i64 1, i64 %t29)
  %t30 = getelementptr inbounds %glitch.delegate, ptr %t27, i32 0, i32 0
  store i64 1, ptr %t30
  %t31 = getelementptr inbounds %glitch.delegate, ptr %t27, i32 0, i32 1
  store ptr @glitch_lambda_36, ptr %t31
  %t32 = getelementptr inbounds %glitch.delegate, ptr %t27, i32 0, i32 2
  %t33 = getelementptr inbounds %glitch.delegate, ptr %t27, i32 0, i32 3
  store ptr null, ptr %t32
  store ptr null, ptr %t33
  call void @glitch_delegate_release(ptr %t27)
  store ptr null, ptr %t25
  br label %conditional_end_2
conditional_end_2:
  %t34 = load ptr, ptr %t25
  %t36 = icmp eq ptr null, null
  %t35 = alloca ptr
  br i1 %t36, label %coalesce_right_4, label %coalesce_left_3
coalesce_left_3:
  store ptr null, ptr %t35
  br label %coalesce_end_5
coalesce_right_4:
  %t37 = call ptr @glitch_calloc(i64 1, i64 24)
  %t38 = call ptr @glitch_calloc(i64 4, i64 8)
  %t39 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 1
  store i64 4, ptr %t39
  %t40 = getelementptr inbounds %glitch.list, ptr %t37, i32 0, i32 2
  store ptr %t38, ptr %t40
  store ptr %t37, ptr %t35
  br label %coalesce_end_5
coalesce_end_5:
  %t41 = load ptr, ptr %t35
  %t42 = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsEnvelope__g0__t165, ptr %t20, i32 0, i32 2
  store ptr %t41, ptr %t42
  ret ptr %t20
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Tags_TagsController__g0__t164_ctor(ptr %this, ptr %mediator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %mediator, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsController__g0__t164, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Tags_TagsController__g0__t164_Get__g0(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Tags_TagsController__g0__t164, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  %t5 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Users_UserData__g0__t167_ctor(ptr %this, ptr %Username, ptr %Email, ptr %Password) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Username, ptr %t1
  %t2 = alloca ptr
  store ptr %Email, ptr %t2
  %t3 = alloca ptr
  store ptr %Password, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %t8, i32 0, i32 3
  %t10 = load ptr, ptr %t2
  call void @glitch_string_retain(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %t12, i32 0, i32 4
  %t14 = load ptr, ptr %t3
  call void @glitch_string_retain(ptr %t14)
  %t15 = load ptr, ptr %t13
  call void @glitch_string_release(ptr %t15)
  store ptr %t14, ptr %t13
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Users_Command__g0__t168_ctor__UserData(ptr %this, ptr %User) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %User, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t2, i32 0, i32 7
  call void @glitch_retain_Conduit_Features_Users_UserData__g0__t167(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Features_Users_UserData__g0__t167(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Users_CommandValidator__g0__t169_ctor__empty(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_37, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  call void @glitch_delegate_release(ptr %t1)
  %t9 = getelementptr %glitch.delegate, ptr null, i32 1
  %t10 = ptrtoint ptr %t9 to i64
  %t8 = call ptr @glitch_calloc(i64 1, i64 %t10)
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t8, i32 0, i32 0
  store i64 1, ptr %t11
  %t12 = getelementptr inbounds %glitch.delegate, ptr %t8, i32 0, i32 1
  store ptr @glitch_lambda_38, ptr %t12
  %t13 = getelementptr inbounds %glitch.delegate, ptr %t8, i32 0, i32 2
  %t14 = getelementptr inbounds %glitch.delegate, ptr %t8, i32 0, i32 3
  store ptr null, ptr %t13
  store ptr null, ptr %t14
  call void @glitch_delegate_release(ptr %t8)
  %t16 = getelementptr %glitch.delegate, ptr null, i32 1
  %t17 = ptrtoint ptr %t16 to i64
  %t15 = call ptr @glitch_calloc(i64 1, i64 %t17)
  %t18 = getelementptr inbounds %glitch.delegate, ptr %t15, i32 0, i32 0
  store i64 1, ptr %t18
  %t19 = getelementptr inbounds %glitch.delegate, ptr %t15, i32 0, i32 1
  store ptr @glitch_lambda_39, ptr %t19
  %t20 = getelementptr inbounds %glitch.delegate, ptr %t15, i32 0, i32 2
  %t21 = getelementptr inbounds %glitch.delegate, ptr %t15, i32 0, i32 3
  store ptr null, ptr %t20
  store ptr null, ptr %t21
  call void @glitch_delegate_release(ptr %t15)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Users_Handler__g0__t170_ctor__ConduitContext_IPasswordHasher_IJwtTokenGenerator_IMapper(ptr %this, ptr %context, ptr %passwordHasher, ptr %jwtTokenGenerator, ptr %mapper) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %passwordHasher, ptr %t2
  %t3 = alloca ptr
  store ptr %jwtTokenGenerator, ptr %t3
  %t4 = alloca ptr
  store ptr %mapper, ptr %t4
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t5, i32 0, i32 2
  %t7 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t7)
  %t8 = load ptr, ptr %t6
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t8)
  store ptr %t7, ptr %t6
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t9, i32 0, i32 4
  %t11 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_Security_IPasswordHasher__g0__t191(ptr %t11)
  %t12 = load ptr, ptr %t10
  call void @glitch_drop_Conduit_Infrastructure_Security_IPasswordHasher__g0__t191(ptr %t12)
  store ptr %t11, ptr %t10
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t13, i32 0, i32 5
  %t15 = load ptr, ptr %t3
  call void @glitch_retain_Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190(ptr %t15)
  %t16 = load ptr, ptr %t14
  call void @glitch_drop_Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190(ptr %t16)
  store ptr %t15, ptr %t14
  %t17 = load ptr, ptr %t0
  %t18 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t17, i32 0, i32 6
  %t19 = load ptr, ptr %t4
  call void @glitch_retain_IMapper__g0__t79(ptr %t19)
  %t20 = load ptr, ptr %t18
  call void @glitch_drop_IMapper__g0__t79(ptr %t20)
  store ptr %t19, ptr %t18
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Users_Handler__g0__t170_ctor__ConduitContext_IPasswordHasher_ICurrentUserAccessor_IMapper(ptr %this, ptr %context, ptr %passwordHasher, ptr %currentUserAccessor, ptr %mapper) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %passwordHasher, ptr %t2
  %t3 = alloca ptr
  store ptr %currentUserAccessor, ptr %t3
  %t4 = alloca ptr
  store ptr %mapper, ptr %t4
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t5, i32 0, i32 2
  %t7 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t7)
  %t8 = load ptr, ptr %t6
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t8)
  store ptr %t7, ptr %t6
  %t9 = load ptr, ptr %t0
  %t10 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t9, i32 0, i32 4
  %t11 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_Security_IPasswordHasher__g0__t191(ptr %t11)
  %t12 = load ptr, ptr %t10
  call void @glitch_drop_Conduit_Infrastructure_Security_IPasswordHasher__g0__t191(ptr %t12)
  store ptr %t11, ptr %t10
  %t13 = load ptr, ptr %t0
  %t14 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t13, i32 0, i32 3
  %t15 = load ptr, ptr %t3
  call void @glitch_retain_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t15)
  %t16 = load ptr, ptr %t14
  call void @glitch_drop_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t16)
  store ptr %t15, ptr %t14
  %t17 = load ptr, ptr %t0
  %t18 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t17, i32 0, i32 6
  %t19 = load ptr, ptr %t4
  call void @glitch_retain_IMapper__g0__t79(ptr %t19)
  %t20 = load ptr, ptr %t18
  call void @glitch_drop_IMapper__g0__t79(ptr %t20)
  store ptr %t19, ptr %t18
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Users_Handler__g0__t170_Handle__g0__Command_CancellationToken(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t6, i32 0, i32 2
  %t8 = load ptr, ptr %t7
  %t9 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t8, i32 0, i32 12
  %t10 = load ptr, ptr %t9
  %t12 = getelementptr %glitch.delegate, ptr null, i32 1
  %t13 = ptrtoint ptr %t12 to i64
  %t11 = call ptr @glitch_calloc(i64 1, i64 %t13)
  %t14 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 0
  store i64 1, ptr %t14
  %t15 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 1
  store ptr @glitch_lambda_40, ptr %t15
  %t16 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 2
  %t17 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 3
  %t18 = getelementptr %glitch.lambda.40.env, ptr null, i32 1
  %t19 = ptrtoint ptr %t18 to i64
  %t20 = call ptr @glitch_calloc(i64 1, i64 %t19)
  %t21 = load ptr, ptr %t1
  %t22 = getelementptr inbounds %glitch.lambda.40.env, ptr %t20, i32 0, i32 0
  store ptr %t21, ptr %t22
  store ptr %t20, ptr %t16
  store ptr @glitch_lambda_40_destroy, ptr %t17
  %t25 = call ptr @Where(ptr %t10, ptr %t11)
  %t26 = load ptr, ptr @glitch_exception_pending
  %t27 = icmp ne ptr %t26, null
  br i1 %t27, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t28 = load ptr, ptr %t2
  %t29 = call ptr @glitch_task_get_result_ptr(ptr null)
  %t30 = icmp ne ptr %t29, null
  br i1 %t30, label %if_then_1, label %if_else_2
if_then_1:
  %t31 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t32 = ptrtoint ptr %t31 to i64
  %t33 = call ptr @glitch_calloc(i64 1, i64 %t32)
  %t34 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t33, i32 0, i32 0
  store i64 1, ptr %t34
  %t35 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t33, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t35
  %t36 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 3
  %t37 = load ptr, ptr %t36
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t33, ptr null, ptr null)
  %t38 = load ptr, ptr @glitch_exception_pending
  %t39 = icmp ne ptr %t38, null
  br i1 %t39, label %exception_unwind, label %call_continue_4
call_continue_4:
  store ptr %t33, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_2:
  br label %if_end_3
if_end_3:
  %t40 = load ptr, ptr %t0
  %t41 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t40, i32 0, i32 2
  %t42 = load ptr, ptr %t41
  %t43 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t42, i32 0, i32 12
  %t44 = load ptr, ptr %t43
  %t46 = getelementptr %glitch.delegate, ptr null, i32 1
  %t47 = ptrtoint ptr %t46 to i64
  %t45 = call ptr @glitch_calloc(i64 1, i64 %t47)
  %t48 = getelementptr inbounds %glitch.delegate, ptr %t45, i32 0, i32 0
  store i64 1, ptr %t48
  %t49 = getelementptr inbounds %glitch.delegate, ptr %t45, i32 0, i32 1
  store ptr @glitch_lambda_41, ptr %t49
  %t50 = getelementptr inbounds %glitch.delegate, ptr %t45, i32 0, i32 2
  %t51 = getelementptr inbounds %glitch.delegate, ptr %t45, i32 0, i32 3
  %t52 = getelementptr %glitch.lambda.41.env, ptr null, i32 1
  %t53 = ptrtoint ptr %t52 to i64
  %t54 = call ptr @glitch_calloc(i64 1, i64 %t53)
  %t55 = load ptr, ptr %t1
  %t56 = getelementptr inbounds %glitch.lambda.41.env, ptr %t54, i32 0, i32 0
  store ptr %t55, ptr %t56
  store ptr %t54, ptr %t50
  store ptr @glitch_lambda_41_destroy, ptr %t51
  %t59 = call ptr @Where(ptr %t44, ptr %t45)
  %t60 = load ptr, ptr @glitch_exception_pending
  %t61 = icmp ne ptr %t60, null
  br i1 %t61, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t62 = load ptr, ptr %t2
  %t63 = call ptr @glitch_task_get_result_ptr(ptr null)
  %t64 = icmp ne ptr %t63, null
  br i1 %t64, label %if_then_6, label %if_else_7
if_then_6:
  %t65 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t66 = ptrtoint ptr %t65 to i64
  %t67 = call ptr @glitch_calloc(i64 1, i64 %t66)
  %t68 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t67, i32 0, i32 0
  store i64 1, ptr %t68
  %t69 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t67, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t69
  %t70 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 3
  %t71 = load ptr, ptr %t70
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t67, ptr null, ptr null)
  %t72 = load ptr, ptr @glitch_exception_pending
  %t73 = icmp ne ptr %t72, null
  br i1 %t73, label %exception_unwind, label %call_continue_9
call_continue_9:
  store ptr %t67, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_7:
  br label %if_end_8
if_end_8:
  store ptr null, ptr %t3
  %t74 = getelementptr %glitch.Conduit_Domain_Person__g0__t106, ptr null, i32 1
  %t75 = ptrtoint ptr %t74 to i64
  %t76 = call ptr @glitch_calloc(i64 1, i64 %t75)
  %t77 = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %t76, i32 0, i32 0
  store i64 1, ptr %t77
  %t78 = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %t76, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Domain_Person__g0__t106, ptr %t78
  %t79 = load ptr, ptr %t1
  %t80 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t79, i32 0, i32 7
  %t81 = load ptr, ptr %t80
  %t82 = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %t81, i32 0, i32 2
  %t83 = load ptr, ptr %t82
  call void @glitch_string_retain(ptr %t83)
  %t84 = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %t76, i32 0, i32 3
  store ptr %t83, ptr %t84
  %t85 = load ptr, ptr %t1
  %t86 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t85, i32 0, i32 7
  %t87 = load ptr, ptr %t86
  %t88 = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %t87, i32 0, i32 3
  %t89 = load ptr, ptr %t88
  call void @glitch_string_retain(ptr %t89)
  %t90 = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %t76, i32 0, i32 4
  store ptr %t89, ptr %t90
  %t91 = load ptr, ptr %t0
  %t92 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t91, i32 0, i32 4
  %t93 = load ptr, ptr %t92
  %t94 = load ptr, ptr %t1
  %t95 = getelementptr inbounds %glitch.Conduit_Features_Users_Command__g0__t168, ptr %t94, i32 0, i32 7
  %t96 = load ptr, ptr %t95
  %t97 = getelementptr inbounds %glitch.Conduit_Features_Users_UserData__g0__t167, ptr %t96, i32 0, i32 4
  %t98 = load ptr, ptr %t97
  %t100 = icmp eq ptr %t98, null
  %t99 = alloca ptr
  br i1 %t100, label %coalesce_right_11, label %coalesce_left_10
coalesce_left_10:
  store ptr %t98, ptr %t99
  br label %coalesce_end_12
coalesce_right_11:
  store ptr null, ptr @glitch_exception_pending
  br label %exception_unwind
coalesce_end_12:
  %t101 = load ptr, ptr %t99
  %t102 = load ptr, ptr %t3
  %t103 = call ptr @Conduit_Infrastructure_Security_PasswordHasher__g0__t194_Hash__g0(ptr %t93, ptr %t101, ptr %t102)
  %t104 = load ptr, ptr @glitch_exception_pending
  %t105 = icmp ne ptr %t104, null
  br i1 %t105, label %exception_unwind, label %call_continue_13
call_continue_13:
  %t106 = call ptr @glitch_task_get_result_ptr(ptr %t103)
  %t107 = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %t76, i32 0, i32 10
  store ptr %t106, ptr %t107
  %t108 = load ptr, ptr %t3
  %t109 = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %t76, i32 0, i32 11
  store ptr %t108, ptr %t109
  %t110 = load ptr, ptr %t4
  call void @glitch_drop_Conduit_Domain_Person__g0__t106(ptr %t110)
  store ptr %t76, ptr %t4
  %t111 = load ptr, ptr %t4
  %t112 = load ptr, ptr %t2
  %t113 = call ptr @AddAsync(ptr %t111, ptr %t112)
  %t114 = load ptr, ptr @glitch_exception_pending
  %t115 = icmp ne ptr %t114, null
  br i1 %t115, label %exception_unwind, label %call_continue_14
call_continue_14:
  %t116 = call ptr @glitch_task_get_result_ptr(ptr %t113)
  %t117 = load ptr, ptr %t2
  %t118 = call ptr @SaveChangesAsync__object(ptr %t117)
  %t119 = load ptr, ptr @glitch_exception_pending
  %t120 = icmp ne ptr %t119, null
  br i1 %t120, label %exception_unwind, label %call_continue_15
call_continue_15:
  %t121 = call ptr @glitch_task_get_result_ptr(ptr %t118)
  %t122 = load ptr, ptr %t0
  %t123 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t122, i32 0, i32 6
  %t124 = load ptr, ptr %t123
  %t125 = load ptr, ptr %t4
  %t126 = call ptr @Mapper__g0__t80_Map__g1(ptr %t124, ptr %t125)
  %t127 = load ptr, ptr @glitch_exception_pending
  %t128 = icmp ne ptr %t127, null
  br i1 %t128, label %exception_unwind, label %call_continue_16
call_continue_16:
  store ptr %t126, ptr %t5
  %t129 = load ptr, ptr %t0
  %t130 = getelementptr inbounds %glitch.Conduit_Features_Users_Handler__g0__t170, ptr %t129, i32 0, i32 5
  %t131 = load ptr, ptr %t130
  %t132 = load ptr, ptr %t4
  %t133 = getelementptr inbounds %glitch.Conduit_Domain_Person__g0__t106, ptr %t132, i32 0, i32 3
  %t134 = load ptr, ptr %t133
  %t136 = icmp eq ptr %t134, null
  %t135 = alloca ptr
  br i1 %t136, label %coalesce_right_18, label %coalesce_left_17
coalesce_left_17:
  store ptr %t134, ptr %t135
  br label %coalesce_end_19
coalesce_right_18:
  store ptr null, ptr @glitch_exception_pending
  br label %exception_unwind
coalesce_end_19:
  %t137 = load ptr, ptr %t135
  %t138 = call ptr @Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193_CreateToken__g0(ptr %t131, ptr %t137)
  %t139 = load ptr, ptr @glitch_exception_pending
  %t140 = icmp ne ptr %t139, null
  br i1 %t140, label %exception_unwind, label %call_continue_20
call_continue_20:
  %t141 = getelementptr %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr null, i32 1
  %t142 = ptrtoint ptr %t141 to i64
  %t143 = call ptr @glitch_calloc(i64 1, i64 %t142)
  %t144 = getelementptr inbounds %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr %t143, i32 0, i32 0
  store i64 1, ptr %t144
  %t145 = getelementptr inbounds %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr %t143, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Users_UserEnvelope__g0__t179, ptr %t145
  %t146 = load ptr, ptr %t5
  call void @Conduit_Features_Users_UserEnvelope__g0__t179_ctor(ptr %t143, ptr %t146)
  %t147 = load ptr, ptr @glitch_exception_pending
  %t148 = icmp ne ptr %t147, null
  br i1 %t148, label %exception_unwind, label %call_continue_21
call_continue_21:
  %t149 = load ptr, ptr %t4
  call void @glitch_drop_Conduit_Domain_Person__g0__t106(ptr %t149)
  ret ptr %t143
exception_unwind:
  %t150 = load ptr, ptr %t4
  call void @glitch_drop_Conduit_Domain_Person__g0__t106(ptr %t150)
  ret ptr null
}

define void @Conduit_Features_Users_Query__g0__t172_ctor(ptr %this, ptr %Username) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %Username, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_Query__g0__t172, ptr %t2, i32 0, i32 9
  %t4 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_string_release(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Users_QueryValidator__g0__t173_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_42, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  call void @glitch_delegate_release(ptr %t1)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Users_QueryHandler__g0__t174_ctor(ptr %this, ptr %context, ptr %jwtTokenGenerator, ptr %mapper) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %jwtTokenGenerator, ptr %t2
  %t3 = alloca ptr
  store ptr %mapper, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t6)
  %t7 = load ptr, ptr %t5
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t7)
  store ptr %t6, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t8, i32 0, i32 5
  %t10 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190(ptr %t10)
  %t11 = load ptr, ptr %t9
  call void @glitch_drop_Conduit_Infrastructure_Security_IJwtTokenGenerator__g0__t190(ptr %t11)
  store ptr %t10, ptr %t9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t12, i32 0, i32 6
  %t14 = load ptr, ptr %t3
  call void @glitch_retain_IMapper__g0__t79(ptr %t14)
  %t15 = load ptr, ptr %t13
  call void @glitch_drop_IMapper__g0__t79(ptr %t15)
  store ptr %t14, ptr %t13
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Users_QueryHandler__g0__t174_Handle__g0(ptr %this, ptr %message, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %message, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t6 = getelementptr %glitch.delegate, ptr null, i32 1
  %t7 = ptrtoint ptr %t6 to i64
  %t5 = call ptr @glitch_calloc(i64 1, i64 %t7)
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t5, i32 0, i32 0
  store i64 1, ptr %t8
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t5, i32 0, i32 1
  store ptr @glitch_lambda_43, ptr %t9
  %t10 = getelementptr inbounds %glitch.delegate, ptr %t5, i32 0, i32 2
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t5, i32 0, i32 3
  %t12 = getelementptr %glitch.lambda.43.env, ptr null, i32 1
  %t13 = ptrtoint ptr %t12 to i64
  %t14 = call ptr @glitch_calloc(i64 1, i64 %t13)
  %t15 = load ptr, ptr %t1
  %t16 = getelementptr inbounds %glitch.lambda.43.env, ptr %t14, i32 0, i32 0
  store ptr %t15, ptr %t16
  store ptr %t14, ptr %t10
  store ptr @glitch_lambda_43_destroy, ptr %t11
  %t19 = load ptr, ptr %t2
  %t20 = call ptr @FirstOrDefaultAsync__object_object(ptr %t5, ptr %t19)
  %t21 = load ptr, ptr @glitch_exception_pending
  %t22 = icmp ne ptr %t21, null
  br i1 %t22, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t23 = call ptr @glitch_task_get_result_ptr(ptr %t20)
  store ptr %t23, ptr %t3
  %t24 = load ptr, ptr %t3
  %t25 = icmp eq ptr %t24, null
  br i1 %t25, label %if_then_1, label %if_else_2
if_then_1:
  %t26 = getelementptr %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr null, i32 1
  %t27 = ptrtoint ptr %t26 to i64
  %t28 = call ptr @glitch_calloc(i64 1, i64 %t27)
  %t29 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t28, i32 0, i32 0
  store i64 1, ptr %t29
  %t30 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t28, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t30
  %t31 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_Constants__g0__t185, ptr null, i32 0, i32 2
  %t32 = load ptr, ptr %t31
  call void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %t28, ptr null, ptr null)
  %t33 = load ptr, ptr @glitch_exception_pending
  %t34 = icmp ne ptr %t33, null
  br i1 %t34, label %exception_unwind, label %call_continue_4
call_continue_4:
  store ptr %t28, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_2:
  br label %if_end_3
if_end_3:
  %t35 = load ptr, ptr %t0
  %t36 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t35, i32 0, i32 6
  %t37 = load ptr, ptr %t36
  %t38 = load ptr, ptr %t3
  %t39 = call ptr @Mapper__g0__t80_Map__g1(ptr %t37, ptr %t38)
  %t40 = load ptr, ptr @glitch_exception_pending
  %t41 = icmp ne ptr %t40, null
  br i1 %t41, label %exception_unwind, label %call_continue_5
call_continue_5:
  store ptr %t39, ptr %t4
  %t42 = load ptr, ptr %t0
  %t43 = getelementptr inbounds %glitch.Conduit_Features_Users_QueryHandler__g0__t174, ptr %t42, i32 0, i32 5
  %t44 = load ptr, ptr %t43
  %t46 = icmp eq ptr null, null
  %t45 = alloca ptr
  br i1 %t46, label %coalesce_right_7, label %coalesce_left_6
coalesce_left_6:
  store ptr null, ptr %t45
  br label %coalesce_end_8
coalesce_right_7:
  store ptr null, ptr @glitch_exception_pending
  br label %exception_unwind
coalesce_end_8:
  %t47 = load ptr, ptr %t45
  %t48 = call ptr @Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193_CreateToken__g0(ptr %t44, ptr %t47)
  %t49 = load ptr, ptr @glitch_exception_pending
  %t50 = icmp ne ptr %t49, null
  br i1 %t50, label %exception_unwind, label %call_continue_9
call_continue_9:
  %t51 = getelementptr %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr null, i32 1
  %t52 = ptrtoint ptr %t51 to i64
  %t53 = call ptr @glitch_calloc(i64 1, i64 %t52)
  %t54 = getelementptr inbounds %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr %t53, i32 0, i32 0
  store i64 1, ptr %t54
  %t55 = getelementptr inbounds %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr %t53, i32 0, i32 1
  store ptr @glitch_destroy_Conduit_Features_Users_UserEnvelope__g0__t179, ptr %t55
  %t56 = load ptr, ptr %t4
  call void @Conduit_Features_Users_UserEnvelope__g0__t179_ctor(ptr %t53, ptr %t56)
  %t57 = load ptr, ptr @glitch_exception_pending
  %t58 = icmp ne ptr %t57, null
  br i1 %t58, label %exception_unwind, label %call_continue_10
call_continue_10:
  ret ptr %t53
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Users_MappingProfile__g0__t177_ctor(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Users_UserEnvelope__g0__t179_ctor(ptr %this, ptr %User) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %User, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_UserEnvelope__g0__t179, ptr %t2, i32 0, i32 2
  call void @glitch_retain_Conduit_Features_Users_User__g0__t178(ptr null)
  %t4 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Features_Users_User__g0__t178(ptr %t4)
  store ptr null, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Features_Users_UserController__g0__t180_ctor(ptr %this, ptr %mediator, ptr %currentUserAccessor) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %mediator, ptr %t1
  %t2 = alloca ptr
  store ptr %currentUserAccessor, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_UserController__g0__t180, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t1
  store ptr %t5, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.Conduit_Features_Users_UserController__g0__t180, ptr %t6, i32 0, i32 3
  %t8 = load ptr, ptr %t2
  call void @glitch_retain_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t8)
  %t9 = load ptr, ptr %t7
  call void @glitch_drop_Conduit_Infrastructure_ICurrentUserAccessor__g0__t189(ptr %t9)
  store ptr %t8, ptr %t7
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Users_UserController__g0__t180_GetCurrent__g0(ptr %this, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %cancellationToken, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_UserController__g0__t180, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t3
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.Conduit_Features_Users_UserController__g0__t180, ptr %t5, i32 0, i32 3
  %t7 = load ptr, ptr %t6
  %t8 = call ptr @Conduit_Infrastructure_CurrentUserAccessor__g0__t183_GetCurrentUsername__g0(ptr %t7)
  %t9 = load ptr, ptr @glitch_exception_pending
  %t10 = icmp ne ptr %t9, null
  br i1 %t10, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t12 = icmp eq ptr %t8, null
  %t11 = alloca ptr
  br i1 %t12, label %coalesce_right_2, label %coalesce_left_1
coalesce_left_1:
  store ptr %t8, ptr %t11
  br label %coalesce_end_3
coalesce_right_2:
  store ptr getelementptr inbounds ({ i64, i64, [10 x i8] }, ptr @.str.44, i32 0, i32 2, i64 0), ptr %t11
  br label %coalesce_end_3
coalesce_end_3:
  %t13 = load ptr, ptr %t11
  %t14 = load ptr, ptr %t1
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Users_UserController__g0__t180_UpdateUser__g0(ptr %this, ptr %command, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %command, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_UserController__g0__t180, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Features_Users_UsersController__g0__t181_ctor(ptr %this, ptr %mediator) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %mediator, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Features_Users_UsersController__g0__t181, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Features_Users_UsersController__g0__t181_Create__g0(ptr %this, ptr %command, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %command, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_UsersController__g0__t181, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Features_Users_UsersController__g0__t181_Login__g0(ptr %this, ptr %command, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %command, ptr %t1
  %t2 = alloca ptr
  store ptr %cancellationToken, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Features_Users_UsersController__g0__t181, ptr %t3, i32 0, i32 2
  %t5 = load ptr, ptr %t4
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Infrastructure_ConduitContext__g0__t182_ctor(ptr %this, ptr %options) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %options, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t2, i32 0, i32 8
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_DbContextOptions__g1__t29(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_DbContextOptions__g1__t29(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Infrastructure_ConduitContext__g0__t182_OnModelCreating__g0(ptr %this, ptr %modelBuilder) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %modelBuilder, ptr %t1
  %t2 = load ptr, ptr %t1
  %t4 = getelementptr %glitch.delegate, ptr null, i32 1
  %t5 = ptrtoint ptr %t4 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t5)
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 1
  store ptr @glitch_lambda_44, ptr %t7
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 2
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 3
  store ptr null, ptr %t8
  store ptr null, ptr %t9
  call void @glitch_delegate_release(ptr %t3)
  %t10 = load ptr, ptr %t1
  %t12 = getelementptr %glitch.delegate, ptr null, i32 1
  %t13 = ptrtoint ptr %t12 to i64
  %t11 = call ptr @glitch_calloc(i64 1, i64 %t13)
  %t14 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 0
  store i64 1, ptr %t14
  %t15 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 1
  store ptr @glitch_lambda_45, ptr %t15
  %t16 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 2
  %t17 = getelementptr inbounds %glitch.delegate, ptr %t11, i32 0, i32 3
  store ptr null, ptr %t16
  store ptr null, ptr %t17
  call void @glitch_delegate_release(ptr %t11)
  %t18 = load ptr, ptr %t1
  %t20 = getelementptr %glitch.delegate, ptr null, i32 1
  %t21 = ptrtoint ptr %t20 to i64
  %t19 = call ptr @glitch_calloc(i64 1, i64 %t21)
  %t22 = getelementptr inbounds %glitch.delegate, ptr %t19, i32 0, i32 0
  store i64 1, ptr %t22
  %t23 = getelementptr inbounds %glitch.delegate, ptr %t19, i32 0, i32 1
  store ptr @glitch_lambda_46, ptr %t23
  %t24 = getelementptr inbounds %glitch.delegate, ptr %t19, i32 0, i32 2
  %t25 = getelementptr inbounds %glitch.delegate, ptr %t19, i32 0, i32 3
  store ptr null, ptr %t24
  store ptr null, ptr %t25
  call void @glitch_delegate_release(ptr %t19)
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Infrastructure_ConduitContext__g0__t182_BeginTransaction__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t1, i32 0, i32 9
  %t3 = load ptr, ptr %t2
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %if_then_0, label %if_else_1
if_then_0:
  ret void
if_else_1:
  br label %if_end_2
if_end_2:
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t5, i32 0, i32 6
  %t7 = load ptr, ptr %t6
  %t8 = icmp ne ptr null, null
  %t9 = xor i1 %t8, true
  br i1 %t9, label %if_then_3, label %if_else_4
if_then_3:
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t10, i32 0, i32 9
  %t12 = load ptr, ptr %t0
  %t13 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t12, i32 0, i32 6
  %t14 = load ptr, ptr %t13
  store ptr null, ptr %t11
  br label %if_end_5
if_else_4:
  br label %if_end_5
if_end_5:
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Infrastructure_ConduitContext__g0__t182_CommitTransaction__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t1, i32 0, i32 9
  %t3 = load ptr, ptr %t2
  %t4 = icmp eq ptr %t3, null
  %t5 = alloca ptr
  br i1 %t4, label %conditional_true_3, label %conditional_false_4
conditional_true_3:
  store ptr null, ptr %t5
  br label %conditional_end_5
conditional_false_4:
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t6, i32 0, i32 9
  %t8 = load ptr, ptr %t7
  store ptr null, ptr %t5
  br label %conditional_end_5
conditional_end_5:
  %t9 = load ptr, ptr %t5
  br label %try_finally_1
try_catch_0:
  %t10 = load ptr, ptr @glitch_exception_pending
  store ptr null, ptr @glitch_exception_pending
  store ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.45, i32 0, i32 2, i64 0), ptr @glitch_exception_pending
  br label %try_finally_1
try_finally_1:
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t11, i32 0, i32 9
  %t13 = load ptr, ptr %t12
  %t14 = icmp eq ptr %t13, null
  %t15 = alloca ptr
  br i1 %t14, label %conditional_true_6, label %conditional_false_7
conditional_true_6:
  store ptr null, ptr %t15
  br label %conditional_end_8
conditional_false_7:
  %t16 = load ptr, ptr %t0
  %t17 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t16, i32 0, i32 9
  %t18 = load ptr, ptr %t17
  store ptr null, ptr %t15
  br label %conditional_end_8
conditional_end_8:
  %t19 = load ptr, ptr %t15
  %t20 = load ptr, ptr %t0
  %t21 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t20, i32 0, i32 9
  store ptr null, ptr %t21
  %t22 = load ptr, ptr @glitch_exception_pending
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %exception_unwind, label %try_end_2
try_end_2:
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Infrastructure_ConduitContext__g0__t182_RollbackTransaction__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t1, i32 0, i32 9
  %t3 = load ptr, ptr %t2
  %t4 = icmp eq ptr %t3, null
  %t5 = alloca ptr
  br i1 %t4, label %conditional_true_3, label %conditional_false_4
conditional_true_3:
  store ptr null, ptr %t5
  br label %conditional_end_5
conditional_false_4:
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t6, i32 0, i32 9
  %t8 = load ptr, ptr %t7
  store ptr null, ptr %t5
  br label %conditional_end_5
conditional_end_5:
  %t9 = load ptr, ptr %t5
  br label %try_finally_1
try_finally_1:
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t10, i32 0, i32 9
  %t12 = load ptr, ptr %t11
  %t13 = icmp eq ptr %t12, null
  %t14 = alloca ptr
  br i1 %t13, label %conditional_true_6, label %conditional_false_7
conditional_true_6:
  store ptr null, ptr %t14
  br label %conditional_end_8
conditional_false_7:
  %t15 = load ptr, ptr %t0
  %t16 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t15, i32 0, i32 9
  %t17 = load ptr, ptr %t16
  store ptr null, ptr %t14
  br label %conditional_end_8
conditional_end_8:
  %t18 = load ptr, ptr %t14
  %t19 = load ptr, ptr %t0
  %t20 = getelementptr inbounds %glitch.Conduit_Infrastructure_ConduitContext__g0__t182, ptr %t19, i32 0, i32 9
  store ptr null, ptr %t20
  %t21 = load ptr, ptr @glitch_exception_pending
  %t22 = icmp ne ptr %t21, null
  br i1 %t22, label %exception_unwind, label %try_end_2
try_end_2:
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Infrastructure_CurrentUserAccessor__g0__t183_ctor(ptr %this, ptr %httpContextAccessor) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %httpContextAccessor, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Infrastructure_CurrentUserAccessor__g0__t183, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_IHttpContextAccessor__g0__t62(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_IHttpContextAccessor__g0__t62(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Infrastructure_CurrentUserAccessor__g0__t183_GetCurrentUsername__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_47, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  %t8 = call ptr @FirstOrDefault__object(ptr %t1)
  %t9 = load ptr, ptr @glitch_exception_pending
  %t10 = icmp ne ptr %t9, null
  br i1 %t10, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t11 = icmp eq ptr %t8, null
  %t12 = alloca ptr
  br i1 %t11, label %conditional_true_1, label %conditional_false_2
conditional_true_1:
  store ptr null, ptr %t12
  br label %conditional_end_3
conditional_false_2:
  store ptr null, ptr %t12
  br label %conditional_end_3
conditional_end_3:
  %t13 = load ptr, ptr %t12
  ret ptr %t13
exception_unwind:
  ret ptr null
}

define void @Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184_ctor(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  call void @glitch_retain_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t4)
  %t5 = load ptr, ptr %t3
  call void @glitch_drop_Conduit_Infrastructure_ConduitContext__g0__t182(ptr %t5)
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184_Handle__g0(ptr %this, ptr %request, ptr %next, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %request, ptr %t1
  %t2 = alloca ptr
  store ptr %next, ptr %t2
  %t3 = alloca ptr
  store ptr %cancellationToken, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  store ptr null, ptr %t4
  %t5 = load ptr, ptr %t0
  %t6 = getelementptr inbounds %glitch.Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184, ptr %t5, i32 0, i32 2
  %t7 = load ptr, ptr %t6
  call void @Conduit_Infrastructure_ConduitContext__g0__t182_BeginTransaction__g0(ptr %t7)
  %t8 = load ptr, ptr @glitch_exception_pending
  %t9 = icmp ne ptr %t8, null
  br i1 %t9, label %try_catch_0, label %call_continue_3
call_continue_3:
  %t10 = load ptr, ptr %t2
  %t11 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 1
  %t12 = load ptr, ptr %t11
  %t13 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 2
  %t14 = load ptr, ptr %t13
  %t15 = call ptr %t12(ptr %t14)
  %t16 = call ptr @glitch_task_get_result_ptr(ptr %t15)
  store ptr %t16, ptr %t4
  %t17 = load ptr, ptr %t0
  %t18 = getelementptr inbounds %glitch.Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184, ptr %t17, i32 0, i32 2
  %t19 = load ptr, ptr %t18
  call void @Conduit_Infrastructure_ConduitContext__g0__t182_CommitTransaction__g0(ptr %t19)
  %t20 = load ptr, ptr @glitch_exception_pending
  %t21 = icmp ne ptr %t20, null
  br i1 %t21, label %try_catch_0, label %call_continue_4
call_continue_4:
  br label %try_end_2
try_catch_0:
  %t22 = load ptr, ptr @glitch_exception_pending
  store ptr null, ptr @glitch_exception_pending
  %t23 = load ptr, ptr %t0
  %t24 = getelementptr inbounds %glitch.Conduit_Infrastructure_DBContextTransactionPipelineBehavior__g2__t184, ptr %t23, i32 0, i32 2
  %t25 = load ptr, ptr %t24
  call void @Conduit_Infrastructure_ConduitContext__g0__t182_RollbackTransaction__g0(ptr %t25)
  %t26 = load ptr, ptr @glitch_exception_pending
  %t27 = icmp ne ptr %t26, null
  br i1 %t27, label %exception_unwind, label %call_continue_5
call_continue_5:
  store ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.46, i32 0, i32 2, i64 0), ptr @glitch_exception_pending
  br label %exception_unwind
try_end_2:
  %t28 = load ptr, ptr %t4
  ret ptr %t28
exception_unwind:
  ret ptr null
}

define void @Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186_ctor(ptr %this, ptr %next, ptr %localizer, ptr %logger) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %next, ptr %t1
  %t2 = alloca ptr
  store ptr %localizer, ptr %t2
  %t3 = alloca ptr
  store ptr %logger, ptr %t3
  %t4 = load ptr, ptr %t0
  %t5 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %t4, i32 0, i32 2
  %t6 = load ptr, ptr %t1
  store ptr %t6, ptr %t5
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t2
  store ptr %t9, ptr %t8
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %t10, i32 0, i32 4
  %t12 = load ptr, ptr %t3
  call void @glitch_retain_ILogger__g0__t89(ptr %t12)
  %t13 = load ptr, ptr %t11
  call void @glitch_drop_ILogger__g0__t89(ptr %t13)
  store ptr %t12, ptr %t11
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186_Invoke__g0(ptr %this, ptr %context) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = load ptr, ptr %t1
  %t3 = call ptr @glitch_task_get_result_ptr(ptr null)
  br label %try_end_2
try_catch_0:
  %t4 = load ptr, ptr @glitch_exception_pending
  store ptr null, ptr @glitch_exception_pending
  %t5 = alloca ptr
  store ptr %t4, ptr %t5
  %t6 = load ptr, ptr %t1
  %t7 = load ptr, ptr %t5
  %t8 = load ptr, ptr %t0
  %t9 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %t8, i32 0, i32 4
  %t10 = load ptr, ptr %t9
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186, ptr %t11, i32 0, i32 3
  %t13 = load ptr, ptr %t12
  %t14 = call ptr @glitch_task_get_result_ptr(ptr null)
  br label %try_end_2
try_end_2:
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Infrastructure_Errors_ErrorHandlingMiddleware__g0__t186_HandleExceptionAsync__g0(ptr %this, ptr %context, ptr %exception, ptr %logger, ptr %localizer) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %context, ptr %t1
  %t2 = alloca ptr
  store ptr %exception, ptr %t2
  %t3 = alloca ptr
  store ptr %logger, ptr %t3
  %t4 = alloca ptr
  store ptr %localizer, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t6)
  store ptr null, ptr %t5
  %t7 = load ptr, ptr %t2
  br label %switch_compare_3
switch_compare_3:
  %t8 = load ptr, ptr %t2
  %t9 = alloca ptr
  store ptr %t8, ptr %t9
  %t10 = icmp ne ptr %t8, null
  br i1 %t10, label %switch_case_2, label %switch_default_1
switch_case_2:
  %t11 = load ptr, ptr %t1
  %t12 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t11, i32 0, i32 3
  %t13 = load ptr, ptr %t12
  %t14 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t13, i32 0, i32 2
  %t15 = load ptr, ptr %t9
  %t16 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t15, i32 0, i32 6
  %t17 = load ptr, ptr %t16
  store i32 0, ptr %t14
  %t18 = load ptr, ptr %t9
  %t19 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t18, i32 0, i32 5
  %t20 = load ptr, ptr %t19
  %t21 = call ptr @Serialize(ptr null)
  %t22 = load ptr, ptr @glitch_exception_pending
  %t23 = icmp ne ptr %t22, null
  br i1 %t23, label %exception_unwind, label %call_continue_4
call_continue_4:
  %t24 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t24)
  store ptr %t21, ptr %t5
  br label %switch_end_0
switch_default_1:
  %t25 = load ptr, ptr %t1
  %t26 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t25, i32 0, i32 3
  %t27 = load ptr, ptr %t26
  %t28 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t27, i32 0, i32 2
  store i32 0, ptr %t28
  %t29 = load ptr, ptr %t3
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [20 x i8] }, ptr @.str.47, i32 0, i32 2, i64 0))
  %t30 = load ptr, ptr %t2
  %t31 = call ptr @Serialize(ptr null)
  %t32 = load ptr, ptr @glitch_exception_pending
  %t33 = icmp ne ptr %t32, null
  br i1 %t33, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t34 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t34)
  store ptr %t31, ptr %t5
  br label %switch_end_0
switch_end_0:
  %t35 = load ptr, ptr %t1
  %t36 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t35, i32 0, i32 3
  %t37 = load ptr, ptr %t36
  %t38 = load ptr, ptr %t5
  %t39 = call ptr @glitch_task_get_result_ptr(ptr null)
  %t40 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t40)
  ret ptr null
exception_unwind:
  %t41 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t41)
  ret ptr null
}

define void @Conduit_Infrastructure_Errors_RestException__g0__t187_ctor(ptr %this, ptr %code, ptr %errors) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %code, ptr %t1
  %t2 = alloca ptr
  store ptr %errors, ptr %t2
  %t3 = load ptr, ptr %t0
  %t4 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t3, i32 0, i32 3
  %t5 = load ptr, ptr %t1
  store ptr %t5, ptr %t4
  %t6 = load ptr, ptr %t0
  %t7 = getelementptr inbounds %glitch.Conduit_Infrastructure_Errors_RestException__g0__t187, ptr %t6, i32 0, i32 4
  %t8 = load ptr, ptr %t2
  store ptr %t8, ptr %t7
  ret void
exception_unwind:
  ret void
}

define void @Conduit_Infrastructure_GroupByApiRootConvention__g0__t188_Apply__g0(ptr %this, ptr %controller) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %controller, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  store ptr null, ptr %t2
  %t4 = load ptr, ptr %t2
  %t5 = icmp eq ptr %t4, null
  %t6 = alloca ptr
  br i1 %t5, label %conditional_true_0, label %conditional_false_1
conditional_true_0:
  store ptr null, ptr %t6
  br label %conditional_end_2
conditional_false_1:
  store ptr null, ptr %t6
  br label %conditional_end_2
conditional_end_2:
  %t7 = load ptr, ptr %t6
  %t8 = icmp eq ptr %t7, null
  %t9 = alloca ptr
  br i1 %t8, label %conditional_true_3, label %conditional_false_4
conditional_true_3:
  store ptr null, ptr %t9
  br label %conditional_end_5
conditional_false_4:
  %t10 = load ptr, ptr %t2
  %t11 = icmp eq ptr %t10, null
  %t12 = alloca ptr
  br i1 %t11, label %conditional_true_6, label %conditional_false_7
conditional_true_6:
  store ptr null, ptr %t12
  br label %conditional_end_8
conditional_false_7:
  store ptr null, ptr %t12
  br label %conditional_end_8
conditional_end_8:
  %t13 = load ptr, ptr %t12
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.49, i32 0, i32 2, i64 0))
  store ptr null, ptr %t9
  br label %conditional_end_5
conditional_end_5:
  %t14 = load ptr, ptr %t9
  %t16 = icmp eq ptr null, null
  %t15 = alloca ptr
  br i1 %t16, label %coalesce_right_10, label %coalesce_left_9
coalesce_left_9:
  store ptr null, ptr %t15
  br label %coalesce_end_11
coalesce_right_10:
  store ptr getelementptr inbounds ({ i64, i64, [8 x i8] }, ptr @.str.50, i32 0, i32 2, i64 0), ptr %t15
  br label %coalesce_end_11
coalesce_end_11:
  %t17 = load ptr, ptr %t15
  store ptr %t17, ptr %t3
  %t18 = load ptr, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192_get_NotBefore__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr inbounds %glitch.DateTime__g0__t17, ptr null, i32 0, i32 3
  %t2 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192_get_IssuedAt__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = getelementptr inbounds %glitch.DateTime__g0__t17, ptr null, i32 0, i32 3
  %t2 = load ptr, ptr %t1
  call void @glitch_string_retain(ptr %t2)
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192_get_Expiration__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %t1, i32 0, i32 6
  %t3 = load ptr, ptr %t2
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192_get_JtiGenerator__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t2 = getelementptr %glitch.delegate, ptr null, i32 1
  %t3 = ptrtoint ptr %t2 to i64
  %t1 = call ptr @glitch_calloc(i64 1, i64 %t3)
  %t4 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 1
  store ptr @glitch_lambda_48, ptr %t5
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 2
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t1, i32 0, i32 3
  store ptr null, ptr %t6
  store ptr null, ptr %t7
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define void @Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193_ctor(ptr %this, ptr %jwtOptions) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %jwtOptions, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193_CreateToken__g0(ptr %this, ptr %username) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %username, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = call ptr @glitch_calloc(i64 1, i64 16)
  %t6 = call ptr @glitch_calloc(i64 3, i64 8)
  %t7 = getelementptr inbounds %glitch.array, ptr %t5, i32 0, i32 0
  store i64 3, ptr %t7
  %t8 = getelementptr inbounds %glitch.array, ptr %t5, i32 0, i32 1
  store ptr %t6, ptr %t8
  %t9 = load ptr, ptr %t1
  %t10 = getelementptr inbounds ptr, ptr %t6, i64 0
  store ptr null, ptr %t10
  %t11 = load ptr, ptr %t0
  %t12 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %t11, i32 0, i32 3
  %t13 = load ptr, ptr %t12
  %t14 = getelementptr inbounds ptr, ptr %t6, i64 1
  store ptr null, ptr %t14
  %t15 = load ptr, ptr %t0
  %t16 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %t15, i32 0, i32 3
  %t17 = load ptr, ptr %t16
  %t18 = call ptr @Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192_get_IssuedAt__g0(ptr %t17)
  %t19 = load ptr, ptr @glitch_exception_pending
  %t20 = icmp ne ptr %t19, null
  br i1 %t20, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t21 = getelementptr inbounds ptr, ptr %t6, i64 2
  store ptr null, ptr %t21
  %t22 = load ptr, ptr %t2
  %t23 = icmp eq ptr %t22, null
  br i1 %t23, label %array_release_done_2, label %array_release_1
array_release_1:
  %t24 = getelementptr inbounds %glitch.array, ptr %t22, i32 0, i32 0
  %t26 = getelementptr inbounds %glitch.array, ptr %t22, i32 0, i32 1
  %t25 = load i64, ptr %t24
  %t27 = load ptr, ptr %t26
  call void @glitch_free(ptr %t27)
  call void @glitch_free(ptr %t22)
  br label %array_release_done_2
array_release_done_2:
  store ptr %t5, ptr %t2
  %t28 = load ptr, ptr %t0
  %t29 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %t28, i32 0, i32 3
  %t30 = load ptr, ptr %t29
  %t31 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %t30, i32 0, i32 3
  %t32 = load ptr, ptr %t31
  %t33 = load ptr, ptr %t0
  %t34 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %t33, i32 0, i32 3
  %t35 = load ptr, ptr %t34
  %t36 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %t35, i32 0, i32 5
  %t37 = load ptr, ptr %t36
  %t38 = load ptr, ptr %t2
  %t39 = load ptr, ptr %t0
  %t40 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %t39, i32 0, i32 3
  %t41 = load ptr, ptr %t40
  %t42 = call ptr @Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192_get_NotBefore__g0(ptr %t41)
  %t43 = load ptr, ptr @glitch_exception_pending
  %t44 = icmp ne ptr %t43, null
  br i1 %t44, label %exception_unwind, label %call_continue_3
call_continue_3:
  %t45 = load ptr, ptr %t0
  %t46 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %t45, i32 0, i32 3
  %t47 = load ptr, ptr %t46
  %t48 = call ptr @Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192_get_Expiration__g0(ptr %t47)
  %t49 = load ptr, ptr @glitch_exception_pending
  %t50 = icmp ne ptr %t49, null
  br i1 %t50, label %exception_unwind, label %call_continue_4
call_continue_4:
  %t51 = load ptr, ptr %t0
  %t52 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtTokenGenerator__g0__t193, ptr %t51, i32 0, i32 3
  %t53 = load ptr, ptr %t52
  %t54 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_JwtIssuerOptions__g0__t192, ptr %t53, i32 0, i32 7
  %t55 = load ptr, ptr %t54
  store ptr null, ptr %t3
  %t56 = load ptr, ptr %t3
  store ptr null, ptr %t4
  %t57 = load ptr, ptr %t4
  %t58 = load ptr, ptr %t2
  %t59 = icmp eq ptr %t58, null
  br i1 %t59, label %array_release_done_6, label %array_release_5
array_release_5:
  %t60 = getelementptr inbounds %glitch.array, ptr %t58, i32 0, i32 0
  %t62 = getelementptr inbounds %glitch.array, ptr %t58, i32 0, i32 1
  %t61 = load i64, ptr %t60
  %t63 = load ptr, ptr %t62
  call void @glitch_free(ptr %t63)
  call void @glitch_free(ptr %t58)
  br label %array_release_done_6
array_release_done_6:
  ret ptr %t57
exception_unwind:
  %t64 = load ptr, ptr %t2
  %t65 = icmp eq ptr %t64, null
  br i1 %t65, label %array_release_done_8, label %array_release_7
array_release_7:
  %t66 = getelementptr inbounds %glitch.array, ptr %t64, i32 0, i32 0
  %t68 = getelementptr inbounds %glitch.array, ptr %t64, i32 0, i32 1
  %t67 = load i64, ptr %t66
  %t69 = load ptr, ptr %t68
  call void @glitch_free(ptr %t69)
  call void @glitch_free(ptr %t64)
  br label %array_release_done_8
array_release_done_8:
  ret ptr null
}

define ptr @Conduit_Infrastructure_Security_PasswordHasher__g0__t194_Hash__g0(ptr %this, ptr %password, ptr %salt) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %password, ptr %t1
  %t2 = alloca ptr
  store ptr %salt, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = load ptr, ptr %t1
  store ptr null, ptr %t3
  %t10 = load ptr, ptr %t2
  %t11 = getelementptr inbounds %glitch.array, ptr %t10, i32 0, i32 0
  %t12 = load i64, ptr %t11
  %t13 = trunc i64 %t12 to i32
  %t6 = call ptr @glitch_calloc(i64 1, i64 16)
  %t7 = call ptr @glitch_calloc(i64 0, i64 1)
  %t8 = getelementptr inbounds %glitch.array, ptr %t6, i32 0, i32 0
  store i64 0, ptr %t8
  %t9 = getelementptr inbounds %glitch.array, ptr %t6, i32 0, i32 1
  store ptr %t7, ptr %t9
  %t14 = load ptr, ptr %t4
  %t15 = icmp eq ptr %t14, null
  br i1 %t15, label %array_release_done_1, label %array_release_0
array_release_0:
  %t16 = getelementptr inbounds %glitch.array, ptr %t14, i32 0, i32 0
  %t18 = getelementptr inbounds %glitch.array, ptr %t14, i32 0, i32 1
  %t17 = load i64, ptr %t16
  %t19 = load ptr, ptr %t18
  call void @glitch_free(ptr %t19)
  call void @glitch_free(ptr %t14)
  br label %array_release_done_1
array_release_done_1:
  store ptr %t6, ptr %t4
  %t20 = load ptr, ptr %t3
  %t21 = load ptr, ptr %t4
  %t22 = load ptr, ptr %t2
  %t23 = load ptr, ptr %t4
  %t24 = load ptr, ptr %t2
  %t25 = getelementptr inbounds %glitch.array, ptr %t24, i32 0, i32 0
  %t26 = load i64, ptr %t25
  %t27 = trunc i64 %t26 to i32
  %t28 = load ptr, ptr %t0
  %t29 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_PasswordHasher__g0__t194, ptr %t28, i32 0, i32 2
  %t30 = load ptr, ptr %t29
  %t31 = load ptr, ptr %t4
  %t32 = load ptr, ptr %t4
  %t33 = icmp eq ptr %t32, null
  br i1 %t33, label %array_release_done_3, label %array_release_2
array_release_2:
  %t34 = getelementptr inbounds %glitch.array, ptr %t32, i32 0, i32 0
  %t36 = getelementptr inbounds %glitch.array, ptr %t32, i32 0, i32 1
  %t35 = load i64, ptr %t34
  %t37 = load ptr, ptr %t36
  call void @glitch_free(ptr %t37)
  call void @glitch_free(ptr %t32)
  br label %array_release_done_3
array_release_done_3:
  ret ptr null
exception_unwind:
  %t38 = load ptr, ptr %t4
  %t39 = icmp eq ptr %t38, null
  br i1 %t39, label %array_release_done_5, label %array_release_4
array_release_4:
  %t40 = getelementptr inbounds %glitch.array, ptr %t38, i32 0, i32 0
  %t42 = getelementptr inbounds %glitch.array, ptr %t38, i32 0, i32 1
  %t41 = load i64, ptr %t40
  %t43 = load ptr, ptr %t42
  call void @glitch_free(ptr %t43)
  call void @glitch_free(ptr %t38)
  br label %array_release_done_5
array_release_done_5:
  ret ptr null
}

define void @Conduit_Infrastructure_Security_PasswordHasher__g0__t194_Dispose__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = load ptr, ptr %t0
  %t2 = getelementptr inbounds %glitch.Conduit_Infrastructure_Security_PasswordHasher__g0__t194, ptr %t1, i32 0, i32 2
  %t3 = load ptr, ptr %t2
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Infrastructure_Slug__g0__t195_GenerateSlug__g0(ptr %this, ptr %phrase) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %phrase, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t1
  %t4 = icmp eq ptr %t3, null
  br i1 %t4, label %if_then_0, label %if_else_1
if_then_0:
  ret ptr null
if_else_1:
  br label %if_end_2
if_end_2:
  %t5 = load ptr, ptr %t1
  store ptr null, ptr %t2
  %t6 = load ptr, ptr %t2
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [1 x i8] }, ptr @.str.51, i32 0, i32 2, i64 0))
  store ptr null, ptr %t2
  %t7 = load ptr, ptr %t2
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.52, i32 0, i32 2, i64 0))
  store ptr null, ptr %t2
  %t8 = load ptr, ptr %t2
  %t9 = icmp sle ptr null, null
  %t10 = alloca ptr
  br i1 %t9, label %conditional_true_3, label %conditional_false_4
conditional_true_3:
  store ptr null, ptr %t10
  br label %conditional_end_5
conditional_false_4:
  store ptr null, ptr %t10
  br label %conditional_end_5
conditional_end_5:
  %t11 = load ptr, ptr %t10
  store ptr null, ptr %t2
  %t12 = load ptr, ptr %t2
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [2 x i8] }, ptr @.str.53, i32 0, i32 2, i64 0))
  store ptr null, ptr %t2
  %t13 = load ptr, ptr %t2
  ret ptr %t13
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Infrastructure_Slug__g0__t195_InvalidCharsRegex__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Infrastructure_Slug__g0__t195_MultipleSpacesRegex__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define ptr @Conduit_Infrastructure_Slug__g0__t195_TrimRegex__g0(ptr %this) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196_ctor(ptr %this, ptr %validators) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %validators, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = getelementptr inbounds %glitch.Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196, ptr %t2, i32 0, i32 2
  %t4 = load ptr, ptr %t1
  store ptr %t4, ptr %t3
  ret void
exception_unwind:
  ret void
}

define ptr @Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196_Handle__g0(ptr %this, ptr %request, ptr %next, ptr %cancellationToken) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %request, ptr %t1
  %t2 = alloca ptr
  store ptr %next, ptr %t2
  %t3 = alloca ptr
  store ptr %cancellationToken, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = load ptr, ptr %t1
  store ptr null, ptr %t4
  %t7 = load ptr, ptr %t0
  %t8 = getelementptr inbounds %glitch.Conduit_Infrastructure_ValidationPipelineBehavior__g2__t196, ptr %t7, i32 0, i32 3
  %t9 = load ptr, ptr %t8
  %t11 = getelementptr %glitch.delegate, ptr null, i32 1
  %t12 = ptrtoint ptr %t11 to i64
  %t10 = call ptr @glitch_calloc(i64 1, i64 %t12)
  %t13 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 0
  store i64 1, ptr %t13
  %t14 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 1
  store ptr @glitch_lambda_49, ptr %t14
  %t15 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 2
  %t16 = getelementptr inbounds %glitch.delegate, ptr %t10, i32 0, i32 3
  %t17 = getelementptr %glitch.lambda.49.env, ptr null, i32 1
  %t18 = ptrtoint ptr %t17 to i64
  %t19 = call ptr @glitch_calloc(i64 1, i64 %t18)
  %t20 = load ptr, ptr %t4
  %t21 = getelementptr inbounds %glitch.lambda.49.env, ptr %t19, i32 0, i32 0
  store ptr %t20, ptr %t21
  store ptr %t19, ptr %t15
  store ptr @glitch_lambda_49_destroy, ptr %t16
  call void @glitch_delegate_release(ptr %t10)
  %t25 = getelementptr %glitch.delegate, ptr null, i32 1
  %t26 = ptrtoint ptr %t25 to i64
  %t24 = call ptr @glitch_calloc(i64 1, i64 %t26)
  %t27 = getelementptr inbounds %glitch.delegate, ptr %t24, i32 0, i32 0
  store i64 1, ptr %t27
  %t28 = getelementptr inbounds %glitch.delegate, ptr %t24, i32 0, i32 1
  store ptr @glitch_lambda_50, ptr %t28
  %t29 = getelementptr inbounds %glitch.delegate, ptr %t24, i32 0, i32 2
  %t30 = getelementptr inbounds %glitch.delegate, ptr %t24, i32 0, i32 3
  store ptr null, ptr %t29
  store ptr null, ptr %t30
  call void @glitch_delegate_release(ptr %t24)
  %t32 = getelementptr %glitch.delegate, ptr null, i32 1
  %t33 = ptrtoint ptr %t32 to i64
  %t31 = call ptr @glitch_calloc(i64 1, i64 %t33)
  %t34 = getelementptr inbounds %glitch.delegate, ptr %t31, i32 0, i32 0
  store i64 1, ptr %t34
  %t35 = getelementptr inbounds %glitch.delegate, ptr %t31, i32 0, i32 1
  store ptr @glitch_lambda_51, ptr %t35
  %t36 = getelementptr inbounds %glitch.delegate, ptr %t31, i32 0, i32 2
  %t37 = getelementptr inbounds %glitch.delegate, ptr %t31, i32 0, i32 3
  store ptr null, ptr %t36
  store ptr null, ptr %t37
  call void @glitch_delegate_release(ptr %t31)
  store ptr null, ptr %t5
  %t38 = icmp ne ptr null, null
  br i1 %t38, label %if_then_0, label %if_else_1
if_then_0:
  %t39 = load ptr, ptr %t5
  store ptr null, ptr @glitch_exception_pending
  br label %exception_unwind
if_else_1:
  br label %if_end_2
if_end_2:
  %t40 = load ptr, ptr %t2
  %t41 = getelementptr inbounds %glitch.delegate, ptr %t40, i32 0, i32 1
  %t42 = load ptr, ptr %t41
  %t43 = getelementptr inbounds %glitch.delegate, ptr %t40, i32 0, i32 2
  %t44 = load ptr, ptr %t43
  %t45 = call ptr %t42(ptr %t44)
  %t46 = call ptr @glitch_task_get_result_ptr(ptr %t45)
  ret ptr %t46
exception_unwind:
  ret ptr null
}

define void @Conduit_Infrastructure_ValidatorActionFilter__g0__t197_OnActionExecuting__g0(ptr %this, ptr %filterContext) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %filterContext, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = load ptr, ptr %t1
  %t7 = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %t6, i32 0, i32 3
  %t8 = load ptr, ptr %t7
  %t9 = getelementptr inbounds %glitch.ModelStateDictionary__g0__t77, ptr %t8, i32 0, i32 2
  %t10 = load i1, ptr %t9
  %t11 = xor i1 %t10, true
  br i1 %t11, label %if_then_0, label %if_else_1
if_then_0:
  store ptr null, ptr %t2
  %t12 = call ptr @glitch_calloc(i64 1, i64 32)
  %t13 = call ptr @glitch_calloc(i64 4, i64 8)
  %t14 = call ptr @glitch_calloc(i64 4, i64 8)
  %t15 = getelementptr inbounds %glitch.dict, ptr %t12, i32 0, i32 1
  store i64 4, ptr %t15
  %t16 = getelementptr inbounds %glitch.dict, ptr %t12, i32 0, i32 2
  store ptr %t13, ptr %t16
  %t17 = getelementptr inbounds %glitch.dict, ptr %t12, i32 0, i32 3
  store ptr %t14, ptr %t17
  %t18 = load ptr, ptr %t3
  %t19 = icmp eq ptr %t18, null
  br i1 %t19, label %collection_release_done_4, label %collection_release_3
collection_release_3:
  %t20 = getelementptr inbounds %glitch.dict, ptr %t18, i32 0, i32 0
  %t21 = getelementptr inbounds %glitch.dict, ptr %t18, i32 0, i32 2
  %t22 = getelementptr inbounds %glitch.dict, ptr %t18, i32 0, i32 3
  %t23 = load i64, ptr %t20
  %t24 = load ptr, ptr %t21
  %t25 = load ptr, ptr %t22
  %t26 = alloca i64
  store i64 0, ptr %t26
  br label %element_drop_loop_5
element_drop_loop_5:
  %t27 = load i64, ptr %t26
  %t28 = icmp ult i64 %t27, %t23
  br i1 %t28, label %element_drop_body_6, label %element_drop_done_7
element_drop_body_6:
  %t29 = getelementptr inbounds ptr, ptr %t24, i64 %t27
  %t30 = load ptr, ptr %t29
  call void @glitch_string_release(ptr %t30)
  %t31 = add i64 %t27, 1
  store i64 %t31, ptr %t26
  br label %element_drop_loop_5
element_drop_done_7:
  %t32 = alloca i64
  store i64 0, ptr %t32
  br label %element_drop_loop_8
element_drop_loop_8:
  %t33 = load i64, ptr %t32
  %t34 = icmp ult i64 %t33, %t23
  br i1 %t34, label %element_drop_body_9, label %element_drop_done_10
element_drop_body_9:
  %t35 = getelementptr inbounds ptr, ptr %t25, i64 %t33
  %t36 = load ptr, ptr %t35
  %t38 = icmp eq ptr %t36, null
  br i1 %t38, label %array_release_done_12, label %array_release_11
array_release_11:
  %t39 = getelementptr inbounds %glitch.array, ptr %t36, i32 0, i32 0
  %t41 = getelementptr inbounds %glitch.array, ptr %t36, i32 0, i32 1
  %t40 = load i64, ptr %t39
  %t42 = load ptr, ptr %t41
  %t43 = alloca i64
  store i64 0, ptr %t43
  br label %element_drop_loop_13
element_drop_loop_13:
  %t44 = load i64, ptr %t43
  %t45 = icmp ult i64 %t44, %t40
  br i1 %t45, label %element_drop_body_14, label %element_drop_done_15
element_drop_body_14:
  %t46 = getelementptr inbounds ptr, ptr %t42, i64 %t44
  %t47 = load ptr, ptr %t46
  call void @glitch_string_release(ptr %t47)
  %t48 = add i64 %t44, 1
  store i64 %t48, ptr %t43
  br label %element_drop_loop_13
element_drop_done_15:
  call void @glitch_free(ptr %t42)
  call void @glitch_free(ptr %t36)
  br label %array_release_done_12
array_release_done_12:
  %t37 = add i64 %t33, 1
  store i64 %t37, ptr %t32
  br label %element_drop_loop_8
element_drop_done_10:
  call void @glitch_free(ptr %t24)
  call void @glitch_free(ptr %t25)
  call void @glitch_free(ptr %t18)
  br label %collection_release_done_4
collection_release_done_4:
  store ptr %t12, ptr %t3
  %t49 = load ptr, ptr %t1
  %t50 = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %t49, i32 0, i32 3
  %t51 = load ptr, ptr %t50
  %t52 = load ptr, ptr %t3
  %t53 = call ptr @Serialize(ptr null)
  %t54 = load ptr, ptr @glitch_exception_pending
  %t55 = icmp ne ptr %t54, null
  br i1 %t55, label %exception_unwind, label %call_continue_16
call_continue_16:
  store ptr %t53, ptr %t5
  %t56 = load ptr, ptr %t5
  %t57 = load ptr, ptr %t1
  %t58 = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %t57, i32 0, i32 4
  %t59 = load ptr, ptr %t58
  %t60 = getelementptr inbounds %glitch.HttpContext__g0__t61, ptr %t59, i32 0, i32 3
  %t61 = load ptr, ptr %t60
  %t62 = getelementptr inbounds %glitch.HttpResponse__g0__t58, ptr %t61, i32 0, i32 2
  %t63 = trunc i64 422 to i32
  store i32 %t63, ptr %t62
  %t64 = load ptr, ptr %t1
  %t65 = getelementptr inbounds %glitch.ActionExecutingContext__g0__t73, ptr %t64, i32 0, i32 5
  %t66 = load ptr, ptr %t2
  store ptr %t66, ptr %t65
  br label %if_end_2
if_else_1:
  br label %if_end_2
if_end_2:
  %t67 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t67)
  %t68 = load ptr, ptr %t3
  %t69 = icmp eq ptr %t68, null
  br i1 %t69, label %collection_release_done_18, label %collection_release_17
collection_release_17:
  %t70 = getelementptr inbounds %glitch.dict, ptr %t68, i32 0, i32 0
  %t71 = getelementptr inbounds %glitch.dict, ptr %t68, i32 0, i32 2
  %t72 = getelementptr inbounds %glitch.dict, ptr %t68, i32 0, i32 3
  %t73 = load i64, ptr %t70
  %t74 = load ptr, ptr %t71
  %t75 = load ptr, ptr %t72
  %t76 = alloca i64
  store i64 0, ptr %t76
  br label %element_drop_loop_19
element_drop_loop_19:
  %t77 = load i64, ptr %t76
  %t78 = icmp ult i64 %t77, %t73
  br i1 %t78, label %element_drop_body_20, label %element_drop_done_21
element_drop_body_20:
  %t79 = getelementptr inbounds ptr, ptr %t74, i64 %t77
  %t80 = load ptr, ptr %t79
  call void @glitch_string_release(ptr %t80)
  %t81 = add i64 %t77, 1
  store i64 %t81, ptr %t76
  br label %element_drop_loop_19
element_drop_done_21:
  %t82 = alloca i64
  store i64 0, ptr %t82
  br label %element_drop_loop_22
element_drop_loop_22:
  %t83 = load i64, ptr %t82
  %t84 = icmp ult i64 %t83, %t73
  br i1 %t84, label %element_drop_body_23, label %element_drop_done_24
element_drop_body_23:
  %t85 = getelementptr inbounds ptr, ptr %t75, i64 %t83
  %t86 = load ptr, ptr %t85
  %t88 = icmp eq ptr %t86, null
  br i1 %t88, label %array_release_done_26, label %array_release_25
array_release_25:
  %t89 = getelementptr inbounds %glitch.array, ptr %t86, i32 0, i32 0
  %t91 = getelementptr inbounds %glitch.array, ptr %t86, i32 0, i32 1
  %t90 = load i64, ptr %t89
  %t92 = load ptr, ptr %t91
  %t93 = alloca i64
  store i64 0, ptr %t93
  br label %element_drop_loop_27
element_drop_loop_27:
  %t94 = load i64, ptr %t93
  %t95 = icmp ult i64 %t94, %t90
  br i1 %t95, label %element_drop_body_28, label %element_drop_done_29
element_drop_body_28:
  %t96 = getelementptr inbounds ptr, ptr %t92, i64 %t94
  %t97 = load ptr, ptr %t96
  call void @glitch_string_release(ptr %t97)
  %t98 = add i64 %t94, 1
  store i64 %t98, ptr %t93
  br label %element_drop_loop_27
element_drop_done_29:
  call void @glitch_free(ptr %t92)
  call void @glitch_free(ptr %t86)
  br label %array_release_done_26
array_release_done_26:
  %t87 = add i64 %t83, 1
  store i64 %t87, ptr %t82
  br label %element_drop_loop_22
element_drop_done_24:
  call void @glitch_free(ptr %t74)
  call void @glitch_free(ptr %t75)
  call void @glitch_free(ptr %t68)
  br label %collection_release_done_18
collection_release_done_18:
  ret void
exception_unwind:
  %t99 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t99)
  %t100 = load ptr, ptr %t3
  %t101 = icmp eq ptr %t100, null
  br i1 %t101, label %collection_release_done_31, label %collection_release_30
collection_release_30:
  %t102 = getelementptr inbounds %glitch.dict, ptr %t100, i32 0, i32 0
  %t103 = getelementptr inbounds %glitch.dict, ptr %t100, i32 0, i32 2
  %t104 = getelementptr inbounds %glitch.dict, ptr %t100, i32 0, i32 3
  %t105 = load i64, ptr %t102
  %t106 = load ptr, ptr %t103
  %t107 = load ptr, ptr %t104
  %t108 = alloca i64
  store i64 0, ptr %t108
  br label %element_drop_loop_32
element_drop_loop_32:
  %t109 = load i64, ptr %t108
  %t110 = icmp ult i64 %t109, %t105
  br i1 %t110, label %element_drop_body_33, label %element_drop_done_34
element_drop_body_33:
  %t111 = getelementptr inbounds ptr, ptr %t106, i64 %t109
  %t112 = load ptr, ptr %t111
  call void @glitch_string_release(ptr %t112)
  %t113 = add i64 %t109, 1
  store i64 %t113, ptr %t108
  br label %element_drop_loop_32
element_drop_done_34:
  %t114 = alloca i64
  store i64 0, ptr %t114
  br label %element_drop_loop_35
element_drop_loop_35:
  %t115 = load i64, ptr %t114
  %t116 = icmp ult i64 %t115, %t105
  br i1 %t116, label %element_drop_body_36, label %element_drop_done_37
element_drop_body_36:
  %t117 = getelementptr inbounds ptr, ptr %t107, i64 %t115
  %t118 = load ptr, ptr %t117
  %t120 = icmp eq ptr %t118, null
  br i1 %t120, label %array_release_done_39, label %array_release_38
array_release_38:
  %t121 = getelementptr inbounds %glitch.array, ptr %t118, i32 0, i32 0
  %t123 = getelementptr inbounds %glitch.array, ptr %t118, i32 0, i32 1
  %t122 = load i64, ptr %t121
  %t124 = load ptr, ptr %t123
  %t125 = alloca i64
  store i64 0, ptr %t125
  br label %element_drop_loop_40
element_drop_loop_40:
  %t126 = load i64, ptr %t125
  %t127 = icmp ult i64 %t126, %t122
  br i1 %t127, label %element_drop_body_41, label %element_drop_done_42
element_drop_body_41:
  %t128 = getelementptr inbounds ptr, ptr %t124, i64 %t126
  %t129 = load ptr, ptr %t128
  call void @glitch_string_release(ptr %t129)
  %t130 = add i64 %t126, 1
  store i64 %t130, ptr %t125
  br label %element_drop_loop_40
element_drop_done_42:
  call void @glitch_free(ptr %t124)
  call void @glitch_free(ptr %t118)
  br label %array_release_done_39
array_release_done_39:
  %t119 = add i64 %t115, 1
  store i64 %t119, ptr %t114
  br label %element_drop_loop_35
element_drop_done_37:
  call void @glitch_free(ptr %t106)
  call void @glitch_free(ptr %t107)
  call void @glitch_free(ptr %t100)
  br label %collection_release_done_31
collection_release_done_31:
  ret void
}

define void @Conduit_Infrastructure_ValidatorActionFilter__g0__t197_OnActionExecuted__g0(ptr %this, ptr %filterContext) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %filterContext, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @Conduit_ServicesExtensions__g0__t198_AddConduit__g0(ptr %this, ptr %services) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %services, ptr %t1
  %t2 = load ptr, ptr %t1
  %t4 = getelementptr %glitch.delegate, ptr null, i32 1
  %t5 = ptrtoint ptr %t4 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t5)
  %t6 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t6
  %t7 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 1
  store ptr @glitch_lambda_52, ptr %t7
  %t8 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 2
  %t9 = getelementptr inbounds %glitch.delegate, ptr %t3, i32 0, i32 3
  store ptr null, ptr %t8
  store ptr null, ptr %t9
  call void @glitch_delegate_release(ptr %t3)
  %t10 = load ptr, ptr %t1
  %t11 = load ptr, ptr %t1
  call void @ServiceCollection__g0__t52_AddScoped__g0__object_object(ptr %t11, ptr null, ptr null)
  %t12 = load ptr, ptr @glitch_exception_pending
  %t13 = icmp ne ptr %t12, null
  br i1 %t13, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t14 = load ptr, ptr %t1
  %t15 = load ptr, ptr %t1
  %t17 = getelementptr %glitch.delegate, ptr null, i32 1
  %t18 = ptrtoint ptr %t17 to i64
  %t16 = call ptr @glitch_calloc(i64 1, i64 %t18)
  %t19 = getelementptr inbounds %glitch.delegate, ptr %t16, i32 0, i32 0
  store i64 1, ptr %t19
  %t20 = getelementptr inbounds %glitch.delegate, ptr %t16, i32 0, i32 1
  store ptr @glitch_lambda_53, ptr %t20
  %t21 = getelementptr inbounds %glitch.delegate, ptr %t16, i32 0, i32 2
  %t22 = getelementptr inbounds %glitch.delegate, ptr %t16, i32 0, i32 3
  store ptr null, ptr %t21
  store ptr null, ptr %t22
  call void @ServiceCollection__g0__t52_AddAutoMapper__g0(ptr %t15, ptr %t16)
  %t23 = load ptr, ptr @glitch_exception_pending
  %t24 = icmp ne ptr %t23, null
  br i1 %t24, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t25 = load ptr, ptr %t1
  call void @ServiceCollection__g0__t52_AddScoped__g2__overload(ptr %t25)
  %t26 = load ptr, ptr @glitch_exception_pending
  %t27 = icmp ne ptr %t26, null
  br i1 %t27, label %exception_unwind, label %call_continue_2
call_continue_2:
  %t28 = load ptr, ptr %t1
  call void @ServiceCollection__g0__t52_AddScoped__g2__overload(ptr %t28)
  %t29 = load ptr, ptr @glitch_exception_pending
  %t30 = icmp ne ptr %t29, null
  br i1 %t30, label %exception_unwind, label %call_continue_3
call_continue_3:
  %t31 = load ptr, ptr %t1
  call void @ServiceCollection__g0__t52_AddScoped__g2__overload(ptr %t31)
  %t32 = load ptr, ptr @glitch_exception_pending
  %t33 = icmp ne ptr %t32, null
  br i1 %t33, label %exception_unwind, label %call_continue_4
call_continue_4:
  %t34 = load ptr, ptr %t1
  call void @ServiceCollection__g0__t52_AddScoped__g2__overload(ptr %t34)
  %t35 = load ptr, ptr @glitch_exception_pending
  %t36 = icmp ne ptr %t35, null
  br i1 %t36, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t37 = load ptr, ptr %t1
  ret void
exception_unwind:
  ret void
}

define void @Conduit_ServicesExtensions__g0__t198_AddJwt__g0(ptr %this, ptr %services) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %services, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  %t3 = alloca ptr
  store ptr null, ptr %t3
  %t4 = alloca ptr
  store ptr null, ptr %t4
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = alloca ptr
  store ptr null, ptr %t6
  %t7 = load ptr, ptr %t1
  %t8 = call ptr @glitch_calloc(i64 1, i64 16)
  %t9 = call ptr @glitch_calloc(i64 45, i64 1)
  %t10 = getelementptr inbounds %glitch.array, ptr %t8, i32 0, i32 0
  store i64 45, ptr %t10
  %t11 = getelementptr inbounds %glitch.array, ptr %t8, i32 0, i32 1
  store ptr %t9, ptr %t11
  %t12 = trunc i64 115 to i8
  %t13 = getelementptr inbounds i8, ptr %t9, i64 0
  store i8 %t12, ptr %t13
  %t14 = trunc i64 111 to i8
  %t15 = getelementptr inbounds i8, ptr %t9, i64 1
  store i8 %t14, ptr %t15
  %t16 = trunc i64 109 to i8
  %t17 = getelementptr inbounds i8, ptr %t9, i64 2
  store i8 %t16, ptr %t17
  %t18 = trunc i64 101 to i8
  %t19 = getelementptr inbounds i8, ptr %t9, i64 3
  store i8 %t18, ptr %t19
  %t20 = trunc i64 116 to i8
  %t21 = getelementptr inbounds i8, ptr %t9, i64 4
  store i8 %t20, ptr %t21
  %t22 = trunc i64 104 to i8
  %t23 = getelementptr inbounds i8, ptr %t9, i64 5
  store i8 %t22, ptr %t23
  %t24 = trunc i64 105 to i8
  %t25 = getelementptr inbounds i8, ptr %t9, i64 6
  store i8 %t24, ptr %t25
  %t26 = trunc i64 110 to i8
  %t27 = getelementptr inbounds i8, ptr %t9, i64 7
  store i8 %t26, ptr %t27
  %t28 = trunc i64 103 to i8
  %t29 = getelementptr inbounds i8, ptr %t9, i64 8
  store i8 %t28, ptr %t29
  %t30 = trunc i64 108 to i8
  %t31 = getelementptr inbounds i8, ptr %t9, i64 9
  store i8 %t30, ptr %t31
  %t32 = trunc i64 111 to i8
  %t33 = getelementptr inbounds i8, ptr %t9, i64 10
  store i8 %t32, ptr %t33
  %t34 = trunc i64 110 to i8
  %t35 = getelementptr inbounds i8, ptr %t9, i64 11
  store i8 %t34, ptr %t35
  %t36 = trunc i64 103 to i8
  %t37 = getelementptr inbounds i8, ptr %t9, i64 12
  store i8 %t36, ptr %t37
  %t38 = trunc i64 101 to i8
  %t39 = getelementptr inbounds i8, ptr %t9, i64 13
  store i8 %t38, ptr %t39
  %t40 = trunc i64 114 to i8
  %t41 = getelementptr inbounds i8, ptr %t9, i64 14
  store i8 %t40, ptr %t41
  %t42 = trunc i64 102 to i8
  %t43 = getelementptr inbounds i8, ptr %t9, i64 15
  store i8 %t42, ptr %t43
  %t44 = trunc i64 111 to i8
  %t45 = getelementptr inbounds i8, ptr %t9, i64 16
  store i8 %t44, ptr %t45
  %t46 = trunc i64 114 to i8
  %t47 = getelementptr inbounds i8, ptr %t9, i64 17
  store i8 %t46, ptr %t47
  %t48 = trunc i64 116 to i8
  %t49 = getelementptr inbounds i8, ptr %t9, i64 18
  store i8 %t48, ptr %t49
  %t50 = trunc i64 104 to i8
  %t51 = getelementptr inbounds i8, ptr %t9, i64 19
  store i8 %t50, ptr %t51
  %t52 = trunc i64 105 to i8
  %t53 = getelementptr inbounds i8, ptr %t9, i64 20
  store i8 %t52, ptr %t53
  %t54 = trunc i64 115 to i8
  %t55 = getelementptr inbounds i8, ptr %t9, i64 21
  store i8 %t54, ptr %t55
  %t56 = trunc i64 100 to i8
  %t57 = getelementptr inbounds i8, ptr %t9, i64 22
  store i8 %t56, ptr %t57
  %t58 = trunc i64 117 to i8
  %t59 = getelementptr inbounds i8, ptr %t9, i64 23
  store i8 %t58, ptr %t59
  %t60 = trunc i64 109 to i8
  %t61 = getelementptr inbounds i8, ptr %t9, i64 24
  store i8 %t60, ptr %t61
  %t62 = trunc i64 98 to i8
  %t63 = getelementptr inbounds i8, ptr %t9, i64 25
  store i8 %t62, ptr %t63
  %t64 = trunc i64 97 to i8
  %t65 = getelementptr inbounds i8, ptr %t9, i64 26
  store i8 %t64, ptr %t65
  %t66 = trunc i64 108 to i8
  %t67 = getelementptr inbounds i8, ptr %t9, i64 27
  store i8 %t66, ptr %t67
  %t68 = trunc i64 103 to i8
  %t69 = getelementptr inbounds i8, ptr %t9, i64 28
  store i8 %t68, ptr %t69
  %t70 = trunc i64 111 to i8
  %t71 = getelementptr inbounds i8, ptr %t9, i64 29
  store i8 %t70, ptr %t71
  %t72 = trunc i64 114 to i8
  %t73 = getelementptr inbounds i8, ptr %t9, i64 30
  store i8 %t72, ptr %t73
  %t74 = trunc i64 105 to i8
  %t75 = getelementptr inbounds i8, ptr %t9, i64 31
  store i8 %t74, ptr %t75
  %t76 = trunc i64 116 to i8
  %t77 = getelementptr inbounds i8, ptr %t9, i64 32
  store i8 %t76, ptr %t77
  %t78 = trunc i64 104 to i8
  %t79 = getelementptr inbounds i8, ptr %t9, i64 33
  store i8 %t78, ptr %t79
  %t80 = trunc i64 109 to i8
  %t81 = getelementptr inbounds i8, ptr %t9, i64 34
  store i8 %t80, ptr %t81
  %t82 = trunc i64 105 to i8
  %t83 = getelementptr inbounds i8, ptr %t9, i64 35
  store i8 %t82, ptr %t83
  %t84 = trunc i64 115 to i8
  %t85 = getelementptr inbounds i8, ptr %t9, i64 36
  store i8 %t84, ptr %t85
  %t86 = trunc i64 114 to i8
  %t87 = getelementptr inbounds i8, ptr %t9, i64 37
  store i8 %t86, ptr %t87
  %t88 = trunc i64 101 to i8
  %t89 = getelementptr inbounds i8, ptr %t9, i64 38
  store i8 %t88, ptr %t89
  %t90 = trunc i64 113 to i8
  %t91 = getelementptr inbounds i8, ptr %t9, i64 39
  store i8 %t90, ptr %t91
  %t92 = trunc i64 117 to i8
  %t93 = getelementptr inbounds i8, ptr %t9, i64 40
  store i8 %t92, ptr %t93
  %t94 = trunc i64 105 to i8
  %t95 = getelementptr inbounds i8, ptr %t9, i64 41
  store i8 %t94, ptr %t95
  %t96 = trunc i64 114 to i8
  %t97 = getelementptr inbounds i8, ptr %t9, i64 42
  store i8 %t96, ptr %t97
  %t98 = trunc i64 101 to i8
  %t99 = getelementptr inbounds i8, ptr %t9, i64 43
  store i8 %t98, ptr %t99
  %t100 = trunc i64 100 to i8
  %t101 = getelementptr inbounds i8, ptr %t9, i64 44
  store i8 %t100, ptr %t101
  store ptr null, ptr %t2
  %t102 = load ptr, ptr %t2
  store ptr null, ptr %t3
  %t103 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t103)
  store ptr getelementptr inbounds ({ i64, i64, [7 x i8] }, ptr @.str.55, i32 0, i32 2, i64 0), ptr %t4
  %t104 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t104)
  store ptr getelementptr inbounds ({ i64, i64, [9 x i8] }, ptr @.str.56, i32 0, i32 2, i64 0), ptr %t5
  %t105 = load ptr, ptr %t1
  %t107 = getelementptr %glitch.delegate, ptr null, i32 1
  %t108 = ptrtoint ptr %t107 to i64
  %t106 = call ptr @glitch_calloc(i64 1, i64 %t108)
  %t109 = getelementptr inbounds %glitch.delegate, ptr %t106, i32 0, i32 0
  store i64 1, ptr %t109
  %t110 = getelementptr inbounds %glitch.delegate, ptr %t106, i32 0, i32 1
  store ptr @glitch_lambda_54, ptr %t110
  %t111 = getelementptr inbounds %glitch.delegate, ptr %t106, i32 0, i32 2
  %t112 = getelementptr inbounds %glitch.delegate, ptr %t106, i32 0, i32 3
  store ptr null, ptr %t111
  store ptr null, ptr %t112
  call void @ServiceCollection__g0__t52_Configure__g0(ptr %t105, ptr %t106)
  %t113 = load ptr, ptr @glitch_exception_pending
  %t114 = icmp ne ptr %t113, null
  br i1 %t114, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t115 = load ptr, ptr %t4
  %t116 = load ptr, ptr %t5
  store ptr null, ptr %t6
  %t117 = load ptr, ptr %t1
  %t118 = getelementptr inbounds %glitch.JwtBearerDefaults__g0__t71, ptr null, i32 0, i32 2
  %t119 = load ptr, ptr %t118
  %t121 = getelementptr %glitch.delegate, ptr null, i32 1
  %t122 = ptrtoint ptr %t121 to i64
  %t120 = call ptr @glitch_calloc(i64 1, i64 %t122)
  %t123 = getelementptr inbounds %glitch.delegate, ptr %t120, i32 0, i32 0
  store i64 1, ptr %t123
  %t124 = getelementptr inbounds %glitch.delegate, ptr %t120, i32 0, i32 1
  store ptr @glitch_lambda_55, ptr %t124
  %t125 = getelementptr inbounds %glitch.delegate, ptr %t120, i32 0, i32 2
  %t126 = getelementptr inbounds %glitch.delegate, ptr %t120, i32 0, i32 3
  store ptr null, ptr %t125
  store ptr null, ptr %t126
  call void @glitch_delegate_release(ptr %t120)
  %t127 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t127)
  %t128 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t128)
  ret void
exception_unwind:
  %t129 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t129)
  %t130 = load ptr, ptr %t4
  call void @glitch_string_release(ptr %t130)
  ret void
}

define void @Conduit_ServicesExtensions__g0__t198_AddSerilogLogging__g0(ptr %this, ptr %loggerFactory) {
entry:
  %t0 = alloca ptr
  store ptr %this, ptr %t0
  %t1 = alloca ptr
  store ptr %loggerFactory, ptr %t1
  %t2 = alloca ptr
  store ptr null, ptr %t2
  call void @glitch_string_release(ptr getelementptr inbounds ({ i64, i64, [77 x i8] }, ptr @.str.57, i32 0, i32 2, i64 0))
  store ptr null, ptr %t2
  %t3 = load ptr, ptr %t1
  %t4 = load ptr, ptr %t2
  %t5 = load ptr, ptr %t2
  ret void
exception_unwind:
  ret void
}

define ptr @SetString(ptr %context, ptr %table) {
entry:
  %t0 = alloca ptr
  store ptr %context, ptr %t0
  %t1 = alloca ptr
  store ptr %table, ptr %t1
  %t2 = load ptr, ptr %t0
  call void @DbContext__g0__t38_EnsureNotDisposed__g0(ptr %t2)
  %t3 = load ptr, ptr @glitch_exception_pending
  %t4 = icmp ne ptr %t3, null
  br i1 %t4, label %exception_unwind, label %call_continue_0
call_continue_0:
  %t5 = getelementptr %glitch.DbSetString__g0__t40, ptr null, i32 1
  %t6 = ptrtoint ptr %t5 to i64
  %t7 = call ptr @glitch_calloc(i64 1, i64 %t6)
  %t8 = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %t7, i32 0, i32 0
  store i64 1, ptr %t8
  %t9 = getelementptr inbounds %glitch.DbSetString__g0__t40, ptr %t7, i32 0, i32 1
  store ptr @glitch_destroy_DbSetString__g0__t40, ptr %t9
  %t10 = load ptr, ptr %t0
  %t11 = getelementptr inbounds %glitch.DbContext__g0__t38, ptr %t10, i32 0, i32 2
  %t12 = load ptr, ptr %t11
  %t13 = load ptr, ptr %t1
  call void @DbSetString__g0__t40_ctor(ptr %t7, ptr %t12, ptr %t13)
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
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @UseIdentityColumns(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
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
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5) {
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
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6) {
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
  ret ptr %t7
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7) {
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
  ret ptr %t8
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__object_object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7, ptr %arg8) {
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
  ret ptr %t9
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__object_object_object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7, ptr %arg8, ptr %arg9) {
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
  ret ptr %t10
exception_unwind:
  ret ptr null
}

define ptr @AlterColumn__object_object_object_object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7, ptr %arg8, ptr %arg9, ptr %arg10) {
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
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__object_object_object(ptr %target, ptr %arg1, ptr %arg2) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %arg1, ptr %t1
  %t2 = alloca ptr
  store ptr %arg2, ptr %t2
  %t3 = load ptr, ptr %t0
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3) {
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
  ret ptr %t4
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4) {
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
  ret ptr %t5
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5) {
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
  ret ptr %t6
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6) {
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
  ret ptr %t7
exception_unwind:
  ret ptr null
}

define ptr @AddColumn__object_object_object_object_object_object_object_object(ptr %target, ptr %arg1, ptr %arg2, ptr %arg3, ptr %arg4, ptr %arg5, ptr %arg6, ptr %arg7) {
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
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @HasConversion(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @IsUnique(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
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
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @AsQueryable(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @IgnoreQueryFilters(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
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
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @ToList(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
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
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @Any__object(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
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
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @ToArray(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
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
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @FirstOrDefault__object(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
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
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @FirstOrDefaultAsync__object(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
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
  ret ptr %t1
exception_unwind:
  ret ptr null
}

define ptr @AddRangeAsync(ptr %target, ptr %entities) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = alloca ptr
  store ptr %entities, ptr %t1
  %t2 = load ptr, ptr %t0
  ret ptr %t2
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
  ret ptr %t2
exception_unwind:
  ret ptr null
}

define ptr @AsNoTrackingWithIdentityResolution(ptr %target) {
entry:
  %t0 = alloca ptr
  store ptr %target, ptr %t0
  %t1 = load ptr, ptr %t0
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
  ret ptr %t3
exception_unwind:
  ret ptr null
}

define ptr @CreateBuilder(ptr %args) {
entry:
  %t0 = alloca ptr
  store ptr %args, ptr %t0
  %t1 = getelementptr %glitch.WebApplicationBuilder__g0__t56, ptr null, i32 1
  %t2 = ptrtoint ptr %t1 to i64
  %t3 = call ptr @glitch_calloc(i64 1, i64 %t2)
  %t4 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t3, i32 0, i32 0
  store i64 1, ptr %t4
  %t5 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t3, i32 0, i32 1
  store ptr @glitch_destroy_WebApplicationBuilder__g0__t56, ptr %t5
  call void @WebApplicationBuilder__g0__t56_ctor(ptr %t3)
  %t6 = load ptr, ptr @glitch_exception_pending
  %t7 = icmp ne ptr %t6, null
  br i1 %t7, label %exception_unwind, label %call_continue_0
call_continue_0:
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

define ptr @ReadAllText(ptr %path) {
entry:
  %t0 = alloca ptr
  store ptr %path, ptr %t0
  %t1 = load ptr, ptr %t0
  ret ptr null
exception_unwind:
  ret ptr null
}

define void @WriteAllText(ptr %path, ptr %contents) {
entry:
  %t0 = alloca ptr
  store ptr %path, ptr %t0
  %t1 = alloca ptr
  store ptr %contents, ptr %t1
  %t2 = load ptr, ptr %t0
  %t3 = load ptr, ptr %t1
  ret void
exception_unwind:
  ret void
}

define i32 @main(i32 %argc, ptr %argv) {
entry:
  %t0 = alloca ptr
  %t1 = alloca %glitch.array
  %t2 = zext i32 %argc to i64
  %t3 = getelementptr inbounds %glitch.array, ptr %t1, i32 0, i32 0
  store i64 %t2, ptr %t3
  %t4 = getelementptr inbounds %glitch.array, ptr %t1, i32 0, i32 1
  store ptr %argv, ptr %t4
  store ptr %t1, ptr %t0
  %t5 = alloca ptr
  store ptr null, ptr %t5
  %t6 = alloca ptr
  store ptr null, ptr %t6
  %t7 = alloca ptr
  store ptr null, ptr %t7
  %t8 = alloca ptr
  store ptr null, ptr %t8
  %t9 = alloca ptr
  store ptr null, ptr %t9
  %t10 = alloca ptr
  store ptr null, ptr %t10
  %t11 = alloca ptr
  store ptr null, ptr %t11
  %t12 = alloca ptr
  store ptr null, ptr %t12
  %t13 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t13)
  store ptr getelementptr inbounds ({ i64, i64, [22 x i8] }, ptr @.str.58, i32 0, i32 2, i64 0), ptr %t5
  %t14 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t14)
  store ptr getelementptr inbounds ({ i64, i64, [7 x i8] }, ptr @.str.59, i32 0, i32 2, i64 0), ptr %t6
  %t15 = load ptr, ptr %t0
  %t16 = call ptr @CreateBuilder(ptr %t15)
  %t17 = load ptr, ptr @glitch_exception_pending
  %t18 = icmp ne ptr %t17, null
  br i1 %t18, label %exception_unwind, label %call_continue_0
call_continue_0:
  store ptr %t16, ptr %t7
  %t19 = load ptr, ptr %t5
  call void @glitch_string_retain(ptr %t19)
  %t20 = load ptr, ptr %t8
  call void @glitch_string_release(ptr %t20)
  store ptr %t19, ptr %t8
  %t21 = load ptr, ptr %t6
  call void @glitch_string_retain(ptr %t21)
  %t22 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t22)
  store ptr %t21, ptr %t9
  %t23 = load ptr, ptr %t7
  %t24 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t23, i32 0, i32 2
  %t25 = load ptr, ptr %t24
  %t27 = getelementptr %glitch.delegate, ptr null, i32 1
  %t28 = ptrtoint ptr %t27 to i64
  %t26 = call ptr @glitch_calloc(i64 1, i64 %t28)
  %t29 = getelementptr inbounds %glitch.delegate, ptr %t26, i32 0, i32 0
  store i64 1, ptr %t29
  %t30 = getelementptr inbounds %glitch.delegate, ptr %t26, i32 0, i32 1
  store ptr @glitch_lambda_56, ptr %t30
  %t31 = getelementptr inbounds %glitch.delegate, ptr %t26, i32 0, i32 2
  %t32 = getelementptr inbounds %glitch.delegate, ptr %t26, i32 0, i32 3
  store ptr null, ptr %t31
  store ptr null, ptr %t32
  call void @glitch_delegate_release(ptr %t26)
  %t33 = load ptr, ptr %t7
  %t34 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t33, i32 0, i32 2
  %t35 = load ptr, ptr %t34
  %t37 = getelementptr %glitch.delegate, ptr null, i32 1
  %t38 = ptrtoint ptr %t37 to i64
  %t36 = call ptr @glitch_calloc(i64 1, i64 %t38)
  %t39 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 0
  store i64 1, ptr %t39
  %t40 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 1
  store ptr @glitch_lambda_57, ptr %t40
  %t41 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 2
  %t42 = getelementptr inbounds %glitch.delegate, ptr %t36, i32 0, i32 3
  store ptr null, ptr %t41
  store ptr null, ptr %t42
  call void @glitch_delegate_release(ptr %t36)
  %t43 = load ptr, ptr %t7
  %t44 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t43, i32 0, i32 2
  %t45 = load ptr, ptr %t44
  %t47 = getelementptr %glitch.delegate, ptr null, i32 1
  %t48 = ptrtoint ptr %t47 to i64
  %t46 = call ptr @glitch_calloc(i64 1, i64 %t48)
  %t49 = getelementptr inbounds %glitch.delegate, ptr %t46, i32 0, i32 0
  store i64 1, ptr %t49
  %t50 = getelementptr inbounds %glitch.delegate, ptr %t46, i32 0, i32 1
  store ptr @glitch_lambda_58, ptr %t50
  %t51 = getelementptr inbounds %glitch.delegate, ptr %t46, i32 0, i32 2
  %t52 = getelementptr inbounds %glitch.delegate, ptr %t46, i32 0, i32 3
  store ptr null, ptr %t51
  store ptr null, ptr %t52
  call void @ServiceCollection__g0__t52_AddSwaggerGen__g0(ptr %t45, ptr %t46)
  %t53 = load ptr, ptr @glitch_exception_pending
  %t54 = icmp ne ptr %t53, null
  br i1 %t54, label %exception_unwind, label %call_continue_1
call_continue_1:
  %t55 = load ptr, ptr %t7
  %t56 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t55, i32 0, i32 2
  %t57 = load ptr, ptr %t56
  %t58 = load ptr, ptr %t7
  %t59 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t58, i32 0, i32 2
  %t60 = load ptr, ptr %t59
  %t62 = getelementptr %glitch.delegate, ptr null, i32 1
  %t63 = ptrtoint ptr %t62 to i64
  %t61 = call ptr @glitch_calloc(i64 1, i64 %t63)
  %t64 = getelementptr inbounds %glitch.delegate, ptr %t61, i32 0, i32 0
  store i64 1, ptr %t64
  %t65 = getelementptr inbounds %glitch.delegate, ptr %t61, i32 0, i32 1
  store ptr @glitch_lambda_59, ptr %t65
  %t66 = getelementptr inbounds %glitch.delegate, ptr %t61, i32 0, i32 2
  %t67 = getelementptr inbounds %glitch.delegate, ptr %t61, i32 0, i32 3
  store ptr null, ptr %t66
  store ptr null, ptr %t67
  call void @glitch_delegate_release(ptr %t61)
  %t69 = getelementptr %glitch.delegate, ptr null, i32 1
  %t70 = ptrtoint ptr %t69 to i64
  %t68 = call ptr @glitch_calloc(i64 1, i64 %t70)
  %t71 = getelementptr inbounds %glitch.delegate, ptr %t68, i32 0, i32 0
  store i64 1, ptr %t71
  %t72 = getelementptr inbounds %glitch.delegate, ptr %t68, i32 0, i32 1
  store ptr @glitch_lambda_60, ptr %t72
  %t73 = getelementptr inbounds %glitch.delegate, ptr %t68, i32 0, i32 2
  %t74 = getelementptr inbounds %glitch.delegate, ptr %t68, i32 0, i32 3
  store ptr null, ptr %t73
  store ptr null, ptr %t74
  call void @glitch_delegate_release(ptr %t68)
  %t75 = load ptr, ptr %t7
  %t76 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t75, i32 0, i32 2
  %t77 = load ptr, ptr %t76
  %t78 = load ptr, ptr %t7
  %t79 = getelementptr inbounds %glitch.WebApplicationBuilder__g0__t56, ptr %t78, i32 0, i32 2
  %t80 = load ptr, ptr %t79
  %t81 = load ptr, ptr %t7
  %t82 = call ptr @WebApplicationBuilder__g0__t56_Build__g0(ptr %t81)
  %t83 = load ptr, ptr @glitch_exception_pending
  %t84 = icmp ne ptr %t83, null
  br i1 %t84, label %exception_unwind, label %call_continue_2
call_continue_2:
  store ptr %t82, ptr %t10
  %t85 = load ptr, ptr %t10
  %t86 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t85, i32 0, i32 6
  %t87 = load ptr, ptr %t86
  %t88 = call ptr @ServiceProvider__g0__t47_GetRequiredService__g1__overload(ptr %t87)
  %t89 = load ptr, ptr @glitch_exception_pending
  %t90 = icmp ne ptr %t89, null
  br i1 %t90, label %exception_unwind, label %call_continue_3
call_continue_3:
  %t91 = load ptr, ptr %t10
  call void @WebApplication__g0__t65_UseMiddleware__g1__overload(ptr %t91)
  %t92 = load ptr, ptr @glitch_exception_pending
  %t93 = icmp ne ptr %t92, null
  br i1 %t93, label %exception_unwind, label %call_continue_4
call_continue_4:
  %t94 = load ptr, ptr %t10
  %t96 = getelementptr %glitch.delegate, ptr null, i32 1
  %t97 = ptrtoint ptr %t96 to i64
  %t95 = call ptr @glitch_calloc(i64 1, i64 %t97)
  %t98 = getelementptr inbounds %glitch.delegate, ptr %t95, i32 0, i32 0
  store i64 1, ptr %t98
  %t99 = getelementptr inbounds %glitch.delegate, ptr %t95, i32 0, i32 1
  store ptr @glitch_lambda_61, ptr %t99
  %t100 = getelementptr inbounds %glitch.delegate, ptr %t95, i32 0, i32 2
  %t101 = getelementptr inbounds %glitch.delegate, ptr %t95, i32 0, i32 3
  store ptr null, ptr %t100
  store ptr null, ptr %t101
  call void @glitch_delegate_release(ptr %t95)
  %t102 = load ptr, ptr %t10
  call void @WebApplication__g0__t65_UseAuthentication__g0(ptr %t102)
  %t103 = load ptr, ptr @glitch_exception_pending
  %t104 = icmp ne ptr %t103, null
  br i1 %t104, label %exception_unwind, label %call_continue_5
call_continue_5:
  %t105 = load ptr, ptr %t10
  %t106 = load ptr, ptr %t10
  %t108 = getelementptr %glitch.delegate, ptr null, i32 1
  %t109 = ptrtoint ptr %t108 to i64
  %t107 = call ptr @glitch_calloc(i64 1, i64 %t109)
  %t110 = getelementptr inbounds %glitch.delegate, ptr %t107, i32 0, i32 0
  store i64 1, ptr %t110
  %t111 = getelementptr inbounds %glitch.delegate, ptr %t107, i32 0, i32 1
  store ptr @glitch_lambda_62, ptr %t111
  %t112 = getelementptr inbounds %glitch.delegate, ptr %t107, i32 0, i32 2
  %t113 = getelementptr inbounds %glitch.delegate, ptr %t107, i32 0, i32 3
  store ptr null, ptr %t112
  store ptr null, ptr %t113
  call void @glitch_delegate_release(ptr %t107)
  %t114 = load ptr, ptr %t10
  %t116 = getelementptr %glitch.delegate, ptr null, i32 1
  %t117 = ptrtoint ptr %t116 to i64
  %t115 = call ptr @glitch_calloc(i64 1, i64 %t117)
  %t118 = getelementptr inbounds %glitch.delegate, ptr %t115, i32 0, i32 0
  store i64 1, ptr %t118
  %t119 = getelementptr inbounds %glitch.delegate, ptr %t115, i32 0, i32 1
  store ptr @glitch_lambda_63, ptr %t119
  %t120 = getelementptr inbounds %glitch.delegate, ptr %t115, i32 0, i32 2
  %t121 = getelementptr inbounds %glitch.delegate, ptr %t115, i32 0, i32 3
  store ptr null, ptr %t120
  store ptr null, ptr %t121
  call void @WebApplication__g0__t65_UseSwaggerUI__g0__object(ptr %t114, ptr %t115)
  %t122 = load ptr, ptr @glitch_exception_pending
  %t123 = icmp ne ptr %t122, null
  br i1 %t123, label %exception_unwind, label %call_continue_6
call_continue_6:
  %t124 = load ptr, ptr %t10
  %t125 = getelementptr inbounds %glitch.WebApplication__g0__t65, ptr %t124, i32 0, i32 6
  %t126 = load ptr, ptr %t125
  %t127 = call ptr @ServiceProvider__g0__t47_CreateScope__g0(ptr %t126)
  %t128 = load ptr, ptr @glitch_exception_pending
  %t129 = icmp ne ptr %t128, null
  br i1 %t129, label %exception_unwind, label %call_continue_7
call_continue_7:
  store ptr %t127, ptr %t11
  store ptr null, ptr %t12
  %t130 = load ptr, ptr %t10
  call void @WebApplication__g0__t65_Run__g0__overload(ptr %t130)
  %t131 = load ptr, ptr @glitch_exception_pending
  %t132 = icmp ne ptr %t131, null
  br i1 %t132, label %exception_unwind, label %call_continue_8
call_continue_8:
  %t133 = load ptr, ptr %t11
  call void @glitch_drop_IServiceScope__g0__t48(ptr %t133)
  %t134 = load ptr, ptr %t10
  call void @glitch_drop_WebApplication__g0__t65(ptr %t134)
  %t135 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t135)
  %t136 = load ptr, ptr %t8
  call void @glitch_string_release(ptr %t136)
  %t137 = load ptr, ptr %t7
  call void @glitch_drop_WebApplicationBuilder__g0__t56(ptr %t137)
  %t138 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t138)
  %t139 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t139)
  %t140 = load i64, ptr @glitch_live_allocations
  %t141 = icmp ne i64 %t140, 0
  %t142 = load ptr, ptr @glitch_exception_pending
  %t143 = icmp ne ptr %t142, null
  %t144 = or i1 %t141, %t143
  %t145 = zext i1 %t144 to i32
  %t146 = call ptr @getenv(ptr @.env_report_leaks)
  %t147 = icmp ne ptr %t146, null
  br i1 %t147, label %report_leaks_9, label %main_return_10
report_leaks_9:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t140)
  br label %main_return_10
main_return_10:
  ret i32 %t145
exception_unwind:
  %t148 = load ptr, ptr %t11
  call void @glitch_drop_IServiceScope__g0__t48(ptr %t148)
  %t149 = load ptr, ptr %t10
  call void @glitch_drop_WebApplication__g0__t65(ptr %t149)
  %t150 = load ptr, ptr %t9
  call void @glitch_string_release(ptr %t150)
  %t151 = load ptr, ptr %t8
  call void @glitch_string_release(ptr %t151)
  %t152 = load ptr, ptr %t7
  call void @glitch_drop_WebApplicationBuilder__g0__t56(ptr %t152)
  %t153 = load ptr, ptr %t6
  call void @glitch_string_release(ptr %t153)
  %t154 = load ptr, ptr %t5
  call void @glitch_string_release(ptr %t154)
  %t155 = load i64, ptr @glitch_live_allocations
  %t156 = icmp ne i64 %t155, 0
  %t157 = load ptr, ptr @glitch_exception_pending
  %t158 = icmp ne ptr %t157, null
  %t159 = or i1 %t156, %t158
  %t160 = zext i1 %t159 to i32
  %t161 = call ptr @getenv(ptr @.env_report_leaks)
  %t162 = icmp ne ptr %t161, null
  br i1 %t162, label %report_leaks_11, label %main_return_12
report_leaks_11:
  call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([6 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %t155)
  br label %main_return_12
main_return_12:
  ret i32 %t160
}

define i1 @glitch_endpoint_handlers_contains(ptr %app, ptr %method, ptr %path) {
entry:
  ret i1 false
}

define ptr @glitch_endpoint_handlers_invoke(ptr %app, ptr %method, ptr %path, ptr %body) {
entry:
  ret ptr getelementptr inbounds ({ i64, i64, [4 x i8] }, ptr @.str.64, i32 0, i32 2, i64 0)
}

