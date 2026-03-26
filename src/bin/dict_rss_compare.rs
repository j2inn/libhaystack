// Copyright (C) 2020 - 2022, J2 Innovations

//! Release-mode RSS comparison for Dict backing profiles.
//!
//! Run:
//! cargo run --release --bin dict_rss_compare

#[cfg(not(target_os = "linux"))]
compile_error!("dict_rss_compare is Linux-only (requires /proc/self/status)");

use libhaystack::dict;
use libhaystack::val::*;
use std::env;
use std::fs;
use std::process::Command;

const DICT_COUNT: usize = 100_000;
const DICT_ENTRY_COUNT: usize = 32;
const CHILD_MARKER: &str = "RSS_RESULT";
const MODE_ARG: &str = "--mode";

#[derive(Clone, Copy)]
enum RssMode {
    Vec32,
    Tree0,
}

impl RssMode {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "vec32" => Some(Self::Vec32),
            "tree0" => Some(Self::Tree0),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Vec32 => "vec32",
            Self::Tree0 => "tree0",
        }
    }

    fn small_max_entries(self) -> usize {
        match self {
            Self::Vec32 => 32,
            Self::Tree0 => 0,
        }
    }
}

fn main() {
    if let Some(mode) = mode_from_args() {
        run_child(mode);
        return;
    }

    let vec32_delta_kb = run_mode_subprocess(RssMode::Vec32);
    let tree0_delta_kb = run_mode_subprocess(RssMode::Tree0);

    let winner = if vec32_delta_kb < tree0_delta_kb {
        "vec32"
    } else if tree0_delta_kb < vec32_delta_kb {
        "tree0"
    } else {
        "tie"
    };

    let vec_vs_tree_pct = if tree0_delta_kb > 0 {
        (tree0_delta_kb as f64 - vec32_delta_kb as f64) * 100.0 / tree0_delta_kb as f64
    } else {
        0.0
    };

    println!(
        "RSS_COMPARE dicts={} entries={} vec32_delta_kb={} tree0_delta_kb={} winner={} vec_vs_tree_pct={:.2}",
        DICT_COUNT, DICT_ENTRY_COUNT, vec32_delta_kb, tree0_delta_kb, winner, vec_vs_tree_pct
    );
}

fn mode_from_args() -> Option<RssMode> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == MODE_ARG {
            let value = args.next()?;
            return RssMode::from_str(&value);
        }
    }
    None
}

fn run_mode_subprocess(mode: RssMode) -> u64 {
    let exe = env::current_exe().expect("current executable path");
    let output = Command::new(exe)
        .arg(MODE_ARG)
        .arg(mode.as_str())
        .output()
        .expect("spawn child process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        panic!(
            "child mode {} failed\nstdout:\n{}\nstderr:\n{}",
            mode.as_str(),
            stdout,
            stderr
        );
    }

    parse_child_delta_kb(&stdout)
        .or_else(|| parse_child_delta_kb(&stderr))
        .unwrap_or_else(|| {
            panic!(
                "missing RSS marker for mode {}\nstdout:\n{}\nstderr:\n{}",
                mode.as_str(),
                stdout,
                stderr
            )
        })
}

fn parse_child_delta_kb(output: &str) -> Option<u64> {
    for line in output.lines() {
        if !line.contains(CHILD_MARKER) {
            continue;
        }
        for part in line.split_whitespace() {
            if let Some(value) = part.strip_prefix("delta_kb=")
                && let Ok(parsed) = value.parse::<u64>()
            {
                return Some(parsed);
            }
        }
    }
    None
}

fn run_child(mode: RssMode) {
    let before_kb = current_rss_kb();
    let dicts = build_dicts(mode);
    let after_kb = current_rss_kb();

    let checksum: usize = dicts.iter().map(|d| d.len()).sum();
    assert_eq!(checksum, DICT_COUNT * DICT_ENTRY_COUNT);

    let delta_kb = after_kb.saturating_sub(before_kb);
    println!(
        "{} mode={} before_kb={} after_kb={} delta_kb={}",
        CHILD_MARKER,
        mode.as_str(),
        before_kb,
        after_kb,
        delta_kb
    );

    std::hint::black_box(dicts);
}

