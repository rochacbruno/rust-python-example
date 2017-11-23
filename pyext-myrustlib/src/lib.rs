#[macro_use] extern crate cpython;
// extern crate regex;

use cpython::{Python, PyResult};
// use regex::Regex;
use std::mem;

fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }

    Ok(total)
}

fn count_doubles_once(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    let mut chars = val.chars();
    if let Some(mut c1) = chars.next() {
        for c2 in chars {
            if c1 == c2 {
                total += 1;
            }
            c1 = c2;
        }
    }

    Ok(total)
}

fn count_doubles_memreplace(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    let mut chars = val.chars();
    if let Some(mut c1) = chars.next() {
        for c2 in chars {
            if c1 == c2 {
                total += 1;
            }
            mem::replace(&mut c1, c2);
        }
    }

    Ok(total)
}

fn count_doubles_fold(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    let mut chars = val.chars();
    if let Some(c1) = chars.next() {
        chars.fold(c1, |c1, c2| {
            if c1 == c2 {
                total += 1
            }
            c2
        });
    }

    Ok(total)
}

fn count_doubles_peek(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    let mut chars = val.chars().peekable();

    while let Some(c1) = chars.next() {
        if let Some(c2) = chars.peek() {
            if &c1 == c2 {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn count_doubles_once_bytes(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    let mut chars = val.bytes();
    if let Some(mut c1) = chars.next() {
        for c2 in chars {
            if c1 == c2 {
                total += 1;
            }
            c1 = c2;
        }
    }

    Ok(total)
}


// Rust Gegex crate does not support lokkaround/backreference
// https://github.com/rust-lang/regex/issues/302
// fn count_doubles_regex(_py: Python, val: &str) -> PyResult<u64> {
//     let re = Regex::new(r"(?=(.)\1)").unwrap();
//     let total: u64 = re.captures_iter(val).count() as u64;
//     Ok(total)
// }


py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_myrustlib, |py, m | {
    try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    try!(m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str))));
    try!(m.add(py, "count_doubles_once", py_fn!(py, count_doubles_once(val: &str))));
    try!(m.add(py, "count_doubles_once_bytes", py_fn!(py, count_doubles_once_bytes(val: &str))));
    try!(m.add(py, "count_doubles_peek", py_fn!(py, count_doubles_peek(val: &str))));
    try!(m.add(py, "count_doubles_memreplace", py_fn!(py, count_doubles_memreplace(val: &str))));
    try!(m.add(py, "count_doubles_fold", py_fn!(py, count_doubles_fold(val: &str))));
    // try!(m.add(py, "count_doubles_regex", py_fn!(py, count_doubles_regex(val: &str))));
    Ok(())
});
