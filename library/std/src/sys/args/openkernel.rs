//! Open Kernel process-start arguments.

pub use super::common::Args;

use core::sync::atomic::{AtomicIsize, AtomicPtr, Ordering};

use crate::ffi::{CStr, OsString};

static ARGC: AtomicIsize = AtomicIsize::new(0);
static ARGV: AtomicPtr<*const u8> = AtomicPtr::new(core::ptr::null_mut());

/// Stores the immutable process-start argument vector. Each call to `args`
/// copies bytes into owned `OsString` values.
pub unsafe fn init(argc: isize, argv: *const *const u8) {
    ARGC.store(argc, Ordering::Relaxed);
    ARGV.store(argv.cast_mut(), Ordering::Relaxed);
}

pub fn args() -> Args {
    let argc = ARGC.load(Ordering::Relaxed).max(0) as usize;
    let argv = ARGV.load(Ordering::Relaxed);
    if argv.is_null() {
        return Args::new(vec![]);
    }

    let mut values = Vec::with_capacity(argc);
    for index in 0..argc {
        let pointer = unsafe { argv.add(index).read() };
        if pointer.is_null() {
            break;
        }
        let bytes = unsafe { CStr::from_ptr(pointer.cast()).to_bytes().to_vec() };
        values.push(unsafe { OsString::from_encoded_bytes_unchecked(bytes) });
    }
    Args::new(values)
}