fn build_dicts(mode: RssMode) -> Vec<Dict> {
    let mut dicts = Vec::with_capacity(DICT_COUNT);
    for i in 0..DICT_COUNT {
        let mut dict = Dict::with_small_max_entries(mode.small_max_entries());
        for (key, value) in make_entries(i) {
            dict.insert(key, value);
        }
        dicts.push(dict);
    }
    dicts
}

fn make_entries(seed: usize) -> Vec<(String, Value)> {
    let date = Date::from_ymd(2024, ((seed % 12) + 1) as u32, ((seed % 28) + 1) as u32)
        .expect("valid date");
    let time = Time::from_hms_milli(
        (seed % 24) as u32,
        (seed % 60) as u32,
        ((seed * 7) % 60) as u32,
        ((seed * 13) % 1000) as u32,
    )
    .expect("valid time");
    let datetime = DateTime::parse_from_rfc3339("2024-06-19T19:48:23Z").expect("valid datetime");

    let list_value = Value::List(vec![
        Value::from(seed as i32),
        Value::make_marker(),
        Value::from("l"),
    ]);
    let inner_dict = dict! {
        "innerBool" => seed.is_multiple_of(2),
        "innerNum" => (seed % 1000) as i32,
        "innerStr" => format!("inner-{seed}")
    };
    let grid_value = Value::make_grid_from_dicts(vec![
        dict! {
            "id" => Value::make_ref(&format!("r:{seed}")),
            "val" => (seed % 100) as i32,
        },
        dict! {
            "id" => Value::make_ref(&format!("r:{}", seed + 1)),
            "val" => ((seed + 1) % 100) as i32,
        },
    ]);

    vec![
        ("k00".into(), Value::Null),
        ("k01".into(), Value::make_marker()),
        ("k02".into(), Value::make_remove()),
        ("k03".into(), Value::make_na()),
        ("k04".into(), Value::from(seed.is_multiple_of(2))),
        ("k05".into(), Value::from(seed as i32)),
        ("k06".into(), Value::from((seed as f64) * 1.5)),
        ("k07".into(), Value::from(format!("str-{seed}"))),
        ("k08".into(), Value::make_uri(&format!("/p/{seed}"))),
        ("k09".into(), Value::make_ref(&format!("ref:{seed}"))),
        ("k10".into(), Value::make_symbol(&format!("sym{seed}"))),
        ("k11".into(), Value::make_date(date)),
        ("k12".into(), Value::make_time(time)),
        ("k13".into(), Value::make_datetime(datetime)),
        (
            "k14".into(),
            Value::make_coord_from(37.0 + seed as f64 / 1000.0, -122.0),
        ),
        (
            "k15".into(),
            Value::make_xstr_from("TypeA", &format!("x-{seed}")),
        ),
        ("k16".into(), list_value),
        ("k17".into(), Value::make_dict(inner_dict)),
        ("k18".into(), grid_value),
        (
            "k19".into(),
            Value::from(format!("long-value-{seed}-abcdefg")),
        ),
        ("k20".into(), Value::from(((seed % 10_000) as i32) - 5000)),
        ("k21".into(), Value::make_false()),
        ("k22".into(), Value::make_datetime(datetime)),
        ("k23".into(), Value::List(vec![])),
        ("k24".into(), Value::make_dict(Dict::new())),
        ("k25".into(), Value::make_grid(Grid::make_empty())),
        (
            "k26".into(),
            Value::make_ref_with_dis(&format!("refdis:{seed}"), &format!("Ref {seed}")),
        ),
        ("k27".into(), Value::make_uri("/static/path")),
        ("k28".into(), Value::make_symbol("equip")),
        (
            "k29".into(),
            Value::make_coord_from(51.5, -0.1 - seed as f64 / 10_000.0),
        ),
        (
            "k30".into(),
            Value::make_xstr_from("TypeB", &format!("payload-{seed}")),
        ),
        ("k31".into(), Value::from(((seed % 1_000_000) as i32) * 3)),
    ]
}

fn current_rss_kb() -> u64 {
    let status = fs::read_to_string("/proc/self/status").expect("read /proc/self/status");
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            let value = rest
                .split_whitespace()
                .next()
                .expect("VmRSS value")
                .parse::<u64>()
                .expect("VmRSS parse");
            return value;
        }
    }
    panic!("VmRSS not found in /proc/self/status");
}
