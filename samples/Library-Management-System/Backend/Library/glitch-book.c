#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>


struct GlitchArray_bool { int *data; int len; };
struct GlitchArray_byte { unsigned char *data; int len; };
struct GlitchArray_short { short *data; int len; };
struct GlitchArray_int { int *data; int len; };
struct GlitchArray_i64 { long long *data; int len; };
struct GlitchArray_uint { unsigned int *data; int len; };
struct GlitchArray_f64 { double *data; int len; };
struct GlitchArray_decimal { long double *data; int len; };

struct List_int { int *data; int len; int cap; };
static char * glitch_strdup(const char *source);
static struct List_int List_int_new(void) { struct List_int list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(int) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_int_add(struct List_int *list, int value) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(int) * (size_t)list->cap); if (!list->data) { abort(); } } list->data[list->len++] = value; }
static int List_int_get(struct List_int *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_int_contains(struct List_int *list, int value) { for (int i = 0; i < list->len; i++) { if (list->data[i] == value) { return 1; } } return 0; }
static void List_int_clear(struct List_int *list) { list->len = 0; }
static void List_int_free(struct List_int *list) { free(list->data); list->data = NULL; list->len = 0; list->cap = 0; }

struct List_i64 { long long *data; int len; int cap; };
static struct List_i64 List_i64_new(void) { struct List_i64 list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(long long) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_i64_add(struct List_i64 *list, long long value) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(long long) * (size_t)list->cap); if (!list->data) { abort(); } } list->data[list->len++] = value; }
static long long List_i64_get(struct List_i64 *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_i64_contains(struct List_i64 *list, long long value) { for (int i = 0; i < list->len; i++) { if (list->data[i] == value) { return 1; } } return 0; }
static void List_i64_clear(struct List_i64 *list) { list->len = 0; }
static void List_i64_free(struct List_i64 *list) { free(list->data); list->data = NULL; list->len = 0; list->cap = 0; }

struct List_bool { int *data; int len; int cap; };
static struct List_bool List_bool_new(void) { struct List_bool list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(int) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_bool_add(struct List_bool *list, int value) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(int) * (size_t)list->cap); if (!list->data) { abort(); } } list->data[list->len++] = value; }
static int List_bool_get(struct List_bool *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_bool_contains(struct List_bool *list, int value) { for (int i = 0; i < list->len; i++) { if (list->data[i] == value) { return 1; } } return 0; }
static void List_bool_clear(struct List_bool *list) { list->len = 0; }
static void List_bool_free(struct List_bool *list) { free(list->data); list->data = NULL; list->len = 0; list->cap = 0; }

struct List_f64 { double *data; int len; int cap; };
static struct List_f64 List_f64_new(void) { struct List_f64 list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(double) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_f64_add(struct List_f64 *list, double value) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(double) * (size_t)list->cap); if (!list->data) { abort(); } } list->data[list->len++] = value; }
static double List_f64_get(struct List_f64 *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_f64_contains(struct List_f64 *list, double value) { for (int i = 0; i < list->len; i++) { if (list->data[i] == value) { return 1; } } return 0; }
static void List_f64_clear(struct List_f64 *list) { list->len = 0; }
static void List_f64_free(struct List_f64 *list) { free(list->data); list->data = NULL; list->len = 0; list->cap = 0; }

struct List_string { char **data; int len; int cap; };
static struct List_string List_string_new(void) { struct List_string list; list.len = 0; list.cap = 4; list.data = malloc(sizeof(char *) * (size_t)list.cap); if (!list.data) { abort(); } return list; }
static void List_string_reserve_one(struct List_string *list) { if (list->len >= list->cap) { list->cap *= 2; list->data = realloc(list->data, sizeof(char *) * (size_t)list->cap); if (!list->data) { abort(); } } }
static void List_string_add_owned(struct List_string *list, char *value) { List_string_reserve_one(list); list->data[list->len++] = value ? value : glitch_strdup(""); }
static void List_string_add(struct List_string *list, const char *value) { List_string_add_owned(list, glitch_strdup(value ? value : "")); }
static char * List_string_get(struct List_string *list, int index) { if (index < 0 || index >= list->len) { abort(); } return list->data[index]; }
static int List_string_contains(struct List_string *list, const char *value) { for (int i = 0; i < list->len; i++) { if (strcmp(list->data[i], value) == 0) { return 1; } } return 0; }
static void List_string_clear(struct List_string *list) { for (int i = 0; i < list->len; i++) { free(list->data[i]); } list->len = 0; }
static void List_string_free(struct List_string *list) { List_string_clear(list); free(list->data); list->data = NULL; list->cap = 0; }

struct IEnumerable_int { struct List_int *source; };
struct IEnumerable_i64 { struct List_i64 *source; };
struct IEnumerable_bool { struct List_bool *source; };
struct IEnumerable_f64 { struct List_f64 *source; };
struct IEnumerable_string { struct List_string *source; };
static struct IEnumerable_int IEnumerable_int_from_List_int(struct List_int *source) { struct IEnumerable_int enumerable; enumerable.source = source; return enumerable; }
static struct IEnumerable_i64 IEnumerable_i64_from_List_i64(struct List_i64 *source) { struct IEnumerable_i64 enumerable; enumerable.source = source; return enumerable; }
static struct IEnumerable_bool IEnumerable_bool_from_List_bool(struct List_bool *source) { struct IEnumerable_bool enumerable; enumerable.source = source; return enumerable; }
static struct IEnumerable_f64 IEnumerable_f64_from_List_f64(struct List_f64 *source) { struct IEnumerable_f64 enumerable; enumerable.source = source; return enumerable; }
static struct IEnumerable_string IEnumerable_string_from_List_string(struct List_string *source) { struct IEnumerable_string enumerable; enumerable.source = source; return enumerable; }

struct Dict_string_int_entry { char *key; int value; };
struct Dict_string_int { struct Dict_string_int_entry *entries; int len; int cap; };
static char * glitch_strdup(const char *source) { size_t len = strlen(source) + 1; char *copy = malloc(len); if (!copy) { abort(); } memcpy(copy, source, len); return copy; }
static char * glitch_string_concat(const char *left, const char *right) { left = left ? left : ""; right = right ? right : ""; size_t left_len = strlen(left); size_t right_len = strlen(right); char *copy = malloc(left_len + right_len + 1); if (!copy) { abort(); } memcpy(copy, left, left_len); memcpy(copy + left_len, right, right_len + 1); return copy; }
struct GlitchException { char *message; };
struct GlitchExceptionFrame { jmp_buf env; struct GlitchExceptionFrame *prev; struct GlitchException exception; };
static struct GlitchExceptionFrame *glitch_exception_stack = NULL;
static struct GlitchException glitch_exception_from_owned(char *message) { struct GlitchException ex; ex.message = message ? message : glitch_strdup(""); return ex; }
static struct GlitchException glitch_exception_new(const char *message) { return glitch_exception_from_owned(glitch_strdup(message ? message : "")); }
static struct GlitchException glitch_exception_clone(struct GlitchException *source) { return glitch_exception_new(source && source->message ? source->message : ""); }
static void glitch_exception_free(struct GlitchException *exception) { if (!exception) { return; } free(exception->message); exception->message = NULL; }
static void glitch_exception_push(struct GlitchExceptionFrame *frame) { frame->prev = glitch_exception_stack; frame->exception.message = NULL; glitch_exception_stack = frame; }
static void glitch_exception_pop(struct GlitchExceptionFrame *frame) { if (glitch_exception_stack == frame) { glitch_exception_stack = frame->prev; } }
static void glitch_throw(struct GlitchException exception) { if (!glitch_exception_stack) { fprintf(stderr, "Unhandled exception: %s\n", exception.message ? exception.message : ""); glitch_exception_free(&exception); abort(); } glitch_exception_stack->exception = exception; longjmp(glitch_exception_stack->env, 1); }
static struct Dict_string_int Dict_string_int_new(void) { struct Dict_string_int dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_int_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_int_add(struct Dict_string_int *dict, const char *key, int value) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_int_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } dict->entries[dict->len].key = glitch_strdup(key); dict->entries[dict->len].value = value; dict->len++; }
static int Dict_string_int_contains_key(struct Dict_string_int *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static int Dict_string_int_get(struct Dict_string_int *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_int_remove(struct Dict_string_int *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_int_clear(struct Dict_string_int *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); } dict->len = 0; }
static void Dict_string_int_free(struct Dict_string_int *dict) { Dict_string_int_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

struct Dict_string_i64_entry { char *key; long long value; };
struct Dict_string_i64 { struct Dict_string_i64_entry *entries; int len; int cap; };
static struct Dict_string_i64 Dict_string_i64_new(void) { struct Dict_string_i64 dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_i64_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_i64_add(struct Dict_string_i64 *dict, const char *key, long long value) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_i64_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } dict->entries[dict->len].key = glitch_strdup(key); dict->entries[dict->len].value = value; dict->len++; }
static int Dict_string_i64_contains_key(struct Dict_string_i64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static long long Dict_string_i64_get(struct Dict_string_i64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_i64_remove(struct Dict_string_i64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_i64_clear(struct Dict_string_i64 *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); } dict->len = 0; }
static void Dict_string_i64_free(struct Dict_string_i64 *dict) { Dict_string_i64_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

struct Dict_string_bool_entry { char *key; int value; };
struct Dict_string_bool { struct Dict_string_bool_entry *entries; int len; int cap; };
static struct Dict_string_bool Dict_string_bool_new(void) { struct Dict_string_bool dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_bool_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_bool_add(struct Dict_string_bool *dict, const char *key, int value) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_bool_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } dict->entries[dict->len].key = glitch_strdup(key); dict->entries[dict->len].value = value; dict->len++; }
static int Dict_string_bool_contains_key(struct Dict_string_bool *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static int Dict_string_bool_get(struct Dict_string_bool *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_bool_remove(struct Dict_string_bool *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_bool_clear(struct Dict_string_bool *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); } dict->len = 0; }
static void Dict_string_bool_free(struct Dict_string_bool *dict) { Dict_string_bool_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

struct Dict_string_f64_entry { char *key; double value; };
struct Dict_string_f64 { struct Dict_string_f64_entry *entries; int len; int cap; };
static struct Dict_string_f64 Dict_string_f64_new(void) { struct Dict_string_f64 dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_f64_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_f64_add(struct Dict_string_f64 *dict, const char *key, double value) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_f64_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } dict->entries[dict->len].key = glitch_strdup(key); dict->entries[dict->len].value = value; dict->len++; }
static int Dict_string_f64_contains_key(struct Dict_string_f64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static double Dict_string_f64_get(struct Dict_string_f64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_f64_remove(struct Dict_string_f64 *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_f64_clear(struct Dict_string_f64 *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); } dict->len = 0; }
static void Dict_string_f64_free(struct Dict_string_f64 *dict) { Dict_string_f64_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

struct Dict_string_string_entry { char *key; char *value; };
struct Dict_string_string { struct Dict_string_string_entry *entries; int len; int cap; };
static struct Dict_string_string Dict_string_string_new(void) { struct Dict_string_string dict; dict.len = 0; dict.cap = 4; dict.entries = malloc(sizeof(struct Dict_string_string_entry) * (size_t)dict.cap); if (!dict.entries) { abort(); } return dict; }
static void Dict_string_string_reserve_one(struct Dict_string_string *dict) { if (dict->len >= dict->cap) { dict->cap *= 2; dict->entries = realloc(dict->entries, sizeof(struct Dict_string_string_entry) * (size_t)dict->cap); if (!dict->entries) { abort(); } } }
static void Dict_string_string_add_owned(struct Dict_string_string *dict, const char *key, char *value) { const char *safe_key = key ? key : ""; for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, safe_key) == 0) { free(dict->entries[i].value); dict->entries[i].value = value ? value : glitch_strdup(""); return; } } Dict_string_string_reserve_one(dict); dict->entries[dict->len].key = glitch_strdup(safe_key); dict->entries[dict->len].value = value ? value : glitch_strdup(""); dict->len++; }
static void Dict_string_string_add(struct Dict_string_string *dict, const char *key, const char *value) { Dict_string_string_add_owned(dict, key, glitch_strdup(value ? value : "")); }
static int Dict_string_string_contains_key(struct Dict_string_string *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return 1; } } return 0; }
static char * Dict_string_string_get(struct Dict_string_string *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { return dict->entries[i].value; } } abort(); }
static int Dict_string_string_remove(struct Dict_string_string *dict, const char *key) { for (int i = 0; i < dict->len; i++) { if (strcmp(dict->entries[i].key, key) == 0) { free(dict->entries[i].key); free(dict->entries[i].value); for (int j = i + 1; j < dict->len; j++) { dict->entries[j - 1] = dict->entries[j]; } dict->len--; return 1; } } return 0; }
static void Dict_string_string_clear(struct Dict_string_string *dict) { for (int i = 0; i < dict->len; i++) { free(dict->entries[i].key); free(dict->entries[i].value); } dict->len = 0; }
static void Dict_string_string_free(struct Dict_string_string *dict) { Dict_string_string_clear(dict); free(dict->entries); dict->entries = NULL; dict->cap = 0; }

