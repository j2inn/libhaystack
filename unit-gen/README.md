# Haystack Units generation tool

Tool used to generate the unit types from the Fantom [units database](https://raw.githubusercontent.com/fantom-lang/fantom/master/etc/sys/units.txt)

# Running the tool

## Run the generator
`cargo run --manifest-path .\unit-gen\Cargo.toml`

## Format the output
`cargo fmt`

# Output

The tool generates a single Rust file containing all the Haystack Units.
The generated module is located at: `src/haystack/units/units_generated.rs`