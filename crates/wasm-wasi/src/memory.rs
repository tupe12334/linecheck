//! Linear-memory ABI: the host allocates/reads/frees byte ranges by pointer + length.
use std::slice;

/// Allocate `len` bytes in linear memory and return a pointer the host can write into.
#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: u32) -> u32 {
    let mut buf = vec![0u8; len as usize].into_boxed_slice();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as u32
}

/// Free a buffer previously returned by `alloc`, or either half of a `check` result.
///
/// # Safety
/// `ptr`/`len` must be a pair previously returned by `alloc`, or the pointer/length
/// decoded from a `check` return value, and must not have already been freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: u32, len: u32) {
    unsafe {
        drop(Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize));
    }
}

/// # Safety
/// `ptr`/`len` must describe a valid, initialized, readable byte range.
pub(crate) unsafe fn read_bytes<'a>(ptr: u32, len: u32) -> &'a [u8] {
    unsafe { slice::from_raw_parts(ptr as *const u8, len as usize) }
}

/// # Safety
/// `ptr`/`len` must describe a valid, initialized, readable UTF-8 byte range.
pub(crate) unsafe fn read_str<'a>(ptr: u32, len: u32) -> &'a str {
    unsafe { std::str::from_utf8(read_bytes(ptr, len)).expect("host wrote non-UTF-8 bytes") }
}

/// Leak a byte buffer into linear memory and pack it as a `(ptr << 32) | len` `u64`
/// for the host to read and later free with [`dealloc`].
pub(crate) fn leak(bytes: Box<[u8]>) -> u64 {
    let ptr = bytes.as_ptr() as u64;
    let len = bytes.len() as u64;
    std::mem::forget(bytes);
    (ptr << 32) | len
}
