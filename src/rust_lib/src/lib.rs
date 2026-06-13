use std::ffi::CString;
use libc::c_char;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("-----------------------------------------");
    println!("Hello from Rust! This is ngspice-46+ (Hybrid)");
    println!("-----------------------------------------");
}

#[no_mangle]
pub extern "C" fn get_rust_message() -> *mut c_char {
    let s = CString::new("This message was generated in Rust!").unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn free_rust_message(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
