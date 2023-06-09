use tinysyscall::{self, mem::MemProt, mem::MemFlags};

#[test]
fn mem_alloc_and_free() {
    const MEM_SIZE: usize = 10;

    let addr = tinysyscall::mem::mmap(
        0, MEM_SIZE,
        MemProt::READ | MemProt::WRITE,
        MemFlags::PRIVATE | MemFlags::ANONYMOUS,
        -1,
        0
    );

    assert_ne!(addr, 0);

    let ptr = &addr as *const usize as *mut u8;

    unsafe {
        let tmp_str: &str = "\nsal: memory test\n";
        tinysyscall::memcpy(ptr, tmp_str.as_bytes().as_ptr(), tmp_str.len());
        tinysyscall::file::write(
            1,
            core::mem::transmute(core::slice::from_raw_parts(ptr, tmp_str.len()))
        );
    }

    let ret = tinysyscall::mem::unmmap(addr, MEM_SIZE);

    assert_ne!(ret, -1);
    
}