// Copyright (C) 2020 - 2022, J2 Innovations

//! Generates `benches/brio/points.brio` from `benches/json/points.json`.
//!
//! Run with:
//!
//! ```
//! cargo run --benches gen_brio_fixture --features brio
//! ```

use libhaystack::encoding::brio::encode::ToBrio;
use libhaystack::haystack::val::{Grid, Value};
use std::fs;

fn main() {
    let json = fs::read_to_string("benches/json/points.json")
        .expect("cannot read benches/json/points.json");

    let value: Value = serde_json::from_str(&json).expect("JSON parse failed");
    let grid = Grid::try_from(&value).expect("not a Grid");

    let bytes = Value::from(grid).to_brio_vec().expect("brio encode failed");

    fs::create_dir_all("benches/brio").expect("cannot create benches/brio");
    fs::write("benches/brio/points.brio", &bytes).expect("cannot write benches/brio/points.brio");

    println!("Wrote {} bytes to benches/brio/points.brio", bytes.len());
}
