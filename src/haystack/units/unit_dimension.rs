// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Unit Dimension type

use std::ops::{Add, Sub};

///
/// A unit dimension.
///
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Clone, Copy)]
pub struct UnitDimensions {
    /// Kilograms
    pub kg: i8,
    /// Meters
    pub m: i8,
    /// Seconds
    pub sec: i8,
    /// Kelvins
    pub k: i8,
    /// Amperes
    pub a: i8,
    /// Mols
    pub mol: i8,
    /// Candelas
    pub cd: i8,
}

/// Addition operator
impl Add for UnitDimensions {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            kg: self.kg + other.kg,
            m: self.m + other.m,
            sec: self.sec + other.sec,
            k: self.k + other.k,
            a: self.a + other.a,
            mol: self.mol + other.mol,
            cd: self.cd + other.cd,
        }
    }
}

/// Subtraction operator
impl Sub for UnitDimensions {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            kg: self.kg - other.kg,
            m: self.m - other.m,
            sec: self.sec - other.sec,
            k: self.k - other.k,
            a: self.a - other.a,
            mol: self.mol - other.mol,
            cd: self.cd - other.cd,
        }
    }
}

#[cfg(test)]
mod test {
    use super::UnitDimensions;
    #[test]
    fn test_add() {
        let dim1 = UnitDimensions {
            kg: 1,
            m: 1,
            sec: 1,
            k: 1,
            a: 1,
            mol: 1,
            cd: 1,
        };
        let dim2 = UnitDimensions {
            kg: 2,
            m: 2,
            sec: 2,
            k: 2,
            a: 2,
            mol: 2,
            cd: 2,
        };

        assert_eq!(
            dim1 + dim2,
            UnitDimensions {
                kg: 3,
                m: 3,
                sec: 3,
                k: 3,
                a: 3,
                mol: 3,
                cd: 3
            }
        )
    }

    #[test]
    fn test_sub() {
        let dim1 = UnitDimensions {
            kg: 3,
            m: 3,
            sec: 3,
            k: 3,
            a: 3,
            mol: 3,
            cd: 3,
        };
        let dim2 = UnitDimensions {
            kg: 2,
            m: 2,
            sec: 2,
            k: 2,
            a: 2,
            mol: 2,
            cd: 2,
        };

        assert_eq!(
            dim1 - dim2,
            UnitDimensions {
                kg: 1,
                m: 1,
                sec: 1,
                k: 1,
                a: 1,
                mol: 1,
                cd: 1,
            }
        )
    }
}
