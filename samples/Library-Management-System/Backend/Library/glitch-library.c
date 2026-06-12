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

/*
Glitching System.Collections.Generic package marker.

Current monomorphized runtime instantiations:
- List<int>
- List<long>
- List<bool>
- List<double>
- List<string>
- IEnumerable<int>
- IEnumerable<long>
- IEnumerable<bool>
- IEnumerable<double>
- IEnumerable<string>
- Dictionary<string,int>
- Dictionary<string,long>
- Dictionary<string,bool>
- Dictionary<string,double>
- Dictionary<string,string>

The compiler maps C# using System.Collections.Generic; to these owned,
drop-checked runtime structures. String collections own stored strings.

Supported members in this package slice:
- List<T>.Add(T)
- List<T>.Clear()
- List<T>.Contains(T)
- List<T>.Count
- List<T>[int]
- foreach over List<T>
- foreach over IEnumerable<T>
- Dictionary<string,T>.Add(string, T)
- Dictionary<string,T>.Clear()
- Dictionary<string,T>.ContainsKey(string)
- Dictionary<string,T>.Remove(string)
- Dictionary<string,T>.Count
- Dictionary<string,T>[string]
*/
/*
Minimal Microsoft.EntityFrameworkCore groundwork:
- DbContext owns connection/provider/tracked entity state and exposes Dispose().
- DbSetString and IQueryableString model tracked vs no-tracking query ownership.
- Query materialization returns owned List<string> results.
- This intentionally avoids shared arbitrary entity references; tracked state is
  represented by owned string keys until generic entities and lifetimes are richer.
*/
/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
enum BorrowingAction {
    BorrowingAction_Request,
    BorrowingAction_Confirm,
    BorrowingAction_Cancel,
    BorrowingAction_Approve,
    BorrowingAction_Reject,
    BorrowingAction_Return,
};

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities attributes=Flags */
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

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities */
enum BorrowingStatus {
    BorrowingStatus_Pending,
    BorrowingStatus_Cancelled,
    BorrowingStatus_Approved,
    BorrowingStatus_Rejected,
    BorrowingStatus_Borrowed,
    BorrowingStatus_Returned,
};

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Entities */
enum Role {
    Role_Admin,
    Role_User,
};

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Entities */
enum Sex {
    Sex_Male,
    Sex_Female,
};

struct SqlProvider {
    char * Name;
    char * ConnectionString;
};