typedef void (*glitch_thread_entry)(void);
struct GlitchThread { glitch_thread_entry entry; };
static struct GlitchThread GlitchThread_new(glitch_thread_entry entry) { struct GlitchThread thread; thread.entry = entry; return thread; }
static void GlitchThread_start(struct GlitchThread *thread) { thread->entry(); }
static void GlitchThread_join(struct GlitchThread *thread) { (void)thread; }
struct GlitchTask { struct GlitchThread thread; };
static struct GlitchTask GlitchTask_run(glitch_thread_entry entry) { struct GlitchTask task; task.thread = GlitchThread_new(entry); GlitchThread_start(&task.thread); return task; }
static void GlitchTask_wait(struct GlitchTask *task) { GlitchThread_join(&task->thread); }

typedef int (*glitch_task_i32_entry)(void);
typedef long long (*glitch_task_i64_entry)(void);
typedef int (*glitch_task_bool_entry)(void);
typedef double (*glitch_task_f64_entry)(void);
typedef char * (*glitch_task_string_entry)(void);
typedef void * (*glitch_task_ptr_entry)(void);
struct GlitchTask_bool { int result; };
struct GlitchTask_i32 { int result; };
struct GlitchTask_i64 { long long result; };
struct GlitchTask_f64 { double result; };
struct GlitchTask_string { char *result; };
struct GlitchTask_ptr { void *result; };
static struct GlitchTask_bool GlitchTask_bool_run(glitch_task_bool_entry entry) { struct GlitchTask_bool task; task.result = entry(); return task; }
static struct GlitchTask_i32 GlitchTask_i32_run(glitch_task_i32_entry entry) { struct GlitchTask_i32 task; task.result = entry(); return task; }
static struct GlitchTask_i64 GlitchTask_i64_run(glitch_task_i64_entry entry) { struct GlitchTask_i64 task; task.result = entry(); return task; }
static struct GlitchTask_f64 GlitchTask_f64_run(glitch_task_f64_entry entry) { struct GlitchTask_f64 task; task.result = entry(); return task; }
static struct GlitchTask_string GlitchTask_string_run(glitch_task_string_entry entry) { struct GlitchTask_string task; task.result = entry(); return task; }
static struct GlitchTask_ptr GlitchTask_ptr_run(glitch_task_ptr_entry entry) { struct GlitchTask_ptr task; task.result = entry(); return task; }
static struct GlitchTask_bool GlitchTask_bool_from_result(int result) { struct GlitchTask_bool task; task.result = result; return task; }
static struct GlitchTask_i32 GlitchTask_i32_from_result(int result) { struct GlitchTask_i32 task; task.result = result; return task; }
static struct GlitchTask_i64 GlitchTask_i64_from_result(long long result) { struct GlitchTask_i64 task; task.result = result; return task; }
static struct GlitchTask_f64 GlitchTask_f64_from_result(double result) { struct GlitchTask_f64 task; task.result = result; return task; }
static struct GlitchTask_string GlitchTask_string_from_result(const char *result) { struct GlitchTask_string task; task.result = glitch_strdup(result ? result : ""); return task; }
static struct GlitchTask_string GlitchTask_string_from_owned(char *result) { struct GlitchTask_string task; task.result = result; return task; }
static struct GlitchTask_ptr GlitchTask_ptr_from_result(void *result) { struct GlitchTask_ptr task; task.result = result; return task; }
static void GlitchTask_bool_wait(struct GlitchTask_bool *task) { (void)task; }
static void GlitchTask_i32_wait(struct GlitchTask_i32 *task) { (void)task; }
static void GlitchTask_i64_wait(struct GlitchTask_i64 *task) { (void)task; }
static void GlitchTask_f64_wait(struct GlitchTask_f64 *task) { (void)task; }
static void GlitchTask_string_wait(struct GlitchTask_string *task) { (void)task; }
static void GlitchTask_ptr_wait(struct GlitchTask_ptr *task) { (void)task; }
static int GlitchTask_bool_result(struct GlitchTask_bool *task) { return task->result; }
static int GlitchTask_i32_result(struct GlitchTask_i32 *task) { return task->result; }
static long long GlitchTask_i64_result(struct GlitchTask_i64 *task) { return task->result; }
static double GlitchTask_f64_result(struct GlitchTask_f64 *task) { return task->result; }
static char * GlitchTask_string_result(struct GlitchTask_string *task) { char *result = task->result; task->result = NULL; return result; }
static void * GlitchTask_ptr_result(struct GlitchTask_ptr *task) { return task->result; }
static void GlitchTask_bool_free(struct GlitchTask_bool *task) { (void)task; }
static void GlitchTask_i32_free(struct GlitchTask_i32 *task) { (void)task; }
static void GlitchTask_i64_free(struct GlitchTask_i64 *task) { (void)task; }
static void GlitchTask_f64_free(struct GlitchTask_f64 *task) { (void)task; }
static void GlitchTask_string_free(struct GlitchTask_string *task) { free(task->result); task->result = NULL; }
static void GlitchTask_ptr_free(struct GlitchTask_ptr *task) { task->result = NULL; }

