//! Open Kernel platform abstraction layer.
//!
//! This module is deliberately independent of libc.  Its syscall veneer is
//! supplied by the Open Kernel userspace runtime; subsystems are enabled here
//! only after their ABI is implemented and tested.

#![deny(unsafe_op_in_unsafe_fn)]

use crate::io;

pub fn unsupported<T>() -> io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> io::Error {
    io::Error::UNSUPPORTED_PLATFORM
}

pub fn abort_internal() -> ! {
    unsafe {
        core::arch::asm!("ud2", options(noreturn, nomem, nostack));
    }
}

// SAFETY: the Open Kernel initial stack supplies argc and argv once per
// process, before Rust code begins executing.
pub unsafe fn init(argc: isize, argv: *const *const u8, _sigpipe: u8) {
    unsafe {
        crate::sys::args::init(argc, argv);
    }
}

// SAFETY: called by the Rust runtime during process teardown.
pub unsafe fn cleanup() {}
