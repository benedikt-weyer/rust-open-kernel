//! Allocation through the Open Kernel process heap (`brk` syscall).

use core::arch::asm;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::{MIN_ALIGN, realloc_fallback};
use crate::alloc::Layout;
use crate::ptr;

const SYS_BRK: u64 = 18;
const FAILURE: u64 = u64::MAX;

static LOCKED: AtomicBool = AtomicBool::new(false);
static NEXT: AtomicUsize = AtomicUsize::new(0);

fn lock() {
    while LOCKED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        core::hint::spin_loop();
    }
}

fn unlock() {
    LOCKED.store(false, Ordering::Release);
}

fn brk(address: usize) -> Option<usize> {
    let result: u64;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") SYS_BRK => result,
            in("rdi") address as u64,
            clobber_abi("sysv64"),
        );
    }
    (result != FAILURE).then_some(result as usize)
}

#[inline]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    lock();
    let heap_start = match NEXT.load(Ordering::Relaxed) {
        0 => match brk(0) {
            Some(value) => value,
            None => {
                unlock();
                return ptr::null_mut();
            }
        },
        value => value,
    };
    let alignment = layout.align().max(MIN_ALIGN);
    let Some(aligned_start) = heap_start
        .checked_add(alignment - 1)
        .map(|address| address & !(alignment - 1))
    else {
        unlock();
        return ptr::null_mut();
    };
    let Some(new_break) = aligned_start.checked_add(layout.size().max(1)) else {
        unlock();
        return ptr::null_mut();
    };
    if brk(new_break).is_none() {
        unlock();
        return ptr::null_mut();
    }
    NEXT.store(new_break, Ordering::Relaxed);
    unlock();
    aligned_start as *mut u8
}

#[inline]
pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    let pointer = unsafe { alloc(layout) };
    if !pointer.is_null() {
        unsafe { pointer.write_bytes(0, layout.size()) };
    }
    pointer
}

#[inline]
pub unsafe fn dealloc(_pointer: *mut u8, _layout: Layout) {
    // The current kernel heap supports growing the break only. Space is
    // reclaimed when the process exits.
}

#[inline]
pub unsafe fn realloc(pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    unsafe { realloc_fallback(pointer, layout, new_size) }
}
