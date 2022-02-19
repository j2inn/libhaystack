// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc decoding

use super::super::parser::ParserType;
use super::dict::parse_dict_parts;
use crate::haystack::val::{Column, Dict, Grid, Value};
use std::io::{Error, Read};

// Zinc [Grid](crate::val::Grid) meta
#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct GridMeta {
    ver: String,
    meta: Option<Dict>,
}

/// Parses a Zinc [Grid](crate::val::Grid)
/// # Arguments
/// - a generic ParserType
/// # Returns
/// - a [Grid](crate::val::Grid) Result
pub fn parse_grid<'a, 'b: 'a, R: Read>(parser: &'a mut ParserType<'b, R>) -> Result<Grid, Error> {
    let rows_iterator = parse_grid_iterator(parser)?;

    // Copy the grid meta and columns
    let mut grid = (&rows_iterator.grid).clone();

    // Collect the lazy parser result as a Row vector
    let rows: Result<Vec<Dict>, Error> = rows_iterator.into_iter().collect();

    match rows {
        Ok(rows) => {
            grid.rows = rows;
            Ok(grid)
        }
        Err(err) => Err(err),
    }
}

/// Returns a [Grid](crate::val::Grid) iterator that allows lazily parsing of a Grid and return
/// the Grid rows as an iterator.
///
/// # Arguments
/// - a generic ParserType
/// # Returns
/// - a [Dict](crate::val::Dict) iterator Result
pub fn parse_grid_iterator<'a, 'b: 'a, R: Read>(
    parser: &'a mut ParserType<'b, R>,
) -> Result<RowIterator<'a, 'b, R>, Error> {
    let (grid, rows_parser) = parse_grid_content(parser)?;

    Ok(RowIterator { grid, rows_parser })
}

/// Parse a Zinc [Grid](crate::val::Grid) with a lazy row parser
fn parse_grid_content<'a, 'b: 'a, R: Read>(
    parser: &'a mut ParserType<'b, R>,
) -> Result<(Grid, RowParser<'a, 'b, R>), Error> {
    let has_nested_grid_start = parse_nested_grid_start(parser)?;

    parse_grid_ver(parser)?;

    parser.lexer.read()?;

    let meta = parse_grid_meta(parser)?;
    parser.lexer.expect_char(b'\n', "Grid meta")?;

    let columns = parse_grid_columns(parser)?;
    parser.lexer.expect_char(b'\n', "Grid columns")?;

    parser.lexer.read()?;

    let grid = Grid {
        meta: if meta.is_empty() { None } else { Some(meta) },
        columns,
        rows: Vec::default(),
    };

    Ok((
        grid,
        RowParser {
            parser,
            has_nested_grid_start,
            has_nested_grid_end: false,
        },
    ))
}

