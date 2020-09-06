use std::os::raw::*;

#[repr(C)]

pub struct git_error {
    pub message: *const c_char,
    pub klass: c_int
}

// typedef struct {
//     char *message;
//     int klass;
// } fit_error;

#[repr(i16)]
enum git_error_code {
    GIT_OK        =  0,
    GIT_ERROR     = -1,
    GIT_ENOTFOUND = -3,
    GIT_EEXISTS   = -4,
}

// enum git_error_code: int16_t {
//     GIT_OK        =  0,
//     GIT_ERROR     = -1,
//     GIT_ENOTFOUND = -3,
//     GIT_EEXISTS   = -4,
// }

fn main() {
    println!("Hello, world!");
}
