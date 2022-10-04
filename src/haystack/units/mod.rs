// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Units module

pub mod unit;
pub mod unit_dimension;
#[cfg(feature = "units-db")]
pub mod units_generated;
use lazy_static::lazy_static;

pub use unit::Unit;
pub use unit_dimension::UnitDimensions;

/// Get unit by name, if it is defined in the units database
#[allow(unused_variables)]
pub fn get_unit(unit: &str) -> Option<&'static Unit> {
    #[cfg(feature = "units-db")]
    {
        return units_generated::UNITS.get(unit).copied();
    }
    #[cfg(not(feature = "units-db"))]
    return None;
}

/// Tries to get the unit by name, if none is found, return a default unit
pub fn get_unit_or_default(unit: &str) -> &'static Unit {
    get_unit(unit).unwrap_or_else(|| &*DEFAULT_UNIT)
}

/// Match units for the dimension
#[allow(unused_variables)]
pub fn match_units(dim: UnitDimensions, scale: f64) -> Vec<&'static Unit> {
    #[cfg(feature = "units-db")]
    {
        units_generated::UNITS
            .iter()
            .filter_map(|(_, u)| {
                if u.dimensions.as_ref() == Some(&dim) && approx_eq(u.scale, scale) {
                    Some(*u)
                } else {
                    None
                }
            })
            .collect()
    }
    #[cfg(not(feature = "units-db"))]
    return Vec::default();
}

#[cfg(feature = "units-db")]
fn approx_eq(a: f64, b: f64) -> bool {
    if a == b {
        return true;
    }
    let min_precision = f64::min(f64::abs(a / 1e3), f64::abs(b / 1e3));
    f64::abs(a - b) <= min_precision
}

lazy_static! {
    /// The dimensionless default unit
    pub static ref DEFAULT_UNIT: Unit = Unit::default();
}
