// native/xunit_runner.c
// Simple xUnit test runner for Glang. It provides a registration API that the generated
// Glang code can call to register each test method. When `XUnit_RunAllTests` is invoked,
// the runner executes all registered tests and reports PASS/FAIL to stdout.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef void (*TestFn)();

typedef struct {
    const char* class_name;
    const char* test_name;
    TestFn      fn;
} TestEntry;

// Simple static registration table – enough for prototype purposes.
#define MAX_TESTS 1024
static TestEntry g_tests[MAX_TESTS];
static size_t    g_test_count = 0;

static size_t g_malloc_count = 0;
static size_t g_free_count = 0;

void* tracked_malloc(size_t size) {
    g_malloc_count++;
    return malloc(size);
}

void tracked_free(void* ptr) {
    if (ptr) {
        g_free_count++;
        free(ptr);
    }
}

// Exported registration function – called from Glang generated code.
// The name matches the one we will declare in System.XUnit.gl (via native interop).
void XUnit_AddTest(const char* class_name, const char* test_name, TestFn fn) {
    if (g_test_count >= MAX_TESTS) {
        fprintf(stderr, "[xUnit] Too many tests registered (max %d)\n", MAX_TESTS);
        return;
    }
    g_tests[g_test_count].class_name = class_name;
    g_tests[g_test_count].test_name  = test_name;
    g_tests[g_test_count].fn        = fn;
    ++g_test_count;
}

// Helper to run a single test and catch a failure via a longjmp style.
static void run_single(const TestEntry* entry) {
    // In a real implementation we would catch C++/C# exceptions. Here we just execute.
    entry->fn();
    printf("PASS: %s.%s\n", entry->class_name, entry->test_name);
}

// Entry point called from Glang code.
void XUnit_RunAllTests() {
    printf("[xUnit] Running %zu tests...\n", g_test_count);
    for (size_t i = 0; i < g_test_count; ++i) {
        const TestEntry* e = &g_tests[i];
        // Simple try/catch simulation: if a test throws AssertionFailedException,
        // the generated Glang code will translate that to a call to abort()/exit().
        // For this prototype we just run the test and assume it either returns
        // normally (PASS) or calls exit(1) which aborts the process – not ideal
        // but sufficient for a minimal demo.
        run_single(e);
    }
    printf("[xUnit] All tests completed.\n");
    printf("[xUnit] Memory tracking: %zu mallocs, %zu frees. %s\n", 
           g_malloc_count, g_free_count, 
           (g_malloc_count == g_free_count) ? "Clean." : "LEAKS DETECTED!");
}
// LeakDetector native API ----------------------------------------------------
void Native_LeakDetector_Reset() {
    g_malloc_count = 0;
    g_free_count = 0;
}
unsigned int Native_LeakDetector_GetMallocCount() {
    return (unsigned int)g_malloc_count;
}
unsigned int Native_LeakDetector_GetFreeCount() {
    return (unsigned int)g_free_count;
}
bool Native_LeakDetector_HasLeaks() {
    return g_malloc_count != g_free_count;
}