/// Parse a nested [Grid](crate::val::Grid) start marker
fn parse_nested_grid_start<R: Read>(parser: &mut ParserType<R>) -> Result<bool, Error> {
    if parser.lexer.is_char(b'<') {
        parser.lexer.read()?;
        parser.lexer.expect_char(b'<', "Nested grid start")?;
        parser.lexer.consume_white_spaces()?;
        parser.lexer.read()?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Parse a nested [Grid](crate::val::Grid) end marker
fn parse_nested_grid_end<R: Read>(parser: &mut ParserType<R>) -> Result<(), Error> {
    parser.lexer.read()?;
    parser.lexer.expect_char(b'>', "Nested grid end")?;
    Ok(())
}

/// Parse [Grid](crate::val::Grid) ver
fn parse_grid_ver<R: Read>(parser: &mut ParserType<R>) -> Result<String, Error> {
    let ver = parser.lexer.expect_id()?.to_string();

    if ver != "ver" {
        return parser
            .lexer
            .make_generic_err(&format!("Grid Expecting 'ver', got '{ver}'."));
    }

    parser.lexer.read()?;

    parser.lexer.expect_char(b':', "Grid version")?;

    parser.lexer.read()?;
    let ver = parser.lexer.expect_value()?;

    match ver {
        Value::Str(str) => Ok(str.value),
        _ => parser
            .lexer
            .make_generic_err(&format!("Expecting 'ver' to be a Str, got '{ver:?}'.")),
    }
}

/// Parse [Grid](crate::val::Grid) meta data
fn parse_grid_meta<'a, 'b: 'a, R: Read>(parser: &'a mut ParserType<'b, R>) -> Result<Dict, Error> {
    parse_dict_parts(parser)
}

/// Parse [Grid](crate::val::Grid) columns
fn parse_grid_columns<'a, 'b: 'a, R: Read>(
    parser: &'a mut ParserType<'b, R>,
) -> Result<Vec<Column>, Error> {
    let mut list = Vec::<Column>::new();

    let mut done = false;

    while !done {
        parser.lexer.read()?;

        let name = parser.lexer.expect_id()?;

        parser.lexer.read()?;

        if parser.lexer.is_char(b'\n') || parser.lexer.is_char(b',') {
            list.push(Column {
                name: name.to_string(),
                meta: None,
            });
            if parser.lexer.is_char(b'\n') {
                done = true;
                break;
            } else {
                continue;
            }
        } else if parser.lexer.is_eof() {
            break;
        } else {
            match parse_grid_column_meta(parser) {
                Ok(dict) => {
                    if !dict.is_empty() {
                        list.push(Column {
                            name: name.to_string(),
                            meta: Some(dict),
                        });

                        if !parser.lexer.is_eof() {
                            parser.lexer.expect_char(b',', "Grid columns")?;
                        } else {
                            break;
                        }
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }

    if !done {
        return parser
            .lexer
            .make_generic_err("Invalid grid column, missing '\\n'");
    }

    Ok(list)
}

/// Parse [Grid](crate::val::Grid) col metadata
fn parse_grid_column_meta<'a, 'b: 'a, R: Read>(
    parser: &'a mut ParserType<'b, R>,
) -> Result<Dict, Error> {
    let mut dict = Dict::new();

    while !parser.lexer.is_eof() {
        if parser.lexer.is_char(b',') || !parser.lexer.is_id() {
            break;
        }

        let key = &parser.lexer.expect_id()?;

        parser.lexer.read()?;

        if parser.lexer.is_eof() {
            dict.insert(key.to_string(), Value::Marker);
            break;
        }

        if parser.lexer.is_char(b':') {
            parser.lexer.read()?;
            let value = parser.parse_value()?;
            dict.insert(key.to_string(), value);
            parser.lexer.read()?;
        } else {
            dict.insert(key.to_string(), Value::Marker);
        }
    }

    Ok(dict)
}

/// Parse [Grid](crate::val::Grid) rows
/// This struct will be used to implement the lazy parser
/// for the rows in a Zinc Grid
struct RowParser<'a, 'b: 'a, R: Read> {
    parser: &'a mut ParserType<'b, R>,
    has_nested_grid_start: bool,
    has_nested_grid_end: bool,
}

impl<'a, 'b: 'a, R: Read> RowParser<'a, 'b, R> {
    fn is_done(&self) -> bool {
        self.parser.lexer.is_eof() || self.has_nested_grid_end
    }

    fn parse_row(&mut self, grid: &Grid) -> Result<Dict, Error> {
        let mut dict = Dict::new();
        let cols = &grid.columns;

        let mut col_num = 0;
        let mut row_terminated = false;

        while !row_terminated {
            if self.parser.lexer.is_char(b',') {
                self.parser.lexer.read()?;
                col_num += 1;
                continue;
            }

            if self.parser.lexer.is_char(b'\n') {
                row_terminated = true;
                break;
            }

            let val = self.parser.parse_value()?;
            dict.insert(cols[col_num].name.clone(), val);

            self.parser.lexer.read()?;
        }

        if !row_terminated {
            return self
                .parser
                .lexer
                .make_generic_err("Zinc Grid parser: Unterminated Row.");
        }

        self.consume_end()?;
        Ok(dict)
    }

    fn consume_end(&mut self) -> Result<bool, Error> {
        if self.parser.lexer.is_char(b'\n') {
            self.parser.lexer.consume_white_spaces()?;
            if !self.parser.lexer.is_eof() {
                self.parser.lexer.read()?;
            }
        }

        if self.has_nested_grid_start && self.parser.lexer.is_char(b'>') {
            parse_nested_grid_end(self.parser)?;
            self.has_nested_grid_end = true
        }
        Ok(self.has_nested_grid_end)
    }
}

/// Iterator for [Grid](crate::val::Grid) rows that uses lazy parsing.
pub struct RowIterator<'a, 'b: 'a, R: Read> {
    grid: Grid,
    rows_parser: RowParser<'a, 'b, R>,
}

