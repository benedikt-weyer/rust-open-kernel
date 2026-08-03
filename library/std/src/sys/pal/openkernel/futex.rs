//! Open Kernel futex syscall bindings.

use core::arch::asm;

use crate::sync::atomic::Atomic;
use crate::time::Duration;

/// An atomic for use as a futex that is at least 32 bits.
pub type Futex = Atomic<Primitive>;
pub type Primitive = u32;
/// An atomic for compact futex state machines.
pub type SmallFutex = Atomic<SmallPrimitive>;
pub type SmallPrimitive = u32;

const FUTEX_WAIT: u64 = 28;
const FUTEX_WAKE: u64 = 29;
const NO_TIMEOUT: u64 = u64::MAX;

fn syscall3(number: u64, first: u64, second: u64, third: u64) -> u64 {
    let result: u64;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") number => result,
            in("rdi") first,
            in("rsi") second,
            in("rdx") third,
            clobber_abi("sysv64"),
        );
    }
    result
}

pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    let timeout_ms = timeout
        .and_then(|duration| duration.as_millis().try_into().ok())
        .unwrap_or(NO_TIMEOUT);
    // EAGAIN is a successful non-blocking wake: the value changed between the
    // caller's atomic load and this syscall.
    syscall3(FUTEX_WAIT, futex.as_ptr().addr() as u64, expected as u64, timeout_ms) != u64::MAX
}

#[inline]
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    syscall3(FUTEX_WAKE, futex.as_ptr().addr() as u64, 1, 0) != 0
}

#[inline]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    let _ = syscall3(FUTEX_WAKE, futex.as_ptr().addr() as u64, u64::MAX, 0);
}