static struct SqlProvider * glitch_alloc_SqlProvider(struct SqlProvider value) { struct SqlProvider *ptr = malloc(sizeof(struct SqlProvider)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SqlProvider(struct SqlProvider *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->ConnectionString);
}

struct DbContext {
    char * ConnectionString;
    int IsDisposed;
    struct List_string TrackedEntities;
    struct SqlProvider * Provider;
};

static struct DbContext * glitch_alloc_DbContext(struct DbContext value) { struct DbContext *ptr = malloc(sizeof(struct DbContext)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_DbContext(struct DbContext *value) {
    if (!value) { return; }
    free(value->ConnectionString);
    List_string_free(&value->TrackedEntities);
    if (value->Provider) { glitch_drop_SqlProvider(value->Provider); free(value->Provider); }
}

struct IQueryableString {
    char * ConnectionString;
    char * Table;
    int Tracking;
};

static struct IQueryableString * glitch_alloc_IQueryableString(struct IQueryableString value) { struct IQueryableString *ptr = malloc(sizeof(struct IQueryableString)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_IQueryableString(struct IQueryableString *value) {
    if (!value) { return; }
    free(value->ConnectionString);
    free(value->Table);
}

struct DbSetString {
    char * ConnectionString;
    char * Table;
};

static struct DbSetString * glitch_alloc_DbSetString(struct DbSetString value) { struct DbSetString *ptr = malloc(sizeof(struct DbSetString)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_DbSetString(struct DbSetString *value) {
    if (!value) { return; }
    free(value->ConnectionString);
    free(value->Table);
}

/* metadata: namespace=Library.Api.ApiVersionSupport */
struct ApiVersionConfiguration {
};

static struct ApiVersionConfiguration * glitch_alloc_ApiVersionConfiguration(struct ApiVersionConfiguration value) { struct ApiVersionConfiguration *ptr = malloc(sizeof(struct ApiVersionConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ApiVersionConfiguration(struct ApiVersionConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=Route("api/[controller]"), Route("api/v{version:apiVersion}/[controller]"), ApiController, ApiVersion("1.0-Beta") */
struct AuthController {
    struct JwtOptions * _jwtOptions;
    struct IUserDataService * _userDataService;
};

static struct AuthController * glitch_alloc_AuthController(struct AuthController value) { struct AuthController *ptr = malloc(sizeof(struct AuthController)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AuthController(struct AuthController *value) {
    if (!value) { return; }
    if (value->_jwtOptions) { glitch_drop_JwtOptions(value->_jwtOptions); free(value->_jwtOptions); }
    if (value->_userDataService) { glitch_drop_IUserDataService(value->_userDataService); free(value->_userDataService); }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
struct AuthorController {
};

static struct AuthorController * glitch_alloc_AuthorController(struct AuthorController value) { struct AuthorController *ptr = malloc(sizeof(struct AuthorController)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AuthorController(struct AuthorController *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers.Base attributes=ApiController, Route("api/[controller]"), Route("api/v{version:apiVersion}/[controller]"), ApiVersion("1.0-Beta") */
struct BaseCrudController {
    struct IBaseRepo_TEntity * _mainRepo;
    struct IAppLogging_TController * _logger;
    struct IMapper * _mapper;
};

static struct BaseCrudController * glitch_alloc_BaseCrudController(struct BaseCrudController value) { struct BaseCrudController *ptr = malloc(sizeof(struct BaseCrudController)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BaseCrudController(struct BaseCrudController *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
struct BookController {
    struct IBookDataService * _bookDataService;
};

static struct BookController * glitch_alloc_BookController(struct BookController value) { struct BookController *ptr = malloc(sizeof(struct BookController)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookController(struct BookController *value) {
    if (!value) { return; }
    if (value->_bookDataService) { glitch_drop_IBookDataService(value->_bookDataService); free(value->_bookDataService); }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
struct BorrowingController {
    struct IBorrowingDataService * _borrwingDataService;
};

static struct BorrowingController * glitch_alloc_BorrowingController(struct BorrowingController value) { struct BorrowingController *ptr = malloc(sizeof(struct BorrowingController)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingController(struct BorrowingController *value) {
    if (!value) { return; }
    if (value->_borrwingDataService) { glitch_drop_IBorrowingDataService(value->_borrwingDataService); free(value->_borrwingDataService); }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
struct PublisherController {
};

static struct PublisherController * glitch_alloc_PublisherController(struct PublisherController value) { struct PublisherController *ptr = malloc(sizeof(struct PublisherController)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PublisherController(struct PublisherController *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
struct UserController {
    struct IUserDataService * _userDataService;
};

static struct UserController * glitch_alloc_UserController(struct UserController value) { struct UserController *ptr = malloc(sizeof(struct UserController)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserController(struct UserController *value) {
    if (!value) { return; }
    if (value->_userDataService) { glitch_drop_IUserDataService(value->_userDataService); free(value->_userDataService); }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
struct WebException {
    char * TypeBase;
    char * Type;
    int Status;
    char * Title;
    char * Code;
};

static struct WebException * glitch_alloc_WebException(struct WebException value) { struct WebException *ptr = malloc(sizeof(struct WebException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_WebException(struct WebException *value) {
    if (!value) { return; }
    free(value->TypeBase);
    free(value->Type);
    free(value->Title);
    free(value->Code);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
struct ConflictException {
    struct WebException __base;
};

static struct ConflictException * glitch_alloc_ConflictException(struct ConflictException value) { struct ConflictException *ptr = malloc(sizeof(struct ConflictException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ConflictException(struct ConflictException *value) {
    if (!value) { return; }
    glitch_drop_WebException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
struct ForbiddenException {
    struct WebException __base;
};

static struct ForbiddenException * glitch_alloc_ForbiddenException(struct ForbiddenException value) { struct ForbiddenException *ptr = malloc(sizeof(struct ForbiddenException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ForbiddenException(struct ForbiddenException *value) {
    if (!value) { return; }
    glitch_drop_WebException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
struct NotFoundException {
    struct WebException __base;
};

static struct NotFoundException * glitch_alloc_NotFoundException(struct NotFoundException value) { struct NotFoundException *ptr = malloc(sizeof(struct NotFoundException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_NotFoundException(struct NotFoundException *value) {
    if (!value) { return; }
    glitch_drop_WebException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
struct RateLimitExceededException {
    struct WebException __base;
};

static struct RateLimitExceededException * glitch_alloc_RateLimitExceededException(struct RateLimitExceededException value) { struct RateLimitExceededException *ptr = malloc(sizeof(struct RateLimitExceededException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_RateLimitExceededException(struct RateLimitExceededException *value) {
    if (!value) { return; }
    glitch_drop_WebException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
struct UnauthorizedException {
    struct WebException __base;
};

static struct UnauthorizedException * glitch_alloc_UnauthorizedException(struct UnauthorizedException value) { struct UnauthorizedException *ptr = malloc(sizeof(struct UnauthorizedException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UnauthorizedException(struct UnauthorizedException *value) {
    if (!value) { return; }
    glitch_drop_WebException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
struct ValidationException {
    struct WebException __base;
    struct IDictionary_string_array_string * Errors;
};

static struct ValidationException * glitch_alloc_ValidationException(struct ValidationException value) { struct ValidationException *ptr = malloc(sizeof(struct ValidationException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ValidationException(struct ValidationException *value) {
    if (!value) { return; }
    glitch_drop_WebException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.Action */
struct ValidateImageUploadAttribute {
    char * _parameterName;
};

static struct ValidateImageUploadAttribute * glitch_alloc_ValidateImageUploadAttribute(struct ValidateImageUploadAttribute value) { struct ValidateImageUploadAttribute *ptr = malloc(sizeof(struct ValidateImageUploadAttribute)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ValidateImageUploadAttribute(struct ValidateImageUploadAttribute *value) {
    if (!value) { return; }
    free(value->_parameterName);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters */
struct CustomExceptionFilter {
    struct IWebHostEnvironment * _hostEnviroment;
};

static struct CustomExceptionFilter * glitch_alloc_CustomExceptionFilter(struct CustomExceptionFilter value) { struct CustomExceptionFilter *ptr = malloc(sizeof(struct CustomExceptionFilter)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_CustomExceptionFilter(struct CustomExceptionFilter *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Middlewares */
struct RateLimitMiddleware {
    struct RequestDelegate * _next;
    struct IMemoryCache * _cache;
};

static struct RateLimitMiddleware * glitch_alloc_RateLimitMiddleware(struct RateLimitMiddleware value) { struct RateLimitMiddleware *ptr = malloc(sizeof(struct RateLimitMiddleware)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_RateLimitMiddleware(struct RateLimitMiddleware *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Middlewares */
struct RateLimit {
    char * LastRequest;
    int Requests;
};

static struct RateLimit * glitch_alloc_RateLimit(struct RateLimit value) { struct RateLimit *ptr = malloc(sizeof(struct RateLimit)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_RateLimit(struct RateLimit *value) {
    if (!value) { return; }
    free(value->LastRequest);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models */
struct SwaggerApplicationSettings {
    char * Title;
    struct List_SwaggerVersionDescription * Descriptions;
    char * ContactName;
    char * ContactEmail;
    char * ContactUrl;
};

static struct SwaggerApplicationSettings * glitch_alloc_SwaggerApplicationSettings(struct SwaggerApplicationSettings value) { struct SwaggerApplicationSettings *ptr = malloc(sizeof(struct SwaggerApplicationSettings)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SwaggerApplicationSettings(struct SwaggerApplicationSettings *value) {
    if (!value) { return; }
    free(value->Title);
    free(value->ContactName);
    free(value->ContactEmail);
    free(value->ContactUrl);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models */
struct SwaggerVersionDescription {
    int MajorVersion;
    int MinorVersion;
    char * Status;
    char * Description;
};

static struct SwaggerVersionDescription * glitch_alloc_SwaggerVersionDescription(struct SwaggerVersionDescription value) { struct SwaggerVersionDescription *ptr = malloc(sizeof(struct SwaggerVersionDescription)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SwaggerVersionDescription(struct SwaggerVersionDescription *value) {
    if (!value) { return; }
    free(value->Status);
    free(value->Description);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger */
struct SwaggerConfigOptions {
    struct IApiVersionDescriptionProvider * _provider;
    struct SwaggerApplicationSettings * _settings;
};

static struct SwaggerConfigOptions * glitch_alloc_SwaggerConfigOptions(struct SwaggerConfigOptions value) { struct SwaggerConfigOptions *ptr = malloc(sizeof(struct SwaggerConfigOptions)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SwaggerConfigOptions(struct SwaggerConfigOptions *value) {
    if (!value) { return; }
    if (value->_settings) { glitch_drop_SwaggerApplicationSettings(value->_settings); free(value->_settings); }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger */
struct SwaggerConfiguration {
};

static struct SwaggerConfiguration * glitch_alloc_SwaggerConfiguration(struct SwaggerConfiguration value) { struct SwaggerConfiguration *ptr = malloc(sizeof(struct SwaggerConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SwaggerConfiguration(struct SwaggerConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger */
struct SwaggerDocumentFilter {
};

static struct SwaggerDocumentFilter * glitch_alloc_SwaggerDocumentFilter(struct SwaggerDocumentFilter value) { struct SwaggerDocumentFilter *ptr = malloc(sizeof(struct SwaggerDocumentFilter)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SwaggerDocumentFilter(struct SwaggerDocumentFilter *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures */
struct ApplicationDbContext {
    struct DbContext __base;
    struct DbSet_Book * Books;
    struct DbSet_Publisher * Publishers;
    struct DbSet_BookPublisher * BookPublishers;
    struct DbSet_Author * Authors;
    struct DbSet_BookAuthor * BookAuthors;
    struct DbSet_Borrowing * Borrowings;
    struct DbSet_User * Users;
    struct DbSet_SeriLogEntry * SeriLogEntries;
};

static struct ApplicationDbContext * glitch_alloc_ApplicationDbContext(struct ApplicationDbContext value) { struct ApplicationDbContext *ptr = malloc(sizeof(struct ApplicationDbContext)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ApplicationDbContext(struct ApplicationDbContext *value) {
    if (!value) { return; }
    glitch_drop_DbContext(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EfStructures */
struct ApplicationDbContextFactory {
};

static struct ApplicationDbContextFactory * glitch_alloc_ApplicationDbContextFactory(struct ApplicationDbContextFactory value) { struct ApplicationDbContextFactory *ptr = malloc(sizeof(struct ApplicationDbContextFactory)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ApplicationDbContextFactory(struct ApplicationDbContextFactory *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures */
struct ExceptionInterceptor {
};

static struct ExceptionInterceptor * glitch_alloc_ExceptionInterceptor(struct ExceptionInterceptor value) { struct ExceptionInterceptor *ptr = malloc(sizeof(struct ExceptionInterceptor)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ExceptionInterceptor(struct ExceptionInterceptor *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240329134110_Initial") */
struct Initial {
};

static struct Initial * glitch_alloc_Initial(struct Initial value) { struct Initial *ptr = malloc(sizeof(struct Initial)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_Initial(struct Initial *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct Initial {
};

static struct Initial * glitch_alloc_Initial(struct Initial value) { struct Initial *ptr = malloc(sizeof(struct Initial)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_Initial(struct Initial *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240329141659_UpdatedUsersEmailAsIndex") */
struct UpdatedUsersEmailAsIndex {
};

static struct UpdatedUsersEmailAsIndex * glitch_alloc_UpdatedUsersEmailAsIndex(struct UpdatedUsersEmailAsIndex value) { struct UpdatedUsersEmailAsIndex *ptr = malloc(sizeof(struct UpdatedUsersEmailAsIndex)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UpdatedUsersEmailAsIndex(struct UpdatedUsersEmailAsIndex *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct UpdatedUsersEmailAsIndex {
};

static struct UpdatedUsersEmailAsIndex * glitch_alloc_UpdatedUsersEmailAsIndex(struct UpdatedUsersEmailAsIndex value) { struct UpdatedUsersEmailAsIndex *ptr = malloc(sizeof(struct UpdatedUsersEmailAsIndex)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UpdatedUsersEmailAsIndex(struct UpdatedUsersEmailAsIndex *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240416115200_ImageToBookSupport") */
struct ImageToBookSupport {
};

static struct ImageToBookSupport * glitch_alloc_ImageToBookSupport(struct ImageToBookSupport value) { struct ImageToBookSupport *ptr = malloc(sizeof(struct ImageToBookSupport)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ImageToBookSupport(struct ImageToBookSupport *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct ImageToBookSupport {
};

static struct ImageToBookSupport * glitch_alloc_ImageToBookSupport(struct ImageToBookSupport value) { struct ImageToBookSupport *ptr = malloc(sizeof(struct ImageToBookSupport)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ImageToBookSupport(struct ImageToBookSupport *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240422125324_ReplacedIEumerablesWithICollections") */
struct ReplacedIEumerablesWithICollections {
};

static struct ReplacedIEumerablesWithICollections * glitch_alloc_ReplacedIEumerablesWithICollections(struct ReplacedIEumerablesWithICollections value) { struct ReplacedIEumerablesWithICollections *ptr = malloc(sizeof(struct ReplacedIEumerablesWithICollections)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ReplacedIEumerablesWithICollections(struct ReplacedIEumerablesWithICollections *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct ReplacedIEumerablesWithICollections {
};

static struct ReplacedIEumerablesWithICollections * glitch_alloc_ReplacedIEumerablesWithICollections(struct ReplacedIEumerablesWithICollections value) { struct ReplacedIEumerablesWithICollections *ptr = malloc(sizeof(struct ReplacedIEumerablesWithICollections)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ReplacedIEumerablesWithICollections(struct ReplacedIEumerablesWithICollections *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240423042244_RenamedLoanToBorrowing") */
struct RenamedLoanToBorrowing {
};

static struct RenamedLoanToBorrowing * glitch_alloc_RenamedLoanToBorrowing(struct RenamedLoanToBorrowing value) { struct RenamedLoanToBorrowing *ptr = malloc(sizeof(struct RenamedLoanToBorrowing)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_RenamedLoanToBorrowing(struct RenamedLoanToBorrowing *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct RenamedLoanToBorrowing {
};

static struct RenamedLoanToBorrowing * glitch_alloc_RenamedLoanToBorrowing(struct RenamedLoanToBorrowing value) { struct RenamedLoanToBorrowing *ptr = malloc(sizeof(struct RenamedLoanToBorrowing)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_RenamedLoanToBorrowing(struct RenamedLoanToBorrowing *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240424025629_AddedCreditToUsersAndBooks") */
struct AddedCreditToUsersAndBooks {
};

static struct AddedCreditToUsersAndBooks * glitch_alloc_AddedCreditToUsersAndBooks(struct AddedCreditToUsersAndBooks value) { struct AddedCreditToUsersAndBooks *ptr = malloc(sizeof(struct AddedCreditToUsersAndBooks)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddedCreditToUsersAndBooks(struct AddedCreditToUsersAndBooks *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct AddedCreditToUsersAndBooks {
};

static struct AddedCreditToUsersAndBooks * glitch_alloc_AddedCreditToUsersAndBooks(struct AddedCreditToUsersAndBooks value) { struct AddedCreditToUsersAndBooks *ptr = malloc(sizeof(struct AddedCreditToUsersAndBooks)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddedCreditToUsersAndBooks(struct AddedCreditToUsersAndBooks *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240424025936_AddedIsReturnedToBookBorrowing") */
struct AddedIsReturnedToBookBorrowing {
};

static struct AddedIsReturnedToBookBorrowing * glitch_alloc_AddedIsReturnedToBookBorrowing(struct AddedIsReturnedToBookBorrowing value) { struct AddedIsReturnedToBookBorrowing *ptr = malloc(sizeof(struct AddedIsReturnedToBookBorrowing)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddedIsReturnedToBookBorrowing(struct AddedIsReturnedToBookBorrowing *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct AddedIsReturnedToBookBorrowing {
};

static struct AddedIsReturnedToBookBorrowing * glitch_alloc_AddedIsReturnedToBookBorrowing(struct AddedIsReturnedToBookBorrowing value) { struct AddedIsReturnedToBookBorrowing *ptr = malloc(sizeof(struct AddedIsReturnedToBookBorrowing)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddedIsReturnedToBookBorrowing(struct AddedIsReturnedToBookBorrowing *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240424032626_FixingBookBorrowingIsReturned") */
struct FixingBookBorrowingIsReturned {
};

static struct FixingBookBorrowingIsReturned * glitch_alloc_FixingBookBorrowingIsReturned(struct FixingBookBorrowingIsReturned value) { struct FixingBookBorrowingIsReturned *ptr = malloc(sizeof(struct FixingBookBorrowingIsReturned)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_FixingBookBorrowingIsReturned(struct FixingBookBorrowingIsReturned *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct FixingBookBorrowingIsReturned {
};

static struct FixingBookBorrowingIsReturned * glitch_alloc_FixingBookBorrowingIsReturned(struct FixingBookBorrowingIsReturned value) { struct FixingBookBorrowingIsReturned *ptr = malloc(sizeof(struct FixingBookBorrowingIsReturned)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_FixingBookBorrowingIsReturned(struct FixingBookBorrowingIsReturned *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240428101523_AddUserImage") */
struct AddUserImage {
};

static struct AddUserImage * glitch_alloc_AddUserImage(struct AddUserImage value) { struct AddUserImage *ptr = malloc(sizeof(struct AddUserImage)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddUserImage(struct AddUserImage *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct AddUserImage {
};

static struct AddUserImage * glitch_alloc_AddUserImage(struct AddUserImage value) { struct AddUserImage *ptr = malloc(sizeof(struct AddUserImage)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddUserImage(struct AddUserImage *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240503083509_ChangeBorrowingSchema") */
struct ChangeBorrowingSchema {
};

static struct ChangeBorrowingSchema * glitch_alloc_ChangeBorrowingSchema(struct ChangeBorrowingSchema value) { struct ChangeBorrowingSchema *ptr = malloc(sizeof(struct ChangeBorrowingSchema)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ChangeBorrowingSchema(struct ChangeBorrowingSchema *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct ChangeBorrowingSchema {
};

static struct ChangeBorrowingSchema * glitch_alloc_ChangeBorrowingSchema(struct ChangeBorrowingSchema value) { struct ChangeBorrowingSchema *ptr = malloc(sizeof(struct ChangeBorrowingSchema)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ChangeBorrowingSchema(struct ChangeBorrowingSchema *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240504082145_UpdateBorrowingsDates") */
struct UpdateBorrowingsDates {
};

static struct UpdateBorrowingsDates * glitch_alloc_UpdateBorrowingsDates(struct UpdateBorrowingsDates value) { struct UpdateBorrowingsDates *ptr = malloc(sizeof(struct UpdateBorrowingsDates)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UpdateBorrowingsDates(struct UpdateBorrowingsDates *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct UpdateBorrowingsDates {
};

static struct UpdateBorrowingsDates * glitch_alloc_UpdateBorrowingsDates(struct UpdateBorrowingsDates value) { struct UpdateBorrowingsDates *ptr = malloc(sizeof(struct UpdateBorrowingsDates)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UpdateBorrowingsDates(struct UpdateBorrowingsDates *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240505081029_MakeUserPhoneOptional") */
struct MakeUserPhoneOptional {
};

static struct MakeUserPhoneOptional * glitch_alloc_MakeUserPhoneOptional(struct MakeUserPhoneOptional value) { struct MakeUserPhoneOptional *ptr = malloc(sizeof(struct MakeUserPhoneOptional)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_MakeUserPhoneOptional(struct MakeUserPhoneOptional *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct MakeUserPhoneOptional {
};

static struct MakeUserPhoneOptional * glitch_alloc_MakeUserPhoneOptional(struct MakeUserPhoneOptional value) { struct MakeUserPhoneOptional *ptr = malloc(sizeof(struct MakeUserPhoneOptional)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_MakeUserPhoneOptional(struct MakeUserPhoneOptional *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240505081427_MakeUserCreditOptional") */
struct MakeUserCreditOptional {
};

static struct MakeUserCreditOptional * glitch_alloc_MakeUserCreditOptional(struct MakeUserCreditOptional value) { struct MakeUserCreditOptional *ptr = malloc(sizeof(struct MakeUserCreditOptional)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_MakeUserCreditOptional(struct MakeUserCreditOptional *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct MakeUserCreditOptional {
};

static struct MakeUserCreditOptional * glitch_alloc_MakeUserCreditOptional(struct MakeUserCreditOptional value) { struct MakeUserCreditOptional *ptr = malloc(sizeof(struct MakeUserCreditOptional)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_MakeUserCreditOptional(struct MakeUserCreditOptional *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240507041611_ChangeIsReturnedToStatusInBorrowingTable") */
struct ChangeIsReturnedToStatusInBorrowingTable {
};

static struct ChangeIsReturnedToStatusInBorrowingTable * glitch_alloc_ChangeIsReturnedToStatusInBorrowingTable(struct ChangeIsReturnedToStatusInBorrowingTable value) { struct ChangeIsReturnedToStatusInBorrowingTable *ptr = malloc(sizeof(struct ChangeIsReturnedToStatusInBorrowingTable)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ChangeIsReturnedToStatusInBorrowingTable(struct ChangeIsReturnedToStatusInBorrowingTable *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct ChangeIsReturnedToStatusInBorrowingTable {
};

static struct ChangeIsReturnedToStatusInBorrowingTable * glitch_alloc_ChangeIsReturnedToStatusInBorrowingTable(struct ChangeIsReturnedToStatusInBorrowingTable value) { struct ChangeIsReturnedToStatusInBorrowingTable *ptr = malloc(sizeof(struct ChangeIsReturnedToStatusInBorrowingTable)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ChangeIsReturnedToStatusInBorrowingTable(struct ChangeIsReturnedToStatusInBorrowingTable *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240507042851_AddAdminReferencesToBorrowingTable") */
struct AddAdminReferencesToBorrowingTable {
};

static struct AddAdminReferencesToBorrowingTable * glitch_alloc_AddAdminReferencesToBorrowingTable(struct AddAdminReferencesToBorrowingTable value) { struct AddAdminReferencesToBorrowingTable *ptr = malloc(sizeof(struct AddAdminReferencesToBorrowingTable)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddAdminReferencesToBorrowingTable(struct AddAdminReferencesToBorrowingTable *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct AddAdminReferencesToBorrowingTable {
};

static struct AddAdminReferencesToBorrowingTable * glitch_alloc_AddAdminReferencesToBorrowingTable(struct AddAdminReferencesToBorrowingTable value) { struct AddAdminReferencesToBorrowingTable *ptr = malloc(sizeof(struct AddAdminReferencesToBorrowingTable)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddAdminReferencesToBorrowingTable(struct AddAdminReferencesToBorrowingTable *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240508043348_AddCreatedAtToBooksAndUsersAndBorrowings") */
struct AddCreatedAtToBooksAndUsersAndBorrowings {
};

static struct AddCreatedAtToBooksAndUsersAndBorrowings * glitch_alloc_AddCreatedAtToBooksAndUsersAndBorrowings(struct AddCreatedAtToBooksAndUsersAndBorrowings value) { struct AddCreatedAtToBooksAndUsersAndBorrowings *ptr = malloc(sizeof(struct AddCreatedAtToBooksAndUsersAndBorrowings)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddCreatedAtToBooksAndUsersAndBorrowings(struct AddCreatedAtToBooksAndUsersAndBorrowings *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct AddCreatedAtToBooksAndUsersAndBorrowings {
};

static struct AddCreatedAtToBooksAndUsersAndBorrowings * glitch_alloc_AddCreatedAtToBooksAndUsersAndBorrowings(struct AddCreatedAtToBooksAndUsersAndBorrowings value) { struct AddCreatedAtToBooksAndUsersAndBorrowings *ptr = malloc(sizeof(struct AddCreatedAtToBooksAndUsersAndBorrowings)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddCreatedAtToBooksAndUsersAndBorrowings(struct AddCreatedAtToBooksAndUsersAndBorrowings *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240509053831_AddBioToUsers") */
struct AddBioToUsers {
};

static struct AddBioToUsers * glitch_alloc_AddBioToUsers(struct AddBioToUsers value) { struct AddBioToUsers *ptr = malloc(sizeof(struct AddBioToUsers)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddBioToUsers(struct AddBioToUsers *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct AddBioToUsers {
};

static struct AddBioToUsers * glitch_alloc_AddBioToUsers(struct AddBioToUsers value) { struct AddBioToUsers *ptr = malloc(sizeof(struct AddBioToUsers)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddBioToUsers(struct AddBioToUsers *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240517042624_AddUserSex") */
struct AddUserSex {
};

static struct AddUserSex * glitch_alloc_AddUserSex(struct AddUserSex value) { struct AddUserSex *ptr = malloc(sizeof(struct AddUserSex)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddUserSex(struct AddUserSex *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct AddUserSex {
};

static struct AddUserSex * glitch_alloc_AddUserSex(struct AddUserSex value) { struct AddUserSex *ptr = malloc(sizeof(struct AddUserSex)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddUserSex(struct AddUserSex *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240517051007_AddGenreToBooks") */
struct AddGenreToBooks {
};

static struct AddGenreToBooks * glitch_alloc_AddGenreToBooks(struct AddGenreToBooks value) { struct AddGenreToBooks *ptr = malloc(sizeof(struct AddGenreToBooks)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddGenreToBooks(struct AddGenreToBooks *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct AddGenreToBooks {
};

static struct AddGenreToBooks * glitch_alloc_AddGenreToBooks(struct AddGenreToBooks value) { struct AddGenreToBooks *ptr = malloc(sizeof(struct AddGenreToBooks)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddGenreToBooks(struct AddGenreToBooks *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>), Migration("20240603074642_AddBirthDateToUsers") */
struct AddBirthDateToUsers {
};

static struct AddBirthDateToUsers * glitch_alloc_AddBirthDateToUsers(struct AddBirthDateToUsers value) { struct AddBirthDateToUsers *ptr = malloc(sizeof(struct AddBirthDateToUsers)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddBirthDateToUsers(struct AddBirthDateToUsers *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
struct AddBirthDateToUsers {
};

static struct AddBirthDateToUsers * glitch_alloc_AddBirthDateToUsers(struct AddBirthDateToUsers value) { struct AddBirthDateToUsers *ptr = malloc(sizeof(struct AddBirthDateToUsers)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AddBirthDateToUsers(struct AddBirthDateToUsers *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations attributes=DbContext(<expr>) */
struct ApplicationDbContextModelSnapshot {
};

static struct ApplicationDbContextModelSnapshot * glitch_alloc_ApplicationDbContextModelSnapshot(struct ApplicationDbContextModelSnapshot value) { struct ApplicationDbContextModelSnapshot *ptr = malloc(sizeof(struct ApplicationDbContextModelSnapshot)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_ApplicationDbContextModelSnapshot(struct ApplicationDbContextModelSnapshot *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Exceptions */
struct UnknownDatabaseException {
};

static struct UnknownDatabaseException * glitch_alloc_UnknownDatabaseException(struct UnknownDatabaseException value) { struct UnknownDatabaseException *ptr = malloc(sizeof(struct UnknownDatabaseException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UnknownDatabaseException(struct UnknownDatabaseException *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
struct LinqHelpers {
    struct HashSet_string * singleOperandOperators;
};

static struct LinqHelpers * glitch_alloc_LinqHelpers(struct LinqHelpers value) { struct LinqHelpers *ptr = malloc(sizeof(struct LinqHelpers)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_LinqHelpers(struct LinqHelpers *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
struct SampleData {
};

static struct SampleData * glitch_alloc_SampleData(struct SampleData value) { struct SampleData *ptr = malloc(sizeof(struct SampleData)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SampleData(struct SampleData *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
struct SampleDataInitializer {
};

static struct SampleDataInitializer * glitch_alloc_SampleDataInitializer(struct SampleDataInitializer value) { struct SampleDataInitializer *ptr = malloc(sizeof(struct SampleDataInitializer)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SampleDataInitializer(struct SampleDataInitializer *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
struct AuthorRepo {
};

static struct AuthorRepo * glitch_alloc_AuthorRepo(struct AuthorRepo value) { struct AuthorRepo *ptr = malloc(sizeof(struct AuthorRepo)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AuthorRepo(struct AuthorRepo *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
struct BaseRepo {
};

static struct BaseRepo * glitch_alloc_BaseRepo(struct BaseRepo value) { struct BaseRepo *ptr = malloc(sizeof(struct BaseRepo)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BaseRepo(struct BaseRepo *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
struct BaseViewRepo {
    int disposedValue;
    struct DbSet_T * Table;
    struct ApplicationDbContext * Context;
};

static struct BaseViewRepo * glitch_alloc_BaseViewRepo(struct BaseViewRepo value) { struct BaseViewRepo *ptr = malloc(sizeof(struct BaseViewRepo)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BaseViewRepo(struct BaseViewRepo *value) {
    if (!value) { return; }
    if (value->Context) { glitch_drop_ApplicationDbContext(value->Context); free(value->Context); }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
/* interface IBaseRepo */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
/* interface IBaseViewRepo */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
struct BookRepo {
};

static struct BookRepo * glitch_alloc_BookRepo(struct BookRepo value) { struct BookRepo *ptr = malloc(sizeof(struct BookRepo)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookRepo(struct BookRepo *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
struct BorrowingRepo {
};

static struct BorrowingRepo * glitch_alloc_BorrowingRepo(struct BorrowingRepo value) { struct BorrowingRepo *ptr = malloc(sizeof(struct BorrowingRepo)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingRepo(struct BorrowingRepo *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
struct PublisherRepo {
};

static struct PublisherRepo * glitch_alloc_PublisherRepo(struct PublisherRepo value) { struct PublisherRepo *ptr = malloc(sizeof(struct PublisherRepo)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PublisherRepo(struct PublisherRepo *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
struct UserRepo {
};

static struct UserRepo * glitch_alloc_UserRepo(struct UserRepo value) { struct UserRepo *ptr = malloc(sizeof(struct UserRepo)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserRepo(struct UserRepo *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
/* interface IAuthorRepo */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.interfaces */
/* interface IBookRepo */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
/* interface IBorrowingRepo */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
/* interface IPublisherRepo */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
/* interface IUserRepo */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct AuthorCreateRequestDTO {
    char * Name;
};

static struct AuthorCreateRequestDTO * glitch_alloc_AuthorCreateRequestDTO(struct AuthorCreateRequestDTO value) { struct AuthorCreateRequestDTO *ptr = malloc(sizeof(struct AuthorCreateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AuthorCreateRequestDTO(struct AuthorCreateRequestDTO *value) {
    if (!value) { return; }
    free(value->Name);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct AuthorDTO {
    struct BaseDTO __base;
    char * Name;
};

static struct AuthorDTO * glitch_alloc_AuthorDTO(struct AuthorDTO value) { struct AuthorDTO *ptr = malloc(sizeof(struct AuthorDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AuthorDTO(struct AuthorDTO *value) {
    if (!value) { return; }
    free(value->Name);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct AuthorResponseDTO {
    struct BaseDTO __base;
    char * Name;
};

static struct AuthorResponseDTO * glitch_alloc_AuthorResponseDTO(struct AuthorResponseDTO value) { struct AuthorResponseDTO *ptr = malloc(sizeof(struct AuthorResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AuthorResponseDTO(struct AuthorResponseDTO *value) {
    if (!value) { return; }
    free(value->Name);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct AuthorUpdateRequestDTO {
    struct BaseDTO __base;
    char * Name;
};

static struct AuthorUpdateRequestDTO * glitch_alloc_AuthorUpdateRequestDTO(struct AuthorUpdateRequestDTO value) { struct AuthorUpdateRequestDTO *ptr = malloc(sizeof(struct AuthorUpdateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AuthorUpdateRequestDTO(struct AuthorUpdateRequestDTO *value) {
    if (!value) { return; }
    free(value->Name);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO.Base */
struct BaseDTO {
    int Id;
    struct GlitchArray_byte TimeStamp;
};

static struct BaseDTO * glitch_alloc_BaseDTO(struct BaseDTO value) { struct BaseDTO *ptr = malloc(sizeof(struct BaseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BaseDTO(struct BaseDTO *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO.Base */
/* interface IImageUploadable */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BookCreateRequestDTO {
    char * Title;
    char * Description;
    int BookGenre;
    int Credit;
    struct IFormFile * Image;
    int NumberOfTotalCopies;
    int NumberOfAvailableCopies;
};

static struct BookCreateRequestDTO * glitch_alloc_BookCreateRequestDTO(struct BookCreateRequestDTO value) { struct BookCreateRequestDTO *ptr = malloc(sizeof(struct BookCreateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookCreateRequestDTO(struct BookCreateRequestDTO *value) {
    if (!value) { return; }
    free(value->Title);
    free(value->Description);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BookDTO {
    struct BaseDTO __base;
    char * Title;
    char * Description;
    int BookGenre;
    int Credit;
    int NumberOfTotalCopies;
    int NumberOfAvailableCopies;
};

static struct BookDTO * glitch_alloc_BookDTO(struct BookDTO value) { struct BookDTO *ptr = malloc(sizeof(struct BookDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookDTO(struct BookDTO *value) {
    if (!value) { return; }
    free(value->Title);
    free(value->Description);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BookResponseDTO {
    struct BaseDTO __base;
    char * Title;
    char * Description;
    int BookGenre;
    int Credit;
    char * ImageURL;
    struct IEnumerable_AuthorResponseDTO * Authors;
    struct IEnumerable_PublisherResponseDTO * Publishers;
    int NumberOfTotalCopies;
    int NumberOfAvailableCopies;
    char * CreatedAt;
};

static struct BookResponseDTO * glitch_alloc_BookResponseDTO(struct BookResponseDTO value) { struct BookResponseDTO *ptr = malloc(sizeof(struct BookResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookResponseDTO(struct BookResponseDTO *value) {
    if (!value) { return; }
    free(value->Title);
    free(value->Description);
    free(value->ImageURL);
    free(value->CreatedAt);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BookUpdateRequestDTO {
    struct BaseDTO __base;
    char * Title;
    char * Description;
    int BookGenre;
    int Credit;
    struct List_int AuthorsIds;
    struct List_int PublishersIds;
    int NumberOfTotalCopies;
    int NumberOfAvailableCopies;
};

static struct BookUpdateRequestDTO * glitch_alloc_BookUpdateRequestDTO(struct BookUpdateRequestDTO value) { struct BookUpdateRequestDTO *ptr = malloc(sizeof(struct BookUpdateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookUpdateRequestDTO(struct BookUpdateRequestDTO *value) {
    if (!value) { return; }
    free(value->Title);
    free(value->Description);
    List_int_free(&value->AuthorsIds);
    List_int_free(&value->PublishersIds);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BorrowingDTO {
    struct BaseDTO __base;
    char * DateOut;
    char * DueDate;
    int Status;
    int UserId;
    struct BookResponseDTO * BookNavigation;
    struct UserResponseDTO * ApprovedByNavigation;
    struct UserResponseDTO * ReturnedByNavigation;
    struct UserResponseDTO * RejectedByNavigation;
    struct UserResponseDTO * UserNavigation;
    int BookId;
};

static struct BorrowingDTO * glitch_alloc_BorrowingDTO(struct BorrowingDTO value) { struct BorrowingDTO *ptr = malloc(sizeof(struct BorrowingDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingDTO(struct BorrowingDTO *value) {
    if (!value) { return; }
    free(value->DateOut);
    free(value->DueDate);
    if (value->BookNavigation) { glitch_drop_BookResponseDTO(value->BookNavigation); free(value->BookNavigation); }
    if (value->ApprovedByNavigation) { glitch_drop_UserResponseDTO(value->ApprovedByNavigation); free(value->ApprovedByNavigation); }
    if (value->ReturnedByNavigation) { glitch_drop_UserResponseDTO(value->ReturnedByNavigation); free(value->ReturnedByNavigation); }
    if (value->RejectedByNavigation) { glitch_drop_UserResponseDTO(value->RejectedByNavigation); free(value->RejectedByNavigation); }
    if (value->UserNavigation) { glitch_drop_UserResponseDTO(value->UserNavigation); free(value->UserNavigation); }
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BorrowingResponseDTO {
    struct BaseDTO __base;
    int Status;
    char * DateOut;
    char * DueDate;
    char * CreatedAt;
    struct BookResponseDTO * BookNavigation;
    struct MinimalUserResponseDTO * ApprovedByNavigation;
    struct MinimalUserResponseDTO * ReturnedByNavigation;
    struct MinimalUserResponseDTO * RejectedByNavigation;
    struct MinimalUserResponseDTO * UserNavigation;
};

static struct BorrowingResponseDTO * glitch_alloc_BorrowingResponseDTO(struct BorrowingResponseDTO value) { struct BorrowingResponseDTO *ptr = malloc(sizeof(struct BorrowingResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingResponseDTO(struct BorrowingResponseDTO *value) {
    if (!value) { return; }
    free(value->DateOut);
    free(value->DueDate);
    free(value->CreatedAt);
    if (value->BookNavigation) { glitch_drop_BookResponseDTO(value->BookNavigation); free(value->BookNavigation); }
    if (value->ApprovedByNavigation) { glitch_drop_MinimalUserResponseDTO(value->ApprovedByNavigation); free(value->ApprovedByNavigation); }
    if (value->ReturnedByNavigation) { glitch_drop_MinimalUserResponseDTO(value->ReturnedByNavigation); free(value->ReturnedByNavigation); }
    if (value->RejectedByNavigation) { glitch_drop_MinimalUserResponseDTO(value->RejectedByNavigation); free(value->RejectedByNavigation); }
    if (value->UserNavigation) { glitch_drop_MinimalUserResponseDTO(value->UserNavigation); free(value->UserNavigation); }
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BorrowingStatusUpdateRequestDTO {
    int Action;
    struct List_int BorrowingIds;
};

static struct BorrowingStatusUpdateRequestDTO * glitch_alloc_BorrowingStatusUpdateRequestDTO(struct BorrowingStatusUpdateRequestDTO value) { struct BorrowingStatusUpdateRequestDTO *ptr = malloc(sizeof(struct BorrowingStatusUpdateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingStatusUpdateRequestDTO(struct BorrowingStatusUpdateRequestDTO *value) {
    if (!value) { return; }
    List_int_free(&value->BorrowingIds);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BorrowingStatusUpdateResponseDTO {
    struct List_BorrowingResponseDTO * Success;
    struct List_BorrowingRequestsErrors * Errors;
};

static struct BorrowingStatusUpdateResponseDTO * glitch_alloc_BorrowingStatusUpdateResponseDTO(struct BorrowingStatusUpdateResponseDTO value) { struct BorrowingStatusUpdateResponseDTO *ptr = malloc(sizeof(struct BorrowingStatusUpdateResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingStatusUpdateResponseDTO(struct BorrowingStatusUpdateResponseDTO *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BorrowingRequestsErrors {
    int BorrowingId;
    char * Message;
};

static struct BorrowingRequestsErrors * glitch_alloc_BorrowingRequestsErrors(struct BorrowingRequestsErrors value) { struct BorrowingRequestsErrors *ptr = malloc(sizeof(struct BorrowingRequestsErrors)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingRequestsErrors(struct BorrowingRequestsErrors *value) {
    if (!value) { return; }
    free(value->Message);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct PendingBorrowingRequestDTO {
    struct List_int BookIds;
};

static struct PendingBorrowingRequestDTO * glitch_alloc_PendingBorrowingRequestDTO(struct PendingBorrowingRequestDTO value) { struct PendingBorrowingRequestDTO *ptr = malloc(sizeof(struct PendingBorrowingRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PendingBorrowingRequestDTO(struct PendingBorrowingRequestDTO *value) {
    if (!value) { return; }
    List_int_free(&value->BookIds);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct PendingBorrowingResponseDTO {
    struct List_BorrowingResponseDTO * Success;
    struct List_BorrowBooksErrors * Errors;
};

static struct PendingBorrowingResponseDTO * glitch_alloc_PendingBorrowingResponseDTO(struct PendingBorrowingResponseDTO value) { struct PendingBorrowingResponseDTO *ptr = malloc(sizeof(struct PendingBorrowingResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PendingBorrowingResponseDTO(struct PendingBorrowingResponseDTO *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct BorrowBooksErrors {
    int BookId;
    char * Message;
};

static struct BorrowBooksErrors * glitch_alloc_BorrowBooksErrors(struct BorrowBooksErrors value) { struct BorrowBooksErrors *ptr = malloc(sizeof(struct BorrowBooksErrors)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowBooksErrors(struct BorrowBooksErrors *value) {
    if (!value) { return; }
    free(value->Message);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct PublisherCreateRequestDTO {
    char * Name;
    char * Email;
    char * Address;
};

static struct PublisherCreateRequestDTO * glitch_alloc_PublisherCreateRequestDTO(struct PublisherCreateRequestDTO value) { struct PublisherCreateRequestDTO *ptr = malloc(sizeof(struct PublisherCreateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PublisherCreateRequestDTO(struct PublisherCreateRequestDTO *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Email);
    free(value->Address);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct PublisherDTO {
    struct BaseDTO __base;
    char * Name;
    char * Email;
    char * Address;
};

static struct PublisherDTO * glitch_alloc_PublisherDTO(struct PublisherDTO value) { struct PublisherDTO *ptr = malloc(sizeof(struct PublisherDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PublisherDTO(struct PublisherDTO *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Email);
    free(value->Address);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct PublisherResponseDTO {
    struct BaseDTO __base;
    char * Name;
    char * Email;
    char * Address;
};

static struct PublisherResponseDTO * glitch_alloc_PublisherResponseDTO(struct PublisherResponseDTO value) { struct PublisherResponseDTO *ptr = malloc(sizeof(struct PublisherResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PublisherResponseDTO(struct PublisherResponseDTO *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Email);
    free(value->Address);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct PublisherUpdateRequestDTO {
    struct BaseDTO __base;
    char * Name;
    char * Email;
    char * Address;
};

static struct PublisherUpdateRequestDTO * glitch_alloc_PublisherUpdateRequestDTO(struct PublisherUpdateRequestDTO value) { struct PublisherUpdateRequestDTO *ptr = malloc(sizeof(struct PublisherUpdateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PublisherUpdateRequestDTO(struct PublisherUpdateRequestDTO *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Email);
    free(value->Address);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct AuthResponseDTO {
    int UserId;
    char * UserName;
    int UserRole;
    char * ImageUrl;
    char * AccessToken;
};

static struct AuthResponseDTO * glitch_alloc_AuthResponseDTO(struct AuthResponseDTO value) { struct AuthResponseDTO *ptr = malloc(sizeof(struct AuthResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AuthResponseDTO(struct AuthResponseDTO *value) {
    if (!value) { return; }
    free(value->UserName);
    free(value->ImageUrl);
    free(value->AccessToken);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct LoginUserRequestDTO {
    char * Email;
    char * Password;
};

static struct LoginUserRequestDTO * glitch_alloc_LoginUserRequestDTO(struct LoginUserRequestDTO value) { struct LoginUserRequestDTO *ptr = malloc(sizeof(struct LoginUserRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_LoginUserRequestDTO(struct LoginUserRequestDTO *value) {
    if (!value) { return; }
    free(value->Email);
    free(value->Password);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct MinimalUserResponseDTO {
    struct BaseDTO __base;
    char * Name;
};

static struct MinimalUserResponseDTO * glitch_alloc_MinimalUserResponseDTO(struct MinimalUserResponseDTO value) { struct MinimalUserResponseDTO *ptr = malloc(sizeof(struct MinimalUserResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_MinimalUserResponseDTO(struct MinimalUserResponseDTO *value) {
    if (!value) { return; }
    free(value->Name);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct RegisterUserRequestDTO {
    char * Name;
    char * Email;
    char * Password;
};

static struct RegisterUserRequestDTO * glitch_alloc_RegisterUserRequestDTO(struct RegisterUserRequestDTO value) { struct RegisterUserRequestDTO *ptr = malloc(sizeof(struct RegisterUserRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_RegisterUserRequestDTO(struct RegisterUserRequestDTO *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Email);
    free(value->Password);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct UpdatePasswordRequestDTO {
    char * OldPassword;
    char * NewPassword;
};

static struct UpdatePasswordRequestDTO * glitch_alloc_UpdatePasswordRequestDTO(struct UpdatePasswordRequestDTO value) { struct UpdatePasswordRequestDTO *ptr = malloc(sizeof(struct UpdatePasswordRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UpdatePasswordRequestDTO(struct UpdatePasswordRequestDTO *value) {
    if (!value) { return; }
    free(value->OldPassword);
    free(value->NewPassword);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct UserDTO {
    struct BaseDTO __base;
    char * Name;
    struct DateOnly * BirthDate;
    char * Address;
    int UserSex;
    int Credit;
    char * Email;
    char * Phone;
    int UserRole;
};

static struct UserDTO * glitch_alloc_UserDTO(struct UserDTO value) { struct UserDTO *ptr = malloc(sizeof(struct UserDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserDTO(struct UserDTO *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Address);
    free(value->Email);
    free(value->Phone);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct UserResponseDTO {
    struct BaseDTO __base;
    char * Name;
    char * Bio;
    struct DateOnly * BirthDate;
    char * ImageURL;
    char * Address;
    int UserSex;
    int Credit;
    char * Email;
    char * Phone;
    int UserRole;
    char * CreatedAt;
};

static struct UserResponseDTO * glitch_alloc_UserResponseDTO(struct UserResponseDTO value) { struct UserResponseDTO *ptr = malloc(sizeof(struct UserResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserResponseDTO(struct UserResponseDTO *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Bio);
    free(value->ImageURL);
    free(value->Address);
    free(value->Email);
    free(value->Phone);
    free(value->CreatedAt);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.DTO */
struct UserUpdateRequestDTO {
    struct BaseDTO __base;
    char * Name;
    char * Bio;
    char * Address;
    struct DateOnly * BirthDate;
    int UserSex;
    int Credit;
    struct IFormFile * Image;
    char * Email;
    char * Phone;
    int UserRole;
};

static struct UserUpdateRequestDTO * glitch_alloc_UserUpdateRequestDTO(struct UserUpdateRequestDTO value) { struct UserUpdateRequestDTO *ptr = malloc(sizeof(struct UserUpdateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserUpdateRequestDTO(struct UserUpdateRequestDTO *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Bio);
    free(value->Address);
    free(value->Email);
    free(value->Phone);
    glitch_drop_BaseDTO(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities */
struct Author {
    struct BaseEntity __base;
    char * Name;
    struct IEnumerable_Book * Books;
    struct IEnumerable_BookAuthor * BookAuthors;
};

static struct Author * glitch_alloc_Author(struct Author value) { struct Author *ptr = malloc(sizeof(struct Author)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_Author(struct Author *value) {
    if (!value) { return; }
    free(value->Name);
    glitch_drop_BaseEntity(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Base */
struct BaseEntity {
    int Id;
    struct GlitchArray_byte TimeStamp;
};

static struct BaseEntity * glitch_alloc_BaseEntity(struct BaseEntity value) { struct BaseEntity *ptr = malloc(sizeof(struct BaseEntity)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BaseEntity(struct BaseEntity *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities attributes=EntityTypeConfiguration(<expr>) */
struct Book {
    struct BaseEntity __base;
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
    glitch_drop_BaseEntity(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities */
struct BookAuthor {
    struct BaseEntity __base;
    int BookId;
    struct Book * BookNavigation;
    int AuthorId;
    struct Author * AuthorNavigation;
};

static struct BookAuthor * glitch_alloc_BookAuthor(struct BookAuthor value) { struct BookAuthor *ptr = malloc(sizeof(struct BookAuthor)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookAuthor(struct BookAuthor *value) {
    if (!value) { return; }
    if (value->BookNavigation) { glitch_drop_Book(value->BookNavigation); free(value->BookNavigation); }
    if (value->AuthorNavigation) { glitch_drop_Author(value->AuthorNavigation); free(value->AuthorNavigation); }
    glitch_drop_BaseEntity(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities */
struct BookPublisher {
    struct BaseEntity __base;
    int BookId;
    struct Book * BookNavigation;
    int PublisherId;
    struct Publisher * PublisherNavigation;
};

static struct BookPublisher * glitch_alloc_BookPublisher(struct BookPublisher value) { struct BookPublisher *ptr = malloc(sizeof(struct BookPublisher)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookPublisher(struct BookPublisher *value) {
    if (!value) { return; }
    if (value->BookNavigation) { glitch_drop_Book(value->BookNavigation); free(value->BookNavigation); }
    if (value->PublisherNavigation) { glitch_drop_Publisher(value->PublisherNavigation); free(value->PublisherNavigation); }
    glitch_drop_BaseEntity(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities attributes=EntityTypeConfiguration(<expr>) */
struct Borrowing {
    struct BaseEntity __base;
    int UserId;
    struct User * UserNavigation;
    int BookId;
    struct Book * BookNavigation;
    char * DateOut;
    char * DueDate;
    int Status;
    int ApprovedById;
    struct User * ApprovedByNavigation;
    int ReturnedById;
    struct User * ReturnedByNavigation;
    int RejectedById;
    struct User * RejectedByNavigation;
    char * CreatedAt;
};

static struct Borrowing * glitch_alloc_Borrowing(struct Borrowing value) { struct Borrowing *ptr = malloc(sizeof(struct Borrowing)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_Borrowing(struct Borrowing *value) {
    if (!value) { return; }
    if (value->UserNavigation) { glitch_drop_User(value->UserNavigation); free(value->UserNavigation); }
    if (value->BookNavigation) { glitch_drop_Book(value->BookNavigation); free(value->BookNavigation); }
    free(value->DateOut);
    free(value->DueDate);
    if (value->ApprovedByNavigation) { glitch_drop_User(value->ApprovedByNavigation); free(value->ApprovedByNavigation); }
    if (value->ReturnedByNavigation) { glitch_drop_User(value->ReturnedByNavigation); free(value->ReturnedByNavigation); }
    if (value->RejectedByNavigation) { glitch_drop_User(value->RejectedByNavigation); free(value->RejectedByNavigation); }
    free(value->CreatedAt);
    glitch_drop_BaseEntity(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration */
struct BookConfiguration {
};

static struct BookConfiguration * glitch_alloc_BookConfiguration(struct BookConfiguration value) { struct BookConfiguration *ptr = malloc(sizeof(struct BookConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookConfiguration(struct BookConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration */
struct BorrowingConfiguration {
};

static struct BorrowingConfiguration * glitch_alloc_BorrowingConfiguration(struct BorrowingConfiguration value) { struct BorrowingConfiguration *ptr = malloc(sizeof(struct BorrowingConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingConfiguration(struct BorrowingConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers */
struct DateOnlyComparer {
};

static struct DateOnlyComparer * glitch_alloc_DateOnlyComparer(struct DateOnlyComparer value) { struct DateOnlyComparer *ptr = malloc(sizeof(struct DateOnlyComparer)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_DateOnlyComparer(struct DateOnlyComparer *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers */
struct DateOnlyConverter {
};

static struct DateOnlyConverter * glitch_alloc_DateOnlyConverter(struct DateOnlyConverter value) { struct DateOnlyConverter *ptr = malloc(sizeof(struct DateOnlyConverter)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_DateOnlyConverter(struct DateOnlyConverter *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration */
struct SeriLogEntryConfiguration {
};

static struct SeriLogEntryConfiguration * glitch_alloc_SeriLogEntryConfiguration(struct SeriLogEntryConfiguration value) { struct SeriLogEntryConfiguration *ptr = malloc(sizeof(struct SeriLogEntryConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SeriLogEntryConfiguration(struct SeriLogEntryConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Configuration */
struct UserConfiguration {
};

static struct UserConfiguration * glitch_alloc_UserConfiguration(struct UserConfiguration value) { struct UserConfiguration *ptr = malloc(sizeof(struct UserConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserConfiguration(struct UserConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities */
struct Publisher {
    struct BaseEntity __base;
    char * Name;
    char * Email;
    char * Address;
    struct IEnumerable_Book * Books;
    struct IEnumerable_BookPublisher * BookPublishers;
};

static struct Publisher * glitch_alloc_Publisher(struct Publisher value) { struct Publisher *ptr = malloc(sizeof(struct Publisher)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_Publisher(struct Publisher *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Email);
    free(value->Address);
    glitch_drop_BaseEntity(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities attributes=Table("SeriLogs", Schema = "Logging"), EntityTypeConfiguration(<expr>) */
struct SeriLogEntry {
    int Id;
    char * Message;
    char * MessageTemplate;
    char * Level;
    char * TimeStamp;
    char * Exception;
    char * Properties;
    char * LogEvent;
    char * SourceContext;
    char * RequestPath;
    char * ActionName;
    char * ApplicationName;
    char * MachineName;
    char * FilePath;
    char * MemberName;
    int LineNumber;
};

static struct SeriLogEntry * glitch_alloc_SeriLogEntry(struct SeriLogEntry value) { struct SeriLogEntry *ptr = malloc(sizeof(struct SeriLogEntry)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SeriLogEntry(struct SeriLogEntry *value) {
    if (!value) { return; }
    free(value->Message);
    free(value->MessageTemplate);
    free(value->Level);
    free(value->TimeStamp);
    free(value->Exception);
    free(value->Properties);
    free(value->LogEvent);
    free(value->SourceContext);
    free(value->RequestPath);
    free(value->ActionName);
    free(value->ApplicationName);
    free(value->MachineName);
    free(value->FilePath);
    free(value->MemberName);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Entities attributes=EntityTypeConfiguration(<expr>) */
struct User {
    struct BaseEntity __base;
    char * Name;
    char * Bio;
    struct DateOnly * BirthDate;
    char * Address;
    int UserSex;
    int Credit;
    char * ImageURL;
    char * ImagePath;
    struct IFormFile * Image;
    char * Email;
    char * Phone;
    int UserRole;
    struct IEnumerable_Borrowing * Borrowings;
    char * PasswordHash;
    char * CreatedAt;
};

static struct User * glitch_alloc_User(struct User value) { struct User *ptr = malloc(sizeof(struct User)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_User(struct User *value) {
    if (!value) { return; }
    free(value->Name);
    free(value->Bio);
    free(value->Address);
    free(value->ImageURL);
    free(value->ImagePath);
    free(value->Email);
    free(value->Phone);
    free(value->PasswordHash);
    free(value->CreatedAt);
    glitch_drop_BaseEntity(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Options.JwtOptions */
struct JwtOptions {
    char * Issuer;
    char * Audience;
    int Lifetime;
    char * SignKey;
};

static struct JwtOptions * glitch_alloc_JwtOptions(struct JwtOptions value) { struct JwtOptions *ptr = malloc(sizeof(struct JwtOptions)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_JwtOptions(struct JwtOptions *value) {
    if (!value) { return; }
    free(value->Issuer);
    free(value->Audience);
    free(value->SignKey);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Profiles */
struct AutoMapperProfile {
};

static struct AutoMapperProfile * glitch_alloc_AutoMapperProfile(struct AutoMapperProfile value) { struct AutoMapperProfile *ptr = malloc(sizeof(struct AutoMapperProfile)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AutoMapperProfile(struct AutoMapperProfile *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Profiles */
struct MapperExtensions {
};

static struct MapperExtensions * glitch_alloc_MapperExtensions(struct MapperExtensions value) { struct MapperExtensions *ptr = malloc(sizeof(struct MapperExtensions)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_MapperExtensions(struct MapperExtensions *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal.Base */
struct BaseDalDataService {
    struct IBaseRepo_TEntity * _mainRepo;
    struct IAppLogging_TDataService * _logger;
};

static struct BaseDalDataService * glitch_alloc_BaseDalDataService(struct BaseDalDataService value) { struct BaseDalDataService *ptr = malloc(sizeof(struct BaseDalDataService)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BaseDalDataService(struct BaseDalDataService *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
struct BookDalDataService {
    struct IHttpContextAccessor * _httpContextAccessor;
    struct IMapper * _mapper;
    struct IAuthorRepo * _authorRepo;
    struct IPublisherRepo * _publisherRepo;
};

static struct BookDalDataService * glitch_alloc_BookDalDataService(struct BookDalDataService value) { struct BookDalDataService *ptr = malloc(sizeof(struct BookDalDataService)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookDalDataService(struct BookDalDataService *value) {
    if (!value) { return; }
    if (value->_authorRepo) { glitch_drop_IAuthorRepo(value->_authorRepo); free(value->_authorRepo); }
    if (value->_publisherRepo) { glitch_drop_IPublisherRepo(value->_publisherRepo); free(value->_publisherRepo); }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
struct BorrowingDalDataService {
    struct IBookRepo * _bookRepo;
    struct IUserRepo * _userRepo;
    struct IMapper * _mapper;
    struct IHttpContextAccessor * _httpContextAccessor;
    struct HashSet_BorrowingAction * AdminActions;
};

static struct BorrowingDalDataService * glitch_alloc_BorrowingDalDataService(struct BorrowingDalDataService value) { struct BorrowingDalDataService *ptr = malloc(sizeof(struct BorrowingDalDataService)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingDalDataService(struct BorrowingDalDataService *value) {
    if (!value) { return; }
    if (value->_bookRepo) { glitch_drop_IBookRepo(value->_bookRepo); free(value->_bookRepo); }
    if (value->_userRepo) { glitch_drop_IUserRepo(value->_userRepo); free(value->_userRepo); }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
struct UserDalDataService {
    struct IHttpContextAccessor * _httpContextAccessor;
};

static struct UserDalDataService * glitch_alloc_UserDalDataService(struct UserDalDataService value) { struct UserDalDataService *ptr = malloc(sizeof(struct UserDalDataService)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserDalDataService(struct UserDalDataService *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices */
struct DataServiceConfiguration {
};

static struct DataServiceConfiguration * glitch_alloc_DataServiceConfiguration(struct DataServiceConfiguration value) { struct DataServiceConfiguration *ptr = malloc(sizeof(struct DataServiceConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_DataServiceConfiguration(struct DataServiceConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions */
struct DataServiceException {
};

static struct DataServiceException * glitch_alloc_DataServiceException(struct DataServiceException value) { struct DataServiceException *ptr = malloc(sizeof(struct DataServiceException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_DataServiceException(struct DataServiceException *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Book */
struct BookNotFoundException {
    struct DataServiceException __base;
};

static struct BookNotFoundException * glitch_alloc_BookNotFoundException(struct BookNotFoundException value) { struct BookNotFoundException *ptr = malloc(sizeof(struct BookNotFoundException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookNotFoundException(struct BookNotFoundException *value) {
    if (!value) { return; }
    glitch_drop_DataServiceException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Book */
struct BookUpdateConflictException {
    struct DataServiceException __base;
};

static struct BookUpdateConflictException * glitch_alloc_BookUpdateConflictException(struct BookUpdateConflictException value) { struct BookUpdateConflictException *ptr = malloc(sizeof(struct BookUpdateConflictException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookUpdateConflictException(struct BookUpdateConflictException *value) {
    if (!value) { return; }
    glitch_drop_DataServiceException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Borrowing */
struct BorrowingActionForbiddenException {
    struct DataServiceException __base;
};

static struct BorrowingActionForbiddenException * glitch_alloc_BorrowingActionForbiddenException(struct BorrowingActionForbiddenException value) { struct BorrowingActionForbiddenException *ptr = malloc(sizeof(struct BorrowingActionForbiddenException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingActionForbiddenException(struct BorrowingActionForbiddenException *value) {
    if (!value) { return; }
    glitch_drop_DataServiceException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Borrowing */
struct BorrowingUserNotFoundException {
    struct DataServiceException __base;
};

static struct BorrowingUserNotFoundException * glitch_alloc_BorrowingUserNotFoundException(struct BorrowingUserNotFoundException value) { struct BorrowingUserNotFoundException *ptr = malloc(sizeof(struct BorrowingUserNotFoundException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingUserNotFoundException(struct BorrowingUserNotFoundException *value) {
    if (!value) { return; }
    glitch_drop_DataServiceException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
struct InvalidUserException {
    struct DataServiceException __base;
};

static struct InvalidUserException * glitch_alloc_InvalidUserException(struct InvalidUserException value) { struct InvalidUserException *ptr = malloc(sizeof(struct InvalidUserException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_InvalidUserException(struct InvalidUserException *value) {
    if (!value) { return; }
    glitch_drop_DataServiceException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
struct UserAlreadyExistException {
    struct DataServiceException __base;
};

static struct UserAlreadyExistException * glitch_alloc_UserAlreadyExistException(struct UserAlreadyExistException value) { struct UserAlreadyExistException *ptr = malloc(sizeof(struct UserAlreadyExistException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserAlreadyExistException(struct UserAlreadyExistException *value) {
    if (!value) { return; }
    glitch_drop_DataServiceException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
struct UserForbidenExcepiton {
    struct DataServiceException __base;
};

static struct UserForbidenExcepiton * glitch_alloc_UserForbidenExcepiton(struct UserForbidenExcepiton value) { struct UserForbidenExcepiton *ptr = malloc(sizeof(struct UserForbidenExcepiton)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserForbidenExcepiton(struct UserForbidenExcepiton *value) {
    if (!value) { return; }
    glitch_drop_DataServiceException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
struct UserNotFoundException {
    struct DataServiceException __base;
};

static struct UserNotFoundException * glitch_alloc_UserNotFoundException(struct UserNotFoundException value) { struct UserNotFoundException *ptr = malloc(sizeof(struct UserNotFoundException)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserNotFoundException(struct UserNotFoundException *value) {
    if (!value) { return; }
    glitch_drop_DataServiceException(&value->__base);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Helpers */
struct JwtHelpers {
};

static struct JwtHelpers * glitch_alloc_JwtHelpers(struct JwtHelpers value) { struct JwtHelpers *ptr = malloc(sizeof(struct JwtHelpers)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_JwtHelpers(struct JwtHelpers *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Interfaces */
/* interface IBaseDataService */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Interfaces */
/* interface IBookDataService */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Interfaces */
/* interface IBorrowingDataService */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Interfaces */
/* interface IUserDataService */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
struct AppLogging {
    struct ILogger_T * _logger;
};

static struct AppLogging * glitch_alloc_AppLogging(struct AppLogging value) { struct AppLogging *ptr = malloc(sizeof(struct AppLogging)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AppLogging(struct AppLogging *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging.Library.Services.Logging.Configuration */
struct LoggingConfiguration {
    char * OutputTemplate;
    struct ColumnOptions * ColumnOptions;
};

static struct LoggingConfiguration * glitch_alloc_LoggingConfiguration(struct LoggingConfiguration value) { struct LoggingConfiguration *ptr = malloc(sizeof(struct LoggingConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_LoggingConfiguration(struct LoggingConfiguration *value) {
    if (!value) { return; }
    free(value->OutputTemplate);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging.Library.Services.Logging.Interfaces */
/* interface IAppLogging */

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging.Library.Services.Logging.Interfaces.Library.Services.Logging.Settings */
struct AppLoggingSettings {
    struct GeneralSettings * General;
    struct FileSettings * File;
    struct SqlServerSettings * MSSqlServer;
};

static struct AppLoggingSettings * glitch_alloc_AppLoggingSettings(struct AppLoggingSettings value) { struct AppLoggingSettings *ptr = malloc(sizeof(struct AppLoggingSettings)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AppLoggingSettings(struct AppLoggingSettings *value) {
    if (!value) { return; }
}

static struct SqlProvider * SqlProvider_new(char * name, char * connectionString);
static char * SqlProvider_BuildSelectAll(struct SqlProvider * self, char * table);
static struct DbContext * DbContext_new(char * connectionString);
static void DbContext_EnsureNotDisposed(struct DbContext * self);
static void DbContext_Track(struct DbContext * self, char * entityKey);
static int DbContext_get_TrackedCount(struct DbContext * self);
static void DbContext_Dispose(struct DbContext * self);
static struct IQueryableString * IQueryableString_new(char * connectionString, char * table, int tracking);
static struct IQueryableString * IQueryableString_AsNoTracking(struct IQueryableString * self);
static char * IQueryableString_ToQueryString(struct IQueryableString * self);
static struct List_string IQueryableString_ToList(struct IQueryableString * self);
static struct DbSetString * DbSetString_new(char * connectionString, char * table);
static struct IQueryableString * DbSetString_AsQueryable(struct DbSetString * self);
static struct IQueryableString * DbSetString_AsNoTracking(struct DbSetString * self);
static struct List_string DbSetString_ToList(struct DbSetString * self);
static struct IServiceCollection * ApiVersionConfiguration_AddLibraryApiVersionConfiguration(struct ApiVersionConfiguration * self, struct IServiceCollection * services, struct ApiVersion * defaultVersion);
static struct AuthController * AuthController_new(struct JwtOptions * jwtOptions, struct IUserDataService * userDataService);
static struct GlitchTask_ptr AuthController_Register(struct AuthController * self, struct RegisterUserRequestDTO * registerRequestDto);
static struct GlitchTask_ptr AuthController_Login(struct AuthController * self, struct LoginUserRequestDTO * user);
static struct GlitchTask_ptr AuthController_ChangePassword(struct AuthController * self, struct UpdatePasswordRequestDTO * user);
static struct AuthorController * AuthorController_new(struct IAppLogging_AuthorController * logger, struct IAuthorRepo * mainRepo, struct IMapper * mapper);
static struct BaseCrudController * BaseCrudController_new(struct IAppLogging_TController * logger, struct IBaseRepo_TEntity * mainRepo, struct IMapper * mapper);
static struct GlitchTask_ptr BaseCrudController_GetAll(struct BaseCrudController * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber);
static struct GlitchTask_ptr BaseCrudController_GetOneAsync(struct BaseCrudController * self, int id);
static struct GlitchTask_ptr BaseCrudController_UpdateOneAsync(struct BaseCrudController * self, int id, struct TUpdateRequestDto * entity);
static struct GlitchTask_ptr BaseCrudController_AddOneAsync(struct BaseCrudController * self, struct TCreateRequestDto * entity);
static struct GlitchTask_ptr BaseCrudController_DeleteOneAsync(struct BaseCrudController * self, int id, struct BaseDTO * entity);
static struct BookController * BookController_new(struct IAppLogging_BookController * logger, struct IBookRepo * mainRepo, struct IBookDataService * bookDataService, struct IMapper * mapper);
static struct GlitchTask_ptr BookController_AddOneAsync(struct BookController * self, struct BookCreateRequestDTO * entity);
static struct GlitchTask_ptr BookController_UpdateOneAsync(struct BookController * self, int id, struct BookUpdateRequestDTO * editedBookDto);
static struct BorrowingController * BorrowingController_new(struct IAppLogging_BorrowingController * logger, struct IBorrowingRepo * mainRepo, struct IBorrowingDataService * borrowingDataService, struct IMapper * mapper);
static struct GlitchTask_ptr BorrowingController_GetAll(struct BorrowingController * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber);
static struct GlitchTask_ptr BorrowingController_UpdateOneAsync(struct BorrowingController * self, int id, struct BorrowingDTO * entity);
static struct GlitchTask_ptr BorrowingController_AddOneAsync(struct BorrowingController * self, struct BorrowingDTO * entity);
static struct GlitchTask_ptr BorrowingController_InitiateBorrowingAsync(struct BorrowingController * self, struct PendingBorrowingRequestDTO * userBorrowingRequest);
static struct GlitchTask_ptr BorrowingController_ActOnBorrowingStatusAsync(struct BorrowingController * self, struct BorrowingStatusUpdateRequestDTO * borrowingStatusUpdateRequest);
static struct PublisherController * PublisherController_new(struct IAppLogging_PublisherController * logger, struct IPublisherRepo * mainRepo, struct IMapper * mapper);
static struct UserController * UserController_new(struct IAppLogging_UserController * logger, struct IUserRepo * mainRepo, struct IUserDataService * userDataService, struct IMapper * mapper);
static struct GlitchTask_ptr UserController_AddOneAsync(struct UserController * self, struct UserDTO * entity);
static struct GlitchTask_ptr UserController_UpdateOneAsync(struct UserController * self, int id, struct UserUpdateRequestDTO * entity);
static struct WebException * WebException_new(void);
static struct WebException * WebException_new(char * message);
static struct WebException * WebException_new(char * message, struct GlitchException innerException);
static struct ConflictException * ConflictException_new(void);
static struct ConflictException * ConflictException_new(char * message);
static struct ConflictException * ConflictException_new(char * message, struct GlitchException innerException);
static struct ForbiddenException * ForbiddenException_new(void);
static struct ForbiddenException * ForbiddenException_new(char * message);
static struct ForbiddenException * ForbiddenException_new(char * message, struct GlitchException innerException);
static struct NotFoundException * NotFoundException_new(void);
static struct NotFoundException * NotFoundException_new(char * message);
static struct NotFoundException * NotFoundException_new(char * message, struct GlitchException innerException);
static struct RateLimitExceededException * RateLimitExceededException_new(void);
static struct RateLimitExceededException * RateLimitExceededException_new(char * message);
static struct RateLimitExceededException * RateLimitExceededException_new(char * message, struct GlitchException innerException);
static struct UnauthorizedException * UnauthorizedException_new(void);
static struct UnauthorizedException * UnauthorizedException_new(char * message);
static struct UnauthorizedException * UnauthorizedException_new(char * message, struct GlitchException innerException);
static struct ValidationException * ValidationException_new(void);
static struct ValidationException * ValidationException_new(char * message);
static struct ValidationException * ValidationException_new(char * errors);
static struct ValidationException * ValidationException_new(char * message, struct GlitchException innerException);
static struct ValidationException * ValidationException_new(char * message, struct GlitchException errors);
static struct ValidationException * ValidationException_new(char * message, struct GlitchException innerException, struct IDictionary_string_array_string * errors);
static struct ValidateImageUploadAttribute * ValidateImageUploadAttribute_new(char * parameterName);
static void ValidateImageUploadAttribute_OnActionExecuting(struct ValidateImageUploadAttribute * self, struct ActionExecutingContext * context);
static struct CustomExceptionFilter * CustomExceptionFilter_new(struct IWebHostEnvironment * hostEnviroment);
static void CustomExceptionFilter_OnException(struct CustomExceptionFilter * self, struct ExceptionContext * context);
static struct RateLimitMiddleware * RateLimitMiddleware_new(struct RequestDelegate * next, struct IMemoryCache * cache);
static struct GlitchTask RateLimitMiddleware_InvokeAsync(struct RateLimitMiddleware * self, struct HttpContext * context);
static struct SwaggerConfigOptions * SwaggerConfigOptions_new(struct IApiVersionDescriptionProvider * provider, struct IOptionsMonitor_SwaggerApplicationSettings * settingsMonitor);
static void SwaggerConfigOptions_Configure(struct SwaggerConfigOptions * self, struct SwaggerGenOptions * options);
static struct OpenApiInfo * SwaggerConfigOptions_CreateInfoForApiVersion(struct SwaggerConfigOptions * self, struct ApiVersionDescription * description, struct SwaggerApplicationSettings * settings);
static void SwaggerConfiguration_AddAndConfigureSwagger(struct SwaggerConfiguration * self, struct IServiceCollection * services, struct IConfiguration * config, char * xmlPathAndFile, int addBearerSecurity);
static void SwaggerDocumentFilter_Apply(struct SwaggerDocumentFilter * self, struct OpenApiDocument * swaggerDoc, struct DocumentFilterContext * context);
static struct ApplicationDbContext * ApplicationDbContext_new(struct DbContextOptions_ApplicationDbContext * options);
static void ApplicationDbContext_OnModelCreating(struct ApplicationDbContext * self, struct ModelBuilder * modelBuilder);
static struct ApplicationDbContext * ApplicationDbContextFactory_CreateDbContext(struct ApplicationDbContextFactory * self, struct string_array * args);
static struct InterceptionResult_int * ExceptionInterceptor_SavingChanges(struct ExceptionInterceptor * self, struct DbContextEventData * eventData, struct InterceptionResult_int * result);
static struct GlitchTask_ptr ExceptionInterceptor_SavingChangesAsync(struct ExceptionInterceptor * self, struct DbContextEventData * eventData, struct InterceptionResult_int * result, struct CancellationToken * cancellationToken);
static void Initial_BuildTargetModel(struct Initial * self, struct ModelBuilder * modelBuilder);
static void Initial_Up(struct Initial * self, struct MigrationBuilder * migrationBuilder);
static void Initial_Down(struct Initial * self, struct MigrationBuilder * migrationBuilder);
static void UpdatedUsersEmailAsIndex_BuildTargetModel(struct UpdatedUsersEmailAsIndex * self, struct ModelBuilder * modelBuilder);
static void UpdatedUsersEmailAsIndex_Up(struct UpdatedUsersEmailAsIndex * self, struct MigrationBuilder * migrationBuilder);
static void UpdatedUsersEmailAsIndex_Down(struct UpdatedUsersEmailAsIndex * self, struct MigrationBuilder * migrationBuilder);
static void ImageToBookSupport_BuildTargetModel(struct ImageToBookSupport * self, struct ModelBuilder * modelBuilder);
static void ImageToBookSupport_Up(struct ImageToBookSupport * self, struct MigrationBuilder * migrationBuilder);
static void ImageToBookSupport_Down(struct ImageToBookSupport * self, struct MigrationBuilder * migrationBuilder);
static void ReplacedIEumerablesWithICollections_BuildTargetModel(struct ReplacedIEumerablesWithICollections * self, struct ModelBuilder * modelBuilder);
static void ReplacedIEumerablesWithICollections_Up(struct ReplacedIEumerablesWithICollections * self, struct MigrationBuilder * migrationBuilder);
static void ReplacedIEumerablesWithICollections_Down(struct ReplacedIEumerablesWithICollections * self, struct MigrationBuilder * migrationBuilder);
static void RenamedLoanToBorrowing_BuildTargetModel(struct RenamedLoanToBorrowing * self, struct ModelBuilder * modelBuilder);
static void RenamedLoanToBorrowing_Up(struct RenamedLoanToBorrowing * self, struct MigrationBuilder * migrationBuilder);
static void RenamedLoanToBorrowing_Down(struct RenamedLoanToBorrowing * self, struct MigrationBuilder * migrationBuilder);
static void AddedCreditToUsersAndBooks_BuildTargetModel(struct AddedCreditToUsersAndBooks * self, struct ModelBuilder * modelBuilder);
static void AddedCreditToUsersAndBooks_Up(struct AddedCreditToUsersAndBooks * self, struct MigrationBuilder * migrationBuilder);
static void AddedCreditToUsersAndBooks_Down(struct AddedCreditToUsersAndBooks * self, struct MigrationBuilder * migrationBuilder);
static void AddedIsReturnedToBookBorrowing_BuildTargetModel(struct AddedIsReturnedToBookBorrowing * self, struct ModelBuilder * modelBuilder);
static void AddedIsReturnedToBookBorrowing_Up(struct AddedIsReturnedToBookBorrowing * self, struct MigrationBuilder * migrationBuilder);
static void AddedIsReturnedToBookBorrowing_Down(struct AddedIsReturnedToBookBorrowing * self, struct MigrationBuilder * migrationBuilder);
static void FixingBookBorrowingIsReturned_BuildTargetModel(struct FixingBookBorrowingIsReturned * self, struct ModelBuilder * modelBuilder);
static void FixingBookBorrowingIsReturned_Up(struct FixingBookBorrowingIsReturned * self, struct MigrationBuilder * migrationBuilder);
static void FixingBookBorrowingIsReturned_Down(struct FixingBookBorrowingIsReturned * self, struct MigrationBuilder * migrationBuilder);
static void AddUserImage_BuildTargetModel(struct AddUserImage * self, struct ModelBuilder * modelBuilder);
static void AddUserImage_Up(struct AddUserImage * self, struct MigrationBuilder * migrationBuilder);
static void AddUserImage_Down(struct AddUserImage * self, struct MigrationBuilder * migrationBuilder);
static void ChangeBorrowingSchema_BuildTargetModel(struct ChangeBorrowingSchema * self, struct ModelBuilder * modelBuilder);
static void ChangeBorrowingSchema_Up(struct ChangeBorrowingSchema * self, struct MigrationBuilder * migrationBuilder);
static void ChangeBorrowingSchema_Down(struct ChangeBorrowingSchema * self, struct MigrationBuilder * migrationBuilder);
static void UpdateBorrowingsDates_BuildTargetModel(struct UpdateBorrowingsDates * self, struct ModelBuilder * modelBuilder);
static void UpdateBorrowingsDates_Up(struct UpdateBorrowingsDates * self, struct MigrationBuilder * migrationBuilder);
static void UpdateBorrowingsDates_Down(struct UpdateBorrowingsDates * self, struct MigrationBuilder * migrationBuilder);
static void MakeUserPhoneOptional_BuildTargetModel(struct MakeUserPhoneOptional * self, struct ModelBuilder * modelBuilder);
static void MakeUserPhoneOptional_Up(struct MakeUserPhoneOptional * self, struct MigrationBuilder * migrationBuilder);
static void MakeUserPhoneOptional_Down(struct MakeUserPhoneOptional * self, struct MigrationBuilder * migrationBuilder);
static void MakeUserCreditOptional_BuildTargetModel(struct MakeUserCreditOptional * self, struct ModelBuilder * modelBuilder);
static void MakeUserCreditOptional_Up(struct MakeUserCreditOptional * self, struct MigrationBuilder * migrationBuilder);
static void MakeUserCreditOptional_Down(struct MakeUserCreditOptional * self, struct MigrationBuilder * migrationBuilder);
static void ChangeIsReturnedToStatusInBorrowingTable_BuildTargetModel(struct ChangeIsReturnedToStatusInBorrowingTable * self, struct ModelBuilder * modelBuilder);
static void ChangeIsReturnedToStatusInBorrowingTable_Up(struct ChangeIsReturnedToStatusInBorrowingTable * self, struct MigrationBuilder * migrationBuilder);
static void ChangeIsReturnedToStatusInBorrowingTable_Down(struct ChangeIsReturnedToStatusInBorrowingTable * self, struct MigrationBuilder * migrationBuilder);
static void AddAdminReferencesToBorrowingTable_BuildTargetModel(struct AddAdminReferencesToBorrowingTable * self, struct ModelBuilder * modelBuilder);
static void AddAdminReferencesToBorrowingTable_Up(struct AddAdminReferencesToBorrowingTable * self, struct MigrationBuilder * migrationBuilder);
static void AddAdminReferencesToBorrowingTable_Down(struct AddAdminReferencesToBorrowingTable * self, struct MigrationBuilder * migrationBuilder);
static void AddCreatedAtToBooksAndUsersAndBorrowings_BuildTargetModel(struct AddCreatedAtToBooksAndUsersAndBorrowings * self, struct ModelBuilder * modelBuilder);
static void AddCreatedAtToBooksAndUsersAndBorrowings_Up(struct AddCreatedAtToBooksAndUsersAndBorrowings * self, struct MigrationBuilder * migrationBuilder);
static void AddCreatedAtToBooksAndUsersAndBorrowings_Down(struct AddCreatedAtToBooksAndUsersAndBorrowings * self, struct MigrationBuilder * migrationBuilder);
static void AddBioToUsers_BuildTargetModel(struct AddBioToUsers * self, struct ModelBuilder * modelBuilder);
static void AddBioToUsers_Up(struct AddBioToUsers * self, struct MigrationBuilder * migrationBuilder);
static void AddBioToUsers_Down(struct AddBioToUsers * self, struct MigrationBuilder * migrationBuilder);
static void AddUserSex_BuildTargetModel(struct AddUserSex * self, struct ModelBuilder * modelBuilder);
static void AddUserSex_Up(struct AddUserSex * self, struct MigrationBuilder * migrationBuilder);
static void AddUserSex_Down(struct AddUserSex * self, struct MigrationBuilder * migrationBuilder);
static void AddGenreToBooks_BuildTargetModel(struct AddGenreToBooks * self, struct ModelBuilder * modelBuilder);
static void AddGenreToBooks_Up(struct AddGenreToBooks * self, struct MigrationBuilder * migrationBuilder);
static void AddGenreToBooks_Down(struct AddGenreToBooks * self, struct MigrationBuilder * migrationBuilder);
static void AddBirthDateToUsers_BuildTargetModel(struct AddBirthDateToUsers * self, struct ModelBuilder * modelBuilder);
static void AddBirthDateToUsers_Up(struct AddBirthDateToUsers * self, struct MigrationBuilder * migrationBuilder);
static void AddBirthDateToUsers_Down(struct AddBirthDateToUsers * self, struct MigrationBuilder * migrationBuilder);
static void ApplicationDbContextModelSnapshot_BuildModel(struct ApplicationDbContextModelSnapshot * self, struct ModelBuilder * modelBuilder);
static struct UnknownDatabaseException * UnknownDatabaseException_new(void);
static struct UnknownDatabaseException * UnknownDatabaseException_new(char * message);
static struct UnknownDatabaseException * UnknownDatabaseException_new(char * message, struct GlitchException innerException);
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicate(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery);
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForBoolProperty(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery);
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForDateTimeProperty(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery);
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForRangeOfDates(struct LinqHelpers * self, struct ParameterExpression * parameter, struct MemberExpression * property, struct MethodInfo * compareToMethod, struct Expression_Func_T_bool * lambda, char * date1, char * date2);
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForSingleDate(struct LinqHelpers * self, struct ParameterExpression * parameter, struct MemberExpression * property, struct MethodInfo * compareToMethod, struct Expression_Func_T_bool * lambda, char * symbol, char * date);
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForIntProperty(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery);
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForRangeOfInts(struct LinqHelpers * self, struct ParameterExpression * parameter, struct MemberExpression * property, struct Expression_Func_T_bool * lambda, int int1, int int2);
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForSingleInt(struct LinqHelpers * self, struct ParameterExpression * parameter, struct MemberExpression * property, struct Expression_Func_T_bool * lambda, char * symbol, int int1);
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForStringProperty(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery);
static char * LinqHelpers_BuildWherePredicateForNestedProperty(struct LinqHelpers * self, char * filterOn, char * filterQuery);
static struct Expression_Func_T_object * LinqHelpers_BuildOrderByFunction(struct LinqHelpers * self, struct PropertyInfo * propertyInfo);
static struct List_Book * SampleData_get_Books(struct SampleData * self);
static struct List_Author * SampleData_get_Authors(struct SampleData * self);
static struct List_BookAuthor * SampleData_get_BookAuthors(struct SampleData * self);
static struct List_Publisher * SampleData_get_Publishers(struct SampleData * self);
static struct List_BookPublisher * SampleData_get_BookPublishers(struct SampleData * self);
static struct List_User * SampleData_get_Users(struct SampleData * self);
static struct List_Borrowing * SampleData_get_Borrowings(struct SampleData * self);
static void SampleDataInitializer_DropAndCreateDatabase(struct SampleDataInitializer * self, struct ApplicationDbContext * context);
static void SampleDataInitializer_SeedData(struct SampleDataInitializer * self, struct ApplicationDbContext * context);
static void SampleDataInitializer_InitializeData(struct SampleDataInitializer * self, struct ApplicationDbContext * context);
static struct AuthorRepo * AuthorRepo_new(struct ApplicationDbContext * context);
static struct BaseRepo * BaseRepo_new(struct ApplicationDbContext * context);
static struct GlitchTask_ptr BaseRepo_FindAsync(struct BaseRepo * self, int id);
static struct GlitchTask_ptr BaseRepo_FindAsNoTrackingAsync(struct BaseRepo * self, int id);
static struct GlitchTask_ptr BaseRepo_FindIgnoreQueryFiltersAsync(struct BaseRepo * self, int id);
static void BaseRepo_ExecuteParameterizedQuery(struct BaseRepo * self, char * sql, struct object_array * sqlParametersObjects);
static struct GlitchTask_i32 BaseRepo_AddAsync(struct BaseRepo * self, struct T * entity, int persist);
static struct GlitchTask_i32 BaseRepo_AddRangeAsync(struct BaseRepo * self, struct IEnumerable_T * entities, int persist);
static struct GlitchTask_i32 BaseRepo_UpdateAsync(struct BaseRepo * self, struct T * entity, int persist);
static struct GlitchTask_i32 BaseRepo_UpdateRangeAsync(struct BaseRepo * self, struct IEnumerable_T * entities, int persist);
static struct GlitchTask_i32 BaseRepo_DeleteAsync__int_Array_Byte__0__bool(struct BaseRepo * self, int id, struct GlitchArray_byte timeStamp, int persist);
static struct GlitchTask_i32 BaseRepo_DeleteAsync__T_bool(struct BaseRepo * self, struct T * entity, int persist);
static struct GlitchTask_i32 BaseRepo_DeleteRangeAsync(struct BaseRepo * self, struct IEnumerable_T * entities, int persist);
static struct GlitchTask_i32 BaseRepo_SaveChangesAsync(struct BaseRepo * self);
static struct BaseViewRepo * BaseViewRepo_new(struct ApplicationDbContext * context);
static struct IEnumerable_T * BaseViewRepo_ExecuteSqlString(struct BaseViewRepo * self, char * sqlString);
static struct IEnumerable_T * BaseViewRepo_GetAll(struct BaseViewRepo * self);
static struct IEnumerable_T * BaseViewRepo_GetAllIgnoreQueryFilters(struct BaseViewRepo * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber);
static void BaseViewRepo_Dispose__bool(struct BaseViewRepo * self, int disposing);
static void BaseViewRepo_Dispose__overload(struct BaseViewRepo * self);
static struct BookRepo * BookRepo_new(struct ApplicationDbContext * context);
static struct IEnumerable_Book * BookRepo_GetAllIgnoreQueryFilters(struct BookRepo * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber);
static struct GlitchTask_ptr BookRepo_FindAsync(struct BookRepo * self, int id);
static struct BorrowingRepo * BorrowingRepo_new(struct ApplicationDbContext * context);
static struct IEnumerable_Borrowing * BorrowingRepo_GetAllIgnoreQueryFilters(struct BorrowingRepo * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber);
static struct GlitchTask_ptr BorrowingRepo_FindAsync(struct BorrowingRepo * self, int id);
static struct PublisherRepo * PublisherRepo_new(struct ApplicationDbContext * context);
static struct UserRepo * UserRepo_new(struct ApplicationDbContext * context);
static struct GlitchTask_ptr UserRepo_FindByEmailAsync(struct UserRepo * self, char * email);
static void BookConfiguration_Configure(struct BookConfiguration * self, struct EntityTypeBuilder_Book * builder);
static void BorrowingConfiguration_Configure(struct BorrowingConfiguration * self, struct EntityTypeBuilder_Borrowing * builder);
static struct DateOnlyComparer * DateOnlyComparer_new(void);
static struct DateOnlyConverter * DateOnlyConverter_new(void);
static void SeriLogEntryConfiguration_Configure(struct SeriLogEntryConfiguration * self, struct EntityTypeBuilder_SeriLogEntry * builder);
static void UserConfiguration_Configure(struct UserConfiguration * self, struct EntityTypeBuilder_User * builder);
static struct XElement * SeriLogEntry_get_PropertiesXml(struct SeriLogEntry * self);
static struct AutoMapperProfile * AutoMapperProfile_new(void);
static struct IMappingExpression_TSource_TDestination * MapperExtensions_IgnoreAllMembers(struct MapperExtensions * self, struct IMappingExpression_TSource_TDestination * expr);
static struct BaseDalDataService * BaseDalDataService_new(struct IBaseRepo_TEntity * mainRepo, struct IAppLogging_TDataService * logger);
static struct GlitchTask_ptr BaseDalDataService_GetAllAsync(struct BaseDalDataService * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber);
static struct GlitchTask_ptr BaseDalDataService_FindAsync(struct BaseDalDataService * self, int id);
static struct GlitchTask_ptr BaseDalDataService_UpdateAsync(struct BaseDalDataService * self, struct TEntity * entity, int persist);
static struct GlitchTask BaseDalDataService_DeleteAsync(struct BaseDalDataService * self, struct TEntity * entity, int persist);
static struct GlitchTask_ptr BaseDalDataService_AddAsync(struct BaseDalDataService * self, struct TEntity * entity, int persist);
static void BaseDalDataService_ResetChangeTracker(struct BaseDalDataService * self);
static struct BookDalDataService * BookDalDataService_new(struct IBookRepo * mainRepo, struct IAuthorRepo * authorRepo, struct IPublisherRepo * publisherRepo, struct IAppLogging_BookDalDataService * logger, struct IHttpContextAccessor * httpContextAccessor, struct IMapper * mapper);
static struct GlitchTask_ptr BookDalDataService_UpdateBookAndItsPublishersAndAuthorsAsync(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, int persist);
static struct GlitchTask_ptr BookDalDataService_AddAsync(struct BookDalDataService * self, struct Book * entity, int persist);
static void BookDalDataService_DeletePublisherFromBook(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, struct Book * existingBook);
static struct GlitchTask BookDalDataService_AddPublisherToBook(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, struct Book * existingBook);
static void BookDalDataService_DeleteAuthorFromBook(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, struct Book * existingBook);
static struct GlitchTask BookDalDataService_AddAuthorToBook(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, struct Book * existingBook);
static struct BorrowingDalDataService * BorrowingDalDataService_new(struct IBorrowingRepo * mainRepo, struct IBookRepo * bookRepo, struct IUserRepo * userRepo, struct IAppLogging_BorrowingDalDataService * logger, struct IHttpContextAccessor * httpContextAccessor, struct IMapper * mapper);
static struct GlitchTask_ptr BorrowingDalDataService_GetAllAsync(struct BorrowingDalDataService * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber);
static struct GlitchTask_ptr BorrowingDalDataService_CreatePendingBorrowingAsync(struct BorrowingDalDataService * self, struct PendingBorrowingRequestDTO * userBorrowingRequest);
static struct GlitchTask_ptr BorrowingDalDataService_UpdateBorrowingStatusAsync(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateRequestDTO * borrowingStatusRequestDTO);
static int BorrowingDalDataService_HandleReturn(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue);
static int BorrowingDalDataService_HandleCancel(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue);
static int BorrowingDalDataService_HandleConfirm(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue);
static int BorrowingDalDataService_HandleReject(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue);
static int BorrowingDalDataService_HandleApprove(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue);
static void BorrowingDalDataService_ConfirmBorrowing(struct BorrowingDalDataService * self, struct Borrowing * borrowing);
static void BorrowingDalDataService_ReturnBorrowing(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing);
static void BorrowingDalDataService_CancelBorrowing(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, int borrowingId, struct Borrowing * borrowing);
static void BorrowingDalDataService_RejectBorrowing(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing);
static void BorrowingDalDataService_ApproveBorrowing(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing);
static struct GlitchTask_ptr BorrowingDalDataService_GetUserFromTokenAsync(struct BorrowingDalDataService * self);
static struct UserDalDataService * UserDalDataService_new(struct IUserRepo * mainRepo, struct IHttpContextAccessor * httpContextAccessor, struct IAppLogging_UserDalDataService * logger);
static struct GlitchTask_ptr UserDalDataService_LoginUserAsync(struct UserDalDataService * self, struct LoginUserRequestDTO * userDTO, struct JwtOptions * jwtOptions);
static struct GlitchTask_ptr UserDalDataService_RegisterUserAsync(struct UserDalDataService * self, struct RegisterUserRequestDTO * userDTO, struct JwtOptions * jwtOptions);
static struct GlitchTask_ptr UserDalDataService_UpdateAsync(struct UserDalDataService * self, struct User * entity, int persist);
static struct GlitchTask_ptr UserDalDataService_UpdatePasswordAsync(struct UserDalDataService * self, struct UpdatePasswordRequestDTO * userDTO, struct JwtOptions * jwtOptions);
static struct IServiceCollection * DataServiceConfiguration_AddRepositories(struct DataServiceConfiguration * self, struct IServiceCollection * services);
static struct IServiceCollection * DataServiceConfiguration_AddDataServices(struct DataServiceConfiguration * self, struct IServiceCollection * services);
static struct DataServiceException * DataServiceException_new(void);
static struct DataServiceException * DataServiceException_new(char * message);
static struct DataServiceException * DataServiceException_new(char * message, struct GlitchException innerException);
static struct BookNotFoundException * BookNotFoundException_new(void);
static struct BookNotFoundException * BookNotFoundException_new(char * message);
static struct BookNotFoundException * BookNotFoundException_new(char * message, struct GlitchException innerException);
static struct BookUpdateConflictException * BookUpdateConflictException_new(void);
static struct BookUpdateConflictException * BookUpdateConflictException_new(char * message);
static struct BookUpdateConflictException * BookUpdateConflictException_new(char * message, struct GlitchException innerException);
static struct BorrowingActionForbiddenException * BorrowingActionForbiddenException_new(void);
static struct BorrowingActionForbiddenException * BorrowingActionForbiddenException_new(char * message);
static struct BorrowingActionForbiddenException * BorrowingActionForbiddenException_new(char * message, struct GlitchException innerException);
static struct BorrowingUserNotFoundException * BorrowingUserNotFoundException_new(void);
static struct BorrowingUserNotFoundException * BorrowingUserNotFoundException_new(char * message);
static struct BorrowingUserNotFoundException * BorrowingUserNotFoundException_new(char * message, struct GlitchException innerException);
static struct InvalidUserException * InvalidUserException_new(void);
static struct InvalidUserException * InvalidUserException_new(char * message);
static struct InvalidUserException * InvalidUserException_new(char * message, struct GlitchException innerException);
static struct UserAlreadyExistException * UserAlreadyExistException_new(void);
static struct UserAlreadyExistException * UserAlreadyExistException_new(char * message);
static struct UserAlreadyExistException * UserAlreadyExistException_new(char * message, struct GlitchException innerException);
static struct UserForbidenExcepiton * UserForbidenExcepiton_new(void);
static struct UserForbidenExcepiton * UserForbidenExcepiton_new(char * message);
static struct UserForbidenExcepiton * UserForbidenExcepiton_new(char * message, struct GlitchException innerException);
static struct UserNotFoundException * UserNotFoundException_new(void);
static struct UserNotFoundException * UserNotFoundException_new(char * message);
static struct UserNotFoundException * UserNotFoundException_new(char * message, struct GlitchException innerException);
static char * JwtHelpers_GenerateJwtToken(struct JwtHelpers * self, struct User * user, struct JwtOptions * jwtOptions);
static int JwtHelpers_ValidateJwtToken(struct JwtHelpers * self, char * token);
static char * JwtHelpers_HashPassword(struct JwtHelpers * self, char * password);
static int JwtHelpers_VerifyPasswordHash(struct JwtHelpers * self, char * password, char * storedHash);
static struct AppLogging * AppLogging_new(struct ILogger_T * logger);
static void AppLogging_LogWithException(struct AppLogging * self, char * memberName, char * sourceFilePath, int sourceLineNumber, struct GlitchException ex, char * message, struct Action_Exception_string_array_object * logAction);
static void AppLogging_LogWithoutException(struct AppLogging * self, char * memberName, char * sourceFilePath, int sourceLineNumber, char * message, struct Action_string_array_object * logAction);
static void AppLogging_LogAppError__Exception_string_string_string_int(struct AppLogging * self, struct GlitchException exception, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber);
static void AppLogging_LogAppError__string_string_string_int(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber);
static void AppLogging_LogAppCritical__Exception_string_string_string_int(struct AppLogging * self, struct GlitchException exception, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber);
static void AppLogging_LogAppCritical__string_string_string_int(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber);
static void AppLogging_LogAppDebug(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber);
static void AppLogging_LogAppTrace(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber);
static void AppLogging_LogAppInformation(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber);
static void AppLogging_LogAppWarning(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber);
static struct IServiceCollection * LoggingConfiguration_RegisterLoggingInterfaces(struct LoggingConfiguration * self, struct IServiceCollection * services);
static void LoggingConfiguration_ConfigureSerilog(struct LoggingConfiguration * self, struct WebApplicationBuilder * builder);
static struct DbSetString * SetString(struct DbContext * context, char * table);

static struct SqlProvider * SqlProvider_new(char * name, char * connectionString) {
    struct SqlProvider * self = glitch_alloc_SqlProvider((struct SqlProvider){0});
    free(self->Name);
    self->Name = glitch_strdup(name);
    free(self->ConnectionString);
    self->ConnectionString = glitch_strdup(connectionString);
    return self;
}

static char * SqlProvider_BuildSelectAll(struct SqlProvider * self, char * table) {
    char * prefix = glitch_strdup("select * from ");
    char * __glitch_return = glitch_string_concat(prefix, table);
    free(prefix);
    return __glitch_return;
}

static struct DbContext * DbContext_new(char * connectionString) {
    struct DbContext * self = glitch_alloc_DbContext((struct DbContext){0});
    free(self->ConnectionString);
    self->ConnectionString = glitch_strdup(connectionString);
    self->IsDisposed = 0;
    List_string_free(&self->TrackedEntities);
    self->TrackedEntities = List_string_new();
    if (self->Provider) { glitch_drop_SqlProvider(self->Provider); free(self->Provider); }
    self->Provider = SqlProvider_new("sql", connectionString);
    return self;
}

static void DbContext_EnsureNotDisposed(struct DbContext * self) {
    if (self->IsDisposed) {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("DbContext is disposed")));
    }
}

static void DbContext_Track(struct DbContext * self, char * entityKey) {
    DbContext_EnsureNotDisposed(self);
    List_string_add(&self->TrackedEntities, entityKey);
}

static int DbContext_get_TrackedCount(struct DbContext * self) {
    int __glitch_return = self->TrackedEntities.len;
    return __glitch_return;
}

static void DbContext_Dispose(struct DbContext * self) {
    self->IsDisposed = 1;
    List_string_clear(&self->TrackedEntities);
}

static struct IQueryableString * IQueryableString_new(char * connectionString, char * table, int tracking) {
    struct IQueryableString * self = glitch_alloc_IQueryableString((struct IQueryableString){0});
    free(self->ConnectionString);
    self->ConnectionString = glitch_strdup(connectionString);
    free(self->Table);
    self->Table = glitch_strdup(table);
    self->Tracking = tracking;
    return self;
}

static struct IQueryableString * IQueryableString_AsNoTracking(struct IQueryableString * self) {
    struct IQueryableString * __glitch_return = IQueryableString_new(self->ConnectionString, self->Table, 0);
    return __glitch_return;
}

static char * IQueryableString_ToQueryString(struct IQueryableString * self) {
    struct SqlProvider * provider = SqlProvider_new("sql", self->ConnectionString);
    char * __glitch_return = SqlProvider_BuildSelectAll(provider, self->Table);
    if (provider) { glitch_drop_SqlProvider(provider); free(provider); }
    return __glitch_return;
}

static struct List_string IQueryableString_ToList(struct IQueryableString * self) {
    struct List_string values = List_string_new();
    char * query = IQueryableString_ToQueryString(self);
    if (self->Tracking) {
    char * prefix = glitch_strdup("tracked:");
    List_string_add(&values, glitch_string_concat(prefix, query));
    free(prefix);
    } else {
    List_string_add(&values, query);
    }
    struct List_string __glitch_return = values;
    free(query);
    List_string_free(&values);
    return __glitch_return;
}

static struct DbSetString * DbSetString_new(char * connectionString, char * table) {
    struct DbSetString * self = glitch_alloc_DbSetString((struct DbSetString){0});
    free(self->ConnectionString);
    self->ConnectionString = glitch_strdup(connectionString);
    free(self->Table);
    self->Table = glitch_strdup(table);
    return self;
}

static struct IQueryableString * DbSetString_AsQueryable(struct DbSetString * self) {
    struct IQueryableString * __glitch_return = IQueryableString_new(self->ConnectionString, self->Table, 1);
    return __glitch_return;
}

static struct IQueryableString * DbSetString_AsNoTracking(struct DbSetString * self) {
    struct IQueryableString * __glitch_return = IQueryableString_new(self->ConnectionString, self->Table, 0);
    return __glitch_return;
}

static struct List_string DbSetString_ToList(struct DbSetString * self) {
    struct List_string __glitch_return = IQueryableString_ToList(DbSetString_AsQueryable(self));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport */
static struct IServiceCollection * ApiVersionConfiguration_AddLibraryApiVersionConfiguration(struct ApiVersionConfiguration * self, struct IServiceCollection * services, struct ApiVersion * defaultVersion) {
    if ((defaultVersion == NULL)) {
    defaultVersion = NULL;
    }
    NULL;
    NULL;
    struct IServiceCollection * __glitch_return = services;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
static struct AuthController * AuthController_new(struct JwtOptions * jwtOptions, struct IUserDataService * userDataService) {
    struct AuthController * self = glitch_alloc_AuthController((struct AuthController){0});
    if (self->_jwtOptions) { glitch_drop_JwtOptions(self->_jwtOptions); free(self->_jwtOptions); }
    self->_jwtOptions = jwtOptions;
    if (self->_userDataService) { glitch_drop_IUserDataService(self->_userDataService); free(self->_userDataService); }
    self->_userDataService = userDataService;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=HttpPost, Route("Register") */
static struct GlitchTask_ptr AuthController_Register(struct AuthController * self, struct RegisterUserRequestDTO * registerRequestDto) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    struct AuthResponseDTO * authResponse = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    if (authResponse) { glitch_drop_AuthResponseDTO(authResponse); free(authResponse); }
    authResponse = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(authResponse));
    if (authResponse) { glitch_drop_AuthResponseDTO(authResponse); free(authResponse); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=HttpPost, Route("Login") */
static struct GlitchTask_ptr AuthController_Login(struct AuthController * self, struct LoginUserRequestDTO * user) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    struct AuthResponseDTO * authResponse = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    if (authResponse) { glitch_drop_AuthResponseDTO(authResponse); free(authResponse); }
    authResponse = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(authResponse));
    if (authResponse) { glitch_drop_AuthResponseDTO(authResponse); free(authResponse); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=HttpPut, Route("ChangePassword"), Authorize */
static struct GlitchTask_ptr AuthController_ChangePassword(struct AuthController * self, struct UpdatePasswordRequestDTO * user) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    struct AuthResponseDTO * authResponse = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    if (authResponse) { glitch_drop_AuthResponseDTO(authResponse); free(authResponse); }
    authResponse = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(authResponse));
    if (authResponse) { glitch_drop_AuthResponseDTO(authResponse); free(authResponse); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
static struct AuthorController * AuthorController_new(struct IAppLogging_AuthorController * logger, struct IAuthorRepo * mainRepo, struct IMapper * mapper) {
    struct AuthorController * self = glitch_alloc_AuthorController((struct AuthorController){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers.Base */
static struct BaseCrudController * BaseCrudController_new(struct IAppLogging_TController * logger, struct IBaseRepo_TEntity * mainRepo, struct IMapper * mapper) {
    struct BaseCrudController * self = glitch_alloc_BaseCrudController((struct BaseCrudController){0});
    self->_mainRepo = mainRepo;
    self->_logger = logger;
    self->_mapper = mapper;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers.Base attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(200, "The execution was successful"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(404, "The requested resource was not found"), SwaggerResponse(500, "An internal server error has occurred"), HttpGet */
static struct GlitchTask_ptr BaseCrudController_GetAll(struct BaseCrudController * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber) {
    struct IEnumerable_TEntity * entities = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    entities = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    if ((entities == NULL)) {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("The requested resource was not found")));
    }
    struct _mapper_Map * entityResponseDto = NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(entityResponseDto));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers.Base attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(200, "The execution was successful"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(404, "The requested resource was not found"), SwaggerResponse(500, "An internal server error has occurred"), HttpGet("{id}") */
static struct GlitchTask_ptr BaseCrudController_GetOneAsync(struct BaseCrudController * self, int id) {
    struct _mainRepo_FindAsync_awaited * entity = NULL;
    if ((entity == NULL)) {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("The requested resource was not found")));
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(NULL));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers.Base attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(200, "The execution was successful"), SwaggerResponse(400, "The request was invalid"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(404, "The requested resource was not found"), SwaggerResponse(500, "An internal server error has occurred"), HttpPut("{id}") */
static struct GlitchTask_ptr BaseCrudController_UpdateOneAsync(struct BaseCrudController * self, int id, struct TUpdateRequestDto * entity) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    if ((id != NULL)) {
    NULL;
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Id in the route and the entity do not match")));
    }
    struct TEntity * domainEntity = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(NULL));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers.Base attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(201, "The execution was successful"), SwaggerResponse(400, "The request was invalid"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(500, "An internal server error has occurred"), HttpPost */
static struct GlitchTask_ptr BaseCrudController_AddOneAsync(struct BaseCrudController * self, struct TCreateRequestDto * entity) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    struct TEntity * domainEntity = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(CreatedAtAction(glitch_strdup(""), NULL, NULL));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers.Base attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(200, "The execution was successful"), SwaggerResponse(400, "The request was invalid"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(404, "The requested resource was not found"), SwaggerResponse(500, "An internal server error has occurred"), HttpDelete("{id}") */
static struct GlitchTask_ptr BaseCrudController_DeleteOneAsync(struct BaseCrudController * self, int id, struct BaseDTO * entity) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    if ((id != entity->Id)) {
    NULL;
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Id in the route and the entity do not match")));
    }
    struct TEntity * domainEntity = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok());
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
static struct BookController * BookController_new(struct IAppLogging_BookController * logger, struct IBookRepo * mainRepo, struct IBookDataService * bookDataService, struct IMapper * mapper) {
    struct BookController * self = glitch_alloc_BookController((struct BookController){0});
    if (self->_bookDataService) { glitch_drop_IBookDataService(self->_bookDataService); free(self->_bookDataService); }
    self->_bookDataService = bookDataService;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(201, "The execution was successful"), SwaggerResponse(400, "The request was invalid"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(500, "An internal server error has occurred"), HttpPost, ValidateImageUpload("entity") */
static struct GlitchTask_ptr BookController_AddOneAsync(struct BookController * self, struct BookCreateRequestDTO * entity) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    struct Book * domainEntity = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    if (domainEntity) { glitch_drop_Book(domainEntity); free(domainEntity); }
    domainEntity = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(CreatedAtAction(glitch_strdup(""), NULL, NULL));
    if (domainEntity) { glitch_drop_Book(domainEntity); free(domainEntity); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(200, "The execution was successful"), SwaggerResponse(400, "The request was invalid"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(404, "The requested resource was not found"), SwaggerResponse(500, "An internal server error has occurred"), HttpPut("{id}") */
static struct GlitchTask_ptr BookController_UpdateOneAsync(struct BookController * self, int id, struct BookUpdateRequestDTO * editedBookDto) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    if ((id != editedBookDto->__base.Id)) {
    NULL;
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Id in route and body do not match")));
    }
    struct Book * editedBook = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    if (editedBook) { glitch_drop_Book(editedBook); free(editedBook); }
    editedBook = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(NULL));
    if (editedBook) { glitch_drop_Book(editedBook); free(editedBook); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
static struct BorrowingController * BorrowingController_new(struct IAppLogging_BorrowingController * logger, struct IBorrowingRepo * mainRepo, struct IBorrowingDataService * borrowingDataService, struct IMapper * mapper) {
    struct BorrowingController * self = glitch_alloc_BorrowingController((struct BorrowingController){0});
    if (self->_borrwingDataService) { glitch_drop_IBorrowingDataService(self->_borrwingDataService); free(self->_borrwingDataService); }
    self->_borrwingDataService = borrowingDataService;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(200, "The execution was successful"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(404, "The requested resource was not found"), SwaggerResponse(500, "An internal server error has occurred"), HttpGet, Authorize */
static struct GlitchTask_ptr BorrowingController_GetAll(struct BorrowingController * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber) {
    struct IEnumerable_Borrowing * entities = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    entities = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    if ((entities == NULL)) {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("The requested resource was not found")));
    }
    struct _mapper_Map * entityResponseDto = NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(entityResponseDto));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=ApiExplorerSettings(IgnoreApi = true) */
static struct GlitchTask_ptr BorrowingController_UpdateOneAsync(struct BorrowingController * self, int id, struct BorrowingDTO * entity) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NoContent());
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=ApiExplorerSettings(IgnoreApi = true) */
static struct GlitchTask_ptr BorrowingController_AddOneAsync(struct BorrowingController * self, struct BorrowingDTO * entity) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NoContent());
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(201, "The borrowing request was successfully created"), SwaggerResponse(207, "Some books were not requested successfully"), SwaggerResponse(400, "The request was invalid"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(500, "An internal server error has occurred"), HttpPost("initiate"), Authorize */
static struct GlitchTask_ptr BorrowingController_InitiateBorrowingAsync(struct BorrowingController * self, struct PendingBorrowingRequestDTO * userBorrowingRequest) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    struct PendingBorrowingResponseDTO * borrowBooksResponseDto = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    if (borrowBooksResponseDto) { glitch_drop_PendingBorrowingResponseDTO(borrowBooksResponseDto); free(borrowBooksResponseDto); }
    borrowBooksResponseDto = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    if ((!1)) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(CreatedAtAction(glitch_strdup(""), NULL, borrowBooksResponseDto));
    if (borrowBooksResponseDto) { glitch_drop_PendingBorrowingResponseDTO(borrowBooksResponseDto); free(borrowBooksResponseDto); }
    return __glitch_return;
    }
    if ((!1)) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(BadRequest(borrowBooksResponseDto));
    if (borrowBooksResponseDto) { glitch_drop_PendingBorrowingResponseDTO(borrowBooksResponseDto); free(borrowBooksResponseDto); }
    return __glitch_return;
    }
    NULL;
    struct ObjectResult * response = NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(response);
    if (borrowBooksResponseDto) { glitch_drop_PendingBorrowingResponseDTO(borrowBooksResponseDto); free(borrowBooksResponseDto); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(200, "The execution was successful"), SwaggerResponse(207, "Some borrowings has not been updated successfully"), SwaggerResponse(400, "The request was invalid"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(404, "The requested resource was not found"), SwaggerResponse(500, "An internal server error has occurred"), Authorize, HttpPut("act-on-borrowing-status") */
static struct GlitchTask_ptr BorrowingController_ActOnBorrowingStatusAsync(struct BorrowingController * self, struct BorrowingStatusUpdateRequestDTO * borrowingStatusUpdateRequest) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    struct BorrowingStatusUpdateResponseDTO * borrowingStatusUpdateResponse = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    if (borrowingStatusUpdateResponse) { glitch_drop_BorrowingStatusUpdateResponseDTO(borrowingStatusUpdateResponse); free(borrowingStatusUpdateResponse); }
    borrowingStatusUpdateResponse = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    if ((!1)) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(borrowingStatusUpdateResponse));
    if (borrowingStatusUpdateResponse) { glitch_drop_BorrowingStatusUpdateResponseDTO(borrowingStatusUpdateResponse); free(borrowingStatusUpdateResponse); }
    return __glitch_return;
    }
    if ((!1)) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(BadRequest(borrowingStatusUpdateResponse));
    if (borrowingStatusUpdateResponse) { glitch_drop_BorrowingStatusUpdateResponseDTO(borrowingStatusUpdateResponse); free(borrowingStatusUpdateResponse); }
    return __glitch_return;
    }
    struct ObjectResult * response = NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(response);
    if (borrowingStatusUpdateResponse) { glitch_drop_BorrowingStatusUpdateResponseDTO(borrowingStatusUpdateResponse); free(borrowingStatusUpdateResponse); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
static struct PublisherController * PublisherController_new(struct IAppLogging_PublisherController * logger, struct IPublisherRepo * mainRepo, struct IMapper * mapper) {
    struct PublisherController * self = glitch_alloc_PublisherController((struct PublisherController){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers */
static struct UserController * UserController_new(struct IAppLogging_UserController * logger, struct IUserRepo * mainRepo, struct IUserDataService * userDataService, struct IMapper * mapper) {
    struct UserController * self = glitch_alloc_UserController((struct UserController){0});
    if (self->_userDataService) { glitch_drop_IUserDataService(self->_userDataService); free(self->_userDataService); }
    self->_userDataService = userDataService;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=ApiExplorerSettings(IgnoreApi = true) */
static struct GlitchTask_ptr UserController_AddOneAsync(struct UserController * self, struct UserDTO * entity) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NoContent());
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Controllers attributes=Produces("application/json"), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), ProducesResponseType(<expr>), SwaggerResponse(200, "The execution was successful"), SwaggerResponse(400, "The request was invalid"), SwaggerResponse(401, "Unauthorized access attempted"), SwaggerResponse(403, "Forbidden access attempted"), SwaggerResponse(404, "The requested resource was not found"), SwaggerResponse(500, "An internal server error has occurred"), HttpPut("{id}"), ValidateImageUpload("entity"), Authorize */
static struct GlitchTask_ptr UserController_UpdateOneAsync(struct UserController * self, int id, struct UserUpdateRequestDTO * entity) {
    if ((!NULL)) {
    struct Dictionary_string_array_string * errors = NULL;
    glitch_throw(glitch_exception_new(""));
    }
    if ((id != entity->__base.Id)) {
    NULL;
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Id in the route and the entity do not match")));
    }
    struct User * domainEntity = NULL;
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    if (domainEntity) { glitch_drop_User(domainEntity); free(domainEntity); }
    domainEntity = NULL;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_new(ex->message));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(Ok(NULL));
    if (domainEntity) { glitch_drop_User(domainEntity); free(domainEntity); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct WebException * WebException_new(void) {
    struct WebException * self = glitch_alloc_WebException((struct WebException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct WebException * WebException_new(char * message) {
    struct WebException * self = glitch_alloc_WebException((struct WebException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct WebException * WebException_new(char * message, struct GlitchException innerException) {
    struct WebException * self = glitch_alloc_WebException((struct WebException){0});
    free(self->Type);
    self->Type = (self->TypeBase + glitch_strdup("internal-server-error"));
    free(self->Code);
    self->Code = glitch_strdup("InternalError");
    self->Status = 500;
    free(self->Title);
    self->Title = glitch_strdup("Internal Server Error");
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ConflictException * ConflictException_new(void) {
    struct ConflictException * self = glitch_alloc_ConflictException((struct ConflictException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ConflictException * ConflictException_new(char * message) {
    struct ConflictException * self = glitch_alloc_ConflictException((struct ConflictException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ConflictException * ConflictException_new(char * message, struct GlitchException innerException) {
    struct ConflictException * self = glitch_alloc_ConflictException((struct ConflictException){0});
    free(self->__base.Type);
    self->__base.Type = (self->__base.TypeBase + glitch_strdup("conflict"));
    self->__base.Status = 409;
    free(self->__base.Title);
    self->__base.Title = glitch_strdup("Conflict");
    free(self->__base.Code);
    self->__base.Code = glitch_strdup("Conflict");
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ForbiddenException * ForbiddenException_new(void) {
    struct ForbiddenException * self = glitch_alloc_ForbiddenException((struct ForbiddenException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ForbiddenException * ForbiddenException_new(char * message) {
    struct ForbiddenException * self = glitch_alloc_ForbiddenException((struct ForbiddenException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ForbiddenException * ForbiddenException_new(char * message, struct GlitchException innerException) {
    struct ForbiddenException * self = glitch_alloc_ForbiddenException((struct ForbiddenException){0});
    free(self->__base.Type);
    self->__base.Type = (self->__base.TypeBase + glitch_strdup("forbidden"));
    self->__base.Status = 403;
    free(self->__base.Title);
    self->__base.Title = glitch_strdup("Forbidden");
    free(self->__base.Code);
    self->__base.Code = glitch_strdup("UserNotAdmin");
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct NotFoundException * NotFoundException_new(void) {
    struct NotFoundException * self = glitch_alloc_NotFoundException((struct NotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct NotFoundException * NotFoundException_new(char * message) {
    struct NotFoundException * self = glitch_alloc_NotFoundException((struct NotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct NotFoundException * NotFoundException_new(char * message, struct GlitchException innerException) {
    struct NotFoundException * self = glitch_alloc_NotFoundException((struct NotFoundException){0});
    free(self->__base.Type);
    self->__base.Type = (self->__base.TypeBase + glitch_strdup("resource-not-found"));
    self->__base.Status = 404;
    free(self->__base.Title);
    self->__base.Title = glitch_strdup("Not Found");
    free(self->__base.Code);
    self->__base.Code = glitch_strdup("ResourceNotFound");
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct RateLimitExceededException * RateLimitExceededException_new(void) {
    struct RateLimitExceededException * self = glitch_alloc_RateLimitExceededException((struct RateLimitExceededException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct RateLimitExceededException * RateLimitExceededException_new(char * message) {
    struct RateLimitExceededException * self = glitch_alloc_RateLimitExceededException((struct RateLimitExceededException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct RateLimitExceededException * RateLimitExceededException_new(char * message, struct GlitchException innerException) {
    struct RateLimitExceededException * self = glitch_alloc_RateLimitExceededException((struct RateLimitExceededException){0});
    free(self->__base.Type);
    self->__base.Type = (self->__base.TypeBase + glitch_strdup("rate-limit-exceeded"));
    self->__base.Status = 429;
    free(self->__base.Title);
    self->__base.Title = glitch_strdup("Rate Limit Exceeded");
    free(self->__base.Code);
    self->__base.Code = glitch_strdup("ServerRateExceeded");
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct UnauthorizedException * UnauthorizedException_new(void) {
    struct UnauthorizedException * self = glitch_alloc_UnauthorizedException((struct UnauthorizedException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct UnauthorizedException * UnauthorizedException_new(char * message) {
    struct UnauthorizedException * self = glitch_alloc_UnauthorizedException((struct UnauthorizedException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct UnauthorizedException * UnauthorizedException_new(char * message, struct GlitchException innerException) {
    struct UnauthorizedException * self = glitch_alloc_UnauthorizedException((struct UnauthorizedException){0});
    free(self->__base.Type);
    self->__base.Type = (self->__base.TypeBase + glitch_strdup("unauthorized"));
    self->__base.Status = 401;
    free(self->__base.Title);
    self->__base.Title = glitch_strdup("Unauthorized");
    free(self->__base.Code);
    self->__base.Code = glitch_strdup("Unauthorized");
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ValidationException * ValidationException_new(void) {
    struct ValidationException * self = glitch_alloc_ValidationException((struct ValidationException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ValidationException * ValidationException_new(char * message) {
    struct ValidationException * self = glitch_alloc_ValidationException((struct ValidationException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ValidationException * ValidationException_new(char * errors) {
    struct ValidationException * self = glitch_alloc_ValidationException((struct ValidationException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ValidationException * ValidationException_new(char * message, struct GlitchException innerException) {
    struct ValidationException * self = glitch_alloc_ValidationException((struct ValidationException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ValidationException * ValidationException_new(char * message, struct GlitchException errors) {
    struct ValidationException * self = glitch_alloc_ValidationException((struct ValidationException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Exceptions */
static struct ValidationException * ValidationException_new(char * message, struct GlitchException innerException, struct IDictionary_string_array_string * errors) {
    struct ValidationException * self = glitch_alloc_ValidationException((struct ValidationException){0});
    self->Errors = errors;
    free(self->__base.Type);
    self->__base.Type = (self->__base.TypeBase + glitch_strdup("bad-request"));
    self->__base.Status = 400;
    free(self->__base.Title);
    self->__base.Title = glitch_strdup("Bad Request");
    free(self->__base.Code);
    self->__base.Code = glitch_strdup("ValidationError");
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.Action */
static struct ValidateImageUploadAttribute * ValidateImageUploadAttribute_new(char * parameterName) {
    struct ValidateImageUploadAttribute * self = glitch_alloc_ValidateImageUploadAttribute((struct ValidateImageUploadAttribute){0});
    free(self->_parameterName);
    self->_parameterName = glitch_strdup(parameterName);
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.Action */
static void ValidateImageUploadAttribute_OnActionExecuting(struct ValidateImageUploadAttribute * self, struct ActionExecutingContext * context) {
    struct out_var * entityObj = NULL;
    if (NULL) {
    if ((entityObj != NULL)) {
    if ((NULL == NULL)) {
    return;
    }
    struct string_array * allowedExtensions = NULL;
    if ((NULL == NULL)) {
    NULL;
    } else {
    if ((!1)) {
    NULL;
    }
    if ((NULL > 10485760)) {
    NULL;
    }
    }
    }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters */
static struct CustomExceptionFilter * CustomExceptionFilter_new(struct IWebHostEnvironment * hostEnviroment) {
    struct CustomExceptionFilter * self = glitch_alloc_CustomExceptionFilter((struct CustomExceptionFilter){0});
    self->_hostEnviroment = hostEnviroment;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters */
static void CustomExceptionFilter_OnException(struct CustomExceptionFilter * self, struct ExceptionContext * context) {
    struct ProblemDetails * problemDetails = NULL;
    struct IActionResult * result = NULL;
    if ((NULL != NULL)) {
    NULL = NULL;
    NULL = NULL;
    NULL = NULL;
    NULL = Dict_string_string_new();
    if (((ex != NULL) && (NULL != NULL))) {
    NULL = NULL;
    }
    result = NULL;
    }
    struct ExceptionContext_HttpContext_TraceIdentifier * traceId = NULL;
    if ((!NULL)) {
    NULL = traceId;
    }
    NULL = result;
    NULL = 1;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Middlewares */
static struct RateLimitMiddleware * RateLimitMiddleware_new(struct RequestDelegate * next, struct IMemoryCache * cache) {
    struct RateLimitMiddleware * self = glitch_alloc_RateLimitMiddleware((struct RateLimitMiddleware){0});
    self->_next = next;
    self->_cache = cache;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Middlewares */
static struct GlitchTask RateLimitMiddleware_InvokeAsync(struct RateLimitMiddleware * self, struct HttpContext * context) {
    struct HttpContext_Connection_RemoteIpAddress_ToString * ipAddress = NULL;
    char * cacheKey = glitch_strdup("{ipAddress}");
    struct out_var * rateLimit = NULL;
    if ((!NULL)) {
    rateLimit = glitch_alloc_RateLimit((struct RateLimit){.LastRequest = NULL, .Requests = 0});
    NULL;
    }
    struct DateTime_UtcNow * timeSinceLastRequest = (NULL - NULL);
    if ((NULL >= 1)) {
    NULL = 0;
    NULL = NULL;
    NULL;
    }
    if ((NULL >= 200)) {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Rate limit exceeded, please try again later")));
    }
    (NULL + 1);
    NULL;
    NULL;
    free(cacheKey);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger */
static struct SwaggerConfigOptions * SwaggerConfigOptions_new(struct IApiVersionDescriptionProvider * provider, struct IOptionsMonitor_SwaggerApplicationSettings * settingsMonitor) {
    struct SwaggerConfigOptions * self = glitch_alloc_SwaggerConfigOptions((struct SwaggerConfigOptions){0});
    self->_provider = provider;
    if (self->_settings) { glitch_drop_SwaggerApplicationSettings(self->_settings); free(self->_settings); }
    self->_settings = NULL;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger */
static void SwaggerConfigOptions_Configure(struct SwaggerConfigOptions * self, struct SwaggerGenOptions * options) {
    {
    for (int __glitch_foreach_i_0 = 0; __glitch_foreach_i_0 < 0; __glitch_foreach_i_0++) {
    struct var * description = /* opaque foreach over _provider_ApiVersionDescriptions */ NULL;
    NULL;
    }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger */
static struct OpenApiInfo * SwaggerConfigOptions_CreateInfoForApiVersion(struct SwaggerConfigOptions * self, struct ApiVersionDescription * description, struct SwaggerApplicationSettings * settings) {
    struct List_SwaggerVersionDescription_FirstOrDefault * versionDesc = NULL;
    struct OpenApiInfo * info = NULL;
    if (NULL) {
    NULL = (NULL + glitch_strdup("<p><font color='red'>This API version has been deprecated.</font></p>"));
    }
    struct OpenApiInfo * __glitch_return = info;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger */
static void SwaggerConfiguration_AddAndConfigureSwagger(struct SwaggerConfiguration * self, struct IServiceCollection * services, struct IConfiguration * config, char * xmlPathAndFile, int addBearerSecurity) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger */
static void SwaggerDocumentFilter_Apply(struct SwaggerDocumentFilter * self, struct OpenApiDocument * swaggerDoc, struct DocumentFilterContext * context) {
    {
    for (int __glitch_foreach_i_1 = 0; __glitch_foreach_i_1 < 0; __glitch_foreach_i_1++) {
    struct var * desc = /* opaque foreach over DocumentFilterContext_ApiDescriptions */ NULL;
    if (1) {
    NULL;
    }
    }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures */
static struct ApplicationDbContext * ApplicationDbContext_new(struct DbContextOptions_ApplicationDbContext * options) {
    struct ApplicationDbContext * self = glitch_alloc_ApplicationDbContext((struct ApplicationDbContext){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures */
static void ApplicationDbContext_OnModelCreating(struct ApplicationDbContext * self, struct ModelBuilder * modelBuilder) {
    NULL;
    SeriLogEntryConfiguration_Configure(glitch_alloc_SeriLogEntryConfiguration((struct SeriLogEntryConfiguration){}), NULL);
    BorrowingConfiguration_Configure(glitch_alloc_BorrowingConfiguration((struct BorrowingConfiguration){}), NULL);
    BookConfiguration_Configure(glitch_alloc_BookConfiguration((struct BookConfiguration){}), NULL);
    UserConfiguration_Configure(glitch_alloc_UserConfiguration((struct UserConfiguration){}), NULL);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EfStructures */
static struct ApplicationDbContext * ApplicationDbContextFactory_CreateDbContext(struct ApplicationDbContextFactory * self, struct string_array * args) {
    struct DbContextOptionsBuilder_ApplicationDbContext * optionsBuilder = NULL;
    char * connectionString = glitch_strdup("Server=.,1433;Database=Library;User Id=sa;Password=1234;TrustServerCertificate = true;");
    NULL;
    NULL;
    struct ApplicationDbContext * __glitch_return = ApplicationDbContext_new(NULL);
    free(connectionString);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures */
static struct InterceptionResult_int * ExceptionInterceptor_SavingChanges(struct ExceptionInterceptor * self, struct DbContextEventData * eventData, struct InterceptionResult_int * result) {
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    struct InterceptionResult_int * __glitch_return = NULL;
    return __glitch_return;
    } else {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("rethrow")));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures */
static struct GlitchTask_ptr ExceptionInterceptor_SavingChangesAsync(struct ExceptionInterceptor * self, struct DbContextEventData * eventData, struct InterceptionResult_int * result, struct CancellationToken * cancellationToken) {
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    struct GlitchTask_ptr __glitch_return = NULL;
    return __glitch_return;
    } else {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("rethrow")));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void Initial_BuildTargetModel(struct Initial * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void Initial_Up(struct Initial * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void Initial_Down(struct Initial * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void UpdatedUsersEmailAsIndex_BuildTargetModel(struct UpdatedUsersEmailAsIndex * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void UpdatedUsersEmailAsIndex_Up(struct UpdatedUsersEmailAsIndex * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void UpdatedUsersEmailAsIndex_Down(struct UpdatedUsersEmailAsIndex * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ImageToBookSupport_BuildTargetModel(struct ImageToBookSupport * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ImageToBookSupport_Up(struct ImageToBookSupport * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ImageToBookSupport_Down(struct ImageToBookSupport * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ReplacedIEumerablesWithICollections_BuildTargetModel(struct ReplacedIEumerablesWithICollections * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ReplacedIEumerablesWithICollections_Up(struct ReplacedIEumerablesWithICollections * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ReplacedIEumerablesWithICollections_Down(struct ReplacedIEumerablesWithICollections * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void RenamedLoanToBorrowing_BuildTargetModel(struct RenamedLoanToBorrowing * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void RenamedLoanToBorrowing_Up(struct RenamedLoanToBorrowing * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void RenamedLoanToBorrowing_Down(struct RenamedLoanToBorrowing * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddedCreditToUsersAndBooks_BuildTargetModel(struct AddedCreditToUsersAndBooks * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddedCreditToUsersAndBooks_Up(struct AddedCreditToUsersAndBooks * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddedCreditToUsersAndBooks_Down(struct AddedCreditToUsersAndBooks * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddedIsReturnedToBookBorrowing_BuildTargetModel(struct AddedIsReturnedToBookBorrowing * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddedIsReturnedToBookBorrowing_Up(struct AddedIsReturnedToBookBorrowing * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddedIsReturnedToBookBorrowing_Down(struct AddedIsReturnedToBookBorrowing * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void FixingBookBorrowingIsReturned_BuildTargetModel(struct FixingBookBorrowingIsReturned * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void FixingBookBorrowingIsReturned_Up(struct FixingBookBorrowingIsReturned * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void FixingBookBorrowingIsReturned_Down(struct FixingBookBorrowingIsReturned * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddUserImage_BuildTargetModel(struct AddUserImage * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddUserImage_Up(struct AddUserImage * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddUserImage_Down(struct AddUserImage * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ChangeBorrowingSchema_BuildTargetModel(struct ChangeBorrowingSchema * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ChangeBorrowingSchema_Up(struct ChangeBorrowingSchema * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ChangeBorrowingSchema_Down(struct ChangeBorrowingSchema * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void UpdateBorrowingsDates_BuildTargetModel(struct UpdateBorrowingsDates * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void UpdateBorrowingsDates_Up(struct UpdateBorrowingsDates * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void UpdateBorrowingsDates_Down(struct UpdateBorrowingsDates * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void MakeUserPhoneOptional_BuildTargetModel(struct MakeUserPhoneOptional * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void MakeUserPhoneOptional_Up(struct MakeUserPhoneOptional * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void MakeUserPhoneOptional_Down(struct MakeUserPhoneOptional * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void MakeUserCreditOptional_BuildTargetModel(struct MakeUserCreditOptional * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void MakeUserCreditOptional_Up(struct MakeUserCreditOptional * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void MakeUserCreditOptional_Down(struct MakeUserCreditOptional * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ChangeIsReturnedToStatusInBorrowingTable_BuildTargetModel(struct ChangeIsReturnedToStatusInBorrowingTable * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ChangeIsReturnedToStatusInBorrowingTable_Up(struct ChangeIsReturnedToStatusInBorrowingTable * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ChangeIsReturnedToStatusInBorrowingTable_Down(struct ChangeIsReturnedToStatusInBorrowingTable * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddAdminReferencesToBorrowingTable_BuildTargetModel(struct AddAdminReferencesToBorrowingTable * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddAdminReferencesToBorrowingTable_Up(struct AddAdminReferencesToBorrowingTable * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddAdminReferencesToBorrowingTable_Down(struct AddAdminReferencesToBorrowingTable * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddCreatedAtToBooksAndUsersAndBorrowings_BuildTargetModel(struct AddCreatedAtToBooksAndUsersAndBorrowings * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddCreatedAtToBooksAndUsersAndBorrowings_Up(struct AddCreatedAtToBooksAndUsersAndBorrowings * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddCreatedAtToBooksAndUsersAndBorrowings_Down(struct AddCreatedAtToBooksAndUsersAndBorrowings * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddBioToUsers_BuildTargetModel(struct AddBioToUsers * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddBioToUsers_Up(struct AddBioToUsers * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddBioToUsers_Down(struct AddBioToUsers * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddUserSex_BuildTargetModel(struct AddUserSex * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddUserSex_Up(struct AddUserSex * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddUserSex_Down(struct AddUserSex * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddGenreToBooks_BuildTargetModel(struct AddGenreToBooks * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddGenreToBooks_Up(struct AddGenreToBooks * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddGenreToBooks_Down(struct AddGenreToBooks * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddBirthDateToUsers_BuildTargetModel(struct AddBirthDateToUsers * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddBirthDateToUsers_Up(struct AddBirthDateToUsers * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void AddBirthDateToUsers_Down(struct AddBirthDateToUsers * self, struct MigrationBuilder * migrationBuilder) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.EFStructures.Migrations */
static void ApplicationDbContextModelSnapshot_BuildModel(struct ApplicationDbContextModelSnapshot * self, struct ModelBuilder * modelBuilder) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Exceptions */
static struct UnknownDatabaseException * UnknownDatabaseException_new(void) {
    struct UnknownDatabaseException * self = glitch_alloc_UnknownDatabaseException((struct UnknownDatabaseException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Exceptions */
static struct UnknownDatabaseException * UnknownDatabaseException_new(char * message) {
    struct UnknownDatabaseException * self = glitch_alloc_UnknownDatabaseException((struct UnknownDatabaseException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Exceptions */
static struct UnknownDatabaseException * UnknownDatabaseException_new(char * message, struct GlitchException innerException) {
    struct UnknownDatabaseException * self = glitch_alloc_UnknownDatabaseException((struct UnknownDatabaseException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicate(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery) {
    if ((NULL == NULL)) {
    struct Expression_Func_T_bool * __glitch_return = BuildWherePredicateForStringProperty(propertyInfo, filterQuery);
    return __glitch_return;
    } else {
    if ((NULL == NULL)) {
    struct Expression_Func_T_bool * __glitch_return = BuildWherePredicateForDateTimeProperty(propertyInfo, filterQuery);
    return __glitch_return;
    } else {
    if ((NULL == NULL)) {
    struct Expression_Func_T_bool * __glitch_return = BuildWherePredicateForIntProperty(propertyInfo, filterQuery);
    return __glitch_return;
    } else {
    if ((NULL == NULL)) {
    struct Expression_Func_T_bool * __glitch_return = BuildWherePredicateForBoolProperty(propertyInfo, filterQuery);
    return __glitch_return;
    } else {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Filtering on this property is not supported")));
    }
    }
    }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForBoolProperty(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery) {
    struct Expression_Parameter * parameter = NULL;
    struct Expression_Property * property = NULL;
    struct Expression_Constant * boolConstant = NULL;
    struct Expression_Equal * binaryExpression = NULL;
    struct Expression_Func_T_bool * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForDateTimeProperty(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery) {
    struct System_Type_GetMethod * compareToMethod = NULL;
    struct Expression_Parameter * parameter = NULL;
    struct Expression_Property * property = NULL;
    struct Expression_Func_T_bool * lambda = NULL;
    if ((NULL || 1)) {
    if (NULL) {
    char * symbol = glitch_strdup("");
    {
    for (int __glitch_foreach_i_2 = 0; __glitch_foreach_i_2 < 0; __glitch_foreach_i_2++) {
    char * op = "";
    if (1) {
    free(symbol);
    symbol = glitch_strdup(op);
    }
    }
    }
    char * dateString = glitch_strdup("");
    char * date = NULL;
    lambda = BuildWherePredicateForSingleDate(parameter, property, compareToMethod, lambda, symbol, date);
    free(date);
    free(dateString);
    free(symbol);
    }
    if (1) {
    struct string_array * dates = NULL;
    struct DateTime_Parse * date1 = NULL;
    struct DateTime_Parse * date2 = NULL;
    lambda = BuildWherePredicateForRangeOfDates(parameter, property, compareToMethod, lambda, date1, date2);
    }
    } else {
    glitch_throw(glitch_exception_from_owned(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat("Please make sure to use the correct format for date. Please follow the following instructions for guidance: \n ", "If you want to fetch records with:\n"), "1. exact date, use the following format: =2022-01-01\n"), "2. date greater than or equal to, use the following format: >=2022-01-01\n"), "3. date less than or equal to, use the following format: <=2022-01-01\n"), "4. date greater than, use the following format: >2022-01-01\n"), "5. date less than, use the following format: <2022-01-01\n"), "6. dates between two dates, use the following format: 2022-01-01~2022-01-02\n")));
    }
    struct Expression_Func_T_bool * __glitch_return = lambda;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForRangeOfDates(struct LinqHelpers * self, struct ParameterExpression * parameter, struct MemberExpression * property, struct MethodInfo * compareToMethod, struct Expression_Func_T_bool * lambda, char * date1, char * date2) {
    struct Expression_Call * compareToExpression1 = NULL;
    struct Expression_Call * compareToExpression2 = NULL;
    struct Expression_GreaterThanOrEqual * greaterThanOrEqualExpression = NULL;
    struct Expression_LessThanOrEqual * lessThanOrEqualExpression = NULL;
    struct Expression_AndAlso * binaryExpression = NULL;
    lambda = NULL;
    struct Expression_Func_T_bool * __glitch_return = lambda;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForSingleDate(struct LinqHelpers * self, struct ParameterExpression * parameter, struct MemberExpression * property, struct MethodInfo * compareToMethod, struct Expression_Func_T_bool * lambda, char * symbol, char * date) {
    struct ConstantExpression * constant = NULL;
    struct MethodCallExpression * compareToExpression = NULL;
    struct BinaryExpression * binaryExpression = NULL;
    switch (symbol) {
    case glitch_strdup("="):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    case glitch_strdup(">"):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    case glitch_strdup("<"):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    case glitch_strdup(">="):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    case glitch_strdup("<="):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    default:
    break;
    }
    struct Expression_Func_T_bool * __glitch_return = lambda;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForIntProperty(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery) {
    struct Expression_Parameter * parameter = NULL;
    struct Expression_Property * property = NULL;
    struct Expression_Func_T_bool * lambda = NULL;
    if ((NULL || 1)) {
    if (NULL) {
    char * symbol = glitch_strdup("");
    {
    for (int __glitch_foreach_i_3 = 0; __glitch_foreach_i_3 < 0; __glitch_foreach_i_3++) {
    char * op = "";
    if (1) {
    free(symbol);
    symbol = glitch_strdup(op);
    }
    }
    }
    char * intString = glitch_strdup("");
    struct int_Parse * int1 = NULL;
    lambda = BuildWherePredicateForSingleInt(parameter, property, lambda, symbol, int1);
    free(intString);
    free(symbol);
    }
    if (1) {
    struct string_array * ints = NULL;
    struct int_Parse * int1 = NULL;
    struct int_Parse * int2 = NULL;
    lambda = BuildWherePredicateForRangeOfInts(parameter, property, lambda, int1, int2);
    }
    } else {
    glitch_throw(glitch_exception_from_owned(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat("Please make sure to use the correct format for date. Please follow the following instructions for guidance: \n ", "If you want to fetch records with:\n"), "1. exact number, use the following format: =100\n"), "2. numbers greater than or equal to, use the following format: >=100\n"), "3. numbers less than or equal to, use the following format: <=100\n"), "4. numbers greater than, use the following format: >100\n"), "5. numbers less than, use the following format: <100\n"), "6. numbers between two dates, use the following format: 100~200\n")));
    }
    struct Expression_Func_T_bool * __glitch_return = lambda;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForRangeOfInts(struct LinqHelpers * self, struct ParameterExpression * parameter, struct MemberExpression * property, struct Expression_Func_T_bool * lambda, int int1, int int2) {
    struct Expression_Constant * intConstant1 = NULL;
    struct Expression_Constant * intConstant2 = NULL;
    struct Expression_GreaterThanOrEqual * greaterThanOrEqualExpression = NULL;
    struct Expression_LessThanOrEqual * lessThanOrEqualExpression = NULL;
    struct Expression_AndAlso * binaryExpression = NULL;
    lambda = NULL;
    struct Expression_Func_T_bool * __glitch_return = lambda;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForSingleInt(struct LinqHelpers * self, struct ParameterExpression * parameter, struct MemberExpression * property, struct Expression_Func_T_bool * lambda, char * symbol, int int1) {
    struct ConstantExpression * constant = NULL;
    struct BinaryExpression * binaryExpression = NULL;
    switch (symbol) {
    case glitch_strdup("="):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    case glitch_strdup(">"):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    case glitch_strdup("<"):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    case glitch_strdup(">="):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    case glitch_strdup("<="):
    binaryExpression = NULL;
    lambda = NULL;
    break;
    default:
    break;
    }
    struct Expression_Func_T_bool * __glitch_return = lambda;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_bool * LinqHelpers_BuildWherePredicateForStringProperty(struct LinqHelpers * self, struct PropertyInfo * propertyInfo, char * filterQuery) {
    struct Expression_Parameter * parameter = NULL;
    struct Expression_Property * property = NULL;
    struct System_Type_GetMethod * toLowerMethod = NULL;
    struct Expression_Call * toLowerExpression = NULL;
    struct Expression_Constant * constant = NULL;
    struct System_Type_GetMethod * containsMethod = NULL;
    struct Expression_Call * methodCall = NULL;
    struct Expression_Lambda_Compile * testCombile = NULL;
    struct Expression_Func_T_bool * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static char * LinqHelpers_BuildWherePredicateForNestedProperty(struct LinqHelpers * self, char * filterOn, char * filterQuery) {
    struct string_array * segments = NULL;
    if ((!(NULL == 2))) {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Invalid filterOn property")));
    }
    char * constructedLambdaString = glitch_strdup("x => true");
    struct Type * nestedPropertyType = NULL;
    int isCollection = (NULL && (NULL == NULL));
    if (isCollection) {
    nestedPropertyType = NULL;
    free(constructedLambdaString);
    constructedLambdaString = glitch_strdup("{segments[0]}.Any({segments[1]})");
    } else {
    free(constructedLambdaString);
    constructedLambdaString = glitch_strdup("{segments[0]}.{segments[1]}");
    }
    struct PropertyInfo * deeplyNestedPropertyInfo = NULL;
    struct Type * deeplyNestedPropertyType = NULL;
    if ((deeplyNestedPropertyType == NULL)) {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Invalid filterOn property")));
    } else {
    if ((deeplyNestedPropertyType == NULL)) {
    free(constructedLambdaString);
    constructedLambdaString = constructedLambdaString;
    } else {
    if ((deeplyNestedPropertyType == NULL)) {
    char * symbol = glitch_strdup("");
    {
    for (int __glitch_foreach_i_4 = 0; __glitch_foreach_i_4 < 0; __glitch_foreach_i_4++) {
    char * op = "";
    if (1) {
    free(symbol);
    symbol = glitch_strdup(op);
    }
    }
    }
    struct out_var * queryInt = NULL;
    if (((!NULL) && NULL)) {
    switch (symbol) {
    case glitch_strdup("="):
    free(constructedLambdaString);
    constructedLambdaString = constructedLambdaString;
    break;
    case glitch_strdup(">"):
    case glitch_strdup("<"):
    case glitch_strdup(">="):
    case glitch_strdup("<="):
    free(constructedLambdaString);
    constructedLambdaString = constructedLambdaString;
    break;
    default:
    break;
    }
    } else {
    struct out_var * queryInt1 = NULL;
    struct out_var * queryInt2 = NULL;
    if (((1 && NULL) && NULL)) {
    if (isCollection) {
    free(constructedLambdaString);
    constructedLambdaString = constructedLambdaString;
    } else {
    free(constructedLambdaString);
    constructedLambdaString = constructedLambdaString;
    }
    } else {
    glitch_throw(glitch_exception_from_owned(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat(glitch_string_concat("Please make sure to use the correct format for date. Please follow the following instructions for guidance: \n ", "If you want to fetch records with:\n"), "1. exact number, use the following format: =100\n"), "2. numbers greater than or equal to, use the following format: >=100\n"), "3. numbers less than or equal to, use the following format: <=100\n"), "4. numbers greater than, use the following format: >100\n"), "5. numbers less than, use the following format: <100\n"), "6. numbers between two numbers, use the following format: 100~200\n")));
    }
    }
    free(symbol);
    } else {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Filtering on this property is not supported")));
    }
    }
    }
    char * __glitch_return = constructedLambdaString;
    free(constructedLambdaString);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Helpers */
static struct Expression_Func_T_object * LinqHelpers_BuildOrderByFunction(struct LinqHelpers * self, struct PropertyInfo * propertyInfo) {
    struct Expression_Parameter * parameter = NULL;
    struct Expression_Property * property = NULL;
    struct Expression_Lambda * lambda = NULL;
    struct Expression_Func_T_object * __glitch_return = lambda;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static struct List_Book * SampleData_get_Books(struct SampleData * self) {
    struct List_Book * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static struct List_Author * SampleData_get_Authors(struct SampleData * self) {
    struct List_Author * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static struct List_BookAuthor * SampleData_get_BookAuthors(struct SampleData * self) {
    struct List_BookAuthor * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static struct List_Publisher * SampleData_get_Publishers(struct SampleData * self) {
    struct List_Publisher * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static struct List_BookPublisher * SampleData_get_BookPublishers(struct SampleData * self) {
    struct List_BookPublisher * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static struct List_User * SampleData_get_Users(struct SampleData * self) {
    struct List_User * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static struct List_Borrowing * SampleData_get_Borrowings(struct SampleData * self) {
    struct List_Borrowing * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static void SampleDataInitializer_DropAndCreateDatabase(struct SampleDataInitializer * self, struct ApplicationDbContext * context) {
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static void SampleDataInitializer_SeedData(struct SampleDataInitializer * self, struct ApplicationDbContext * context) {
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    ProcessInsert(context, context->Books, NULL);
    ProcessInsert(context, context->Authors, NULL);
    ProcessInsert(context, context->Publishers, NULL);
    ProcessInsert(context, context->Users, NULL);
    ProcessInsert(context, context->Borrowings, NULL);
    ProcessInsert(context, context->BookAuthors, NULL);
    ProcessInsert(context, context->BookPublishers, NULL);
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    NULL;
    glitch_throw(glitch_exception_from_owned(glitch_strdup("rethrow")));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Initialization */
static void SampleDataInitializer_InitializeData(struct SampleDataInitializer * self, struct ApplicationDbContext * context) {
    DropAndCreateDatabase(context);
    SeedData(context);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct AuthorRepo * AuthorRepo_new(struct ApplicationDbContext * context) {
    struct AuthorRepo * self = glitch_alloc_AuthorRepo((struct AuthorRepo){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct BaseRepo * BaseRepo_new(struct ApplicationDbContext * context) {
    struct BaseRepo * self = glitch_alloc_BaseRepo((struct BaseRepo){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_ptr BaseRepo_FindAsync(struct BaseRepo * self, int id) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_ptr BaseRepo_FindAsNoTrackingAsync(struct BaseRepo * self, int id) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_ptr BaseRepo_FindIgnoreQueryFiltersAsync(struct BaseRepo * self, int id) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static void BaseRepo_ExecuteParameterizedQuery(struct BaseRepo * self, char * sql, struct object_array * sqlParametersObjects) {
    NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_i32 BaseRepo_AddAsync(struct BaseRepo * self, struct T * entity, int persist) {
    NULL;
    struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result((persist ? NULL : 0));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_i32 BaseRepo_AddRangeAsync(struct BaseRepo * self, struct IEnumerable_T * entities, int persist) {
    NULL;
    struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result((persist ? NULL : 0));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_i32 BaseRepo_UpdateAsync(struct BaseRepo * self, struct T * entity, int persist) {
    NULL;
    NULL;
    NULL;
    struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result((persist ? NULL : 0));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_i32 BaseRepo_UpdateRangeAsync(struct BaseRepo * self, struct IEnumerable_T * entities, int persist) {
    NULL;
    struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result((persist ? NULL : 0));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_i32 BaseRepo_DeleteAsync__int_Array_Byte__0__bool(struct BaseRepo * self, int id, struct GlitchArray_byte timeStamp, int persist) {
    struct T * entity = NULL;
    NULL = NULL;
    struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result((persist ? NULL : 0));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_i32 BaseRepo_DeleteAsync__T_bool(struct BaseRepo * self, struct T * entity, int persist) {
    NULL;
    struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result((persist ? NULL : 0));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_i32 BaseRepo_DeleteRangeAsync(struct BaseRepo * self, struct IEnumerable_T * entities, int persist) {
    NULL;
    struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result((persist ? NULL : 0));
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct GlitchTask_i32 BaseRepo_SaveChangesAsync(struct BaseRepo * self) {
    {
    struct GlitchExceptionFrame __glitch_frame;
    int __glitch_uncaught = 0;
    glitch_exception_push(&__glitch_frame);
    if (setjmp(__glitch_frame.env) == 0) {
    struct GlitchTask_i32 __glitch_return = GlitchTask_i32_from_result(NULL);
    return __glitch_return;
    } else {
    struct GlitchException * ex = &__glitch_frame.exception;
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Record has been modified or deleted by another user")));
    }
    glitch_exception_pop(&__glitch_frame);
    if (__glitch_uncaught) { glitch_throw(__glitch_frame.exception); }
    glitch_exception_free(&__glitch_frame.exception);
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct BaseViewRepo * BaseViewRepo_new(struct ApplicationDbContext * context) {
    struct BaseViewRepo * self = glitch_alloc_BaseViewRepo((struct BaseViewRepo){0});
    if (self->Context) { glitch_drop_ApplicationDbContext(self->Context); free(self->Context); }
    self->Context = context;
    self->Table = NULL;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct IEnumerable_T * BaseViewRepo_ExecuteSqlString(struct BaseViewRepo * self, char * sqlString) {
    struct IEnumerable_T * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct IEnumerable_T * BaseViewRepo_GetAll(struct BaseViewRepo * self) {
    struct IEnumerable_T * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static struct IEnumerable_T * BaseViewRepo_GetAllIgnoreQueryFilters(struct BaseViewRepo * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber) {
    struct Table_IgnoreQueryFilters_AsQueryable * table = NULL;
    if (((!NULL) && (!NULL))) {
    struct System_Type_GetProperty * propertyInfo = NULL;
    if ((propertyInfo != NULL)) {
    struct LinqHelpers_BuildWherePredicate * predicate = NULL;
    table = NULL;
    } else {
    if (1) {
    char * constructedLambdaString = NULL;
    table = NULL;
    free(constructedLambdaString);
    } else {
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Invalid filterOn property")));
    }
    }
    }
    if ((!NULL)) {
    struct System_Type_GetProperty * propertyInfo = NULL;
    if ((propertyInfo != NULL)) {
    struct LinqHelpers_BuildOrderByFunction * function = NULL;
    table = (isAscending ? NULL : NULL);
    }
    }
    int toSkip = ((pageNumber - 1) * pageSize);
    table = NULL;
    struct IEnumerable_T * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static void BaseViewRepo_Dispose__bool(struct BaseViewRepo * self, int disposing) {
    if ((!self->disposedValue)) {
    if (disposing) {
    NULL;
    }
    self->disposedValue = 1;
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos.Base */
static void BaseViewRepo_Dispose__overload(struct BaseViewRepo * self) {
    Dispose(1);
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct BookRepo * BookRepo_new(struct ApplicationDbContext * context) {
    struct BookRepo * self = glitch_alloc_BookRepo((struct BookRepo){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct IEnumerable_Book * BookRepo_GetAllIgnoreQueryFilters(struct BookRepo * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber) {
    struct base_GetAllIgnoreQueryFilters * table = NULL;
    struct base_GetAllIgnoreQueryFilters_Select_ToList * bookIds = NULL;
    NULL;
    struct IEnumerable_Book * __glitch_return = table;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct GlitchTask_ptr BookRepo_FindAsync(struct BookRepo * self, int id) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct BorrowingRepo * BorrowingRepo_new(struct ApplicationDbContext * context) {
    struct BorrowingRepo * self = glitch_alloc_BorrowingRepo((struct BorrowingRepo){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct IEnumerable_Borrowing * BorrowingRepo_GetAllIgnoreQueryFilters(struct BorrowingRepo * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber) {
    struct base_GetAllIgnoreQueryFilters * table = NULL;
    NULL;
    struct IEnumerable_Borrowing * __glitch_return = table;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct GlitchTask_ptr BorrowingRepo_FindAsync(struct BorrowingRepo * self, int id) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct PublisherRepo * PublisherRepo_new(struct ApplicationDbContext * context) {
    struct PublisherRepo * self = glitch_alloc_PublisherRepo((struct PublisherRepo){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct UserRepo * UserRepo_new(struct ApplicationDbContext * context) {
    struct UserRepo * self = glitch_alloc_UserRepo((struct UserRepo){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Dal.Repos */
static struct GlitchTask_ptr UserRepo_FindByEmailAsync(struct UserRepo * self, char * email) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration */
static void BookConfiguration_Configure(struct BookConfiguration * self, struct EntityTypeBuilder_Book * builder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration */
static void BorrowingConfiguration_Configure(struct BorrowingConfiguration * self, struct EntityTypeBuilder_Borrowing * builder) {
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers */
static struct DateOnlyComparer * DateOnlyComparer_new(void) {
    struct DateOnlyComparer * self = glitch_alloc_DateOnlyComparer((struct DateOnlyComparer){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers */
static struct DateOnlyConverter * DateOnlyConverter_new(void) {
    struct DateOnlyConverter * self = glitch_alloc_DateOnlyConverter((struct DateOnlyConverter){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration */
static void SeriLogEntryConfiguration_Configure(struct SeriLogEntryConfiguration * self, struct EntityTypeBuilder_SeriLogEntry * builder) {
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Configuration */
static void UserConfiguration_Configure(struct UserConfiguration * self, struct EntityTypeBuilder_User * builder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities attributes=NotMapped */
static struct XElement * SeriLogEntry_get_PropertiesXml(struct SeriLogEntry * self) {
    struct XElement * __glitch_return = ((self->Properties != NULL) ? NULL : NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Profiles */
static struct AutoMapperProfile * AutoMapperProfile_new(void) {
    struct AutoMapperProfile * self = glitch_alloc_AutoMapperProfile((struct AutoMapperProfile){0});
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    CreateMap();
    CreateMap();
    CreateMap();
    NULL;
    CreateMap();
    CreateMap();
    CreateMap();
    CreateMap();
    CreateMap();
    NULL;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Profiles */
static struct IMappingExpression_TSource_TDestination * MapperExtensions_IgnoreAllMembers(struct MapperExtensions * self, struct IMappingExpression_TSource_TDestination * expr) {
    struct System_Type * destinationType = NULL;
    {
    for (int __glitch_foreach_i_5 = 0; __glitch_foreach_i_5 < 0; __glitch_foreach_i_5++) {
    struct var * property = /* opaque foreach over System_Type_GetProperties */ NULL;
    NULL;
    }
    }
    struct IMappingExpression_TSource_TDestination * __glitch_return = expr;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal.Base */
static struct BaseDalDataService * BaseDalDataService_new(struct IBaseRepo_TEntity * mainRepo, struct IAppLogging_TDataService * logger) {
    struct BaseDalDataService * self = glitch_alloc_BaseDalDataService((struct BaseDalDataService){0});
    self->_mainRepo = mainRepo;
    self->_logger = logger;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal.Base */
static struct GlitchTask_ptr BaseDalDataService_GetAllAsync(struct BaseDalDataService * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal.Base */
static struct GlitchTask_ptr BaseDalDataService_FindAsync(struct BaseDalDataService * self, int id) {
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal.Base */
static struct GlitchTask_ptr BaseDalDataService_UpdateAsync(struct BaseDalDataService * self, struct TEntity * entity, int persist) {
    NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(entity);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal.Base */
static struct GlitchTask BaseDalDataService_DeleteAsync(struct BaseDalDataService * self, struct TEntity * entity, int persist) {
    struct GlitchTask __glitch_return = GlitchTask_unsupported_from_result(NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal.Base */
static struct GlitchTask_ptr BaseDalDataService_AddAsync(struct BaseDalDataService * self, struct TEntity * entity, int persist) {
    NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(entity);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal.Base */
static void BaseDalDataService_ResetChangeTracker(struct BaseDalDataService * self) {
    NULL;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct BookDalDataService * BookDalDataService_new(struct IBookRepo * mainRepo, struct IAuthorRepo * authorRepo, struct IPublisherRepo * publisherRepo, struct IAppLogging_BookDalDataService * logger, struct IHttpContextAccessor * httpContextAccessor, struct IMapper * mapper) {
    struct BookDalDataService * self = glitch_alloc_BookDalDataService((struct BookDalDataService){0});
    self->_httpContextAccessor = httpContextAccessor;
    self->_mapper = mapper;
    if (self->_authorRepo) { glitch_drop_IAuthorRepo(self->_authorRepo); free(self->_authorRepo); }
    self->_authorRepo = authorRepo;
    if (self->_publisherRepo) { glitch_drop_IPublisherRepo(self->_publisherRepo); free(self->_publisherRepo); }
    self->_publisherRepo = publisherRepo;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr BookDalDataService_UpdateBookAndItsPublishersAndAuthorsAsync(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, int persist) {
    struct Book * existingBook = NULL;
    int timestampValid = 1;
    if ((!timestampValid)) {
    NULL;
    glitch_throw(glitch_exception_from_owned(glitch_strdup("Book has been modified or deleted by another user")));
    }
    if ((existingBook == NULL)) {
    NULL;
    glitch_throw(glitch_exception_new(""));
    }
    NULL;
    NULL;
    DeleteAuthorFromBook(editedBookDto, existingBook);
    NULL;
    DeletePublisherFromBook(editedBookDto, existingBook);
    if (persist) {
    NULL;
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(existingBook);
    if (existingBook) { glitch_drop_Book(existingBook); free(existingBook); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr BookDalDataService_AddAsync(struct BookDalDataService * self, struct Book * entity, int persist) {
    if ((entity->Image != NULL)) {
    char * imageName = NULL;
    struct Path_Combine * localImagePath = NULL;
    struct FileStream * stream = NULL;
    NULL;
    char * imageUrl = glitch_strdup("{_httpContextAccessor.HttpContext.Request.Scheme}://{_httpContextAccessor.HttpContext.Request.Host}{_httpContextAccessor.HttpContext.Request.PathBase}/StaticFiles/Images/Books/{imageName}");
    free(entity->ImagePath);
    entity->ImagePath = localImagePath;
    free(entity->ImageURL);
    entity->ImageURL = imageUrl;
    free(imageUrl);
    free(imageName);
    }
    NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(entity);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static void BookDalDataService_DeletePublisherFromBook(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, struct Book * existingBook) {
    struct IEnumerable_Publisher_ToList * existingBookPublishers = NULL;
    {
    int Id = (NULL - 1);
    while ((Id >= 0)) {
    struct Publisher * publisher = NULL;
    if ((!(editedBookDto->PublishersIds.len > 0))) {
    NULL;
    }
    (Id - 1);
    }
    if (publisher) { glitch_drop_Publisher(publisher); free(publisher); }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask BookDalDataService_AddPublisherToBook(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, struct Book * existingBook) {
    {
    for (int __glitch_foreach_i_6 = 0; __glitch_foreach_i_6 < editedBookDto->PublishersIds.len; __glitch_foreach_i_6++) {
    int publisherId = editedBookDto->PublishersIds.data[__glitch_foreach_i_6];
    if ((!1)) {
    struct Publisher * newPublisher = NULL;
    if ((newPublisher == NULL)) {
    NULL;
    glitch_throw(glitch_exception_new(""));
    }
    NULL;
    if (newPublisher) { glitch_drop_Publisher(newPublisher); free(newPublisher); }
    }
    }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static void BookDalDataService_DeleteAuthorFromBook(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, struct Book * existingBook) {
    struct IEnumerable_Author_ToList * existingBookAuthors = NULL;
    {
    int Id = (NULL - 1);
    while ((Id >= 0)) {
    struct Author * author = NULL;
    if ((!(editedBookDto->AuthorsIds.len > 0))) {
    NULL;
    }
    (Id - 1);
    }
    if (author) { glitch_drop_Author(author); free(author); }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask BookDalDataService_AddAuthorToBook(struct BookDalDataService * self, struct BookUpdateRequestDTO * editedBookDto, struct Book * existingBook) {
    {
    for (int __glitch_foreach_i_7 = 0; __glitch_foreach_i_7 < editedBookDto->AuthorsIds.len; __glitch_foreach_i_7++) {
    int authorId = editedBookDto->AuthorsIds.data[__glitch_foreach_i_7];
    if ((!1)) {
    struct Author * newAuthor = NULL;
    if ((newAuthor == NULL)) {
    NULL;
    glitch_throw(glitch_exception_new(""));
    }
    NULL;
    if (newAuthor) { glitch_drop_Author(newAuthor); free(newAuthor); }
    }
    }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct BorrowingDalDataService * BorrowingDalDataService_new(struct IBorrowingRepo * mainRepo, struct IBookRepo * bookRepo, struct IUserRepo * userRepo, struct IAppLogging_BorrowingDalDataService * logger, struct IHttpContextAccessor * httpContextAccessor, struct IMapper * mapper) {
    struct BorrowingDalDataService * self = glitch_alloc_BorrowingDalDataService((struct BorrowingDalDataService){0});
    if (self->_bookRepo) { glitch_drop_IBookRepo(self->_bookRepo); free(self->_bookRepo); }
    self->_bookRepo = bookRepo;
    if (self->_userRepo) { glitch_drop_IUserRepo(self->_userRepo); free(self->_userRepo); }
    self->_userRepo = userRepo;
    self->_mapper = mapper;
    self->_httpContextAccessor = httpContextAccessor;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr BorrowingDalDataService_GetAllAsync(struct BorrowingDalDataService * self, char * filterOn, char * filterQuery, char * sortBy, int isAscending, int pageSize, int pageNumber) {
    struct _mainRepo_GetAllIgnoreQueryFilters * borrowings = NULL;
    struct User * user = NULL;
    if ((user->UserRole != Role_Admin)) {
    borrowings = NULL;
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    if (user) { glitch_drop_User(user); free(user); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr BorrowingDalDataService_CreatePendingBorrowingAsync(struct BorrowingDalDataService * self, struct PendingBorrowingRequestDTO * userBorrowingRequest) {
    struct PendingBorrowingResponseDTO * borrowBooksResponseDTO = glitch_alloc_PendingBorrowingResponseDTO((struct PendingBorrowingResponseDTO){});
    struct List_Borrowing * pendingBorrowings = NULL;
    struct User * user = NULL;
    {
    for (int __glitch_foreach_i_8 = 0; __glitch_foreach_i_8 < userBorrowingRequest->BookIds.len; __glitch_foreach_i_8++) {
    int bookId = userBorrowingRequest->BookIds.data[__glitch_foreach_i_8];
    struct Book * bookToBorrow = NULL;
    if (((bookToBorrow == NULL) || (bookToBorrow->NumberOfAvailableCopies <= 0))) {
    NULL;
    continue;
    }
    if ((user->Credit < bookToBorrow->Credit)) {
    NULL;
    continue;
    }
    struct Borrowing * newPendingBorrowing = glitch_alloc_Borrowing((struct Borrowing){.UserId = user->__base.Id, .BookId = bookId});
    int rows = NULL;
    bookToBorrow->NumberOfAvailableCopies = (bookToBorrow->NumberOfAvailableCopies - 1);
    user->Credit = (user->Credit - bookToBorrow->Credit);
    NULL;
    NULL;
    if (newPendingBorrowing) { glitch_drop_Borrowing(newPendingBorrowing); free(newPendingBorrowing); }
    if (bookToBorrow) { glitch_drop_Book(bookToBorrow); free(bookToBorrow); }
    }
    }
    struct List_Borrowing_Select_ToList * borrowingIds = NULL;
    NULL;
    NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(borrowBooksResponseDTO);
    if (user) { glitch_drop_User(user); free(user); }
    if (borrowBooksResponseDTO) { glitch_drop_PendingBorrowingResponseDTO(borrowBooksResponseDTO); free(borrowBooksResponseDTO); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr BorrowingDalDataService_UpdateBorrowingStatusAsync(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateRequestDTO * borrowingStatusRequestDTO) {
    struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO = glitch_alloc_BorrowingStatusUpdateResponseDTO((struct BorrowingStatusUpdateResponseDTO){});
    struct List_Borrowing * borrowings = NULL;
    int isAdminAction = NULL;
    struct User * userFromToken = NULL;
    if ((isAdminAction && (userFromToken->UserRole != Role_Admin))) {
    glitch_throw(glitch_exception_new(""));
    }
    if ((borrowingStatusRequestDTO->Action != BorrowingAction_Request)) {
    {
    for (int __glitch_foreach_i_9 = 0; __glitch_foreach_i_9 < borrowingStatusRequestDTO->BorrowingIds.len; __glitch_foreach_i_9++) {
    int borrowingId = borrowingStatusRequestDTO->BorrowingIds.data[__glitch_foreach_i_9];
    struct Borrowing * borrowing = NULL;
    if ((borrowing == NULL)) {
    NULL;
    continue;
    }
    int shouldContinue = 1;
    switch (borrowingStatusRequestDTO->Action) {
    case BorrowingAction_Approve:
    shouldContinue = HandleApprove(borrowingRequestResponseDTO, userFromToken, borrowingId, borrowing, shouldContinue);
    break;
    case BorrowingAction_Reject:
    shouldContinue = HandleReject(borrowingRequestResponseDTO, userFromToken, borrowingId, borrowing, shouldContinue);
    break;
    case BorrowingAction_Confirm:
    shouldContinue = HandleConfirm(borrowingRequestResponseDTO, userFromToken, borrowingId, borrowing, shouldContinue);
    break;
    case BorrowingAction_Cancel:
    shouldContinue = HandleCancel(borrowingRequestResponseDTO, userFromToken, borrowingId, borrowing, shouldContinue);
    break;
    case BorrowingAction_Return:
    shouldContinue = HandleReturn(borrowingRequestResponseDTO, userFromToken, borrowingId, borrowing, shouldContinue);
    break;
    default:
    continue;
    }
    if ((!shouldContinue)) {
    continue;
    }
    NULL;
    NULL;
    if (borrowing) { glitch_drop_Borrowing(borrowing); free(borrowing); }
    }
    }
    }
    NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(borrowingRequestResponseDTO);
    if (userFromToken) { glitch_drop_User(userFromToken); free(userFromToken); }
    if (borrowingRequestResponseDTO) { glitch_drop_BorrowingStatusUpdateResponseDTO(borrowingRequestResponseDTO); free(borrowingRequestResponseDTO); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static int BorrowingDalDataService_HandleReturn(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue) {
    if ((borrowing->Status != BorrowingStatus_Borrowed)) {
    NULL;
    shouldContinue = 0;
    }
    if (shouldContinue) {
    ReturnBorrowing(borrowingRequestResponseDTO, userFromToken, borrowingId, borrowing);
    }
    int __glitch_return = shouldContinue;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static int BorrowingDalDataService_HandleCancel(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue) {
    if (((borrowing->UserId != userFromToken->__base.Id) && (userFromToken->UserRole != Role_Admin))) {
    NULL;
    shouldContinue = 0;
    }
    if ((shouldContinue && (borrowing->Status != BorrowingStatus_Pending))) {
    NULL;
    shouldContinue = 0;
    }
    if (shouldContinue) {
    CancelBorrowing(borrowingRequestResponseDTO, borrowingId, borrowing);
    }
    int __glitch_return = shouldContinue;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static int BorrowingDalDataService_HandleConfirm(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue) {
    if ((borrowing->UserId != userFromToken->__base.Id)) {
    NULL;
    shouldContinue = 0;
    }
    if ((shouldContinue && (borrowing->Status != BorrowingStatus_Approved))) {
    NULL;
    shouldContinue = 0;
    }
    if (shouldContinue) {
    ConfirmBorrowing(borrowing);
    }
    int __glitch_return = shouldContinue;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static int BorrowingDalDataService_HandleReject(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue) {
    if ((!((borrowing->Status == BorrowingStatus_Pending) || (borrowing->Status == BorrowingStatus_Approved)))) {
    NULL;
    shouldContinue = 0;
    }
    if (shouldContinue) {
    RejectBorrowing(borrowingRequestResponseDTO, userFromToken, borrowingId, borrowing);
    }
    int __glitch_return = shouldContinue;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static int BorrowingDalDataService_HandleApprove(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing, int shouldContinue) {
    if ((borrowing->Status != BorrowingStatus_Pending)) {
    NULL;
    shouldContinue = 0;
    }
    if (shouldContinue) {
    ApproveBorrowing(borrowingRequestResponseDTO, userFromToken, borrowingId, borrowing);
    }
    int __glitch_return = shouldContinue;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static void BorrowingDalDataService_ConfirmBorrowing(struct BorrowingDalDataService * self, struct Borrowing * borrowing) {
    borrowing->Status = BorrowingStatus_Borrowed;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static void BorrowingDalDataService_ReturnBorrowing(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing) {
    borrowing->Status = BorrowingStatus_Returned;
    borrowing->ReturnedById = userFromToken->__base.Id;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static void BorrowingDalDataService_CancelBorrowing(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, int borrowingId, struct Borrowing * borrowing) {
    borrowing->Status = BorrowingStatus_Cancelled;
    borrowing->UserNavigation->Credit = (borrowing->UserNavigation->Credit + borrowing->BookNavigation->Credit);
    borrowing->BookNavigation->NumberOfAvailableCopies = (borrowing->BookNavigation->NumberOfAvailableCopies + 1);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static void BorrowingDalDataService_RejectBorrowing(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing) {
    borrowing->Status = BorrowingStatus_Rejected;
    borrowing->RejectedById = userFromToken->__base.Id;
    borrowing->UserNavigation->Credit = (borrowing->UserNavigation->Credit + borrowing->BookNavigation->Credit);
    borrowing->BookNavigation->NumberOfAvailableCopies = (borrowing->BookNavigation->NumberOfAvailableCopies + 1);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static void BorrowingDalDataService_ApproveBorrowing(struct BorrowingDalDataService * self, struct BorrowingStatusUpdateResponseDTO * borrowingRequestResponseDTO, struct User * userFromToken, int borrowingId, struct Borrowing * borrowing) {
    borrowing->Status = BorrowingStatus_Approved;
    borrowing->ApprovedById = userFromToken->__base.Id;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr BorrowingDalDataService_GetUserFromTokenAsync(struct BorrowingDalDataService * self) {
    struct ClaimsIdentity * identity = NULL;
    int userId = NULL;
    struct User * userFromToken = NULL;
    if ((userFromToken == NULL)) {
    glitch_throw(glitch_exception_new(""));
    }
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(userFromToken);
    if (userFromToken) { glitch_drop_User(userFromToken); free(userFromToken); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct UserDalDataService * UserDalDataService_new(struct IUserRepo * mainRepo, struct IHttpContextAccessor * httpContextAccessor, struct IAppLogging_UserDalDataService * logger) {
    struct UserDalDataService * self = glitch_alloc_UserDalDataService((struct UserDalDataService){0});
    self->_httpContextAccessor = httpContextAccessor;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr UserDalDataService_LoginUserAsync(struct UserDalDataService * self, struct LoginUserRequestDTO * userDTO, struct JwtOptions * jwtOptions) {
    struct User * user = NULL;
    if ((user == NULL)) {
    NULL;
    glitch_throw(glitch_exception_new(""));
    }
    int passwordIsValid = NULL;
    if ((!passwordIsValid)) {
    NULL;
    glitch_throw(glitch_exception_new(""));
    }
    char * accessToken = NULL;
    struct AuthResponseDTO * authResponse = glitch_alloc_AuthResponseDTO((struct AuthResponseDTO){.AccessToken = accessToken, .UserId = user->__base.Id, .UserName = user->Name, .UserRole = user->UserRole, .ImageUrl = user->ImageURL});
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(authResponse);
    if (authResponse) { glitch_drop_AuthResponseDTO(authResponse); free(authResponse); }
    free(accessToken);
    if (user) { glitch_drop_User(user); free(user); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr UserDalDataService_RegisterUserAsync(struct UserDalDataService * self, struct RegisterUserRequestDTO * userDTO, struct JwtOptions * jwtOptions) {
    struct User * user = NULL;
    if ((!(user == NULL))) {
    NULL;
    glitch_throw(glitch_exception_new(""));
    }
    char * imageName = glitch_strdup("User.png");
    struct Path_Combine * localImagePath = NULL;
    char * imageUrl = glitch_strdup("{_httpContextAccessor.HttpContext.Request.Scheme}://{_httpContextAccessor.HttpContext.Request.Host}{_httpContextAccessor.HttpContext.Request.PathBase}/StaticFiles/Images/Users/{imageName}");
    if (user) { glitch_drop_User(user); free(user); }
    user = glitch_alloc_User((struct User){.Name = userDTO->Name, .Email = userDTO->Email, .PasswordHash = NULL, .UserRole = Role_User, .ImagePath = localImagePath, .ImageURL = imageUrl});
    NULL;
    NULL;
    char * accessToken = NULL;
    struct AuthResponseDTO * authResponse = glitch_alloc_AuthResponseDTO((struct AuthResponseDTO){.AccessToken = accessToken, .UserId = user->__base.Id, .UserName = user->Name, .UserRole = user->UserRole, .ImageUrl = user->ImageURL});
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(authResponse);
    if (authResponse) { glitch_drop_AuthResponseDTO(authResponse); free(authResponse); }
    free(accessToken);
    free(imageUrl);
    free(imageName);
    if (user) { glitch_drop_User(user); free(user); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr UserDalDataService_UpdateAsync(struct UserDalDataService * self, struct User * entity, int persist) {
    struct ClaimsIdentity * identity = NULL;
    int userId = NULL;
    struct User * authUser = NULL;
    int userRole = authUser->UserRole;
    struct User * userToEdit = NULL;
    if ((userToEdit == NULL)) {
    NULL;
    glitch_throw(glitch_exception_new(""));
    }
    if ((!((userRole == Role_Admin) || (userId == entity->__base.Id)))) {
    glitch_throw(glitch_exception_new(""));
    }
    if (((userRole != Role_Admin) && ((entity->Credit != NULL) || (entity->UserRole != userToEdit->UserRole)))) {
    glitch_throw(glitch_exception_new(""));
    }
    if ((entity->Image != NULL)) {
    struct Path_GetFileName * imageName = NULL;
    struct Path_Combine * localImagePath = NULL;
    char * imageUrl = glitch_strdup("{_httpContextAccessor.HttpContext.Request.Scheme}://{_httpContextAccessor.HttpContext.Request.Host}{_httpContextAccessor.HttpContext.Request.PathBase}/StaticFiles/Images/Users/{imageName}");
    struct FileStream * stream = NULL;
    NULL;
    free(userToEdit->ImagePath);
    userToEdit->ImagePath = localImagePath;
    free(userToEdit->ImageURL);
    userToEdit->ImageURL = imageUrl;
    free(imageUrl);
    }
    userToEdit->__base.TimeStamp = entity->__base.TimeStamp;
    free(userToEdit->Name);
    userToEdit->Name = entity->Name;
    userToEdit->BirthDate = ((entity->BirthDate != NULL) ? entity->BirthDate : userToEdit->BirthDate);
    free(userToEdit->Bio);
    userToEdit->Bio = ((entity->Bio != NULL) ? entity->Bio : userToEdit->Bio);
    free(userToEdit->Address);
    userToEdit->Address = ((entity->Address != NULL) ? entity->Address : userToEdit->Address);
    userToEdit->UserSex = ((entity->UserSex != NULL) ? entity->UserSex : userToEdit->UserSex);
    free(userToEdit->Email);
    userToEdit->Email = entity->Email;
    free(userToEdit->Phone);
    userToEdit->Phone = ((entity->Phone != NULL) ? entity->Phone : userToEdit->Phone);
    userToEdit->UserRole = ((userRole == Role_Admin) ? entity->UserRole : userToEdit->UserRole);
    userToEdit->Credit = (((userRole == Role_Admin) && (entity->Credit != NULL)) ? entity->Credit : userToEdit->Credit);
    NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(userToEdit);
    if (userToEdit) { glitch_drop_User(userToEdit); free(userToEdit); }
    if (authUser) { glitch_drop_User(authUser); free(authUser); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Dal */
static struct GlitchTask_ptr UserDalDataService_UpdatePasswordAsync(struct UserDalDataService * self, struct UpdatePasswordRequestDTO * userDTO, struct JwtOptions * jwtOptions) {
    struct ClaimsIdentity * identity = NULL;
    int userId = NULL;
    struct User * authUser = NULL;
    int passwordIsValid = NULL;
    if ((!passwordIsValid)) {
    glitch_throw(glitch_exception_new(""));
    }
    free(authUser->PasswordHash);
    authUser->PasswordHash = NULL;
    NULL;
    struct GlitchTask_ptr __glitch_return = GlitchTask_ptr_from_result(NULL);
    if (authUser) { glitch_drop_User(authUser); free(authUser); }
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices */
static struct IServiceCollection * DataServiceConfiguration_AddRepositories(struct DataServiceConfiguration * self, struct IServiceCollection * services) {
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    struct IServiceCollection * __glitch_return = services;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices */
static struct IServiceCollection * DataServiceConfiguration_AddDataServices(struct DataServiceConfiguration * self, struct IServiceCollection * services) {
    NULL;
    NULL;
    NULL;
    struct IServiceCollection * __glitch_return = services;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions */
static struct DataServiceException * DataServiceException_new(void) {
    struct DataServiceException * self = glitch_alloc_DataServiceException((struct DataServiceException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions */
static struct DataServiceException * DataServiceException_new(char * message) {
    struct DataServiceException * self = glitch_alloc_DataServiceException((struct DataServiceException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions */
static struct DataServiceException * DataServiceException_new(char * message, struct GlitchException innerException) {
    struct DataServiceException * self = glitch_alloc_DataServiceException((struct DataServiceException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Book */
static struct BookNotFoundException * BookNotFoundException_new(void) {
    struct BookNotFoundException * self = glitch_alloc_BookNotFoundException((struct BookNotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Book */
static struct BookNotFoundException * BookNotFoundException_new(char * message) {
    struct BookNotFoundException * self = glitch_alloc_BookNotFoundException((struct BookNotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Book */
static struct BookNotFoundException * BookNotFoundException_new(char * message, struct GlitchException innerException) {
    struct BookNotFoundException * self = glitch_alloc_BookNotFoundException((struct BookNotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Book */
static struct BookUpdateConflictException * BookUpdateConflictException_new(void) {
    struct BookUpdateConflictException * self = glitch_alloc_BookUpdateConflictException((struct BookUpdateConflictException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Book */
static struct BookUpdateConflictException * BookUpdateConflictException_new(char * message) {
    struct BookUpdateConflictException * self = glitch_alloc_BookUpdateConflictException((struct BookUpdateConflictException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Book */
static struct BookUpdateConflictException * BookUpdateConflictException_new(char * message, struct GlitchException innerException) {
    struct BookUpdateConflictException * self = glitch_alloc_BookUpdateConflictException((struct BookUpdateConflictException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Borrowing */
static struct BorrowingActionForbiddenException * BorrowingActionForbiddenException_new(void) {
    struct BorrowingActionForbiddenException * self = glitch_alloc_BorrowingActionForbiddenException((struct BorrowingActionForbiddenException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Borrowing */
static struct BorrowingActionForbiddenException * BorrowingActionForbiddenException_new(char * message) {
    struct BorrowingActionForbiddenException * self = glitch_alloc_BorrowingActionForbiddenException((struct BorrowingActionForbiddenException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Borrowing */
static struct BorrowingActionForbiddenException * BorrowingActionForbiddenException_new(char * message, struct GlitchException innerException) {
    struct BorrowingActionForbiddenException * self = glitch_alloc_BorrowingActionForbiddenException((struct BorrowingActionForbiddenException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Borrowing */
static struct BorrowingUserNotFoundException * BorrowingUserNotFoundException_new(void) {
    struct BorrowingUserNotFoundException * self = glitch_alloc_BorrowingUserNotFoundException((struct BorrowingUserNotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Borrowing */
static struct BorrowingUserNotFoundException * BorrowingUserNotFoundException_new(char * message) {
    struct BorrowingUserNotFoundException * self = glitch_alloc_BorrowingUserNotFoundException((struct BorrowingUserNotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.Borrowing */
static struct BorrowingUserNotFoundException * BorrowingUserNotFoundException_new(char * message, struct GlitchException innerException) {
    struct BorrowingUserNotFoundException * self = glitch_alloc_BorrowingUserNotFoundException((struct BorrowingUserNotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct InvalidUserException * InvalidUserException_new(void) {
    struct InvalidUserException * self = glitch_alloc_InvalidUserException((struct InvalidUserException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct InvalidUserException * InvalidUserException_new(char * message) {
    struct InvalidUserException * self = glitch_alloc_InvalidUserException((struct InvalidUserException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct InvalidUserException * InvalidUserException_new(char * message, struct GlitchException innerException) {
    struct InvalidUserException * self = glitch_alloc_InvalidUserException((struct InvalidUserException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct UserAlreadyExistException * UserAlreadyExistException_new(void) {
    struct UserAlreadyExistException * self = glitch_alloc_UserAlreadyExistException((struct UserAlreadyExistException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct UserAlreadyExistException * UserAlreadyExistException_new(char * message) {
    struct UserAlreadyExistException * self = glitch_alloc_UserAlreadyExistException((struct UserAlreadyExistException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct UserAlreadyExistException * UserAlreadyExistException_new(char * message, struct GlitchException innerException) {
    struct UserAlreadyExistException * self = glitch_alloc_UserAlreadyExistException((struct UserAlreadyExistException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct UserForbidenExcepiton * UserForbidenExcepiton_new(void) {
    struct UserForbidenExcepiton * self = glitch_alloc_UserForbidenExcepiton((struct UserForbidenExcepiton){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct UserForbidenExcepiton * UserForbidenExcepiton_new(char * message) {
    struct UserForbidenExcepiton * self = glitch_alloc_UserForbidenExcepiton((struct UserForbidenExcepiton){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct UserForbidenExcepiton * UserForbidenExcepiton_new(char * message, struct GlitchException innerException) {
    struct UserForbidenExcepiton * self = glitch_alloc_UserForbidenExcepiton((struct UserForbidenExcepiton){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct UserNotFoundException * UserNotFoundException_new(void) {
    struct UserNotFoundException * self = glitch_alloc_UserNotFoundException((struct UserNotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct UserNotFoundException * UserNotFoundException_new(char * message) {
    struct UserNotFoundException * self = glitch_alloc_UserNotFoundException((struct UserNotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Exceptions.User */
static struct UserNotFoundException * UserNotFoundException_new(char * message, struct GlitchException innerException) {
    struct UserNotFoundException * self = glitch_alloc_UserNotFoundException((struct UserNotFoundException){0});
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Helpers */
static char * JwtHelpers_GenerateJwtToken(struct JwtHelpers * self, struct User * user, struct JwtOptions * jwtOptions) {
    struct JwtSecurityTokenHandler * tokenHandler = NULL;
    struct SecurityTokenDescriptor * tokenDesciptor = NULL;
    struct JwtSecurityTokenHandler_CreateToken * securityToken = NULL;
    struct JwtSecurityTokenHandler_WriteToken * accessToken = NULL;
    char * __glitch_return = accessToken;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Helpers */
static int JwtHelpers_ValidateJwtToken(struct JwtHelpers * self, char * token) {
    glitch_throw(glitch_exception_new(""));
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Helpers */
static char * JwtHelpers_HashPassword(struct JwtHelpers * self, char * password) {
    struct BCrypt_Net_BCrypt_GenerateSalt * salt = NULL;
    char * __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.DataServices.Helpers */
static int JwtHelpers_VerifyPasswordHash(struct JwtHelpers * self, char * password, char * storedHash) {
    int __glitch_return = NULL;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static struct AppLogging * AppLogging_new(struct ILogger_T * logger) {
    struct AppLogging * self = glitch_alloc_AppLogging((struct AppLogging){0});
    self->_logger = logger;
    return self;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogWithException(struct AppLogging * self, char * memberName, char * sourceFilePath, int sourceLineNumber, struct GlitchException ex, char * message, struct Action_Exception_string_array_object * logAction) {
    struct List_IDisposable * list = NULL;
    logAction(ex, message, NULL);
    {
    for (int __glitch_foreach_i_10 = 0; __glitch_foreach_i_10 < 0; __glitch_foreach_i_10++) {
    struct var * item = /* opaque foreach over List_IDisposable */ NULL;
    NULL;
    }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogWithoutException(struct AppLogging * self, char * memberName, char * sourceFilePath, int sourceLineNumber, char * message, struct Action_string_array_object * logAction) {
    struct List_IDisposable * list = NULL;
    logAction(message, NULL);
    {
    for (int __glitch_foreach_i_11 = 0; __glitch_foreach_i_11 < 0; __glitch_foreach_i_11++) {
    struct var * item = /* opaque foreach over List_IDisposable */ NULL;
    NULL;
    }
    }
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogAppError__Exception_string_string_string_int(struct AppLogging * self, struct GlitchException exception, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber) {
    LogWithException(memberName, sourceFilePath, sourceLineNumber, exception, message, NULL);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogAppError__string_string_string_int(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber) {
    LogWithoutException(memberName, sourceFilePath, sourceLineNumber, message, NULL);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogAppCritical__Exception_string_string_string_int(struct AppLogging * self, struct GlitchException exception, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber) {
    LogWithException(memberName, sourceFilePath, sourceLineNumber, exception, message, NULL);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogAppCritical__string_string_string_int(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber) {
    LogWithoutException(memberName, sourceFilePath, sourceLineNumber, message, NULL);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogAppDebug(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber) {
    LogWithoutException(memberName, sourceFilePath, sourceLineNumber, message, NULL);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogAppTrace(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber) {
    LogWithoutException(memberName, sourceFilePath, sourceLineNumber, message, NULL);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogAppInformation(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber) {
    LogWithoutException(memberName, sourceFilePath, sourceLineNumber, message, NULL);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging */
static void AppLogging_LogAppWarning(struct AppLogging * self, char * message, char * memberName, char * sourceFilePath, int sourceLineNumber) {
    LogWithoutException(memberName, sourceFilePath, sourceLineNumber, message, NULL);
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging.Library.Services.Logging.Configuration */
static struct IServiceCollection * LoggingConfiguration_RegisterLoggingInterfaces(struct LoggingConfiguration * self, struct IServiceCollection * services) {
    NULL;
    struct IServiceCollection * __glitch_return = services;
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters.Library.Api.Swagger.Models.Library.Api.Swagger.Models.Library.Api.Swagger.Library.Api.Swagger.Library.Api.Swagger.Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Services.Logging.Library.Services.Logging.Configuration */
static void LoggingConfiguration_ConfigureSerilog(struct LoggingConfiguration * self, struct WebApplicationBuilder * builder) {
    NULL;
    struct WebApplicationBuilder_Configuration * config = NULL;
    struct WebApplicationBuilder_Configuration_GetSection_Get * settings = NULL;
    struct WebApplicationBuilder_Configuration_GetSection_Get_MSSqlServer_ConnectionStringName * connectionStringName = NULL;
    struct WebApplicationBuilder_Configuration_GetConnectionString * connectionString = NULL;
    struct WebApplicationBuilder_Configuration_GetSection_Get_MSSqlServer_TableName * tableName = NULL;
    struct WebApplicationBuilder_Configuration_GetSection_Get_MSSqlServer_Schema * schema = NULL;
    char * restrictedToMinimumLevel = NULL;
    struct out_var * logLevel = NULL;
    if ((!NULL)) {
    logLevel = NULL;
    }
    struct MSSqlServerSinkOptions * sqlOptions = NULL;
    if (NULL) {
    NULL = NULL;
    NULL = 1;
    }
    struct LoggerConfiguration_MinimumLevel_Is_Enrich_FromLogContext_Enrich_With_Enrich_WithMachineName_WriteTo_File_WriteTo_Console_WriteTo_MSSqlServer * log = NULL;
    NULL;
    free(restrictedToMinimumLevel);
}

static struct DbSetString * SetString(struct DbContext * context, char * table) {
    DbContext_EnsureNotDisposed(context);
    struct DbSetString * __glitch_return = DbSetString_new(context->ConnectionString, table);
    return __glitch_return;
}

/* metadata: namespace=Library.Api.ApiVersionSupport.Library.Api.Filters.ExceptionFilters */
int main(void) {
    struct string_array * args = NULL;
    struct WebApplication_CreateBuilder * builder = NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    struct WebApplication_CreateBuilder_Configuration_GetConnectionString * connectionString = NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    struct WebApplication_CreateBuilder_Configuration_GetSection_Get * jwtOptions = NULL;
    NULL;
    NULL;
    NULL;
    struct WebApplication_CreateBuilder_Build * app = NULL;
    if (NULL) {
    if (NULL) {
    struct WebApplication_CreateBuilder_Build_Services_CreateScope * scope = NULL;
    struct WebApplication_CreateBuilder_Build_Services_CreateScope_ServiceProvider_GetRequiredService * dbContext = NULL;
    NULL;
    }
    }
    {
    struct WebApplication_CreateBuilder_Build_Services_CreateScope * scope = NULL;
    {
    struct WebApplication_CreateBuilder_Build_Services_CreateScope_ServiceProvider_GetRequiredService * memoryCache = NULL;
    NULL;
    }
    }
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    NULL;
    return 0;
}

