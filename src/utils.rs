
pub unsafe fn memset<T>(byte: u8) -> T {
    let mut value: T = core::mem::zeroed();
    let ptr = &mut value as *mut T;
    let len = core::mem::size_of::<T>();
    core::ptr::write_bytes(ptr as *mut u8, byte, len);
    value
}

pub unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
}

