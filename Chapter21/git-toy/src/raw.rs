#![allow(non_camel_case_types)]

use std::os::raw::{c_int, c_char, c_uchar};

#[link(name = "git2")]
extern {
    pub fn git_libgit2_init() -> c_int;
    pub fn git_libgit2_shutdown() -> c_int;
    pub fn gitter_last() -> *const git_error;

    pub fn git repository_open(out: *mut *mut git_repository,
                               path: *const c_char) -> c_int;
}

