use std::ffi::{c_char, c_void, CStr};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread::JoinHandle;
use std::time::Duration;

#[allow(dead_code)]
unsafe extern "C" {
    fn free(ptr: *mut c_void);
    fn calloc(count: usize, size: usize) -> *mut c_void;
}

// ---------------------------------------------------------------------------
// Thread-safe string-registry lock
// ---------------------------------------------------------------------------
// The LLVM-emitted glitch_string_allocate / _retain / _release functions
// manipulate a global linked list (@glitch_string_allocations).  They call
// these two functions as a mutex guard around every list mutation so that
// concurrent request threads do not corrupt the registry.

static STRING_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
static LIVE_ALLOCATIONS: AtomicI64 = AtomicI64::new(0);

#[no_mangle]
pub extern "C" fn GlitchLiveAllocations_Add(delta: i64) -> i64 {
    if delta >= 0 {
        LIVE_ALLOCATIONS.fetch_add(delta, Ordering::SeqCst) + delta
    } else {
        LIVE_ALLOCATIONS.fetch_sub((-delta) as i64, Ordering::SeqCst) + delta
    }
}

#[no_mangle]
pub extern "C" fn GlitchLiveAllocations_Load() -> i64 {
    LIVE_ALLOCATIONS.load(Ordering::SeqCst)
}

// SAFETY: called from LLVM IR before any access to the string registry.
// Returns an opaque token (the raw MutexGuard pointer) that the caller must
// pass back to GlitchString_Unlock.  Because the guard is heap-allocated here
// and freed in Unlock, it survives across the C ABI boundary.
#[no_mangle]
pub extern "C" fn GlitchString_Lock() -> *mut std::ffi::c_void {
    let lock = STRING_LOCK.get_or_init(|| Mutex::new(()));
    // We box the guard to give it a stable address across the FFI boundary.
    // The guard keeps the mutex locked until it is dropped in GlitchString_Unlock.
    let guard = match lock.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(), // poisoned — recover
    };
    let boxed: Box<std::sync::MutexGuard<'static, ()>> =
        unsafe { std::mem::transmute(Box::new(guard)) };
    Box::into_raw(boxed) as *mut std::ffi::c_void
}

// SAFETY: `token` must be a pointer returned by GlitchString_Lock and must
// not be used again after this call.
#[no_mangle]
pub unsafe extern "C" fn GlitchString_Unlock(token: *mut std::ffi::c_void) {
    if !token.is_null() {
        drop(Box::from_raw(
            token as *mut std::sync::MutexGuard<'static, ()>,
        ));
    }
}

#[repr(C)]
struct GlitchDelegate {
    refs: i64,
    invoke: *mut c_void,
    env: *mut c_void,
    destroy: *mut c_void,
}

#[repr(C)]
struct GlitchTask {
    completed: i32,
    result: *mut c_void,
    state: *mut GlitchTaskState,
}

struct GlitchTaskState {
    worker: Mutex<Option<JoinHandle<u64>>>,
    delegate: *mut GlitchDelegate,
}

unsafe fn glitch_delegate_release_owned(delegate: *mut GlitchDelegate) {
    if delegate.is_null() {
        return;
    }
    let refs_ptr = std::ptr::addr_of_mut!((*delegate).refs).cast::<AtomicI64>();
    let old_refs = (*refs_ptr).fetch_sub(1, Ordering::SeqCst);
    if old_refs != 1 {
        return;
    }
    let destroy = (*delegate).destroy;
    if !destroy.is_null() {
        let destroy_fn: unsafe extern "C" fn(*mut c_void) = std::mem::transmute(destroy);
        destroy_fn((*delegate).env);
    }
    free(delegate.cast::<c_void>());
    GlitchLiveAllocations_Add(-1);
}

unsafe fn task_state_finished(state: *mut GlitchTaskState) -> bool {
    if state.is_null() {
        return true;
    }
    let guard = match (*state).worker.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.as_ref().map(|worker| worker.is_finished()).unwrap_or(true)
}

unsafe fn task_wait(task: *mut GlitchTask) {
    if task.is_null() || (*task).completed != 0 {
        return;
    }
    let state = (*task).state;
    if state.is_null() {
        (*task).completed = 1;
        return;
    }
    let handle = {
        let mut guard = match (*state).worker.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.take()
    };
    let result_bits = match handle {
        Some(worker) => worker.join().unwrap_or_default(),
        None => 0,
    };
    (*task).result = result_bits as usize as *mut c_void;
    (*task).completed = 1;
    glitch_delegate_release_owned((*state).delegate);
    drop(Box::from_raw(state));
    (*task).state = std::ptr::null_mut();
    GlitchLiveAllocations_Add(-1);
}

