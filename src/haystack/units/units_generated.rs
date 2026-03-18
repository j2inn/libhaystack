// Copyright (C) 2020 - 2022, J2 Innovations
// Haystack Unit module - auto generated.

#![allow(clippy::approx_constant)]
use super::{Unit, UnitDimensions};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;

// dimensionless
pub static PERCENT: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("percent"), Cow::Borrowed("%")]),
    dimensions: None,
    scale: 0.01,
    offset: 0.0,
};

pub static PERCENT_RELATIVE_HUMIDITY: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("percent_relative_humidity"),
        Cow::Borrowed("%RH"),
    ]),
    dimensions: None,
    scale: 0.01,
    offset: 0.0,
};

pub static PIXEL: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pixel"), Cow::Borrowed("px")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DECIBEL: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("decibel"), Cow::Borrowed("db")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static POWER_FACTOR: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("power_factor"), Cow::Borrowed("pf")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PH: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("ph"), Cow::Borrowed("pH")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("grams_of_water_per_kilogram_dry_air"),
        Cow::Borrowed("gH₂O/kgAir"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static VOLTS_PER_DEGREE_KELVIN: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("volts_per_degree_kelvin"),
        Cow::Borrowed("V/K"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DEGREE_DAYS_CELSIUS: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("degree_days_celsius"),
        Cow::Borrowed("°daysC"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DEGREE_DAYS_FAHRENHEIT: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("degree_days_fahrenheit"),
        Cow::Borrowed("°daysF"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PERCENT_OBSCURATION_PER_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("percent_obscuration_per_foot"),
        Cow::Borrowed("%obsc/ft"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PERCENT_OBSCURATION_PER_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("percent_obscuration_per_meter"),
        Cow::Borrowed("%obsc/m"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PSI_PER_DEGREE_FAHRENHEIT: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("psi_per_degree_fahrenheit"),
        Cow::Borrowed("psi/°F"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SQUARE_METERS_PER_NEWTON: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("square_meters_per_newton"),
        Cow::Borrowed("m²/N"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static WATTS_PER_SQUARE_METER_DEGREE_KELVIN: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watts_per_square_meter_degree_kelvin"),
        Cow::Borrowed("W/m²K"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DB_MILLIVOLT: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("db_millivolt"), Cow::Borrowed("dBmV")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DB_MICROVOLT: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("db_microvolt"), Cow::Borrowed("dBµV")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PARTS_PER_UNIT: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("parts_per_unit"), Cow::Borrowed("ppu")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PARTS_PER_MILLION: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("parts_per_million"), Cow::Borrowed("ppm")]),
    dimensions: None,
    scale: 1e-6,
    offset: 0.0,
};

pub static PARTS_PER_BILLION: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("parts_per_billion"), Cow::Borrowed("ppb")]),
    dimensions: None,
    scale: 1e-9,
    offset: 0.0,
};

pub static GRAMS_PER_KILOGRAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("grams_per_kilogram"), Cow::Borrowed("g/kg")]),
    dimensions: None,
    scale: 0.001,
    offset: 0.0,
};

pub static RADIAN: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("radian"), Cow::Borrowed("rad")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DEGREES_ANGULAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("degrees_angular"), Cow::Borrowed("deg")]),
    dimensions: None,
    scale: 0.017453292519943,
    offset: 0.0,
};

pub static DEGREES_PHASE: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("degrees_phase"), Cow::Borrowed("degPh")]),
    dimensions: None,
    scale: 0.017453292519943,
    offset: 0.0,
};

