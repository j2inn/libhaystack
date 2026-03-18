// Copyright (C) 2020 - 2024, J2 Innovations

//!
//! C API for Brio binary encoding.
//!

use super::err::{new_error, update_last_error};
use crate::haystack::encoding::brio::decode::FromBrio;
use crate::haystack::encoding::brio::encode::ToBrio;
use crate::haystack::val::Value;
use std::io::Cursor;

/// Encodes a [Value](crate::val::Value) to Brio binary bytes.
///
/// # Arguments
/// - `val` Value to encode
/// - `out_len` pointer that receives the byte count of the returned buffer
///
/// # Return
/// A pointer to a heap-allocated byte buffer containing the Brio-encoded value,
/// or `null` on error.  The caller **must** free the buffer by passing the
/// pointer and length to [`haystack_brio_bytes_free`].
///
/// On error, [`last_error_message`](super::err::last_error_message) returns the
/// reason.
///
/// # Safety
/// Panics on invalid input data
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haystack_value_to_brio(val: *const Value, out_len: *mut usize) -> *mut u8 {
    if out_len.is_null() {
        new_error("out_len must not be null");
        return std::ptr::null_mut();
    }

    match unsafe { val.as_ref() } {
        Some(val) => match val.to_brio_vec() {
            Ok(bytes) => {
                let boxed: Box<[u8]> = bytes.into_boxed_slice();
                unsafe { *out_len = boxed.len() };
                Box::into_raw(boxed) as *mut u8
            }
            Err(err) => {
                update_last_error(err);
                std::ptr::null_mut()
            }
        },
        None => {
            new_error("Invalid value");
            std::ptr::null_mut()
        }
    }
}

/// Decodes a [Value](crate::val::Value) from Brio binary bytes.
///
/// # Arguments
/// - `data` pointer to the Brio-encoded bytes
/// - `len` number of bytes at `data`
///
/// # Return
/// The decoded [Value](crate::val::Value), or `null` on error.  Call
/// [`last_error_message`](super::err::last_error_message) to retrieve the
/// error description.
///
/// # Safety
/// Panics on invalid input data
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haystack_value_from_brio(
    data: *const u8,
    len: usize,
) -> Option<Box<Value>> {
    if data.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }

    let bytes = unsafe { std::slice::from_raw_parts(data, len) };
    let mut cursor = Cursor::new(bytes);

    match Value::from_brio(&mut cursor) {
        Ok(val) => Some(Box::new(val)),
        Err(err) => {
            update_last_error(err);
            None
        }
    }
}

/// Frees a byte buffer previously returned by [`haystack_value_to_brio`].
///
/// # Arguments
/// - `ptr` the pointer returned by [`haystack_value_to_brio`]
/// - `len` the byte count written to `out_len` by [`haystack_value_to_brio`]
///
/// Passing `null` is a no-op.
///
/// # Safety
/// `ptr` must have been allocated by [`haystack_value_to_brio`] with the
/// matching `len`.  Double-free or mismatched length is undefined behaviour.
#[unsafe(no_mangle)]
pub extern "C" fn haystack_brio_bytes_free(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len)) });
    }
}