unsafe fn task_spawn<F>(task: *mut GlitchTask, delegate: *mut GlitchDelegate, worker: F)
where
    F: FnOnce(*mut GlitchDelegate) -> u64 + Send + 'static,
{
    if task.is_null() {
        glitch_delegate_release_owned(delegate);
        return;
    }
    (*task).completed = 0;
    (*task).result = std::ptr::null_mut();
    let delegate_addr = delegate as usize;
    let handle = std::thread::spawn(move || worker(delegate_addr as *mut GlitchDelegate));
    let state = Box::new(GlitchTaskState {
        worker: Mutex::new(Some(handle)),
        delegate,
    });
    (*task).state = Box::into_raw(state);
    GlitchLiveAllocations_Add(1);
}

#[no_mangle]
pub unsafe extern "C" fn GlitchTask_RunVoid(task: *mut c_void, delegate: *mut c_void) {
    let task = task as *mut GlitchTask;
    let delegate = delegate as *mut GlitchDelegate;
    task_spawn(task, delegate, |delegate| {
        let invoke: unsafe extern "C" fn(*mut c_void) = std::mem::transmute((*delegate).invoke);
        let _ = std::panic::catch_unwind(|| invoke((*delegate).env));
        0
    });
}

#[no_mangle]
pub unsafe extern "C" fn GlitchTask_RunI32(task: *mut c_void, delegate: *mut c_void) {
    let task = task as *mut GlitchTask;
    let delegate = delegate as *mut GlitchDelegate;
    task_spawn(task, delegate, |delegate| {
        let invoke: unsafe extern "C" fn(*mut c_void) -> i32 =
            std::mem::transmute((*delegate).invoke);
        let value = std::panic::catch_unwind(|| invoke((*delegate).env)).unwrap_or_default();
        value as u32 as u64
    });
}

#[no_mangle]
pub unsafe extern "C" fn GlitchTask_RunBool(task: *mut c_void, delegate: *mut c_void) {
    let task = task as *mut GlitchTask;
    let delegate = delegate as *mut GlitchDelegate;
    task_spawn(task, delegate, |delegate| {
        let invoke: unsafe extern "C" fn(*mut c_void) -> bool =
            std::mem::transmute((*delegate).invoke);
        let value = std::panic::catch_unwind(|| invoke((*delegate).env)).unwrap_or_default();
        value as u64
    });
}

#[no_mangle]
pub unsafe extern "C" fn GlitchTask_RunI64(task: *mut c_void, delegate: *mut c_void) {
    let task = task as *mut GlitchTask;
    let delegate = delegate as *mut GlitchDelegate;
    task_spawn(task, delegate, |delegate| {
        let invoke: unsafe extern "C" fn(*mut c_void) -> i64 =
            std::mem::transmute((*delegate).invoke);
        let value = std::panic::catch_unwind(|| invoke((*delegate).env)).unwrap_or_default();
        value as u64
    });
}

#[no_mangle]
pub unsafe extern "C" fn GlitchTask_RunDouble(task: *mut c_void, delegate: *mut c_void) {
    let task = task as *mut GlitchTask;
    let delegate = delegate as *mut GlitchDelegate;
    task_spawn(task, delegate, |delegate| {
        let invoke: unsafe extern "C" fn(*mut c_void) -> f64 =
            std::mem::transmute((*delegate).invoke);
        let value = std::panic::catch_unwind(|| invoke((*delegate).env)).unwrap_or_default();
        value.to_bits()
    });
}

#[no_mangle]
pub unsafe extern "C" fn GlitchTask_RunPtr(task: *mut c_void, delegate: *mut c_void) {
    let task = task as *mut GlitchTask;
    let delegate = delegate as *mut GlitchDelegate;
    task_spawn(task, delegate, |delegate| {
        let invoke: unsafe extern "C" fn(*mut c_void) -> *mut c_void =
            std::mem::transmute((*delegate).invoke);
        let value = std::panic::catch_unwind(|| invoke((*delegate).env))
            .unwrap_or(std::ptr::null_mut());
        value as usize as u64
    });
}

