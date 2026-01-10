use std::ffi::CStr;
use std::str::Utf8Error;

#[no_mangle]
pub unsafe extern "C" fn ptr_to_str<'a>(
    ptr: *const std::ffi::c_char,
) -> Result<&'a str, Utf8Error> {
    CStr::from_ptr(ptr).to_str()
}

pub fn bytes_to_int<'a>(s: &'a [u8]) -> (Option<i64>, &'a [u8]) {
    let mut i = 0;
    if s[i] == b'-' {
        i += 1;
    }

    let start_digits = i;
    while i < s.len() && s[i].is_ascii_digit() {
        i += 1;
    }

    if i == start_digits {
        return (None, s);
    }

    let (num, rest) = s.split_at(i);
    (str::from_utf8(num).ok().unwrap().parse().ok(), rest)
}

pub fn str_to_int<'a>(s: &'a str) -> (Option<i64>, &'a str) {
    let mut i = 0;
    let mut neg = false;
    if s.chars().nth(0) == Some('-') {
        neg = true;
        i += 1;
    }

    let start_digits = i;
    while i < s.len() && s.chars().nth(i).unwrap().is_ascii_digit() {
        i += 1;
    }

    if i == start_digits {
        return (None, s);
    }

    let (num, rest) = s.split_at(i);
    (num.parse().ok(), rest)
}

pub fn error(message: &str) {
    eprintln!("{}", message);
    panic!();
}
