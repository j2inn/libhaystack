// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Grid

#[cfg(test)]
use libhaystack::dict;
use libhaystack::val::*;

#[test]
fn test_grid_make_value() {
    let grid = Grid::make_empty();

    assert!(grid.is_empty());

    let value: Value = grid.into();

    assert!(value.is_grid());
    assert!(!value.is_dict());

    assert_eq!(Grid::try_from(&value), Ok(Grid::make_empty()));
}

#[test]
fn test_grid_make_err() {
    let grid = Grid::make_err("error");

    assert!(grid.is_empty());
    assert!(grid.is_err());
}

#[test]
fn test_grid_make_rows() {
    let grid = Grid::make_from_dicts(vec![
        dict! {"a"=> Value::Marker},
        dict! {"b" => Value::make_true()},
    ]);

    assert!(!grid.is_empty());
    assert!(!grid.is_err());
    assert_eq!(grid.len(), 2);
    assert_ne!(grid, Grid::default());

    assert_eq!(
        grid.columns
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<String>>(),
        vec!["a", "b"]
    );

    assert_eq!(grid[0], dict! {"a" => Value::Marker});
    assert_eq!(grid[1], dict! {"b" => Value::make_true()});
}

#[test]
fn test_grid_iterator() {
    let recs = vec![
        dict! {
            "site" => Value::make_marker(),
            "dis" => Value::make_str("Site")
        },
        dict! {
            "equip" => Value::make_marker(),
            "navName" => Value::make_str("Equip")
        },
    ];
    let value = Value::make_grid(Grid::make_from_dicts(recs));
    assert!(value.is_grid());

    let grid = Grid::try_from(&value).unwrap();
    let mut pos = 0;
    for rec in grid.into_iter() {
        if pos == 0 {
            assert_eq!(rec, &grid[pos])
        }
        if pos == 1 {
            assert_eq!(rec, &grid[pos])
        }
        pos += 1;
    }
    assert_eq!(pos, 2);
}