#[no_mangle]
pub unsafe extern "C" fn GlitchRestHost_read_env_int(name: *const c_char, fallback: i32) -> i32 {
    if name.is_null() {
        return fallback;
    }
    let Ok(name) = CStr::from_ptr(name).to_str() else {
        return fallback;
    };
    let Some(value) = resolve_configuration_value(name) else {
        return fallback;
    };
    let Ok(parsed) = value.trim().parse::<i32>() else {
        return fallback;
    };
    parsed
}

#[no_mangle]
pub unsafe extern "C" fn GlitchRestHost_read_env_i64(name: *const c_char, fallback: i64) -> i64 {
    if name.is_null() {
        return fallback;
    }
    let Ok(name) = CStr::from_ptr(name).to_str() else {
        return fallback;
    };
    let Some(value) = resolve_configuration_value(name) else {
        return fallback;
    };
    value.trim().parse::<i64>().unwrap_or(fallback)
}

#[no_mangle]
pub unsafe extern "C" fn GlitchRestHost_read_env_bool(name: *const c_char, fallback: bool) -> bool {
    if name.is_null() {
        return fallback;
    }
    let Ok(name) = CStr::from_ptr(name).to_str() else {
        return fallback;
    };
    let Some(value) = resolve_configuration_value(name) else {
        return fallback;
    };
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => fallback,
    }
}

#[no_mangle]
pub unsafe extern "C" fn GlitchRestHost_read_env_string(
    name: *const c_char,
    fallback: *const c_char,
) -> *mut c_char {
    let fallback_text = if fallback.is_null() {
        ""
    } else {
        CStr::from_ptr(fallback).to_str().unwrap_or_default()
    };
    if name.is_null() {
        return allocate_glitch_string_from_str(fallback_text);
    }
    let Ok(name) = CStr::from_ptr(name).to_str() else {
        return allocate_glitch_string_from_str(fallback_text);
    };
    let value = resolve_configuration_value(name).unwrap_or_else(|| fallback_text.to_string());
    allocate_glitch_string_from_str(&value)
}

#[no_mangle]
pub unsafe extern "C" fn GlitchRestHost_read_connection_string(
    name: *const c_char,
    fallback: *const c_char,
) -> *mut c_char {
    let fallback_text = if fallback.is_null() {
        ""
    } else {
        CStr::from_ptr(fallback).to_str().unwrap_or_default()
    };
    if name.is_null() {
        return allocate_glitch_string_from_str(fallback_text);
    }
    let Ok(name) = CStr::from_ptr(name).to_str() else {
        return allocate_glitch_string_from_str(fallback_text);
    };
    let value = resolve_connection_string(name).unwrap_or_else(|| fallback_text.to_string());
    allocate_glitch_string_from_str(&value)
}

#[no_mangle]
pub unsafe extern "C" fn GlitchTask_Wait(task: *mut c_void) {
    task_wait(task as *mut GlitchTask);
}

