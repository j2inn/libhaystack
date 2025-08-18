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
    pub static ref PERCENT_RELATIVE_HUMIDITY: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["percent_relative_humidity".to_string(), "%RH".to_string(),].to_vec(),
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

lazy_static! {
    pub static ref KILOGRAMS_PER_LITER: Unit = Unit {
        quantity: Some("density".to_string(),),
        ids: ["kilograms_per_liter".to_string(), "kg/L".to_string(),].to_vec(),
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
    pub static ref MILLIGRAMS_PER_LITER: Unit = Unit {
        quantity: Some("density".to_string(),),
        ids: ["milligrams_per_liter".to_string(), "mg/L".to_string(),].to_vec(),
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
        ids: [
            "hundred_cubic_feet_natural_gas".to_string(),
            "Hcf_natural_gas".to_string(),
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
        scale: 108649800.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CENTUM_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: [
            "centum_cubic_feet_natural_gas".to_string(),
            "Ccf_natural_gas".to_string(),
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
        scale: 108649800.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref THOUSAND_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: [
            "thousand_cubic_feet_natural_gas".to_string(),
            "Mcf_natural_gas".to_string(),
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
        scale: 1086498000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLION_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
        quantity: Some("energy".to_string(),),
        ids: [
            "million_cubic_feet_natural_gas".to_string(),
            "MMcf_natural_gas".to_string(),
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
    pub static ref BTU_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("energy by area".to_string(),),
        ids: ["btu_per_square_foot".to_string(), "BTU/ft²".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 1,
            m: 0,
            sec: -2,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 11354.33731957,
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

// energy by volume

lazy_static! {
    pub static ref JOULES_PER_CUBIC_METER: Unit = Unit {
        quantity: Some("energy by volume".to_string(),),
        ids: ["joules_per_cubic_meter".to_string(), "J/m³".to_string(),].to_vec(),
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
    pub static ref GIGAJOULES_PER_CUBIC_METER: Unit = Unit {
        quantity: Some("energy by volume".to_string(),),
        ids: [
            "gigajoules_per_cubic_meter".to_string(),
            "GJ/m³".to_string(),
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
        scale: 1000000000.0,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref KILOWATT_HOURS_PER_CUBIC_METER: Unit = Unit {
        quantity: Some("energy by volume".to_string(),),
        ids: [
            "kilowatt_hours_per_cubic_meter".to_string(),
            "kWh/m³".to_string(),
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
        scale: 3600000.0,
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
        ids: ["second".to_string(), "s".to_string(), "sec".to_string(),].to_vec(),
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
        ids: ["hour".to_string(), "h".to_string(), "hr".to_string(),].to_vec(),
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
    pub static ref INCHES_PER_HOUR: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["inches_per_hour".to_string(), "in/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
            sec: -1,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 7.055555555555556e-6,
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
    pub static ref MILLIMETERS_PER_HOUR: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: ["millimeters_per_hour".to_string(), "mm/h".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 1,
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

lazy_static! {
    pub static ref CUBIC_FEET_PER_MINUTE_PER_SQUARE_FOOT: Unit = Unit {
        quantity: Some("velocity".to_string(),),
        ids: [
            "cubic_feet_per_minute_per_square_foot".to_string(),
            "cfm/ft²".to_string(),
        ]
        .to_vec(),
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
        scale: 0.028316846592,
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
        scale: 2.8316846592,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref HUNDRED_CUBIC_FEET: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["hundred_cubic_feet".to_string(), "Hcf".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2.8316846592,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref CENTUM_CUBIC_FEET: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["centum_cubic_feet".to_string(), "Ccf".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 2.8316846592,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref THOUSAND_CUBIC_FEET: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["thousand_cubic_feet".to_string(), "Mcf".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 28.316846592,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref MILLION_CUBIC_FEET: Unit = Unit {
        quantity: Some("volume".to_string(),),
        ids: ["million_cubic_feet".to_string(), "MMcf".to_string(),].to_vec(),
        dimensions: Some(UnitDimensions {
            kg: 0,
            m: 3,
            sec: 0,
            k: 0,
            a: 0,
            mol: 0,
            cd: 0,
        },),
        scale: 28316.846592,
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
        ("per_hour", &*PER_HOUR),
        ("₩", &*SOUTH_KOREAN_WON),
        ("tunisian_dinar", &*TUNISIAN_DINAR),
        ("MYR", &*MALAYSIAN_RINGGIT),
        ("T", &*TESLA),
        ("Pa", &*PASCAL),
        ("kWh/m²", &*KILOWATT_HOURS_PER_SQUARE_METER),
        ("galUK/min", &*IMPERIAL_GALLONS_PER_MINUTE),
        ("mph", &*MILES_PER_HOUR),
        ("hft³", &*HECTO_CUBIC_FOOT),
        ("mL", &*MILLILITER),
        ("RM", &*MALAYSIAN_RINGGIT),
        ("₭", &*KIP),
        ("VARh", &*VOLT_AMPERE_REACTIVE_HOUR),
        ("kilobtu", &*KILOBTU),
        ("degrees_kelvin_per_second", &*DEGREES_KELVIN_PER_SECOND),
        ("kilograms_per_minute", &*KILOGRAMS_PER_MINUTE),
        ("megajoule", &*MEGAJOULE),
        ("kg/min", &*KILOGRAMS_PER_MINUTE),
        ("kJ", &*KILOJOULE),
        ("Ωm", &*OHM_METER),
        ("megawatt_hour", &*MEGAWATT_HOUR),
        ("sr", &*STERADIAN),
        ("kelvin_degrees", &*KELVIN_DEGREES),
        ("day", &*DAY),
        ("yr", &*YEAR),
        ("hundredths_second", &*HUNDREDTHS_SECOND),
        ("joules_per_kilogram_dry_air", &*JOULES_PER_KILOGRAM_DRY_AIR),
        ("kilobtus_per_hour", &*KILOBTUS_PER_HOUR),
        ("₲", &*GUARANI),
        ("quetzal", &*QUETZAL),
        ("K/s", &*DEGREES_KELVIN_PER_SECOND),
        ("steradian", &*STERADIAN),
        ("TB", &*TERABYTE),
        ("THB", &*BAHT),
        ("km/h", &*KILOMETERS_PER_HOUR),
        ("megahertz", &*MEGAHERTZ),
        ("S/m", &*SIEMENS_PER_METER),
        ("baht", &*BAHT),
        (
            "thousand_cubic_feet_natural_gas",
            &*THOUSAND_CUBIC_FEET_NATURAL_GAS
        ),
        ("CDF", &*CONGOLESE_FRANC),
        ("lb/min", &*POUNDS_PER_MINUTE),
        ("kilovolt_ampere_hour", &*KILOVOLT_AMPERE_HOUR),
        ("gal/hr", &*GALLONS_PER_HOUR),
        ("micrometer", &*MICROMETER),
        ("bar", &*BAR),
        ("foot", &*FOOT),
        (
            "watts_per_square_meter_irradiance",
            &*WATTS_PER_SQUARE_METER_IRRADIANCE
        ),
        ("singapore_dollar", &*SINGAPORE_DOLLAR),
        ("cordoba_oro", &*CORDOBA_ORO),
        ("loti", &*LOTI),
        ("oz", &*OUNCE),
        ("MDL", &*MOLDAVIAN_LEU),
        ("Ω", &*OHM),
        ("ΔK", &*KELVIN_DEGREES),
        ("KPW", &*NORTH_KOREAN_WON),
        ("degree_days_fahrenheit", &*DEGREE_DAYS_FAHRENHEIT),
        ("kilograms_per_second", &*KILOGRAMS_PER_SECOND),
        ("A/m²", &*AMPERES_PER_SQUARE_METER),
        ("fiji_dollar", &*FIJI_DOLLAR),
        ("centimeters_of_water", &*CENTIMETERS_OF_WATER),
        ("km²", &*SQUARE_KILOMETER),
        (
            "data_center_infrastructure_efficiency",
            &*DATA_CENTER_INFRASTRUCTURE_EFFICIENCY
        ),
        ("aruban_guilder_florin", &*ARUBAN_GUILDER_FLORIN),
        ("australian_dollar", &*AUSTRALIAN_DOLLAR),
        ("cuban_peso", &*CUBAN_PESO),
        ("CAD", &*CANADIAN_DOLLAR),
        ("gallons_per_minute", &*GALLONS_PER_MINUTE),
        (
            "megawatt_hours_per_square_foot",
            &*MEGAWATT_HOURS_PER_SQUARE_FOOT
        ),
        ("GB", &*GIGABYTE),
        ("swiss_franc", &*SWISS_FRANC),
        ("ƒ", &*ARUBAN_GUILDER_FLORIN),
        ("XPF", &*CFP_FRANC),
        ("Le", &*LEONE),
        ("Mcf", &*THOUSAND_CUBIC_FEET),
        ("m²", &*SQUARE_METER),
        ("BWP", &*PULA),
        ("VA", &*VOLT_AMPERE),
        ("NPR", &*NEPALESE_RUPEE),
        (
            "kilowatt_hours_per_square_foot",
            &*KILOWATT_HOURS_PER_SQUARE_FOOT
        ),
        ("degrees_angular", &*DEGREES_ANGULAR),
        ("°C", &*CELSIUS),
        ("inches_of_water", &*INCHES_OF_WATER),
        ("kr", &*DANISH_KRONE),
        ("seychelles_rupee", &*SEYCHELLES_RUPEE),
        ("MWh", &*MEGAWATT_HOUR),
        ("square_centimeter", &*SQUARE_CENTIMETER),
        ("dalasi", &*DALASI),
        ("manat", &*MANAT),
        ("standard_cubic_meter", &*CUBIC_METERS_NATURAL_GAS),
        (
            "megawatt_hours_per_square_meter",
            &*MEGAWATT_HOURS_PER_SQUARE_METER
        ),
        ("MMBTU/h", &*MEGABTUS_PER_HOUR),
        ("nepalese_rupee", &*NEPALESE_RUPEE),
        ("hectopascal", &*HECTOPASCAL),
        ("hectoliter", &*HECTOLITER),
        ("LSL", &*LOTI),
        ("kilojoules_per_kilogram", &*KILOJOULES_PER_KILOGRAM),
        ("millimeters_per_second", &*MILLIMETERS_PER_SECOND),
        ("power_factor", &*POWER_FACTOR),
        ("cm³", &*CUBIC_CENTIMETER),
        ("imperial_gallons_per_minute", &*IMPERIAL_GALLONS_PER_MINUTE),
        ("TZS", &*TANZANIAN_SHILLING),
        ("bulgarian_lev", &*BULGARIAN_LEV),
        ("N/m", &*NEWTONS_PER_METER),
        ("sec", &*SECOND),
        ("pascal", &*PASCAL),
        ("mm/h", &*MILLIMETERS_PER_HOUR),
        ("namibia_dollar", &*NAMIBIA_DOLLAR),
        ("rwanda_franc", &*RWANDA_FRANC),
        ("mW", &*MILLIWATT),
        ("W/ft²_irr", &*WATTS_PER_SQUARE_FOOT_IRRADIANCE),
        ("metical", &*METICAL),
        ("SZL", &*LILANGENI),
        ("min", &*MINUTE),
        ("PUE", &*POWER_USAGE_EFFECTIVENESS),
        ("MMcf_natural_gas", &*MILLION_CUBIC_FEET_NATURAL_GAS),
        ("MUR", &*MAURITIUS_RUPEE),
        ("GW", &*GIGAWATT),
        ("AMD", &*ARMENIAN_DRAM),
        ("m/min", &*METERS_PER_MINUTE),
        ("phot", &*PHOT),
        ("knot", &*KNOT),
        ("russian_ruble", &*RUSSIAN_RUBLE),
        ("Volt", &*VOLT),
        ("pounds_per_second", &*POUNDS_PER_SECOND),
        ("megajoules_per_hour", &*MEGAJOULES_PER_HOUR),
        ("MGA", &*MALAGASY_ARIARY),
        (
            "kilobtus_per_hour_per_square_foot",
            &*KILOBTUS_PER_HOUR_PER_SQUARE_FOOT
        ),
        ("megabtu", &*MEGABTU),
        ("decibel", &*DECIBEL),
        ("pH", &*PH),
        ("LAK", &*KIP),
        ("afghani", &*AFGHANI),
        ("ILS", &*NEW_ISRAELI_SHEKEL),
        ("MΩ", &*MEGOHM),
        ("watt", &*WATT),
        ("MJ/h", &*MEGAJOULES_PER_HOUR),
        ("atm", &*ATMOSPHERE),
        ("W/m°K", &*WATTS_PER_METER_DEGREE_KELVIN),
        ("btus_per_hour", &*BTUS_PER_HOUR),
        ("btus_per_pound_dry_air", &*BTUS_PER_POUND_DRY_AIR),
        ("cmHg", &*CENTIMETERS_OF_MERCURY),
        ("K", &*KELVIN),
        ("Hcf", &*HUNDRED_CUBIC_FEET),
        ("Mcf_natural_gas", &*THOUSAND_CUBIC_FEET_NATURAL_GAS),
        ("percent_relative_humidity", &*PERCENT_RELATIVE_HUMIDITY),
        ("rupiah", &*RUPIAH),
        ("UZS", &*UZBEKISTAN_SUM),
        ("F", &*FARAD),
        ("kBTU", &*KILOBTU),
        ("bahamian_dollar", &*BAHAMIAN_DOLLAR),
        ("NOK", &*NORWEGIAN_KRONE),
        ("pzloty", &*PZLOTY),
        ("J/h", &*JOULES_PER_HOUR),
        ("GJ/h", &*GIGAJOULES_PER_HOUR),
        ("kW/kcfm", &*KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE),
        ("m³", &*CUBIC_METER),
        ("s", &*SECOND),
        ("mexican_peso", &*MEXICAN_PESO),
        ("gal", &*GALLON),
        ("megabtu_per_square_foot", &*MEGABTU_PER_SQUARE_FOOT),
        ("°daysC", &*DEGREE_DAYS_CELSIUS),
        ("SRD", &*SURINAME_DOLLAR),
        ("watts_per_square_foot", &*WATTS_PER_SQUARE_FOOT),
        ("yen", &*YEN),
        ("parts_per_unit", &*PARTS_PER_UNIT),
        ("jordanian_dinar", &*JORDANIAN_DINAR),
        ("MΩ", &*MEGOHM),
        ("SGD", &*SINGAPORE_DOLLAR),
        ("J/kg", &*JOULES_PER_KILOGRAM),
        ("ton/h", &*METRIC_TONS_PER_HOUR),
        ("kBTU/h", &*KILOBTUS_PER_HOUR),
        ("revolutions_per_minute", &*REVOLUTIONS_PER_MINUTE),
        ("COP", &*COEFFICIENT_OF_PERFORMANCE),
        ("libyan_dinar", &*LIBYAN_DINAR),
        ("forint", &*FORINT),
        ("ISK", &*ICELAND_KRONA),
        ("parts_per_million", &*PARTS_PER_MILLION),
        ("cd/ft²", &*CANDELS_PER_SQUARE_FOOT),
        ("GTQ", &*QUETZAL),
        (
            "watts_per_square_foot_irradiance",
            &*WATTS_PER_SQUARE_FOOT_IRRADIANCE
        ),
        ("Δ°C", &*CELSIUS_DEGREES),
        ("gigajoule", &*GIGAJOULE),
        ("MVAR", &*MEGAVOLT_AMPERE_REACTIVE),
        ("megavolt_ampere_hour", &*MEGAVOLT_AMPERE_HOUR),
        ("megavolt_ampere_reactive", &*MEGAVOLT_AMPERE_REACTIVE),
        ("µs", &*MICROSECOND),
        ("kW/gal/min", &*KILOWATTS_PER_GALLONS_PER_MINUTE),
        ("calorie", &*CALORIE),
        ("btu_per_square_foot", &*BTU_PER_SQUARE_FOOT),
        ("H", &*HENRY),
        ("rad/s²", &*RADIANS_PER_SECOND_SQUARED),
        ("PEN", &*NUEVO_SOL),
        ("philippine_peso", &*PHILIPPINE_PESO),
        ("cubic_meters_per_second", &*CUBIC_METERS_PER_SECOND),
        ("candels_per_square_foot", &*CANDELS_PER_SQUARE_FOOT),
        ("ngultrum", &*NGULTRUM),
        ("algerian_dinar", &*ALGERIAN_DINAR),
        ("pula", &*PULA),
        ("hour", &*HOUR),
        ("₨", &*PAKISTAN_RUPEE),
        ("₡", &*COSTA_RICAN_COLON),
        ("nanosecond", &*NANOSECOND),
        ("mL/s", &*MILLILITERS_PER_SECOND),
        ("Kn", &*CROATIAN_KUNA),
        ("radian", &*RADIAN),
        (
            "grams_of_water_per_kilogram_dry_air",
            &*GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR
        ),
        ("lilangeni", &*LILANGENI),
        ("percent", &*PERCENT),
        ("microsecond", &*MICROSECOND),
        ("iranian_rial", &*IRANIAN_RIAL),
        ("C$", &*CORDOBA_ORO),
        ("kilometer", &*KILOMETER),
        ("qatari_rial", &*QATARI_RIAL),
        ("megavolt_ampere", &*MEGAVOLT_AMPERE),
        ("falkland_islands_pound", &*FALKLAND_ISLANDS_POUND),
        ("fl_oz", &*FLUID_OUNCE),
        ("degrees_celsius_per_minute", &*DEGREES_CELSIUS_PER_MINUTE),
        ("HNL", &*LEMPIRA),
        ("btu/lb_dry", &*BTUS_PER_POUND_DRY_AIR),
        ("volts_per_meter", &*VOLTS_PER_METER),
        ("klb", &*KILOPOUND),
        ("meters_per_minute", &*METERS_PER_MINUTE),
        ("kiloliter", &*KILOLITER),
        ("hong_kong_dollar", &*HONG_KONG_DOLLAR),
        ("K/h", &*DEGREES_KELVIN_PER_HOUR),
        ("TRY", &*TURKISH_LIRA),
        ("acre", &*ACRE),
        ("cd/m²", &*CANDELAS_PER_SQUARE_METER),
        ("AOA", &*KWANZA),
        ("million_cubic_feet", &*MILLION_CUBIC_FEET),
        ("tala", &*TALA),
        ("kJ/°K", &*KILOJOULES_PER_DEGREE_KELVIN),
        ("percent_per_second", &*PERCENT_PER_SECOND),
        ("BTU/h", &*BTUS_PER_HOUR),
        ("megajoules_per_square_foot", &*MEGAJOULES_PER_SQUARE_FOOT),
        ("Hcf_natural_gas", &*HUNDRED_CUBIC_FEET_NATURAL_GAS),
        ("КМ", &*KONVERTIBILNA_MARKA),
        ("MW", &*MEGAWATT),
        ("GEL", &*LARI),
        ("AWG", &*ARUBAN_GUILDER_FLORIN),
        ("joules_per_gram", &*JOULES_PER_GRAM),
        ("GNF", &*GUINEA_FRANC),
        ("in³", &*CUBIC_INCH),
        ("%RH", &*PERCENT_RELATIVE_HUMIDITY),
        ("kilopascal", &*KILOPASCAL),
        (
            "million_cubic_feet_natural_gas",
            &*MILLION_CUBIC_FEET_NATURAL_GAS
        ),
        ("cubic_foot", &*CUBIC_FOOT),
        ("uzbekistan_sum", &*UZBEKISTAN_SUM),
        ("Wb", &*WEBER),
        ("MB", &*MEGABYTE),
        ("W/m³/s", &*WATTS_PER_CUBIC_METERS_PER_SECOND),
        ("%/s", &*PERCENT_PER_SECOND),
        ("lbf", &*POUND_FORCE),
        ("CZK", &*CZECH_KORUNA),
        ("amperes_per_square_meter", &*AMPERES_PER_SQUARE_METER),
        ("in/h", &*INCHES_PER_HOUR),
        ("burundi_franc", &*BURUNDI_FRANC),
        ("GJ", &*GIGAJOULE),
        ("watt_hour", &*WATT_HOUR),
        ("brazilian_real", &*BRAZILIAN_REAL),
        ("cycles_per_minute", &*CYCLES_PER_MINUTE),
        ("QAR", &*QATARI_RIAL),
        ("somali_shilling", &*SOMALI_SHILLING),
        ("parts_per_billion", &*PARTS_PER_BILLION),
        ("UGX", &*UGANDA_SHILLING),
        ("grams_per_square_meter", &*GRAMS_PER_SQUARE_METER),
        ("fahrenheit", &*FAHRENHEIT),
        ("joules_per_cubic_meter", &*JOULES_PER_CUBIC_METER),
        ("dominican_peso", &*DOMINICAN_PESO),
        ("m³_gas", &*CUBIC_METERS_NATURAL_GAS),
        (
            "percent_obscuration_per_foot",
            &*PERCENT_OBSCURATION_PER_FOOT
        ),
        ("guarani", &*GUARANI),
        ("MK", &*KWACHA),
        ("lux", &*LUX),
        ("STD", &*DOBRA),
        ("ampere", &*AMPERE),
        ("CVE", &*CAPE_VERDE_ESCUDO),
        ("€", &*EURO),
        ("kVAh", &*KILOVOLT_AMPERE_HOUR),
        ("CUP", &*CUBAN_PESO),
        ("VND", &*DONG),
        ("$", &*US_DOLLAR),
        ("BTU", &*BTU),
        ("brunei_dollar", &*BRUNEI_DOLLAR),
        ("costa_rican_colon", &*COSTA_RICAN_COLON),
        ("SDG", &*SUDANESE_POUND),
        ("therm/h", &*THERMS_PER_HOUR),
        ("barbados_dollar", &*BARBADOS_DOLLAR),
        ("watt_hours_per_square_meter", &*WATT_HOURS_PER_SQUARE_METER),
        ("AED", &*UAE_DIRHAM),
        ("cm", &*CENTIMETER),
        ("hryvnia", &*HRYVNIA),
        ("riel", &*RIEL),
        ("kL", &*KILOLITER),
        ("taiwan_dollar", &*TAIWAN_DOLLAR),
        ("milliampere", &*MILLIAMPERE),
        ("°daysF", &*DEGREE_DAYS_FAHRENHEIT),
        ("ZWL", &*ZIMBABWE_DOLLAR),
        ("hp", &*HORSEPOWER),
        ("hPa", &*HECTOPASCAL),
        ("NAD", &*NAMIBIA_DOLLAR),
        ("J/g", &*JOULES_PER_GRAM),
        ("h", &*HOUR),
        ("newtons_per_meter", &*NEWTONS_PER_METER),
        ("iceland_krona", &*ICELAND_KRONA),
        ("XCD", &*EAST_CARIBBEAN_DOLLAR),
        ("sudanese_pound", &*SUDANESE_POUND),
        ("V", &*VOLT),
        ("degrees_celsius_per_hour", &*DEGREES_CELSIUS_PER_HOUR),
        ("cubic_millimeter", &*CUBIC_MILLIMETER),
        ("meter", &*METER),
        ("kPa", &*KILOPASCAL),
        ("dBmV", &*DB_MILLIVOLT),
        ("LYD", &*LIBYAN_DINAR),
        ("TJS", &*SOMONI),
        ("EUR", &*EURO),
        ("btu", &*BTU),
        ("year", &*YEAR),
        ("Wh", &*WATT_HOUR),
        ("XAF", &*CFA_FRANC_BCEAO),
        ("week", &*WEEK),
        ("J/kg_dry", &*JOULES_PER_KILOGRAM_DRY_AIR),
        ("cape_verde_escudo", &*CAPE_VERDE_ESCUDO),
        ("fluid_ounce", &*FLUID_OUNCE),
        ("kwacha", &*KWACHA),
        ("millimeters_per_minute", &*MILLIMETERS_PER_MINUTE),
        ("LRD", &*LIBERIAN_DOLLAR),
        ("inch", &*INCH),
        ("tonref", &*TONS_REFRIGERATION),
        ("g/m²", &*GRAMS_PER_SQUARE_METER),
        ("kuwaiti_dinar", &*KUWAITI_DINAR),
        ("kJ/h", &*KILOJOULES_PER_HOUR),
        ("mile²", &*SQUARE_MILE),
        ("energy_efficiency_ratio", &*ENERGY_EFFICIENCY_RATIO),
        ("uae_dirham", &*UAE_DIRHAM),
        ("cayman_islands_dollar", &*CAYMAN_ISLANDS_DOLLAR),
        ("kVAR", &*KILOVOLT_AMPERE_REACTIVE),
        ("kilograms_per_cubic_meter", &*KILOGRAMS_PER_CUBIC_METER),
        ("JPY", &*YEN),
        ("J/°K", &*JOULES_PER_DEGREE_KELVIN),
        ("gigawatt", &*GIGAWATT),
        ("hL/s", &*HECTOLITERS_PER_SECOND),
        ("TND", &*TUNISIAN_DINAR),
        ("lumen", &*LUMEN),
        (
            "percent_obscuration_per_meter",
            &*PERCENT_OBSCURATION_PER_METER
        ),
        ("kwanza", &*KWANZA),
        ("joule_second", &*JOULE_SECOND),
        ("NZD", &*NEW_ZEALAND_DOLLAR),
        ("¥", &*YEN),
        ("kilograms_per_liter", &*KILOGRAMS_PER_LITER),
        ("kcfm", &*KILOCUBIC_FEET_PER_MINUTE),
        ("hertz", &*HERTZ),
        ("kelvin", &*KELVIN),
        ("GMD", &*DALASI),
        ("byte", &*BYTE),
        ("kilogallon", &*KILOGALLON),
        ("cubic_meters_per_hour", &*CUBIC_METERS_PER_HOUR),
        ("BND", &*BRUNEI_DOLLAR),
        ("gourde", &*GOURDE),
        ("gallon", &*GALLON),
        ("Ccf", &*CENTUM_CUBIC_FEET),
        ("L", &*LITER),
        ("ntu", &*NEPHELOMETRIC_TURBIDITY_UNITS),
        ("ds", &*TENTHS_SECOND),
        ("som", &*SOM),
        ("farad", &*FARAD),
        ("ethiopian_birr", &*ETHIOPIAN_BIRR),
        ("square_millimeter", &*SQUARE_MILLIMETER),
        ("petabyte", &*PETABYTE),
        ("kilowatt_hour", &*KILOWATT_HOUR),
        ("₵", &*CEDI),
        (
            "kilojoules_per_kilogram_dry_air",
            &*KILOJOULES_PER_KILOGRAM_DRY_AIR
        ),
        ("GBP", &*POUND_STERLING),
        ("NIO", &*CORDOBA_ORO),
        ("cubic_meter", &*CUBIC_METER),
        ("Br", &*BELARUSSIAN_RUBLE),
        ("yd³", &*CUBIC_YARD),
        ("°F/min", &*DEGREES_FAHRENHEIT_PER_MINUTE),
        ("volt_ampere_reactive", &*VOLT_AMPERE_REACTIVE),
        ("BBD", &*BARBADOS_DOLLAR),
        ("ALL", &*LEK),
        ("therms_per_hour", &*THERMS_PER_HOUR),
        ("ЅМ", &*SOMONI),
        ("HTG", &*GOURDE),
        ("gibraltar_pound", &*GIBRALTAR_POUND),
        ("mV", &*MILLIVOLT),
        ("J/m³", &*JOULES_PER_CUBIC_METER),
        ("kg/h", &*KILOGRAMS_PER_HOUR),
        ("₫", &*DONG),
        ("cubic_feet_natural_gas", &*CUBIC_FEET_NATURAL_GAS),
        ("TWD", &*TAIWAN_DOLLAR),
        ("kg/m²", &*KILOGRAMS_PER_SQUARE_METER),
        ("MJ/ft²", &*MEGAJOULES_PER_SQUARE_FOOT),
        ("ppm", &*PARTS_PER_MILLION),
        ("cubic_feet_per_hour", &*CUBIC_FEET_PER_HOUR),
        ("A", &*AMPERE),
        ("DJF", &*DJIBOUTI_FRANC),
        ("Kr", &*ICELAND_KRONA),
        ("pixel", &*PIXEL),
        ("NGN", &*NAIRA),
        (
            "kilowatt_hours_per_cubic_meter",
            &*KILOWATT_HOURS_PER_CUBIC_METER
        ),
        ("hecto_cubic_foot", &*HECTO_CUBIC_FOOT),
        ("kip", &*KIP),
        ("millibar", &*MILLIBAR),
        ("₦", &*NAIRA),
        (
            "megavolt_ampere_reactive_hour",
            &*MEGAVOLT_AMPERE_REACTIVE_HOUR
        ),
        ("imperial_gallon", &*IMPERIAL_GALLON),
        ("kHz", &*KILOHERTZ),
        ("Wh/m²", &*WATT_HOURS_PER_SQUARE_METER),
        ("Ah", &*AMPERE_HOUR),
        ("OMR", &*RIAL_OMANI),
        ("cal", &*CALORIE),
        ("congolese_franc", &*CONGOLESE_FRANC),
        ("PYG", &*GUARANI),
        ("°C/min", &*DEGREES_CELSIUS_PER_MINUTE),
        ("metric_ton", &*METRIC_TON),
        ("megabyte", &*MEGABYTE),
        ("V/K", &*VOLTS_PER_DEGREE_KELVIN),
        ("UYU", &*PESO_URUGUAYO),
        ("cph", &*CYCLES_PER_HOUR),
        ("newton_second", &*NEWTON_SECOND),
        ("dobra", &*DOBRA),
        ("RSD", &*SERBIAN_DINAR),
        ("new_zealand_dollar", &*NEW_ZEALAND_DOLLAR),
        ("calorie_per_gram", &*CALORIE_PER_GRAM),
        ("лв", &*BULGARIAN_LEV),
        ("megohm", &*MEGOHM),
        ("A/m", &*AMPERES_PER_METER),
        ("cycles_per_hour", &*CYCLES_PER_HOUR),
        ("kVA", &*KILOVOLT_AMPERE),
        ("mmHg", &*MILLIMETERS_OF_MERCURY),
        ("kilopounds_per_hour", &*KILOPOUNDS_PER_HOUR),
        ("degrees_kelvin_per_minute", &*DEGREES_KELVIN_PER_MINUTE),
        (
            "kilowatts_per_gallons_per_minute",
            &*KILOWATTS_PER_GALLONS_PER_MINUTE
        ),
        ("cubic_meters_natural_gas", &*CUBIC_METERS_NATURAL_GAS),
        ("mauritius_rupee", &*MAURITIUS_RUPEE),
        ("klb/h", &*KILOPOUNDS_PER_HOUR),
        ("mbar", &*MILLIBAR),
        ("cd", &*CANDELA),
        ("Ns", &*NEWTON_SECOND),
        ("cfm", &*CUBIC_FEET_PER_MINUTE),
        ("somoni", &*SOMONI),
        ("lebanese_pound", &*LEBANESE_POUND),
        ("VAR", &*VOLT_AMPERE_REACTIVE),
        ("MVARh", &*MEGAVOLT_AMPERE_REACTIVE_HOUR),
        ("kJ/kg", &*KILOJOULES_PER_KILOGRAM),
        ("DKK", &*DANISH_KRONE),
        ("pound_force", &*POUND_FORCE),
        ("tesla", &*TESLA),
        ("mo", &*JULIAN_MONTH),
        ("DOP", &*DOMINICAN_PESO),
        ("saudi_riyal", &*SAUDI_RIYAL),
        ("kina", &*KINA),
        ("pounds_per_square_inch", &*POUNDS_PER_SQUARE_INCH),
        ("kyat", &*KYAT),
        ("MRO", &*OUGUIYA),
        ("SCR", &*SEYCHELLES_RUPEE),
        ("bahraini_dinar", &*BAHRAINI_DINAR),
        ("IRR", &*IRANIAN_RIAL),
        ("per_second", &*PER_SECOND),
        ("g", &*GRAM),
        ("MVR", &*RUFIYAA),
        ("gram", &*GRAM),
        ("millimeters_of_mercury", &*MILLIMETERS_OF_MERCURY),
        ("centum_cubic_feet", &*CENTUM_CUBIC_FEET),
        ("kB", &*KILOBYTE),
        (
            "centum_cubic_feet_natural_gas",
            &*CENTUM_CUBIC_FEET_NATURAL_GAS
        ),
        ("moroccan_dirham", &*MOROCCAN_DIRHAM),
        ("ton", &*METRIC_TON),
        ("Rp", &*RUPIAH),
        ("kWh/m³", &*KILOWATT_HOURS_PER_CUBIC_METER),
        ("₸", &*TENGE),
        ("mm²", &*SQUARE_MILLIMETER),
        ("cal/g", &*CALORIE_PER_GRAM),
        ("lm", &*LUMEN),
        ("chilean_peso", &*CHILEAN_PESO),
        ("Sh", &*KENYAN_SHILLING),
        ("USD", &*US_DOLLAR),
        ("SOS", &*SOMALI_SHILLING),
        ("danish_krone", &*DANISH_KRONE),
        ("BMD", &*BERMUDIAN_DOLLAR),
        ("pound_sterling", &*POUND_STERLING),
        ("AFN", &*AFGHANI),
        ("bermudian_dollar", &*BERMUDIAN_DOLLAR),
        ("meters_per_second_squared", &*METERS_PER_SECOND_SQUARED),
        ("square_meter", &*SQUARE_METER),
        ("S", &*SIEMENS),
        ("MJ/m²", &*MEGAJOULES_PER_SQUARE_METER),
        ("kilohm", &*KILOHM),
        ("bolivar_fuerte", &*BOLIVAR_FUERTE),
        ("MXN", &*MEXICAN_PESO),
        ("W/m²K", &*WATTS_PER_SQUARE_METER_DEGREE_KELVIN),
        ("kVARh", &*KILOVOLT_AMPERE_REACTIVE_HOUR),
        ("pint", &*PINT),
        ("jamaican_dollar", &*JAMAICAN_DOLLAR),
        (
            "watts_per_cubic_meters_per_second",
            &*WATTS_PER_CUBIC_METERS_PER_SECOND
        ),
        ("denar", &*DENAR),
        ("Wh/ft²", &*WATT_HOURS_PER_SQUARE_FOOT),
        ("Hz", &*HERTZ),
        ("megavolt", &*MEGAVOLT),
        ("atmosphere", &*ATMOSPHERE),
        ("%obsc/ft", &*PERCENT_OBSCURATION_PER_FOOT),
        ("cubic_feet_per_minute", &*CUBIC_FEET_PER_MINUTE),
        (
            "degrees_fahrenheit_per_minute",
            &*DEGREES_FAHRENHEIT_PER_MINUTE
        ),
        ("cfp_franc", &*CFP_FRANC),
        ("din", &*SERBIAN_DINAR),
        ("kV", &*KILOVOLT),
        ("MBTU/h", &*MEGABTUS_PER_HOUR),
        ("naira", &*NAIRA),
        ("GWh", &*GIGAWATT_HOUR),
        ("cmH₂O", &*CENTIMETERS_OF_WATER),
        ("₤", &*TURKISH_LIRA),
        ("us_dollar", &*US_DOLLAR),
        ("mg", &*MILLIGRAM),
        ("DZD", &*ALGERIAN_DINAR),
        ("kilojoule", &*KILOJOULE),
        ("kilowatt", &*KILOWATT),
        ("megabtus_per_hour", &*MEGABTUS_PER_HOUR),
        ("LKR", &*SRI_LANKA_RUPEE),
        ("BTN", &*NGULTRUM),
        ("rufiyaa", &*RUFIYAA),
        ("IDR", &*RUPIAH),
        ("hr", &*HOUR),
        ("uganda_shilling", &*UGANDA_SHILLING),
        ("kg/L", &*KILOGRAMS_PER_LITER),
        ("ampere_hour", &*AMPERE_HOUR),
        ("ACH", &*AIR_CHANGES_PER_HOUR),
        ("kilometers_per_second", &*KILOMETERS_PER_SECOND),
        ("CRC", &*COSTA_RICAN_COLON),
        ("nuevo_sol", &*NUEVO_SOL),
        ("liters_per_second", &*LITERS_PER_SECOND),
        ("Rs", &*SRI_LANKA_RUPEE),
        ("FJD", &*FIJI_DOLLAR),
        ("SEK", &*SWEDISH_KRONA),
        ("degree_days_celsius", &*DEGREE_DAYS_CELSIUS),
        ("PAB", &*BALBOA),
        ("milligrams_per_liter", &*MILLIGRAMS_PER_LITER),
        ("Nm", &*NEWTON_METER),
        ("VAh", &*VOLT_AMPERE_HOUR),
        ("newton", &*NEWTON),
        ("joules_per_kilogram", &*JOULES_PER_KILOGRAM),
        ("KES", &*KENYAN_SHILLING),
        ("new_israeli_shekel", &*NEW_ISRAELI_SHEKEL),
        ("PLN", &*PZLOTY),
        ("ZAR", &*RAND),
        ("minute", &*MINUTE),
        ("hectoliters_per_second", &*HECTOLITERS_PER_SECOND),
        ("WST", &*TALA),
        ("Btu/Wh", &*ENERGY_EFFICIENCY_RATIO),
        ("galUK", &*IMPERIAL_GALLON),
        ("BYR", &*BELARUSSIAN_RUBLE),
        ("chinese_yuan", &*CHINESE_YUAN),
        ("sri_lanka_rupee", &*SRI_LANKA_RUPEE),
        ("L/h", &*LITERS_PER_HOUR),
        ("egyptian_pound", &*EGYPTIAN_POUND),
        ("GYD", &*GUYANA_DOLLAR),
        ("mΩ", &*MILLIOHM),
        ("ft²", &*SQUARE_FOOT),
        ("indian_rupee", &*INDIAN_RUPEE),
        ("cubic_yard", &*CUBIC_YARD),
        ("nakfa", &*NAKFA),
        ("MVAh", &*MEGAVOLT_AMPERE_HOUR),
        ("hph", &*HORSEPOWER_HOUR),
        ("cfa_franc_bceao", &*CFA_FRANC_BCEAO),
        ("MWh/m²", &*MEGAWATT_HOURS_PER_SQUARE_METER),
        ("/s", &*PER_SECOND),
        ("°F", &*FAHRENHEIT),
        ("centimeters_of_mercury", &*CENTIMETERS_OF_MERCURY),
        ("kW/ton", &*KILOWATT_PER_TON),
        ("cm²", &*SQUARE_CENTIMETER),
        ("square_inch", &*SQUARE_INCH),
        ("kΩ", &*KILOHM),
        ("元", &*CHINESE_YUAN),
        ("inches_of_mercury", &*INCHES_OF_MERCURY),
        ("milliohm", &*MILLIOHM),
        ("croatian_kuna", &*CROATIAN_KUNA),
        ("PKR", &*PAKISTAN_RUPEE),
        ("CLP", &*CHILEAN_PESO),
        ("m/s", &*METERS_PER_SECOND),
        ("balboa", &*BALBOA),
        ("kenyan_shilling", &*KENYAN_SHILLING),
        ("volt_ampere_reactive_hour", &*VOLT_AMPERE_REACTIVE_HOUR),
        ("gal/min", &*GALLONS_PER_MINUTE),
        ("milligrams_per_cubic_meter", &*MILLIGRAMS_PER_CUBIC_METER),
        ("serbian_dinar", &*SERBIAN_DINAR),
        ("gph", &*GALLONS_PER_HOUR),
        ("millivolt", &*MILLIVOLT),
        ("rad", &*RADIAN),
        ("J", &*JOULE),
        ("belize_dollar", &*BELIZE_DOLLAR),
        ("psi_per_degree_fahrenheit", &*PSI_PER_DEGREE_FAHRENHEIT),
        ("mA", &*MILLIAMPERE),
        ("kilovolt", &*KILOVOLT),
        ("lx", &*LUX),
        ("vatu", &*VATU),
        ("ERN", &*NAKFA),
        ("Kz", &*KWANZA),
        ("TMT", &*MANAT),
        ("₪", &*NEW_ISRAELI_SHEKEL),
        ("J/m²", &*JOULES_PER_SQUARE_METER),
        ("horsepower_hour", &*HORSEPOWER_HOUR),
        ("degPh", &*DEGREES_PHASE),
        ("air_changes_per_hour", &*AIR_CHANGES_PER_HOUR),
        ("L/s", &*LITERS_PER_SECOND),
        ("ph", &*PH),
        ("BDT", &*TAKA),
        ("yd", &*YARD),
        ("liberian_dollar", &*LIBERIAN_DOLLAR),
        ("inH₂O", &*INCHES_OF_WATER),
        ("ppb", &*PARTS_PER_BILLION),
        ("quart", &*QUART),
        ("kilohertz", &*KILOHERTZ),
        ("ETB", &*ETHIOPIAN_BIRR),
        ("BZD", &*BELIZE_DOLLAR),
        ("kBTU/h/ft²", &*KILOBTUS_PER_HOUR_PER_SQUARE_FOOT),
        ("kilovolt_ampere_reactive", &*KILOVOLT_AMPERE_REACTIVE),
        ("kilowatt_per_ton", &*KILOWATT_PER_TON),
        ("g/kg", &*GRAMS_PER_KILOGRAM),
        ("east_caribbean_dollar", &*EAST_CARIBBEAN_DOLLAR),
        (
            "watts_per_square_meter_degree_kelvin",
            &*WATTS_PER_SQUARE_METER_DEGREE_KELVIN
        ),
        ("coulomb", &*COULOMB),
        ("TTD", &*TRINIDAD_AND_TOBAGO_DOLLAR),
        ("terabyte", &*TERABYTE),
        ("Ω", &*OHM),
        ("INR", &*INDIAN_RUPEE),
        ("guyana_dollar", &*GUYANA_DOLLAR),
        ("belarussian_ruble", &*BELARUSSIAN_RUBLE),
        ("db_microvolt", &*DB_MICROVOLT),
        ("millimeter", &*MILLIMETER),
        ("yemeni_rial", &*YEMENI_RIAL),
        ("£", &*POUND_STERLING),
        ("EER", &*ENERGY_EFFICIENCY_RATIO),
        ("MNT", &*TUGRIK),
        ("MJ/kg_dry", &*MEGAJOULES_PER_KILOGRAM_DRY_AIR),
        ("mm/min", &*MILLIMETERS_PER_MINUTE),
        ("px", &*PIXEL),
        ("KWD", &*KUWAITI_DINAR),
        ("deg", &*DEGREES_ANGULAR),
        ("rad/s", &*RADIANS_PER_SECOND),
        ("RON", &*LEU),
        ("azerbaijanian_manat", &*AZERBAIJANIAN_MANAT),
        ("kΩ", &*KILOHM),
        ("dBµV", &*DB_MICROVOLT),
        ("short_ton", &*SHORT_TON),
        ("g/min", &*GRAMS_PER_MINUTE),
        ("in/wc", &*INCHES_OF_WATER),
        ("KRW", &*SOUTH_KOREAN_WON),
        ("SYP", &*SYRIAN_POUND),
        ("centimeter", &*CENTIMETER),
        ("m/h", &*METERS_PER_HOUR),
        ("micrograms_per_cubic_meter", &*MICROGRAMS_PER_CUBIC_METER),
        ("scm", &*CUBIC_METERS_NATURAL_GAS),
        ("MMK", &*KYAT),
        ("feet_per_minute", &*FEET_PER_MINUTE),
        ("rpm", &*REVOLUTIONS_PER_MINUTE),
        ("kilobyte", &*KILOBYTE),
        ("AZN", &*AZERBAIJANIAN_MANAT),
        ("kilometers_per_hour", &*KILOMETERS_PER_HOUR),
        ("SFr", &*SWISS_FRANC),
        ("kilopound", &*KILOPOUND),
        ("liters_per_hour", &*LITERS_PER_HOUR),
        ("lb/s", &*POUNDS_PER_SECOND),
        ("lb", &*POUND),
        ("Դ", &*ARMENIAN_DRAM),
        (
            "kilowatt_hours_per_square_meter",
            &*KILOWATT_HOURS_PER_SQUARE_METER
        ),
        ("miles_per_hour", &*MILES_PER_HOUR),
        ("KGS", &*SOM),
        ("kg/s", &*KILOGRAMS_PER_SECOND),
        ("norwegian_krone", &*NORWEGIAN_KRONE),
        ("zimbabwe_dollar", &*ZIMBABWE_DOLLAR),
        ("volts_per_degree_kelvin", &*VOLTS_PER_DEGREE_KELVIN),
        ("IQD", &*IRAQI_DINAR),
        ("fahrenheit_degrees", &*FAHRENHEIT_DEGREES),
        ("ohm_meter", &*OHM_METER),
        ("RWF", &*RWANDA_FRANC),
        ("dong", &*DONG),
        ("euro", &*EURO),
        (
            "watts_per_meter_degree_kelvin",
            &*WATTS_PER_METER_DEGREE_KELVIN
        ),
        (
            "cubic_feet_per_minute_per_square_foot",
            &*CUBIC_FEET_PER_MINUTE_PER_SQUARE_FOOT
        ),
        ("north_korean_won", &*NORTH_KOREAN_WON),
        ("C", &*COULOMB),
        ("HUF", &*FORINT),
        ("gigawatt_hour", &*GIGAWATT_HOUR),
        ("kBTU/ft²", &*KILOBTU_PER_SQUARE_FOOT),
        ("Vt", &*VATU),
        ("t", &*SHORT_TON),
        ("ounce", &*OUNCE),
        ("PHP", &*PHILIPPINE_PESO),
        ("kilovolt_ampere", &*KILOVOLT_AMPERE),
        ("celsius_degrees", &*CELSIUS_DEGREES),
        ("formazin_nephelometric_unit", &*FORMAZIN_NEPHELOMETRIC_UNIT),
        ("ft³", &*CUBIC_FOOT),
        ("UM", &*OUGUIYA),
        ("CNY", &*CHINESE_YUAN),
        ("iraqi_dinar", &*IRAQI_DINAR),
        ("mm", &*MILLIMETER),
        ("metric_tons_per_hour", &*METRIC_TONS_PER_HOUR),
        ("volt_ampere", &*VOLT_AMPERE),
        ("armenian_dram", &*ARMENIAN_DRAM),
        ("MBTU/ft²", &*MEGABTU_PER_SQUARE_FOOT),
        ("W/m²_irr", &*WATTS_PER_SQUARE_METER_IRRADIANCE),
        ("W/ft²", &*WATTS_PER_SQUARE_FOOT),
        ("MWK", &*KWACHA),
        ("MZN", &*METICAL),
        ("lb/h", &*POUNDS_PER_HOUR),
        ("solomon_islands_dollar", &*SOLOMON_ISLANDS_DOLLAR),
        ("BOB", &*BOLIVIANO),
        ("radians_per_second_squared", &*RADIANS_PER_SECOND_SQUARED),
        ("second", &*SECOND),
        ("kW/ft²", &*KILOWATTS_PER_SQUARE_FOOT),
        ("canadian_dollar", &*CANADIAN_DOLLAR),
        ("m²/N", &*SQUARE_METERS_PER_NEWTON),
        ("henry", &*HENRY),
        ("power_usage_effectiveness", &*POWER_USAGE_EFFECTIVENESS),
        ("MOP", &*PATACA),
        ("BIF", &*BURUNDI_FRANC),
        ("mg/m³", &*MILLIGRAMS_PER_CUBIC_METER),
        ("tonrefh", &*TONS_REFRIGERATION_HOUR),
        ("ft³_gas", &*CUBIC_FEET_NATURAL_GAS),
        ("qt", &*QUART),
        ("cubic_feet_per_second", &*CUBIC_FEET_PER_SECOND),
        ("VEF", &*BOLIVAR_FUERTE),
        ("guinea_franc", &*GUINEA_FRANC),
        ("kWh/ft²", &*KILOWATT_HOURS_PER_SQUARE_FOOT),
        ("ft/s", &*FEET_PER_SECOND),
        ("mm/s", &*MILLIMETERS_PER_SECOND),
        ("newton_meter", &*NEWTON_METER),
        ("tons_refrigeration_hour", &*TONS_REFRIGERATION_HOUR),
        ("%", &*PERCENT),
        ("wk", &*WEEK),
        ("square_mile", &*SQUARE_MILE),
        ("kilograms_per_hour", &*KILOGRAMS_PER_HOUR),
        ("ft/min", &*FEET_PER_MINUTE),
        ("foot_pounds_per_second", &*FOOT_POUNDS_PER_SECOND),
        ("fc", &*FOOTCANDLE),
        ("kilocubic_feet_per_minute", &*KILOCUBIC_FEET_PER_MINUTE),
        ("cedi", &*CEDI),
        ("SLL", &*LEONE),
        ("coefficient_of_performance", &*COEFFICIENT_OF_PERFORMANCE),
        ("K/min", &*DEGREES_KELVIN_PER_MINUTE),
        (
            "watts_per_cubic_feet_per_minute",
            &*WATTS_PER_CUBIC_FEET_PER_MINUTE
        ),
        ("mVA", &*MEGAVOLT_AMPERE),
        ("BGN", &*BULGARIAN_LEV),
        ("L/min", &*LITERS_PER_MINUTE),
        ("mile", &*MILE),
        ("µg/m³", &*MICROGRAMS_PER_CUBIC_METER),
        ("millimeters_per_hour", &*MILLIMETERS_PER_HOUR),
        ("grams_per_cubic_meter", &*GRAMS_PER_CUBIC_METER),
        ("tons_refrigeration", &*TONS_REFRIGERATION),
        ("JMD", &*JAMAICAN_DOLLAR),
        ("lari", &*LARI),
        ("inHg", &*INCHES_OF_MERCURY),
        ("BRL", &*BRAZILIAN_REAL),
        ("tenge", &*TENGE),
        ("ppu", &*PARTS_PER_UNIT),
        (
            "kilowatts_per_kilocubic_feet_per_minute",
            &*KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE
        ),
        ("MTn", &*METICAL),
        ("konvertibilna_marka", &*KONVERTIBILNA_MARKA),
        ("BAM", &*KONVERTIBILNA_MARKA),
        ("kilojoules_per_hour", &*KILOJOULES_PER_HOUR),
        ("Δ°F", &*FAHRENHEIT_DEGREES),
        ("Kč", &*CZECH_KORUNA),
        ("cfs", &*CUBIC_FEET_PER_SECOND),
        ("peso_uruguayo", &*PESO_URUGUAYO),
        ("kilowatts_per_square_meter", &*KILOWATTS_PER_SQUARE_METER),
        ("cfh", &*CUBIC_FEET_PER_HOUR),
        ("PB", &*PETABYTE),
        ("malaysian_ringgit", &*MALAYSIAN_RINGGIT),
        ("ман", &*AZERBAIJANIAN_MANAT),
        ("km/s", &*KILOMETERS_PER_SECOND),
        ("south_korean_won", &*SOUTH_KOREAN_WON),
        ("thousand_cubic_feet", &*THOUSAND_CUBIC_FEET),
        ("cs", &*HUNDREDTHS_SECOND),
        ("g/s", &*GRAMS_PER_SECOND),
        ("KYD", &*CAYMAN_ISLANDS_DOLLAR),
        ("saint_helena_pound", &*SAINT_HELENA_POUND),
        ("scf", &*CUBIC_FEET_NATURAL_GAS),
        ("degrees_fahrenheit_per_hour", &*DEGREES_FAHRENHEIT_PER_HOUR),
        ("Am²", &*AMPERE_SQUARE_METER),
        ("pataca", &*PATACA),
        ("/min", &*PER_MINUTE),
        ("square_foot", &*SQUARE_FOOT),
        ("joules_per_square_meter", &*JOULES_PER_SQUARE_METER),
        (
            "kilojoules_per_degree_kelvin",
            &*KILOJOULES_PER_DEGREE_KELVIN
        ),
        ("HRK", &*CROATIAN_KUNA),
        ("ampere_square_meter", &*AMPERE_SQUARE_METER),
        ("megajoules_per_square_meter", &*MEGAJOULES_PER_SQUARE_METER),
        ("weber", &*WEBER),
        ("mm³", &*CUBIC_MILLIMETER),
        ("km", &*KILOMETER),
        ("MMcf", &*MILLION_CUBIC_FEET),
        ("YER", &*YEMENI_RIAL),
        ("KHR", &*RIEL),
        ("MV", &*MEGAVOLT),
        ("ftlbs/sec", &*FOOT_POUNDS_PER_SECOND),
        ("tugrik", &*TUGRIK),
        ("°F/h", &*DEGREES_FAHRENHEIT_PER_HOUR),
        ("AUD", &*AUSTRALIAN_DOLLAR),
        ("gigajoules_per_hour", &*GIGAJOULES_PER_HOUR),
        ("Nfk", &*NAKFA),
        ("meters_per_second", &*METERS_PER_SECOND),
        ("zambian_kwacha", &*ZAMBIAN_KWACHA),
        ("gigabyte", &*GIGABYTE),
        ("trinidad_and_tobago_dollar", &*TRINIDAD_AND_TOBAGO_DOLLAR),
        ("zł", &*PZLOTY),
        ("standard_cubic_foot", &*CUBIC_FEET_NATURAL_GAS),
        ("joules_per_degree_kelvin", &*JOULES_PER_DEGREE_KELVIN),
        ("ft", &*FOOT),
        ("leu", &*LEU),
        (
            "megajoules_per_kilogram_dry_air",
            &*MEGAJOULES_PER_KILOGRAM_DRY_AIR
        ),
        ("grams_per_kilogram", &*GRAMS_PER_KILOGRAM),
        ("swedish_krona", &*SWEDISH_KRONA),
        ("MMBTU", &*MEGABTU),
        ("MAD", &*MOROCCAN_DIRHAM),
        ("ZMW", &*ZAMBIAN_KWACHA),
        ("SAR", &*SAUDI_RIYAL),
        ("square_yard", &*SQUARE_YARD),
        ("BTU/lb", &*BTU_PER_POUND),
        ("czech_koruna", &*CZECH_KORUNA),
        ("pounds_per_minute", &*POUNDS_PER_MINUTE),
        (
            "hundred_cubic_feet_natural_gas",
            &*HUNDRED_CUBIC_FEET_NATURAL_GAS
        ),
        ("Db", &*DOBRA),
        ("gigajoules_per_cubic_meter", &*GIGAJOULES_PER_CUBIC_METER),
        ("BSD", &*BAHAMIAN_DOLLAR),
        ("SHP", &*SAINT_HELENA_POUND),
        ("৳", &*TAKA),
        ("kg/m³", &*KILOGRAMS_PER_CUBIC_METER),
        ("footcandle", &*FOOTCANDLE),
        ("R$", &*BRAZILIAN_REAL),
        ("J/kg°K", &*JOULES_PER_KILOGRAM_DEGREE_KELVIN),
        ("amperes_per_meter", &*AMPERES_PER_METER),
        ("LBP", &*LEBANESE_POUND),
        ("/h", &*PER_HOUR),
        ("inches_per_hour", &*INCHES_PER_HOUR),
        ("lempira", &*LEMPIRA),
        ("MHz", &*MEGAHERTZ),
        ("candelas_per_square_meter", &*CANDELAS_PER_SQUARE_METER),
        ("ARS", &*ARGENTINE_PESO),
        ("₴", &*HRYVNIA),
        ("celsius", &*CELSIUS),
        ("pound", &*POUND),
        ("kgal", &*KILOGALLON),
        ("SBD", &*SOLOMON_ISLANDS_DOLLAR),
        (
            "nephelometric_turbidity_units",
            &*NEPHELOMETRIC_TURBIDITY_UNITS
        ),
        ("joules_per_hour", &*JOULES_PER_HOUR),
        ("cfm/ft²", &*CUBIC_FEET_PER_MINUTE_PER_SQUARE_FOOT),
        ("W/m²", &*WATTS_PER_SQUARE_METER),
        ("db", &*DECIBEL),
        ("kilowatts_per_square_foot", &*KILOWATTS_PER_SQUARE_FOOT),
        ("milliwatt", &*MILLIWATT),
        ("btu_per_pound", &*BTU_PER_POUND),
        ("turkish_lira", &*TURKISH_LIRA),
        ("rial_omani", &*RIAL_OMANI),
        ("gH₂O/kgAir", &*GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR),
        ("meters_per_hour", &*METERS_PER_HOUR),
        ("square_kilometer", &*SQUARE_KILOMETER),
        ("ns", &*NANOSECOND),
        ("millisecond", &*MILLISECOND),
        ("argentine_peso", &*ARGENTINE_PESO),
        ("m³/s", &*CUBIC_METERS_PER_SECOND),
        (
            "kilovolt_ampere_reactive_hour",
            &*KILOVOLT_AMPERE_REACTIVE_HOUR
        ),
        ("kW", &*KILOWATT),
        ("yd²", &*SQUARE_YARD),
        ("BTU/ft²", &*BTU_PER_SQUARE_FOOT),
        ("watts_per_square_meter", &*WATTS_PER_SQUARE_METER),
        ("kWh", &*KILOWATT_HOUR),
        ("degrees_kelvin_per_hour", &*DEGREES_KELVIN_PER_HOUR),
        ("GJ/m³", &*GIGAJOULES_PER_CUBIC_METER),
        ("฿", &*BAHT),
        ("₹", &*INDIAN_RUPEE),
        ("ZK", &*ZAMBIAN_KWACHA),
        ("volt_ampere_hour", &*VOLT_AMPERE_HOUR),
        ("MBTU", &*MEGABTU),
        ("lek", &*LEK),
        ("tanzanian_shilling", &*TANZANIAN_SHILLING),
        ("ohm", &*OHM),
        ("MKD", &*DENAR),
        ("joule", &*JOULE),
        ("in²", &*SQUARE_INCH),
        ("DCIE", &*DATA_CENTER_INFRASTRUCTURE_EFFICIENCY),
        ("taka", &*TAKA),
        ("Ccf_natural_gas", &*CENTUM_CUBIC_FEET_NATURAL_GAS),
        ("EGP", &*EGYPTIAN_POUND),
        ("cubic_inch", &*CUBIC_INCH),
        ("MWh/ft²", &*MEGAWATT_HOURS_PER_SQUARE_FOOT),
        ("V/m", &*VOLTS_PER_METER),
        ("kilogram", &*KILOGRAM),
        ("fnu", &*FORMAZIN_NEPHELOMETRIC_UNIT),
        ("julian_month", &*JULIAN_MONTH),
        ("hL", &*HECTOLITER),
        ("volt", &*VOLT),
        ("cpm", &*CYCLES_PER_MINUTE),
        ("suriname_dollar", &*SURINAME_DOLLAR),
        ("%obsc/m", &*PERCENT_OBSCURATION_PER_METER),
        ("psi/°F", &*PSI_PER_DEGREE_FAHRENHEIT),
        ("therm", &*THERM),
        ("hundred_cubic_feet", &*HUNDRED_CUBIC_FEET),
        ("liters_per_minute", &*LITERS_PER_MINUTE),
        ("syrian_pound", &*SYRIAN_POUND),
        ("₮", &*TUGRIK),
        ("JOD", &*JORDANIAN_DINAR),
        ("feet_per_second", &*FEET_PER_SECOND),
        ("tenths_second", &*TENTHS_SECOND),
        ("ден", &*DENAR),
        ("µm", &*MICROMETER),
        ("GIP", &*GIBRALTAR_POUND),
        ("KZT", &*TENGE),
        ("GHS", &*CEDI),
        ("boliviano", &*BOLIVIANO),
        ("rand", &*RAND),
        ("in", &*INCH),
        ("ms", &*MILLISECOND),
        ("per_minute", &*PER_MINUTE),
        ("megawatt", &*MEGAWATT),
        ("MJ/°K", &*MEGAJOULES_PER_DEGREE_KELVIN),
        ("ლ", &*LARI),
        ("m", &*METER),
        (
            "joules_per_kilogram_degree_kelvin",
            &*JOULES_PER_KILOGRAM_DEGREE_KELVIN
        ),
        ("yard", &*YARD),
        ("pounds_per_hour", &*POUNDS_PER_HOUR),
        ("mΩ", &*MILLIOHM),
        ("UAH", &*HRYVNIA),
        ("grams_per_minute", &*GRAMS_PER_MINUTE),
        ("kilobtu_per_square_foot", &*KILOBTU_PER_SQUARE_FOOT),
        ("g/m³", &*GRAMS_PER_CUBIC_METER),
        ("MJ", &*MEGAJOULE),
        ("N", &*NEWTON),
        ("milliliters_per_second", &*MILLILITERS_PER_SECOND),
        ("kilograms_per_square_meter", &*KILOGRAMS_PER_SQUARE_METER),
        ("BHD", &*BAHRAINI_DINAR),
        ("₱", &*PHILIPPINE_PESO),
        ("Af", &*AFGHANI),
        ("degrees_phase", &*DEGREES_PHASE),
        ("moldavian_leu", &*MOLDAVIAN_LEU),
        ("kW/m²", &*KILOWATTS_PER_SQUARE_METER),
        ("square_meters_per_newton", &*SQUARE_METERS_PER_NEWTON),
        ("Js", &*JOULE_SECOND),
        ("kg", &*KILOGRAM),
        ("djibouti_franc", &*DJIBOUTI_FRANC),
        ("cubic_centimeter", &*CUBIC_CENTIMETER),
        ("m³/h", &*CUBIC_METERS_PER_HOUR),
        ("horsepower", &*HORSEPOWER),
        ("CHF", &*SWISS_FRANC),
        ("m³/min", &*CUBIC_METERS_PER_MINUTE),
        ("PGK", &*KINA),
        ("grams_per_second", &*GRAMS_PER_SECOND),
        ("pt", &*PINT),
        ("milliliter", &*MILLILITER),
        ("malagasy_ariary", &*MALAGASY_ARIARY),
        ("VUV", &*VATU),
        ("mg/L", &*MILLIGRAMS_PER_LITER),
        ("FKP", &*FALKLAND_ISLANDS_POUND),
        ("m/s²", &*METERS_PER_SECOND_SQUARED),
        ("pakistan_rupee", &*PAKISTAN_RUPEE),
        ("W", &*WATT),
        ("siemens_per_meter", &*SIEMENS_PER_METER),
        ("HKD", &*HONG_KONG_DOLLAR),
        ("Ωm", &*OHM_METER),
        (
            "megajoules_per_degree_kelvin",
            &*MEGAJOULES_PER_DEGREE_KELVIN
        ),
        ("cubic_meters_per_minute", &*CUBIC_METERS_PER_MINUTE),
        ("°C/h", &*DEGREES_CELSIUS_PER_HOUR),
        ("milligram", &*MILLIGRAM),
        ("gallons_per_hour", &*GALLONS_PER_HOUR),
        ("candela", &*CANDELA),
        ("radians_per_second", &*RADIANS_PER_SECOND),
        ("db_millivolt", &*DB_MILLIVOLT),
        ("W/cfm", &*WATTS_PER_CUBIC_FEET_PER_MINUTE),
        ("liter", &*LITER),
        ("RUB", &*RUSSIAN_RUBLE),
        ("ouguiya", &*OUGUIYA),
        ("leone", &*LEONE),
        ("watt_hours_per_square_foot", &*WATT_HOURS_PER_SQUARE_FOOT),
        ("kJ/kg_dry", &*KILOJOULES_PER_KILOGRAM_DRY_AIR),
        ("psi", &*POUNDS_PER_SQUARE_INCH),
        ("siemens", &*SIEMENS),
        ("pf", &*POWER_FACTOR),
    ]
    .iter()
    .cloned()
    .collect();
}