impl<'a, 'b: 'a, R: Read> Iterator for RowIterator<'a, 'b, R> {
    type Item = Result<Dict, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.rows_parser.is_done() {
            match self.rows_parser.consume_end() {
                Ok(end) => {
                    if end || self.rows_parser.is_done() {
                        None
                    } else {
                        Some(self.rows_parser.parse_row(&self.grid))
                    }
                }
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::super::parser::Parser;
    use super::{parse_grid, parse_grid_columns, parse_grid_meta, parse_grid_ver};
    use crate::dict;
    use crate::haystack::val::*;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_grid_ver() {
        let mut input = Cursor::new(r#"ver: "3.0""#.as_bytes());
        let mut parser = Parser::make(&mut input).expect("Parser");

        let ver = parse_grid_ver(&mut parser);
        assert_eq!(ver.ok(), Some("3.0".to_string()))
    }

    #[test]
    fn test_zinc_parse_grid_meta() {
        let mut input = Cursor::new(r#"foo: T bar:M test:[12,34]"#.as_bytes());
        let mut parser = Parser::make(&mut input).expect("Parser");

        let meta = parse_grid_meta(&mut parser);
        assert_eq!(
            meta.ok(),
            Some(dict! {"foo"=> Value::make_true(),
            "bar" => Value::Marker,
            "test" => vec![Value::make_number(12.0),Value::make_number(34.0)].into()})
        )
    }

    #[test]
    fn test_zinc_parse_grid_columns() {
        let mut input = Cursor::new("\nsite, equip foo:T , point\n".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Parser");

        let cols = parse_grid_columns(&mut parser);
        assert_eq!(
            cols.ok(),
            Some(vec![
                Column {
                    name: String::from("site"),
                    meta: None
                },
                Column {
                    name: String::from("equip"),
                    meta: Some(dict! {"foo"=> Value::make_true()})
                },
                Column {
                    name: String::from("point"),
                    meta: None
                }
            ])
        )
    }

    #[test]
    fn test_zinc_parse_grid_null_values() {
        let mut input = Cursor::new(
            r#"ver: "3.0"
            a,b,c,f,e
            "Berlin",,,"m³_gas",

        "#
            .as_bytes(),
        );

        let mut parser = Parser::make(&mut input).expect("Parser");
        let grid = parse_grid(&mut parser);

        assert!(grid.is_ok(), "Parses grid {err:?}", err = grid.err());

        let grid = grid.unwrap();

        assert_eq!(
            grid.columns
                .into_iter()
                .map(|c| c.name)
                .collect::<Vec<String>>()
                .join(","),
            "a,b,c,f,e"
        );
    }

    #[test]
    fn test_zinc_parse_grid() {
        let mut input =
            Cursor::new(concat!(r#"ver: "3.0""#, "\n", "val,id", "\n", ",@foo", "\n").as_bytes());
        let mut parser = Parser::make(&mut input).expect("Parser");

        let grid = parse_grid(&mut parser);

        assert!(grid.is_ok(), "Parses grid {err:?}", err = grid.err());

        assert_eq!(
            grid.ok(),
            Some(Grid {
                meta: None,
                columns: vec![
                    Column {
                        name: String::from("val"),
                        meta: None
                    },
                    Column {
                        name: String::from("id"),
                        meta: None
                    }
                ],
                rows: vec![dict! {"id" => Value::make_ref("foo")}],
            })
        )
    }

    #[test]
    fn test_zinc_parse_grid_embedded() {
        let mut input = Cursor::new(
            concat!(
                "<<",
                "\n",
                r#"ver: "3.0""#,
                "\n",
                "id",
                "\n\r",
                "@foo",
                "\n",
                "@bar",
                "\n",
                ">>"
            )
            .as_bytes(),
        );
        let mut parser = Parser::make(&mut input).expect("Parser");

        let grid = parse_grid(&mut parser);

        assert!(grid.is_ok(), "Parses grid {err:?}", err = grid.err());

        assert_eq!(
            grid.ok(),
            Some(Grid {
                meta: None,
                columns: vec![Column {
                    name: String::from("id"),
                    meta: None
                }],
                rows: vec![
                    dict! {"id" => Value::make_ref("foo")},
                    dict! {"id" => Value::make_ref("bar")}
                ],
            })
        )
    }

    #[test]
    fn test_zinc_parse_nested_grid() {
        let grid = r#"ver: "3.0"
            test,val
            M,<<
            ver:"3.0"
            end,date,val,dis,start,dateRule,weekdays,weekly,defaultEvent
            00:00:00,"*-01-01",F,"New Years",00:00:00,,,,
            00:00:00,,F,"Presidents Day",00:00:00,"3rd mon in feb",,,
            00:00:00,"*-04-18",F,"Good Friday",00:00:00,,,,
            00:00:00,,F,"Memorial Day",00:00:00,"last mon in may",,,
            00:00:00,"*-07-04",F,"Independence Day",00:00:00,,,,
            00:00:00,,F,"Labor Day",00:00:00,"1st mon in sep",,,
            00:00:00,,F,"Thanksgiving Day",00:00:00,"4th thu in nov",,,
            00:00:00,"*-12-25",F,"Christmas",00:00:00,,,,
            16:00:00,,T,"MON 08:00 am — 04:00 pm",08:00:00,,"mon",M,
            16:00:00,,T,"TUE 08:00 am — 04:00 pm",08:00:00,,"tue",M,
            16:00:00,,T,"WED 08:00 am — 04:00 pm",08:00:00,,"wed",M,
            16:00:00,,T,"THU 08:00 am — 04:00 pm",08:00:00,,"thu",M,
            16:00:00,,T,"FRI 08:00 am — 04:00 pm",08:00:00,,"fri",M,
            00:00:00,,F,,00:00:00,,"sun,mon,tue,wed,thu,fri,sat",,M

            >>,
            "#;
        let mut input = Cursor::new(grid.as_bytes());
        let mut parser = Parser::make(&mut input).expect("Parser");

        let grid = parse_grid(&mut parser);
        assert!(
            grid.is_ok(),
            "Grid decode error: {err:?}",
            err = grid.map_err(|e| e.to_string())
        );

        let grid = grid.unwrap();

        assert_eq!(
            grid.columns
                .iter()
                .map(|c| c.name.as_str())
                .collect::<Vec<&str>>(),
            vec!["test", "val",]
        );

        assert_eq!(
            Grid::try_from(&grid.rows[0]["val"]).expect("Grid").len(),
            14
        )
    }
}
