// Copyright (C) 2020 - 2022, J2 Innovations
// Haystack Unit module - auto generated.

#![allow(clippy::approx_constant)]
use super::{Unit, UnitDimensions};
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
        scale: 0.000001,
        offset: 0.0,
    };
}

lazy_static! {
    pub static ref PARTS_PER_BILLION: Unit = Unit {
        quantity: Some("dimensionless".to_string(),),
        ids: ["parts_per_billion".to_string(), "ppb".to_string(),].to_vec(),
        dimensions: None,
        scale: 0.000000001,
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
        scale: 0.000001,
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
        scale: 0.000001,
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
        scale: 0.000000001,
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
        ids: ["cubic_meters_natural_gas".to_string(), "m³_gas".to_string(),].to_vec(),
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
        ids: ["cubic_feet_natural_gas".to_string(), "ft³_gas".to_string(),].to_vec(),
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
        scale: 0.00001,
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
        scale: 0.000001,
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
        scale: 0.000016666666666666667,
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
        scale: 0.000000001,
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
        scale: 0.000001,
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
        scale: 0.000016666666666666667,
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
        scale: 0.000000001,
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
        scale: 0.000001,
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
        scale: 0.000001,
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
        scale: 0.000016387064,
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
        scale: 0.0000295729,
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
        scale: 0.000001,
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
        scale: 0.000007866,
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
        scale: 0.000016666666666666667,
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
        scale: 0.0000630901964,
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
        scale: 0.0000010515033,
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
        scale: 0.00000027777777777777776,
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
        ("algerian_dinar", &*ALGERIAN_DINAR),
        ("micrograms_per_cubic_meter", &*MICROGRAMS_PER_CUBIC_METER),
        ("joules_per_gram", &*JOULES_PER_GRAM),
        ("THB", &*BAHT),
        ("joule", &*JOULE),
        ("therm", &*THERM),
        ("CRC", &*COSTA_RICAN_COLON),
        ("EER", &*ENERGY_EFFICIENCY_RATIO),
        ("元", &*CHINESE_YUAN),
        ("meters_per_minute", &*METERS_PER_MINUTE),
        ("GW", &*GIGAWATT),
        ("kJ/°K", &*KILOJOULES_PER_DEGREE_KELVIN),
        ("kilobyte", &*KILOBYTE),
        ("TTD", &*TRINIDAD_AND_TOBAGO_DOLLAR),
        ("kiloliter", &*KILOLITER),
        ("congolese_franc", &*CONGOLESE_FRANC),
        ("DCIE", &*DATA_CENTER_INFRASTRUCTURE_EFFICIENCY),
        ("meters_per_second_squared", &*METERS_PER_SECOND_SQUARED),
        ("inches_of_mercury", &*INCHES_OF_MERCURY),
        ("kenyan_shilling", &*KENYAN_SHILLING),
        ("PB", &*PETABYTE),
        ("radian", &*RADIAN),
        ("gigajoules_per_hour", &*GIGAJOULES_PER_HOUR),
        ("RSD", &*SERBIAN_DINAR),
        ("ZK", &*ZAMBIAN_KWACHA),
        ("candels_per_square_foot", &*CANDELS_PER_SQUARE_FOOT),
        ("mo", &*JULIAN_MONTH),
        ("RUB", &*RUSSIAN_RUBLE),
        ("sri_lanka_rupee", &*SRI_LANKA_RUPEE),
        ("tanzanian_shilling", &*TANZANIAN_SHILLING),
        ("Ωm", &*OHM_METER),
        ("mexican_peso", &*MEXICAN_PESO),
        ("candelas_per_square_meter", &*CANDELAS_PER_SQUARE_METER),
        (
            "megajoules_per_kilogram_dry_air",
            &*MEGAJOULES_PER_KILOGRAM_DRY_AIR
        ),
        ("ман", &*AZERBAIJANIAN_MANAT),
        ("tonrefh", &*TONS_REFRIGERATION_HOUR),
        ("psi_per_degree_fahrenheit", &*PSI_PER_DEGREE_FAHRENHEIT),
        ("BWP", &*PULA),
        ("cape_verde_escudo", &*CAPE_VERDE_ESCUDO),
        ("SDG", &*SUDANESE_POUND),
        ("TZS", &*TANZANIAN_SHILLING),
        ("uganda_shilling", &*UGANDA_SHILLING),
        ("degrees_kelvin_per_second", &*DEGREES_KELVIN_PER_SECOND),
        ("kilojoule", &*KILOJOULE),
        ("kJ", &*KILOJOULE),
        ("ohm_meter", &*OHM_METER),
        ("weber", &*WEBER),
        (
            "data_center_infrastructure_efficiency",
            &*DATA_CENTER_INFRASTRUCTURE_EFFICIENCY
        ),
        ("IDR", &*RUPIAH),
        ("aruban_guilder_florin", &*ARUBAN_GUILDER_FLORIN),
        ("N/m", &*NEWTONS_PER_METER),
        ("qt", &*QUART),
        ("taka", &*TAKA),
        ("AMD", &*ARMENIAN_DRAM),
        ("watt_hour", &*WATT_HOUR),
        ("chilean_peso", &*CHILEAN_PESO),
        ("pascal", &*PASCAL),
        (
            "megawatt_hours_per_square_foot",
            &*MEGAWATT_HOURS_PER_SQUARE_FOOT
        ),
        ("kJ/h", &*KILOJOULES_PER_HOUR),
        ("ft²", &*SQUARE_FOOT),
        ("BYR", &*BELARUSSIAN_RUBLE),
        ("volt_ampere_hour", &*VOLT_AMPERE_HOUR),
        ("cubic_feet_natural_gas", &*CUBIC_FEET_NATURAL_GAS),
        ("BAM", &*KONVERTIBILNA_MARKA),
        ("ACH", &*AIR_CHANGES_PER_HOUR),
        ("$", &*US_DOLLAR),
        ("kcfm", &*KILOCUBIC_FEET_PER_MINUTE),
        ("HKD", &*HONG_KONG_DOLLAR),
        ("XCD", &*EAST_CARIBBEAN_DOLLAR),
        ("MTn", &*METICAL),
        ("KRW", &*SOUTH_KOREAN_WON),
        ("liters_per_minute", &*LITERS_PER_MINUTE),
        ("liberian_dollar", &*LIBERIAN_DOLLAR),
        ("Js", &*JOULE_SECOND),
        ("radians_per_second", &*RADIANS_PER_SECOND),
        ("°F/h", &*DEGREES_FAHRENHEIT_PER_HOUR),
        ("Kn", &*CROATIAN_KUNA),
        ("WST", &*TALA),
        ("৳", &*TAKA),
        ("UGX", &*UGANDA_SHILLING),
        ("kilograms_per_minute", &*KILOGRAMS_PER_MINUTE),
        ("%", &*PERCENT),
        ("cubic_millimeter", &*CUBIC_MILLIMETER),
        ("newton", &*NEWTON),
        (
            "watts_per_square_foot_irradiance",
            &*WATTS_PER_SQUARE_FOOT_IRRADIANCE
        ),
        ("YER", &*YEMENI_RIAL),
        ("cd", &*CANDELA),
        (
            "watts_per_cubic_feet_per_minute",
            &*WATTS_PER_CUBIC_FEET_PER_MINUTE
        ),
        ("kWh", &*KILOWATT_HOUR),
        ("OMR", &*RIAL_OMANI),
        ("AED", &*UAE_DIRHAM),
        ("km", &*KILOMETER),
        ("atm", &*ATMOSPHERE),
        ("australian_dollar", &*AUSTRALIAN_DOLLAR),
        ("tesla", &*TESLA),
        ("PYG", &*GUARANI),
        ("milliampere", &*MILLIAMPERE),
        ("fahrenheit_degrees", &*FAHRENHEIT_DEGREES),
        ("watt_hours_per_square_foot", &*WATT_HOURS_PER_SQUARE_FOOT),
        ("watts_per_square_meter", &*WATTS_PER_SQUARE_METER),
        ("GIP", &*GIBRALTAR_POUND),
        ("air_changes_per_hour", &*AIR_CHANGES_PER_HOUR),
        ("CDF", &*CONGOLESE_FRANC),
        ("rufiyaa", &*RUFIYAA),
        ("N", &*NEWTON),
        ("€", &*EURO),
        ("BBD", &*BARBADOS_DOLLAR),
        ("yen", &*YEN),
        ("kilowatt_per_ton", &*KILOWATT_PER_TON),
        ("W/ft²_irr", &*WATTS_PER_SQUARE_FOOT_IRRADIANCE),
        ("SBD", &*SOLOMON_ISLANDS_DOLLAR),
        ("BOB", &*BOLIVIANO),
        ("cubic_feet_per_second", &*CUBIC_FEET_PER_SECOND),
        ("RON", &*LEU),
        ("peso_uruguayo", &*PESO_URUGUAYO),
        ("cubic_feet_per_minute", &*CUBIC_FEET_PER_MINUTE),
        ("mΩ", &*MILLIOHM),
        ("rwanda_franc", &*RWANDA_FRANC),
        ("lx", &*LUX),
        ("VUV", &*VATU),
        ("grams_per_minute", &*GRAMS_PER_MINUTE),
        ("MUR", &*MAURITIUS_RUPEE),
        ("°C/min", &*DEGREES_CELSIUS_PER_MINUTE),
        ("m/s²", &*METERS_PER_SECOND_SQUARED),
        ("MNT", &*TUGRIK),
        ("kVAh", &*KILOVOLT_AMPERE_HOUR),
        ("megahertz", &*MEGAHERTZ),
        ("Wb", &*WEBER),
        ("terabyte", &*TERABYTE),
        ("mm/s", &*MILLIMETERS_PER_SECOND),
        ("m³/h", &*CUBIC_METERS_PER_HOUR),
        ("kVAR", &*KILOVOLT_AMPERE_REACTIVE),
        ("Ωm", &*OHM_METER),
        ("MVAh", &*MEGAVOLT_AMPERE_HOUR),
        ("₦", &*NAIRA),
        ("joules_per_square_meter", &*JOULES_PER_SQUARE_METER),
        ("gal", &*GALLON),
        ("cuban_peso", &*CUBAN_PESO),
        ("ethiopian_birr", &*ETHIOPIAN_BIRR),
        ("megavolt_ampere_reactive", &*MEGAVOLT_AMPERE_REACTIVE),
        ("F", &*FARAD),
        ("cm", &*CENTIMETER),
        ("%obsc/ft", &*PERCENT_OBSCURATION_PER_FOOT),
        ("W/m²K", &*WATTS_PER_SQUARE_METER_DEGREE_KELVIN),
        (
            "watts_per_square_meter_irradiance",
            &*WATTS_PER_SQUARE_METER_IRRADIANCE
        ),
        ("rupiah", &*RUPIAH),
        ("cubic_inch", &*CUBIC_INCH),
        ("dong", &*DONG),
        ("km/h", &*KILOMETERS_PER_HOUR),
        ("W/m²_irr", &*WATTS_PER_SQUARE_METER_IRRADIANCE),
        ("Db", &*DOBRA),
        ("hour", &*HOUR),
        ("nuevo_sol", &*NUEVO_SOL),
        ("saudi_riyal", &*SAUDI_RIYAL),
        ("amperes_per_square_meter", &*AMPERES_PER_SQUARE_METER),
        ("cfa_franc_bceao", &*CFA_FRANC_BCEAO),
        ("joule_second", &*JOULE_SECOND),
        ("ampere_square_meter", &*AMPERE_SQUARE_METER),
        ("kilohm", &*KILOHM),
        ("kilograms_per_cubic_meter", &*KILOGRAMS_PER_CUBIC_METER),
        ("norwegian_krone", &*NORWEGIAN_KRONE),
        ("DOP", &*DOMINICAN_PESO),
        ("MVAR", &*MEGAVOLT_AMPERE_REACTIVE),
        ("degree_days_fahrenheit", &*DEGREE_DAYS_FAHRENHEIT),
        ("milliliter", &*MILLILITER),
        ("¥", &*YEN),
        ("square_foot", &*SQUARE_FOOT),
        ("TMT", &*MANAT),
        ("cfs", &*CUBIC_FEET_PER_SECOND),
        ("taiwan_dollar", &*TAIWAN_DOLLAR),
        ("ISK", &*ICELAND_KRONA),
        ("C$", &*CORDOBA_ORO),
        ("ERN", &*NAKFA),
        ("pf", &*POWER_FACTOR),
        ("kilowatts_per_square_meter", &*KILOWATTS_PER_SQUARE_METER),
        ("kilobtus_per_hour", &*KILOBTUS_PER_HOUR),
        ("tenge", &*TENGE),
        ("LSL", &*LOTI),
        ("BTN", &*NGULTRUM),
        ("canadian_dollar", &*CANADIAN_DOLLAR),
        ("COP", &*COEFFICIENT_OF_PERFORMANCE),
        ("kilovolt_ampere_hour", &*KILOVOLT_AMPERE_HOUR),
        ("kVA", &*KILOVOLT_AMPERE),
        ("lek", &*LEK),
        ("UZS", &*UZBEKISTAN_SUM),
        ("coulomb", &*COULOMB),
        ("cubic_meters_natural_gas", &*CUBIC_METERS_NATURAL_GAS),
        ("DZD", &*ALGERIAN_DINAR),
        ("AWG", &*ARUBAN_GUILDER_FLORIN),
        ("V/m", &*VOLTS_PER_METER),
        ("JMD", &*JAMAICAN_DOLLAR),
        ("ph", &*PH),
        ("g/kg", &*GRAMS_PER_KILOGRAM),
        ("belize_dollar", &*BELIZE_DOLLAR),
        ("sudanese_pound", &*SUDANESE_POUND),
        ("MVR", &*RUFIYAA),
        ("lb/min", &*POUNDS_PER_MINUTE),
        ("btu", &*BTU),
        ("Kr", &*ICELAND_KRONA),
        ("volt_ampere_reactive_hour", &*VOLT_AMPERE_REACTIVE_HOUR),
        ("pounds_per_minute", &*POUNDS_PER_MINUTE),
        ("pounds_per_square_inch", &*POUNDS_PER_SQUARE_INCH),
        ("FJD", &*FIJI_DOLLAR),
        ("day", &*DAY),
        ("psi/°F", &*PSI_PER_DEGREE_FAHRENHEIT),
        ("power_usage_effectiveness", &*POWER_USAGE_EFFECTIVENESS),
        ("UAH", &*HRYVNIA),
        ("btu_per_pound", &*BTU_PER_POUND),
        ("volt_ampere", &*VOLT_AMPERE),
        ("h", &*HOUR),
        ("mW", &*MILLIWATT),
        ("lb/h", &*POUNDS_PER_HOUR),
        ("psi", &*POUNDS_PER_SQUARE_INCH),
        ("megajoule", &*MEGAJOULE),
        ("celsius_degrees", &*CELSIUS_DEGREES),
        ("Ω", &*OHM),
        ("UM", &*OUGUIYA),
        ("btus_per_pound_dry_air", &*BTUS_PER_POUND_DRY_AIR),
        ("miles_per_hour", &*MILES_PER_HOUR),
        ("g/m²", &*GRAMS_PER_SQUARE_METER),
        ("MJ", &*MEGAJOULE),
        ("MJ/kg_dry", &*MEGAJOULES_PER_KILOGRAM_DRY_AIR),
        ("azerbaijanian_manat", &*AZERBAIJANIAN_MANAT),
        ("L/h", &*LITERS_PER_HOUR),
        ("cm³", &*CUBIC_CENTIMETER),
        (
            "megavolt_ampere_reactive_hour",
            &*MEGAVOLT_AMPERE_REACTIVE_HOUR
        ),
        ("tala", &*TALA),
        ("dobra", &*DOBRA),
        ("nakfa", &*NAKFA),
        ("Wh/m²", &*WATT_HOURS_PER_SQUARE_METER),
        ("per_hour", &*PER_HOUR),
        (
            "degrees_fahrenheit_per_minute",
            &*DEGREES_FAHRENHEIT_PER_MINUTE
        ),
        ("kilograms_per_second", &*KILOGRAMS_PER_SECOND),
        ("XPF", &*CFP_FRANC),
        ("₲", &*GUARANI),
        ("deg", &*DEGREES_ANGULAR),
        ("MWK", &*KWACHA),
        ("north_korean_won", &*NORTH_KOREAN_WON),
        ("tons_refrigeration_hour", &*TONS_REFRIGERATION_HOUR),
        ("foot_pounds_per_second", &*FOOT_POUNDS_PER_SECOND),
        ("AZN", &*AZERBAIJANIAN_MANAT),
        ("H", &*HENRY),
        ("square_inch", &*SQUARE_INCH),
        ("kilobtu", &*KILOBTU),
        ("milliohm", &*MILLIOHM),
        ("ft/s", &*FEET_PER_SECOND),
        ("somali_shilling", &*SOMALI_SHILLING),
        ("mA", &*MILLIAMPERE),
        ("cedi", &*CEDI),
        ("fc", &*FOOTCANDLE),
        ("nepalese_rupee", &*NEPALESE_RUPEE),
        ("din", &*SERBIAN_DINAR),
        ("ppu", &*PARTS_PER_UNIT),
        ("₨", &*PAKISTAN_RUPEE),
        ("GEL", &*LARI),
        ("Rs", &*SRI_LANKA_RUPEE),
        ("J/kg", &*JOULES_PER_KILOGRAM),
        ("milligram", &*MILLIGRAM),
        (
            "kilojoules_per_degree_kelvin",
            &*KILOJOULES_PER_DEGREE_KELVIN
        ),
        ("kilopounds_per_hour", &*KILOPOUNDS_PER_HOUR),
        ("zimbabwe_dollar", &*ZIMBABWE_DOLLAR),
        ("manat", &*MANAT),
        ("gram", &*GRAM),
        ("TWD", &*TAIWAN_DOLLAR),
        ("east_caribbean_dollar", &*EAST_CARIBBEAN_DOLLAR),
        ("acre", &*ACRE),
        ("Nm", &*NEWTON_METER),
        (
            "megajoules_per_degree_kelvin",
            &*MEGAJOULES_PER_DEGREE_KELVIN
        ),
        ("kilograms_per_square_meter", &*KILOGRAMS_PER_SQUARE_METER),
        ("EUR", &*EURO),
        ("mL", &*MILLILITER),
        ("GTQ", &*QUETZAL),
        ("metric_ton", &*METRIC_TON),
        ("dBmV", &*DB_MILLIVOLT),
        ("joules_per_hour", &*JOULES_PER_HOUR),
        ("°F", &*FAHRENHEIT),
        ("Nfk", &*NAKFA),
        ("MJ/ft²", &*MEGAJOULES_PER_SQUARE_FOOT),
        ("BND", &*BRUNEI_DOLLAR),
        ("kJ/kg_dry", &*KILOJOULES_PER_KILOGRAM_DRY_AIR),
        ("BIF", &*BURUNDI_FRANC),
        ("megavolt_ampere_hour", &*MEGAVOLT_AMPERE_HOUR),
        ("mm", &*MILLIMETER),
        ("cfm", &*CUBIC_FEET_PER_MINUTE),
        ("HUF", &*FORINT),
        ("kVARh", &*KILOVOLT_AMPERE_REACTIVE_HOUR),
        ("horsepower", &*HORSEPOWER),
        ("square_millimeter", &*SQUARE_MILLIMETER),
        ("rial_omani", &*RIAL_OMANI),
        ("ft/min", &*FEET_PER_MINUTE),
        ("AUD", &*AUSTRALIAN_DOLLAR),
        ("quart", &*QUART),
        ("kBTU/ft²", &*KILOBTU_PER_SQUARE_FOOT),
        ("SOS", &*SOMALI_SHILLING),
        ("leone", &*LEONE),
        ("kΩ", &*KILOHM),
        ("steradian", &*STERADIAN),
        ("£", &*POUND_STERLING),
        ("kΩ", &*KILOHM),
        ("ft³_gas", &*CUBIC_FEET_NATURAL_GAS),
        ("Դ", &*ARMENIAN_DRAM),
        ("mph", &*MILES_PER_HOUR),
        ("Pa", &*PASCAL),
        ("joules_per_kilogram", &*JOULES_PER_KILOGRAM),
        ("degPh", &*DEGREES_PHASE),
        ("%RH", &*PERCENT_RELATIVE_HUMIDITY),
        ("CNY", &*CHINESE_YUAN),
        ("kW/ton", &*KILOWATT_PER_TON),
        ("cd/ft²", &*CANDELS_PER_SQUARE_FOOT),
        ("T", &*TESLA),
        ("/s", &*PER_SECOND),
        ("quetzal", &*QUETZAL),
        ("MDL", &*MOLDAVIAN_LEU),
        ("mg/m³", &*MILLIGRAMS_PER_CUBIC_METER),
        ("amperes_per_meter", &*AMPERES_PER_METER),
        ("us_dollar", &*US_DOLLAR),
        ("degrees_celsius_per_minute", &*DEGREES_CELSIUS_PER_MINUTE),
        ("degrees_fahrenheit_per_hour", &*DEGREES_FAHRENHEIT_PER_HOUR),
        ("lm", &*LUMEN),
        ("kg/m²", &*KILOGRAMS_PER_SQUARE_METER),
        ("hp", &*HORSEPOWER),
        ("euro", &*EURO),
        ("W/m°K", &*WATTS_PER_METER_DEGREE_KELVIN),
        ("km/s", &*KILOMETERS_PER_SECOND),
        ("barbados_dollar", &*BARBADOS_DOLLAR),
        ("cubic_centimeter", &*CUBIC_CENTIMETER),
        ("millivolt", &*MILLIVOLT),
        ("ALL", &*LEK),
        ("%/s", &*PERCENT_PER_SECOND),
        ("per_minute", &*PER_MINUTE),
        ("cycles_per_minute", &*CYCLES_PER_MINUTE),
        ("lux", &*LUX),
        ("HRK", &*CROATIAN_KUNA),
        ("PKR", &*PAKISTAN_RUPEE),
        ("J/°K", &*JOULES_PER_DEGREE_KELVIN),
        ("GB", &*GIGABYTE),
        ("brazilian_real", &*BRAZILIAN_REAL),
        ("iceland_krona", &*ICELAND_KRONA),
        ("joules_per_degree_kelvin", &*JOULES_PER_DEGREE_KELVIN),
        ("m/h", &*METERS_PER_HOUR),
        ("gourde", &*GOURDE),
        ("SLL", &*LEONE),
        ("%obsc/m", &*PERCENT_OBSCURATION_PER_METER),
        ("watt_hours_per_square_meter", &*WATT_HOURS_PER_SQUARE_METER),
        ("NGN", &*NAIRA),
        ("yard", &*YARD),
        ("feet_per_second", &*FEET_PER_SECOND),
        ("L/min", &*LITERS_PER_MINUTE),
        ("kilometers_per_hour", &*KILOMETERS_PER_HOUR),
        ("moroccan_dirham", &*MOROCCAN_DIRHAM),
        ("kilohertz", &*KILOHERTZ),
        ("BMD", &*BERMUDIAN_DOLLAR),
        ("jamaican_dollar", &*JAMAICAN_DOLLAR),
        ("megawatt_hour", &*MEGAWATT_HOUR),
        ("лв", &*BULGARIAN_LEV),
        ("costa_rican_colon", &*COSTA_RICAN_COLON),
        ("indian_rupee", &*INDIAN_RUPEE),
        ("SYP", &*SYRIAN_POUND),
        ("µs", &*MICROSECOND),
        ("SEK", &*SWEDISH_KRONA),
        ("J/kg_dry", &*JOULES_PER_KILOGRAM_DRY_AIR),
        ("lb/s", &*POUNDS_PER_SECOND),
        ("MMBTU", &*MEGABTU),
        ("QAR", &*QATARI_RIAL),
        ("hPa", &*HECTOPASCAL),
        ("ETB", &*ETHIOPIAN_BIRR),
        ("JPY", &*YEN),
        ("Btu/Wh", &*ENERGY_EFFICIENCY_RATIO),
        ("GMD", &*DALASI),
        ("IRR", &*IRANIAN_RIAL),
        ("dBµV", &*DB_MICROVOLT),
        ("klb", &*KILOPOUND),
        ("bar", &*BAR),
        ("cubic_yard", &*CUBIC_YARD),
        ("pounds_per_hour", &*POUNDS_PER_HOUR),
        ("kg", &*KILOGRAM),
        ("meter", &*METER),
        ("new_zealand_dollar", &*NEW_ZEALAND_DOLLAR),
        ("volts_per_degree_kelvin", &*VOLTS_PER_DEGREE_KELVIN),
        ("EGP", &*EGYPTIAN_POUND),
        ("year", &*YEAR),
        ("mm³", &*CUBIC_MILLIMETER),
        ("cal/g", &*CALORIE_PER_GRAM),
        ("ppm", &*PARTS_PER_MILLION),
        ("chinese_yuan", &*CHINESE_YUAN),
        ("hecto_cubic_foot", &*HECTO_CUBIC_FOOT),
        ("kilopound", &*KILOPOUND),
        ("watt", &*WATT),
        ("ILS", &*NEW_ISRAELI_SHEKEL),
        ("short_ton", &*SHORT_TON),
        ("kilojoules_per_hour", &*KILOJOULES_PER_HOUR),
        ("in/wc", &*INCHES_OF_WATER),
        ("syrian_pound", &*SYRIAN_POUND),
        ("megajoules_per_square_foot", &*MEGAJOULES_PER_SQUARE_FOOT),
        ("CLP", &*CHILEAN_PESO),
        ("MBTU/ft²", &*MEGABTU_PER_SQUARE_FOOT),
        ("L/s", &*LITERS_PER_SECOND),
        ("KZT", &*TENGE),
        ("volt", &*VOLT),
        ("boliviano", &*BOLIVIANO),
        ("radians_per_second_squared", &*RADIANS_PER_SECOND_SQUARED),
        ("byte", &*BYTE),
        ("STD", &*DOBRA),
        ("Am²", &*AMPERE_SQUARE_METER),
        ("fnu", &*FORMAZIN_NEPHELOMETRIC_UNIT),
        ("naira", &*NAIRA),
        ("MOP", &*PATACA),
        ("moldavian_leu", &*MOLDAVIAN_LEU),
        ("kilojoules_per_kilogram", &*KILOJOULES_PER_KILOGRAM),
        ("BZD", &*BELIZE_DOLLAR),
        ("in", &*INCH),
        ("g/s", &*GRAMS_PER_SECOND),
        ("afghani", &*AFGHANI),
        ("CZK", &*CZECH_KORUNA),
        ("hectoliter", &*HECTOLITER),
        ("SZL", &*LILANGENI),
        ("square_meters_per_newton", &*SQUARE_METERS_PER_NEWTON),
        ("siemens_per_meter", &*SIEMENS_PER_METER),
        ("falkland_islands_pound", &*FALKLAND_ISLANDS_POUND),
        ("kilocubic_feet_per_minute", &*KILOCUBIC_FEET_PER_MINUTE),
        ("liters_per_hour", &*LITERS_PER_HOUR),
        ("megohm", &*MEGOHM),
        ("₭", &*KIP),
        ("galUK", &*IMPERIAL_GALLON),
        ("min", &*MINUTE),
        ("knot", &*KNOT),
        ("pixel", &*PIXEL),
        ("seychelles_rupee", &*SEYCHELLES_RUPEE),
        ("K/min", &*DEGREES_KELVIN_PER_MINUTE),
        ("megawatt", &*MEGAWATT),
        ("cfp_franc", &*CFP_FRANC),
        ("cs", &*HUNDREDTHS_SECOND),
        ("mg", &*MILLIGRAM),
        ("second", &*SECOND),
        ("AOA", &*KWANZA),
        ("uzbekistan_sum", &*UZBEKISTAN_SUM),
        ("cph", &*CYCLES_PER_HOUR),
        ("BSD", &*BAHAMIAN_DOLLAR),
        ("Δ°C", &*CELSIUS_DEGREES),
        ("ngultrum", &*NGULTRUM),
        ("megabtu", &*MEGABTU),
        ("KGS", &*SOM),
        ("btus_per_hour", &*BTUS_PER_HOUR),
        ("m/min", &*METERS_PER_MINUTE),
        ("inches_of_water", &*INCHES_OF_WATER),
        ("AFN", &*AFGHANI),
        ("power_factor", &*POWER_FACTOR),
        ("W/cfm", &*WATTS_PER_CUBIC_FEET_PER_MINUTE),
        ("L", &*LITER),
        (
            "watts_per_square_meter_degree_kelvin",
            &*WATTS_PER_SQUARE_METER_DEGREE_KELVIN
        ),
        ("kwanza", &*KWANZA),
        ("kilogram", &*KILOGRAM),
        ("cubic_meters_per_minute", &*CUBIC_METERS_PER_MINUTE),
        ("bahraini_dinar", &*BAHRAINI_DINAR),
        ("megajoules_per_square_meter", &*MEGAJOULES_PER_SQUARE_METER),
        (
            "kilobtus_per_hour_per_square_foot",
            &*KILOBTUS_PER_HOUR_PER_SQUARE_FOOT
        ),
        ("NAD", &*NAMIBIA_DOLLAR),
        ("ampere_hour", &*AMPERE_HOUR),
        ("cordoba_oro", &*CORDOBA_ORO),
        ("ΔK", &*KELVIN_DEGREES),
        ("julian_month", &*JULIAN_MONTH),
        ("metric_tons_per_hour", &*METRIC_TONS_PER_HOUR),
        ("kilograms_per_hour", &*KILOGRAMS_PER_HOUR),
        ("denar", &*DENAR),
        ("A/m", &*AMPERES_PER_METER),
        ("₤", &*TURKISH_LIRA),
        ("°daysC", &*DEGREE_DAYS_CELSIUS),
        ("kip", &*KIP),
        ("danish_krone", &*DANISH_KRONE),
        ("energy_efficiency_ratio", &*ENERGY_EFFICIENCY_RATIO),
        (
            "watts_per_meter_degree_kelvin",
            &*WATTS_PER_METER_DEGREE_KELVIN
        ),
        ("candela", &*CANDELA),
        ("₩", &*SOUTH_KOREAN_WON),
        ("fluid_ounce", &*FLUID_OUNCE),
        ("ft", &*FOOT),
        ("BTU/lb", &*BTU_PER_POUND),
        ("LRD", &*LIBERIAN_DOLLAR),
        ("₪", &*NEW_ISRAELI_SHEKEL),
        ("R$", &*BRAZILIAN_REAL),
        ("₮", &*TUGRIK),
        ("bahamian_dollar", &*BAHAMIAN_DOLLAR),
        (
            "percent_obscuration_per_foot",
            &*PERCENT_OBSCURATION_PER_FOOT
        ),
        ("qatari_rial", &*QATARI_RIAL),
        ("CUP", &*CUBAN_PESO),
        ("som", &*SOM),
        (
            "kilovolt_ampere_reactive_hour",
            &*KILOVOLT_AMPERE_REACTIVE_HOUR
        ),
        ("°daysF", &*DEGREE_DAYS_FAHRENHEIT),
        ("iraqi_dinar", &*IRAQI_DINAR),
        ("minute", &*MINUTE),
        ("therm/h", &*THERMS_PER_HOUR),
        ("PGK", &*KINA),
        ("GJ/h", &*GIGAJOULES_PER_HOUR),
        ("฿", &*BAHT),
        ("VA", &*VOLT_AMPERE),
        ("°C", &*CELSIUS),
        ("kW", &*KILOWATT),
        ("cmHg", &*CENTIMETERS_OF_MERCURY),
        ("namibia_dollar", &*NAMIBIA_DOLLAR),
        ("SGD", &*SINGAPORE_DOLLAR),
        ("Ns", &*NEWTON_SECOND),
        ("mbar", &*MILLIBAR),
        ("decibel", &*DECIBEL),
        (
            "thousand_cubic_feet_natural_gas",
            &*THOUSAND_CUBIC_FEET_NATURAL_GAS
        ),
        ("guyana_dollar", &*GUYANA_DOLLAR),
        ("kilowatt", &*KILOWATT),
        ("kina", &*KINA),
        ("cpm", &*CYCLES_PER_MINUTE),
        ("m²", &*SQUARE_METER),
        (
            "kilowatts_per_gallons_per_minute",
            &*KILOWATTS_PER_GALLONS_PER_MINUTE
        ),
        ("millimeters_per_second", &*MILLIMETERS_PER_SECOND),
        ("GHS", &*CEDI),
        ("rad", &*RADIAN),
        ("philippine_peso", &*PHILIPPINE_PESO),
        ("W", &*WATT),
        ("db", &*DECIBEL),
        ("parts_per_billion", &*PARTS_PER_BILLION),
        ("pound_sterling", &*POUND_STERLING),
        ("ftlbs/sec", &*FOOT_POUNDS_PER_SECOND),
        ("grams_per_second", &*GRAMS_PER_SECOND),
        (
            "joules_per_kilogram_degree_kelvin",
            &*JOULES_PER_KILOGRAM_DEGREE_KELVIN
        ),
        ("kyat", &*KYAT),
        ("megajoules_per_hour", &*MEGAJOULES_PER_HOUR),
        ("kilowatts_per_square_foot", &*KILOWATTS_PER_SQUARE_FOOT),
        ("MRO", &*OUGUIYA),
        ("ns", &*NANOSECOND),
        ("hectoliters_per_second", &*HECTOLITERS_PER_SECOND),
        ("kPa", &*KILOPASCAL),
        ("K/h", &*DEGREES_KELVIN_PER_HOUR),
        ("kg/m³", &*KILOGRAMS_PER_CUBIC_METER),
        ("riel", &*RIEL),
        ("degree_days_celsius", &*DEGREE_DAYS_CELSIUS),
        ("microsecond", &*MICROSECOND),
        ("djibouti_franc", &*DJIBOUTI_FRANC),
        ("TRY", &*TURKISH_LIRA),
        ("yd³", &*CUBIC_YARD),
        ("feet_per_minute", &*FEET_PER_MINUTE),
        ("bermudian_dollar", &*BERMUDIAN_DOLLAR),
        ("leu", &*LEU),
        ("SFr", &*SWISS_FRANC),
        ("mauritius_rupee", &*MAURITIUS_RUPEE),
        ("mile", &*MILE),
        ("yd", &*YARD),
        ("GBP", &*POUND_STERLING),
        ("SAR", &*SAUDI_RIYAL),
        ("bulgarian_lev", &*BULGARIAN_LEV),
        ("swiss_franc", &*SWISS_FRANC),
        ("SCR", &*SEYCHELLES_RUPEE),
        ("konvertibilna_marka", &*KONVERTIBILNA_MARKA),
        ("Kz", &*KWANZA),
        ("g", &*GRAM),
        ("grams_per_kilogram", &*GRAMS_PER_KILOGRAM),
        ("GYD", &*GUYANA_DOLLAR),
        ("g/min", &*GRAMS_PER_MINUTE),
        ("ds", &*TENTHS_SECOND),
        ("fl_oz", &*FLUID_OUNCE),
        ("J/kg°K", &*JOULES_PER_KILOGRAM_DEGREE_KELVIN),
        ("parts_per_unit", &*PARTS_PER_UNIT),
        ("pound_force", &*POUND_FORCE),
        ("kg/min", &*KILOGRAMS_PER_MINUTE),
        ("µm", &*MICROMETER),
        ("formazin_nephelometric_unit", &*FORMAZIN_NEPHELOMETRIC_UNIT),
        ("mV", &*MILLIVOLT),
        ("K/s", &*DEGREES_KELVIN_PER_SECOND),
        ("hL", &*HECTOLITER),
        ("volts_per_meter", &*VOLTS_PER_METER),
        ("kelvin", &*KELVIN),
        ("millimeters_per_minute", &*MILLIMETERS_PER_MINUTE),
        (
            "megawatt_hours_per_square_meter",
            &*MEGAWATT_HOURS_PER_SQUARE_METER
        ),
        ("centimeter", &*CENTIMETER),
        ("loti", &*LOTI),
        ("millimeters_of_mercury", &*MILLIMETERS_OF_MERCURY),
        ("₱", &*PHILIPPINE_PESO),
        ("Rp", &*RUPIAH),
        ("KES", &*KENYAN_SHILLING),
        ("SHP", &*SAINT_HELENA_POUND),
        ("south_korean_won", &*SOUTH_KOREAN_WON),
        ("₴", &*HRYVNIA),
        ("Wh", &*WATT_HOUR),
        ("Δ°F", &*FAHRENHEIT_DEGREES),
        ("phot", &*PHOT),
        ("forint", &*FORINT),
        ("square_mile", &*SQUARE_MILE),
        ("ampere", &*AMPERE),
        (
            "kilowatt_hours_per_square_meter",
            &*KILOWATT_HOURS_PER_SQUARE_METER
        ),
        ("C", &*COULOMB),
        ("liter", &*LITER),
        ("galUK/min", &*IMPERIAL_GALLONS_PER_MINUTE),
        ("cubic_meter", &*CUBIC_METER),
        ("m³/s", &*CUBIC_METERS_PER_SECOND),
        ("LBP", &*LEBANESE_POUND),
        ("MV", &*MEGAVOLT),
        ("saint_helena_pound", &*SAINT_HELENA_POUND),
        ("MΩ", &*MEGOHM),
        ("mL/s", &*MILLILITERS_PER_SECOND),
        ("Sh", &*KENYAN_SHILLING),
        ("metical", &*METICAL),
        ("gigawatt", &*GIGAWATT),
        ("Af", &*AFGHANI),
        ("turkish_lira", &*TURKISH_LIRA),
        ("PAB", &*BALBOA),
        ("°F/min", &*DEGREES_FAHRENHEIT_PER_MINUTE),
        ("UYU", &*PESO_URUGUAYO),
        ("cm²", &*SQUARE_CENTIMETER),
        ("tons_refrigeration", &*TONS_REFRIGERATION),
        ("tenths_second", &*TENTHS_SECOND),
        ("yd²", &*SQUARE_YARD),
        (
            "kilowatt_hours_per_square_foot",
            &*KILOWATT_HOURS_PER_SQUARE_FOOT
        ),
        ("megavolt", &*MEGAVOLT),
        ("joules_per_kilogram_dry_air", &*JOULES_PER_KILOGRAM_DRY_AIR),
        ("vatu", &*VATU),
        ("jordanian_dinar", &*JORDANIAN_DINAR),
        ("hundredths_second", &*HUNDREDTHS_SECOND),
        ("egyptian_pound", &*EGYPTIAN_POUND),
        ("HNL", &*LEMPIRA),
        ("₸", &*TENGE),
        (
            "hundred_cubic_feet_natural_gas",
            &*HUNDRED_CUBIC_FEET_NATURAL_GAS
        ),
        ("gallons_per_hour", &*GALLONS_PER_HOUR),
        ("FKP", &*FALKLAND_ISLANDS_POUND),
        ("m", &*METER),
        ("suriname_dollar", &*SURINAME_DOLLAR),
        ("ZMW", &*ZAMBIAN_KWACHA),
        ("Ω", &*OHM),
        ("lbf", &*POUND_FORCE),
        ("hL/s", &*HECTOLITERS_PER_SECOND),
        ("₫", &*DONG),
        ("czech_koruna", &*CZECH_KORUNA),
        ("INR", &*INDIAN_RUPEE),
        ("NIO", &*CORDOBA_ORO),
        ("square_centimeter", &*SQUARE_CENTIMETER),
        ("kV", &*KILOVOLT),
        ("MW", &*MEGAWATT),
        ("inH₂O", &*INCHES_OF_WATER),
        ("NPR", &*NEPALESE_RUPEE),
        ("new_israeli_shekel", &*NEW_ISRAELI_SHEKEL),
        ("rand", &*RAND),
        ("yr", &*YEAR),
        ("MVARh", &*MEGAVOLT_AMPERE_REACTIVE_HOUR),
        (
            "nephelometric_turbidity_units",
            &*NEPHELOMETRIC_TURBIDITY_UNITS
        ),
        ("ntu", &*NEPHELOMETRIC_TURBIDITY_UNITS),
        ("centimeters_of_water", &*CENTIMETERS_OF_WATER),
        ("cycles_per_hour", &*CYCLES_PER_HOUR),
        ("MXN", &*MEXICAN_PESO),
        ("lempira", &*LEMPIRA),
        ("watts_per_square_foot", &*WATTS_PER_SQUARE_FOOT),
        (
            "percent_obscuration_per_meter",
            &*PERCENT_OBSCURATION_PER_METER
        ),
        ("ARS", &*ARGENTINE_PESO),
        ("MYR", &*MALAYSIAN_RINGGIT),
        ("ZWL", &*ZIMBABWE_DOLLAR),
        ("RM", &*MALAYSIAN_RINGGIT),
        ("cmH₂O", &*CENTIMETERS_OF_WATER),
        ("BTU", &*BTU),
        ("₡", &*COSTA_RICAN_COLON),
        ("kWh/m²", &*KILOWATT_HOURS_PER_SQUARE_METER),
        ("therms_per_hour", &*THERMS_PER_HOUR),
        ("kilometers_per_second", &*KILOMETERS_PER_SECOND),
        ("J/m²", &*JOULES_PER_SQUARE_METER),
        ("Volt", &*VOLT),
        ("ЅМ", &*SOMONI),
        ("kW/kcfm", &*KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE),
        ("uae_dirham", &*UAE_DIRHAM),
        ("/h", &*PER_HOUR),
        ("kg/s", &*KILOGRAMS_PER_SECOND),
        ("kBTU", &*KILOBTU),
        ("Wh/ft²", &*WATT_HOURS_PER_SQUARE_FOOT),
        ("JOD", &*JORDANIAN_DINAR),
        ("RWF", &*RWANDA_FRANC),
        ("nanosecond", &*NANOSECOND),
        ("MBTU/h", &*MEGABTUS_PER_HOUR),
        ("BDT", &*TAKA),
        ("in²", &*SQUARE_INCH),
        ("cd/m²", &*CANDELAS_PER_SQUARE_METER),
        ("ppb", &*PARTS_PER_BILLION),
        ("BGN", &*BULGARIAN_LEV),
        ("LAK", &*KIP),
        ("ƒ", &*ARUBAN_GUILDER_FLORIN),
        ("KWD", &*KUWAITI_DINAR),
        ("MB", &*MEGABYTE),
        ("USD", &*US_DOLLAR),
        ("mm/min", &*MILLIMETERS_PER_MINUTE),
        ("siemens", &*SIEMENS),
        ("PLN", &*PZLOTY),
        ("henry", &*HENRY),
        ("kW/ft²", &*KILOWATTS_PER_SQUARE_FOOT),
        ("TJS", &*SOMONI),
        (
            "kilowatts_per_kilocubic_feet_per_minute",
            &*KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE
        ),
        ("liters_per_second", &*LITERS_PER_SECOND),
        ("pH", &*PH),
        ("somoni", &*SOMONI),
        ("S/m", &*SIEMENS_PER_METER),
        ("newton_meter", &*NEWTON_METER),
        ("CAD", &*CANADIAN_DOLLAR),
        ("wk", &*WEEK),
        ("kgal", &*KILOGALLON),
        ("degrees_celsius_per_hour", &*DEGREES_CELSIUS_PER_HOUR),
        ("m³/min", &*CUBIC_METERS_PER_MINUTE),
        ("m³", &*CUBIC_METER),
        ("micrometer", &*MICROMETER),
        ("degrees_kelvin_per_minute", &*DEGREES_KELVIN_PER_MINUTE),
        ("gibraltar_pound", &*GIBRALTAR_POUND),
        ("solomon_islands_dollar", &*SOLOMON_ISLANDS_DOLLAR),
        ("degrees_phase", &*DEGREES_PHASE),
        ("calorie", &*CALORIE),
        ("J/g", &*JOULES_PER_GRAM),
        ("milliwatt", &*MILLIWATT),
        ("BTU/h", &*BTUS_PER_HOUR),
        ("kJ/kg", &*KILOJOULES_PER_KILOGRAM),
        ("MWh", &*MEGAWATT_HOUR),
        ("tonref", &*TONS_REFRIGERATION),
        ("IQD", &*IRAQI_DINAR),
        ("pounds_per_second", &*POUNDS_PER_SECOND),
        ("VEF", &*BOLIVAR_FUERTE),
        ("MJ/m²", &*MEGAJOULES_PER_SQUARE_METER),
        ("inHg", &*INCHES_OF_MERCURY),
        ("serbian_dinar", &*SERBIAN_DINAR),
        ("/min", &*PER_MINUTE),
        ("kWh/ft²", &*KILOWATT_HOURS_PER_SQUARE_FOOT),
        ("V", &*VOLT),
        ("VAR", &*VOLT_AMPERE_REACTIVE),
        ("libyan_dinar", &*LIBYAN_DINAR),
        ("kW/m²", &*KILOWATTS_PER_SQUARE_METER),
        ("malagasy_ariary", &*MALAGASY_ARIARY),
        ("W/m²", &*WATTS_PER_SQUARE_METER),
        ("calorie_per_gram", &*CALORIE_PER_GRAM),
        ("W/m³/s", &*WATTS_PER_CUBIC_METERS_PER_SECOND),
        ("NZD", &*NEW_ZEALAND_DOLLAR),
        ("W/ft²", &*WATTS_PER_SQUARE_FOOT),
        ("ounce", &*OUNCE),
        ("MHz", &*MEGAHERTZ),
        ("KPW", &*NORTH_KOREAN_WON),
        ("VAh", &*VOLT_AMPERE_HOUR),
        ("NOK", &*NORWEGIAN_KRONE),
        ("petabyte", &*PETABYTE),
        ("m/s", &*METERS_PER_SECOND),
        ("pataca", &*PATACA),
        ("MBTU", &*MEGABTU),
        ("coefficient_of_performance", &*COEFFICIENT_OF_PERFORMANCE),
        ("pound", &*POUND),
        ("hph", &*HORSEPOWER_HOUR),
        ("gigabyte", &*GIGABYTE),
        ("percent_per_second", &*PERCENT_PER_SECOND),
        ("sec", &*SECOND),
        ("fahrenheit", &*FAHRENHEIT),
        ("kwacha", &*KWACHA),
        ("zambian_kwacha", &*ZAMBIAN_KWACHA),
        ("Br", &*BELARUSSIAN_RUBLE),
        ("MWh/m²", &*MEGAWATT_HOURS_PER_SQUARE_METER),
        ("K", &*KELVIN),
        ("guinea_franc", &*GUINEA_FRANC),
        ("kilobtu_per_square_foot", &*KILOBTU_PER_SQUARE_FOOT),
        ("pt", &*PINT),
        ("percent", &*PERCENT),
        ("kB", &*KILOBYTE),
        ("₹", &*INDIAN_RUPEE),
        ("VARh", &*VOLT_AMPERE_REACTIVE_HOUR),
        ("db_millivolt", &*DB_MILLIVOLT),
        ("MGA", &*MALAGASY_ARIARY),
        ("px", &*PIXEL),
        ("SRD", &*SURINAME_DOLLAR),
        ("MAD", &*MOROCCAN_DIRHAM),
        ("kL", &*KILOLITER),
        ("J/h", &*JOULES_PER_HOUR),
        ("hryvnia", &*HRYVNIA),
        ("milligrams_per_cubic_meter", &*MILLIGRAMS_PER_CUBIC_METER),
        ("kHz", &*KILOHERTZ),
        ("centimeters_of_mercury", &*CENTIMETERS_OF_MERCURY),
        ("week", &*WEEK),
        ("pakistan_rupee", &*PAKISTAN_RUPEE),
        ("KHR", &*RIEL),
        ("VND", &*DONG),
        ("russian_ruble", &*RUSSIAN_RUBLE),
        ("celsius", &*CELSIUS),
        ("kilovolt_ampere", &*KILOVOLT_AMPERE),
        ("volt_ampere_reactive", &*VOLT_AMPERE_REACTIVE),
        ("square_yard", &*SQUARE_YARD),
        ("malaysian_ringgit", &*MALAYSIAN_RINGGIT),
        ("MK", &*KWACHA),
        ("zł", &*PZLOTY),
        ("per_second", &*PER_SECOND),
        ("ton/h", &*METRIC_TONS_PER_HOUR),
        ("megabtu_per_square_foot", &*MEGABTU_PER_SQUARE_FOOT),
        ("gallons_per_minute", &*GALLONS_PER_MINUTE),
        ("burundi_franc", &*BURUNDI_FRANC),
        ("dominican_peso", &*DOMINICAN_PESO),
        ("g/m³", &*GRAMS_PER_CUBIC_METER),
        ("m²/N", &*SQUARE_METERS_PER_NEWTON),
        ("A", &*AMPERE),
        ("µg/m³", &*MICROGRAMS_PER_CUBIC_METER),
        ("cubic_meters_per_hour", &*CUBIC_METERS_PER_HOUR),
        ("J", &*JOULE),
        ("megavolt_ampere", &*MEGAVOLT_AMPERE),
        ("imperial_gallons_per_minute", &*IMPERIAL_GALLONS_PER_MINUTE),
        ("percent_relative_humidity", &*PERCENT_RELATIVE_HUMIDITY),
        ("km²", &*SQUARE_KILOMETER),
        ("CVE", &*CAPE_VERDE_ESCUDO),
        ("GNF", &*GUINEA_FRANC),
        ("kilowatt_hour", &*KILOWATT_HOUR),
        (
            "million_cubic_feet_natural_gas",
            &*MILLION_CUBIC_FEET_NATURAL_GAS
        ),
        ("millimeter", &*MILLIMETER),
        ("klb/h", &*KILOPOUNDS_PER_HOUR),
        ("horsepower_hour", &*HORSEPOWER_HOUR),
        ("kg/h", &*KILOGRAMS_PER_HOUR),
        ("S", &*SIEMENS),
        ("kilometer", &*KILOMETER),
        ("meters_per_hour", &*METERS_PER_HOUR),
        ("pint", &*PINT),
        ("MΩ", &*MEGOHM),
        ("square_meter", &*SQUARE_METER),
        ("HTG", &*GOURDE),
        ("cubic_meters_per_second", &*CUBIC_METERS_PER_SECOND),
        ("croatian_kuna", &*CROATIAN_KUNA),
        ("degrees_kelvin_per_hour", &*DEGREES_KELVIN_PER_HOUR),
        ("revolutions_per_minute", &*REVOLUTIONS_PER_MINUTE),
        ("hertz", &*HERTZ),
        ("kilovolt_ampere_reactive", &*KILOVOLT_AMPERE_REACTIVE),
        ("lb", &*POUND),
        ("mmHg", &*MILLIMETERS_OF_MERCURY),
        ("DJF", &*DJIBOUTI_FRANC),
        ("TND", &*TUNISIAN_DINAR),
        ("trinidad_and_tobago_dollar", &*TRINIDAD_AND_TOBAGO_DOLLAR),
        ("guarani", &*GUARANI),
        ("cfh", &*CUBIC_FEET_PER_HOUR),
        ("MJ/°K", &*MEGAJOULES_PER_DEGREE_KELVIN),
        ("V/K", &*VOLTS_PER_DEGREE_KELVIN),
        ("rpm", &*REVOLUTIONS_PER_MINUTE),
        ("db_microvolt", &*DB_MICROVOLT),
        ("belarussian_ruble", &*BELARUSSIAN_RUBLE),
        ("MZN", &*METICAL),
        ("kilopascal", &*KILOPASCAL),
        ("gal/hr", &*GALLONS_PER_HOUR),
        ("XAF", &*CFA_FRANC_BCEAO),
        ("ms", &*MILLISECOND),
        ("LKR", &*SRI_LANKA_RUPEE),
        ("fiji_dollar", &*FIJI_DOLLAR),
        ("newton_second", &*NEWTON_SECOND),
        ("PHP", &*PHILIPPINE_PESO),
        ("btu/lb_dry", &*BTUS_PER_POUND_DRY_AIR),
        ("armenian_dram", &*ARMENIAN_DRAM),
        (
            "grams_of_water_per_kilogram_dry_air",
            &*GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR
        ),
        ("hectopascal", &*HECTOPASCAL),
        ("cayman_islands_dollar", &*CAYMAN_ISLANDS_DOLLAR),
        ("argentine_peso", &*ARGENTINE_PESO),
        ("kilogallon", &*KILOGALLON),
        (
            "kilojoules_per_kilogram_dry_air",
            &*KILOJOULES_PER_KILOGRAM_DRY_AIR
        ),
        ("brunei_dollar", &*BRUNEI_DOLLAR),
        ("mm²", &*SQUARE_MILLIMETER),
        ("footcandle", &*FOOTCANDLE),
        ("lilangeni", &*LILANGENI),
        ("iranian_rial", &*IRANIAN_RIAL),
        ("meters_per_second", &*METERS_PER_SECOND),
        ("gigajoule", &*GIGAJOULE),
        ("rad/s²", &*RADIANS_PER_SECOND_SQUARED),
        ("MKD", &*DENAR),
        ("m³_gas", &*CUBIC_METERS_NATURAL_GAS),
        ("imperial_gallon", &*IMPERIAL_GALLON),
        ("ouguiya", &*OUGUIYA),
        ("oz", &*OUNCE),
        ("BRL", &*BRAZILIAN_REAL),
        ("ton", &*METRIC_TON),
        ("bolivar_fuerte", &*BOLIVAR_FUERTE),
        ("sr", &*STERADIAN),
        ("Kč", &*CZECH_KORUNA),
        ("hong_kong_dollar", &*HONG_KONG_DOLLAR),
        ("megabtus_per_hour", &*MEGABTUS_PER_HOUR),
        ("kW/gal/min", &*KILOWATTS_PER_GALLONS_PER_MINUTE),
        ("ft³", &*CUBIC_FOOT),
        ("gallon", &*GALLON),
        ("GJ", &*GIGAJOULE),
        ("Ah", &*AMPERE_HOUR),
        ("gph", &*GALLONS_PER_HOUR),
        ("Hz", &*HERTZ),
        ("kBTU/h/ft²", &*KILOBTUS_PER_HOUR_PER_SQUARE_FOOT),
        ("hft³", &*HECTO_CUBIC_FOOT),
        ("balboa", &*BALBOA),
        ("PEN", &*NUEVO_SOL),
        ("in³", &*CUBIC_INCH),
        ("TB", &*TERABYTE),
        ("ohm", &*OHM),
        ("parts_per_million", &*PARTS_PER_MILLION),
        ("singapore_dollar", &*SINGAPORE_DOLLAR),
        ("hr", &*HOUR),
        ("lumen", &*LUMEN),
        ("Vt", &*VATU),
        ("BHD", &*BAHRAINI_DINAR),
        ("PUE", &*POWER_USAGE_EFFECTIVENESS),
        ("kilovolt", &*KILOVOLT),
        ("LYD", &*LIBYAN_DINAR),
        ("tugrik", &*TUGRIK),
        ("A/m²", &*AMPERES_PER_SQUARE_METER),
        ("kr", &*DANISH_KRONE),
        ("°C/h", &*DEGREES_CELSIUS_PER_HOUR),
        ("lari", &*LARI),
        ("foot", &*FOOT),
        ("rad/s", &*RADIANS_PER_SECOND),
        ("КМ", &*KONVERTIBILNA_MARKA),
        ("millibar", &*MILLIBAR),
        ("square_kilometer", &*SQUARE_KILOMETER),
        ("millisecond", &*MILLISECOND),
        ("Le", &*LEONE),
        ("₵", &*CEDI),
        ("gH₂O/kgAir", &*GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR),
        ("MJ/h", &*MEGAJOULES_PER_HOUR),
        ("gal/min", &*GALLONS_PER_MINUTE),
        ("megabyte", &*MEGABYTE),
        ("grams_per_cubic_meter", &*GRAMS_PER_CUBIC_METER),
        ("ZAR", &*RAND),
        ("MMK", &*KYAT),
        ("atmosphere", &*ATMOSPHERE),
        ("ლ", &*LARI),
        ("mile²", &*SQUARE_MILE),
        ("lebanese_pound", &*LEBANESE_POUND),
        ("swedish_krona", &*SWEDISH_KRONA),
        ("degrees_angular", &*DEGREES_ANGULAR),
        ("cubic_feet_per_hour", &*CUBIC_FEET_PER_HOUR),
        ("CHF", &*SWISS_FRANC),
        ("tunisian_dinar", &*TUNISIAN_DINAR),
        ("cubic_foot", &*CUBIC_FOOT),
        ("inch", &*INCH),
        ("MWh/ft²", &*MEGAWATT_HOURS_PER_SQUARE_FOOT),
        ("baht", &*BAHT),
        ("milliliters_per_second", &*MILLILITERS_PER_SECOND),
        ("kBTU/h", &*KILOBTUS_PER_HOUR),
        ("KYD", &*CAYMAN_ISLANDS_DOLLAR),
        ("ден", &*DENAR),
        (
            "watts_per_cubic_meters_per_second",
            &*WATTS_PER_CUBIC_METERS_PER_SECOND
        ),
        ("t", &*SHORT_TON),
        ("mVA", &*MEGAVOLT_AMPERE),
        ("kuwaiti_dinar", &*KUWAITI_DINAR),
        ("yemeni_rial", &*YEMENI_RIAL),
        ("mΩ", &*MILLIOHM),
        ("DKK", &*DANISH_KRONE),
        ("farad", &*FARAD),
        ("newtons_per_meter", &*NEWTONS_PER_METER),
        ("dalasi", &*DALASI),
        ("grams_per_square_meter", &*GRAMS_PER_SQUARE_METER),
        ("pula", &*PULA),
        ("cal", &*CALORIE),
        ("kelvin_degrees", &*KELVIN_DEGREES),
        ("pzloty", &*PZLOTY),
        ("s", &*SECOND),
        ("MMBTU/h", &*MEGABTUS_PER_HOUR),
    ]
    .iter()
    .cloned()
    .collect();
}
