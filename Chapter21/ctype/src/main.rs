use std::os::raw::*;
use std::ffi::{CString, CStr};

extern {
    fn strlen(s: *const c_char) -> usize;
    
    static environ: *mut *mut c_char;
}

// #[repr(C)]
// 
// pub struct git_error {
//     pub message: *const c_char,
//     pub klass: c_int
// }
// 
// // typedef struct {
// //     char *message;
// //     int klass;
// // } fit_error;
// 
// #[repr(i16)]
// enum git_error_code {
//     GIT_OK        =  0,
//     GIT_ERROR     = -1,
//     GIT_ENOTFOUND = -3,
//     GIT_EEXISTS   = -4,
// }
// 
// // enum git_error_code: int16_t {
// //     GIT_OK        =  0,
// //     GIT_ERROR     = -1,
// //     GIT_ENOTFOUND = -3,
// //     GIT_EEXISTS   = -4,
// // }

fn main() {
    println!("Hello, world!");
    let rust_str = "I'll be back";
    let null_terminated = CString::new(rust_str).unwrap();
    unsafe {
        println!("strlen(null_terminated.as_ptr()) : {}", 
            strlen(null_terminated.as_ptr()));
    }

    unsafe {
        if !environ.is_null() && !(*environ).is_null() {
            let var = CStr::from_ptr(*environ);
            println!("first environment variable: {}",
                var.to_string_lossy())
        }
    }
}
