use std::ffi::{c_char, c_void, CStr};
use std::io::{self, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

type DelegateFn = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

#[derive(Clone)]
struct TestEntry {
    class_name: String,
    test_name: String,
    fn_ptr: DelegateFn,
    env_ptr: *mut c_void,
    delegate_ptr: *mut c_void,
}

unsafe impl Send for TestEntry {}

static TESTS: OnceLock<Mutex<Vec<TestEntry>>> = OnceLock::new();
static MALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
static FREE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
struct DelegateAbi {
    ref_count: i64,
    fn_ptr: DelegateFn,
    env_ptr: *mut c_void,
    destroy_ptr: *mut c_void,
}

fn tests() -> &'static Mutex<Vec<TestEntry>> {
    TESTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn c_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
    }
}

fn log_line(args: std::fmt::Arguments<'_>) {
    let mut stderr = io::stderr().lock();
    let _ = stderr.write_fmt(args);
    let _ = stderr.write_all(b"\n");
    let _ = stderr.flush();
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn XUnit_AddTest(
    class_name: *const c_char,
    test_name: *const c_char,
    test: *const c_void,
) {
    log_line(format_args!(
        "[xUnit] AddTest entry class={class_name:p} test={test_name:p} delegate={test:p}"
    ));
    let delegate = test as *const DelegateAbi;
    let delegate = unsafe { delegate.as_ref() };
    let (fn_ptr, env_ptr) = match delegate {
        Some(delegate) => (delegate.fn_ptr, delegate.env_ptr),
        None => (
            dummy_delegate as DelegateFn,
            std::ptr::null_mut(),
        ),
    };
    log_line(format_args!(
        "[xUnit] AddTest delegate fn={fn_ptr:p} env={env_ptr:p}"
    ));
    let entry = TestEntry {
        class_name: c_string(class_name),
        test_name: c_string(test_name),
        fn_ptr,
        env_ptr,
        delegate_ptr: test as *mut c_void,
    };
    log_line(format_args!(
        "[xUnit] AddTest strings {}.{}",
        entry.class_name, entry.test_name
    ));
    log_line(format_args!(
        "[xUnit] registered {}.{}",
        entry.class_name, entry.test_name
    ));
    tests().lock().unwrap().push(entry);
}

#[no_mangle]
pub unsafe extern "C" fn XUnit_RunAllTests() {
    let snapshot = tests().lock().unwrap().clone();
    log_line(format_args!("[xUnit] Running {} tests...", snapshot.len()));

    let mut failed = 0usize;
    for entry in &snapshot {
        log_line(format_args!(
            "[xUnit] START {}.{}",
            entry.class_name, entry.test_name
        ));
        let result = catch_unwind(AssertUnwindSafe(|| unsafe {
            (entry.fn_ptr)(entry.env_ptr);
        }));
        if result.is_ok() {
            log_line(format_args!("[xUnit] PASS"));
        } else {
            failed += 1;
            log_line(format_args!("[xUnit] FAIL"));
        }
    }
    tests().lock().unwrap().clear();
    for entry in &snapshot {
        if !entry.delegate_ptr.is_null() {
            unsafe { release_registered_delegate(entry.delegate_ptr) };
        }
    }

    let allocated = MALLOC_COUNT.load(Ordering::SeqCst);
    let freed = FREE_COUNT.load(Ordering::SeqCst);
    log_line(format_args!(
        "[xUnit] Completed: {} passed, {} failed.",
        snapshot.len().saturating_sub(failed),
        failed
    ));
    log_line(format_args!(
        "[xUnit] Memory tracking: {allocated} mallocs, {freed} frees. {}",
        if allocated == freed {
            "Clean."
        } else {
            "LEAKS DETECTED!"
        }
    ));
}

#[no_mangle]
pub extern "C" fn Native_LeakDetector_Reset() {
    MALLOC_COUNT.store(0, Ordering::SeqCst);
    FREE_COUNT.store(0, Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn Native_LeakDetector_GetMallocCount() -> u32 {
    MALLOC_COUNT.load(Ordering::SeqCst) as u32
}

#[no_mangle]
pub extern "C" fn Native_LeakDetector_GetFreeCount() -> u32 {
    FREE_COUNT.load(Ordering::SeqCst) as u32
}

#[no_mangle]
pub extern "C" fn Native_LeakDetector_HasLeaks() -> bool {
    MALLOC_COUNT.load(Ordering::SeqCst) != FREE_COUNT.load(Ordering::SeqCst)
}

#[no_mangle]
pub unsafe extern "C" fn tracked_malloc(size: i32) -> *mut c_void {
    MALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
    let size = size.max(0) as usize;
    malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn tracked_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        FREE_COUNT.fetch_add(1, Ordering::SeqCst);
        free(ptr);
    }
    let _ = io::stderr().flush();
}

unsafe extern "C" fn dummy_delegate(_env: *mut c_void) -> *mut c_void {
    std::ptr::null_mut()
}

#[repr(C)]
struct OwnedDelegateAbi {
    ref_count: AtomicI64,
    fn_ptr: DelegateFn,
    env_ptr: *mut c_void,
    destroy_ptr: Option<unsafe extern "C" fn(*mut c_void)>,
}

unsafe fn release_registered_delegate(value: *mut c_void) {
    if value.is_null() {
        return;
    }
    let delegate = value as *mut OwnedDelegateAbi;
    let refs = (*delegate).ref_count.load(Ordering::SeqCst);
    if refs >= 1_000_000 {
        return;
    }
    let previous = (*delegate).ref_count.fetch_sub(1, Ordering::SeqCst);
    if previous != 1 {
        return;
    }
    if let Some(destroy) = (*delegate).destroy_ptr {
        destroy((*delegate).env_ptr);
    }
    free(value);
}