#[no_mangle]
pub unsafe extern "C" fn GlitchTask_IsCompleted(task: *mut c_void) -> bool {
    let task = task as *mut GlitchTask;
    if task.is_null() {
        return true;
    }
    if (*task).completed != 0 {
        return true;
    }
    let state = (*task).state;
    if state.is_null() {
        return false;
    }
    if task_state_finished(state) {
        task_wait(task);
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn GlitchTask_Destroy(task: *mut c_void) {
    let task = task as *mut GlitchTask;
    if task.is_null() {
        return;
    }
    task_wait(task);
    if !(*task).state.is_null() {
        drop(Box::from_raw((*task).state));
        (*task).state = std::ptr::null_mut();
        GlitchLiveAllocations_Add(-1);
    }
}

type RequestHandler =
    unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char, *const c_char) -> *mut c_char;
type StringRelease = unsafe extern "C" fn(*mut c_char);

fn host_debug_enabled() -> bool {
    std::env::var_os("GLITCH_DEBUG_HOST").is_some()
}

#[no_mangle]
pub unsafe extern "C" fn GlitchRestHost_Run(
    app: *mut c_void,
    port: i32,
    max_requests: i32,
    handler: RequestHandler,
    release: StringRelease,
) {
    let _ = std::panic::catch_unwind(|| run_host(app, port, max_requests, handler, release));
    if host_debug_enabled() {
        eprintln!("Glitching HTTP host returned from run_host");
    }
}

fn run_host(
    app: *mut c_void,
    port: i32,
    max_requests: i32,
    handler: RequestHandler,
    release: StringRelease,
) {
    let port = std::env::var("GLITCH_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .or_else(|| u16::try_from(port).ok())
        .unwrap_or(5000);
    let max_requests = std::env::var("GLITCH_MAX_REQUESTS")
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(max_requests);

    let listener = match TcpListener::bind(("0.0.0.0", port)) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Glitching HTTP host failed to bind port {port}: {error}");
            return;
        }
    };
    eprintln!("Glitching HTTP host listening on port {port}");

    // Thread-safe request counter. If max_requests <= 0, run forever.
    let mut workers = Vec::new();
    let mut handled = 0i32;

    loop {
        if max_requests > 0 && handled >= max_requests {
            break;
        }

        let Ok((stream, _)) = listener.accept() else {
            continue;
        };
        handled += 1;

        // Each connection is handled on its own thread.
        // Handler and release are function pointers — they may be called
        // concurrently; the gl program's own synchronisation must protect
        // shared mutable state (e.g. the string-allocation registry).
        let app_addr = app as usize;
        workers.push(std::thread::spawn(move || {
            let mut stream = stream;
            handle_connection(app_addr as *mut c_void, &mut stream, handler, release);
        }));
    }

    for worker in workers {
        let _ = worker.join();
    }
    if host_debug_enabled() {
        eprintln!("Glitching HTTP host joined all workers");
    }
}

/// Response envelope written back by the generated handler.
/// Layout must match the LLVM struct emitted by the compiler:
///   %glitch.response = type { i32, ptr, ptr }
///   fields: status_code, body (owned ptr), content_type (static ptr)
#[allow(dead_code)]
#[repr(C)]
pub struct GlitchResponse {
    pub status_code: i32,
    /// Owned string body (must be released via `release`).
    pub body: *mut c_char,
    /// Static content-type string (not owned, never released).
    pub content_type: *const c_char,
}

fn handle_connection(
    app: *mut c_void,
    stream: &mut TcpStream,
    handler: RequestHandler,
    release: StringRelease,
) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(10)));
    let mut request = Vec::with_capacity(4096);
    let mut chunk = [0_u8; 4096];
    loop {
        let Ok(read) = stream.read(&mut chunk) else {
            return;
        };
        if read == 0 {
            return;
        }
        request.extend_from_slice(&chunk[..read]);
        if request.windows(4).any(|window| window == b"\r\n\r\n") || request.len() >= 1024 * 1024 {
            break;
        }
    }

    let header_end = request
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
        .unwrap_or(request.len());

    let (method, path, content_length, auth_header) = {
        let header = String::from_utf8_lossy(&request[..header_end]);
        let mut request_line = header.lines().next().unwrap_or_default().split_whitespace();
        let method = request_line.next().unwrap_or("GET").to_string();
        let path = request_line.next().unwrap_or("/").to_string();
        let mut content_length: usize = 0;
        let mut auth_header = String::new();
        for line in header.lines().skip(1) {
            if let Some((name, value)) = line.split_once(':') {
                let name = name.trim();
                let value = value.trim();
                if name.eq_ignore_ascii_case("content-length") {
                    content_length = value.parse().unwrap_or_default();
                } else if name.eq_ignore_ascii_case("authorization") {
                    auth_header = value.to_string();
                }
            }
        }
        (method, path, content_length, auth_header)
    };

    // Append auth header info to path as a query parameter so handlers can
    // extract it without a separate parameter slot (compatible with the current
    // single-pointer path ABI used by the endpoint thunks).
    let path_with_auth = if auth_header.is_empty() {
        path
    } else {
        let sep = if path.contains('?') { '&' } else { '?' };
        format!("{path}{sep}__auth={auth_header}")
    };

    while request.len().saturating_sub(header_end) < content_length {
        let Ok(read) = stream.read(&mut chunk) else {
            return;
        };
        if read == 0 {
            break;
        }
        request.extend_from_slice(&chunk[..read]);
    }
    let body_end = header_end.saturating_add(content_length).min(request.len());
    let body = &request[header_end..body_end];

    let method_ffi = allocate_glitch_string_from_bytes(method.as_bytes());
    let path_ffi = allocate_glitch_string_from_bytes(path_with_auth.as_bytes());
    let body_ffi = allocate_glitch_string_from_bytes(body);

    if host_debug_enabled() {
        eprintln!(
            "Glitching HTTP host handling request method={} path={} body_len={}",
            method,
            path_with_auth,
            body.len()
        );
    }

    // The handler returns a raw JSON string (or "404" / "401" sentinel).
    let response = unsafe {
        handler(
            app,
            method_ffi,
            path_ffi,
            body_ffi,
        )
    };

    if !method_ffi.is_null() {
        unsafe { release(method_ffi) };
    }
    if !path_ffi.is_null() {
        unsafe { release(path_ffi) };
    }
    if !body_ffi.is_null() {
        unsafe { release(body_ffi) };
    }

    if host_debug_enabled() {
        eprintln!("Glitching HTTP host handler returned ptr={:?}", response);
    }

    let response_bytes = if response.is_null() {
        b"null".as_slice()
    } else {
        unsafe { CStr::from_ptr(response) }.to_bytes()
    };

    if host_debug_enabled() {
        eprintln!(
            "Glitching HTTP host response bytes len={} preview={}",
            response_bytes.len(),
            String::from_utf8_lossy(&response_bytes[..response_bytes.len().min(120)])
        );
    }

    let (status, content_type) = classify_response(response_bytes);
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
        response_bytes.len()
    );
    if host_debug_enabled() {
        eprintln!("Glitching HTTP host writing status={status}");
    }
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(response_bytes);
    let _ = stream.flush();
    if !response.is_null() {
        if host_debug_enabled() {
            eprintln!("Glitching HTTP host releasing response ptr={:?}", response);
        }
        unsafe { release(response) };
    }
    if host_debug_enabled() {
        eprintln!("Glitching HTTP host finished request");
    }
}

