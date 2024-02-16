// Copyright (C) 2020 - 2022, J2 Innovations
// Haystack Unit module - auto generated.

#![allow(clippy::approx_constant)]
use super::{Unit, UnitDimensions};
use lazy_static::lazy_static;
use std::collections::HashMap;

// dimensionless

lazy_static! {
    pub static ref PERCENT: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["percent".to_string(), "%".to_string(),].to_vec(),
        dimensions: None,
        scale: 0.01,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PIXEL: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["pixel".to_string(), "px".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DECIBEL: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["decibel".to_string(), "db".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref POWER_FACTOR: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["power_factor".to_string(), "pf".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PH: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["ph".to_string(), "pH".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PERCENT_RELATIVE_HUMIDITY: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["percent_relative_humidity".to_string(), "%RH".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: [
            "grams_of_water_per_kilogram_dry_air".to_string(),
            "gH₂O/kgAir".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref VOLTS_PER_DEGREE_KELVIN: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["volts_per_degree_kelvin".to_string(), "V/K".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREE_DAYS_CELSIUS: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["degree_days_celsius".to_string(), "°daysC".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREE_DAYS_FAHRENHEIT: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["degree_days_fahrenheit".to_string(), "°daysF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PERCENT_OBSCURATION_PER_FOOT: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: [
            "percent_obscuration_per_foot".to_string(),
            "%obsc/ft".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PERCENT_OBSCURATION_PER_METER: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: [
            "percent_obscuration_per_meter".to_string(),
            "%obsc/m".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PSI_PER_DEGREE_FAHRENHEIT: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: [
            "psi_per_degree_fahrenheit".to_string(),
            "psi/°F".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SQUARE_METERS_PER_NEWTON: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["square_meters_per_newton".to_string(), "m²/N".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref WATTS_PER_SQUARE_METER_DEGREE_KELVIN: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: [
            "watts_per_square_meter_degree_kelvin".to_string(),
            "W/m²K".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DB_MILLIVOLT: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["db_millivolt".to_string(), "dBmV".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DB_MICROVOLT: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["db_microvolt".to_string(), "dBµV".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PARTS_PER_UNIT: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["parts_per_unit".to_string(), "ppu".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PARTS_PER_MILLION: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["parts_per_million".to_string(), "ppm".to_string(),].to_vec(),
        dimensions: None,
        scale: 1e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PARTS_PER_BILLION: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["parts_per_billion".to_string(), "ppb".to_string(),].to_vec(),
        dimensions: None,
        scale: 1e-9,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GRAMS_PER_KILOGRAM: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["grams_per_kilogram".to_string(), "g/kg".to_string(),].to_vec(),
        dimensions: None,
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref RADIAN: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["radian".to_string(), "rad".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREES_ANGULAR: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["degrees_angular".to_string(), "deg".to_string(),].to_vec(),
        dimensions: None,
        scale: 0.017453292519943,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREES_PHASE: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["degrees_phase".to_string(), "degPh".to_string(),].to_vec(),
        dimensions: None,
        scale: 0.017453292519943,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref STERADIAN: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["steradian".to_string(), "sr".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NEPHELOMETRIC_TURBIDITY_UNITS: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: [
            "nephelometric_turbidity_units".to_string(),
            "ntu".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FORMAZIN_NEPHELOMETRIC_UNIT: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["formazin_nephelometric_unit".to_string(), "fnu".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref POWER_USAGE_EFFECTIVENESS: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["power_usage_effectiveness".to_string(), "PUE".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DATA_CENTER_INFRASTRUCTURE_EFFICIENCY: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: [
            "data_center_infrastructure_efficiency".to_string(),
            "DCIE".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

// currency

lazy_static! {
    pub static ref AFGHANI: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["afghani".to_string(), "AFN".to_string(), "Af".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ALGERIAN_DINAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["algerian_dinar".to_string(), "DZD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ARGENTINE_PESO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["argentine_peso".to_string(), "ARS".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ARMENIAN_DRAM: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "armenian_dram".to_string(),
            "AMD".to_string(),
            "Դ".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ARUBAN_GUILDER_FLORIN: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "aruban_guilder_florin".to_string(),
            "AWG".to_string(),
            "ƒ".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref AUSTRALIAN_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["australian_dollar".to_string(), "AUD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref AZERBAIJANIAN_MANAT: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "azerbaijanian_manat".to_string(),
            "AZN".to_string(),
            "ман".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BAHAMIAN_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["bahamian_dollar".to_string(), "BSD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BAHRAINI_DINAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["bahraini_dinar".to_string(), "BHD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BAHT: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["baht".to_string(), "THB".to_string(), "฿".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BALBOA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["balboa".to_string(), "PAB".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BARBADOS_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["barbados_dollar".to_string(), "BBD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BELARUSSIAN_RUBLE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "belarussian_ruble".to_string(),
            "BYR".to_string(),
            "Br".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BELIZE_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["belize_dollar".to_string(), "BZD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BERMUDIAN_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["bermudian_dollar".to_string(), "BMD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BOLIVAR_FUERTE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["bolivar_fuerte".to_string(), "VEF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BOLIVIANO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["boliviano".to_string(), "BOB".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BRAZILIAN_REAL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "brazilian_real".to_string(),
            "BRL".to_string(),
            "R$".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BRUNEI_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["brunei_dollar".to_string(), "BND".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BULGARIAN_LEV: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "bulgarian_lev".to_string(),
            "BGN".to_string(),
            "лв".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BURUNDI_FRANC: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["burundi_franc".to_string(), "BIF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CANADIAN_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["canadian_dollar".to_string(), "CAD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CAPE_VERDE_ESCUDO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["cape_verde_escudo".to_string(), "CVE".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CAYMAN_ISLANDS_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["cayman_islands_dollar".to_string(), "KYD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CEDI: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["cedi".to_string(), "GHS".to_string(), "₵".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CFA_FRANC_BCEAO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["cfa_franc_bceao".to_string(), "XAF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CFP_FRANC: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["cfp_franc".to_string(), "XPF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CHILEAN_PESO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["chilean_peso".to_string(), "CLP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CHINESE_YUAN: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "chinese_yuan".to_string(),
            "CNY".to_string(),
            "元".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CONGOLESE_FRANC: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["congolese_franc".to_string(), "CDF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CORDOBA_ORO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "cordoba_oro".to_string(),
            "NIO".to_string(),
            "C$".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref COSTA_RICAN_COLON: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "costa_rican_colon".to_string(),
            "CRC".to_string(),
            "₡".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CROATIAN_KUNA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "croatian_kuna".to_string(),
            "HRK".to_string(),
            "Kn".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBAN_PESO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["cuban_peso".to_string(), "CUP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CZECH_KORUNA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "czech_koruna".to_string(),
            "CZK".to_string(),
            "Kč".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DALASI: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["dalasi".to_string(), "GMD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DANISH_KRONE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "danish_krone".to_string(),
            "DKK".to_string(),
            "kr".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DENAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["denar".to_string(), "MKD".to_string(), "ден".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DJIBOUTI_FRANC: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["djibouti_franc".to_string(), "DJF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DOBRA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["dobra".to_string(), "STD".to_string(), "Db".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DOMINICAN_PESO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["dominican_peso".to_string(), "DOP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DONG: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["dong".to_string(), "VND".to_string(), "₫".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref EAST_CARIBBEAN_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["east_caribbean_dollar".to_string(), "XCD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref EGYPTIAN_POUND: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["egyptian_pound".to_string(), "EGP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ETHIOPIAN_BIRR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["ethiopian_birr".to_string(), "ETB".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref EURO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["euro".to_string(), "EUR".to_string(), "€".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FALKLAND_ISLANDS_POUND: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["falkland_islands_pound".to_string(), "FKP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FIJI_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["fiji_dollar".to_string(), "FJD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FORINT: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["forint".to_string(), "HUF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GIBRALTAR_POUND: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["gibraltar_pound".to_string(), "GIP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GOURDE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["gourde".to_string(), "HTG".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GUARANI: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["guarani".to_string(), "PYG".to_string(), "₲".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GUINEA_FRANC: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["guinea_franc".to_string(), "GNF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GUYANA_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["guyana_dollar".to_string(), "GYD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HONG_KONG_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["hong_kong_dollar".to_string(), "HKD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HRYVNIA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["hryvnia".to_string(), "UAH".to_string(), "₴".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ICELAND_KRONA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "iceland_krona".to_string(),
            "ISK".to_string(),
            "Kr".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref INDIAN_RUPEE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "indian_rupee".to_string(),
            "INR".to_string(),
            "₹".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref IRANIAN_RIAL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["iranian_rial".to_string(), "IRR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref IRAQI_DINAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["iraqi_dinar".to_string(), "IQD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref JAMAICAN_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["jamaican_dollar".to_string(), "JMD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref JORDANIAN_DINAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["jordanian_dinar".to_string(), "JOD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KENYAN_SHILLING: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "kenyan_shilling".to_string(),
            "KES".to_string(),
            "Sh".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KINA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["kina".to_string(), "PGK".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KIP: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["kip".to_string(), "LAK".to_string(), "₭".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KONVERTIBILNA_MARKA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "konvertibilna_marka".to_string(),
            "BAM".to_string(),
            "КМ".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KUWAITI_DINAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["kuwaiti_dinar".to_string(), "KWD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KWACHA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["kwacha".to_string(), "MWK".to_string(), "MK".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KWANZA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["kwanza".to_string(), "AOA".to_string(), "Kz".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KYAT: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["kyat".to_string(), "MMK".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LARI: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["lari".to_string(), "GEL".to_string(), "ლ".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LEBANESE_POUND: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["lebanese_pound".to_string(), "LBP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LEK: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["lek".to_string(), "ALL".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LEMPIRA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["lempira".to_string(), "HNL".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LEONE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["leone".to_string(), "SLL".to_string(), "Le".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LEU: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["leu".to_string(), "RON".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LIBERIAN_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["liberian_dollar".to_string(), "LRD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LIBYAN_DINAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["libyan_dinar".to_string(), "LYD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LILANGENI: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["lilangeni".to_string(), "SZL".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LOTI: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["loti".to_string(), "LSL".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MALAGASY_ARIARY: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["malagasy_ariary".to_string(), "MGA".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MALAYSIAN_RINGGIT: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "malaysian_ringgit".to_string(),
            "MYR".to_string(),
            "RM".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MANAT: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["manat".to_string(), "TMT".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MAURITIUS_RUPEE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["mauritius_rupee".to_string(), "MUR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref METICAL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["metical".to_string(), "MZN".to_string(), "MTn".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEXICAN_PESO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["mexican_peso".to_string(), "MXN".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MOLDAVIAN_LEU: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["moldavian_leu".to_string(), "MDL".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MOROCCAN_DIRHAM: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["moroccan_dirham".to_string(), "MAD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NAIRA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["naira".to_string(), "NGN".to_string(), "₦".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NAKFA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["nakfa".to_string(), "ERN".to_string(), "Nfk".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NAMIBIA_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["namibia_dollar".to_string(), "NAD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NEPALESE_RUPEE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["nepalese_rupee".to_string(), "NPR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NEW_ISRAELI_SHEKEL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "new_israeli_shekel".to_string(),
            "ILS".to_string(),
            "₪".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NEW_ZEALAND_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["new_zealand_dollar".to_string(), "NZD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NGULTRUM: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["ngultrum".to_string(), "BTN".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NORTH_KOREAN_WON: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["north_korean_won".to_string(), "KPW".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NORWEGIAN_KRONE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["norwegian_krone".to_string(), "NOK".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NUEVO_SOL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["nuevo_sol".to_string(), "PEN".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref OUGUIYA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["ouguiya".to_string(), "MRO".to_string(), "UM".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PAKISTAN_RUPEE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "pakistan_rupee".to_string(),
            "PKR".to_string(),
            "₨".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PATACA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["pataca".to_string(), "MOP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PESO_URUGUAYO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["peso_uruguayo".to_string(), "UYU".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PHILIPPINE_PESO: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "philippine_peso".to_string(),
            "PHP".to_string(),
            "₱".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref POUND_STERLING: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "pound_sterling".to_string(),
            "GBP".to_string(),
            "£".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PULA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["pula".to_string(), "BWP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PZLOTY: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["pzloty".to_string(), "PLN".to_string(), "zł".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref QATARI_RIAL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["qatari_rial".to_string(), "QAR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref QUETZAL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["quetzal".to_string(), "GTQ".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref RAND: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["rand".to_string(), "ZAR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref RIAL_OMANI: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["rial_omani".to_string(), "OMR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref RIEL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["riel".to_string(), "KHR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref RUFIYAA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["rufiyaa".to_string(), "MVR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref RUPIAH: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["rupiah".to_string(), "IDR".to_string(), "Rp".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref RUSSIAN_RUBLE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["russian_ruble".to_string(), "RUB".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref RWANDA_FRANC: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["rwanda_franc".to_string(), "RWF".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SAINT_HELENA_POUND: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["saint_helena_pound".to_string(), "SHP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SAUDI_RIYAL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["saudi_riyal".to_string(), "SAR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SERBIAN_DINAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "serbian_dinar".to_string(),
            "RSD".to_string(),
            "din".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SEYCHELLES_RUPEE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["seychelles_rupee".to_string(), "SCR".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SINGAPORE_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["singapore_dollar".to_string(), "SGD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SOLOMON_ISLANDS_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["solomon_islands_dollar".to_string(), "SBD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SOM: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["som".to_string(), "KGS".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SOMALI_SHILLING: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["somali_shilling".to_string(), "SOS".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SOMONI: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["somoni".to_string(), "TJS".to_string(), "ЅМ".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SOUTH_KOREAN_WON: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "south_korean_won".to_string(),
            "KRW".to_string(),
            "₩".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SRI_LANKA_RUPEE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "sri_lanka_rupee".to_string(),
            "LKR".to_string(),
            "Rs".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SUDANESE_POUND: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["sudanese_pound".to_string(), "SDG".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SURINAME_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["suriname_dollar".to_string(), "SRD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SWEDISH_KRONA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["swedish_krona".to_string(), "SEK".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SWISS_FRANC: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "swiss_franc".to_string(),
            "CHF".to_string(),
            "SFr".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SYRIAN_POUND: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["syrian_pound".to_string(), "SYP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TAIWAN_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["taiwan_dollar".to_string(), "TWD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TAKA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["taka".to_string(), "BDT".to_string(), "৳".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TALA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["tala".to_string(), "WST".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TANZANIAN_SHILLING: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["tanzanian_shilling".to_string(), "TZS".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TENGE: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["tenge".to_string(), "KZT".to_string(), "₸".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TRINIDAD_AND_TOBAGO_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["trinidad_and_tobago_dollar".to_string(), "TTD".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TUGRIK: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["tugrik".to_string(), "MNT".to_string(), "₮".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TUNISIAN_DINAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["tunisian_dinar".to_string(), "TND".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TURKISH_LIRA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "turkish_lira".to_string(),
            "TRY".to_string(),
            "₤".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref UAE_DIRHAM: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["uae_dirham".to_string(), "AED".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref UGANDA_SHILLING: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["uganda_shilling".to_string(), "UGX".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref US_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["us_dollar".to_string(), "USD".to_string(), "$".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref UZBEKISTAN_SUM: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["uzbekistan_sum".to_string(), "UZS".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref VATU: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["vatu".to_string(), "VUV".to_string(), "Vt".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref YEMENI_RIAL: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["yemeni_rial".to_string(), "YER".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref YEN: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["yen".to_string(), "JPY".to_string(), "¥".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ZAMBIAN_KWACHA: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: [
            "zambian_kwacha".to_string(),
            "ZMW".to_string(),
            "ZK".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ZIMBABWE_DOLLAR: Unit = Unit {
        quantity: Some("currency".to_string(),),
        ids: ["zimbabwe_dollar".to_string(), "ZWL".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

// acceleration

lazy_static! {
    pub static ref METERS_PER_SECOND_SQUARED: Unit = Unit {
        quantity: Some("acceleration".to_string(),),
        ids: ["meters_per_second_squared".to_string(), "m/s²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// angular acceleration

lazy_static! {
    pub static ref RADIANS_PER_SECOND_SQUARED: Unit = Unit {
        quantity: Some("angular acceleration".to_string(),),
        ids: [
            "radians_per_second_squared".to_string(),
            "rad/s²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// angular momentum

lazy_static! {
    pub static ref JOULE_SECOND: Unit = Unit {
        quantity: Some("angular momentum".to_string(),),
        ids: ["joule_second".to_string(), "Js".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// angular velocity

lazy_static! {
    pub static ref RADIANS_PER_SECOND: Unit = Unit {
        quantity: Some("angular velocity".to_string(),),
        ids: ["radians_per_second".to_string(), "rad/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref REVOLUTIONS_PER_MINUTE: Unit = Unit {
        quantity: Some("angular velocity".to_string(),),
        ids: ["revolutions_per_minute".to_string(), "rpm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 6.2831853071796,
        offset: 0.0,
    };
}

// area

lazy_static! {
    pub static ref SQUARE_METER: Unit = Unit {
        quantity: Some("area".to_string(),),
        ids: ["square_meter".to_string(), "m²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SQUARE_MILLIMETER: Unit = Unit {
        quantity: Some("area".to_string(),),
        ids: ["square_millimeter".to_string(), "mm²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SQUARE_CENTIMETER: Unit = Unit {
        quantity: Some("area".to_string(),),
        ids: ["square_centimeter".to_string(), "cm²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SQUARE_KILOMETER: Unit = Unit {
        quantity: Some("area".to_string(),),
        ids: ["square_kilometer".to_string(), "km²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SQUARE_INCH: Unit = Unit {
        quantity: Some("area".to_string(),),
        ids: ["square_inch".to_string(), "in²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.000645161,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SQUARE_FOOT: Unit = Unit {
        quantity: Some("area".to_string(),),
        ids: ["square_foot".to_string(), "ft²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.092903,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SQUARE_YARD: Unit = Unit {
        quantity: Some("area".to_string(),),
        ids: ["square_yard".to_string(), "yd²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.836131,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SQUARE_MILE: Unit = Unit {
        quantity: Some("area".to_string(),),
        ids: ["square_mile".to_string(), "mile²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2589988.110336,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ACRE: Unit = Unit {
        quantity: Some("area".to_string(),),
        ids: ["acre".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 4046.872627,
        offset: 0.0,
    };
}

// capacitance

lazy_static! {
    pub static ref FARAD: Unit = Unit {
        quantity: Some("capacitance".to_string(),),
        ids: ["farad".to_string(), "F".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: -1,
            m: -2,
            sec: 4,
            k: 0,
            a: 2,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// cooling efficiency

lazy_static! {
    pub static ref COEFFICIENT_OF_PERFORMANCE: Unit = Unit {
        quantity: Some("cooling efficiency".to_string(),),
        ids: ["coefficient_of_performance".to_string(), "COP".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ENERGY_EFFICIENCY_RATIO: Unit = Unit {
        quantity: Some("cooling efficiency".to_string(),),
        ids: [
            "energy_efficiency_ratio".to_string(),
            "Btu/Wh".to_string(),
            "EER".to_string(),
        ]
        .to_vec(),
        dimensions: None,
        scale: 0.2930832356,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATT_PER_TON: Unit = Unit {
        quantity: Some("cooling efficiency".to_string(),),
        ids: ["kilowatt_per_ton".to_string(), "kW/ton".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

// density

lazy_static! {
    pub static ref KILOGRAMS_PER_CUBIC_METER: Unit = Unit {
        quantity: Some("density".to_string(),),
        ids: ["kilograms_per_cubic_meter".to_string(), "kg/m³".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GRAMS_PER_CUBIC_METER: Unit = Unit {
        quantity: Some("density".to_string(),),
        ids: ["grams_per_cubic_meter".to_string(), "g/m³".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIGRAMS_PER_CUBIC_METER: Unit = Unit {
        quantity: Some("density".to_string(),),
        ids: [
            "milligrams_per_cubic_meter".to_string(),
            "mg/m³".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MICROGRAMS_PER_CUBIC_METER: Unit = Unit {
        quantity: Some("density".to_string(),),
        ids: [
            "micrograms_per_cubic_meter".to_string(),
            "µg/m³".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-9,
        offset: 0.0,
    };
}

// electric charge

lazy_static! {
    pub static ref COULOMB: Unit = Unit {
        quantity: Some("electric charge".to_string(),),
        ids: ["coulomb".to_string(), "C".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 1,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref AMPERE_HOUR: Unit = Unit {
        quantity: Some("electric charge".to_string(),),
        ids: ["ampere_hour".to_string(), "Ah".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 1,
            mol: 0,
            cd: 0,
        },),
        scale: 3600.0,
        offset: 0.0,
    };
}

// electric conductance

lazy_static! {
    pub static ref SIEMENS: Unit = Unit {
        quantity: Some("electric conductance".to_string(),),
        ids: ["siemens".to_string(), "S".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: -1,
            m: -2,
            sec: 3,
            k: 0,
            a: 2,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// electric current

lazy_static! {
    pub static ref AMPERE: Unit = Unit {
        quantity: Some("electric current".to_string(),),
        ids: ["ampere".to_string(), "A".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 0,
            a: 1,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIAMPERE: Unit = Unit {
        quantity: Some("electric current".to_string(),),
        ids: ["milliampere".to_string(), "mA".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 0,
            a: 1,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

// electromagnetic moment

lazy_static! {
    pub static ref AMPERE_SQUARE_METER: Unit = Unit {
        quantity: Some("electromagnetic moment".to_string(),),
        ids: ["ampere_square_meter".to_string(), "Am²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: 0,
            k: 0,
            a: 1,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// electric current density

lazy_static! {
    pub static ref AMPERES_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("electric current density".to_string(),),
        ids: ["amperes_per_square_meter".to_string(), "A/m²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: -2,
            sec: 0,
            k: 0,
            a: 1,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// electric field strength

lazy_static! {
    pub static ref VOLTS_PER_METER: Unit = Unit {
        quantity: Some("electric field strength".to_string(),),
        ids: ["volts_per_meter".to_string(), "V/m".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 1,
            sec: -3,
            k: 0,
            a: -1,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// electric potential

lazy_static! {
    pub static ref VOLT: Unit = Unit {
        quantity: Some("electric potential".to_string(),),
        ids: ["volt".to_string(), "Volt".to_string(), "V".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: -1,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIVOLT: Unit = Unit {
        quantity: Some("electric potential".to_string(),),
        ids: ["millivolt".to_string(), "mV".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: -1,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOVOLT: Unit = Unit {
        quantity: Some("electric potential".to_string(),),
        ids: ["kilovolt".to_string(), "kV".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: -1,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAVOLT: Unit = Unit {
        quantity: Some("electric potential".to_string(),),
        ids: ["megavolt".to_string(), "MV".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: -1,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

// electric resistance

lazy_static! {
    pub static ref OHM: Unit = Unit {
        quantity: Some("electric resistance".to_string(),),
        ids: ["ohm".to_string(), "Ω".to_string(), "Ω".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: -2,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOHM: Unit = Unit {
        quantity: Some("electric resistance".to_string(),),
        ids: ["kilohm".to_string(), "kΩ".to_string(), "kΩ".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: -2,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGOHM: Unit = Unit {
        quantity: Some("electric resistance".to_string(),),
        ids: ["megohm".to_string(), "MΩ".to_string(), "MΩ".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: -2,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIOHM: Unit = Unit {
        quantity: Some("electric resistance".to_string(),),
        ids: ["milliohm".to_string(), "mΩ".to_string(), "mΩ".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: -2,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

// electrical conductivity

lazy_static! {
    pub static ref SIEMENS_PER_METER: Unit = Unit {
        quantity: Some("electrical conductivity".to_string(),),
        ids: ["siemens_per_meter".to_string(), "S/m".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: -1,
            m: -3,
            sec: 3,
            k: 0,
            a: 2,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// electrical resistivity

lazy_static! {
    pub static ref OHM_METER: Unit = Unit {
        quantity: Some("electrical resistivity".to_string(),),
        ids: ["ohm_meter".to_string(), "Ωm".to_string(), "Ωm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 3,
            sec: -3,
            k: 0,
            a: -2,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// energy

lazy_static! {
    pub static ref JOULE: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["joule".to_string(), "J".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOJOULE: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["kilojoule".to_string(), "kJ".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref WATT_HOUR: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["watt_hour".to_string(), "Wh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATT_HOUR: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["kilowatt_hour".to_string(), "kWh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAWATT_HOUR: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["megawatt_hour".to_string(), "MWh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GIGAWATT_HOUR: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["gigawatt_hour".to_string(), "GWh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600000000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BTU: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["btu".to_string(), "BTU".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1054.852,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOBTU: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["kilobtu".to_string(), "kBTU".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1054852.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGABTU: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: [
            "megabtu".to_string(),
            "MBTU".to_string(),
            "MMBTU".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1054852000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HORSEPOWER_HOUR: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["horsepower_hour".to_string(), "hph".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2686088.6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CALORIE: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["calorie".to_string(), "cal".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 4.184,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref THERM: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["therm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 105506000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TONS_REFRIGERATION_HOUR: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["tons_refrigeration_hour".to_string(), "tonrefh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 12660670.8,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAJOULE: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["megajoule".to_string(), "MJ".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GIGAJOULE: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["gigajoule".to_string(), "GJ".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref NEWTON_METER: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["newton_meter".to_string(), "Nm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_METERS_NATURAL_GAS: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: [
            "cubic_meters_natural_gas".to_string(),
            "standard_cubic_meter".to_string(),
            "scm".to_string(),
            "m³_gas".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 37313432.83582089,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_FEET_NATURAL_GAS: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: [
            "cubic_feet_natural_gas".to_string(),
            "standard_cubic_foot".to_string(),
            "scf".to_string(),
            "ft³_gas".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1086498.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HUNDRED_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["hundred_cubic_feet_natural_gas".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 108649800.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref THOUSAND_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["thousand_cubic_feet_natural_gas".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1086498000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLION_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: ["million_cubic_feet_natural_gas".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1086498000000.0,
        offset: 0.0,
    };
}

// apparent energy

lazy_static! {
    pub static ref VOLT_AMPERE_HOUR: Unit = Unit {
        quantity: Some("apparent energy".to_string(),),
        ids: ["volt_ampere_hour".to_string(), "VAh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOVOLT_AMPERE_HOUR: Unit = Unit {
        quantity: Some("apparent energy".to_string(),),
        ids: ["kilovolt_ampere_hour".to_string(), "kVAh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAVOLT_AMPERE_HOUR: Unit = Unit {
        quantity: Some("apparent energy".to_string(),),
        ids: ["megavolt_ampere_hour".to_string(), "MVAh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600000000.0,
        offset: 0.0,
    };
}

// reactive energy

lazy_static! {
    pub static ref VOLT_AMPERE_REACTIVE_HOUR: Unit = Unit {
        quantity: Some("reactive energy".to_string(),),
        ids: ["volt_ampere_reactive_hour".to_string(), "VARh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOVOLT_AMPERE_REACTIVE_HOUR: Unit = Unit {
        quantity: Some("reactive energy".to_string(),),
        ids: [
            "kilovolt_ampere_reactive_hour".to_string(),
            "kVARh".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAVOLT_AMPERE_REACTIVE_HOUR: Unit = Unit {
        quantity: Some("reactive energy".to_string(),),
        ids: [
            "megavolt_ampere_reactive_hour".to_string(),
            "MVARh".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600000000.0,
        offset: 0.0,
    };
}

// energy by area

lazy_static! {
    pub static ref JOULES_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: ["joules_per_square_meter".to_string(), "J/m²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref WATT_HOURS_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "watt_hours_per_square_meter".to_string(),
            "Wh/m²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref WATT_HOURS_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "watt_hours_per_square_foot".to_string(),
            "Wh/ft²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 38750.077500155,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATT_HOURS_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "kilowatt_hours_per_square_meter".to_string(),
            "kWh/m²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATT_HOURS_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "kilowatt_hours_per_square_foot".to_string(),
            "kWh/ft²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 38750077.500155,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAWATT_HOURS_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "megawatt_hours_per_square_meter".to_string(),
            "MWh/m²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAWATT_HOURS_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "megawatt_hours_per_square_foot".to_string(),
            "MWh/ft²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 38750077500.155,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAJOULES_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "megajoules_per_square_meter".to_string(),
            "MJ/m²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAJOULES_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "megajoules_per_square_foot".to_string(),
            "MJ/ft²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 10763910.41671,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOBTU_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "kilobtu_per_square_foot".to_string(),
            "kBTU/ft²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 11354337.31957,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGABTU_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: [
            "megabtu_per_square_foot".to_string(),
            "MBTU/ft²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 11354337319.57,
        offset: 0.0,
    };
}

// enthalpy

lazy_static! {
    pub static ref JOULES_PER_GRAM: Unit = Unit {
        quantity: Some("enthalpy".to_string(),),
        ids: ["joules_per_gram".to_string(), "J/g".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref JOULES_PER_KILOGRAM: Unit = Unit {
        quantity: Some("enthalpy".to_string(),),
        ids: ["joules_per_kilogram".to_string(), "J/kg".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref JOULES_PER_KILOGRAM_DRY_AIR: Unit = Unit {
        quantity: Some("enthalpy".to_string(),),
        ids: [
            "joules_per_kilogram_dry_air".to_string(),
            "J/kg_dry".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BTU_PER_POUND: Unit = Unit {
        quantity: Some("enthalpy".to_string(),),
        ids: ["btu_per_pound".to_string(), "BTU/lb".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2325.5576058607867,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BTUS_PER_POUND_DRY_AIR: Unit = Unit {
        quantity: Some("enthalpy".to_string(),),
        ids: [
            "btus_per_pound_dry_air".to_string(),
            "btu/lb_dry".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2326.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOJOULES_PER_KILOGRAM: Unit = Unit {
        quantity: Some("enthalpy".to_string(),),
        ids: ["kilojoules_per_kilogram".to_string(), "kJ/kg".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOJOULES_PER_KILOGRAM_DRY_AIR: Unit = Unit {
        quantity: Some("enthalpy".to_string(),),
        ids: [
            "kilojoules_per_kilogram_dry_air".to_string(),
            "kJ/kg_dry".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAJOULES_PER_KILOGRAM_DRY_AIR: Unit = Unit {
        quantity: Some("enthalpy".to_string(),),
        ids: [
            "megajoules_per_kilogram_dry_air".to_string(),
            "MJ/kg_dry".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CALORIE_PER_GRAM: Unit = Unit {
        quantity: Some("enthalpy".to_string(),),
        ids: ["calorie_per_gram".to_string(), "cal/g".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 4184.0,
        offset: 0.0,
    };
}

// entropy

lazy_static! {
    pub static ref JOULES_PER_DEGREE_KELVIN: Unit = Unit {
        quantity: Some("entropy".to_string(),),
        ids: ["joules_per_degree_kelvin".to_string(), "J/°K".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: -1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOJOULES_PER_DEGREE_KELVIN: Unit = Unit {
        quantity: Some("entropy".to_string(),),
        ids: [
            "kilojoules_per_degree_kelvin".to_string(),
            "kJ/°K".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: -1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAJOULES_PER_DEGREE_KELVIN: Unit = Unit {
        quantity: Some("entropy".to_string(),),
        ids: [
            "megajoules_per_degree_kelvin".to_string(),
            "MJ/°K".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: -1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

// force

lazy_static! {
    pub static ref NEWTON: Unit = Unit {
        quantity: Some("force".to_string(),),
        ids: ["newton".to_string(), "N".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref POUND_FORCE: Unit = Unit {
        quantity: Some("force".to_string(),),
        ids: ["pound_force".to_string(), "lbf".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 4.448222,
        offset: 0.0,
    };
}

// frequency

lazy_static! {
    pub static ref HERTZ: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["hertz".to_string(), "Hz".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOHERTZ: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["kilohertz".to_string(), "kHz".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CYCLES_PER_HOUR: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["cycles_per_hour".to_string(), "cph".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0002777777777777778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CYCLES_PER_MINUTE: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["cycles_per_minute".to_string(), "cpm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.016666666666666666,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAHERTZ: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["megahertz".to_string(), "MHz".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PER_MINUTE: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["per_minute".to_string(), "/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.016666666666666666,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PER_SECOND: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["per_second".to_string(), "/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PER_HOUR: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["per_hour".to_string(), "/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0002777777777777778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PERCENT_PER_SECOND: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["percent_per_second".to_string(), "%/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref AIR_CHANGES_PER_HOUR: Unit = Unit {
        quantity: Some("frequency".to_string(),),
        ids: ["air_changes_per_hour".to_string(), "ACH".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0002777777777777778,
        offset: 0.0,
    };
}

// grammage

lazy_static! {
    pub static ref KILOGRAMS_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("grammage".to_string(),),
        ids: [
            "kilograms_per_square_meter".to_string(),
            "kg/m²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GRAMS_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("grammage".to_string(),),
        ids: ["grams_per_square_meter".to_string(), "g/m²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

// heating rate

lazy_static! {
    pub static ref DEGREES_KELVIN_PER_SECOND: Unit = Unit {
        quantity: Some("heating rate".to_string(),),
        ids: ["degrees_kelvin_per_second".to_string(), "K/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREES_CELSIUS_PER_HOUR: Unit = Unit {
        quantity: Some("heating rate".to_string(),),
        ids: ["degrees_celsius_per_hour".to_string(), "°C/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0002777777777777778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREES_CELSIUS_PER_MINUTE: Unit = Unit {
        quantity: Some("heating rate".to_string(),),
        ids: [
            "degrees_celsius_per_minute".to_string(),
            "°C/min".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.016666666666666666,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREES_FAHRENHEIT_PER_HOUR: Unit = Unit {
        quantity: Some("heating rate".to_string(),),
        ids: [
            "degrees_fahrenheit_per_hour".to_string(),
            "°F/h".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.00015432098765432,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREES_FAHRENHEIT_PER_MINUTE: Unit = Unit {
        quantity: Some("heating rate".to_string(),),
        ids: [
            "degrees_fahrenheit_per_minute".to_string(),
            "°F/min".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0092592592592593,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREES_KELVIN_PER_HOUR: Unit = Unit {
        quantity: Some("heating rate".to_string(),),
        ids: ["degrees_kelvin_per_hour".to_string(), "K/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0002777777777777778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DEGREES_KELVIN_PER_MINUTE: Unit = Unit {
        quantity: Some("heating rate".to_string(),),
        ids: ["degrees_kelvin_per_minute".to_string(), "K/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: -1,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.016666666666666666,
        offset: 0.0,
    };
}

// illuminance

lazy_static! {
    pub static ref LUX: Unit = Unit {
        quantity: Some("illuminance".to_string(),),
        ids: ["lux".to_string(), "lx".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: -2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 1,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FOOTCANDLE: Unit = Unit {
        quantity: Some("illuminance".to_string(),),
        ids: ["footcandle".to_string(), "fc".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: -2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 1,
        },),
        scale: 0.092937,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PHOT: Unit = Unit {
        quantity: Some("illuminance".to_string(),),
        ids: ["phot".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: -2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 1,
        },),
        scale: 10000.0,
        offset: 0.0,
    };
}

// inductance

lazy_static! {
    pub static ref HENRY: Unit = Unit {
        quantity: Some("inductance".to_string(),),
        ids: ["henry".to_string(), "H".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: -2,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// irradiance

lazy_static! {
    pub static ref WATTS_PER_SQUARE_METER_IRRADIANCE: Unit = Unit {
        quantity: Some("irradiance".to_string(),),
        ids: [
            "watts_per_square_meter_irradiance".to_string(),
            "W/m²_irr".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref WATTS_PER_SQUARE_FOOT_IRRADIANCE: Unit = Unit {
        quantity: Some("irradiance".to_string(),),
        ids: [
            "watts_per_square_foot_irradiance".to_string(),
            "W/ft²_irr".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 10.76391041671,
        offset: 0.0,
    };
}

// length

lazy_static! {
    pub static ref METER: Unit = Unit {
        quantity: Some("length".to_string(),),
        ids: ["meter".to_string(), "m".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MICROMETER: Unit = Unit {
        quantity: Some("length".to_string(),),
        ids: ["micrometer".to_string(), "µm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-5,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIMETER: Unit = Unit {
        quantity: Some("length".to_string(),),
        ids: ["millimeter".to_string(), "mm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CENTIMETER: Unit = Unit {
        quantity: Some("length".to_string(),),
        ids: ["centimeter".to_string(), "cm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.01,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOMETER: Unit = Unit {
        quantity: Some("length".to_string(),),
        ids: ["kilometer".to_string(), "km".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref INCH: Unit = Unit {
        quantity: Some("length".to_string(),),
        ids: ["inch".to_string(), "in".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0254,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FOOT: Unit = Unit {
        quantity: Some("length".to_string(),),
        ids: ["foot".to_string(), "ft".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.3048,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref YARD: Unit = Unit {
        quantity: Some("length".to_string(),),
        ids: ["yard".to_string(), "yd".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.9144,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILE: Unit = Unit {
        quantity: Some("length".to_string(),),
        ids: ["mile".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1609.344,
        offset: 0.0,
    };
}

// luminance

lazy_static! {
    pub static ref CANDELAS_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("luminance".to_string(),),
        ids: ["candelas_per_square_meter".to_string(), "cd/m²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: -2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 1,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CANDELS_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("luminance".to_string(),),
        ids: ["candels_per_square_foot".to_string(), "cd/ft²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: -2,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 1,
        },),
        scale: 0.092937,
        offset: 0.0,
    };
}

// luminous flux

lazy_static! {
    pub static ref LUMEN: Unit = Unit {
        quantity: Some("luminous flux".to_string(),),
        ids: ["lumen".to_string(), "lm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 1,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// luminous intensity

lazy_static! {
    pub static ref CANDELA: Unit = Unit {
        quantity: Some("luminous intensity".to_string(),),
        ids: ["candela".to_string(), "cd".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 1,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// magnetic field strength

lazy_static! {
    pub static ref AMPERES_PER_METER: Unit = Unit {
        quantity: Some("magnetic field strength".to_string(),),
        ids: ["amperes_per_meter".to_string(), "A/m".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: -1,
            sec: 0,
            k: 0,
            a: 1,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// magnetic flux

lazy_static! {
    pub static ref WEBER: Unit = Unit {
        quantity: Some("magnetic flux".to_string(),),
        ids: ["weber".to_string(), "Wb".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -2,
            k: 0,
            a: -1,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// magnetic flux density

lazy_static! {
    pub static ref TESLA: Unit = Unit {
        quantity: Some("magnetic flux density".to_string(),),
        ids: ["tesla".to_string(), "T".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: -1,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// mass

lazy_static! {
    pub static ref KILOGRAM: Unit = Unit {
        quantity: Some("mass".to_string(),),
        ids: ["kilogram".to_string(), "kg".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIGRAM: Unit = Unit {
        quantity: Some("mass".to_string(),),
        ids: ["milligram".to_string(), "mg".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GRAM: Unit = Unit {
        quantity: Some("mass".to_string(),),
        ids: ["gram".to_string(), "g".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref OUNCE: Unit = Unit {
        quantity: Some("mass".to_string(),),
        ids: ["ounce".to_string(), "oz".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.02835,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref POUND: Unit = Unit {
        quantity: Some("mass".to_string(),),
        ids: ["pound".to_string(), "lb".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.453591,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOPOUND: Unit = Unit {
        quantity: Some("mass".to_string(),),
        ids: ["kilopound".to_string(), "klb".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 453.591,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref METRIC_TON: Unit = Unit {
        quantity: Some("mass".to_string(),),
        ids: ["metric_ton".to_string(), "ton".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SHORT_TON: Unit = Unit {
        quantity: Some("mass".to_string(),),
        ids: ["short_ton".to_string(), "t".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 907.18474,
        offset: 0.0,
    };
}

// mass flow

lazy_static! {
    pub static ref KILOGRAMS_PER_SECOND: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["kilograms_per_second".to_string(), "kg/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOGRAMS_PER_MINUTE: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["kilograms_per_minute".to_string(), "kg/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.016666666666666666,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOGRAMS_PER_HOUR: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["kilograms_per_hour".to_string(), "kg/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0002777777777777778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref POUNDS_PER_MINUTE: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["pounds_per_minute".to_string(), "lb/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.007559872833333333,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref POUNDS_PER_HOUR: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["pounds_per_hour".to_string(), "lb/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.00012599788055555556,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref POUNDS_PER_SECOND: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["pounds_per_second".to_string(), "lb/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.45359237,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOPOUNDS_PER_HOUR: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["kilopounds_per_hour".to_string(), "klb/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.12599788055555555,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GRAMS_PER_SECOND: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["grams_per_second".to_string(), "g/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GRAMS_PER_MINUTE: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["grams_per_minute".to_string(), "g/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.6666666666666667e-5,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref METRIC_TONS_PER_HOUR: Unit = Unit {
        quantity: Some("mass flow".to_string(),),
        ids: ["metric_tons_per_hour".to_string(), "ton/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.2777777777777778,
        offset: 0.0,
    };
}

// momentum

lazy_static! {
    pub static ref NEWTON_SECOND: Unit = Unit {
        quantity: Some("momentum".to_string(),),
        ids: ["newton_second".to_string(), "Ns".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// power

lazy_static! {
    pub static ref WATT: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["watt".to_string(), "W".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIWATT: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["milliwatt".to_string(), "mW".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATT: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["kilowatt".to_string(), "kW".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAWATT: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["megawatt".to_string(), "MW".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GIGAWATT: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["gigawatt".to_string(), "GW".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BTUS_PER_HOUR: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["btus_per_hour".to_string(), "BTU/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.292875,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref THERMS_PER_HOUR: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["therms_per_hour".to_string(), "therm/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 29287.5,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HORSEPOWER: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["horsepower".to_string(), "hp".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 745.7,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FOOT_POUNDS_PER_SECOND: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: [
            "foot_pounds_per_second".to_string(),
            "ftlbs/sec".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.355818,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TONS_REFRIGERATION: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["tons_refrigeration".to_string(), "tonref".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3516.853,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOBTUS_PER_HOUR: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["kilobtus_per_hour".to_string(), "kBTU/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 293.07107017222,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGABTUS_PER_HOUR: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: [
            "megabtus_per_hour".to_string(),
            "MBTU/h".to_string(),
            "MMBTU/h".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 293071.07017222,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref JOULES_PER_HOUR: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["joules_per_hour".to_string(), "J/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.000277777778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOJOULES_PER_HOUR: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["kilojoules_per_hour".to_string(), "kJ/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.277777778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAJOULES_PER_HOUR: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["megajoules_per_hour".to_string(), "MJ/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 277.777778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GIGAJOULES_PER_HOUR: Unit = Unit {
        quantity: Some("power".to_string(),),
        ids: ["gigajoules_per_hour".to_string(), "GJ/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 277777.778,
        offset: 0.0,
    };
}

// power by area

lazy_static! {
    pub static ref WATTS_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("power by area".to_string(),),
        ids: ["watts_per_square_meter".to_string(), "W/m²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref WATTS_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("power by area".to_string(),),
        ids: ["watts_per_square_foot".to_string(), "W/ft²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 10.7639104,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATTS_PER_SQUARE_METER: Unit = Unit {
        quantity: Some("power by area".to_string(),),
        ids: [
            "kilowatts_per_square_meter".to_string(),
            "kW/m²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATTS_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("power by area".to_string(),),
        ids: [
            "kilowatts_per_square_foot".to_string(),
            "kW/ft²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 10763.9104,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOBTUS_PER_HOUR_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("power by area".to_string(),),
        ids: [
            "kilobtus_per_hour_per_square_foot".to_string(),
            "kBTU/h/ft²".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3153.8257472,
        offset: 0.0,
    };
}

// power by volumetric flow

lazy_static! {
    pub static ref WATTS_PER_CUBIC_METERS_PER_SECOND: Unit = Unit {
        quantity: Some("power by volumetric flow".to_string(),),
        ids: [
            "watts_per_cubic_meters_per_second".to_string(),
            "W/m³/s".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref WATTS_PER_CUBIC_FEET_PER_MINUTE: Unit = Unit {
        quantity: Some("power by volumetric flow".to_string(),),
        ids: [
            "watts_per_cubic_feet_per_minute".to_string(),
            "W/cfm".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2118.8800032893155,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE: Unit = Unit {
        quantity: Some("power by volumetric flow".to_string(),),
        ids: [
            "kilowatts_per_kilocubic_feet_per_minute".to_string(),
            "kW/kcfm".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2118.8800032893155,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATTS_PER_GALLONS_PER_MINUTE: Unit = Unit {
        quantity: Some("power by volumetric flow".to_string(),),
        ids: [
            "kilowatts_per_gallons_per_minute".to_string(),
            "kW/gal/min".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 15850323.0,
        offset: 0.0,
    };
}

// apparent power

lazy_static! {
    pub static ref VOLT_AMPERE: Unit = Unit {
        quantity: Some("apparent power".to_string(),),
        ids: ["volt_ampere".to_string(), "VA".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOVOLT_AMPERE: Unit = Unit {
        quantity: Some("apparent power".to_string(),),
        ids: ["kilovolt_ampere".to_string(), "kVA".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAVOLT_AMPERE: Unit = Unit {
        quantity: Some("apparent power".to_string(),),
        ids: ["megavolt_ampere".to_string(), "mVA".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

// reactive power

lazy_static! {
    pub static ref VOLT_AMPERE_REACTIVE: Unit = Unit {
        quantity: Some("reactive power".to_string(),),
        ids: ["volt_ampere_reactive".to_string(), "VAR".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOVOLT_AMPERE_REACTIVE: Unit = Unit {
        quantity: Some("reactive power".to_string(),),
        ids: ["kilovolt_ampere_reactive".to_string(), "kVAR".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGAVOLT_AMPERE_REACTIVE: Unit = Unit {
        quantity: Some("reactive power".to_string(),),
        ids: ["megavolt_ampere_reactive".to_string(), "MVAR".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 2,
            sec: -3,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000000.0,
        offset: 0.0,
    };
}

// pressure

lazy_static! {
    pub static ref PASCAL: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["pascal".to_string(), "Pa".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOPASCAL: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["kilopascal".to_string(), "kPa".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref BAR: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["bar".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 100000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref ATMOSPHERE: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["atmosphere".to_string(), "atm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 101317.1,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref POUNDS_PER_SQUARE_INCH: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["pounds_per_square_inch".to_string(), "psi".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 6894.73,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CENTIMETERS_OF_WATER: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["centimeters_of_water".to_string(), "cmH₂O".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 98.0665,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref INCHES_OF_WATER: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: [
            "inches_of_water".to_string(),
            "in/wc".to_string(),
            "inH₂O".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 248.84,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIMETERS_OF_MERCURY: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["millimeters_of_mercury".to_string(), "mmHg".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 133.322368421,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CENTIMETERS_OF_MERCURY: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["centimeters_of_mercury".to_string(), "cmHg".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1333.22368421,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref INCHES_OF_MERCURY: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["inches_of_mercury".to_string(), "inHg".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3386.38815789,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HECTOPASCAL: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["hectopascal".to_string(), "hPa".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 100.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIBAR: Unit = Unit {
        quantity: Some("pressure".to_string(),),
        ids: ["millibar".to_string(), "mbar".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: -1,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 100.0,
        offset: 0.0,
    };
}

// specific entropy

lazy_static! {
    pub static ref JOULES_PER_KILOGRAM_DEGREE_KELVIN: Unit = Unit {
        quantity: Some("specific entropy".to_string(),),
        ids: [
            "joules_per_kilogram_degree_kelvin".to_string(),
            "J/kg°K".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 2,
            sec: -2,
            k: -1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// surface tension

lazy_static! {
    pub static ref NEWTONS_PER_METER: Unit = Unit {
        quantity: Some("surface tension".to_string(),),
        ids: ["newtons_per_meter".to_string(), "N/m".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// temperature

lazy_static! {
    pub static ref FAHRENHEIT: Unit = Unit {
        quantity: Some("temperature".to_string(),),
        ids: ["fahrenheit".to_string(), "°F".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.5555555555555556,
        offset: 255.37222222222223,
    };
}

lazy_static! {
    pub static ref CELSIUS: Unit = Unit {
        quantity: Some("temperature".to_string(),),
        ids: ["celsius".to_string(), "°C".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 273.15,
    };
}

lazy_static! {
    pub static ref KELVIN: Unit = Unit {
        quantity: Some("temperature".to_string(),),
        ids: ["kelvin".to_string(), "K".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// temperature differential

lazy_static! {
    pub static ref FAHRENHEIT_DEGREES: Unit = Unit {
        quantity: Some("temperature differential".to_string(),),
        ids: ["fahrenheit_degrees".to_string(), "Δ°F".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.5555555555555556,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CELSIUS_DEGREES: Unit = Unit {
        quantity: Some("temperature differential".to_string(),),
        ids: ["celsius_degrees".to_string(), "Δ°C".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KELVIN_DEGREES: Unit = Unit {
        quantity: Some("temperature differential".to_string(),),
        ids: ["kelvin_degrees".to_string(), "ΔK".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 0,
            k: 1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// thermal conductivity

lazy_static! {
    pub static ref WATTS_PER_METER_DEGREE_KELVIN: Unit = Unit {
        quantity: Some("thermal conductivity".to_string(),),
        ids: [
            "watts_per_meter_degree_kelvin".to_string(),
            "W/m°K".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 1,
            sec: -3,
            k: -1,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

// time

lazy_static! {
    pub static ref NANOSECOND: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["nanosecond".to_string(), "ns".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-9,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MICROSECOND: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["microsecond".to_string(), "µs".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLISECOND: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["millisecond".to_string(), "ms".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HUNDREDTHS_SECOND: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["hundredths_second".to_string(), "cs".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.01,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TENTHS_SECOND: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["tenths_second".to_string(), "ds".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.1,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref SECOND: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["second".to_string(), "sec".to_string(), "s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MINUTE: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["minute".to_string(), "min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 60.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HOUR: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["hour".to_string(), "hr".to_string(), "h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3600.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref DAY: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["day".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 86400.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref WEEK: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["week".to_string(), "wk".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 604800.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref JULIAN_MONTH: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["julian_month".to_string(), "mo".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2629800.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref YEAR: Unit = Unit {
        quantity: Some("time".to_string(),),
        ids: ["year".to_string(), "yr".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 0,
            sec: 1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 31536000.0,
        offset: 0.0,
    };
}

// velocity

lazy_static! {
    pub static ref METERS_PER_SECOND: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["meters_per_second".to_string(), "m/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOMETERS_PER_SECOND: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["kilometers_per_second".to_string(), "km/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOMETERS_PER_HOUR: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["kilometers_per_hour".to_string(), "km/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.277778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILES_PER_HOUR: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["miles_per_hour".to_string(), "mph".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.447027,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FEET_PER_SECOND: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["feet_per_second".to_string(), "ft/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.3048,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FEET_PER_MINUTE: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["feet_per_minute".to_string(), "ft/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.00508,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIMETERS_PER_SECOND: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["millimeters_per_second".to_string(), "mm/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLIMETERS_PER_MINUTE: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["millimeters_per_minute".to_string(), "mm/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.6666666666666667e-5,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref METERS_PER_MINUTE: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["meters_per_minute".to_string(), "m/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.016666666666666666,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref METERS_PER_HOUR: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["meters_per_hour".to_string(), "m/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0002777777777777778,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KNOT: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["knot".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.5144,
        offset: 0.0,
    };
}

// volume

lazy_static! {
    pub static ref CUBIC_METER: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["cubic_meter".to_string(), "m³".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_MILLIMETER: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["cubic_millimeter".to_string(), "mm³".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-9,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_CENTIMETER: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["cubic_centimeter".to_string(), "cm³".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLILITER: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["milliliter".to_string(), "mL".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HECTOLITER: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["hectoliter".to_string(), "hL".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.1,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LITER: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["liter".to_string(), "L".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOLITER: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["kiloliter".to_string(), "kL".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_INCH: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["cubic_inch".to_string(), "in³".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.6387064e-5,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_FOOT: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["cubic_foot".to_string(), "ft³".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.028317,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_YARD: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["cubic_yard".to_string(), "yd³".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.764555,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GALLON: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["gallon".to_string(), "gal".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.003785,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOGALLON: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["kilogallon".to_string(), "kgal".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 3.785,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref QUART: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["quart".to_string(), "qt".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.000946,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PINT: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["pint".to_string(), "pt".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.000473,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref FLUID_OUNCE: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["fluid_ounce".to_string(), "fl_oz".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2.95729e-5,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref IMPERIAL_GALLON: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["imperial_gallon".to_string(), "galUK".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.004546092,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HECTO_CUBIC_FOOT: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["hecto_cubic_foot".to_string(), "hft³".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2.8317,
        offset: 0.0,
    };
}

// volumetric flow

lazy_static! {
    pub static ref CUBIC_METERS_PER_SECOND: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["cubic_meters_per_second".to_string(), "m³/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLILITERS_PER_SECOND: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["milliliters_per_second".to_string(), "mL/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HECTOLITERS_PER_SECOND: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["hectoliters_per_second".to_string(), "hL/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.1,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LITERS_PER_SECOND: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["liters_per_second".to_string(), "L/s".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_FEET_PER_SECOND: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["cubic_feet_per_second".to_string(), "cfs".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.028317,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_FEET_PER_MINUTE: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["cubic_feet_per_minute".to_string(), "cfm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0004719474432,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_FEET_PER_HOUR: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["cubic_feet_per_hour".to_string(), "cfh".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 7.866e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOCUBIC_FEET_PER_MINUTE: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["kilocubic_feet_per_minute".to_string(), "kcfm".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.4719474432,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref IMPERIAL_GALLONS_PER_MINUTE: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: [
            "imperial_gallons_per_minute".to_string(),
            "galUK/min".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.004546092,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LITERS_PER_MINUTE: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["liters_per_minute".to_string(), "L/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.6666666666666667e-5,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GALLONS_PER_MINUTE: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["gallons_per_minute".to_string(), "gal/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 6.30901964e-5,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GALLONS_PER_HOUR: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: [
            "gallons_per_hour".to_string(),
            "gal/hr".to_string(),
            "gph".to_string(),
        ]
        .to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 1.0515033e-6,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref LITERS_PER_HOUR: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["liters_per_hour".to_string(), "L/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2.7777777777777776e-7,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_METERS_PER_MINUTE: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["cubic_meters_per_minute".to_string(), "m³/min".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.016666666666666666,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CUBIC_METERS_PER_HOUR: Unit = Unit {
        quantity: Some("volumetric flow".to_string(),),
        ids: ["cubic_meters_per_hour".to_string(), "m³/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 0.0002777777777777778,
        offset: 0.0,
    };
}

// bytes

lazy_static! {
    pub static ref BYTE: Unit = Unit {
        quantity: Some("bytes".to_string(),),
        ids: ["byte".to_string(),].to_vec(),
        dimensions: None,
        scale: 1.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOBYTE: Unit = Unit {
        quantity: Some("bytes".to_string(),),
        ids: ["kilobyte".to_string(), "kB".to_string(),].to_vec(),
        dimensions: None,
        scale: 1024.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MEGABYTE: Unit = Unit {
        quantity: Some("bytes".to_string(),),
        ids: ["megabyte".to_string(), "MB".to_string(),].to_vec(),
        dimensions: None,
        scale: 1048576.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref GIGABYTE: Unit = Unit {
        quantity: Some("bytes".to_string(),),
        ids: ["gigabyte".to_string(), "GB".to_string(),].to_vec(),
        dimensions: None,
        scale: 1073741824.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref TERABYTE: Unit = Unit {
        quantity: Some("bytes".to_string(),),
        ids: ["terabyte".to_string(), "TB".to_string(),].to_vec(),
        dimensions: None,
        scale: 1099511627776.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PETABYTE: Unit = Unit {
        quantity: Some("bytes".to_string(),),
        ids: ["petabyte".to_string(), "PB".to_string(),].to_vec(),
        dimensions: None,
        scale: 1125899906842624.0,
        offset: 0.0,
    };
}
lazy_static! {
    pub static ref UNITS: HashMap<&'static str, &'static Unit> = [
        ("kW/gal/min", &*KILOWATTS_PER_GALLONS_PER_MINUTE),
        ("milligram", &*MILLIGRAM),
        ("hecto_cubic_foot", &*HECTO_CUBIC_FOOT),
        ("lumen", &*LUMEN),
        ("MW", &*MEGAWATT),
        ("psi", &*POUNDS_PER_SQUARE_INCH),
        ("L", &*LITER),
        ("UAH", &*HRYVNIA),
        ("kHz", &*KILOHERTZ),
        ("megohm", &*MEGOHM),
        ("J/kg", &*JOULES_PER_KILOGRAM),
        ("MHz", &*MEGAHERTZ),
        (
            "kilovolt_ampere_reactive_hour",
            &*KILOVOLT_AMPERE_REACTIVE_HOUR
        ),
        ("galUK/min", &*IMPERIAL_GALLONS_PER_MINUTE),
        ("petabyte", &*PETABYTE),
        ("scf", &*CUBIC_FEET_NATURAL_GAS),
        ("m", &*METER),
        ("meters_per_hour", &*METERS_PER_HOUR),
        ("MBTU/ft²", &*MEGABTU_PER_SQUARE_FOOT),
        ("MVARh", &*MEGAVOLT_AMPERE_REACTIVE_HOUR),
        ("joules_per_degree_kelvin", &*JOULES_PER_DEGREE_KELVIN),
        ("saudi_riyal", &*SAUDI_RIYAL),
        (
            "megajoules_per_degree_kelvin",
            &*MEGAJOULES_PER_DEGREE_KELVIN
        ),
        ("kilovolt_ampere", &*KILOVOLT_AMPERE),
        ("centimeters_of_water", &*CENTIMETERS_OF_WATER),
        ("AOA", &*KWANZA),
        ("kΩ", &*KILOHM),
        ("SFr", &*SWISS_FRANC),
        ("lux", &*LUX),
        (
            "joules_per_kilogram_degree_kelvin",
            &*JOULES_PER_KILOGRAM_DEGREE_KELVIN
        ),
        ("degrees_fahrenheit_per_hour", &*DEGREES_FAHRENHEIT_PER_HOUR),
        ("HNL", &*LEMPIRA),
        ("cape_verde_escudo", &*CAPE_VERDE_ESCUDO),
        ("s", &*SECOND),
        ("ft³", &*CUBIC_FOOT),
        ("CDF", &*CONGOLESE_FRANC),
        ("GMD", &*DALASI),
        ("₲", &*GUARANI),
        ("moldavian_leu", &*MOLDAVIAN_LEU),
        ("formazin_nephelometric_unit", &*FORMAZIN_NEPHELOMETRIC_UNIT),
        ("bahraini_dinar", &*BAHRAINI_DINAR),
        ("MJ/ft²", &*MEGAJOULES_PER_SQUARE_FOOT),
        ("tenths_second", &*TENTHS_SECOND),
        ("tala", &*TALA),
        ("GJ", &*GIGAJOULE),
        ("kW/m²", &*KILOWATTS_PER_SQUARE_METER),
        ("costa_rican_colon", &*COSTA_RICAN_COLON),
        ("volts_per_meter", &*VOLTS_PER_METER),
        ("h", &*HOUR),
        ("singapore_dollar", &*SINGAPORE_DOLLAR),
        ("lb", &*POUND),
        ("gram", &*GRAM),
        ("NIO", &*CORDOBA_ORO),
        ("PHP", &*PHILIPPINE_PESO),
        ("MDL", &*MOLDAVIAN_LEU),
        ("czech_koruna", &*CZECH_KORUNA),
        (
            "percent_obscuration_per_meter",
            &*PERCENT_OBSCURATION_PER_METER
        ),
        (
            "megawatt_hours_per_square_meter",
            &*MEGAWATT_HOURS_PER_SQUARE_METER
        ),
        ("ft³_gas", &*CUBIC_FEET_NATURAL_GAS),
        ("UGX", &*UGANDA_SHILLING),
        ("m³_gas", &*CUBIC_METERS_NATURAL_GAS),
        ("W/m²_irr", &*WATTS_PER_SQUARE_METER_IRRADIANCE),
        ("fc", &*FOOTCANDLE),
        ("taiwan_dollar", &*TAIWAN_DOLLAR),
        ("swiss_franc", &*SWISS_FRANC),
        ("millimeters_of_mercury", &*MILLIMETERS_OF_MERCURY),
        ("Vt", &*VATU),
        ("m³/h", &*CUBIC_METERS_PER_HOUR),
        ("%RH", &*PERCENT_RELATIVE_HUMIDITY),
        ("W", &*WATT),
        ("megawatt", &*MEGAWATT),
        ("cubic_feet_per_minute", &*CUBIC_FEET_PER_MINUTE),
        ("MV", &*MEGAVOLT),
        ("kiloliter", &*KILOLITER),
        ("btu", &*BTU),
        ("XAF", &*CFA_FRANC_BCEAO),
        ("µg/m³", &*MICROGRAMS_PER_CUBIC_METER),
        ("ohm_meter", &*OHM_METER),
        ("tunisian_dinar", &*TUNISIAN_DINAR),
        ("PLN", &*PZLOTY),
        ("klb/h", &*KILOPOUNDS_PER_HOUR),
        ("kelvin_degrees", &*KELVIN_DEGREES),
        ("AFN", &*AFGHANI),
        ("newton", &*NEWTON),
        ("Wh/m²", &*WATT_HOURS_PER_SQUARE_METER),
        ("PAB", &*BALBOA),
        ("amperes_per_square_meter", &*AMPERES_PER_SQUARE_METER),
        ("GYD", &*GUYANA_DOLLAR),
        ("burundi_franc", &*BURUNDI_FRANC),
        ("UZS", &*UZBEKISTAN_SUM),
        ("meter", &*METER),
        ("gH₂O/kgAir", &*GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR),
        ("metical", &*METICAL),
        ("armenian_dram", &*ARMENIAN_DRAM),
        ("rial_omani", &*RIAL_OMANI),
        ("EUR", &*EURO),
        ("cubic_foot", &*CUBIC_FOOT),
        ("mΩ", &*MILLIOHM),
        ("BTU/h", &*BTUS_PER_HOUR),
        ("F", &*FARAD),
        ("°C/min", &*DEGREES_CELSIUS_PER_MINUTE),
        ("SEK", &*SWEDISH_KRONA),
        ("guinea_franc", &*GUINEA_FRANC),
        ("MMBTU/h", &*MEGABTUS_PER_HOUR),
        ("kJ/kg", &*KILOJOULES_PER_KILOGRAM),
        ("kVARh", &*KILOVOLT_AMPERE_REACTIVE_HOUR),
        ("DOP", &*DOMINICAN_PESO),
        ("GW", &*GIGAWATT),
        ("pzloty", &*PZLOTY),
        ("fahrenheit", &*FAHRENHEIT),
        ("UM", &*OUGUIYA),
        ("°F/h", &*DEGREES_FAHRENHEIT_PER_HOUR),
        ("IQD", &*IRAQI_DINAR),
        ("fahrenheit_degrees", &*FAHRENHEIT_DEGREES),
        ("per_second", &*PER_SECOND),
        ("liters_per_second", &*LITERS_PER_SECOND),
        ("m/s²", &*METERS_PER_SECOND_SQUARED),
        ("belize_dollar", &*BELIZE_DOLLAR),
        ("canadian_dollar", &*CANADIAN_DOLLAR),
        ("hectopascal", &*HECTOPASCAL),
        ("VA", &*VOLT_AMPERE),
        ("kBTU/ft²", &*KILOBTU_PER_SQUARE_FOOT),
        ("lek", &*LEK),
        ("$", &*US_DOLLAR),
        ("°C", &*CELSIUS),
        ("mg/m³", &*MILLIGRAMS_PER_CUBIC_METER),
        ("AWG", &*ARUBAN_GUILDER_FLORIN),
        ("philippine_peso", &*PHILIPPINE_PESO),
        ("tenge", &*TENGE),
        ("cfs", &*CUBIC_FEET_PER_SECOND),
        ("mile", &*MILE),
        ("decibel", &*DECIBEL),
        ("yd", &*YARD),
        ("BHD", &*BAHRAINI_DINAR),
        ("MWK", &*KWACHA),
        (
            "thousand_cubic_feet_natural_gas",
            &*THOUSAND_CUBIC_FEET_NATURAL_GAS
        ),
        ("percent_per_second", &*PERCENT_PER_SECOND),
        ("GTQ", &*QUETZAL),
        ("pataca", &*PATACA),
        ("RWF", &*RWANDA_FRANC),
        ("db", &*DECIBEL),
        ("degrees_kelvin_per_hour", &*DEGREES_KELVIN_PER_HOUR),
        ("btu/lb_dry", &*BTUS_PER_POUND_DRY_AIR),
        ("ERN", &*NAKFA),
        ("mph", &*MILES_PER_HOUR),
        ("kenyan_shilling", &*KENYAN_SHILLING),
        ("ARS", &*ARGENTINE_PESO),
        ("kyat", &*KYAT),
        ("₦", &*NAIRA),
        ("N", &*NEWTON),
        ("TZS", &*TANZANIAN_SHILLING),
        ("RUB", &*RUSSIAN_RUBLE),
        ("MWh/ft²", &*MEGAWATT_HOURS_PER_SQUARE_FOOT),
        ("cm", &*CENTIMETER),
        ("H", &*HENRY),
        ("milliwatt", &*MILLIWATT),
        ("mo", &*JULIAN_MONTH),
        ("square_kilometer", &*SQUARE_KILOMETER),
        ("KPW", &*NORTH_KOREAN_WON),
        ("lm", &*LUMEN),
        ("cedi", &*CEDI),
        ("VND", &*DONG),
        ("SYP", &*SYRIAN_POUND),
        ("volt_ampere", &*VOLT_AMPERE),
        ("cordoba_oro", &*CORDOBA_ORO),
        ("MTn", &*METICAL),
        ("hryvnia", &*HRYVNIA),
        ("congolese_franc", &*CONGOLESE_FRANC),
        ("gallons_per_minute", &*GALLONS_PER_MINUTE),
        ("micrometer", &*MICROMETER),
        ("centimeters_of_mercury", &*CENTIMETERS_OF_MERCURY),
        ("kL", &*KILOLITER),
        ("denar", &*DENAR),
        ("kilovolt_ampere_reactive", &*KILOVOLT_AMPERE_REACTIVE),
        ("kwanza", &*KWANZA),
        ("mL/s", &*MILLILITERS_PER_SECOND),
        ("GHS", &*CEDI),
        ("grams_per_square_meter", &*GRAMS_PER_SQUARE_METER),
        ("NOK", &*NORWEGIAN_KRONE),
        ("¥", &*YEN),
        ("kilobtu_per_square_foot", &*KILOBTU_PER_SQUARE_FOOT),
        (
            "degrees_fahrenheit_per_minute",
            &*DEGREES_FAHRENHEIT_PER_MINUTE
        ),
        ("joules_per_hour", &*JOULES_PER_HOUR),
        ("m³/min", &*CUBIC_METERS_PER_MINUTE),
        ("kilogallon", &*KILOGALLON),
        ("kilometers_per_hour", &*KILOMETERS_PER_HOUR),
        ("kWh", &*KILOWATT_HOUR),
        ("psi/°F", &*PSI_PER_DEGREE_FAHRENHEIT),
        ("kBTU", &*KILOBTU),
        ("K/min", &*DEGREES_KELVIN_PER_MINUTE),
        ("Ω", &*OHM),
        ("cubic_feet_natural_gas", &*CUBIC_FEET_NATURAL_GAS),
        ("ohm", &*OHM),
        ("ampere", &*AMPERE),
        ("g/m²", &*GRAMS_PER_SQUARE_METER),
        ("µm", &*MICROMETER),
        ("TWD", &*TAIWAN_DOLLAR),
        ("GJ/h", &*GIGAJOULES_PER_HOUR),
        ("inches_of_mercury", &*INCHES_OF_MERCURY),
        ("kilograms_per_cubic_meter", &*KILOGRAMS_PER_CUBIC_METER),
        ("lb/s", &*POUNDS_PER_SECOND),
        ("MMBTU", &*MEGABTU),
        ("acre", &*ACRE),
        ("kVA", &*KILOVOLT_AMPERE),
        ("Δ°F", &*FAHRENHEIT_DEGREES),
        ("pounds_per_square_inch", &*POUNDS_PER_SQUARE_INCH),
        ("CUP", &*CUBAN_PESO),
        ("som", &*SOM),
        ("yemeni_rial", &*YEMENI_RIAL),
        ("gigajoules_per_hour", &*GIGAJOULES_PER_HOUR),
        ("kuwaiti_dinar", &*KUWAITI_DINAR),
        ("tesla", &*TESLA),
        ("N/m", &*NEWTONS_PER_METER),
        ("Kč", &*CZECH_KORUNA),
        ("namibia_dollar", &*NAMIBIA_DOLLAR),
        ("MVAR", &*MEGAVOLT_AMPERE_REACTIVE),
        ("megabyte", &*MEGABYTE),
        ("cubic_meters_per_minute", &*CUBIC_METERS_PER_MINUTE),
        ("TRY", &*TURKISH_LIRA),
        ("kWh/ft²", &*KILOWATT_HOURS_PER_SQUARE_FOOT),
        ("dBmV", &*DB_MILLIVOLT),
        ("BBD", &*BARBADOS_DOLLAR),
        ("IDR", &*RUPIAH),
        ("INR", &*INDIAN_RUPEE),
        ("inch", &*INCH),
        ("yd²", &*SQUARE_YARD),
        ("hr", &*HOUR),
        ("deg", &*DEGREES_ANGULAR),
        ("somali_shilling", &*SOMALI_SHILLING),
        ("jamaican_dollar", &*JAMAICAN_DOLLAR),
        ("J/m²", &*JOULES_PER_SQUARE_METER),
        ("radians_per_second", &*RADIANS_PER_SECOND),
        ("MJ/°K", &*MEGAJOULES_PER_DEGREE_KELVIN),
        ("BMD", &*BERMUDIAN_DOLLAR),
        ("ppm", &*PARTS_PER_MILLION),
        ("ƒ", &*ARUBAN_GUILDER_FLORIN),
        ("THB", &*BAHT),
        ("m³/s", &*CUBIC_METERS_PER_SECOND),
        ("kilowatts_per_square_foot", &*KILOWATTS_PER_SQUARE_FOOT),
        ("PGK", &*KINA),
        ("°F", &*FAHRENHEIT),
        ("Ωm", &*OHM_METER),
        ("LYD", &*LIBYAN_DINAR),
        ("kilogram", &*KILOGRAM),
        ("watts_per_square_meter", &*WATTS_PER_SQUARE_METER),
        ("L/min", &*LITERS_PER_MINUTE),
        ("dBµV", &*DB_MICROVOLT),
        ("dominican_peso", &*DOMINICAN_PESO),
        ("manat", &*MANAT),
        ("cfa_franc_bceao", &*CFA_FRANC_BCEAO),
        ("gph", &*GALLONS_PER_HOUR),
        ("SOS", &*SOMALI_SHILLING),
        ("lari", &*LARI),
        (
            "kilowatts_per_kilocubic_feet_per_minute",
            &*KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE
        ),
        ("grams_per_kilogram", &*GRAMS_PER_KILOGRAM),
        ("sr", &*STERADIAN),
        ("afghani", &*AFGHANI),
        ("W/m°K", &*WATTS_PER_METER_DEGREE_KELVIN),
        ("cubic_millimeter", &*CUBIC_MILLIMETER),
        ("J/°K", &*JOULES_PER_DEGREE_KELVIN),
        ("kg/s", &*KILOGRAMS_PER_SECOND),
        ("south_korean_won", &*SOUTH_KOREAN_WON),
        ("hL", &*HECTOLITER),
        ("EGP", &*EGYPTIAN_POUND),
        ("forint", &*FORINT),
        ("horsepower", &*HORSEPOWER),
        ("Af", &*AFGHANI),
        ("MUR", &*MAURITIUS_RUPEE),
        ("ton/h", &*METRIC_TONS_PER_HOUR),
        ("kBTU/h", &*KILOBTUS_PER_HOUR),
        ("nuevo_sol", &*NUEVO_SOL),
        ("g/min", &*GRAMS_PER_MINUTE),
        ("gal/min", &*GALLONS_PER_MINUTE),
        ("GWh", &*GIGAWATT_HOUR),
        ("week", &*WEEK),
        ("liters_per_hour", &*LITERS_PER_HOUR),
        ("inches_of_water", &*INCHES_OF_WATER),
        ("milliohm", &*MILLIOHM),
        ("C", &*COULOMB),
        ("AMD", &*ARMENIAN_DRAM),
        ("power_factor", &*POWER_FACTOR),
        ("mm²", &*SQUARE_MILLIMETER),
        ("JOD", &*JORDANIAN_DINAR),
        ("new_zealand_dollar", &*NEW_ZEALAND_DOLLAR),
        ("in³", &*CUBIC_INCH),
        ("JPY", &*YEN),
        ("kilopound", &*KILOPOUND),
        ("cs", &*HUNDREDTHS_SECOND),
        ("parts_per_billion", &*PARTS_PER_BILLION),
        ("J/kg_dry", &*JOULES_PER_KILOGRAM_DRY_AIR),
        ("Ωm", &*OHM_METER),
        ("short_ton", &*SHORT_TON),
        ("millimeters_per_minute", &*MILLIMETERS_PER_MINUTE),
        ("moroccan_dirham", &*MOROCCAN_DIRHAM),
        ("watt_hours_per_square_meter", &*WATT_HOURS_PER_SQUARE_METER),
        ("ден", &*DENAR),
        ("foot_pounds_per_second", &*FOOT_POUNDS_PER_SECOND),
        ("cmHg", &*CENTIMETERS_OF_MERCURY),
        ("guyana_dollar", &*GUYANA_DOLLAR),
        ("JMD", &*JAMAICAN_DOLLAR),
        ("kwacha", &*KWACHA),
        ("MJ", &*MEGAJOULE),
        ("fnu", &*FORMAZIN_NEPHELOMETRIC_UNIT),
        ("LRD", &*LIBERIAN_DOLLAR),
        ("%", &*PERCENT),
        ("degrees_phase", &*DEGREES_PHASE),
        ("candelas_per_square_meter", &*CANDELAS_PER_SQUARE_METER),
        ("MB", &*MEGABYTE),
        ("pH", &*PH),
        ("cuban_peso", &*CUBAN_PESO),
        ("HRK", &*CROATIAN_KUNA),
        ("Volt", &*VOLT),
        ("second", &*SECOND),
        ("DCIE", &*DATA_CENTER_INFRASTRUCTURE_EFFICIENCY),
        ("ман", &*AZERBAIJANIAN_MANAT),
        ("hL/s", &*HECTOLITERS_PER_SECOND),
        ("gigawatt", &*GIGAWATT),
        ("₡", &*COSTA_RICAN_COLON),
        ("algerian_dinar", &*ALGERIAN_DINAR),
        (
            "percent_obscuration_per_foot",
            &*PERCENT_OBSCURATION_PER_FOOT
        ),
        ("kilograms_per_minute", &*KILOGRAMS_PER_MINUTE),
        ("Am²", &*AMPERE_SQUARE_METER),
        ("falkland_islands_pound", &*FALKLAND_ISLANDS_POUND),
        ("₮", &*TUGRIK),
        ("KWD", &*KUWAITI_DINAR),
        ("J", &*JOULE),
        ("candela", &*CANDELA),
        ("mg", &*MILLIGRAM),
        ("Ω", &*OHM),
        ("tons_refrigeration", &*TONS_REFRIGERATION),
        ("MVAh", &*MEGAVOLT_AMPERE_HOUR),
        ("kJ/h", &*KILOJOULES_PER_HOUR),
        ("NZD", &*NEW_ZEALAND_DOLLAR),
        ("kelvin", &*KELVIN),
        ("percent_relative_humidity", &*PERCENT_RELATIVE_HUMIDITY),
        ("cd/m²", &*CANDELAS_PER_SQUARE_METER),
        ("kilowatt", &*KILOWATT),
        ("nakfa", &*NAKFA),
        ("pascal", &*PASCAL),
        ("cubic_meters_per_second", &*CUBIC_METERS_PER_SECOND),
        ("A/m", &*AMPERES_PER_METER),
        ("per_minute", &*PER_MINUTE),
        ("bulgarian_lev", &*BULGARIAN_LEV),
        ("siemens_per_meter", &*SIEMENS_PER_METER),
        ("meters_per_second", &*METERS_PER_SECOND),
        ("chinese_yuan", &*CHINESE_YUAN),
        ("joule_second", &*JOULE_SECOND),
        ("GIP", &*GIBRALTAR_POUND),
        ("jordanian_dinar", &*JORDANIAN_DINAR),
        ("ILS", &*NEW_ISRAELI_SHEKEL),
        ("ACH", &*AIR_CHANGES_PER_HOUR),
        ("fiji_dollar", &*FIJI_DOLLAR),
        ("BAM", &*KONVERTIBILNA_MARKA),
        ("Wh", &*WATT_HOUR),
        ("K/h", &*DEGREES_KELVIN_PER_HOUR),
        ("gal", &*GALLON),
        ("g/s", &*GRAMS_PER_SECOND),
        ("kilojoules_per_hour", &*KILOJOULES_PER_HOUR),
        ("lebanese_pound", &*LEBANESE_POUND),
        (
            "kilowatt_hours_per_square_meter",
            &*KILOWATT_HOURS_PER_SQUARE_METER
        ),
        ("iranian_rial", &*IRANIAN_RIAL),
        ("btus_per_hour", &*BTUS_PER_HOUR),
        (
            "grams_of_water_per_kilogram_dry_air",
            &*GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR
        ),
        ("kilograms_per_hour", &*KILOGRAMS_PER_HOUR),
        ("metric_tons_per_hour", &*METRIC_TONS_PER_HOUR),
        ("guarani", &*GUARANI),
        ("swedish_krona", &*SWEDISH_KRONA),
        ("mm", &*MILLIMETER),
        ("R$", &*BRAZILIAN_REAL),
        ("coefficient_of_performance", &*COEFFICIENT_OF_PERFORMANCE),
        ("atm", &*ATMOSPHERE),
        ("pounds_per_minute", &*POUNDS_PER_MINUTE),
        ("milliliter", &*MILLILITER),
        ("lb/h", &*POUNDS_PER_HOUR),
        ("leone", &*LEONE),
        ("btus_per_pound_dry_air", &*BTUS_PER_POUND_DRY_AIR),
        ("LSL", &*LOTI),
        ("SDG", &*SUDANESE_POUND),
        ("VAR", &*VOLT_AMPERE_REACTIVE),
        ("degrees_kelvin_per_second", &*DEGREES_KELVIN_PER_SECOND),
        ("bahamian_dollar", &*BAHAMIAN_DOLLAR),
        ("BRL", &*BRAZILIAN_REAL),
        ("dalasi", &*DALASI),
        ("BWP", &*PULA),
        ("konvertibilna_marka", &*KONVERTIBILNA_MARKA),
        ("kr", &*DANISH_KRONE),
        ("hertz", &*HERTZ),
        ("square_mile", &*SQUARE_MILE),
        ("CZK", &*CZECH_KORUNA),
        ("cycles_per_hour", &*CYCLES_PER_HOUR),
        ("MZN", &*METICAL),
        ("BIF", &*BURUNDI_FRANC),
        (
            "hundred_cubic_feet_natural_gas",
            &*HUNDRED_CUBIC_FEET_NATURAL_GAS
        ),
        (
            "million_cubic_feet_natural_gas",
            &*MILLION_CUBIC_FEET_NATURAL_GAS
        ),
        ("megavolt_ampere_reactive", &*MEGAVOLT_AMPERE_REACTIVE),
        ("cph", &*CYCLES_PER_HOUR),
        ("Btu/Wh", &*ENERGY_EFFICIENCY_RATIO),
        ("percent", &*PERCENT),
        ("g/kg", &*GRAMS_PER_KILOGRAM),
        ("kg/m³", &*KILOGRAMS_PER_CUBIC_METER),
        ("₱", &*PHILIPPINE_PESO),
        ("kW/kcfm", &*KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE),
        ("minute", &*MINUTE),
        ("m/s", &*METERS_PER_SECOND),
        (
            "watts_per_square_foot_irradiance",
            &*WATTS_PER_SQUARE_FOOT_IRRADIANCE
        ),
        ("ΔK", &*KELVIN_DEGREES),
        ("VARh", &*VOLT_AMPERE_REACTIVE_HOUR),
        ("air_changes_per_hour", &*AIR_CHANGES_PER_HOUR),
        ("lilangeni", &*LILANGENI),
        ("brazilian_real", &*BRAZILIAN_REAL),
        ("HKD", &*HONG_KONG_DOLLAR),
        ("baht", &*BAHT),
        ("newtons_per_meter", &*NEWTONS_PER_METER),
        ("degrees_kelvin_per_minute", &*DEGREES_KELVIN_PER_MINUTE),
        ("cd/ft²", &*CANDELS_PER_SQUARE_FOOT),
        ("zł", &*PZLOTY),
        ("mA", &*MILLIAMPERE),
        ("in²", &*SQUARE_INCH),
        ("croatian_kuna", &*CROATIAN_KUNA),
        ("MΩ", &*MEGOHM),
        ("VAh", &*VOLT_AMPERE_HOUR),
        ("IRR", &*IRANIAN_RIAL),
        ("pixel", &*PIXEL),
        ("Wb", &*WEBER),
        ("north_korean_won", &*NORTH_KOREAN_WON),
        ("micrograms_per_cubic_meter", &*MICROGRAMS_PER_CUBIC_METER),
        ("ouguiya", &*OUGUIYA),
        ("riel", &*RIEL),
        ("km/h", &*KILOMETERS_PER_HOUR),
        ("inH₂O", &*INCHES_OF_WATER),
        ("ethiopian_birr", &*ETHIOPIAN_BIRR),
        ("east_caribbean_dollar", &*EAST_CARIBBEAN_DOLLAR),
        ("horsepower_hour", &*HORSEPOWER_HOUR),
        ("zambian_kwacha", &*ZAMBIAN_KWACHA),
        ("kilovolt_ampere_hour", &*KILOVOLT_AMPERE_HOUR),
        ("kBTU/h/ft²", &*KILOBTUS_PER_HOUR_PER_SQUARE_FOOT),
        ("db_millivolt", &*DB_MILLIVOLT),
        ("taka", &*TAKA),
        ("KGS", &*SOM),
        ("kgal", &*KILOGALLON),
        ("rpm", &*REVOLUTIONS_PER_MINUTE),
        (
            "watts_per_meter_degree_kelvin",
            &*WATTS_PER_METER_DEGREE_KELVIN
        ),
        ("bar", &*BAR),
        ("₵", &*CEDI),
        ("g", &*GRAM),
        ("radian", &*RADIAN),
        ("joules_per_kilogram_dry_air", &*JOULES_PER_KILOGRAM_DRY_AIR),
        ("kPa", &*KILOPASCAL),
        ("pint", &*PINT),
        ("iceland_krona", &*ICELAND_KRONA),
        ("A", &*AMPERE),
        ("W/m²K", &*WATTS_PER_SQUARE_METER_DEGREE_KELVIN),
        ("gibraltar_pound", &*GIBRALTAR_POUND),
        ("SZL", &*LILANGENI),
        ("watt_hour", &*WATT_HOUR),
        (
            "watts_per_square_meter_irradiance",
            &*WATTS_PER_SQUARE_METER_IRRADIANCE
        ),
        ("mL", &*MILLILITER),
        ("GEL", &*LARI),
        ("hph", &*HORSEPOWER_HOUR),
        ("K/s", &*DEGREES_KELVIN_PER_SECOND),
        ("BZD", &*BELIZE_DOLLAR),
        ("malaysian_ringgit", &*MALAYSIAN_RINGGIT),
        ("suriname_dollar", &*SURINAME_DOLLAR),
        ("phot", &*PHOT),
        ("km/s", &*KILOMETERS_PER_SECOND),
        ("ft/s", &*FEET_PER_SECOND),
        ("dong", &*DONG),
        ("pula", &*PULA),
        ("ZWL", &*ZIMBABWE_DOLLAR),
        ("/min", &*PER_MINUTE),
        ("kΩ", &*KILOHM),
        ("C$", &*CORDOBA_ORO),
        ("gal/hr", &*GALLONS_PER_HOUR),
        ("kilovolt", &*KILOVOLT),
        ("°daysC", &*DEGREE_DAYS_CELSIUS),
        ("wk", &*WEEK),
        ("КМ", &*KONVERTIBILNA_MARKA),
        ("BOB", &*BOLIVIANO),
        ("BDT", &*TAKA),
        ("kilometers_per_second", &*KILOMETERS_PER_SECOND),
        ("MNT", &*TUGRIK),
        ("kW/ft²", &*KILOWATTS_PER_SQUARE_FOOT),
        ("mm/s", &*MILLIMETERS_PER_SECOND),
        ("sri_lanka_rupee", &*SRI_LANKA_RUPEE),
        ("GNF", &*GUINEA_FRANC),
        ("volts_per_degree_kelvin", &*VOLTS_PER_DEGREE_KELVIN),
        ("mexican_peso", &*MEXICAN_PESO),
        ("Wh/ft²", &*WATT_HOURS_PER_SQUARE_FOOT),
        ("megajoules_per_square_meter", &*MEGAJOULES_PER_SQUARE_METER),
        ("pakistan_rupee", &*PAKISTAN_RUPEE),
        ("syrian_pound", &*SYRIAN_POUND),
        ("tugrik", &*TUGRIK),
        ("YER", &*YEMENI_RIAL),
        ("kJ/°K", &*KILOJOULES_PER_DEGREE_KELVIN),
        ("°F/min", &*DEGREES_FAHRENHEIT_PER_MINUTE),
        ("bolivar_fuerte", &*BOLIVAR_FUERTE),
        ("kilopascal", &*KILOPASCAL),
        ("J/kg°K", &*JOULES_PER_KILOGRAM_DEGREE_KELVIN),
        ("ISK", &*ICELAND_KRONA),
        ("cpm", &*CYCLES_PER_MINUTE),
        ("rwanda_franc", &*RWANDA_FRANC),
        ("celsius", &*CELSIUS),
        ("tanzanian_shilling", &*TANZANIAN_SHILLING),
        ("inHg", &*INCHES_OF_MERCURY),
        ("HTG", &*GOURDE),
        ("kVAR", &*KILOVOLT_AMPERE_REACTIVE),
        ("volt_ampere_reactive", &*VOLT_AMPERE_REACTIVE),
        ("serbian_dinar", &*SERBIAN_DINAR),
        ("COP", &*COEFFICIENT_OF_PERFORMANCE),
        ("millimeter", &*MILLIMETER),
        ("kg", &*KILOGRAM),
        (
            "watts_per_cubic_meters_per_second",
            &*WATTS_PER_CUBIC_METERS_PER_SECOND
        ),
        ("pound", &*POUND),
        ("CAD", &*CANADIAN_DOLLAR),
        ("SCR", &*SEYCHELLES_RUPEE),
        ("৳", &*TAKA),
        ("mV", &*MILLIVOLT),
        (
            "megawatt_hours_per_square_foot",
            &*MEGAWATT_HOURS_PER_SQUARE_FOOT
        ),
        ("%/s", &*PERCENT_PER_SECOND),
        ("RON", &*LEU),
        (
            "megavolt_ampere_reactive_hour",
            &*MEGAVOLT_AMPERE_REACTIVE_HOUR
        ),
        ("cal", &*CALORIE),
        ("KYD", &*CAYMAN_ISLANDS_DOLLAR),
        ("in/wc", &*INCHES_OF_WATER),
        ("imperial_gallon", &*IMPERIAL_GALLON),
        ("rand", &*RAND),
        ("megawatt_hour", &*MEGAWATT_HOUR),
        ("Db", &*DOBRA),
        ("newton_second", &*NEWTON_SECOND),
        ("SGD", &*SINGAPORE_DOLLAR),
        ("kilojoules_per_kilogram", &*KILOJOULES_PER_KILOGRAM),
        ("lempira", &*LEMPIRA),
        ("µs", &*MICROSECOND),
        ("square_meter", &*SQUARE_METER),
        ("J/g", &*JOULES_PER_GRAM),
        ("MJ/h", &*MEGAJOULES_PER_HOUR),
        ("milliliters_per_second", &*MILLILITERS_PER_SECOND),
        ("sec", &*SECOND),
        ("Sh", &*KENYAN_SHILLING),
        ("argentine_peso", &*ARGENTINE_PESO),
        ("qatari_rial", &*QATARI_RIAL),
        ("cubic_meters_natural_gas", &*CUBIC_METERS_NATURAL_GAS),
        ("cubic_meters_per_hour", &*CUBIC_METERS_PER_HOUR),
        ("MOP", &*PATACA),
        ("uganda_shilling", &*UGANDA_SHILLING),
        ("W/m³/s", &*WATTS_PER_CUBIC_METERS_PER_SECOND),
        ("Pa", &*PASCAL),
        ("volt_ampere_hour", &*VOLT_AMPERE_HOUR),
        ("malagasy_ariary", &*MALAGASY_ARIARY),
        ("TJS", &*SOMONI),
        ("oz", &*OUNCE),
        ("Ah", &*AMPERE_HOUR),
        ("cubic_inch", &*CUBIC_INCH),
        ("MWh/m²", &*MEGAWATT_HOURS_PER_SQUARE_METER),
        ("QAR", &*QATARI_RIAL),
        ("mW", &*MILLIWATT),
        ("cd", &*CANDELA),
        ("MRO", &*OUGUIYA),
        ("m²", &*SQUARE_METER),
        ("LAK", &*KIP),
        ("MK", &*KWACHA),
        ("hPa", &*HECTOPASCAL),
        ("ampere_hour", &*AMPERE_HOUR),
        ("LKR", &*SRI_LANKA_RUPEE),
        ("T", &*TESLA),
        ("hour", &*HOUR),
        ("AUD", &*AUSTRALIAN_DOLLAR),
        ("BGN", &*BULGARIAN_LEV),
        ("TB", &*TERABYTE),
        ("kW", &*KILOWATT),
        ("megabtus_per_hour", &*MEGABTUS_PER_HOUR),
        ("dobra", &*DOBRA),
        ("cubic_meter", &*CUBIC_METER),
        ("MΩ", &*MEGOHM),
        ("cubic_centimeter", &*CUBIC_CENTIMETER),
        ("₫", &*DONG),
        ("m³", &*CUBIC_METER),
        ("coulomb", &*COULOMB),
        ("HUF", &*FORINT),
        ("£", &*POUND_STERLING),
        ("MVR", &*RUFIYAA),
        (
            "data_center_infrastructure_efficiency",
            &*DATA_CENTER_INFRASTRUCTURE_EFFICIENCY
        ),
        (
            "megajoules_per_kilogram_dry_air",
            &*MEGAJOULES_PER_KILOGRAM_DRY_AIR
        ),
        ("leu", &*LEU),
        ("joule", &*JOULE),
        ("saint_helena_pound", &*SAINT_HELENA_POUND),
        ("SRD", &*SURINAME_DOLLAR),
        ("yard", &*YARD),
        ("BND", &*BRUNEI_DOLLAR),
        ("ns", &*NANOSECOND),
        ("NGN", &*NAIRA),
        ("kilobtus_per_hour", &*KILOBTUS_PER_HOUR),
        ("weber", &*WEBER),
        ("australian_dollar", &*AUSTRALIAN_DOLLAR),
        (
            "kilojoules_per_kilogram_dry_air",
            &*KILOJOULES_PER_KILOGRAM_DRY_AIR
        ),
        ("seychelles_rupee", &*SEYCHELLES_RUPEE),
        ("MJ/m²", &*MEGAJOULES_PER_SQUARE_METER),
        (
            "kilobtus_per_hour_per_square_foot",
            &*KILOBTUS_PER_HOUR_PER_SQUARE_FOOT
        ),
        ("GB", &*GIGABYTE),
        ("BSD", &*BAHAMIAN_DOLLAR),
        ("m/min", &*METERS_PER_MINUTE),
        ("btu_per_pound", &*BTU_PER_POUND),
        ("solomon_islands_dollar", &*SOLOMON_ISLANDS_DOLLAR),
        ("₨", &*PAKISTAN_RUPEE),
        ("rufiyaa", &*RUFIYAA),
        ("yen", &*YEN),
        ("kilograms_per_second", &*KILOGRAMS_PER_SECOND),
        ("kJ", &*KILOJOULE),
        ("year", &*YEAR),
        ("gigajoule", &*GIGAJOULE),
        ("°C/h", &*DEGREES_CELSIUS_PER_HOUR),
        ("tonrefh", &*TONS_REFRIGERATION_HOUR),
        ("brunei_dollar", &*BRUNEI_DOLLAR),
        ("euro", &*EURO),
        ("degrees_angular", &*DEGREES_ANGULAR),
        ("STD", &*DOBRA),
        ("steradian", &*STERADIAN),
        ("ppu", &*PARTS_PER_UNIT),
        ("peso_uruguayo", &*PESO_URUGUAYO),
        ("MWh", &*MEGAWATT_HOUR),
        ("元", &*CHINESE_YUAN),
        ("EER", &*ENERGY_EFFICIENCY_RATIO),
        ("megajoules_per_square_foot", &*MEGAJOULES_PER_SQUARE_FOOT),
        (
            "watts_per_cubic_feet_per_minute",
            &*WATTS_PER_CUBIC_FEET_PER_MINUTE
        ),
        ("KZT", &*TENGE),
        ("radians_per_second_squared", &*RADIANS_PER_SECOND_SQUARED),
        ("ft", &*FOOT),
        ("square_foot", &*SQUARE_FOOT),
        ("pound_force", &*POUND_FORCE),
        ("ZAR", &*RAND),
        ("calorie", &*CALORIE),
        ("kWh/m²", &*KILOWATT_HOURS_PER_SQUARE_METER),
        ("BTU/lb", &*BTU_PER_POUND),
        (
            "kilowatts_per_gallons_per_minute",
            &*KILOWATTS_PER_GALLONS_PER_MINUTE
        ),
        ("pt", &*PINT),
        ("square_meters_per_newton", &*SQUARE_METERS_PER_NEWTON),
        ("TTD", &*TRINIDAD_AND_TOBAGO_DOLLAR),
        ("megajoule", &*MEGAJOULE),
        ("Le", &*LEONE),
        ("cm³", &*CUBIC_CENTIMETER),
        ("K", &*KELVIN),
        ("yd³", &*CUBIC_YARD),
        ("volt", &*VOLT),
        (
            "nephelometric_turbidity_units",
            &*NEPHELOMETRIC_TURBIDITY_UNITS
        ),
        ("VUV", &*VATU),
        ("cubic_feet_per_hour", &*CUBIC_FEET_PER_HOUR),
        ("g/m³", &*GRAMS_PER_CUBIC_METER),
        ("SAR", &*SAUDI_RIYAL),
        ("rad/s", &*RADIANS_PER_SECOND),
        ("nanosecond", &*NANOSECOND),
        ("in", &*INCH),
        ("tonref", &*TONS_REFRIGERATION),
        ("Kz", &*KWANZA),
        ("us_dollar", &*US_DOLLAR),
        ("mVA", &*MEGAVOLT_AMPERE),
        ("kilohm", &*KILOHM),
        ("db_microvolt", &*DB_MICROVOLT),
        ("therm/h", &*THERMS_PER_HOUR),
        ("grams_per_minute", &*GRAMS_PER_MINUTE),
        ("byte", &*BYTE),
        ("%obsc/ft", &*PERCENT_OBSCURATION_PER_FOOT),
        ("millimeters_per_second", &*MILLIMETERS_PER_SECOND),
        ("megabtu_per_square_foot", &*MEGABTU_PER_SQUARE_FOOT),
        ("cal/g", &*CALORIE_PER_GRAM),
        ("klb", &*KILOPOUND),
        ("UYU", &*PESO_URUGUAYO),
        ("kW/ton", &*KILOWATT_PER_TON),
        ("scm", &*CUBIC_METERS_NATURAL_GAS),
        ("GBP", &*POUND_STERLING),
        ("L/s", &*LITERS_PER_SECOND),
        ("t", &*SHORT_TON),
        ("parts_per_unit", &*PARTS_PER_UNIT),
        ("MXN", &*MEXICAN_PESO),
        ("day", &*DAY),
        ("candels_per_square_foot", &*CANDELS_PER_SQUARE_FOOT),
        ("DJF", &*DJIBOUTI_FRANC),
        ("fluid_ounce", &*FLUID_OUNCE),
        ("joules_per_kilogram", &*JOULES_PER_KILOGRAM),
        ("vatu", &*VATU),
        ("S", &*SIEMENS),
        ("per_hour", &*PER_HOUR),
        ("ft/min", &*FEET_PER_MINUTE),
        ("VEF", &*BOLIVAR_FUERTE),
        ("cmH₂O", &*CENTIMETERS_OF_WATER),
        ("boliviano", &*BOLIVIANO),
        ("indian_rupee", &*INDIAN_RUPEE),
        ("ft²", &*SQUARE_FOOT),
        ("CNY", &*CHINESE_YUAN),
        ("pound_sterling", &*POUND_STERLING),
        ("DKK", &*DANISH_KRONE),
        ("CLP", &*CHILEAN_PESO),
        ("kilowatt_hour", &*KILOWATT_HOUR),
        ("hong_kong_dollar", &*HONG_KONG_DOLLAR),
        ("liberian_dollar", &*LIBERIAN_DOLLAR),
        ("SLL", &*LEONE),
        ("joules_per_gram", &*JOULES_PER_GRAM),
        ("Hz", &*HERTZ),
        ("kg/m²", &*KILOGRAMS_PER_SQUARE_METER),
        ("PKR", &*PAKISTAN_RUPEE),
        ("megajoules_per_hour", &*MEGAJOULES_PER_HOUR),
        ("px", &*PIXEL),
        ("centimeter", &*CENTIMETER),
        ("amperes_per_meter", &*AMPERES_PER_METER),
        ("kilobtu", &*KILOBTU),
        ("newton_meter", &*NEWTON_METER),
        (
            "kilojoules_per_degree_kelvin",
            &*KILOJOULES_PER_DEGREE_KELVIN
        ),
        ("turkish_lira", &*TURKISH_LIRA),
        ("L/h", &*LITERS_PER_HOUR),
        ("tons_refrigeration_hour", &*TONS_REFRIGERATION_HOUR),
        ("power_usage_effectiveness", &*POWER_USAGE_EFFECTIVENESS),
        ("KES", &*KENYAN_SHILLING),
        ("degree_days_fahrenheit", &*DEGREE_DAYS_FAHRENHEIT),
        ("megabtu", &*MEGABTU),
        ("/h", &*PER_HOUR),
        ("KHR", &*RIEL),
        ("knot", &*KNOT),
        ("gigawatt_hour", &*GIGAWATT_HOUR),
        ("PEN", &*NUEVO_SOL),
        ("kV", &*KILOVOLT),
        ("degrees_celsius_per_hour", &*DEGREES_CELSIUS_PER_HOUR),
        ("galUK", &*IMPERIAL_GALLON),
        ("MBTU/h", &*MEGABTUS_PER_HOUR),
        ("cubic_feet_per_second", &*CUBIC_FEET_PER_SECOND),
        ("grams_per_cubic_meter", &*GRAMS_PER_CUBIC_METER),
        ("XPF", &*CFP_FRANC),
        ("watts_per_square_foot", &*WATTS_PER_SQUARE_FOOT),
        ("kip", &*KIP),
        ("fl_oz", &*FLUID_OUNCE),
        ("kcfm", &*KILOCUBIC_FEET_PER_MINUTE),
        ("MGA", &*MALAGASY_ARIARY),
        ("kilowatts_per_square_meter", &*KILOWATTS_PER_SQUARE_METER),
        ("km²", &*SQUARE_KILOMETER),
        ("farad", &*FARAD),
        ("djibouti_franc", &*DJIBOUTI_FRANC),
        ("kina", &*KINA),
        ("ngultrum", &*NGULTRUM),
        ("NAD", &*NAMIBIA_DOLLAR),
        ("energy_efficiency_ratio", &*ENERGY_EFFICIENCY_RATIO),
        ("rad", &*RADIAN),
        ("MJ/kg_dry", &*MEGAJOULES_PER_KILOGRAM_DRY_AIR),
        ("Rs", &*SRI_LANKA_RUPEE),
        ("ЅМ", &*SOMONI),
        ("watt_hours_per_square_foot", &*WATT_HOURS_PER_SQUARE_FOOT),
        ("chilean_peso", &*CHILEAN_PESO),
        (
            "kilowatt_hours_per_square_foot",
            &*KILOWATT_HOURS_PER_SQUARE_FOOT
        ),
        ("lbf", &*POUND_FORCE),
        ("liter", &*LITER),
        ("CVE", &*CAPE_VERDE_ESCUDO),
        ("milligrams_per_cubic_meter", &*MILLIGRAMS_PER_CUBIC_METER),
        ("₭", &*KIP),
        ("лв", &*BULGARIAN_LEV),
        ("%obsc/m", &*PERCENT_OBSCURATION_PER_METER),
        ("naira", &*NAIRA),
        ("square_centimeter", &*SQUARE_CENTIMETER),
        ("FKP", &*FALKLAND_ISLANDS_POUND),
        ("square_yard", &*SQUARE_YARD),
        ("cfh", &*CUBIC_FEET_PER_HOUR),
        ("Js", &*JOULE_SECOND),
        ("CRC", &*COSTA_RICAN_COLON),
        ("W/m²", &*WATTS_PER_SQUARE_METER),
        ("siemens", &*SIEMENS),
        ("loti", &*LOTI),
        ("MKD", &*DENAR),
        ("uae_dirham", &*UAE_DIRHAM),
        ("ph", &*PH),
        ("milliampere", &*MILLIAMPERE),
        ("CHF", &*SWISS_FRANC),
        ("MYR", &*MALAYSIAN_RINGGIT),
        ("Nm", &*NEWTON_METER),
        ("millisecond", &*MILLISECOND),
        ("NPR", &*NEPALESE_RUPEE),
        ("quart", &*QUART),
        ("megavolt_ampere_hour", &*MEGAVOLT_AMPERE_HOUR),
        ("BYR", &*BELARUSSIAN_RUBLE),
        ("sudanese_pound", &*SUDANESE_POUND),
        ("ntu", &*NEPHELOMETRIC_TURBIDITY_UNITS),
        ("new_israeli_shekel", &*NEW_ISRAELI_SHEKEL),
        ("grams_per_second", &*GRAMS_PER_SECOND),
        ("kB", &*KILOBYTE),
        ("Rp", &*RUPIAH),
        ("W/ft²_irr", &*WATTS_PER_SQUARE_FOOT_IRRADIANCE),
        ("aruban_guilder_florin", &*ARUBAN_GUILDER_FLORIN),
        ("kilojoule", &*KILOJOULE),
        ("millibar", &*MILLIBAR),
        ("MAD", &*MOROCCAN_DIRHAM),
        ("V", &*VOLT),
        ("lb/min", &*POUNDS_PER_MINUTE),
        ("azerbaijanian_manat", &*AZERBAIJANIAN_MANAT),
        ("PYG", &*GUARANI),
        ("degrees_celsius_per_minute", &*DEGREES_CELSIUS_PER_MINUTE),
        ("zimbabwe_dollar", &*ZIMBABWE_DOLLAR),
        ("kilohertz", &*KILOHERTZ),
        ("henry", &*HENRY),
        ("megavolt", &*MEGAVOLT),
        ("/s", &*PER_SECOND),
        ("J/h", &*JOULES_PER_HOUR),
        ("barbados_dollar", &*BARBADOS_DOLLAR),
        ("PUE", &*POWER_USAGE_EFFECTIVENESS),
        ("watt", &*WATT),
        ("yr", &*YEAR),
        ("kilowatt_per_ton", &*KILOWATT_PER_TON),
        ("cfp_franc", &*CFP_FRANC),
        ("MBTU", &*MEGABTU),
        ("ftlbs/sec", &*FOOT_POUNDS_PER_SECOND),
        ("gallons_per_hour", &*GALLONS_PER_HOUR),
        ("KRW", &*SOUTH_KOREAN_WON),
        ("ampere_square_meter", &*AMPERE_SQUARE_METER),
        ("hp", &*HORSEPOWER),
        ("revolutions_per_minute", &*REVOLUTIONS_PER_MINUTE),
        ("฿", &*BAHT),
        (
            "watts_per_square_meter_degree_kelvin",
            &*WATTS_PER_SQUARE_METER_DEGREE_KELVIN
        ),
        ("Nfk", &*NAKFA),
        ("atmosphere", &*ATMOSPHERE),
        ("AZN", &*AZERBAIJANIAN_MANAT),
        ("S/m", &*SIEMENS_PER_METER),
        ("₹", &*INDIAN_RUPEE),
        ("Br", &*BELARUSSIAN_RUBLE),
        ("norwegian_krone", &*NORWEGIAN_KRONE),
        ("millivolt", &*MILLIVOLT),
        ("ton", &*METRIC_TON),
        ("FJD", &*FIJI_DOLLAR),
        ("celsius_degrees", &*CELSIUS_DEGREES),
        ("RM", &*MALAYSIAN_RINGGIT),
        ("kilopounds_per_hour", &*KILOPOUNDS_PER_HOUR),
        ("hectoliters_per_second", &*HECTOLITERS_PER_SECOND),
        ("feet_per_second", &*FEET_PER_SECOND),
        ("₤", &*TURKISH_LIRA),
        ("kilobyte", &*KILOBYTE),
        ("Kr", &*ICELAND_KRONA),
        ("ZMW", &*ZAMBIAN_KWACHA),
        ("meters_per_second_squared", &*METERS_PER_SECOND_SQUARED),
        ("W/cfm", &*WATTS_PER_CUBIC_FEET_PER_MINUTE),
        ("danish_krone", &*DANISH_KRONE),
        ("footcandle", &*FOOTCANDLE),
        ("A/m²", &*AMPERES_PER_SQUARE_METER),
        ("Դ", &*ARMENIAN_DRAM),
        ("belarussian_ruble", &*BELARUSSIAN_RUBLE),
        ("quetzal", &*QUETZAL),
        ("psi_per_degree_fahrenheit", &*PSI_PER_DEGREE_FAHRENHEIT),
        ("€", &*EURO),
        ("trinidad_and_tobago_dollar", &*TRINIDAD_AND_TOBAGO_DOLLAR),
        ("mile²", &*SQUARE_MILE),
        ("₴", &*HRYVNIA),
        ("USD", &*US_DOLLAR),
        ("kVAh", &*KILOVOLT_AMPERE_HOUR),
        ("mbar", &*MILLIBAR),
        ("BTU", &*BTU),
        ("square_millimeter", &*SQUARE_MILLIMETER),
        ("liters_per_minute", &*LITERS_PER_MINUTE),
        ("standard_cubic_foot", &*CUBIC_FEET_NATURAL_GAS),
        ("AED", &*UAE_DIRHAM),
        ("pounds_per_hour", &*POUNDS_PER_HOUR),
        ("therms_per_hour", &*THERMS_PER_HOUR),
        ("pf", &*POWER_FACTOR),
        ("meters_per_minute", &*METERS_PER_MINUTE),
        ("cm²", &*SQUARE_CENTIMETER),
        ("gigabyte", &*GIGABYTE),
        ("therm", &*THERM),
        ("cayman_islands_dollar", &*CAYMAN_ISLANDS_DOLLAR),
        ("hundredths_second", &*HUNDREDTHS_SECOND),
        ("SBD", &*SOLOMON_ISLANDS_DOLLAR),
        ("ლ", &*LARI),
        ("mm³", &*CUBIC_MILLIMETER),
        ("ds", &*TENTHS_SECOND),
        ("cfm", &*CUBIC_FEET_PER_MINUTE),
        ("metric_ton", &*METRIC_TON),
        ("terabyte", &*TERABYTE),
        ("ounce", &*OUNCE),
        ("rad/s²", &*RADIANS_PER_SECOND_SQUARED),
        ("Ns", &*NEWTON_SECOND),
        ("gourde", &*GOURDE),
        ("nepalese_rupee", &*NEPALESE_RUPEE),
        ("kilocubic_feet_per_minute", &*KILOCUBIC_FEET_PER_MINUTE),
        ("min", &*MINUTE),
        ("Δ°C", &*CELSIUS_DEGREES),
        ("megahertz", &*MEGAHERTZ),
        ("rupiah", &*RUPIAH),
        ("TND", &*TUNISIAN_DINAR),
        ("TMT", &*MANAT),
        ("m/h", &*METERS_PER_HOUR),
        ("kg/min", &*KILOGRAMS_PER_MINUTE),
        ("WST", &*TALA),
        ("m²/N", &*SQUARE_METERS_PER_NEWTON),
        ("parts_per_million", &*PARTS_PER_MILLION),
        ("ppb", &*PARTS_PER_BILLION),
        ("SHP", &*SAINT_HELENA_POUND),
        ("somoni", &*SOMONI),
        ("volt_ampere_reactive_hour", &*VOLT_AMPERE_REACTIVE_HOUR),
        ("libyan_dinar", &*LIBYAN_DINAR),
        ("pounds_per_second", &*POUNDS_PER_SECOND),
        ("julian_month", &*JULIAN_MONTH),
        ("V/K", &*VOLTS_PER_DEGREE_KELVIN),
        ("RSD", &*SERBIAN_DINAR),
        ("ETB", &*ETHIOPIAN_BIRR),
        ("gallon", &*GALLON),
        ("mauritius_rupee", &*MAURITIUS_RUPEE),
        ("₪", &*NEW_ISRAELI_SHEKEL),
        ("square_inch", &*SQUARE_INCH),
        ("km", &*KILOMETER),
        ("ALL", &*LEK),
        ("₩", &*SOUTH_KOREAN_WON),
        ("standard_cubic_meter", &*CUBIC_METERS_NATURAL_GAS),
        ("kilograms_per_square_meter", &*KILOGRAMS_PER_SQUARE_METER),
        ("microsecond", &*MICROSECOND),
        ("mmHg", &*MILLIMETERS_OF_MERCURY),
        ("ms", &*MILLISECOND),
        ("hectoliter", &*HECTOLITER),
        ("degPh", &*DEGREES_PHASE),
        ("imperial_gallons_per_minute", &*IMPERIAL_GALLONS_PER_MINUTE),
        ("kJ/kg_dry", &*KILOJOULES_PER_KILOGRAM_DRY_AIR),
        ("balboa", &*BALBOA),
        ("V/m", &*VOLTS_PER_METER),
        ("kg/h", &*KILOGRAMS_PER_HOUR),
        ("PB", &*PETABYTE),
        ("XCD", &*EAST_CARIBBEAN_DOLLAR),
        ("miles_per_hour", &*MILES_PER_HOUR),
        ("din", &*SERBIAN_DINAR),
        ("hft³", &*HECTO_CUBIC_FOOT),
        ("lx", &*LUX),
        ("iraqi_dinar", &*IRAQI_DINAR),
        ("egyptian_pound", &*EGYPTIAN_POUND),
        ("megavolt_ampere", &*MEGAVOLT_AMPERE),
        ("qt", &*QUART),
        ("MMK", &*KYAT),
        ("uzbekistan_sum", &*UZBEKISTAN_SUM),
        ("mΩ", &*MILLIOHM),
        ("foot", &*FOOT),
        ("feet_per_minute", &*FEET_PER_MINUTE),
        ("cubic_yard", &*CUBIC_YARD),
        ("BTN", &*NGULTRUM),
        ("₸", &*TENGE),
        ("W/ft²", &*WATTS_PER_SQUARE_FOOT),
        ("cycles_per_minute", &*CYCLES_PER_MINUTE),
        ("kilometer", &*KILOMETER),
        ("LBP", &*LEBANESE_POUND),
        ("russian_ruble", &*RUSSIAN_RUBLE),
        ("DZD", &*ALGERIAN_DINAR),
        ("ZK", &*ZAMBIAN_KWACHA),
        ("°daysF", &*DEGREE_DAYS_FAHRENHEIT),
        ("degree_days_celsius", &*DEGREE_DAYS_CELSIUS),
        ("bermudian_dollar", &*BERMUDIAN_DOLLAR),
        ("Kn", &*CROATIAN_KUNA),
        ("joules_per_square_meter", &*JOULES_PER_SQUARE_METER),
        ("calorie_per_gram", &*CALORIE_PER_GRAM),
        ("mm/min", &*MILLIMETERS_PER_MINUTE),
        ("OMR", &*RIAL_OMANI),
    ]
    .iter()
    .cloned()
    .collect();
}
