use std::{ffi::CStr, os::raw::c_char};

#[derive(Debug)]
#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: usize,
}

#[no_mangle]
pub extern "C" fn free_char(buf: *const c_char) {
    assert!(!buf.is_null());
    unsafe { CStr::from_ptr(buf) };
}

#[no_mangle]
pub extern "C" fn free_buf(buf: Buffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}
