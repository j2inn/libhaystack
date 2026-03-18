// Copyright (C) 2020 - 2022, J2 Innovations

//
//! Units Rust code generator
//!

use std::collections::HashMap;

use super::parser::parse_units;
use libhaystack::units::{Unit, UnitDimensions};

pub(super) fn parse_and_generate(bytes: &[u8]) -> Result<String, pom::Error> {
    let units = parse_units().parse(bytes)?;
    Ok(generate_units(units))
}

fn generate_units(units: Vec<Option<Unit>>) -> String {
    let mut lines = Vec::<String>::new();

    lines.push(gen_file_header());

    let mut last_quantity = String::default();
    let mut units_map: HashMap<String, String> = HashMap::new();

    for unit in units.into_iter().filter(|u| u.is_some()).flatten() {
        if unit.name() == "" {
            continue;
        }

        if let Some(quantity) = unit.quantity.as_ref() {
            if quantity != &last_quantity {
                lines.push(format!("\n// {quantity}\n"));
                last_quantity = quantity.to_string();
            }
        }

        lines.push(unit_to_code(&unit));
        lines.push("\n\n".to_string());

        for id in unit.ids.iter() {
            units_map.insert(id.to_string(), unit.name().to_uppercase());
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
        "use std::borrow::Cow;\n",
        "use std::collections::HashMap;\n",
        "use std::sync::LazyLock;\n"
    )
    .to_string()
}

fn unit_to_code(unit: &Unit) -> String {
    let quantity = match unit.quantity.as_ref() {
        Some(quantity) => format!("Some(Cow::Borrowed(\"{}\"))", quantity),
        None => "None".to_string(),
    };
    let ids = format!(
        "Cow::Borrowed(&[{}])",
        unit.ids
            .iter()
            .map(|id| format!("Cow::Borrowed(\"{}\")", id))
            .collect::<Vec<String>>()
            .join(", ")
    );
    let dimensions = match unit.dimensions {
        Some(dim) => dimensions_to_code(dim),
        None => "None".to_string(),
    };
    let scale = float_to_code(unit.scale);
    let offset = float_to_code(unit.offset);

    format!(
        concat!(
            "pub static {name}: Unit = Unit {{\n",
            "    quantity: {quantity},\n",
            "    ids: {ids},\n",
            "    dimensions: {dimensions},\n",
            "    scale: {scale},\n",
            "    offset: {offset},\n",
            "}};"
        ),
        name = unit.name().to_uppercase(),
        quantity = quantity,
        ids = ids,
        dimensions = dimensions,
        scale = scale,
        offset = offset,
    )
}

fn dimensions_to_code(dim: UnitDimensions) -> String {
    format!(
        concat!(
            "Some(UnitDimensions {{ ",
            "kg: {kg}, m: {m}, sec: {sec}, k: {k}, a: {a}, mol: {mol}, cd: {cd}",
            " }})"
        ),
        kg = dim.kg,
        m = dim.m,
        sec = dim.sec,
        k = dim.k,
        a = dim.a,
        mol = dim.mol,
        cd = dim.cd,
    )
}

fn float_to_code(value: f64) -> String {
    let text = format!("{:?}", value);
    if text.contains('.') || text.contains('e') || text.contains('E') {
        text
    } else {
        format!("{text}.0")
    }
}

fn unit_map_to_code(units: &HashMap<String, String>) -> String {
    format!(
        "pub static UNITS: LazyLock<HashMap<&'static str, &'static Unit>> = LazyLock::new(|| [{unit_list}].iter().cloned().collect());",
        unit_list = units.iter().map(|(k,v)| format!("(\"{k}\", &{v}),")).collect::<Vec<String>>().join("\n")
    )
}