/// Map a handler response body to an HTTP status code and content-type.
/// Handlers signal non-200 conditions by returning a sentinel JSON string
/// of the form `{"__status":NNN,...}` or one of the legacy sentinel bytes.
fn classify_response(body: &[u8]) -> (&'static str, &'static str) {
    // Legacy numeric sentinels kept for backward compat.
    if body == b"404" {
        return ("404 Not Found", "application/json");
    }
    if body == b"401" {
        return ("401 Unauthorized", "application/json");
    }
    if body == b"403" {
        return ("403 Forbidden", "application/json");
    }
    if body == b"400" {
        return ("400 Bad Request", "application/json");
    }
    if body == b"500" {
        return ("500 Internal Server Error", "application/json");
    }
    if body == b"501" {
        return ("501 Not Implemented", "application/json");
    }
    // Structured envelope: `{"__status":NNN, ...}`
    if let Some(status) = extract_status_code(body) {
        let phrase = match status {
            200 => "200 OK",
            201 => "201 Created",
            204 => "204 No Content",
            400 => "400 Bad Request",
            401 => "401 Unauthorized",
            403 => "403 Forbidden",
            404 => "404 Not Found",
            409 => "409 Conflict",
            422 => "422 Unprocessable Entity",
            500 => "500 Internal Server Error",
            _ => "200 OK",
        };
        return (phrase, "application/json");
    }
    // Default: plain text if body doesn't look like JSON.
    if body.first().copied() == Some(b'{') || body.first().copied() == Some(b'[') {
        ("200 OK", "application/json")
    } else {
        ("200 OK", "text/plain; charset=utf-8")
    }
}

fn extract_status_code(body: &[u8]) -> Option<u16> {
    let s = std::str::from_utf8(body).ok()?;
    // Look for `"__status":NNN`
    let marker = "\"__status\":";
    let start = s.find(marker)? + marker.len();
    let rest = s[start..].trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

extern "C" {
    fn malloc(size: usize) -> *mut std::ffi::c_void;
}

fn allocate_glitch_string_from_bytes(bytes: &[u8]) -> *mut c_char {
    let len = bytes.len();
    let total = match len.checked_add(1).and_then(|value| value.checked_add(16)) {
        Some(total) => total,
        None => return std::ptr::null_mut(),
    };
    let node = unsafe { malloc(total) as *mut u8 };
    if node.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        *(node as *mut i64) = 1;
        *((node.add(8)) as *mut i64) = len as i64;
        let data = node.add(16);
        if len > 0 {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), data, len);
        }
        *data.add(len) = 0;
        GlitchLiveAllocations_Add(1);
        data as *mut c_char
    }
}

