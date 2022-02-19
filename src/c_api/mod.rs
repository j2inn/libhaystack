// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack C based API for the core data types, encoders, and filters

pub mod coord;
pub mod date;
pub mod datetime;
pub mod dict;
#[cfg(feature = "c-api-filter")]
pub mod filter;
pub mod grid;
#[cfg(feature = "c-api-json")]
pub mod json;
pub mod list;
pub mod number;
pub mod reference;
pub mod str;
pub mod symbol;
pub mod time;
pub mod uri;
pub mod value;
pub mod xstr;
#[cfg(feature = "c-api-zinc")]
pub mod zinc;

pub mod err;

/// Generic result type for API calls
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum ResultType {
    ERR = -1,
    FALSE = 0,
    TRUE = 1,
}

#[macro_export]
macro_rules! safe_bool_call {
    ($self: ident, $func: ident) => {
        match $self.as_ref() {
            Some(value) => value.$func(),
            None => {
                new_error("Invalid Value reference");
                false
            }
        }
    };
}
