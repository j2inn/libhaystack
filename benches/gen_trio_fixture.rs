// Copyright (C) 2020 - 2024, J2 Innovations

//! Generates `benches/trio/points.trio` from `benches/json/points.json`.
//!
//! Run with:
//!
//! ```
//! cargo bench --bench gen_trio_fixture
//! ```

use libhaystack::encoding::trio::encode::TrioWriter;
use libhaystack::haystack::val::{Grid, Value};
use std::fs;

fn main() {
    let json = fs::read_to_string("benches/json/points.json")
        .expect("cannot read benches/json/points.json");

    let value: Value = serde_json::from_str(&json).expect("JSON parse failed");
    let grid = Grid::try_from(&value).expect("not a Grid");

    let trio = TrioWriter::new().add_grid(&grid).to_trio_string();

    fs::create_dir_all("benches/trio").expect("cannot create benches/trio");
    fs::write("benches/trio/points.trio", &trio).expect("cannot write benches/trio/points.trio");

    println!("Wrote {} bytes to benches/trio/points.trio", trio.len());
}
