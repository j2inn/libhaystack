// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack library
//!
//! Rust Implementation of the [Haystack 4](https://project-haystack.org/) specification for [types](crate::val),
//! [filter](crate::filter), [units](crate::units), and [encoding/decoding](crate::encoding).
//!
//! The library exposes the [Value](crate::val::value::Value) fundamental type that is used throughout the library
//! APIs. The more granular types are also provided for Haystack's type model.

#[cfg(feature = "c-api")]
pub mod c_api;
pub mod haystack;

pub use haystack::*;

#[cfg(feature = "timezone")]
extern crate chrono_tz;
#[macro_use]
extern crate lazy_static;

#[cfg(target_arch = "wasm32")]
extern crate web_sys;

#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
