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

/* metadata: namespace=Library.Models.DTO */
enum BorrowingAction {
    BorrowingAction_Request,
    BorrowingAction_Confirm,
    BorrowingAction_Cancel,
    BorrowingAction_Approve,
    BorrowingAction_Reject,
    BorrowingAction_Return,
};

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

/* metadata: namespace=Library.Models.Entities */
enum BorrowingStatus {
    BorrowingStatus_Pending,
    BorrowingStatus_Cancelled,
    BorrowingStatus_Approved,
    BorrowingStatus_Rejected,
    BorrowingStatus_Borrowed,
    BorrowingStatus_Returned,
};

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Entities */
enum Role {
    Role_Admin,
    Role_User,
};

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Entities */
enum Sex {
    Sex_Male,
    Sex_Female,
};

/* metadata: namespace=Library.Models.DTO */
struct AuthorCreateRequestDTO {
    char * Name;
};

static struct AuthorCreateRequestDTO * glitch_alloc_AuthorCreateRequestDTO(struct AuthorCreateRequestDTO value) { struct AuthorCreateRequestDTO *ptr = malloc(sizeof(struct AuthorCreateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AuthorCreateRequestDTO(struct AuthorCreateRequestDTO *value) {
    if (!value) { return; }
    free(value->Name);
}

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO.Base */
struct BaseDTO {
    int Id;
    struct GlitchArray_byte TimeStamp;
};

static struct BaseDTO * glitch_alloc_BaseDTO(struct BaseDTO value) { struct BaseDTO *ptr = malloc(sizeof(struct BaseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BaseDTO(struct BaseDTO *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.DTO.Base */
/* interface IImageUploadable */

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
struct BorrowingStatusUpdateRequestDTO {
    int Action;
    struct List_int BorrowingIds;
};

static struct BorrowingStatusUpdateRequestDTO * glitch_alloc_BorrowingStatusUpdateRequestDTO(struct BorrowingStatusUpdateRequestDTO value) { struct BorrowingStatusUpdateRequestDTO *ptr = malloc(sizeof(struct BorrowingStatusUpdateRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingStatusUpdateRequestDTO(struct BorrowingStatusUpdateRequestDTO *value) {
    if (!value) { return; }
    List_int_free(&value->BorrowingIds);
}

/* metadata: namespace=Library.Models.DTO */
struct BorrowingStatusUpdateResponseDTO {
    struct List_BorrowingResponseDTO * Success;
    struct List_BorrowingRequestsErrors * Errors;
};

static struct BorrowingStatusUpdateResponseDTO * glitch_alloc_BorrowingStatusUpdateResponseDTO(struct BorrowingStatusUpdateResponseDTO value) { struct BorrowingStatusUpdateResponseDTO *ptr = malloc(sizeof(struct BorrowingStatusUpdateResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingStatusUpdateResponseDTO(struct BorrowingStatusUpdateResponseDTO *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.DTO */
struct BorrowingRequestsErrors {
    int BorrowingId;
    char * Message;
};

static struct BorrowingRequestsErrors * glitch_alloc_BorrowingRequestsErrors(struct BorrowingRequestsErrors value) { struct BorrowingRequestsErrors *ptr = malloc(sizeof(struct BorrowingRequestsErrors)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingRequestsErrors(struct BorrowingRequestsErrors *value) {
    if (!value) { return; }
    free(value->Message);
}

/* metadata: namespace=Library.Models.DTO */
struct PendingBorrowingRequestDTO {
    struct List_int BookIds;
};

static struct PendingBorrowingRequestDTO * glitch_alloc_PendingBorrowingRequestDTO(struct PendingBorrowingRequestDTO value) { struct PendingBorrowingRequestDTO *ptr = malloc(sizeof(struct PendingBorrowingRequestDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PendingBorrowingRequestDTO(struct PendingBorrowingRequestDTO *value) {
    if (!value) { return; }
    List_int_free(&value->BookIds);
}

/* metadata: namespace=Library.Models.DTO */
struct PendingBorrowingResponseDTO {
    struct List_BorrowingResponseDTO * Success;
    struct List_BorrowBooksErrors * Errors;
};

static struct PendingBorrowingResponseDTO * glitch_alloc_PendingBorrowingResponseDTO(struct PendingBorrowingResponseDTO value) { struct PendingBorrowingResponseDTO *ptr = malloc(sizeof(struct PendingBorrowingResponseDTO)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_PendingBorrowingResponseDTO(struct PendingBorrowingResponseDTO *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.DTO */
struct BorrowBooksErrors {
    int BookId;
    char * Message;
};

static struct BorrowBooksErrors * glitch_alloc_BorrowBooksErrors(struct BorrowBooksErrors value) { struct BorrowBooksErrors *ptr = malloc(sizeof(struct BorrowBooksErrors)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowBooksErrors(struct BorrowBooksErrors *value) {
    if (!value) { return; }
    free(value->Message);
}

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.DTO */
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

/* metadata: namespace=Library.Models.Entities */
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

/* metadata: namespace=Library.Models.Entities.Base */
struct BaseEntity {
    int Id;
    struct GlitchArray_byte TimeStamp;
};

static struct BaseEntity * glitch_alloc_BaseEntity(struct BaseEntity value) { struct BaseEntity *ptr = malloc(sizeof(struct BaseEntity)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BaseEntity(struct BaseEntity *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.Entities attributes=EntityTypeConfiguration(<expr>) */
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

/* metadata: namespace=Library.Models.Entities */
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

/* metadata: namespace=Library.Models.Entities */
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

/* metadata: namespace=Library.Models.Entities attributes=EntityTypeConfiguration(<expr>) */
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

/* metadata: namespace=Library.Models.Entities.Configuration */
struct BookConfiguration {
};

static struct BookConfiguration * glitch_alloc_BookConfiguration(struct BookConfiguration value) { struct BookConfiguration *ptr = malloc(sizeof(struct BookConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BookConfiguration(struct BookConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.Entities.Configuration */
struct BorrowingConfiguration {
};

static struct BorrowingConfiguration * glitch_alloc_BorrowingConfiguration(struct BorrowingConfiguration value) { struct BorrowingConfiguration *ptr = malloc(sizeof(struct BorrowingConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_BorrowingConfiguration(struct BorrowingConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers */
struct DateOnlyComparer {
};

static struct DateOnlyComparer * glitch_alloc_DateOnlyComparer(struct DateOnlyComparer value) { struct DateOnlyComparer *ptr = malloc(sizeof(struct DateOnlyComparer)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_DateOnlyComparer(struct DateOnlyComparer *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers */
struct DateOnlyConverter {
};

static struct DateOnlyConverter * glitch_alloc_DateOnlyConverter(struct DateOnlyConverter value) { struct DateOnlyConverter *ptr = malloc(sizeof(struct DateOnlyConverter)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_DateOnlyConverter(struct DateOnlyConverter *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration */
struct SeriLogEntryConfiguration {
};

static struct SeriLogEntryConfiguration * glitch_alloc_SeriLogEntryConfiguration(struct SeriLogEntryConfiguration value) { struct SeriLogEntryConfiguration *ptr = malloc(sizeof(struct SeriLogEntryConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_SeriLogEntryConfiguration(struct SeriLogEntryConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Configuration */
struct UserConfiguration {
};

static struct UserConfiguration * glitch_alloc_UserConfiguration(struct UserConfiguration value) { struct UserConfiguration *ptr = malloc(sizeof(struct UserConfiguration)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_UserConfiguration(struct UserConfiguration *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities */
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

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities attributes=Table("SeriLogs", Schema = "Logging"), EntityTypeConfiguration(<expr>) */
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

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Entities attributes=EntityTypeConfiguration(<expr>) */
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

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Options.JwtOptions */
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

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Profiles */
struct AutoMapperProfile {
};

static struct AutoMapperProfile * glitch_alloc_AutoMapperProfile(struct AutoMapperProfile value) { struct AutoMapperProfile *ptr = malloc(sizeof(struct AutoMapperProfile)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_AutoMapperProfile(struct AutoMapperProfile *value) {
    if (!value) { return; }
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Profiles */
struct MapperExtensions {
};

static struct MapperExtensions * glitch_alloc_MapperExtensions(struct MapperExtensions value) { struct MapperExtensions *ptr = malloc(sizeof(struct MapperExtensions)); if (!ptr) { abort(); } *ptr = value; return ptr; }

static void glitch_drop_MapperExtensions(struct MapperExtensions *value) {
    if (!value) { return; }
}

static void BookConfiguration_Configure(struct BookConfiguration * self, struct EntityTypeBuilder_Book * builder);
static void BorrowingConfiguration_Configure(struct BorrowingConfiguration * self, struct EntityTypeBuilder_Borrowing * builder);
static struct DateOnlyComparer * DateOnlyComparer_new(void);
static struct DateOnlyConverter * DateOnlyConverter_new(void);
static void SeriLogEntryConfiguration_Configure(struct SeriLogEntryConfiguration * self, struct EntityTypeBuilder_SeriLogEntry * builder);
static void UserConfiguration_Configure(struct UserConfiguration * self, struct EntityTypeBuilder_User * builder);
static struct XElement * SeriLogEntry_get_PropertiesXml(struct SeriLogEntry * self);
static struct AutoMapperProfile * AutoMapperProfile_new(void);
static struct IMappingExpression_TSource_TDestination * MapperExtensions_IgnoreAllMembers(struct MapperExtensions * self, struct IMappingExpression_TSource_TDestination * expr);

/* metadata: namespace=Library.Models.Entities.Configuration */
static void BookConfiguration_Configure(struct BookConfiguration * self, struct EntityTypeBuilder_Book * builder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Models.Entities.Configuration */
static void BorrowingConfiguration_Configure(struct BorrowingConfiguration * self, struct EntityTypeBuilder_Borrowing * builder) {
    NULL;
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers */
static struct DateOnlyComparer * DateOnlyComparer_new(void) {
    struct DateOnlyComparer * self = glitch_alloc_DateOnlyComparer((struct DateOnlyComparer){0});
    return self;
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers */
static struct DateOnlyConverter * DateOnlyConverter_new(void) {
    struct DateOnlyConverter * self = glitch_alloc_DateOnlyConverter((struct DateOnlyConverter){0});
    return self;
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration */
static void SeriLogEntryConfiguration_Configure(struct SeriLogEntryConfiguration * self, struct EntityTypeBuilder_SeriLogEntry * builder) {
    NULL;
    NULL;
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Configuration */
static void UserConfiguration_Configure(struct UserConfiguration * self, struct EntityTypeBuilder_User * builder) {
    NULL;
    NULL;
    NULL;
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities attributes=NotMapped */
static struct XElement * SeriLogEntry_get_PropertiesXml(struct SeriLogEntry * self) {
    struct XElement * __glitch_return = ((self->Properties != NULL) ? NULL : NULL);
    return __glitch_return;
}

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Profiles */
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

/* metadata: namespace=Library.Models.Entities.Configuration.Helpers.Library.Models.Entities.Configuration.Library.Models.Entities.Library.Models.Profiles */
static struct IMappingExpression_TSource_TDestination * MapperExtensions_IgnoreAllMembers(struct MapperExtensions * self, struct IMappingExpression_TSource_TDestination * expr) {
    struct System_Type * destinationType = NULL;
    {
    for (int __glitch_foreach_i_0 = 0; __glitch_foreach_i_0 < 0; __glitch_foreach_i_0++) {
    struct var * property = /* opaque foreach over System_Type_GetProperties */ NULL;
    NULL;
    }
    }
    struct IMappingExpression_TSource_TDestination * __glitch_return = expr;
    return __glitch_return;
}