fn allocate_glitch_string_from_str(text: &str) -> *mut c_char {
    allocate_glitch_string_from_bytes(text.as_bytes())
}

fn resolve_configuration_value(name: &str) -> Option<String> {
    configuration_key_candidates(name)
        .into_iter()
        .find_map(|candidate| std::env::var(candidate).ok())
}

fn resolve_connection_string(name: &str) -> Option<String> {
    let mut candidates = vec![
        format!("ConnectionStrings:{name}"),
        format!("ConnectionStrings__{name}"),
        format!("ASPNETCORE_ConnectionStrings__{name}"),
        format!("ASPNETCORE_{name}_ConnectionString"),
        format!("ASPNETCORE_{name}__ConnectionString"),
        format!("GLITCH_{name}_ConnectionString"),
    ];
    candidates.extend(configuration_key_candidates(&format!("ConnectionStrings:{name}")));
    candidates
        .into_iter()
        .find_map(|candidate| std::env::var(candidate).ok())
}

fn configuration_key_candidates(name: &str) -> Vec<String> {
    let canonical = name.replace(':', "__");
    let upper = canonical.to_ascii_uppercase();
    vec![
        name.to_string(),
        canonical.clone(),
        upper.clone(),
        format!("ASPNETCORE_{canonical}"),
        format!("ASPNETCORE_{upper}"),
        format!("GLITCH_{canonical}"),
        format!("GLITCH_{upper}"),
    ]
}

fn allocate_glitch_string_array(values: &[*mut c_char]) -> *mut std::ffi::c_void {
    let len = values.len();
    let data_size = len.max(1) * std::mem::size_of::<*mut c_char>();
    let node = unsafe { malloc(16) as *mut u8 };
    if node.is_null() {
        return std::ptr::null_mut();
    }
    let data = unsafe { malloc(data_size) as *mut *mut c_char };
    if data.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        *(node as *mut i64) = len as i64;
        *((node.add(8)) as *mut *mut c_char) = data as *mut c_char;
        for (index, value) in values.iter().copied().enumerate() {
            *data.add(index) = value;
        }
        GlitchLiveAllocations_Add(2);
        node as *mut std::ffi::c_void
    }
}

fn allocate_glitch_empty_array() -> *mut std::ffi::c_void {
    let node = unsafe { malloc(16) as *mut u8 };
    if node.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        *(node as *mut i64) = 0;
        *((node.add(8)) as *mut *mut std::ffi::c_void) = std::ptr::null_mut();
        GlitchLiveAllocations_Add(1);
        node as *mut std::ffi::c_void
    }
}

#[no_mangle]
pub unsafe extern "C" fn System_String_Substring_Native(
    value: *const c_char,
    start: i64,
) -> *mut c_char {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let bytes = CStr::from_ptr(value).to_bytes();
    let start = start.max(0) as usize;
    let start = start.min(bytes.len());
    allocate_glitch_string_from_bytes(&bytes[start..])
}

#[no_mangle]
pub unsafe extern "C" fn System_String_TrimEnd_Native(
    value: *const c_char,
    chars: *const c_char,
) -> *mut c_char {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let bytes = CStr::from_ptr(value).to_bytes();
    let trim_chars = if chars.is_null() {
        b" \t\r\n\x0b\x0c".as_slice()
    } else {
        CStr::from_ptr(chars).to_bytes()
    };
    let mut end = bytes.len();
    while end > 0 && trim_chars.contains(&bytes[end - 1]) {
        end -= 1;
    }
    allocate_glitch_string_from_bytes(&bytes[..end])
}

#[no_mangle]
pub unsafe extern "C" fn System_String_TrimStart_Native(
    value: *const c_char,
    chars: *const c_char,
) -> *mut c_char {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let bytes = CStr::from_ptr(value).to_bytes();
    let trim_chars = if chars.is_null() {
        b" \t\r\n\x0b\x0c".as_slice()
    } else {
        CStr::from_ptr(chars).to_bytes()
    };
    let mut start = 0usize;
    while start < bytes.len() && trim_chars.contains(&bytes[start]) {
        start += 1;
    }
    allocate_glitch_string_from_bytes(&bytes[start..])
}

#[no_mangle]
pub unsafe extern "C" fn System_Array_Empty_Native() -> *mut std::ffi::c_void {
    allocate_glitch_empty_array()
}

