// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for dealing with error handling.
//!

use std::cell::RefCell;
use std::error::Error;
use std::ffi::CString;
use std::fmt::{Display, Formatter, Result};
use std::os::raw::c_char;

thread_local! {
    /// Last error
    static LAST_ERROR: RefCell<Option<Box<dyn Error>>> = RefCell::new(None);
}

#[derive(Debug)]
struct HaystackErr {
    details: String,
}

impl HaystackErr {
    fn new(msg: &str) -> HaystackErr {
        HaystackErr {
            details: msg.to_string(),
        }
    }
}

impl Display for HaystackErr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for HaystackErr {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Update the most recent error, clearing whatever may have been there before.
pub(super) fn update_last_error<E: Error + 'static>(err: E) {
    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

/// Register new error.
pub(super) fn new_error(msg: &str) {
    update_last_error(HaystackErr::new(msg))
}

/// Retrieve the most recent error, clearing it in the process.
fn take_last_error() -> Option<Box<dyn Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

/// Retrieves last error message if any
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::err::*;
/// # unsafe {
/// let val = haystack_value_make_str(std::ptr::null());
/// assert!(val.is_none());
/// assert_ne!(last_error_message(), std::ptr::null())
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn last_error_message() -> *const c_char {
    match take_last_error() {
        Some(err) => CString::new(err.to_string().as_bytes())
            .expect("Invalid Str")
            .into_raw(),
        None => std::ptr::null(),
    }
}
