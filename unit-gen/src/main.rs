// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Units parsing and code generation

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use unit_gen::parse_and_generate;

mod parser;
mod unit_gen;

/// Tool for generating the unit definitions in Rust from the Fantom unit database
fn main() {
    let def_in_path = &String::from("unit-gen/units.txt");
    let def_out_path = &String::from("src/haystack/units/units_generated.rs");

    println!("Running unit-gen tool.");

    println!(
        "Generator working dir: {}",
        env::current_dir().unwrap().display()
    );

    let args: Vec<String> = env::args().collect();
    let in_path = args.get(1).unwrap_or(def_in_path);
    let out_path = args.get(2).unwrap_or(def_out_path);

    let mut input: Vec<u8> = Vec::new();
    File::open(in_path)
        .expect("Units file.")
        .read_to_end(&mut input)
        .expect("Read units succeeded.");

    let gen = parse_and_generate(input.as_slice()).expect("Unit to parse.");
    File::create(out_path)
        .expect("Out file")
        .write(gen.as_bytes())
        .expect("Write units succeeded.");

    println!("unit-gen tool completed.");
}
