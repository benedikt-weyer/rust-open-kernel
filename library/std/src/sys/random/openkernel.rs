//! Open Kernel has no entropy syscall yet.

#[cold]
fn unsupported() -> ! {
    panic!("Open Kernel does not provide a random entropy source")
}

pub fn fill_bytes(_: &mut [u8]) {
    unsupported()
}

pub fn hashmap_random_keys() -> (u64, u64) {
    unsupported()
}
