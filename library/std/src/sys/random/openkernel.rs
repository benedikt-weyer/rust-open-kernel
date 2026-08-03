//! Entropy is provided by the kernel's virtio-rng or CPU RNG backend.

#[cold]
fn unavailable() -> ! {
    panic!("Open Kernel random entropy is unavailable")
}

pub fn fill_bytes(bytes: &mut [u8]) {
    for chunk in bytes.chunks_mut(4096) {
        let result: usize;
        // Syscall ABI: rax = number, rdi = user buffer, rsi = length.
        unsafe {
            core::arch::asm!(
                "syscall",
                inlateout("rax") 31usize => result,
                in("rdi") chunk.as_mut_ptr(),
                in("rsi") chunk.len(),
                clobber_abi("sysv64"),
            );
        }
        if result != chunk.len() {
            unavailable();
        }
    }
}

pub fn hashmap_random_keys() -> (u64, u64) {
    let mut keys = [0u64; 2];
    fill_bytes(unsafe {
        core::slice::from_raw_parts_mut(keys.as_mut_ptr().cast(), core::mem::size_of_val(&keys))
    });
    (keys[0], keys[1])
}
