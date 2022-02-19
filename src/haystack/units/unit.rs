// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Unit types

use super::match_units;
use super::unit_dimension::UnitDimensions;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Div, Mul};

#[derive(Debug, PartialOrd, Default)]
pub struct Unit {
    pub quantity: Option<String>,
    pub ids: Vec<String>,
    pub dimensions: Option<UnitDimensions>,
    pub scale: f64,
    pub offset: f64,
}

impl Unit {
    ///  The unit name.
    pub fn name(&self) -> &str {
        self.ids.get(0).map_or("", |v| v.as_str())
    }

    /// The unit symbol.
    pub fn symbol(&self) -> &str {
        self.ids.last().map_or("", |v| v.as_str())
    }

    ///
    /// Convert the unit to the new scalar.
    ///
    /// # Arguments
    /// - scalar The new scalar value.
    /// - to The new unit to convert too.
    ///
    /// # Returns
    /// The new scalar value.
    ///
    pub fn convert_to(&self, scalar: f64, to: &Unit) -> Result<f64, String> {
        if !(self.is_byte_unit() && to.is_byte_unit()) && self.dimensions != to.dimensions {
            Err(format!("Inconvertible units: {self} and {to}"))
        } else {
            Ok(((scalar * self.scale + self.offset) - to.offset) / to.scale)
        }
    }

    /// Returns true if the unit is for bytes.
    ///
    /// # Returns
    /// True if the unit is for bytes.
    ///
    pub fn is_byte_unit(&self) -> bool {
        return self.quantity == Some("bytes".to_string())
            || self.name() == "byte"
            || self.name() == "kilobyte"
            || self.name() == "megabyte"
            || self.name() == "gigabyte"
            || self.name() == "terabyte"
            || self.name() == "petabyte";
    }
}

/// Multiplication operator
impl Mul<&'static Unit> for &Unit {
    type Output = Result<&'static Unit, String>;

    fn mul(self, other: &'static Unit) -> Self::Output {
        if let (Some(dim1), Some(dim2)) = (self.dimensions, other.dimensions) {
            // Compute dim/scale of a * b.
            let dim = dim1 + dim2;
            let scale = self.scale * other.scale;
            let units = match_units(dim, scale);

            if units.len() == 1 {
                return Ok(units[0]);
            }

            let expected_name = format!("{this}_{other}", this = self.name(), other = other.name());

            return if let Some(unit) = units.iter().find(|u| u.name() == expected_name) {
                Ok(*unit)
            } else {
                Err(format!(
                    "Cannot match units {this}*{other}",
                    this = self.name(),
                    other = &other.name()
                ))
            };
        }

        Err("Can't multiply dimensionless units".to_string())
    }
}

/// Division operator
impl Div<&'static Unit> for &Unit {
    type Output = Result<&'static Unit, String>;

    fn div(self, other: &'static Unit) -> Self::Output {
        if let (Some(dim1), Some(dim2)) = (self.dimensions, other.dimensions) {
            // Compute dim/scale of a / b.
            let dim = dim1 - dim2;
            let scale = self.scale / other.scale;
            let units = match_units(dim, scale);

            if units.len() == 1 {
                return Ok(units[0]);
            }

            let singular_composition = format!(
                "{this}_per_{other}",
                this = self.name(),
                other = other.name()
            );
            let plural_composition = format!(
                "{this}s_per_{other}",
                this = self.name(),
                other = other.name()
            );

            return if let Some(unit) = units
                .iter()
                .find(|u| u.name() == singular_composition || u.name() == plural_composition)
            {
                Ok(*unit)
            } else {
                Err(format!(
                    "Cannot match units {this}/{other}",
                    this = self.name(),
                    other = &other.name()
                ))
            };
        }

        Err("Can't divide dimensionless units".to_string())
    }
}

impl Display for Unit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.symbol())
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
            && self.ids == other.ids
            && self.dimensions == other.dimensions
            && self.scale.to_bits() == other.scale.to_bits()
            && self.offset.to_bits() == other.offset.to_bits()
    }
}

impl Eq for Unit {}

impl Hash for Unit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.quantity.hash(state);
        self.ids.hash(state);
        self.dimensions.hash(state);
        self.scale.to_bits().hash(state);
        self.offset.to_bits().hash(state);
    }
}

/// Make a Haystack `Unit` from a `&str` slice
/// If no unit is found, the default empty unit is returned
impl From<&str> for &Unit {
    fn from(value: &str) -> Self {
        super::get_unit_or_default(value)
    }
}

#[cfg(test)]
mod test {
    use super::super::get_unit;
    use super::*;

    #[test]
    fn test_unit_from_str() {
        let unit: &Unit = "m³".into();
        assert_eq!(unit.symbol(), "m³")
    }

    #[test]
    fn test_unit_convert() {
        let meters = get_unit("m").expect("Meters");
        let foot = get_unit("ft").expect("Foot");
        assert_eq!(meters.convert_to(1.0, foot).map(|f| f.round()), Ok(3.0));

        let fahrenheit = get_unit("°F").expect("Fahrenheit");
        let celsius = get_unit("°C").expect("Celsius");
        assert_eq!(
            fahrenheit.convert_to(100.0, celsius).map(|f| f.round()),
            Ok(38.0)
        );
    }

    #[test]
    fn test_unit_convert_bad_dimensions() {
        let u1 = get_unit("kWh/m²").expect("Unit");
        let u2 = get_unit("J/kg").expect("Unit");
        assert!(u1.convert_to(1.0, u2).is_err());
    }

    #[test]
    fn test_unit_multiply() {
        let u1 = get_unit("megawatt").expect("Unit");
        let u2 = get_unit("hour").expect("Unit");
        assert_eq!(u1 * u2, Ok(get_unit("megawatt_hour").expect("Unit")));
    }

    #[test]
    fn test_unit_multiply_different() {
        let u1 = get_unit("ft").expect("Unit");
        let u2 = get_unit("hour").expect("Unit");
        assert!((u1 * u2).is_err());
    }

    #[test]
    fn test_unit_divide() {
        let u1 = get_unit("kg").expect("Unit");
        let u2 = get_unit("hour").expect("Unit");
        assert_eq!(u1 / u2, Ok(get_unit("kilograms_per_hour").expect("Unit")));
    }

    #[test]
    fn test_unit_divide_different() {
        let u1 = get_unit("ft").expect("Unit");
        let u2 = get_unit("hour").expect("Unit");
        assert!((u1 / u2).is_err());
    }
}
