// Copyright (C) 2020 - 2022, J2 Innovations
//

//! Dict microbenchmarks for 32-entry workloads
//!
//! Run:
//! cargo bench --bench dict

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use libhaystack::haystack::val::*;
use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Duration;

const PROFILE_THRESHOLDS: [usize; 7] = [2, 4, 8, 16, 24, 32, 48];
const WORKLOAD_SIZE: usize = 32;

fn make_dict_with_profile(small_max_entries: usize) -> Dict {
    let mut dict = Dict::with_small_max_entries(small_max_entries);
    for i in 0..WORKLOAD_SIZE {
        dict.insert(format!("k{i:02}"), Value::from(i as i32));
    }
    dict
}

fn make_btreemap() -> BTreeMap<String, Value> {
    let mut map = BTreeMap::new();
    for i in 0..WORKLOAD_SIZE {
        map.insert(format!("k{i:02}"), Value::from(i as i32));
    }
    map
}

fn criterion_dict_profile_get_32(bench: &mut Criterion) {
    let mut group = bench.benchmark_group("Dict profile get n=32");
    group.sample_size(30);
    group.measurement_time(Duration::from_secs(2));

    let key = format!("k{:02}", WORKLOAD_SIZE.saturating_sub(2));
    for &small_max_entries in &PROFILE_THRESHOLDS {
        let dict = make_dict_with_profile(small_max_entries);
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("max={small_max_entries}")),
            &(dict, key.as_str()),
            |b, (dict, key)| {
                b.iter(|| {
                    let value = dict.get(black_box(*key));
                    assert!(value.is_some());
                });
            },
        );
    }

    group.finish();
}

fn criterion_dict_profile_insert_32(bench: &mut Criterion) {
    let mut group = bench.benchmark_group("Dict profile insert n=32");
    group.sample_size(30);
    group.measurement_time(Duration::from_secs(2));

    for &small_max_entries in &PROFILE_THRESHOLDS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("max={small_max_entries}")),
            &small_max_entries,
            |b, &small_max_entries| {
                b.iter(|| {
                    let mut dict = Dict::with_small_max_entries(small_max_entries);
                    for i in 0..WORKLOAD_SIZE {
                        dict.insert(format!("k{i:02}"), Value::from(i as i32));
                    }
                    black_box(dict)
                });
            },
        );
    }

    group.finish();
}

fn criterion_dict_profile_remove_32(bench: &mut Criterion) {
    let mut group = bench.benchmark_group("Dict profile remove n=32");
    group.sample_size(30);
    group.measurement_time(Duration::from_secs(2));

    let key = format!("k{:02}", WORKLOAD_SIZE / 2);
    for &small_max_entries in &PROFILE_THRESHOLDS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("max={small_max_entries}")),
            &(small_max_entries, key.as_str()),
            |b, &(small_max_entries, key)| {
                b.iter(|| {
                    let mut dict = make_dict_with_profile(small_max_entries);
                    let removed = dict.remove(key);
                    assert!(removed.is_some());
                    black_box(dict)
                });
            },
        );
    }

    group.finish();
}

fn criterion_btreemap_baseline_32(bench: &mut Criterion) {
    let get_key = format!("k{:02}", WORKLOAD_SIZE.saturating_sub(2));
    let rm_key = format!("k{:02}", WORKLOAD_SIZE / 2);

    let mut group_get = bench.benchmark_group("BTreeMap baseline get n=32");
    group_get.sample_size(30);
    group_get.measurement_time(Duration::from_secs(2));
    let map = make_btreemap();
    group_get.bench_function("baseline", |b| {
        b.iter(|| {
            let value = map.get(black_box(get_key.as_str()));
            assert!(value.is_some());
        });
    });
    group_get.finish();

    let mut group_insert = bench.benchmark_group("BTreeMap baseline insert n=32");
    group_insert.sample_size(30);
    group_insert.measurement_time(Duration::from_secs(2));
    group_insert.bench_function("baseline", |b| {
        b.iter(|| {
            let mut map = BTreeMap::new();
            for i in 0..WORKLOAD_SIZE {
                map.insert(format!("k{i:02}"), Value::from(i as i32));
            }
            black_box(map)
        });
    });
    group_insert.finish();

    let mut group_remove = bench.benchmark_group("BTreeMap baseline remove n=32");
    group_remove.sample_size(30);
    group_remove.measurement_time(Duration::from_secs(2));
    group_remove.bench_function("baseline", |b| {
        b.iter(|| {
            let mut map = make_btreemap();
            let removed = map.remove(rm_key.as_str());
            assert!(removed.is_some());
            black_box(map)
        });
    });
    group_remove.finish();
}

criterion_group!(
    benches,
    criterion_dict_profile_get_32,
    criterion_dict_profile_insert_32,
    criterion_dict_profile_remove_32,
    criterion_btreemap_baseline_32
);
criterion_main!(benches);
