[![GitHub CI](https://github.com/j2inn/libhaystack/actions/workflows/master-push.yaml/badge.svg)](https://github.com/j2inn/libhaystack/actions/workflows/master-push.yaml)
[![crates.io](https://img.shields.io/crates/v/libhaystack.svg)](https://crates.io/crates/libhaystack)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://github.com/j2inn/libhaystack/blob/master/LICENSE)

# Haystack library

Implementation of the [Haystack 4 spec](https://project-haystack.org/) in the Rust programming language.

The library covers extensive parts of the specification, and it uses the cargo features to allow opt-in on features such as [decoders](https://docs.rs/libhaystack/latest/libhaystack/haystack/encoding/index.html), [filter](https://docs.rs/libhaystack/latest/libhaystack/haystack/filter/index.html), [units](https://docs.rs/libhaystack/latest/libhaystack/haystack/units/index.html), [timezone](https://docs.rs/libhaystack/latest/libhaystack/haystack/timezone/index.html), and the [C](https://docs.rs/libhaystack/latest/libhaystack/c_api/index.html) FFI API.

This implementation is geared towards constructing high performance haystack application that can also run efficiently on constrained devices, for a more general implementation, J2 Innovations
provides a [TypeScript](https://github.com/j2inn/haystack-core) implementation also.
At the moment, the library requires the allocator feature and the standard library, but it can be compiled to WASM as a non OS target.

## Building

Using cargo `cargo build` creates a debug version, `cargo build --release` creates a release version.

Specialize builds for each feature set can be compiled as `cargo build --release --no-default-features --features "encoders, zinc"`, which will compile only the core types
and the zinc encoding modules, resulting in a small binary (12KB on Windows x86-64)

## Testing

Run unit and integration tests with `cargo test`

## Docs

As usual, docs for the library are generate on [docs.rs](https://docs.rs/libhaystack/latest/libhaystack/) each time we publish.

## Features

### Types

The library fundamental type is [Value](https://docs.rs/libhaystack/latest/libhaystack/haystack/val/value/enum.Value.html). It can hold any of the Haystack supported [data-types](https://project-haystack.org/doc/docHaystack/Kinds).

### Scalar types

Create a [Str](https://docs.rs/libhaystack/latest/libhaystack/haystack/val/string/struct.Str.html)
[Value](https://docs.rs/libhaystack/latest/libhaystack/haystack/val/value/enum.Value.html) from a `&str`
```rust
use libhaystack::val::*;
// Creates a Str Value
let value = Value::from("Hello");

// Get a std::String from the value
assert!(String::try_form(value).expect("String"), "Hello");
```

Create a [Number](https://docs.rs/libhaystack/latest/libhaystack/haystack/val/number/struct.Number.html) [Value](https://docs.rs/libhaystack/latest/libhaystack/haystack/val/value/enum.Value.html) with a [unit](https://docs.rs/libhaystack/latest/libhaystack/haystack/units/index.html)
```rust
use libhaystack::val::*;
use libhaystack::units::*;

// Creates a Number Value using the Value helper function
let a = Value::make_number_unit(42, get_unit("m³"));

// Creates the Number scalar
let b = Number::make_with_unit(100.0, "m³".into());

// Add numbers with units
assert_eq!(Number::try_form(a).expect("Number") + b, Number::make_with_unit(142.0, get_unit("m³")));
```

### Complex types

Create a Haystack [Dict](https://docs.rs/libhaystack/latest/libhaystack/haystack/val/dict/struct.Dict.html)
```rust
use libhaystack::val::*;

// Create the Dict type
let dict = Value::from(dict! {
    "site" => Value::make_marker(),
    "name" => Value::make_str("Foo")
});

assert!(dict.has("site"));
assert_eq!(dict_value.get_str("name"), Some(&"Foo".into()));

// Wrap the type as a Value
let value: Value = dict.into();
assert!(value.is_dict());
```

### Filter

A Haystack 4 compliant [filter](https://docs.rs/libhaystack/latest/libhaystack/haystack/filter/index.html) parser and evaluator is provided that uses the [ontology](https://project-haystack.org/doc/docHaystack/Ontology) definitions from Project Haystack.

```rust
use libhaystack::dict;
use libhaystack::val::*;
use libhaystack::filter::*;

// Parse the filter from a string
let filter = Filter::try_from(r#"site and dis=="Test""#).expect("Filter");

// Define a Dict to apply the filter against
let dict = dict!{"site" => Value::make_marker(), "dis" => Value::from("Test")};

// Verify that the filter matched the Dict
assert_eq!(dict.filter(&filter), true);

// Filter using advanced ontology query
let filter = Filter::try_from(r#"^geoPlace and dis=="Test""#).expect("Filter");

// Sites are [geoPlaces](https://project-haystack.org/doc/lib-phIoT/site)
let dict = dict!{"site" => Value::make_marker(), "dis" => Value::from("Test")};

// Verify that the filter matched the Dict
assert_eq!(dict.filter(&filter), true);

```

### Encoding

The library provides support for [JSON](https://project-haystack.org/doc/docHaystack/Json), [Zinc](https://project-haystack.org/doc/docHaystack/Zinc), [Trio](https://project-haystack.org/doc/docHaystack/Trio), and [Brio](https://haxall.io/doc/docHaxall/Brio) encoding.

JSON is provided through the excellent Serde library. Zinc, Trio, and Brio are hand-tuned decoders and encoders built for performance.

Each format is opt-in via a cargo feature (`json`, `zinc`, `trio`, `brio`).

#### JSON

[JSON](https://project-haystack.org/doc/docHaystack/Json) encoding is provided through the excellent [Serde](https://serde.rs) library. All Haystack types implement `Serialize` and `Deserialize`.

```rust
use libhaystack::val::*;
use libhaystack::dict;

// Decode a Dict from Hayson (Haystack JSON) encoding
let json = r#"{"_kind":"dict","dis":{"_kind":"str","val":"Chiller Plant"},"site":{"_kind":"marker"}}"#;
let value: Value = serde_json::from_str(json).expect("decode");
let dict = Dict::try_from(&value).expect("dict");
assert_eq!(dict.get_str("dis"), Some(&"Chiller Plant".into()));
```

```rust
use libhaystack::val::*;
use libhaystack::dict;

// Encode a Dict to Hayson JSON
let value = Value::make_dict(dict! {
    "dis"  => Value::from("Chiller Plant"),
    "site" => Value::make_marker()
});
let json = serde_json::to_string(&value).expect("encode");
println!("{json}");
// {"_kind":"dict","dis":{"_kind":"str","val":"Chiller Plant"},"site":{"_kind":"marker"}}
```

#### Zinc

[Zinc](https://project-haystack.org/doc/docHaystack/Zinc) is a human-readable text format with a performance-oriented streaming Grid parser.

```rust
use libhaystack::val::*;
use libhaystack::encoding::zinc::decode;

// Decode a Number with unit from Zinc encoding
let value: Value = decode::from_str("42kW").expect("decode");
assert_eq!(value, Value::from(Number::make_with_unit(42.0, "kW".into())));
```

```rust
use libhaystack::val::*;
use libhaystack::dict;
use libhaystack::encoding::zinc::encode::to_zinc_string;

// Encode a Dict to Zinc
let value = Value::make_dict(dict! {
    "dis"  => Value::from("Chiller Plant"),
    "site" => Value::make_marker()
});
let zinc = to_zinc_string(&value).expect("encode");
println!("{zinc}");
// {dis:"Chiller Plant",site:M}
```

#### Trio

[Trio](https://project-haystack.org/doc/docHaystack/Trio) ("Text Record Input/Output") is a human-friendly plain-text format derived from YAML, used for hand-authoring Haystack records.

```rust
use libhaystack::val::*;
use libhaystack::encoding::trio::decode::TrioReader;

// Decode a sequence of dicts from Trio text
let trio = "dis: \"Chiller Plant\"\nsite\n---\ndis: \"AHU-1\"\nequip\n";
let dicts = TrioReader::dicts_from_str(trio).expect("dicts");
assert_eq!(dicts.len(), 2);
```

```rust
use libhaystack::val::*;
use libhaystack::dict;
use libhaystack::encoding::trio::encode::TrioWriter;

// Encode a dict to Trio text
let mut writer = TrioWriter::new();
writer.add_dict(dict! {
    "dis"  => Value::from("Chiller Plant"),
    "site" => Value::make_marker()
});
println!("{}", writer.to_trio_string());
// dis: "Chiller Plant"
// site
```

#### Brio

Brio is a compact binary format used by the [Haxall](https://haxall.io) platform. It provides the most efficient serialization of all supported formats — benchmarks show it decoding ~43% faster than JSON and ~69% faster than Zinc on real-world point grids.

```rust
use libhaystack::val::*;
use libhaystack::dict;
use libhaystack::encoding::brio::encode::ToBrio;
use libhaystack::encoding::brio::decode::from_brio;

// Encode any Value to a compact binary Vec<u8>
let val = Value::make_dict(dict! {
    "site" => Value::make_marker(),
    "dis"  => Value::from("Main Campus")
});
let bytes = val.to_brio_vec().expect("encode");

// Decode back from a byte slice
let decoded = from_brio(&mut bytes.as_slice()).expect("decode");
assert_eq!(val, decoded);
```
## C API

This library exposes a C based API that allows it to be consumed by other programming languages with a C FFI.
The header file generation is done using [cbindgen](https://github.com/eqrion/cbindgen)

Building the header file:
```
cbindgen --lang c -q --crate libhaystack --output src/c_api/libhaystack.h
```
Please consult the [pre-generated](https://github.com/j2inn/libhaystack/blob/master/src/c_api/libhaystack.h) header file distributed within the repo.

### Webassembly support

By leveraging the C API, the function exposed can be called in browsers, Node.js, or Deno.

For this [wasm-pack](https://rustwasm.github.io/) is used to generate the wasm binary file, the JS wrapper for initialization, and a typescript file with the API definitions.

```
wasm-pack build --out-dir wasm --target web
```
