// Copyright (C) 2020 - 2022, J2 Innovations

//
//! Units Rust code generator
//!

use std::collections::HashMap;

use super::parser::parse_units;
use libhaystack::units::Unit;

pub(super) fn parse_and_generate(bytes: &[u8]) -> Result<String, pom::Error> {
    let units = parse_units().parse(bytes)?;
    Ok(generate_units(units))
}

fn generate_units(units: Vec<Option<Unit>>) -> String {
    let mut lines = Vec::<String>::new();

    lines.push(gen_file_header());

    let mut last_quantity = String::default();
    let mut units_map: HashMap<String, String> = HashMap::new();

    for unit in units.into_iter().filter(|u| u.is_some()) {
        if let Some(unit) = unit {
            if unit.name() == "" {
                continue;
            }

            if let Some(quantity) = unit.quantity.as_ref() {
                if quantity != &last_quantity {
                    lines.push(format!("\n// {quantity}\n"));
                    last_quantity = quantity.clone();
                }
            }

            lines.push("\nlazy_static! { ".to_string());
            lines.push(unit_to_code(&unit));
            lines.push("}\n".to_string());

            for id in &unit.ids {
                units_map.insert(id.to_string(), unit.name().to_uppercase());
            }
        }
    }

    lines.push(unit_map_to_code(&units_map));

    lines.join("")
}

fn gen_file_header() -> String {
    concat!(
        "// Copyright (C) 2020 - 2022, J2 Innovations\n",
        "// Haystack Unit module - auto generated.\n",
        "\n",
        "#![allow(clippy::approx_constant)]\n",
        "use super::{Unit, UnitDimensions};\n",
        "use lazy_static::lazy_static;\n",
        "use std::collections::HashMap;\n"
    )
    .to_string()
}

fn unit_to_code(unit: &Unit) -> String {
    format!(
        "pub static ref {name}:Unit = {unit:#?};",
        name = unit.name().to_uppercase(),
    )
    .replace("]", "].to_vec()")
    .replace("\",", "\".to_string(),")
}

fn unit_map_to_code(units: &HashMap<String, String>) -> String {
    format!(
        "lazy_static! {{pub static ref UNITS:HashMap<&'static str, &'static Unit> = [{unit_list}].iter().cloned().collect();}}",
        unit_list = units.iter().map(|(k,v)| format!("(\"{k}\", &*{v}),")).collect::<Vec<String>>().join("\n")
    )
}