#[no_mangle]
pub unsafe extern "C" fn System_Array_Empty_Native__g1() -> *mut std::ffi::c_void {
    System_Array_Empty_Native()
}

#[no_mangle]
pub unsafe extern "C" fn System_String_ToLower_Native(value: *const c_char) -> *mut c_char {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let text = CStr::from_ptr(value).to_string_lossy().to_lowercase();
    allocate_glitch_string_from_str(&text)
}

#[no_mangle]
pub unsafe extern "C" fn System_String_ToLowerInvariant_Native(
    value: *const c_char,
) -> *mut c_char {
    System_String_ToLower_Native(value)
}

#[no_mangle]
pub unsafe extern "C" fn System_String_Replace_Native(
    value: *const c_char,
    old_value: *const c_char,
    new_value: *const c_char,
) -> *mut c_char {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let text = CStr::from_ptr(value).to_string_lossy();
    let old_text = if old_value.is_null() {
        ""
    } else {
        CStr::from_ptr(old_value).to_str().unwrap_or("")
    };
    let new_text = if new_value.is_null() {
        ""
    } else {
        CStr::from_ptr(new_value).to_str().unwrap_or("")
    };
    let replaced = text.replace(old_text, new_text);
    allocate_glitch_string_from_str(&replaced)
}

#[no_mangle]
pub unsafe extern "C" fn System_String_Trim_Native(value: *const c_char) -> *mut c_char {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let text = CStr::from_ptr(value).to_string_lossy();
    let trimmed = text.trim().to_string();
    allocate_glitch_string_from_str(&trimmed)
}

#[no_mangle]
pub unsafe extern "C" fn System_String_Split_Native(
    value: *const c_char,
    separator: *const c_char,
) -> *mut std::ffi::c_void {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let text = CStr::from_ptr(value).to_string_lossy();
    let sep_byte = if separator.is_null() {
        None
    } else {
        CStr::from_ptr(separator).to_bytes().first().copied()
    };
    let Some(sep_byte) = sep_byte else {
        let item = allocate_glitch_string_from_str(&text);
        return allocate_glitch_string_array(&[item]);
    };
    let bytes = text.as_bytes();
    let mut items = Vec::<*mut c_char>::new();
    let mut start = 0usize;
    for (index, byte) in bytes.iter().copied().enumerate() {
        if byte == sep_byte {
            items.push(allocate_glitch_string_from_bytes(&bytes[start..index]));
            start = index + 1;
        }
    }
    items.push(allocate_glitch_string_from_bytes(&bytes[start..]));
    allocate_glitch_string_array(&items)
}

#[no_mangle]
pub unsafe extern "C" fn System_String_Contains_Native(
    value: *const c_char,
    needle: *const c_char,
) -> bool {
    if value.is_null() || needle.is_null() {
        return false;
    }
    let text = CStr::from_ptr(value).to_bytes();
    let needle = CStr::from_ptr(needle).to_bytes();
    if needle.is_empty() {
        return true;
    }
    text.windows(needle.len()).any(|window| window == needle)
}

