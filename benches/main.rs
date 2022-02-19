// Copyright (C) 2020 - 2022, J2 Innovations
//

//!
//! Tests the Hayson decoding performance
//!

use criterion::{criterion_group, criterion_main, Criterion};

use libhaystack::haystack::encoding::zinc::decode::*;
use libhaystack::haystack::val::*;
use std::fs;

fn criterion_json_parse(bench: &mut Criterion) {
    let string = fs::read_to_string("benches/json/points.json").expect("Invalid json test file");
    bench.bench_function("JSON parse points", |b| {
        b.iter(|| {
            let value: Value = serde_json::from_str(&string).expect("Value");

            let grid = Grid::try_from(&value).expect("Grid");

            assert!(!grid.is_empty());
        })
    });
}

fn criterion_zinc_parse(bench: &mut Criterion) {
    let string = fs::read_to_string("benches/zinc/points.zinc").expect("Invalid zinc test file");
    bench.bench_function("Zinc parse points", |b| {
        b.iter(|| {
            let value: Value = from_str(&string).expect("Value");

            let grid = Grid::try_from(&value).expect("Grid");

            assert!(!grid.is_empty());
        });
    });
}

criterion_group!(benches, criterion_zinc_parse, criterion_json_parse);
criterion_main!(benches);

#[cfg(never)]
fn main() {}