pub static STERADIAN: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[Cow::Borrowed("steradian"), Cow::Borrowed("sr")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NEPHELOMETRIC_TURBIDITY_UNITS: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("nephelometric_turbidity_units"),
        Cow::Borrowed("ntu"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static FORMAZIN_NEPHELOMETRIC_UNIT: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("formazin_nephelometric_unit"),
        Cow::Borrowed("fnu"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static POWER_USAGE_EFFECTIVENESS: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("power_usage_effectiveness"),
        Cow::Borrowed("PUE"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DATA_CENTER_INFRASTRUCTURE_EFFICIENCY: Unit = Unit {
    quantity: Some(Cow::Borrowed("dimensionless")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("data_center_infrastructure_efficiency"),
        Cow::Borrowed("DCIE"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

// currency
pub static AFGHANI: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("afghani"),
        Cow::Borrowed("AFN"),
        Cow::Borrowed("Af"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static ALGERIAN_DINAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("algerian_dinar"), Cow::Borrowed("DZD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static ARGENTINE_PESO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("argentine_peso"), Cow::Borrowed("ARS")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static ARMENIAN_DRAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("armenian_dram"),
        Cow::Borrowed("AMD"),
        Cow::Borrowed("Դ"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static ARUBAN_GUILDER_FLORIN: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("aruban_guilder_florin"),
        Cow::Borrowed("AWG"),
        Cow::Borrowed("ƒ"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static AUSTRALIAN_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("australian_dollar"), Cow::Borrowed("AUD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static AZERBAIJANIAN_MANAT: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("azerbaijanian_manat"),
        Cow::Borrowed("AZN"),
        Cow::Borrowed("ман"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BAHAMIAN_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("bahamian_dollar"), Cow::Borrowed("BSD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BAHRAINI_DINAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("bahraini_dinar"), Cow::Borrowed("BHD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BAHT: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("baht"),
        Cow::Borrowed("THB"),
        Cow::Borrowed("฿"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BALBOA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("balboa"), Cow::Borrowed("PAB")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BARBADOS_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("barbados_dollar"), Cow::Borrowed("BBD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BELARUSSIAN_RUBLE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("belarussian_ruble"),
        Cow::Borrowed("BYR"),
        Cow::Borrowed("Br"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BELIZE_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("belize_dollar"), Cow::Borrowed("BZD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BERMUDIAN_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("bermudian_dollar"), Cow::Borrowed("BMD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BOLIVAR_FUERTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("bolivar_fuerte"), Cow::Borrowed("VEF")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BOLIVIANO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("boliviano"), Cow::Borrowed("BOB")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BRAZILIAN_REAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("brazilian_real"),
        Cow::Borrowed("BRL"),
        Cow::Borrowed("R$"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BRUNEI_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("brunei_dollar"), Cow::Borrowed("BND")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BULGARIAN_LEV: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("bulgarian_lev"),
        Cow::Borrowed("BGN"),
        Cow::Borrowed("лв"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static BURUNDI_FRANC: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("burundi_franc"), Cow::Borrowed("BIF")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CANADIAN_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("canadian_dollar"), Cow::Borrowed("CAD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CAPE_VERDE_ESCUDO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cape_verde_escudo"), Cow::Borrowed("CVE")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CAYMAN_ISLANDS_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cayman_islands_dollar"), Cow::Borrowed("KYD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CEDI: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("cedi"),
        Cow::Borrowed("GHS"),
        Cow::Borrowed("₵"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CFA_FRANC_BCEAO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cfa_franc_bceao"), Cow::Borrowed("XAF")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CFP_FRANC: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cfp_franc"), Cow::Borrowed("XPF")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CHILEAN_PESO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("chilean_peso"), Cow::Borrowed("CLP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CHINESE_YUAN: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("chinese_yuan"),
        Cow::Borrowed("CNY"),
        Cow::Borrowed("元"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CONGOLESE_FRANC: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("congolese_franc"), Cow::Borrowed("CDF")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CORDOBA_ORO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("cordoba_oro"),
        Cow::Borrowed("NIO"),
        Cow::Borrowed("C$"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static COSTA_RICAN_COLON: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("costa_rican_colon"),
        Cow::Borrowed("CRC"),
        Cow::Borrowed("₡"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CROATIAN_KUNA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("croatian_kuna"),
        Cow::Borrowed("HRK"),
        Cow::Borrowed("Kn"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CUBAN_PESO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cuban_peso"), Cow::Borrowed("CUP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static CZECH_KORUNA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("czech_koruna"),
        Cow::Borrowed("CZK"),
        Cow::Borrowed("Kč"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DALASI: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("dalasi"), Cow::Borrowed("GMD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DANISH_KRONE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("danish_krone"),
        Cow::Borrowed("DKK"),
        Cow::Borrowed("kr"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DENAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("denar"),
        Cow::Borrowed("MKD"),
        Cow::Borrowed("ден"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DJIBOUTI_FRANC: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("djibouti_franc"), Cow::Borrowed("DJF")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DOBRA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("dobra"),
        Cow::Borrowed("STD"),
        Cow::Borrowed("Db"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DOMINICAN_PESO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("dominican_peso"), Cow::Borrowed("DOP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static DONG: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("dong"),
        Cow::Borrowed("VND"),
        Cow::Borrowed("₫"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static EAST_CARIBBEAN_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("east_caribbean_dollar"), Cow::Borrowed("XCD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static EGYPTIAN_POUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("egyptian_pound"), Cow::Borrowed("EGP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static ETHIOPIAN_BIRR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("ethiopian_birr"), Cow::Borrowed("ETB")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static EURO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("euro"),
        Cow::Borrowed("EUR"),
        Cow::Borrowed("€"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static FALKLAND_ISLANDS_POUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("falkland_islands_pound"),
        Cow::Borrowed("FKP"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static FIJI_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("fiji_dollar"), Cow::Borrowed("FJD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static FORINT: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("forint"), Cow::Borrowed("HUF")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static GIBRALTAR_POUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("gibraltar_pound"), Cow::Borrowed("GIP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static GOURDE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("gourde"), Cow::Borrowed("HTG")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static GUARANI: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("guarani"),
        Cow::Borrowed("PYG"),
        Cow::Borrowed("₲"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static GUINEA_FRANC: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("guinea_franc"), Cow::Borrowed("GNF")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static GUYANA_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("guyana_dollar"), Cow::Borrowed("GYD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static HONG_KONG_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("hong_kong_dollar"), Cow::Borrowed("HKD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static HRYVNIA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("hryvnia"),
        Cow::Borrowed("UAH"),
        Cow::Borrowed("₴"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static ICELAND_KRONA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("iceland_krona"),
        Cow::Borrowed("ISK"),
        Cow::Borrowed("Kr"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static INDIAN_RUPEE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("indian_rupee"),
        Cow::Borrowed("INR"),
        Cow::Borrowed("₹"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static IRANIAN_RIAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("iranian_rial"), Cow::Borrowed("IRR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static IRAQI_DINAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("iraqi_dinar"), Cow::Borrowed("IQD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static JAMAICAN_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("jamaican_dollar"), Cow::Borrowed("JMD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static JORDANIAN_DINAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("jordanian_dinar"), Cow::Borrowed("JOD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static KENYAN_SHILLING: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kenyan_shilling"),
        Cow::Borrowed("KES"),
        Cow::Borrowed("Sh"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static KINA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kina"), Cow::Borrowed("PGK")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static KIP: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kip"),
        Cow::Borrowed("LAK"),
        Cow::Borrowed("₭"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static KONVERTIBILNA_MARKA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("konvertibilna_marka"),
        Cow::Borrowed("BAM"),
        Cow::Borrowed("КМ"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static KUWAITI_DINAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kuwaiti_dinar"), Cow::Borrowed("KWD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static KWACHA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kwacha"),
        Cow::Borrowed("MWK"),
        Cow::Borrowed("MK"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static KWANZA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kwanza"),
        Cow::Borrowed("AOA"),
        Cow::Borrowed("Kz"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static KYAT: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kyat"), Cow::Borrowed("MMK")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LARI: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("lari"),
        Cow::Borrowed("GEL"),
        Cow::Borrowed("ლ"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LEBANESE_POUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("lebanese_pound"), Cow::Borrowed("LBP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LEK: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("lek"), Cow::Borrowed("ALL")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LEMPIRA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("lempira"), Cow::Borrowed("HNL")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LEONE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("leone"),
        Cow::Borrowed("SLL"),
        Cow::Borrowed("Le"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LEU: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("leu"), Cow::Borrowed("RON")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LIBERIAN_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("liberian_dollar"), Cow::Borrowed("LRD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LIBYAN_DINAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("libyan_dinar"), Cow::Borrowed("LYD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LILANGENI: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("lilangeni"), Cow::Borrowed("SZL")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static LOTI: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("loti"), Cow::Borrowed("LSL")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static MALAGASY_ARIARY: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("malagasy_ariary"), Cow::Borrowed("MGA")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static MALAYSIAN_RINGGIT: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("malaysian_ringgit"),
        Cow::Borrowed("MYR"),
        Cow::Borrowed("RM"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static MANAT: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("manat"), Cow::Borrowed("TMT")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static MAURITIUS_RUPEE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("mauritius_rupee"), Cow::Borrowed("MUR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static METICAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("metical"),
        Cow::Borrowed("MZN"),
        Cow::Borrowed("MTn"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static MEXICAN_PESO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("mexican_peso"), Cow::Borrowed("MXN")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static MOLDAVIAN_LEU: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("moldavian_leu"), Cow::Borrowed("MDL")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static MOROCCAN_DIRHAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("moroccan_dirham"), Cow::Borrowed("MAD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NAIRA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("naira"),
        Cow::Borrowed("NGN"),
        Cow::Borrowed("₦"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NAKFA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("nakfa"),
        Cow::Borrowed("ERN"),
        Cow::Borrowed("Nfk"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NAMIBIA_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("namibia_dollar"), Cow::Borrowed("NAD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NEPALESE_RUPEE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("nepalese_rupee"), Cow::Borrowed("NPR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NEW_ISRAELI_SHEKEL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("new_israeli_shekel"),
        Cow::Borrowed("ILS"),
        Cow::Borrowed("₪"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NEW_ZEALAND_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("new_zealand_dollar"), Cow::Borrowed("NZD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NGULTRUM: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("ngultrum"), Cow::Borrowed("BTN")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NORTH_KOREAN_WON: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("north_korean_won"), Cow::Borrowed("KPW")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NORWEGIAN_KRONE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("norwegian_krone"), Cow::Borrowed("NOK")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static NUEVO_SOL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("nuevo_sol"), Cow::Borrowed("PEN")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static OUGUIYA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("ouguiya"),
        Cow::Borrowed("MRO"),
        Cow::Borrowed("UM"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PAKISTAN_RUPEE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("pakistan_rupee"),
        Cow::Borrowed("PKR"),
        Cow::Borrowed("₨"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PATACA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pataca"), Cow::Borrowed("MOP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PESO_URUGUAYO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("peso_uruguayo"), Cow::Borrowed("UYU")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PHILIPPINE_PESO: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("philippine_peso"),
        Cow::Borrowed("PHP"),
        Cow::Borrowed("₱"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static POUND_STERLING: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("pound_sterling"),
        Cow::Borrowed("GBP"),
        Cow::Borrowed("£"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PULA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pula"), Cow::Borrowed("BWP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static PZLOTY: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("pzloty"),
        Cow::Borrowed("PLN"),
        Cow::Borrowed("zł"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static QATARI_RIAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("qatari_rial"), Cow::Borrowed("QAR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static QUETZAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("quetzal"), Cow::Borrowed("GTQ")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static RAND: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("rand"), Cow::Borrowed("ZAR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static RIAL_OMANI: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("rial_omani"), Cow::Borrowed("OMR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static RIEL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("riel"), Cow::Borrowed("KHR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static RUFIYAA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("rufiyaa"), Cow::Borrowed("MVR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static RUPIAH: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("rupiah"),
        Cow::Borrowed("IDR"),
        Cow::Borrowed("Rp"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static RUSSIAN_RUBLE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("russian_ruble"), Cow::Borrowed("RUB")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static RWANDA_FRANC: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("rwanda_franc"), Cow::Borrowed("RWF")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SAINT_HELENA_POUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("saint_helena_pound"), Cow::Borrowed("SHP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SAUDI_RIYAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("saudi_riyal"), Cow::Borrowed("SAR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SERBIAN_DINAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("serbian_dinar"),
        Cow::Borrowed("RSD"),
        Cow::Borrowed("din"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SEYCHELLES_RUPEE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("seychelles_rupee"), Cow::Borrowed("SCR")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SINGAPORE_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("singapore_dollar"), Cow::Borrowed("SGD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SOLOMON_ISLANDS_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("solomon_islands_dollar"),
        Cow::Borrowed("SBD"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SOM: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("som"), Cow::Borrowed("KGS")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SOMALI_SHILLING: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("somali_shilling"), Cow::Borrowed("SOS")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SOMONI: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("somoni"),
        Cow::Borrowed("TJS"),
        Cow::Borrowed("ЅМ"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SOUTH_KOREAN_WON: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("south_korean_won"),
        Cow::Borrowed("KRW"),
        Cow::Borrowed("₩"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SRI_LANKA_RUPEE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("sri_lanka_rupee"),
        Cow::Borrowed("LKR"),
        Cow::Borrowed("Rs"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SUDANESE_POUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("sudanese_pound"), Cow::Borrowed("SDG")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SURINAME_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("suriname_dollar"), Cow::Borrowed("SRD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SWEDISH_KRONA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("swedish_krona"), Cow::Borrowed("SEK")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SWISS_FRANC: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("swiss_franc"),
        Cow::Borrowed("CHF"),
        Cow::Borrowed("SFr"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static SYRIAN_POUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("syrian_pound"), Cow::Borrowed("SYP")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static TAIWAN_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("taiwan_dollar"), Cow::Borrowed("TWD")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static TAKA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("taka"),
        Cow::Borrowed("BDT"),
        Cow::Borrowed("৳"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static TALA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("tala"), Cow::Borrowed("WST")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static TANZANIAN_SHILLING: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("tanzanian_shilling"), Cow::Borrowed("TZS")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static TENGE: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("tenge"),
        Cow::Borrowed("KZT"),
        Cow::Borrowed("₸"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static TRINIDAD_AND_TOBAGO_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("trinidad_and_tobago_dollar"),
        Cow::Borrowed("TTD"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static TUGRIK: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("tugrik"),
        Cow::Borrowed("MNT"),
        Cow::Borrowed("₮"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static TUNISIAN_DINAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("tunisian_dinar"), Cow::Borrowed("TND")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static TURKISH_LIRA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("turkish_lira"),
        Cow::Borrowed("TRY"),
        Cow::Borrowed("₤"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static UAE_DIRHAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("uae_dirham"), Cow::Borrowed("AED")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static UGANDA_SHILLING: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("uganda_shilling"), Cow::Borrowed("UGX")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static US_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("us_dollar"),
        Cow::Borrowed("USD"),
        Cow::Borrowed("$"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static UZBEKISTAN_SUM: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("uzbekistan_sum"), Cow::Borrowed("UZS")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static VATU: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("vatu"),
        Cow::Borrowed("VUV"),
        Cow::Borrowed("Vt"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static YEMENI_RIAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("yemeni_rial"), Cow::Borrowed("YER")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static YEN: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("yen"),
        Cow::Borrowed("JPY"),
        Cow::Borrowed("¥"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static ZAMBIAN_KWACHA: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("zambian_kwacha"),
        Cow::Borrowed("ZMW"),
        Cow::Borrowed("ZK"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static ZIMBABWE_DOLLAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("currency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("zimbabwe_dollar"), Cow::Borrowed("ZWL")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

// acceleration
pub static METERS_PER_SECOND_SQUARED: Unit = Unit {
    quantity: Some(Cow::Borrowed("acceleration")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("meters_per_second_squared"),
        Cow::Borrowed("m/s²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// angular acceleration
pub static RADIANS_PER_SECOND_SQUARED: Unit = Unit {
    quantity: Some(Cow::Borrowed("angular acceleration")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("radians_per_second_squared"),
        Cow::Borrowed("rad/s²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// angular momentum
pub static JOULE_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("angular momentum")),
    ids: Cow::Borrowed(&[Cow::Borrowed("joule_second"), Cow::Borrowed("Js")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// angular velocity
pub static RADIANS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("angular velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("radians_per_second"), Cow::Borrowed("rad/s")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static REVOLUTIONS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("angular velocity")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("revolutions_per_minute"),
        Cow::Borrowed("rpm"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 6.2831853071796,
    offset: 0.0,
};

// area
pub static SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("area")),
    ids: Cow::Borrowed(&[Cow::Borrowed("square_meter"), Cow::Borrowed("m²")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static SQUARE_MILLIMETER: Unit = Unit {
    quantity: Some(Cow::Borrowed("area")),
    ids: Cow::Borrowed(&[Cow::Borrowed("square_millimeter"), Cow::Borrowed("mm²")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-6,
    offset: 0.0,
};

pub static SQUARE_CENTIMETER: Unit = Unit {
    quantity: Some(Cow::Borrowed("area")),
    ids: Cow::Borrowed(&[Cow::Borrowed("square_centimeter"), Cow::Borrowed("cm²")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0001,
    offset: 0.0,
};

pub static SQUARE_KILOMETER: Unit = Unit {
    quantity: Some(Cow::Borrowed("area")),
    ids: Cow::Borrowed(&[Cow::Borrowed("square_kilometer"), Cow::Borrowed("km²")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

pub static SQUARE_INCH: Unit = Unit {
    quantity: Some(Cow::Borrowed("area")),
    ids: Cow::Borrowed(&[Cow::Borrowed("square_inch"), Cow::Borrowed("in²")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.000645161,
    offset: 0.0,
};

pub static SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("area")),
    ids: Cow::Borrowed(&[Cow::Borrowed("square_foot"), Cow::Borrowed("ft²")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.092903,
    offset: 0.0,
};

pub static SQUARE_YARD: Unit = Unit {
    quantity: Some(Cow::Borrowed("area")),
    ids: Cow::Borrowed(&[Cow::Borrowed("square_yard"), Cow::Borrowed("yd²")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.836131,
    offset: 0.0,
};

pub static SQUARE_MILE: Unit = Unit {
    quantity: Some(Cow::Borrowed("area")),
    ids: Cow::Borrowed(&[Cow::Borrowed("square_mile"), Cow::Borrowed("mile²")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2589988.110336,
    offset: 0.0,
};

pub static ACRE: Unit = Unit {
    quantity: Some(Cow::Borrowed("area")),
    ids: Cow::Borrowed(&[Cow::Borrowed("acre")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 4046.872627,
    offset: 0.0,
};

// capacitance
pub static FARAD: Unit = Unit {
    quantity: Some(Cow::Borrowed("capacitance")),
    ids: Cow::Borrowed(&[Cow::Borrowed("farad"), Cow::Borrowed("F")]),
    dimensions: Some(UnitDimensions {
        kg: -1,
        m: -2,
        sec: 4,
        k: 0,
        a: 2,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// cooling efficiency
pub static COEFFICIENT_OF_PERFORMANCE: Unit = Unit {
    quantity: Some(Cow::Borrowed("cooling efficiency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("coefficient_of_performance"),
        Cow::Borrowed("COP"),
    ]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static ENERGY_EFFICIENCY_RATIO: Unit = Unit {
    quantity: Some(Cow::Borrowed("cooling efficiency")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("energy_efficiency_ratio"),
        Cow::Borrowed("Btu/Wh"),
        Cow::Borrowed("EER"),
    ]),
    dimensions: None,
    scale: 0.2930832356,
    offset: 0.0,
};

pub static KILOWATT_PER_TON: Unit = Unit {
    quantity: Some(Cow::Borrowed("cooling efficiency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilowatt_per_ton"), Cow::Borrowed("kW/ton")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

// density
pub static KILOGRAMS_PER_CUBIC_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("density")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilograms_per_cubic_meter"),
        Cow::Borrowed("kg/m³"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static GRAMS_PER_CUBIC_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("density")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("grams_per_cubic_meter"),
        Cow::Borrowed("g/m³"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static MILLIGRAMS_PER_CUBIC_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("density")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("milligrams_per_cubic_meter"),
        Cow::Borrowed("mg/m³"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-6,
    offset: 0.0,
};

pub static MICROGRAMS_PER_CUBIC_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("density")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("micrograms_per_cubic_meter"),
        Cow::Borrowed("µg/m³"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-9,
    offset: 0.0,
};

pub static KILOGRAMS_PER_LITER: Unit = Unit {
    quantity: Some(Cow::Borrowed("density")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilograms_per_liter"), Cow::Borrowed("kg/L")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static MILLIGRAMS_PER_LITER: Unit = Unit {
    quantity: Some(Cow::Borrowed("density")),
    ids: Cow::Borrowed(&[Cow::Borrowed("milligrams_per_liter"), Cow::Borrowed("mg/L")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-9,
    offset: 0.0,
};

// electric charge
pub static COULOMB: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric charge")),
    ids: Cow::Borrowed(&[Cow::Borrowed("coulomb"), Cow::Borrowed("C")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 1,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static AMPERE_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric charge")),
    ids: Cow::Borrowed(&[Cow::Borrowed("ampere_hour"), Cow::Borrowed("Ah")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 1,
        mol: 0,
        cd: 0,
    }),
    scale: 3600.0,
    offset: 0.0,
};

// electric conductance
pub static SIEMENS: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric conductance")),
    ids: Cow::Borrowed(&[Cow::Borrowed("siemens"), Cow::Borrowed("S")]),
    dimensions: Some(UnitDimensions {
        kg: -1,
        m: -2,
        sec: 3,
        k: 0,
        a: 2,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// electric current
pub static AMPERE: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric current")),
    ids: Cow::Borrowed(&[Cow::Borrowed("ampere"), Cow::Borrowed("A")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 0,
        a: 1,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static MILLIAMPERE: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric current")),
    ids: Cow::Borrowed(&[Cow::Borrowed("milliampere"), Cow::Borrowed("mA")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 0,
        a: 1,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

// electromagnetic moment
pub static AMPERE_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("electromagnetic moment")),
    ids: Cow::Borrowed(&[Cow::Borrowed("ampere_square_meter"), Cow::Borrowed("Am²")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: 0,
        k: 0,
        a: 1,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// electric current density
pub static AMPERES_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric current density")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("amperes_per_square_meter"),
        Cow::Borrowed("A/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: -2,
        sec: 0,
        k: 0,
        a: 1,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// electric field strength
pub static VOLTS_PER_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric field strength")),
    ids: Cow::Borrowed(&[Cow::Borrowed("volts_per_meter"), Cow::Borrowed("V/m")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 1,
        sec: -3,
        k: 0,
        a: -1,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// electric potential
pub static VOLT: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric potential")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("volt"),
        Cow::Borrowed("Volt"),
        Cow::Borrowed("V"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: -1,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static MILLIVOLT: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric potential")),
    ids: Cow::Borrowed(&[Cow::Borrowed("millivolt"), Cow::Borrowed("mV")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: -1,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static KILOVOLT: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric potential")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilovolt"), Cow::Borrowed("kV")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: -1,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static MEGAVOLT: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric potential")),
    ids: Cow::Borrowed(&[Cow::Borrowed("megavolt"), Cow::Borrowed("MV")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: -1,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

// electric resistance
pub static OHM: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric resistance")),
    ids: Cow::Borrowed(&[Cow::Borrowed("ohm"), Cow::Borrowed("Ω"), Cow::Borrowed("Ω")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: -2,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KILOHM: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric resistance")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilohm"),
        Cow::Borrowed("kΩ"),
        Cow::Borrowed("kΩ"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: -2,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static MEGOHM: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric resistance")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megohm"),
        Cow::Borrowed("MΩ"),
        Cow::Borrowed("MΩ"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: -2,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

pub static MILLIOHM: Unit = Unit {
    quantity: Some(Cow::Borrowed("electric resistance")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("milliohm"),
        Cow::Borrowed("mΩ"),
        Cow::Borrowed("mΩ"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: -2,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

// electrical conductivity
pub static SIEMENS_PER_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("electrical conductivity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("siemens_per_meter"), Cow::Borrowed("S/m")]),
    dimensions: Some(UnitDimensions {
        kg: -1,
        m: -3,
        sec: 3,
        k: 0,
        a: 2,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// electrical resistivity
pub static OHM_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("electrical resistivity")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("ohm_meter"),
        Cow::Borrowed("Ωm"),
        Cow::Borrowed("Ωm"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 3,
        sec: -3,
        k: 0,
        a: -2,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// energy
pub static JOULE: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("joule"), Cow::Borrowed("J")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KILOJOULE: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilojoule"), Cow::Borrowed("kJ")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static WATT_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("watt_hour"), Cow::Borrowed("Wh")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600.0,
    offset: 0.0,
};

pub static KILOWATT_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilowatt_hour"), Cow::Borrowed("kWh")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000.0,
    offset: 0.0,
};

pub static MEGAWATT_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("megawatt_hour"), Cow::Borrowed("MWh")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000000.0,
    offset: 0.0,
};

pub static GIGAWATT_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("gigawatt_hour"), Cow::Borrowed("GWh")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000000000.0,
    offset: 0.0,
};

pub static BTU: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("btu"), Cow::Borrowed("BTU")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1054.852,
    offset: 0.0,
};

pub static KILOBTU: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilobtu"), Cow::Borrowed("kBTU")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1054852.0,
    offset: 0.0,
};

pub static MEGABTU: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megabtu"),
        Cow::Borrowed("MBTU"),
        Cow::Borrowed("MMBTU"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1054852000.0,
    offset: 0.0,
};

pub static HORSEPOWER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("horsepower_hour"), Cow::Borrowed("hph")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2686088.6,
    offset: 0.0,
};

pub static CALORIE: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("calorie"), Cow::Borrowed("cal")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 4.184,
    offset: 0.0,
};

pub static THERM: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("therm")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 105506000.0,
    offset: 0.0,
};

pub static TONS_REFRIGERATION_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("tons_refrigeration_hour"),
        Cow::Borrowed("tonrefh"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 12660670.8,
    offset: 0.0,
};

pub static MEGAJOULE: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("megajoule"), Cow::Borrowed("MJ")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

pub static GIGAJOULE: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("gigajoule"), Cow::Borrowed("GJ")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000000.0,
    offset: 0.0,
};

pub static NEWTON_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("newton_meter"), Cow::Borrowed("Nm")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static CUBIC_METERS_NATURAL_GAS: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("cubic_meters_natural_gas"),
        Cow::Borrowed("standard_cubic_meter"),
        Cow::Borrowed("scm"),
        Cow::Borrowed("m³_gas"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 37313432.83582089,
    offset: 0.0,
};

pub static CUBIC_FEET_NATURAL_GAS: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("cubic_feet_natural_gas"),
        Cow::Borrowed("standard_cubic_foot"),
        Cow::Borrowed("scf"),
        Cow::Borrowed("ft³_gas"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1086498.0,
    offset: 0.0,
};

pub static HUNDRED_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("hundred_cubic_feet_natural_gas"),
        Cow::Borrowed("Hcf_natural_gas"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 108649800.0,
    offset: 0.0,
};

pub static CENTUM_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("centum_cubic_feet_natural_gas"),
        Cow::Borrowed("Ccf_natural_gas"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 108649800.0,
    offset: 0.0,
};

pub static THOUSAND_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("thousand_cubic_feet_natural_gas"),
        Cow::Borrowed("Mcf_natural_gas"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1086498000.0,
    offset: 0.0,
};

pub static MILLION_CUBIC_FEET_NATURAL_GAS: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("million_cubic_feet_natural_gas"),
        Cow::Borrowed("MMcf_natural_gas"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1086498000000.0,
    offset: 0.0,
};

// apparent energy
pub static VOLT_AMPERE_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("apparent energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("volt_ampere_hour"), Cow::Borrowed("VAh")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600.0,
    offset: 0.0,
};

pub static KILOVOLT_AMPERE_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("apparent energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilovolt_ampere_hour"), Cow::Borrowed("kVAh")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000.0,
    offset: 0.0,
};

pub static MEGAVOLT_AMPERE_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("apparent energy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("megavolt_ampere_hour"), Cow::Borrowed("MVAh")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000000.0,
    offset: 0.0,
};

// reactive energy
pub static VOLT_AMPERE_REACTIVE_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("reactive energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("volt_ampere_reactive_hour"),
        Cow::Borrowed("VARh"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600.0,
    offset: 0.0,
};

pub static KILOVOLT_AMPERE_REACTIVE_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("reactive energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilovolt_ampere_reactive_hour"),
        Cow::Borrowed("kVARh"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000.0,
    offset: 0.0,
};

pub static MEGAVOLT_AMPERE_REACTIVE_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("reactive energy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megavolt_ampere_reactive_hour"),
        Cow::Borrowed("MVARh"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000000.0,
    offset: 0.0,
};

// energy by area
pub static JOULES_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("joules_per_square_meter"),
        Cow::Borrowed("J/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static WATT_HOURS_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watt_hours_per_square_meter"),
        Cow::Borrowed("Wh/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600.0,
    offset: 0.0,
};

pub static WATT_HOURS_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watt_hours_per_square_foot"),
        Cow::Borrowed("Wh/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 38750.077500155,
    offset: 0.0,
};

pub static KILOWATT_HOURS_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilowatt_hours_per_square_meter"),
        Cow::Borrowed("kWh/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000.0,
    offset: 0.0,
};

pub static KILOWATT_HOURS_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilowatt_hours_per_square_foot"),
        Cow::Borrowed("kWh/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 38750077.500155,
    offset: 0.0,
};

pub static MEGAWATT_HOURS_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megawatt_hours_per_square_meter"),
        Cow::Borrowed("MWh/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000000.0,
    offset: 0.0,
};

pub static MEGAWATT_HOURS_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megawatt_hours_per_square_foot"),
        Cow::Borrowed("MWh/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 38750077500.155,
    offset: 0.0,
};

pub static MEGAJOULES_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megajoules_per_square_meter"),
        Cow::Borrowed("MJ/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

pub static MEGAJOULES_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megajoules_per_square_foot"),
        Cow::Borrowed("MJ/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 10763910.41671,
    offset: 0.0,
};

pub static BTU_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("btu_per_square_foot"),
        Cow::Borrowed("BTU/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 11354.33731957,
    offset: 0.0,
};

pub static KILOBTU_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilobtu_per_square_foot"),
        Cow::Borrowed("kBTU/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 11354337.31957,
    offset: 0.0,
};

pub static MEGABTU_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megabtu_per_square_foot"),
        Cow::Borrowed("MBTU/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 11354337319.57,
    offset: 0.0,
};

// energy by volume
pub static JOULES_PER_CUBIC_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by volume")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("joules_per_cubic_meter"),
        Cow::Borrowed("J/m³"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static GIGAJOULES_PER_CUBIC_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by volume")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("gigajoules_per_cubic_meter"),
        Cow::Borrowed("GJ/m³"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000000.0,
    offset: 0.0,
};

pub static KILOWATT_HOURS_PER_CUBIC_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("energy by volume")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilowatt_hours_per_cubic_meter"),
        Cow::Borrowed("kWh/m³"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600000.0,
    offset: 0.0,
};

// enthalpy
pub static JOULES_PER_GRAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("enthalpy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("joules_per_gram"), Cow::Borrowed("J/g")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static JOULES_PER_KILOGRAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("enthalpy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("joules_per_kilogram"), Cow::Borrowed("J/kg")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static JOULES_PER_KILOGRAM_DRY_AIR: Unit = Unit {
    quantity: Some(Cow::Borrowed("enthalpy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("joules_per_kilogram_dry_air"),
        Cow::Borrowed("J/kg_dry"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static BTU_PER_POUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("enthalpy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("btu_per_pound"), Cow::Borrowed("BTU/lb")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2325.5576058607867,
    offset: 0.0,
};

pub static BTUS_PER_POUND_DRY_AIR: Unit = Unit {
    quantity: Some(Cow::Borrowed("enthalpy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("btus_per_pound_dry_air"),
        Cow::Borrowed("btu/lb_dry"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2326.0,
    offset: 0.0,
};

pub static KILOJOULES_PER_KILOGRAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("enthalpy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilojoules_per_kilogram"),
        Cow::Borrowed("kJ/kg"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static KILOJOULES_PER_KILOGRAM_DRY_AIR: Unit = Unit {
    quantity: Some(Cow::Borrowed("enthalpy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilojoules_per_kilogram_dry_air"),
        Cow::Borrowed("kJ/kg_dry"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static MEGAJOULES_PER_KILOGRAM_DRY_AIR: Unit = Unit {
    quantity: Some(Cow::Borrowed("enthalpy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megajoules_per_kilogram_dry_air"),
        Cow::Borrowed("MJ/kg_dry"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

pub static CALORIE_PER_GRAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("enthalpy")),
    ids: Cow::Borrowed(&[Cow::Borrowed("calorie_per_gram"), Cow::Borrowed("cal/g")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 4184.0,
    offset: 0.0,
};

// entropy
pub static JOULES_PER_DEGREE_KELVIN: Unit = Unit {
    quantity: Some(Cow::Borrowed("entropy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("joules_per_degree_kelvin"),
        Cow::Borrowed("J/°K"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: -1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KILOJOULES_PER_DEGREE_KELVIN: Unit = Unit {
    quantity: Some(Cow::Borrowed("entropy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilojoules_per_degree_kelvin"),
        Cow::Borrowed("kJ/°K"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: -1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static MEGAJOULES_PER_DEGREE_KELVIN: Unit = Unit {
    quantity: Some(Cow::Borrowed("entropy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megajoules_per_degree_kelvin"),
        Cow::Borrowed("MJ/°K"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: -1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

// force
pub static NEWTON: Unit = Unit {
    quantity: Some(Cow::Borrowed("force")),
    ids: Cow::Borrowed(&[Cow::Borrowed("newton"), Cow::Borrowed("N")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static POUND_FORCE: Unit = Unit {
    quantity: Some(Cow::Borrowed("force")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pound_force"), Cow::Borrowed("lbf")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 4.448222,
    offset: 0.0,
};

// frequency
pub static HERTZ: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("hertz"), Cow::Borrowed("Hz")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KILOHERTZ: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilohertz"), Cow::Borrowed("kHz")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static CYCLES_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cycles_per_hour"), Cow::Borrowed("cph")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0002777777777777778,
    offset: 0.0,
};

pub static CYCLES_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cycles_per_minute"), Cow::Borrowed("cpm")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.016666666666666666,
    offset: 0.0,
};

pub static MEGAHERTZ: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("megahertz"), Cow::Borrowed("MHz")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

pub static PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("per_minute"), Cow::Borrowed("/min")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.016666666666666666,
    offset: 0.0,
};

pub static PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("per_second"), Cow::Borrowed("/s")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("per_hour"), Cow::Borrowed("/h")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0002777777777777778,
    offset: 0.0,
};

pub static PERCENT_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("percent_per_second"), Cow::Borrowed("%/s")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static AIR_CHANGES_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("frequency")),
    ids: Cow::Borrowed(&[Cow::Borrowed("air_changes_per_hour"), Cow::Borrowed("ACH")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0002777777777777778,
    offset: 0.0,
};

// grammage
pub static KILOGRAMS_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("grammage")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilograms_per_square_meter"),
        Cow::Borrowed("kg/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static GRAMS_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("grammage")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("grams_per_square_meter"),
        Cow::Borrowed("g/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

// heating rate
pub static DEGREES_KELVIN_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("heating rate")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("degrees_kelvin_per_second"),
        Cow::Borrowed("K/s"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static DEGREES_CELSIUS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("heating rate")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("degrees_celsius_per_hour"),
        Cow::Borrowed("°C/h"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0002777777777777778,
    offset: 0.0,
};

pub static DEGREES_CELSIUS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("heating rate")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("degrees_celsius_per_minute"),
        Cow::Borrowed("°C/min"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.016666666666666666,
    offset: 0.0,
};

pub static DEGREES_FAHRENHEIT_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("heating rate")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("degrees_fahrenheit_per_hour"),
        Cow::Borrowed("°F/h"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.00015432098765432,
    offset: 0.0,
};

pub static DEGREES_FAHRENHEIT_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("heating rate")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("degrees_fahrenheit_per_minute"),
        Cow::Borrowed("°F/min"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0092592592592593,
    offset: 0.0,
};

pub static DEGREES_KELVIN_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("heating rate")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("degrees_kelvin_per_hour"),
        Cow::Borrowed("K/h"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0002777777777777778,
    offset: 0.0,
};

pub static DEGREES_KELVIN_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("heating rate")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("degrees_kelvin_per_minute"),
        Cow::Borrowed("K/min"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: -1,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.016666666666666666,
    offset: 0.0,
};

// illuminance
pub static LUX: Unit = Unit {
    quantity: Some(Cow::Borrowed("illuminance")),
    ids: Cow::Borrowed(&[Cow::Borrowed("lux"), Cow::Borrowed("lx")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: -2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 1,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static FOOTCANDLE: Unit = Unit {
    quantity: Some(Cow::Borrowed("illuminance")),
    ids: Cow::Borrowed(&[Cow::Borrowed("footcandle"), Cow::Borrowed("fc")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: -2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 1,
    }),
    scale: 0.092937,
    offset: 0.0,
};

pub static PHOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("illuminance")),
    ids: Cow::Borrowed(&[Cow::Borrowed("phot")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: -2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 1,
    }),
    scale: 10000.0,
    offset: 0.0,
};

// inductance
pub static HENRY: Unit = Unit {
    quantity: Some(Cow::Borrowed("inductance")),
    ids: Cow::Borrowed(&[Cow::Borrowed("henry"), Cow::Borrowed("H")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: -2,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// irradiance
pub static WATTS_PER_SQUARE_METER_IRRADIANCE: Unit = Unit {
    quantity: Some(Cow::Borrowed("irradiance")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watts_per_square_meter_irradiance"),
        Cow::Borrowed("W/m²_irr"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static WATTS_PER_SQUARE_FOOT_IRRADIANCE: Unit = Unit {
    quantity: Some(Cow::Borrowed("irradiance")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watts_per_square_foot_irradiance"),
        Cow::Borrowed("W/ft²_irr"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 10.76391041671,
    offset: 0.0,
};

// length
pub static METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("length")),
    ids: Cow::Borrowed(&[Cow::Borrowed("meter"), Cow::Borrowed("m")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static MICROMETER: Unit = Unit {
    quantity: Some(Cow::Borrowed("length")),
    ids: Cow::Borrowed(&[Cow::Borrowed("micrometer"), Cow::Borrowed("µm")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-5,
    offset: 0.0,
};

pub static MILLIMETER: Unit = Unit {
    quantity: Some(Cow::Borrowed("length")),
    ids: Cow::Borrowed(&[Cow::Borrowed("millimeter"), Cow::Borrowed("mm")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static CENTIMETER: Unit = Unit {
    quantity: Some(Cow::Borrowed("length")),
    ids: Cow::Borrowed(&[Cow::Borrowed("centimeter"), Cow::Borrowed("cm")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.01,
    offset: 0.0,
};

pub static KILOMETER: Unit = Unit {
    quantity: Some(Cow::Borrowed("length")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilometer"), Cow::Borrowed("km")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static INCH: Unit = Unit {
    quantity: Some(Cow::Borrowed("length")),
    ids: Cow::Borrowed(&[Cow::Borrowed("inch"), Cow::Borrowed("in")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0254,
    offset: 0.0,
};

pub static FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("length")),
    ids: Cow::Borrowed(&[Cow::Borrowed("foot"), Cow::Borrowed("ft")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.3048,
    offset: 0.0,
};

pub static YARD: Unit = Unit {
    quantity: Some(Cow::Borrowed("length")),
    ids: Cow::Borrowed(&[Cow::Borrowed("yard"), Cow::Borrowed("yd")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.9144,
    offset: 0.0,
};

pub static MILE: Unit = Unit {
    quantity: Some(Cow::Borrowed("length")),
    ids: Cow::Borrowed(&[Cow::Borrowed("mile")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1609.344,
    offset: 0.0,
};

// luminance
pub static CANDELAS_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("luminance")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("candelas_per_square_meter"),
        Cow::Borrowed("cd/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: -2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 1,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static CANDELS_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("luminance")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("candels_per_square_foot"),
        Cow::Borrowed("cd/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: -2,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 1,
    }),
    scale: 0.092937,
    offset: 0.0,
};

// luminous flux
pub static LUMEN: Unit = Unit {
    quantity: Some(Cow::Borrowed("luminous flux")),
    ids: Cow::Borrowed(&[Cow::Borrowed("lumen"), Cow::Borrowed("lm")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 1,
    }),
    scale: 1.0,
    offset: 0.0,
};

// luminous intensity
pub static CANDELA: Unit = Unit {
    quantity: Some(Cow::Borrowed("luminous intensity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("candela"), Cow::Borrowed("cd")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 1,
    }),
    scale: 1.0,
    offset: 0.0,
};

// magnetic field strength
pub static AMPERES_PER_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("magnetic field strength")),
    ids: Cow::Borrowed(&[Cow::Borrowed("amperes_per_meter"), Cow::Borrowed("A/m")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: -1,
        sec: 0,
        k: 0,
        a: 1,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// magnetic flux
pub static WEBER: Unit = Unit {
    quantity: Some(Cow::Borrowed("magnetic flux")),
    ids: Cow::Borrowed(&[Cow::Borrowed("weber"), Cow::Borrowed("Wb")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -2,
        k: 0,
        a: -1,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// magnetic flux density
pub static TESLA: Unit = Unit {
    quantity: Some(Cow::Borrowed("magnetic flux density")),
    ids: Cow::Borrowed(&[Cow::Borrowed("tesla"), Cow::Borrowed("T")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: -1,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// mass
pub static KILOGRAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilogram"), Cow::Borrowed("kg")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static MILLIGRAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass")),
    ids: Cow::Borrowed(&[Cow::Borrowed("milligram"), Cow::Borrowed("mg")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-6,
    offset: 0.0,
};

pub static GRAM: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass")),
    ids: Cow::Borrowed(&[Cow::Borrowed("gram"), Cow::Borrowed("g")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static OUNCE: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass")),
    ids: Cow::Borrowed(&[Cow::Borrowed("ounce"), Cow::Borrowed("oz")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.02835,
    offset: 0.0,
};

pub static POUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pound"), Cow::Borrowed("lb")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.453591,
    offset: 0.0,
};

pub static KILOPOUND: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilopound"), Cow::Borrowed("klb")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 453.591,
    offset: 0.0,
};

pub static METRIC_TON: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass")),
    ids: Cow::Borrowed(&[Cow::Borrowed("metric_ton"), Cow::Borrowed("ton")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static SHORT_TON: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass")),
    ids: Cow::Borrowed(&[Cow::Borrowed("short_ton"), Cow::Borrowed("t")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 907.18474,
    offset: 0.0,
};

// mass flow
pub static KILOGRAMS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilograms_per_second"), Cow::Borrowed("kg/s")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KILOGRAMS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilograms_per_minute"),
        Cow::Borrowed("kg/min"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.016666666666666666,
    offset: 0.0,
};

pub static KILOGRAMS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilograms_per_hour"), Cow::Borrowed("kg/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0002777777777777778,
    offset: 0.0,
};

pub static POUNDS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pounds_per_minute"), Cow::Borrowed("lb/min")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.007559872833333333,
    offset: 0.0,
};

pub static POUNDS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pounds_per_hour"), Cow::Borrowed("lb/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.00012599788055555556,
    offset: 0.0,
};

pub static POUNDS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pounds_per_second"), Cow::Borrowed("lb/s")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.45359237,
    offset: 0.0,
};

pub static KILOPOUNDS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilopounds_per_hour"), Cow::Borrowed("klb/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.12599788055555555,
    offset: 0.0,
};

pub static GRAMS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("grams_per_second"), Cow::Borrowed("g/s")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static GRAMS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("grams_per_minute"), Cow::Borrowed("g/min")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.6666666666666667e-5,
    offset: 0.0,
};

pub static METRIC_TONS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("mass flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("metric_tons_per_hour"),
        Cow::Borrowed("ton/h"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.2777777777777778,
    offset: 0.0,
};

// momentum
pub static NEWTON_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("momentum")),
    ids: Cow::Borrowed(&[Cow::Borrowed("newton_second"), Cow::Borrowed("Ns")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// power
pub static WATT: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("watt"), Cow::Borrowed("W")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static MILLIWATT: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("milliwatt"), Cow::Borrowed("mW")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static KILOWATT: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilowatt"), Cow::Borrowed("kW")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static MEGAWATT: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("megawatt"), Cow::Borrowed("MW")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

pub static GIGAWATT: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("gigawatt"), Cow::Borrowed("GW")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000000.0,
    offset: 0.0,
};

pub static BTUS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("btus_per_hour"), Cow::Borrowed("BTU/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.292875,
    offset: 0.0,
};

pub static THERMS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("therms_per_hour"), Cow::Borrowed("therm/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 29287.5,
    offset: 0.0,
};

pub static HORSEPOWER: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("horsepower"), Cow::Borrowed("hp")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 745.7,
    offset: 0.0,
};

pub static FOOT_POUNDS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("foot_pounds_per_second"),
        Cow::Borrowed("ftlbs/sec"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.355818,
    offset: 0.0,
};

pub static TONS_REFRIGERATION: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("tons_refrigeration"), Cow::Borrowed("tonref")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3516.853,
    offset: 0.0,
};

pub static KILOBTUS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilobtus_per_hour"), Cow::Borrowed("kBTU/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 293.07107017222,
    offset: 0.0,
};

pub static MEGABTUS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megabtus_per_hour"),
        Cow::Borrowed("MBTU/h"),
        Cow::Borrowed("MMBTU/h"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 293071.07017222,
    offset: 0.0,
};

pub static JOULES_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("joules_per_hour"), Cow::Borrowed("J/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.000277777778,
    offset: 0.0,
};

pub static KILOJOULES_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilojoules_per_hour"), Cow::Borrowed("kJ/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.277777778,
    offset: 0.0,
};

pub static MEGAJOULES_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("megajoules_per_hour"), Cow::Borrowed("MJ/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 277.777778,
    offset: 0.0,
};

pub static GIGAJOULES_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("gigajoules_per_hour"), Cow::Borrowed("GJ/h")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 277777.778,
    offset: 0.0,
};

// power by area
pub static WATTS_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("power by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watts_per_square_meter"),
        Cow::Borrowed("W/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static WATTS_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("power by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watts_per_square_foot"),
        Cow::Borrowed("W/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 10.7639104,
    offset: 0.0,
};

pub static KILOWATTS_PER_SQUARE_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("power by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilowatts_per_square_meter"),
        Cow::Borrowed("kW/m²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static KILOWATTS_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("power by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilowatts_per_square_foot"),
        Cow::Borrowed("kW/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 10763.9104,
    offset: 0.0,
};

pub static KILOBTUS_PER_HOUR_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("power by area")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilobtus_per_hour_per_square_foot"),
        Cow::Borrowed("kBTU/h/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3153.8257472,
    offset: 0.0,
};

// power by volumetric flow
pub static WATTS_PER_CUBIC_METERS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("power by volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watts_per_cubic_meters_per_second"),
        Cow::Borrowed("W/m³/s"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static WATTS_PER_CUBIC_FEET_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("power by volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watts_per_cubic_feet_per_minute"),
        Cow::Borrowed("W/cfm"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2118.8800032893155,
    offset: 0.0,
};

pub static KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("power by volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilowatts_per_kilocubic_feet_per_minute"),
        Cow::Borrowed("kW/kcfm"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2118.8800032893155,
    offset: 0.0,
};

pub static KILOWATTS_PER_GALLONS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("power by volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilowatts_per_gallons_per_minute"),
        Cow::Borrowed("kW/gal/min"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 15850323.0,
    offset: 0.0,
};

// apparent power
pub static VOLT_AMPERE: Unit = Unit {
    quantity: Some(Cow::Borrowed("apparent power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("volt_ampere"), Cow::Borrowed("VA")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KILOVOLT_AMPERE: Unit = Unit {
    quantity: Some(Cow::Borrowed("apparent power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilovolt_ampere"), Cow::Borrowed("kVA")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static MEGAVOLT_AMPERE: Unit = Unit {
    quantity: Some(Cow::Borrowed("apparent power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("megavolt_ampere"), Cow::Borrowed("mVA")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

// reactive power
pub static VOLT_AMPERE_REACTIVE: Unit = Unit {
    quantity: Some(Cow::Borrowed("reactive power")),
    ids: Cow::Borrowed(&[Cow::Borrowed("volt_ampere_reactive"), Cow::Borrowed("VAR")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KILOVOLT_AMPERE_REACTIVE: Unit = Unit {
    quantity: Some(Cow::Borrowed("reactive power")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilovolt_ampere_reactive"),
        Cow::Borrowed("kVAR"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static MEGAVOLT_AMPERE_REACTIVE: Unit = Unit {
    quantity: Some(Cow::Borrowed("reactive power")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("megavolt_ampere_reactive"),
        Cow::Borrowed("MVAR"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 2,
        sec: -3,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000000.0,
    offset: 0.0,
};

// pressure
pub static PASCAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pascal"), Cow::Borrowed("Pa")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KILOPASCAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilopascal"), Cow::Borrowed("kPa")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static BAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[Cow::Borrowed("bar")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 100000.0,
    offset: 0.0,
};

pub static ATMOSPHERE: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[Cow::Borrowed("atmosphere"), Cow::Borrowed("atm")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 101317.1,
    offset: 0.0,
};

pub static POUNDS_PER_SQUARE_INCH: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("pounds_per_square_inch"),
        Cow::Borrowed("psi"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 6894.73,
    offset: 0.0,
};

pub static CENTIMETERS_OF_WATER: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("centimeters_of_water"),
        Cow::Borrowed("cmH₂O"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 98.0665,
    offset: 0.0,
};

pub static INCHES_OF_WATER: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("inches_of_water"),
        Cow::Borrowed("in/wc"),
        Cow::Borrowed("inH₂O"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 248.84,
    offset: 0.0,
};

pub static MILLIMETERS_OF_MERCURY: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("millimeters_of_mercury"),
        Cow::Borrowed("mmHg"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 133.322368421,
    offset: 0.0,
};

pub static CENTIMETERS_OF_MERCURY: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("centimeters_of_mercury"),
        Cow::Borrowed("cmHg"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1333.22368421,
    offset: 0.0,
};

pub static INCHES_OF_MERCURY: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[Cow::Borrowed("inches_of_mercury"), Cow::Borrowed("inHg")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3386.38815789,
    offset: 0.0,
};

pub static HECTOPASCAL: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[Cow::Borrowed("hectopascal"), Cow::Borrowed("hPa")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 100.0,
    offset: 0.0,
};

pub static MILLIBAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("pressure")),
    ids: Cow::Borrowed(&[Cow::Borrowed("millibar"), Cow::Borrowed("mbar")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: -1,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 100.0,
    offset: 0.0,
};

// specific entropy
pub static JOULES_PER_KILOGRAM_DEGREE_KELVIN: Unit = Unit {
    quantity: Some(Cow::Borrowed("specific entropy")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("joules_per_kilogram_degree_kelvin"),
        Cow::Borrowed("J/kg°K"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 2,
        sec: -2,
        k: -1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// surface tension
pub static NEWTONS_PER_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("surface tension")),
    ids: Cow::Borrowed(&[Cow::Borrowed("newtons_per_meter"), Cow::Borrowed("N/m")]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 0,
        sec: -2,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// temperature
pub static FAHRENHEIT: Unit = Unit {
    quantity: Some(Cow::Borrowed("temperature")),
    ids: Cow::Borrowed(&[Cow::Borrowed("fahrenheit"), Cow::Borrowed("°F")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.5555555555555556,
    offset: 255.37222222222223,
};

pub static CELSIUS: Unit = Unit {
    quantity: Some(Cow::Borrowed("temperature")),
    ids: Cow::Borrowed(&[Cow::Borrowed("celsius"), Cow::Borrowed("°C")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 273.15,
};

pub static KELVIN: Unit = Unit {
    quantity: Some(Cow::Borrowed("temperature")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kelvin"), Cow::Borrowed("K")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// temperature differential
pub static FAHRENHEIT_DEGREES: Unit = Unit {
    quantity: Some(Cow::Borrowed("temperature differential")),
    ids: Cow::Borrowed(&[Cow::Borrowed("fahrenheit_degrees"), Cow::Borrowed("Δ°F")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.5555555555555556,
    offset: 0.0,
};

pub static CELSIUS_DEGREES: Unit = Unit {
    quantity: Some(Cow::Borrowed("temperature differential")),
    ids: Cow::Borrowed(&[Cow::Borrowed("celsius_degrees"), Cow::Borrowed("Δ°C")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KELVIN_DEGREES: Unit = Unit {
    quantity: Some(Cow::Borrowed("temperature differential")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kelvin_degrees"), Cow::Borrowed("ΔK")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 0,
        k: 1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// thermal conductivity
pub static WATTS_PER_METER_DEGREE_KELVIN: Unit = Unit {
    quantity: Some(Cow::Borrowed("thermal conductivity")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("watts_per_meter_degree_kelvin"),
        Cow::Borrowed("W/m°K"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 1,
        m: 1,
        sec: -3,
        k: -1,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

// time
pub static NANOSECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("nanosecond"), Cow::Borrowed("ns")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-9,
    offset: 0.0,
};

pub static MICROSECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("microsecond"), Cow::Borrowed("µs")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-6,
    offset: 0.0,
};

pub static MILLISECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("millisecond"), Cow::Borrowed("ms")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static HUNDREDTHS_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("hundredths_second"), Cow::Borrowed("cs")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.01,
    offset: 0.0,
};

pub static TENTHS_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("tenths_second"), Cow::Borrowed("ds")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.1,
    offset: 0.0,
};

pub static SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("second"),
        Cow::Borrowed("s"),
        Cow::Borrowed("sec"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("minute"), Cow::Borrowed("min")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 60.0,
    offset: 0.0,
};

pub static HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("hour"),
        Cow::Borrowed("h"),
        Cow::Borrowed("hr"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3600.0,
    offset: 0.0,
};

pub static DAY: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("day")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 86400.0,
    offset: 0.0,
};

pub static WEEK: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("week"), Cow::Borrowed("wk")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 604800.0,
    offset: 0.0,
};

pub static JULIAN_MONTH: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("julian_month"), Cow::Borrowed("mo")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2629800.0,
    offset: 0.0,
};

pub static YEAR: Unit = Unit {
    quantity: Some(Cow::Borrowed("time")),
    ids: Cow::Borrowed(&[Cow::Borrowed("year"), Cow::Borrowed("yr")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 0,
        sec: 1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 31536000.0,
    offset: 0.0,
};

// velocity
pub static METERS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("meters_per_second"), Cow::Borrowed("m/s")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static KILOMETERS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilometers_per_second"),
        Cow::Borrowed("km/s"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1000.0,
    offset: 0.0,
};

pub static KILOMETERS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilometers_per_hour"), Cow::Borrowed("km/h")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.277778,
    offset: 0.0,
};

pub static MILES_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("miles_per_hour"), Cow::Borrowed("mph")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.447027,
    offset: 0.0,
};

pub static FEET_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("feet_per_second"), Cow::Borrowed("ft/s")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.3048,
    offset: 0.0,
};

pub static FEET_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("feet_per_minute"), Cow::Borrowed("ft/min")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.00508,
    offset: 0.0,
};

pub static INCHES_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("inches_per_hour"), Cow::Borrowed("in/h")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 7.055555555555556e-6,
    offset: 0.0,
};

pub static MILLIMETERS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("millimeters_per_second"),
        Cow::Borrowed("mm/s"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static MILLIMETERS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("millimeters_per_minute"),
        Cow::Borrowed("mm/min"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.6666666666666667e-5,
    offset: 0.0,
};

pub static MILLIMETERS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("millimeters_per_hour"), Cow::Borrowed("mm/h")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2.7777777777777776e-7,
    offset: 0.0,
};

pub static METERS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("meters_per_minute"), Cow::Borrowed("m/min")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.016666666666666666,
    offset: 0.0,
};

pub static METERS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("meters_per_hour"), Cow::Borrowed("m/h")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0002777777777777778,
    offset: 0.0,
};

pub static KNOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[Cow::Borrowed("knot")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.5144,
    offset: 0.0,
};

pub static CUBIC_FEET_PER_MINUTE_PER_SQUARE_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("velocity")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("cubic_feet_per_minute_per_square_foot"),
        Cow::Borrowed("cfm/ft²"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 1,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.00508,
    offset: 0.0,
};

// volume
pub static CUBIC_METER: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cubic_meter"), Cow::Borrowed("m³")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static CUBIC_MILLIMETER: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cubic_millimeter"), Cow::Borrowed("mm³")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-9,
    offset: 0.0,
};

pub static CUBIC_CENTIMETER: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cubic_centimeter"), Cow::Borrowed("cm³")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-6,
    offset: 0.0,
};

pub static MILLILITER: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("milliliter"), Cow::Borrowed("mL")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-6,
    offset: 0.0,
};

pub static HECTOLITER: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("hectoliter"), Cow::Borrowed("hL")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.1,
    offset: 0.0,
};

pub static LITER: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("liter"), Cow::Borrowed("L")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static KILOLITER: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kiloliter"), Cow::Borrowed("kL")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static CUBIC_INCH: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cubic_inch"), Cow::Borrowed("in³")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.6387064e-5,
    offset: 0.0,
};

pub static CUBIC_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cubic_foot"), Cow::Borrowed("ft³")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.028316846592,
    offset: 0.0,
};

pub static CUBIC_YARD: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cubic_yard"), Cow::Borrowed("yd³")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.764555,
    offset: 0.0,
};

pub static GALLON: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("gallon"), Cow::Borrowed("gal")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.003785,
    offset: 0.0,
};

pub static KILOGALLON: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilogallon"), Cow::Borrowed("kgal")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 3.785,
    offset: 0.0,
};

pub static QUART: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("quart"), Cow::Borrowed("qt")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.000946,
    offset: 0.0,
};

pub static PINT: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("pint"), Cow::Borrowed("pt")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.000473,
    offset: 0.0,
};

pub static FLUID_OUNCE: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("fluid_ounce"), Cow::Borrowed("fl_oz")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2.95729e-5,
    offset: 0.0,
};

pub static IMPERIAL_GALLON: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("imperial_gallon"), Cow::Borrowed("galUK")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.004546092,
    offset: 0.0,
};

pub static HECTO_CUBIC_FOOT: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("hecto_cubic_foot"), Cow::Borrowed("hft³")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2.8316846592,
    offset: 0.0,
};

pub static HUNDRED_CUBIC_FEET: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("hundred_cubic_feet"), Cow::Borrowed("Hcf")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2.8316846592,
    offset: 0.0,
};

pub static CENTUM_CUBIC_FEET: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("centum_cubic_feet"), Cow::Borrowed("Ccf")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2.8316846592,
    offset: 0.0,
};

pub static THOUSAND_CUBIC_FEET: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("thousand_cubic_feet"), Cow::Borrowed("Mcf")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 28.316846592,
    offset: 0.0,
};

pub static MILLION_CUBIC_FEET: Unit = Unit {
    quantity: Some(Cow::Borrowed("volume")),
    ids: Cow::Borrowed(&[Cow::Borrowed("million_cubic_feet"), Cow::Borrowed("MMcf")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: 0,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 28316.846592,
    offset: 0.0,
};

// volumetric flow
pub static CUBIC_METERS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("cubic_meters_per_second"),
        Cow::Borrowed("m³/s"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0,
    offset: 0.0,
};

pub static MILLILITERS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("milliliters_per_second"),
        Cow::Borrowed("mL/s"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1e-6,
    offset: 0.0,
};

pub static HECTOLITERS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("hectoliters_per_second"),
        Cow::Borrowed("hL/s"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.1,
    offset: 0.0,
};

pub static LITERS_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("liters_per_second"), Cow::Borrowed("L/s")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.001,
    offset: 0.0,
};

pub static CUBIC_FEET_PER_SECOND: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cubic_feet_per_second"), Cow::Borrowed("cfs")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.028317,
    offset: 0.0,
};

pub static CUBIC_FEET_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cubic_feet_per_minute"), Cow::Borrowed("cfm")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0004719474432,
    offset: 0.0,
};

pub static CUBIC_FEET_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("cubic_feet_per_hour"), Cow::Borrowed("cfh")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 7.866e-6,
    offset: 0.0,
};

pub static KILOCUBIC_FEET_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("kilocubic_feet_per_minute"),
        Cow::Borrowed("kcfm"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.4719474432,
    offset: 0.0,
};

pub static IMPERIAL_GALLONS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("imperial_gallons_per_minute"),
        Cow::Borrowed("galUK/min"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.004546092,
    offset: 0.0,
};

pub static LITERS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("liters_per_minute"), Cow::Borrowed("L/min")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.6666666666666667e-5,
    offset: 0.0,
};

pub static GALLONS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("gallons_per_minute"),
        Cow::Borrowed("gal/min"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 6.30901964e-5,
    offset: 0.0,
};

pub static GALLONS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("gallons_per_hour"),
        Cow::Borrowed("gal/hr"),
        Cow::Borrowed("gph"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 1.0515033e-6,
    offset: 0.0,
};

pub static LITERS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[Cow::Borrowed("liters_per_hour"), Cow::Borrowed("L/h")]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 2.7777777777777776e-7,
    offset: 0.0,
};

pub static CUBIC_METERS_PER_MINUTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("cubic_meters_per_minute"),
        Cow::Borrowed("m³/min"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.016666666666666666,
    offset: 0.0,
};

pub static CUBIC_METERS_PER_HOUR: Unit = Unit {
    quantity: Some(Cow::Borrowed("volumetric flow")),
    ids: Cow::Borrowed(&[
        Cow::Borrowed("cubic_meters_per_hour"),
        Cow::Borrowed("m³/h"),
    ]),
    dimensions: Some(UnitDimensions {
        kg: 0,
        m: 3,
        sec: -1,
        k: 0,
        a: 0,
        mol: 0,
        cd: 0,
    }),
    scale: 0.0002777777777777778,
    offset: 0.0,
};

// bytes
pub static BYTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("bytes")),
    ids: Cow::Borrowed(&[Cow::Borrowed("byte")]),
    dimensions: None,
    scale: 1.0,
    offset: 0.0,
};

pub static KILOBYTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("bytes")),
    ids: Cow::Borrowed(&[Cow::Borrowed("kilobyte"), Cow::Borrowed("kB")]),
    dimensions: None,
    scale: 1024.0,
    offset: 0.0,
};

pub static MEGABYTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("bytes")),
    ids: Cow::Borrowed(&[Cow::Borrowed("megabyte"), Cow::Borrowed("MB")]),
    dimensions: None,
    scale: 1048576.0,
    offset: 0.0,
};

pub static GIGABYTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("bytes")),
    ids: Cow::Borrowed(&[Cow::Borrowed("gigabyte"), Cow::Borrowed("GB")]),
    dimensions: None,
    scale: 1073741824.0,
    offset: 0.0,
};

pub static TERABYTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("bytes")),
    ids: Cow::Borrowed(&[Cow::Borrowed("terabyte"), Cow::Borrowed("TB")]),
    dimensions: None,
    scale: 1099511627776.0,
    offset: 0.0,
};

pub static PETABYTE: Unit = Unit {
    quantity: Some(Cow::Borrowed("bytes")),
    ids: Cow::Borrowed(&[Cow::Borrowed("petabyte"), Cow::Borrowed("PB")]),
    dimensions: None,
    scale: 1125899906842624.0,
    offset: 0.0,
};

pub static UNITS: LazyLock<HashMap<&'static str, &'static Unit>> = LazyLock::new(|| {
    [
        (
            "percent_obscuration_per_meter",
            &PERCENT_OBSCURATION_PER_METER,
        ),
        ("V/m", &VOLTS_PER_METER),
        ("kJ/h", &KILOJOULES_PER_HOUR),
        ("kJ/kg_dry", &KILOJOULES_PER_KILOGRAM_DRY_AIR),
        ("CAD", &CANADIAN_DOLLAR),
        ("MJ/ft²", &MEGAJOULES_PER_SQUARE_FOOT),
        (
            "watts_per_square_meter_irradiance",
            &WATTS_PER_SQUARE_METER_IRRADIANCE,
        ),
        ("short_ton", &SHORT_TON),
        ("inch", &INCH),
        ("in", &INCH),
        ("Kz", &KWANZA),
        ("%", &PERCENT),
        ("taka", &TAKA),
        ("volt_ampere", &VOLT_AMPERE),
        ("siemens", &SIEMENS),
        (
            "megajoules_per_kilogram_dry_air",
            &MEGAJOULES_PER_KILOGRAM_DRY_AIR,
        ),
        ("sri_lanka_rupee", &SRI_LANKA_RUPEE),
        ("kilometer", &KILOMETER),
        ("tenths_second", &TENTHS_SECOND),
        ("mm/min", &MILLIMETERS_PER_MINUTE),
        ("gallons_per_hour", &GALLONS_PER_HOUR),
        ("belize_dollar", &BELIZE_DOLLAR),
        ("seychelles_rupee", &SEYCHELLES_RUPEE),
        ("kJ/°K", &KILOJOULES_PER_DEGREE_KELVIN),
        ("BIF", &BURUNDI_FRANC),
        ("new_zealand_dollar", &NEW_ZEALAND_DOLLAR),
        ("psi/°F", &PSI_PER_DEGREE_FAHRENHEIT),
        ("kilograms_per_second", &KILOGRAMS_PER_SECOND),
        ("afghani", &AFGHANI),
        ("therm", &THERM),
        ("namibia_dollar", &NAMIBIA_DOLLAR),
        ("watt_hours_per_square_foot", &WATT_HOURS_PER_SQUARE_FOOT),
        ("CDF", &CONGOLESE_FRANC),
        ("BWP", &PULA),
        ("amperes_per_square_meter", &AMPERES_PER_SQUARE_METER),
        ("celsius_degrees", &CELSIUS_DEGREES),
        ("megajoule", &MEGAJOULE),
        ("ns", &NANOSECOND),
        ("azerbaijanian_manat", &AZERBAIJANIAN_MANAT),
        ("euro", &EURO),
        ("kilograms_per_hour", &KILOGRAMS_PER_HOUR),
        ("kB", &KILOBYTE),
        ("yd³", &CUBIC_YARD),
        ("kilowatts_per_square_foot", &KILOWATTS_PER_SQUARE_FOOT),
        ("moldavian_leu", &MOLDAVIAN_LEU),
        ("kΩ", &KILOHM),
        ("MB", &MEGABYTE),
        ("A/m²", &AMPERES_PER_SQUARE_METER),
        ("percent_relative_humidity", &PERCENT_RELATIVE_HUMIDITY),
        (
            "kilowatt_hours_per_square_meter",
            &KILOWATT_HOURS_PER_SQUARE_METER,
        ),
        ("HKD", &HONG_KONG_DOLLAR),
        (
            "percent_obscuration_per_foot",
            &PERCENT_OBSCURATION_PER_FOOT,
        ),
        ("PEN", &NUEVO_SOL),
        ("square_kilometer", &SQUARE_KILOMETER),
        ("ft/min", &FEET_PER_MINUTE),
        ("SCR", &SEYCHELLES_RUPEE),
        ("kilometers_per_second", &KILOMETERS_PER_SECOND),
        ("tonref", &TONS_REFRIGERATION),
        ("centum_cubic_feet", &CENTUM_CUBIC_FEET),
        ("ms", &MILLISECOND),
        ("volts_per_meter", &VOLTS_PER_METER),
        ("cfm", &CUBIC_FEET_PER_MINUTE),
        ("in²", &SQUARE_INCH),
        ("liters_per_minute", &LITERS_PER_MINUTE),
        ("ftlbs/sec", &FOOT_POUNDS_PER_SECOND),
        ("LKR", &SRI_LANKA_RUPEE),
        ("kilograms_per_square_meter", &KILOGRAMS_PER_SQUARE_METER),
        ("฿", &BAHT),
        ("siemens_per_meter", &SIEMENS_PER_METER),
        ("meters_per_second_squared", &METERS_PER_SECOND_SQUARED),
        ("nakfa", &NAKFA),
        ("gram", &GRAM),
        ("atm", &ATMOSPHERE),
        ("/h", &PER_HOUR),
        ("iranian_rial", &IRANIAN_RIAL),
        ("cph", &CYCLES_PER_HOUR),
        ("fahrenheit", &FAHRENHEIT),
        ("db_microvolt", &DB_MICROVOLT),
        ("foot", &FOOT),
        ("watt_hours_per_square_meter", &WATT_HOURS_PER_SQUARE_METER),
        ("°daysF", &DEGREE_DAYS_FAHRENHEIT),
        ("MK", &KWACHA),
        ("MW", &MEGAWATT),
        ("joules_per_kilogram", &JOULES_PER_KILOGRAM),
        ("S", &SIEMENS),
        ("per_minute", &PER_MINUTE),
        ("SGD", &SINGAPORE_DOLLAR),
        ("VEF", &BOLIVAR_FUERTE),
        ("J/kg°K", &JOULES_PER_KILOGRAM_DEGREE_KELVIN),
        ("kcfm", &KILOCUBIC_FEET_PER_MINUTE),
        ("VARh", &VOLT_AMPERE_REACTIVE_HOUR),
        (
            "watts_per_meter_degree_kelvin",
            &WATTS_PER_METER_DEGREE_KELVIN,
        ),
        ("db", &DECIBEL),
        ("Դ", &ARMENIAN_DRAM),
        ("GB", &GIGABYTE),
        ("KES", &KENYAN_SHILLING),
        ("pound_force", &POUND_FORCE),
        ("MΩ", &MEGOHM),
        ("AFN", &AFGHANI),
        ("K", &KELVIN),
        ("LAK", &KIP),
        ("petabyte", &PETABYTE),
        ("MJ/h", &MEGAJOULES_PER_HOUR),
        ("kilopound", &KILOPOUND),
        ("tugrik", &TUGRIK),
        ("JOD", &JORDANIAN_DINAR),
        ("rwanda_franc", &RWANDA_FRANC),
        ("guarani", &GUARANI),
        ("lebanese_pound", &LEBANESE_POUND),
        ("₦", &NAIRA),
        ("belarussian_ruble", &BELARUSSIAN_RUBLE),
        ("mo", &JULIAN_MONTH),
        ("ALL", &LEK),
        ("SOS", &SOMALI_SHILLING),
        (
            "thousand_cubic_feet_natural_gas",
            &THOUSAND_CUBIC_FEET_NATURAL_GAS,
        ),
        ("SFr", &SWISS_FRANC),
        ("centimeter", &CENTIMETER),
        ("mg/m³", &MILLIGRAMS_PER_CUBIC_METER),
        ("J/h", &JOULES_PER_HOUR),
        ("turkish_lira", &TURKISH_LIRA),
        ("VAR", &VOLT_AMPERE_REACTIVE),
        ("kilograms_per_liter", &KILOGRAMS_PER_LITER),
        ("Δ°F", &FAHRENHEIT_DEGREES),
        ("EGP", &EGYPTIAN_POUND),
        ("C$", &CORDOBA_ORO),
        ("NOK", &NORWEGIAN_KRONE),
        ("cd/ft²", &CANDELS_PER_SQUARE_FOOT),
        ("canadian_dollar", &CANADIAN_DOLLAR),
        ("L/h", &LITERS_PER_HOUR),
        ("centimeters_of_mercury", &CENTIMETERS_OF_MERCURY),
        ("BGN", &BULGARIAN_LEV),
        ("Br", &BELARUSSIAN_RUBLE),
        ("KGS", &SOM),
        ("ft³_gas", &CUBIC_FEET_NATURAL_GAS),
        ("hectopascal", &HECTOPASCAL),
        (
            "kilowatt_hours_per_cubic_meter",
            &KILOWATT_HOURS_PER_CUBIC_METER,
        ),
        ("fnu", &FORMAZIN_NEPHELOMETRIC_UNIT),
        ("joules_per_degree_kelvin", &JOULES_PER_DEGREE_KELVIN),
        ("LRD", &LIBERIAN_DOLLAR),
        (
            "watts_per_square_foot_irradiance",
            &WATTS_PER_SQUARE_FOOT_IRRADIANCE,
        ),
        ("nuevo_sol", &NUEVO_SOL),
        (
            "data_center_infrastructure_efficiency",
            &DATA_CENTER_INFRASTRUCTURE_EFFICIENCY,
        ),
        ("TND", &TUNISIAN_DINAR),
        ("PGK", &KINA),
        ("joules_per_square_meter", &JOULES_PER_SQUARE_METER),
        (
            "kilojoules_per_degree_kelvin",
            &KILOJOULES_PER_DEGREE_KELVIN,
        ),
        ("klb", &KILOPOUND),
        ("somoni", &SOMONI),
        ("৳", &TAKA),
        ("megajoules_per_square_foot", &MEGAJOULES_PER_SQUARE_FOOT),
        ("€", &EURO),
        ("kg/s", &KILOGRAMS_PER_SECOND),
        ("WST", &TALA),
        ("MJ", &MEGAJOULE),
        ("square_centimeter", &SQUARE_CENTIMETER),
        ("NIO", &CORDOBA_ORO),
        ("ZWL", &ZIMBABWE_DOLLAR),
        ("ohm", &OHM),
        ("in³", &CUBIC_INCH),
        ("per_second", &PER_SECOND),
        ("W/m³/s", &WATTS_PER_CUBIC_METERS_PER_SECOND),
        ("coefficient_of_performance", &COEFFICIENT_OF_PERFORMANCE),
        ("HNL", &LEMPIRA),
        ("radians_per_second_squared", &RADIANS_PER_SECOND_SQUARED),
        ("SZL", &LILANGENI),
        ("cubic_meters_per_minute", &CUBIC_METERS_PER_MINUTE),
        ("newton_meter", &NEWTON_METER),
        ("air_changes_per_hour", &AIR_CHANGES_PER_HOUR),
        ("kilopounds_per_hour", &KILOPOUNDS_PER_HOUR),
        ("AOA", &KWANZA),
        ("uae_dirham", &UAE_DIRHAM),
        ("SRD", &SURINAME_DOLLAR),
        ("kilobtu_per_square_foot", &KILOBTU_PER_SQUARE_FOOT),
        ("fiji_dollar", &FIJI_DOLLAR),
        ("ton", &METRIC_TON),
        ("kW/ft²", &KILOWATTS_PER_SQUARE_FOOT),
        ("GHS", &CEDI),
        ("moroccan_dirham", &MOROCCAN_DIRHAM),
        ("new_israeli_shekel", &NEW_ISRAELI_SHEKEL),
        ("east_caribbean_dollar", &EAST_CARIBBEAN_DOLLAR),
        ("FKP", &FALKLAND_ISLANDS_POUND),
        ("RM", &MALAYSIAN_RINGGIT),
        ("MTn", &METICAL),
        ("watts_per_square_foot", &WATTS_PER_SQUARE_FOOT),
        ("kW/kcfm", &KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE),
        ("atmosphere", &ATMOSPHERE),
        ("btus_per_hour", &BTUS_PER_HOUR),
        ("pounds_per_square_inch", &POUNDS_PER_SQUARE_INCH),
        ("fahrenheit_degrees", &FAHRENHEIT_DEGREES),
        ("saudi_riyal", &SAUDI_RIYAL),
        ("ppb", &PARTS_PER_BILLION),
        ("L/min", &LITERS_PER_MINUTE),
        ("leone", &LEONE),
        ("joule_second", &JOULE_SECOND),
        ("brunei_dollar", &BRUNEI_DOLLAR),
        ("danish_krone", &DANISH_KRONE),
        ("djibouti_franc", &DJIBOUTI_FRANC),
        ("pascal", &PASCAL),
        ("Rp", &RUPIAH),
        ("quetzal", &QUETZAL),
        ("CZK", &CZECH_KORUNA),
        ("calorie", &CALORIE),
        ("MVAR", &MEGAVOLT_AMPERE_REACTIVE),
        ("hr", &HOUR),
        ("Le", &LEONE),
        ("GW", &GIGAWATT),
        ("kJ", &KILOJOULE),
        ("MRO", &OUGUIYA),
        ("imperial_gallons_per_minute", &IMPERIAL_GALLONS_PER_MINUTE),
        ("dobra", &DOBRA),
        ("µm", &MICROMETER),
        ("ACH", &AIR_CHANGES_PER_HOUR),
        ("W/m²_irr", &WATTS_PER_SQUARE_METER_IRRADIANCE),
        ("denar", &DENAR),
        ("rial_omani", &RIAL_OMANI),
        ("kL", &KILOLITER),
        ("KRW", &SOUTH_KOREAN_WON),
        ("radians_per_second", &RADIANS_PER_SECOND),
        ("zimbabwe_dollar", &ZIMBABWE_DOLLAR),
        ("HUF", &FORINT),
        ("cordoba_oro", &CORDOBA_ORO),
        ("million_cubic_feet", &MILLION_CUBIC_FEET),
        ("lm", &LUMEN),
        ("parts_per_million", &PARTS_PER_MILLION),
        ("BTU", &BTU),
        ("minute", &MINUTE),
        ("SEK", &SWEDISH_KRONA),
        ("milliwatt", &MILLIWATT),
        ("degree_days_fahrenheit", &DEGREE_DAYS_FAHRENHEIT),
        ("cuban_peso", &CUBAN_PESO),
        ("TMT", &MANAT),
        ("kelvin_degrees", &KELVIN_DEGREES),
        ("microsecond", &MICROSECOND),
        ("BTU/ft²", &BTU_PER_SQUARE_FOOT),
        ("pound_sterling", &POUND_STERLING),
        ("m³", &CUBIC_METER),
        ("%obsc/m", &PERCENT_OBSCURATION_PER_METER),
        ("yen", &YEN),
        ("EUR", &EURO),
        ("som", &SOM),
        ("celsius", &CELSIUS),
        ("kg/m³", &KILOGRAMS_PER_CUBIC_METER),
        ("MWh", &MEGAWATT_HOUR),
        ("元", &CHINESE_YUAN),
        ("MMBTU/h", &MEGABTUS_PER_HOUR),
        ("DOP", &DOMINICAN_PESO),
        ("mΩ", &MILLIOHM),
        ("PHP", &PHILIPPINE_PESO),
        ("kVA", &KILOVOLT_AMPERE),
        ("MBTU", &MEGABTU),
        ("saint_helena_pound", &SAINT_HELENA_POUND),
        ("BMD", &BERMUDIAN_DOLLAR),
        ("algerian_dinar", &ALGERIAN_DINAR),
        ("₱", &PHILIPPINE_PESO),
        ("farad", &FARAD),
        ("MHz", &MEGAHERTZ),
        ("pixel", &PIXEL),
        ("UM", &OUGUIYA),
        ("J/g", &JOULES_PER_GRAM),
        ("bahraini_dinar", &BAHRAINI_DINAR),
        ("kBTU/h", &KILOBTUS_PER_HOUR),
        ("g/m²", &GRAMS_PER_SQUARE_METER),
        ("ман", &AZERBAIJANIAN_MANAT),
        ("volts_per_degree_kelvin", &VOLTS_PER_DEGREE_KELVIN),
        ("ETB", &ETHIOPIAN_BIRR),
        ("ლ", &LARI),
        ("mm³", &CUBIC_MILLIMETER),
        ("Kn", &CROATIAN_KUNA),
        ("s", &SECOND),
        ("ЅМ", &SOMONI),
        ("ouguiya", &OUGUIYA),
        ("cfa_franc_bceao", &CFA_FRANC_BCEAO),
        ("¥", &YEN),
        ("ft³", &CUBIC_FOOT),
        ("₲", &GUARANI),
        ("UZS", &UZBEKISTAN_SUM),
        ("GBP", &POUND_STERLING),
        ("watt_hour", &WATT_HOUR),
        ("m³/min", &CUBIC_METERS_PER_MINUTE),
        (
            "kilovolt_ampere_reactive_hour",
            &KILOVOLT_AMPERE_REACTIVE_HOUR,
        ),
        ("GJ/h", &GIGAJOULES_PER_HOUR),
        ("Btu/Wh", &ENERGY_EFFICIENCY_RATIO),
        ("psi_per_degree_fahrenheit", &PSI_PER_DEGREE_FAHRENHEIT),
        ("m/s", &METERS_PER_SECOND),
        ("pounds_per_hour", &POUNDS_PER_HOUR),
        (
            "watts_per_cubic_feet_per_minute",
            &WATTS_PER_CUBIC_FEET_PER_MINUTE,
        ),
        ("HRK", &CROATIAN_KUNA),
        ("degrees_fahrenheit_per_hour", &DEGREES_FAHRENHEIT_PER_HOUR),
        ("Js", &JOULE_SECOND),
        ("K/s", &DEGREES_KELVIN_PER_SECOND),
        ("Hcf", &HUNDRED_CUBIC_FEET),
        ("gigawatt", &GIGAWATT),
        ("kilowatts_per_square_meter", &KILOWATTS_PER_SQUARE_METER),
        ("scm", &CUBIC_METERS_NATURAL_GAS),
        ("kilowatt_hour", &KILOWATT_HOUR),
        ("day", &DAY),
        ("V", &VOLT),
        ("AZN", &AZERBAIJANIAN_MANAT),
        ("inches_per_hour", &INCHES_PER_HOUR),
        ("megawatt_hour", &MEGAWATT_HOUR),
        ("coulomb", &COULOMB),
        ("m", &METER),
        ("klb/h", &KILOPOUNDS_PER_HOUR),
        ("KHR", &RIEL),
        ("aruban_guilder_florin", &ARUBAN_GUILDER_FLORIN),
        ("gph", &GALLONS_PER_HOUR),
        ("manat", &MANAT),
        ("baht", &BAHT),
        ("MMcf", &MILLION_CUBIC_FEET),
        ("pula", &PULA),
        ("TWD", &TAIWAN_DOLLAR),
        ("m/min", &METERS_PER_MINUTE),
        ("tenge", &TENGE),
        ("ZMW", &ZAMBIAN_KWACHA),
        ("RWF", &RWANDA_FRANC),
        (
            "kilowatt_hours_per_square_foot",
            &KILOWATT_HOURS_PER_SQUARE_FOOT,
        ),
        ("MUR", &MAURITIUS_RUPEE),
        ("DZD", &ALGERIAN_DINAR),
        ("MBTU/h", &MEGABTUS_PER_HOUR),
        ("second", &SECOND),
        ("XAF", &CFA_FRANC_BCEAO),
        ("KWD", &KUWAITI_DINAR),
        ("db_millivolt", &DB_MILLIVOLT),
        ("bolivar_fuerte", &BOLIVAR_FUERTE),
        ("Ccf_natural_gas", &CENTUM_CUBIC_FEET_NATURAL_GAS),
        ("degrees_celsius_per_hour", &DEGREES_CELSIUS_PER_HOUR),
        ("RON", &LEU),
        ("vatu", &VATU),
        ("watt", &WATT),
        ("inH₂O", &INCHES_OF_WATER),
        ("imperial_gallon", &IMPERIAL_GALLON),
        ("konvertibilna_marka", &KONVERTIBILNA_MARKA),
        ("Ωm", &OHM_METER),
        ("milligrams_per_liter", &MILLIGRAMS_PER_LITER),
        ("kilojoules_per_kilogram", &KILOJOULES_PER_KILOGRAM),
        ("cayman_islands_dollar", &CAYMAN_ISLANDS_DOLLAR),
        ("CRC", &COSTA_RICAN_COLON),
        ("K/min", &DEGREES_KELVIN_PER_MINUTE),
        ("henry", &HENRY),
        ("grams_per_second", &GRAMS_PER_SECOND),
        ("steradian", &STERADIAN),
        ("riel", &RIEL),
        ("feet_per_second", &FEET_PER_SECOND),
        ("₤", &TURKISH_LIRA),
        ("ft²", &SQUARE_FOOT),
        ("DKK", &DANISH_KRONE),
        ("millibar", &MILLIBAR),
        ("kΩ", &KILOHM),
        ("mg", &MILLIGRAM),
        ("Kč", &CZECH_KORUNA),
        ("boliviano", &BOLIVIANO),
        ("square_foot", &SQUARE_FOOT),
        (
            "kilowatts_per_kilocubic_feet_per_minute",
            &KILOWATTS_PER_KILOCUBIC_FEET_PER_MINUTE,
        ),
        ("CLP", &CHILEAN_PESO),
        ("CNY", &CHINESE_YUAN),
        ("hundred_cubic_feet", &HUNDRED_CUBIC_FEET),
        ("ph", &PH),
        ("gal/min", &GALLONS_PER_MINUTE),
        ("candelas_per_square_meter", &CANDELAS_PER_SQUARE_METER),
        ("egyptian_pound", &EGYPTIAN_POUND),
        ("MWK", &KWACHA),
        ("ppm", &PARTS_PER_MILLION),
        ("metric_tons_per_hour", &METRIC_TONS_PER_HOUR),
        ("S/m", &SIEMENS_PER_METER),
        ("gal", &GALLON),
        ("kilovolt_ampere_hour", &KILOVOLT_AMPERE_HOUR),
        ("gigajoule", &GIGAJOULE),
        ("degrees_celsius_per_minute", &DEGREES_CELSIUS_PER_MINUTE),
        ("MVR", &RUFIYAA),
        ("MMcf_natural_gas", &MILLION_CUBIC_FEET_NATURAL_GAS),
        ("btu/lb_dry", &BTUS_PER_POUND_DRY_AIR),
        ("tunisian_dinar", &TUNISIAN_DINAR),
        ("KPW", &NORTH_KOREAN_WON),
        ("MV", &MEGAVOLT),
        ("indian_rupee", &INDIAN_RUPEE),
        ("hundredths_second", &HUNDREDTHS_SECOND),
        ("percent", &PERCENT),
        ("CHF", &SWISS_FRANC),
        ("gal/hr", &GALLONS_PER_HOUR),
        ("kr", &DANISH_KRONE),
        ("Nfk", &NAKFA),
        ("PYG", &GUARANI),
        ("millimeter", &MILLIMETER),
        ("g", &GRAM),
        (
            "megajoules_per_degree_kelvin",
            &MEGAJOULES_PER_DEGREE_KELVIN,
        ),
        (
            "kilobtus_per_hour_per_square_foot",
            &KILOBTUS_PER_HOUR_PER_SQUARE_FOOT,
        ),
        ("R$", &BRAZILIAN_REAL),
        ("guinea_franc", &GUINEA_FRANC),
        ("lb/h", &POUNDS_PER_HOUR),
        ("millimeters_per_second", &MILLIMETERS_PER_SECOND),
        ("cubic_meters_per_second", &CUBIC_METERS_PER_SECOND),
        ("pint", &PINT),
        ("leu", &LEU),
        ("BBD", &BARBADOS_DOLLAR),
        ("BYR", &BELARUSSIAN_RUBLE),
        ("norwegian_krone", &NORWEGIAN_KRONE),
        ("hph", &HORSEPOWER_HOUR),
        ("cm", &CENTIMETER),
        ("kVAR", &KILOVOLT_AMPERE_REACTIVE),
        ("cubic_inch", &CUBIC_INCH),
        ("MΩ", &MEGOHM),
        ("lbf", &POUND_FORCE),
        ("lek", &LEK),
        ("cedi", &CEDI),
        ("INR", &INDIAN_RUPEE),
        ("lempira", &LEMPIRA),
        ("SAR", &SAUDI_RIYAL),
        ("square_yard", &SQUARE_YARD),
        ("square_inch", &SQUARE_INCH),
        ("kJ/kg", &KILOJOULES_PER_KILOGRAM),
        ("deg", &DEGREES_ANGULAR),
        ("MJ/m²", &MEGAJOULES_PER_SQUARE_METER),
        ("btus_per_pound_dry_air", &BTUS_PER_POUND_DRY_AIR),
        ("LSL", &LOTI),
        ("kwacha", &KWACHA),
        ("pt", &PINT),
        ("kWh/ft²", &KILOWATT_HOURS_PER_SQUARE_FOOT),
        ("energy_efficiency_ratio", &ENERGY_EFFICIENCY_RATIO),
        ("kilohertz", &KILOHERTZ),
        ("MWh/ft²", &MEGAWATT_HOURS_PER_SQUARE_FOOT),
        ("cubic_foot", &CUBIC_FOOT),
        ("square_mile", &SQUARE_MILE),
        ("metical", &METICAL),
        ("YER", &YEMENI_RIAL),
        ("megavolt", &MEGAVOLT),
        ("m³/s", &CUBIC_METERS_PER_SECOND),
        ("g/m³", &GRAMS_PER_CUBIC_METER),
        ("cubic_meters_per_hour", &CUBIC_METERS_PER_HOUR),
        ("byte", &BYTE),
        ("ISK", &ICELAND_KRONA),
        ("kBTU/ft²", &KILOBTU_PER_SQUARE_FOOT),
        ("N", &NEWTON),
        ("grams_per_kilogram", &GRAMS_PER_KILOGRAM),
        ("sr", &STERADIAN),
        ("feet_per_minute", &FEET_PER_MINUTE),
        ("₵", &CEDI),
        ("W/ft²", &WATTS_PER_SQUARE_FOOT),
        ("peso_uruguayo", &PESO_URUGUAYO),
        ("solomon_islands_dollar", &SOLOMON_ISLANDS_DOLLAR),
        ("jamaican_dollar", &JAMAICAN_DOLLAR),
        ("L/s", &LITERS_PER_SECOND),
        ("µg/m³", &MICROGRAMS_PER_CUBIC_METER),
        ("joules_per_kilogram_dry_air", &JOULES_PER_KILOGRAM_DRY_AIR),
        ("grams_per_square_meter", &GRAMS_PER_SQUARE_METER),
        ("ΔK", &KELVIN_DEGREES),
        ("kilojoule", &KILOJOULE),
        ("Rs", &SRI_LANKA_RUPEE),
        ("foot_pounds_per_second", &FOOT_POUNDS_PER_SECOND),
        ("₨", &PAKISTAN_RUPEE),
        ("megahertz", &MEGAHERTZ),
        (
            "kilowatts_per_gallons_per_minute",
            &KILOWATTS_PER_GALLONS_PER_MINUTE,
        ),
        ("PUE", &POWER_USAGE_EFFECTIVENESS),
        ("DCIE", &DATA_CENTER_INFRASTRUCTURE_EFFICIENCY),
        ("kg/min", &KILOGRAMS_PER_MINUTE),
        ("tons_refrigeration_hour", &TONS_REFRIGERATION_HOUR),
        ("kilowatt_per_ton", &KILOWATT_PER_TON),
        ("Ωm", &OHM_METER),
        ("scf", &CUBIC_FEET_NATURAL_GAS),
        ("лв", &BULGARIAN_LEV),
        ("UAH", &HRYVNIA),
        ("C", &COULOMB),
        ("malaysian_ringgit", &MALAYSIAN_RINGGIT),
        ("₸", &TENGE),
        ("degrees_kelvin_per_hour", &DEGREES_KELVIN_PER_HOUR),
        ("therms_per_hour", &THERMS_PER_HOUR),
        ("cfm/ft²", &CUBIC_FEET_PER_MINUTE_PER_SQUARE_FOOT),
        ("dBmV", &DB_MILLIVOLT),
        ("TRY", &TURKISH_LIRA),
        ("pf", &POWER_FACTOR),
        ("volt_ampere_hour", &VOLT_AMPERE_HOUR),
        ("barbados_dollar", &BARBADOS_DOLLAR),
        ("lilangeni", &LILANGENI),
        ("kelvin", &KELVIN),
        ("newtons_per_meter", &NEWTONS_PER_METER),
        ("KYD", &CAYMAN_ISLANDS_DOLLAR),
        ("BND", &BRUNEI_DOLLAR),
        ("quart", &QUART),
        ("meters_per_minute", &METERS_PER_MINUTE),
        ("yr", &YEAR),
        ("ft", &FOOT),
        ("m/s²", &METERS_PER_SECOND_SQUARED),
        ("ethiopian_birr", &ETHIOPIAN_BIRR),
        ("degrees_kelvin_per_second", &DEGREES_KELVIN_PER_SECOND),
        ("Wh", &WATT_HOUR),
        ("km/h", &KILOMETERS_PER_HOUR),
        ("pakistan_rupee", &PAKISTAN_RUPEE),
        ("kVARh", &KILOVOLT_AMPERE_REACTIVE_HOUR),
        ("czech_koruna", &CZECH_KORUNA),
        ("m³_gas", &CUBIC_METERS_NATURAL_GAS),
        ("ton/h", &METRIC_TONS_PER_HOUR),
        ("degrees_phase", &DEGREES_PHASE),
        ("₪", &NEW_ISRAELI_SHEKEL),
        ("hPa", &HECTOPASCAL),
        ("g/s", &GRAMS_PER_SECOND),
        ("GEL", &LARI),
        ("somali_shilling", &SOMALI_SHILLING),
        ("QAR", &QATARI_RIAL),
        ("kilovolt", &KILOVOLT),
        ("gigabyte", &GIGABYTE),
        ("parts_per_billion", &PARTS_PER_BILLION),
        ("brazilian_real", &BRAZILIAN_REAL),
        ("USD", &US_DOLLAR),
        ("kWh/m³", &KILOWATT_HOURS_PER_CUBIC_METER),
        ("JPY", &YEN),
        ("BDT", &TAKA),
        ("kwanza", &KWANZA),
        ("acre", &ACRE),
        ("cubic_meters_natural_gas", &CUBIC_METERS_NATURAL_GAS),
        ("lx", &LUX),
        ("btu_per_pound", &BTU_PER_POUND),
        ("lari", &LARI),
        ("hp", &HORSEPOWER),
        ("cfs", &CUBIC_FEET_PER_SECOND),
        ("PAB", &BALBOA),
        ("qt", &QUART),
        ("K/h", &DEGREES_KELVIN_PER_HOUR),
        ("centimeters_of_water", &CENTIMETERS_OF_WATER),
        ("pzloty", &PZLOTY),
        ("square_meters_per_newton", &SQUARE_METERS_PER_NEWTON),
        ("cal", &CALORIE),
        ("MOP", &PATACA),
        ("inHg", &INCHES_OF_MERCURY),
        ("suriname_dollar", &SURINAME_DOLLAR),
        ("cubic_meter", &CUBIC_METER),
        ("MMBTU", &MEGABTU),
        ("Mcf_natural_gas", &THOUSAND_CUBIC_FEET_NATURAL_GAS),
        ("hertz", &HERTZ),
        ("cubic_yard", &CUBIC_YARD),
        ("AWG", &ARUBAN_GUILDER_FLORIN),
        ("rad", &RADIAN),
        ("cape_verde_escudo", &CAPE_VERDE_ESCUDO),
        ("₹", &INDIAN_RUPEE),
        ("kW/m²", &KILOWATTS_PER_SQUARE_METER),
        ("lb", &POUND),
        ("VAh", &VOLT_AMPERE_HOUR),
        ("din", &SERBIAN_DINAR),
        ("MBTU/ft²", &MEGABTU_PER_SQUARE_FOOT),
        ("TZS", &TANZANIAN_SHILLING),
        ("mbar", &MILLIBAR),
        ("ppu", &PARTS_PER_UNIT),
        ("cd", &CANDELA),
        ("bar", &BAR),
        ("JMD", &JAMAICAN_DOLLAR),
        ("GWh", &GIGAWATT_HOUR),
        ("kilobtus_per_hour", &KILOBTUS_PER_HOUR),
        ("balboa", &BALBOA),
        ("cycles_per_hour", &CYCLES_PER_HOUR),
        ("burundi_franc", &BURUNDI_FRANC),
        ("AED", &UAE_DIRHAM),
        ("calorie_per_gram", &CALORIE_PER_GRAM),
        ("mauritius_rupee", &MAURITIUS_RUPEE),
        ("Ω", &OHM),
        ("newton_second", &NEWTON_SECOND),
        ("megohm", &MEGOHM),
        ("Ccf", &CENTUM_CUBIC_FEET),
        ("liters_per_hour", &LITERS_PER_HOUR),
        ("km²", &SQUARE_KILOMETER),
        ("Db", &DOBRA),
        ("NAD", &NAMIBIA_DOLLAR),
        ("rpm", &REVOLUTIONS_PER_MINUTE),
        ("mm²", &SQUARE_MILLIMETER),
        ("rad/s²", &RADIANS_PER_SECOND_SQUARED),
        ("Ω", &OHM),
        ("GNF", &GUINEA_FRANC),
        ("liter", &LITER),
        ("h", &HOUR),
        ("₩", &SOUTH_KOREAN_WON),
        ("candels_per_square_foot", &CANDELS_PER_SQUARE_FOOT),
        ("sudanese_pound", &SUDANESE_POUND),
        ("cycles_per_minute", &CYCLES_PER_MINUTE),
        ("degPh", &DEGREES_PHASE),
        ("milliohm", &MILLIOHM),
        ("MJ/kg_dry", &MEGAJOULES_PER_KILOGRAM_DRY_AIR),
        ("ds", &TENTHS_SECOND),
        ("UYU", &PESO_URUGUAYO),
        (
            "grams_of_water_per_kilogram_dry_air",
            &GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR,
        ),
        ("ohm_meter", &OHM_METER),
        ("milliliters_per_second", &MILLILITERS_PER_SECOND),
        ("Volt", &VOLT),
        (
            "degrees_fahrenheit_per_minute",
            &DEGREES_FAHRENHEIT_PER_MINUTE,
        ),
        ("m²/N", &SQUARE_METERS_PER_NEWTON),
        ("kilometers_per_hour", &KILOMETERS_PER_HOUR),
        ("mV", &MILLIVOLT),
        ("congolese_franc", &CONGOLESE_FRANC),
        ("Sh", &KENYAN_SHILLING),
        ("horsepower", &HORSEPOWER),
        ("mm/s", &MILLIMETERS_PER_SECOND),
        ("chinese_yuan", &CHINESE_YUAN),
        ("GYD", &GUYANA_DOLLAR),
        ("RUB", &RUSSIAN_RUBLE),
        ("inches_of_mercury", &INCHES_OF_MERCURY),
        ("L", &LITER),
        ("DJF", &DJIBOUTI_FRANC),
        (
            "megawatt_hours_per_square_foot",
            &MEGAWATT_HOURS_PER_SQUARE_FOOT,
        ),
        ("meters_per_second", &METERS_PER_SECOND),
        ("GJ", &GIGAJOULE),
        ("Kr", &ICELAND_KRONA),
        ("g/min", &GRAMS_PER_MINUTE),
        ("kBTU", &KILOBTU),
        ("J/°K", &JOULES_PER_DEGREE_KELVIN),
        ("candela", &CANDELA),
        ("milligrams_per_cubic_meter", &MILLIGRAMS_PER_CUBIC_METER),
        ("gibraltar_pound", &GIBRALTAR_POUND),
        ("singapore_dollar", &SINGAPORE_DOLLAR),
        ("joules_per_cubic_meter", &JOULES_PER_CUBIC_METER),
        ("V/K", &VOLTS_PER_DEGREE_KELVIN),
        ("ƒ", &ARUBAN_GUILDER_FLORIN),
        ("kilograms_per_minute", &KILOGRAMS_PER_MINUTE),
        ("AUD", &AUSTRALIAN_DOLLAR),
        ("BTU/lb", &BTU_PER_POUND),
        (
            "million_cubic_feet_natural_gas",
            &MILLION_CUBIC_FEET_NATURAL_GAS,
        ),
        ("cubic_centimeter", &CUBIC_CENTIMETER),
        ("kWh", &KILOWATT_HOUR),
        ("swiss_franc", &SWISS_FRANC),
        ("watts_per_square_meter", &WATTS_PER_SQUARE_METER),
        ("kuwaiti_dinar", &KUWAITI_DINAR),
        ("megajoules_per_square_meter", &MEGAJOULES_PER_SQUARE_METER),
        ("STD", &DOBRA),
        ("micrometer", &MICROMETER),
        ("cmH₂O", &CENTIMETERS_OF_WATER),
        ("kW", &KILOWATT),
        ("mL", &MILLILITER),
        ("ntu", &NEPHELOMETRIC_TURBIDITY_UNITS),
        ("SLL", &LEONE),
        ("gallons_per_minute", &GALLONS_PER_MINUTE),
        ("yd", &YARD),
        ("MKD", &DENAR),
        ("degrees_kelvin_per_minute", &DEGREES_KELVIN_PER_MINUTE),
        ("millimeters_per_hour", &MILLIMETERS_PER_HOUR),
        (
            "megavolt_ampere_reactive_hour",
            &MEGAVOLT_AMPERE_REACTIVE_HOUR,
        ),
        ("H", &HENRY),
        ("NGN", &NAIRA),
        ("pounds_per_second", &POUNDS_PER_SECOND),
        ("kg/m²", &KILOGRAMS_PER_SQUARE_METER),
        ("m²", &SQUARE_METER),
        ("nepalese_rupee", &NEPALESE_RUPEE),
        ("Am²", &AMPERE_SQUARE_METER),
        (
            "watts_per_cubic_meters_per_second",
            &WATTS_PER_CUBIC_METERS_PER_SECOND,
        ),
        ("lb/s", &POUNDS_PER_SECOND),
        ("TB", &TERABYTE),
        ("₡", &COSTA_RICAN_COLON),
        ("GMD", &DALASI),
        ("falkland_islands_pound", &FALKLAND_ISLANDS_POUND),
        ("therm/h", &THERMS_PER_HOUR),
        ("ampere_hour", &AMPERE_HOUR),
        ("terabyte", &TERABYTE),
        ("joules_per_gram", &JOULES_PER_GRAM),
        ("MMK", &KYAT),
        ("₭", &KIP),
        ("lux", &LUX),
        ("ounce", &OUNCE),
        ("oz", &OUNCE),
        ("argentine_peso", &ARGENTINE_PESO),
        ("cpm", &CYCLES_PER_MINUTE),
        ("mA", &MILLIAMPERE),
        ("W/m°K", &WATTS_PER_METER_DEGREE_KELVIN),
        ("gigawatt_hour", &GIGAWATT_HOUR),
        ("mm/h", &MILLIMETERS_PER_HOUR),
        ("meter", &METER),
        ("tala", &TALA),
        ("N/m", &NEWTONS_PER_METER),
        ("THB", &BAHT),
        ("uzbekistan_sum", &UZBEKISTAN_SUM),
        ("A/m", &AMPERES_PER_METER),
        ("kilogallon", &KILOGALLON),
        ("kiloliter", &KILOLITER),
        ("megabyte", &MEGABYTE),
        ("sec", &SECOND),
        ("VA", &VOLT_AMPERE),
        ("MVAh", &MEGAVOLT_AMPERE_HOUR),
        ("PKR", &PAKISTAN_RUPEE),
        ("m/h", &METERS_PER_HOUR),
        ("grams_per_cubic_meter", &GRAMS_PER_CUBIC_METER),
        ("nanosecond", &NANOSECOND),
        ("mile²", &SQUARE_MILE),
        ("SHP", &SAINT_HELENA_POUND),
        ("degree_days_celsius", &DEGREE_DAYS_CELSIUS),
        ("decibel", &DECIBEL),
        ("Hz", &HERTZ),
        ("costa_rican_colon", &COSTA_RICAN_COLON),
        ("kilocubic_feet_per_minute", &KILOCUBIC_FEET_PER_MINUTE),
        ("TJS", &SOMONI),
        ("VUV", &VATU),
        ("GJ/m³", &GIGAJOULES_PER_CUBIC_METER),
        ("pataca", &PATACA),
        ("COP", &COEFFICIENT_OF_PERFORMANCE),
        ("%obsc/ft", &PERCENT_OBSCURATION_PER_FOOT),
        ("kilowatt", &KILOWATT),
        ("LBP", &LEBANESE_POUND),
        ("HTG", &GOURDE),
        ("BTN", &NGULTRUM),
        ("galUK", &IMPERIAL_GALLON),
        ("millivolt", &MILLIVOLT),
        ("kW/ton", &KILOWATT_PER_TON),
        ("pH", &PH),
        ("dong", &DONG),
        ("ILS", &NEW_ISRAELI_SHEKEL),
        ("kVAh", &KILOVOLT_AMPERE_HOUR),
        ("russian_ruble", &RUSSIAN_RUBLE),
        ("swedish_krona", &SWEDISH_KRONA),
        ("px", &PIXEL),
        ("croatian_kuna", &CROATIAN_KUNA),
        ("weber", &WEBER),
        ("megavolt_ampere_hour", &MEGAVOLT_AMPERE_HOUR),
        ("AMD", &ARMENIAN_DRAM),
        ("in/wc", &INCHES_OF_WATER),
        ("cubic_feet_per_minute", &CUBIC_FEET_PER_MINUTE),
        ("°C/min", &DEGREES_CELSIUS_PER_MINUTE),
        ("W/ft²_irr", &WATTS_PER_SQUARE_FOOT_IRRADIANCE),
        ("Mcf", &THOUSAND_CUBIC_FEET),
        ("MYR", &MALAYSIAN_RINGGIT),
        ("dalasi", &DALASI),
        ("ERN", &NAKFA),
        ("XCD", &EAST_CARIBBEAN_DOLLAR),
        ("standard_cubic_foot", &CUBIC_FEET_NATURAL_GAS),
        ("cm³", &CUBIC_CENTIMETER),
        ("$", &US_DOLLAR),
        ("qatari_rial", &QATARI_RIAL),
        ("min", &MINUTE),
        ("psi", &POUNDS_PER_SQUARE_INCH),
        ("hectoliters_per_second", &HECTOLITERS_PER_SECOND),
        ("bahamian_dollar", &BAHAMIAN_DOLLAR),
        ("kilobyte", &KILOBYTE),
        ("mW", &MILLIWATT),
        ("rufiyaa", &RUFIYAA),
        ("gH₂O/kgAir", &GRAMS_OF_WATER_PER_KILOGRAM_DRY_AIR),
        ("kilobtu", &KILOBTU),
        ("BOB", &BOLIVIANO),
        ("F", &FARAD),
        ("inches_of_water", &INCHES_OF_WATER),
        ("jordanian_dinar", &JORDANIAN_DINAR),
        ("°C", &CELSIUS),
        ("btu", &BTU),
        ("mg/L", &MILLIGRAMS_PER_LITER),
        (
            "centum_cubic_feet_natural_gas",
            &CENTUM_CUBIC_FEET_NATURAL_GAS,
        ),
        ("Ns", &NEWTON_SECOND),
        ("ngultrum", &NGULTRUM),
        (
            "watts_per_square_meter_degree_kelvin",
            &WATTS_PER_SQUARE_METER_DEGREE_KELVIN,
        ),
        ("parts_per_unit", &PARTS_PER_UNIT),
        ("rupiah", &RUPIAH),
        ("/s", &PER_SECOND),
        ("IQD", &IRAQI_DINAR),
        ("NZD", &NEW_ZEALAND_DOLLAR),
        ("/min", &PER_MINUTE),
        ("iraqi_dinar", &IRAQI_DINAR),
        ("rand", &RAND),
        ("kW/gal/min", &KILOWATTS_PER_GALLONS_PER_MINUTE),
        ("loti", &LOTI),
        ("formazin_nephelometric_unit", &FORMAZIN_NEPHELOMETRIC_UNIT),
        ("kg/L", &KILOGRAMS_PER_LITER),
        ("uganda_shilling", &UGANDA_SHILLING),
        ("°F", &FAHRENHEIT),
        ("fl_oz", &FLUID_OUNCE),
        ("T", &TESLA),
        ("guyana_dollar", &GUYANA_DOLLAR),
        ("cfp_franc", &CFP_FRANC),
        ("kilograms_per_cubic_meter", &KILOGRAMS_PER_CUBIC_METER),
        ("J", &JOULE),
        ("square_meter", &SQUARE_METER),
        ("megajoules_per_hour", &MEGAJOULES_PER_HOUR),
        ("MVARh", &MEGAVOLT_AMPERE_REACTIVE_HOUR),
        ("megavolt_ampere_reactive", &MEGAVOLT_AMPERE_REACTIVE),
        ("galUK/min", &IMPERIAL_GALLONS_PER_MINUTE),
        ("megabtu_per_square_foot", &MEGABTU_PER_SQUARE_FOOT),
        ("kgal", &KILOGALLON),
        ("OMR", &RIAL_OMANI),
        ("volt_ampere_reactive", &VOLT_AMPERE_REACTIVE),
        ("Hcf_natural_gas", &HUNDRED_CUBIC_FEET_NATURAL_GAS),
        ("philippine_peso", &PHILIPPINE_PESO),
        ("Wh/m²", &WATT_HOURS_PER_SQUARE_METER),
        ("kina", &KINA),
        ("W", &WATT),
        ("hour", &HOUR),
        ("dominican_peso", &DOMINICAN_PESO),
        ("fluid_ounce", &FLUID_OUNCE),
        ("%/s", &PERCENT_PER_SECOND),
        ("kPa", &KILOPASCAL),
        ("VND", &DONG),
        ("mexican_peso", &MEXICAN_PESO),
        ("syrian_pound", &SYRIAN_POUND),
        ("SYP", &SYRIAN_POUND),
        ("pound", &POUND),
        ("amperes_per_meter", &AMPERES_PER_METER),
        ("rad/s", &RADIANS_PER_SECOND),
        ("megawatt", &MEGAWATT),
        ("°daysC", &DEGREE_DAYS_CELSIUS),
        ("newton", &NEWTON),
        ("kHz", &KILOHERTZ),
        ("GIP", &GIBRALTAR_POUND),
        ("week", &WEEK),
        ("tons_refrigeration", &TONS_REFRIGERATION),
        (
            "kilojoules_per_kilogram_dry_air",
            &KILOJOULES_PER_KILOGRAM_DRY_AIR,
        ),
        ("BTU/h", &BTUS_PER_HOUR),
        ("ZK", &ZAMBIAN_KWACHA),
        ("kilopascal", &KILOPASCAL),
        ("btu_per_square_foot", &BTU_PER_SQUARE_FOOT),
        ("GTQ", &QUETZAL),
        ("cmHg", &CENTIMETERS_OF_MERCURY),
        ("revolutions_per_minute", &REVOLUTIONS_PER_MINUTE),
        ("ampere", &AMPERE),
        ("liters_per_second", &LITERS_PER_SECOND),
        ("₮", &TUGRIK),
        ("us_dollar", &US_DOLLAR),
        ("LYD", &LIBYAN_DINAR),
        ("ден", &DENAR),
        ("yd²", &SQUARE_YARD),
        ("m³/h", &CUBIC_METERS_PER_HOUR),
        ("MWh/m²", &MEGAWATT_HOURS_PER_SQUARE_METER),
        ("BSD", &BAHAMIAN_DOLLAR),
        ("°C/h", &DEGREES_CELSIUS_PER_HOUR),
        ("volt_ampere_reactive_hour", &VOLT_AMPERE_REACTIVE_HOUR),
        ("mL/s", &MILLILITERS_PER_SECOND),
        ("metric_ton", &METRIC_TON),
        ("BAM", &KONVERTIBILNA_MARKA),
        ("micrograms_per_cubic_meter", &MICROGRAMS_PER_CUBIC_METER),
        ("%RH", &PERCENT_RELATIVE_HUMIDITY),
        ("KZT", &TENGE),
        ("Vt", &VATU),
        ("J/m²", &JOULES_PER_SQUARE_METER),
        ("mΩ", &MILLIOHM),
        ("MAD", &MOROCCAN_DIRHAM),
        ("cs", &HUNDREDTHS_SECOND),
        ("CVE", &CAPE_VERDE_ESCUDO),
        ("iceland_krona", &ICELAND_KRONA),
        ("Wb", &WEBER),
        ("km/s", &KILOMETERS_PER_SECOND),
        ("power_usage_effectiveness", &POWER_USAGE_EFFECTIVENESS),
        ("in/h", &INCHES_PER_HOUR),
        ("pounds_per_minute", &POUNDS_PER_MINUTE),
        ("°F/h", &DEGREES_FAHRENHEIT_PER_HOUR),
        ("FJD", &FIJI_DOLLAR),
        ("PLN", &PZLOTY),
        ("south_korean_won", &SOUTH_KOREAN_WON),
        ("g/kg", &GRAMS_PER_KILOGRAM),
        ("trinidad_and_tobago_dollar", &TRINIDAD_AND_TOBAGO_DOLLAR),
        ("milliampere", &MILLIAMPERE),
        ("power_factor", &POWER_FACTOR),
        ("megabtu", &MEGABTU),
        ("kg/h", &KILOGRAMS_PER_HOUR),
        ("millisecond", &MILLISECOND),
        ("Af", &AFGHANI),
        ("J/kg", &JOULES_PER_KILOGRAM),
        ("hong_kong_dollar", &HONG_KONG_DOLLAR),
        ("cubic_feet_natural_gas", &CUBIC_FEET_NATURAL_GAS),
        ("gigajoules_per_cubic_meter", &GIGAJOULES_PER_CUBIC_METER),
        ("megabtus_per_hour", &MEGABTUS_PER_HOUR),
        ("КМ", &KONVERTIBILNA_MARKA),
        ("chilean_peso", &CHILEAN_PESO),
        (
            "megawatt_hours_per_square_meter",
            &MEGAWATT_HOURS_PER_SQUARE_METER,
        ),
        ("grams_per_minute", &GRAMS_PER_MINUTE),
        ("₫", &DONG),
        ("serbian_dinar", &SERBIAN_DINAR),
        ("gallon", &GALLON),
        ("lumen", &LUMEN),
        ("horsepower_hour", &HORSEPOWER_HOUR),
        ("W/m²K", &WATTS_PER_SQUARE_METER_DEGREE_KELVIN),
        ("kip", &KIP),
        ("meters_per_hour", &METERS_PER_HOUR),
        ("thousand_cubic_feet", &THOUSAND_CUBIC_FEET),
        ("hL/s", &HECTOLITERS_PER_SECOND),
        ("XPF", &CFP_FRANC),
        ("joule", &JOULE),
        ("forint", &FORINT),
        ("MGA", &MALAGASY_ARIARY),
        (
            "joules_per_kilogram_degree_kelvin",
            &JOULES_PER_KILOGRAM_DEGREE_KELVIN,
        ),
        ("TTD", &TRINIDAD_AND_TOBAGO_DOLLAR),
        ("ZAR", &RAND),
        ("cm²", &SQUARE_CENTIMETER),
        ("joules_per_hour", &JOULES_PER_HOUR),
        ("°F/min", &DEGREES_FAHRENHEIT_PER_MINUTE),
        ("kyat", &KYAT),
        ("percent_per_second", &PERCENT_PER_SECOND),
        ("millimeters_of_mercury", &MILLIMETERS_OF_MERCURY),
        ("Δ°C", &CELSIUS_DEGREES),
        ("cubic_feet_per_second", &CUBIC_FEET_PER_SECOND),
        ("Wh/ft²", &WATT_HOURS_PER_SQUARE_FOOT),
        ("square_millimeter", &SQUARE_MILLIMETER),
        ("MJ/°K", &MEGAJOULES_PER_DEGREE_KELVIN),
        ("mph", &MILES_PER_HOUR),
        ("kg", &KILOGRAM),
        ("kilovolt_ampere_reactive", &KILOVOLT_AMPERE_REACTIVE),
        ("armenian_dram", &ARMENIAN_DRAM),
        ("J/kg_dry", &JOULES_PER_KILOGRAM_DRY_AIR),
        ("Pa", &PASCAL),
        ("µs", &MICROSECOND),
        (
            "hundred_cubic_feet_natural_gas",
            &HUNDRED_CUBIC_FEET_NATURAL_GAS,
        ),
        ("tesla", &TESLA),
        ("₴", &HRYVNIA),
        ("liberian_dollar", &LIBERIAN_DOLLAR),
        ("bulgarian_lev", &BULGARIAN_LEV),
        ("dBµV", &DB_MICROVOLT),
        ("malagasy_ariary", &MALAGASY_ARIARY),
        ("MXN", &MEXICAN_PESO),
        ("NPR", &NEPALESE_RUPEE),
        ("SDG", &SUDANESE_POUND),
        ("ampere_square_meter", &AMPERE_SQUARE_METER),
        ("J/m³", &JOULES_PER_CUBIC_METER),
        ("footcandle", &FOOTCANDLE),
        ("milliliter", &MILLILITER),
        ("BZD", &BELIZE_DOLLAR),
        ("UGX", &UGANDA_SHILLING),
        ("julian_month", &JULIAN_MONTH),
        ("BRL", &BRAZILIAN_REAL),
        ("kenyan_shilling", &KENYAN_SHILLING),
        ("cubic_feet_per_hour", &CUBIC_FEET_PER_HOUR),
        ("km", &KILOMETER),
        ("ARS", &ARGENTINE_PESO),
        ("zł", &PZLOTY),
        ("milligram", &MILLIGRAM),
        ("W/m²", &WATTS_PER_SQUARE_METER),
        ("ft/s", &FEET_PER_SECOND),
        ("IRR", &IRANIAN_RIAL),
        ("wk", &WEEK),
        ("MDL", &MOLDAVIAN_LEU),
        (
            "cubic_feet_per_minute_per_square_foot",
            &CUBIC_FEET_PER_MINUTE_PER_SQUARE_FOOT,
        ),
        ("kBTU/h/ft²", &KILOBTUS_PER_HOUR_PER_SQUARE_FOOT),
        ("EER", &ENERGY_EFFICIENCY_RATIO),
        ("megavolt_ampere", &MEGAVOLT_AMPERE),
        ("IDR", &RUPIAH),
        ("mVA", &MEGAVOLT_AMPERE),
        ("cubic_millimeter", &CUBIC_MILLIMETER),
        ("miles_per_hour", &MILES_PER_HOUR),
        ("radian", &RADIAN),
        ("bermudian_dollar", &BERMUDIAN_DOLLAR),
        ("SBD", &SOLOMON_ISLANDS_DOLLAR),
        ("Nm", &NEWTON_METER),
        ("hecto_cubic_foot", &HECTO_CUBIC_FOOT),
        ("knot", &KNOT),
        ("hryvnia", &HRYVNIA),
        ("A", &AMPERE),
        ("per_hour", &PER_HOUR),
        ("Ah", &AMPERE_HOUR),
        ("zambian_kwacha", &ZAMBIAN_KWACHA),
        ("cd/m²", &CANDELAS_PER_SQUARE_METER),
        ("libyan_dinar", &LIBYAN_DINAR),
        ("BHD", &BAHRAINI_DINAR),
        ("cal/g", &CALORIE_PER_GRAM),
        ("standard_cubic_meter", &CUBIC_METERS_NATURAL_GAS),
        ("gigajoules_per_hour", &GIGAJOULES_PER_HOUR),
        ("W/cfm", &WATTS_PER_CUBIC_FEET_PER_MINUTE),
        ("degrees_angular", &DEGREES_ANGULAR),
        ("PB", &PETABYTE),
        ("australian_dollar", &AUSTRALIAN_DOLLAR),
        ("tanzanian_shilling", &TANZANIAN_SHILLING),
        ("phot", &PHOT),
        ("mmHg", &MILLIMETERS_OF_MERCURY),
        ("t", &SHORT_TON),
        ("lb/min", &POUNDS_PER_MINUTE),
        ("£", &POUND_STERLING),
        (
            "nephelometric_turbidity_units",
            &NEPHELOMETRIC_TURBIDITY_UNITS,
        ),
        ("MNT", &TUGRIK),
        ("hft³", &HECTO_CUBIC_FOOT),
        ("kV", &KILOVOLT),
        ("kilovolt_ampere", &KILOVOLT_AMPERE),
        ("taiwan_dollar", &TAIWAN_DOLLAR),
        ("kilohm", &KILOHM),
        ("mile", &MILE),
        ("year", &YEAR),
        ("hL", &HECTOLITER),
        ("naira", &NAIRA),
        ("cfh", &CUBIC_FEET_PER_HOUR),
        ("mm", &MILLIMETER),
        ("gourde", &GOURDE),
        ("fc", &FOOTCANDLE),
        ("volt", &VOLT),
        ("kWh/m²", &KILOWATT_HOURS_PER_SQUARE_METER),
        ("MZN", &METICAL),
        ("kilogram", &KILOGRAM),
        ("tonrefh", &TONS_REFRIGERATION_HOUR),
        ("kilojoules_per_hour", &KILOJOULES_PER_HOUR),
        ("yard", &YARD),
        ("hectoliter", &HECTOLITER),
        ("CUP", &CUBAN_PESO),
        ("RSD", &SERBIAN_DINAR),
        ("yemeni_rial", &YEMENI_RIAL),
        ("north_korean_won", &NORTH_KOREAN_WON),
        ("millimeters_per_minute", &MILLIMETERS_PER_MINUTE),
    ]
    .iter()
    .cloned()
    .collect()
});