#[no_mangle]
pub unsafe extern "C" fn System_IO_File_ReadAllText(path: *const c_char) -> *mut c_char {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = CStr::from_ptr(path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let content = match std::fs::read_to_string(path_str) {
        Ok(s) => s,
        Err(_) => String::new(),
    };
    let bytes = content.as_bytes();
    let len = bytes.len();
    let ptr = malloc(len + 1) as *mut c_char;
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, ptr, len);
    *ptr.add(len) = 0;
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn System_IO_File_WriteAllText(path: *const c_char, contents: *const c_char) {
    if path.is_null() {
        return;
    }
    let c_str = CStr::from_ptr(path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return,
    };
    let content_str = if contents.is_null() {
        ""
    } else {
        match CStr::from_ptr(contents).to_str() {
            Ok(s) => s,
            Err(_) => return,
        }
    };
    let _ = std::fs::write(path_str, content_str);
}

#[no_mangle]
pub unsafe extern "C" fn System_Console_WriteLine_String(value: *const c_char) {
    if value.is_null() {
        println!("null");
    } else {
        let s = CStr::from_ptr(value);
        println!("{}", s.to_string_lossy());
    }
}

#[no_mangle]
pub extern "C" fn System_Console_WriteLine_I64(value: i64) {
    println!("{value}");
}

#[no_mangle]
pub extern "C" fn System_Console_WriteLine_Double(value: f64) {
    println!("{value}");
}

#[no_mangle]
pub extern "C" fn System_Console_WriteLine_Bool(value: bool) {
    println!("{value}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

    unsafe extern "C" fn test_handler(
        _app: *mut c_void,
        _method: *const c_char,
        path: *const c_char,
        _body: *const c_char,
    ) -> *mut c_char {
        static HEALTH: &[u8] = b"{\"status\":\"ok\"}\0";
        static HELLO: &[u8] = b"{\"message\":\"hi\"}\0";
        let path = if path.is_null() {
            ""
        } else {
            CStr::from_ptr(path).to_str().unwrap_or("")
        };
        if path.starts_with("/health") {
            HEALTH.as_ptr() as *mut c_char
        } else {
            HELLO.as_ptr() as *mut c_char
        }
    }

    unsafe extern "C" fn test_release(_value: *mut c_char) {}

    fn free_tcp_port() -> u16 {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("should reserve a free TCP port");
        let port = listener
            .local_addr()
            .expect("reserved TCP port should have an address")
            .port();
        drop(listener);
        port
    }

    fn wait_for_port(port: u16) {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        loop {
            match TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
                Ok(_) => return,
                Err(_) if std::time::Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(25))
                }
                Err(error) => panic!("host did not accept connections on {addr}: {error}"),
            }
        }
    }

    fn http_get(port: u16, path: &str) -> String {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let mut stream =
            TcpStream::connect_timeout(&addr, Duration::from_secs(5)).expect("should connect");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("should set read timeout");
        stream
            .set_write_timeout(Some(Duration::from_secs(5)))
            .expect("should set write timeout");
        let request = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
        stream
            .write_all(request.as_bytes())
            .expect("should write request");
        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .expect("should read response");
        response
    }

    #[test]
    fn host_serves_multiple_requests_and_waits_for_workers() {
        let port = free_tcp_port();
        let worker = std::thread::spawn(move || {
            run_host(std::ptr::null_mut(), port as i32, 3, test_handler, test_release);
        });

        wait_for_port(port);
        let first = http_get(port, "/health");
        let second = http_get(port, "/hello");

        assert!(first.starts_with("HTTP/1.1 200 OK"));
        assert!(first.contains("{\"status\":\"ok\"}"));
        assert!(second.starts_with("HTTP/1.1 200 OK"));
        assert!(second.contains("{\"message\":\"hi\"}"));

        worker.join().expect("host thread should exit cleanly");
    }

    static TASK_STARTED: AtomicBool = AtomicBool::new(false);

    unsafe extern "C" fn delayed_i32(_env: *mut c_void) -> i32 {
        TASK_STARTED.store(true, AtomicOrdering::SeqCst);
        std::thread::sleep(Duration::from_millis(80));
        42
    }

    #[test]
    fn task_runtime_runs_work_on_background_thread_and_waits_for_result() {
        TASK_STARTED.store(false, AtomicOrdering::SeqCst);

        let mut task = GlitchTask {
            completed: 0,
            result: std::ptr::null_mut(),
            state: std::ptr::null_mut(),
        };
        let delegate = unsafe { calloc(1, std::mem::size_of::<GlitchDelegate>()) as *mut GlitchDelegate };
        assert!(!delegate.is_null());

        unsafe {
            (*delegate).refs = 1;
            (*delegate).invoke = delayed_i32 as *mut c_void;
            (*delegate).env = std::ptr::null_mut();
            (*delegate).destroy = std::ptr::null_mut();

            GlitchTask_RunI32(
                (&mut task as *mut GlitchTask).cast::<c_void>(),
                delegate.cast::<c_void>(),
            );
        }

        let started = (0..20).any(|_| {
            if TASK_STARTED.load(AtomicOrdering::SeqCst) {
                true
            } else {
                std::thread::sleep(Duration::from_millis(5));
                false
            }
        });
        assert!(started);
        assert!(!unsafe { GlitchTask_IsCompleted((&mut task as *mut GlitchTask).cast::<c_void>()) });

        unsafe {
            GlitchTask_Wait((&mut task as *mut GlitchTask).cast::<c_void>());
        }

        assert!(unsafe { GlitchTask_IsCompleted((&mut task as *mut GlitchTask).cast::<c_void>()) });
        assert_eq!(task.result as usize as i32, 42);
        assert!(task.state.is_null());
    }
}