/* metadata: namespace=Library.Models.Entities attributes=Flags */
enum Genre {
    Genre_None = 0,
    Genre_SelfHelp = 1,
    Genre_Fiction = 2,
    Genre_Education = 4,
    Genre_Business = 8,
    Genre_Technology = 16,
    Genre_Medicine = 32,
    Genre_History = 64,
    Genre_Politics = 128,
    Genre_Arts = 256,
    Genre_Law = 512,
    Genre_Science = 1024,
    Genre_Psychology = 2048,
    Genre_Religion = 4096,
    Genre_Travel = 8192,
    Genre_Music = 16384,
    Genre_Film = 32768,
    Genre_Drama = 65536,
    Genre_Comedy = 131072,
    Genre_Animation = 262144,
    Genre_Game = 524288,
    Genre_Anime = 1048576,
    Genre_Cartoon = 2097152,
    Genre_Children = 4194304,
    Genre_Comics = 8388608,
    Genre_Fantasy = 16777216,
    Genre_Horror = 33554432,
    Genre_ScienceFiction = 67108864,
    Genre_Romance = 134217728,
    Genre_Thriller = 268435456,
    Genre_Mystery = 536870912,
    Genre_Historical = 1073741824,
    Genre_Biography = 2147483648,
    Genre_Poetry = 4294967296,
    Genre_Cooking = 8589934592,
    Genre_Health = 17179869184,
    Genre_Art = 34359738368,
    Genre_Other = 68719476736,
};

/* metadata: namespace=Library.Models.Entities attributes=EntityTypeConfiguration(<expr>) */
struct Book {
    char * Title;
    char * Description;
    int BookGenre;
    int Credit;
    char * ImageURL;
    char * ImagePath;
    struct IFormFile * Image;
    int NumberOfTotalCopies;
    int NumberOfAvailableCopies;
    struct IEnumerable_Borrowing * Borrowings;
    struct IEnumerable_Author * Authors;
    struct IEnumerable_BookAuthor * BookAuthors;
    struct IEnumerable_Publisher * Publishers;
    struct IEnumerable_BookPublisher * BookPublishers;
    char * CreatedAt;
};

static struct Book * glitch_alloc_Book(struct Book value) { struct Book *ptr = malloc(sizeof(struct Book)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_Book(struct Book *value) {
    if (!value) { return; }
    free(value->Title);
    free(value->Description);
    free(value->ImageURL);
    free(value->ImagePath);
    free(value->CreatedAt);
}

