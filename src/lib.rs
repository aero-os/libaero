#[derive(serde::Deserialize)]
// TOOD: Why did old andy add this repr(C)?
#[repr(C)]
pub struct Mapping {
    pub start: usize,
    pub end: usize,
    pub protection: usize,
    pub flags: usize,
}

#[cfg(target_arch = "x86_64")]
fn get_sp() -> usize {
    use core::arch::asm;

    let stack_pointer: usize;
    unsafe {
        asm!(
            "mov {}, rsp",
            out(reg) stack_pointer,
            options(nostack,nomem),
        );
    }
    stack_pointer
}

#[no_mangle]
pub extern "C" fn libaero_get_own_stackinfo(stack_addr: *mut *mut u8, stack_size: *mut usize) {
    let fp = unsafe { libc::fopen(b"/proc/self/maps".as_ptr().cast(), "r".as_ptr().cast()) };
    assert!(!fp.is_null());

    let mut buffer = [0u8; 4096];
    let count = unsafe { libc::fread(buffer.as_mut_ptr().cast(), 1, 4096 * 2, fp) };
    assert!(count <= buffer.len());

    let maps = unsafe { core::str::from_utf8_unchecked(&buffer[..count]) };
    let json = serde_json::from_str::<Vec<Mapping>>(maps).unwrap();

    let sp = get_sp();
    let stack_mapping = json.iter().find(|x| sp < x.end && sp > x.start).unwrap();

    unsafe {
        stack_addr.write(stack_mapping.start as *mut u8);
        stack_size.write(stack_mapping.end - stack_mapping.start);
    }
}
