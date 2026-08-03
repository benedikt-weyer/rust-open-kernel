//! Open Kernel syscall error translation.
//!
//! Syscalls return a non-negative value on success.  Failures are encoded as a
//! negative `isize` whose magnitude is a stable Open Kernel error number.
//! Unknown values intentionally remain raw OS errors and map to
//! `ErrorKind::Uncategorized`.

use core::sync::atomic::{AtomicI32, Ordering};

use crate::io;

pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const EIO: i32 = 5;
pub const EBADF: i32 = 9;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EEXIST: i32 = 17;
pub const EINVAL: i32 = 22;
pub const ENOSYS: i32 = 38;
pub const ETIMEDOUT: i32 = 110;

static ERRNO: AtomicI32 = AtomicI32::new(0);

pub fn errno() -> i32 {
    ERRNO.load(Ordering::Relaxed)
}

#[allow(dead_code)]
pub fn set_errno(error: i32) {
    ERRNO.store(error, Ordering::Relaxed);
}

#[inline]
pub fn is_interrupted(_error: i32) -> bool {
    false
}

pub fn decode_error_kind(error: i32) -> io::ErrorKind {
    match error {
        EPERM | EACCES => io::ErrorKind::PermissionDenied,
        ENOENT => io::ErrorKind::NotFound,
        EIO => io::ErrorKind::Other,
        EBADF => io::ErrorKind::InvalidInput,
        EAGAIN => io::ErrorKind::WouldBlock,
        ENOMEM => io::ErrorKind::OutOfMemory,
        EEXIST => io::ErrorKind::AlreadyExists,
        EINVAL => io::ErrorKind::InvalidInput,
        ENOSYS => io::ErrorKind::Unsupported,
        ETIMEDOUT => io::ErrorKind::TimedOut,
        _ => io::ErrorKind::Uncategorized,
    }
}

pub fn error_string(error: i32) -> String {
    match error {
        EPERM => "operation not permitted",
        ENOENT => "no such file or directory",
        EIO => "I/O error",
        EBADF => "bad file descriptor",
        EAGAIN => "operation would block",
        ENOMEM => "out of memory",
        EACCES => "permission denied",
        EEXIST => "file already exists",
        EINVAL => "invalid argument",
        ENOSYS => "function not implemented",
        ETIMEDOUT => "operation timed out",
        _ => "unknown Open Kernel error",
    }
    .into()
}
