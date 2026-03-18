// Copyright (C) 2020 - 2024, J2 Innovations

//! Trio format decoder (reader)
//!
//! Reads Trio-encoded text and produces Haystack [`Dict`] and [`Grid`] values.
//!
//! See <https://project-haystack.org/doc/docHaystack/Trio>

use crate::haystack::encoding::zinc::decode::from_str as zinc_from_str;
use crate::haystack::val::{Dict, Grid, Str, Value};
use std::io::{BufRead, BufReader, Cursor, Error, ErrorKind, Read};

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn make_err(msg: &str) -> Error {
    Error::new(ErrorKind::InvalidData, msg)
}

/// True if the trimmed line consists entirely of `-` characters (entity separator).
fn is_separator_line(line: &str) -> bool {
    let t = line.trim();
    !t.is_empty() && t.chars().all(|c| c == '-')
}

/// True if the line begins a comment (`//`).
fn is_comment_line(line: &str) -> bool {
    line.trim_start_matches([' ', '\t']).starts_with("//")
}

/// True if the line is blank (empty or only whitespace).
fn is_blank_line(line: &str) -> bool {
    line.trim().is_empty()
}

/// True if the line starts a tag definition (a–z as first character).
fn is_tag_line(line: &str) -> bool {
    line.chars()
        .next()
        .map(|c| c.is_ascii_lowercase())
        .unwrap_or(false)
}

/// True if the line is indented (starts with a space or tab).
fn is_indented_line(line: &str) -> bool {
    matches!(line.chars().next(), Some(' ') | Some('\t'))
}

/// Parse the inline part of a tag's value: try Zinc first, fall back to a
/// raw [`Str`] if Zinc parsing fails.
fn parse_inline_value(s: &str) -> Value {
    match zinc_from_str(s) {
        Ok(v) => v,
        Err(_) => Value::Str(Str::make(s)),
    }
}

// ---------------------------------------------------------------------------
// TrioReader
// ---------------------------------------------------------------------------

/// Reads Trio-encoded text, yielding one [`Dict`] per entity.
///
/// `TrioReader<R>` is generic over any [`BufRead`] source and reads lazily,
/// one line at a time.  Use [`from_str`][TrioReader::from_str] for in-memory
/// strings, [`from_reader`][TrioReader::from_reader] for files or other
/// [`Read`] streams, or [`new`][TrioReader::new] when you already hold a
/// [`BufRead`].
///
/// # Examples
///
/// ```
/// use libhaystack::encoding::trio::decode::TrioReader;
/// use libhaystack::val::*;
///
/// let input = "site\ndis: \"Site A\"\n---\nsite\ndis: \"Site B\"\n";
/// let dicts = TrioReader::from_str(input).read_all_dicts().expect("dicts");
/// assert_eq!(dicts.len(), 2);
/// ```
#[derive(Debug)]
pub struct TrioReader<R: BufRead> {
    reader: R,
    /// The currently-loaded line, or `None` when the reader is exhausted.
    current: Option<String>,
    /// I/O error captured during line-reading, returned by the next
    /// [`read_dict`][TrioReader::read_dict] call.
    io_error: Option<Error>,
}

impl<R: BufRead> TrioReader<R> {
    /// Creates a new reader wrapping any [`BufRead`] source.
    ///
    /// The first line is pre-loaded eagerly so that `is_eof()` and
    /// `current_line()` are immediately usable.  Prefer the higher-level
    /// constructors [`from_str`][TrioReader::from_str] and
    /// [`from_reader`][TrioReader::from_reader] for typical use.
    pub fn new(reader: R) -> Self {
        let mut r = TrioReader {
            reader,
            current: None,
            io_error: None,
        };
        r.advance(); // prime: load the first line
        r
    }

    // -----------------------------------------------------------------------
    // Public interface
    // -----------------------------------------------------------------------

    /// Returns the next dict in the stream, or `None` at end-of-input.
    ///
    /// Separators (`---`), blank lines, and comment lines between entities are
    /// skipped automatically.
    pub fn read_dict(&mut self) -> Result<Option<Dict>, Error> {
        // Surface any I/O error captured during a previous `advance()`.
        if let Some(e) = self.io_error.take() {
            return Err(e);
        }

        // Skip leading blank lines, comments, and separator lines.
        loop {
            if self.is_eof() {
                return Ok(None);
            }
            let skip = {
                let l = self.current_line();
                is_blank_line(l) || is_comment_line(l) || is_separator_line(l)
            };
            if skip {
                self.advance();
            } else {
                break;
            }
        }

        let mut dict = Dict::new();
        let mut has_tags = false;

        loop {
            if self.is_eof() {
                break;
            }

            if is_separator_line(self.current_line()) {
                self.advance(); // consume the `---` line
                break;
            }

            let skip_line = {
                let line = self.current_line();
                is_blank_line(line) || is_comment_line(line)
            };
            if skip_line {
                self.advance();
                continue;
            }

            if is_tag_line(self.current_line()) {
                self.read_tag(&mut dict)?;
                has_tags = true;
            } else {
                let line = self.current_line().to_owned();
                return Err(make_err(&format!("Unexpected Trio content: {:?}", line)));
            }
        }

        if has_tags { Ok(Some(dict)) } else { Ok(None) }
    }

    /// Reads all dicts from the input and returns them as a `Vec`.
    pub fn read_all_dicts(&mut self) -> Result<Vec<Dict>, Error> {
        let mut dicts = Vec::new();
        while let Some(dict) = self.read_dict()? {
            dicts.push(dict);
        }
        Ok(dicts)
    }

    /// Reads all dicts from the input and returns them as a [`Grid`].
    pub fn read_grid(&mut self) -> Result<Grid, Error> {
        let dicts = self.read_all_dicts()?;
        Ok(Grid::make_from_dicts(dicts))
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn is_eof(&self) -> bool {
        self.current.is_none()
    }

    fn current_line(&self) -> &str {
        self.current.as_deref().unwrap_or("")
    }

    fn advance(&mut self) {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => self.current = None, // EOF
            Ok(_) => {
                // Strip the trailing line ending (\n or \r\n).
                while line.ends_with(['\n', '\r']) {
                    line.pop();
                }
                self.current = Some(line);
            }
            Err(e) => {
                // Defer the error so it can be returned from `read_dict`.
                self.io_error = Some(e);
                self.current = None;
            }
        }
    }

    /// Parses one tag line and inserts the resulting key/value pair into `dict`.
    ///
    /// Grammar (simplified):
    /// ```text
    /// tag       ::= name ( ":" value )?
    /// value     ::= inline_value | multiline_str | multiline_list | multiline_zinc
    /// ```
    fn read_tag(&mut self, dict: &mut Dict) -> Result<(), Error> {
        // Clone before calling advance(), which overwrites `self.current`.
        let line = self.current_line().to_owned();
        self.advance();

        // Split "name : value" at the first colon.
        let (name, value_part) = match line.find(':') {
            Some(pos) => {
                let name = line[..pos].trim_end_matches([' ', '\t']).to_owned();
                let rest = line[pos + 1..].trim_start_matches([' ', '\t']).to_owned();
                (name, Some(rest))
            }
            None => (line.trim_end_matches([' ', '\t']).to_owned(), None),
        };

        // Sanity-check: tag name must start with a lowercase letter.
        if !name
            .chars()
            .next()
            .map(|c| c.is_ascii_lowercase())
            .unwrap_or(false)
        {
            return Err(make_err(&format!("Invalid tag name: {:?}", name)));
        }

        let value = match value_part.as_deref() {
            // No colon -> Marker.
            None => Value::make_marker(),

            // Colon with nothing after it -> multi-line string.
            Some("") => {
                let lines = self.read_indented_lines();
                Value::Str(Str::make(&lines.join("\n")))
            }

            // Colon followed by `[` (possibly with trailing whitespace) -> multi-line Zinc list.
            Some(rest) if rest.trim_end() == "[" => self.read_multiline_list()?,

            // Colon followed by `Zinc:` -> embedded Zinc value (commonly a Grid).
            Some(rest) if rest.starts_with("Zinc:") => {
                let lines = self.read_indented_lines();
                let content = lines.join("\n");
                // Wrap with Zinc nested-grid delimiters and parse.
                let zinc_str = format!("<<\n{}\n>>", content);
                zinc_from_str(&zinc_str)
                    .map_err(|e| make_err(&format!("Trio inline Zinc decode error: {}", e)))?
            }

            // Regular inline value: try Zinc, fall back to plain string.
            Some(rest) => parse_inline_value(rest),
        };

        dict.insert(name, value);
        Ok(())
    }

    /// Collects all consecutive indented lines, stripping their leading whitespace.
    ///
    /// Blank lines within the indented block are consumed from the stream and
    /// dropped from the result (not included in the returned `Vec`), matching
    /// the behaviour of the reference TypeScript implementation.  This means
    /// blank lines cannot be preserved inside multi-line Trio string values.
    fn read_indented_lines(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        loop {
            if self.is_eof() {
                break;
            }

            if is_blank_line(self.current_line()) {
                // Consume blank lines but do not include them in the output.
                self.advance();
                continue;
            }

            if is_indented_line(self.current_line()) {
                // Clone before advance() overwrites self.current.
                let stripped = self
                    .current_line()
                    .trim_start_matches([' ', '\t'])
                    .to_owned();
                self.advance();
                result.push(stripped);
            } else {
                // Non-indented, non-blank line -> end of block.
                break;
            }
        }

        result
    }

    /// Reads a multi-line Zinc list value.
    ///
    /// The caller has already confirmed the tag value is `[`.  This method
    /// collects the indented body lines (filtering `//` comments), joins them
    /// with spaces, prepends `[`, and parses the result as a Zinc value.
    fn read_multiline_list(&mut self) -> Result<Value, Error> {
        let mut lines = self.read_indented_lines();
        // Remove comment lines from the list body.
        lines.retain(|l| !l.trim_start().starts_with("//"));

        // Join with a space and prepend `[` (the closing `]` is in the body).
        let zinc_str = format!("[{}", lines.join(" "));

        zinc_from_str(&zinc_str)
            .map_err(|e| make_err(&format!("Trio multi-line list decode error: {}", e)))
    }
}

// ---------------------------------------------------------------------------
// String convenience constructors
// ---------------------------------------------------------------------------

impl TrioReader<Cursor<String>> {
    /// Creates a reader from a Trio-encoded `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libhaystack::encoding::trio::decode::TrioReader;
    ///
    /// let dicts = TrioReader::from_str("site\ndis: \"HQ\"")
    ///     .read_all_dicts()
    ///     .expect("ok");
    /// assert_eq!(dicts.len(), 1);
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: &str) -> Self {
        TrioReader::new(Cursor::new(input.to_owned()))
    }

    /// Reads all dicts from a Trio-encoded `&str`.
    pub fn dicts_from_str(input: &str) -> Result<Vec<Dict>, Error> {
        TrioReader::from_str(input).read_all_dicts()
    }

    /// Reads a [`Grid`] from a Trio-encoded `&str`.
    pub fn grid_from_str(input: &str) -> Result<Grid, Error> {
        TrioReader::from_str(input).read_grid()
    }
}

// ---------------------------------------------------------------------------
// Reader convenience constructor
// ---------------------------------------------------------------------------

impl<R: Read> TrioReader<BufReader<R>> {
    /// Creates a reader from any [`Read`] source (e.g. [`std::fs::File`]),
    /// wrapping it automatically in a [`BufReader`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use libhaystack::encoding::trio::decode::TrioReader;
    ///
    /// let file = File::open("my_defs.trio").expect("open");
    /// let dicts = TrioReader::from_reader(file)
    ///     .read_all_dicts()
    ///     .expect("parse");
    /// println!("{} dicts", dicts.len());
    /// ```
    pub fn from_reader(reader: R) -> Self {
        TrioReader::new(BufReader::new(reader))
    }
}

// ---------------------------------------------------------------------------
// Iterator
// ---------------------------------------------------------------------------

/// `TrioReader` implements [`Iterator`], yielding one `Result<Dict, Error>`
/// per entity.  Iteration stops at end-of-input (`None`) or short-circuits on
/// the first parse error, which lets callers use the full iterator adaptor API.
///
/// # Examples
///
/// ```
/// use libhaystack::encoding::trio::decode::TrioReader;
///
/// let input = "site\ndis: \"HQ\"\n---\nsite\ndis: \"Branch\"\n";
/// let dicts: Vec<_> = TrioReader::from_str(input)
///     .collect::<Result<Vec<_>, _>>()
///     .expect("parse ok");
/// assert_eq!(dicts.len(), 2);
/// ```
impl<R: BufRead> Iterator for TrioReader<R> {
    type Item = Result<Dict, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_dict() {
            Ok(Some(dict)) => Some(Ok(dict)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dict;
    use crate::haystack::encoding::zinc::decode::from_str as zinc_from_str;
    use crate::haystack::val::*;

    /// Decode a Trio string and return the first dict. Panics on any error.
    fn read_one(input: &str) -> Dict {
        TrioReader::from_str(input)
            .read_dict()
            .expect("no IO error")
            .expect("expected a dict")
    }

    fn trio_colls() -> &'static str {
        concat!(
            "// Trio\n",
            "type:list\n",
            "val:[1,2,3]\n",
            "\n",
            "---\n",
            "type:dict\n",
            "val:{ dis:\"Dict!\" foo}\n",
            "---\n",
            "type:grid\n",
            "val:Zinc:\n",
            "  ver:\"3.0\"\n",
            "  b,a\n",
            "  20,10\n",
        )
    }

    // -----------------------------------------------------------------------
    // Empty / edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_empty_returns_none() {
        let result = TrioReader::from_str("").read_dict().expect("no error");
        assert!(result.is_none(), "empty input should return None");
    }

    #[test]
    fn test_only_whitespace_returns_none() {
        let result = TrioReader::from_str("  \n  \n")
            .read_dict()
            .expect("no error");
        assert!(result.is_none());
    }

    // -----------------------------------------------------------------------
    // Marker
    // -----------------------------------------------------------------------

    #[test]
    fn test_marker_tag() {
        let d = read_one("marker");
        assert_eq!(d, dict! { "marker" => Value::make_marker() });
    }

    // -----------------------------------------------------------------------
    // Numeric values
    // -----------------------------------------------------------------------

    #[test]
    fn test_integer() {
        let d = read_one("num:1");
        assert_eq!(d, dict! { "num" => Value::make_int(1) });
    }

    #[test]
    fn test_integer_with_space() {
        let d = read_one("num: 1");
        assert_eq!(d, dict! { "num" => Value::make_int(1) });
    }

    #[test]
    fn test_number_with_unit() {
        let d = read_one("num:123.4m");
        let expected = zinc_from_str("123.4m").expect("zinc parse");
        assert_eq!(d.get("num"), Some(&expected));
    }

    // -----------------------------------------------------------------------
    // Boolean
    // -----------------------------------------------------------------------

    #[test]
    fn test_bool_true() {
        let d = read_one("bool:T");
        assert_eq!(d, dict! { "bool" => Value::make_bool(true) });
    }

    #[test]
    fn test_bool_false() {
        let d = read_one("bool:F");
        assert_eq!(d, dict! { "bool" => Value::make_bool(false) });
    }

    // -----------------------------------------------------------------------
    // Null
    // -----------------------------------------------------------------------

    #[test]
    fn test_null() {
        let d = read_one("nullVal:N");
        assert_eq!(d.get("nullVal"), Some(&Value::Null));
    }

    // -----------------------------------------------------------------------
    // Strings
    // -----------------------------------------------------------------------

    #[test]
    fn test_quoted_string() {
        let d = read_one(r#"str:"this is a string""#);
        assert_eq!(d, dict! { "str" => Value::make_str("this is a string") });
    }

    #[test]
    fn test_unquoted_safe_string() {
        let d = read_one("str: this is a string");
        assert_eq!(d, dict! { "str" => Value::make_str("this is a string") });
    }

    #[test]
    fn test_multiline_string_double_space() {
        let input = "str:\n  first line\n  second line\n  third line";
        let d = read_one(input);
        assert_eq!(
            d,
            dict! { "str" => Value::make_str("first line\nsecond line\nthird line") }
        );
    }

    #[test]
    fn test_multiline_string_tabs() {
        let input = "str:\n\tfirst line\n\tsecond line\n\tthird line";
        let d = read_one(input);
        assert_eq!(
            d,
            dict! { "str" => Value::make_str("first line\nsecond line\nthird line") }
        );
    }

    #[test]
    fn test_multiline_string_with_blank_lines() {
        let input = "str:\n\tfirst line\n\n \n\tsecond line\n\tthird line";
        let d = read_one(input);
        assert_eq!(
            d,
            dict! { "str" => Value::make_str("first line\nsecond line\nthird line") }
        );
    }

    #[test]
    fn test_empty_multiline_string() {
        let input = "str:\n\t";
        let d = read_one(input);
        assert_eq!(d, dict! { "str" => Value::make_str("") });
    }

    // -----------------------------------------------------------------------
    // URI / Ref / Symbol
    // -----------------------------------------------------------------------

    #[test]
    fn test_uri() {
        let d = read_one("uri:`/foo`");
        assert_eq!(d, dict! { "uri" => Value::make_uri("/foo") });
    }

    #[test]
    fn test_ref() {
        let d = read_one("ref:@foo");
        assert_eq!(d, dict! { "ref" => Value::make_ref("foo") });
    }

    #[test]
    fn test_symbol() {
        let d = read_one("sym:^foo");
        assert_eq!(d, dict! { "sym" => Value::make_symbol("foo") });
    }

    #[test]
    fn test_symbol_with_colon_in_value() {
        // `^lib:ph` contains a colon in the *value*, not a tag separator.
        let d = read_one("def:^lib:ph");
        assert_eq!(d, dict! { "def" => Value::make_symbol("lib:ph") });
    }

    // -----------------------------------------------------------------------
    // Collections
    // -----------------------------------------------------------------------

    #[test]
    fn test_dict_value() {
        let d = read_one("dict:{foo}");
        assert_eq!(
            d,
            dict! { "dict" => Value::make_dict(dict! { "foo" => Value::make_marker() }) }
        );
    }

    #[test]
    fn test_list_value() {
        let d = read_one("lst:[M]");
        assert_eq!(
            d,
            dict! { "lst" => Value::make_list(vec![Value::make_marker()]) }
        );
    }

    #[test]
    fn test_list_numbers() {
        let d = read_one("val:[1,2,3]");
        assert_eq!(
            d,
            dict! {
                "val" => Value::make_list(vec![
                    Value::make_int(1), Value::make_int(2), Value::make_int(3)
                ])
            }
        );
    }

    // -----------------------------------------------------------------------
    // Embedded Zinc grid
    // -----------------------------------------------------------------------

    #[test]
    fn test_zinc_grid_with_trailing_newline() {
        let input = concat!(
            "dict:Zinc:\n",
            "  ver:\"3.0\"\n",
            "  firstName,bday\n",
            "  \"Jack\",1973-07-23\n",
            "  \"Jill\",1975-11-15\n",
        );
        let d = read_one(input);
        let zinc_src = concat!(
            "ver:\"3.0\"\n",
            "firstName,bday\n",
            "\"Jack\",1973-07-23\n",
            "\"Jill\",1975-11-15\n",
        );
        let expected = zinc_from_str(zinc_src).expect("expected zinc grid");
        assert_eq!(d.get("dict"), Some(&expected));
    }

    #[test]
    fn test_zinc_grid_without_trailing_newline() {
        let input = concat!(
            "dict:Zinc:\n",
            "  ver:\"3.0\"\n",
            "  firstName,bday\n",
            "  \"Jack\",1973-07-23\n",
            "  \"Jill\",1975-11-15",
        );
        let d = read_one(input);
        let zinc_src = concat!(
            "ver:\"3.0\"\n",
            "firstName,bday\n",
            "\"Jack\",1973-07-23\n",
            "\"Jill\",1975-11-15\n",
        );
        let expected = zinc_from_str(zinc_src).expect("expected zinc grid");
        assert_eq!(d.get("dict"), Some(&expected));
    }

    // -----------------------------------------------------------------------
    // Multiple tags in one dict
    // -----------------------------------------------------------------------

    #[test]
    fn test_multiple_tags() {
        let d = read_one("marker\nstr: \"a string\"\nboo: T");
        assert_eq!(
            d,
            dict! {
                "boo" => Value::make_bool(true),
                "marker" => Value::make_marker(),
                "str" => Value::make_str("a string")
            }
        );
    }

    // -----------------------------------------------------------------------
    // Comments
    // -----------------------------------------------------------------------

    #[test]
    fn test_ignores_comment_line() {
        let d = read_one("// A comment\nfoo");
        assert_eq!(d, dict! { "foo" => Value::make_marker() });
    }

    #[test]
    fn test_skips_comment_block_before_dict() {
        let input = "// Test\n\n---------\ndef:^lib:ph";
        let d = read_one(input);
        assert_eq!(d, dict! { "def" => Value::make_symbol("lib:ph") });
    }

    // -----------------------------------------------------------------------
    // Multiple dicts / separators
    // -----------------------------------------------------------------------

    #[test]
    fn test_two_dicts_single_dash() {
        let mut reader = TrioReader::from_str("foo\n-\nboo");
        assert_eq!(
            reader.read_dict().expect("ok").expect("d1"),
            dict! { "foo" => Value::make_marker() }
        );
        assert_eq!(
            reader.read_dict().expect("ok").expect("d2"),
            dict! { "boo" => Value::make_marker() }
        );
        assert!(reader.read_dict().expect("ok").is_none());
    }

    #[test]
    fn test_two_dicts_many_dashes() {
        let mut reader = TrioReader::from_str("foo\n----------\nboo");
        assert_eq!(
            reader.read_dict().expect("ok").expect("d1"),
            dict! { "foo" => Value::make_marker() }
        );
        assert_eq!(
            reader.read_dict().expect("ok").expect("d2"),
            dict! { "boo" => Value::make_marker() }
        );
    }

    // -----------------------------------------------------------------------
    // Multi-line list
    // -----------------------------------------------------------------------

    #[test]
    fn test_multiline_list_of_dicts() {
        let input = "list: [\n  {this, is, one, dict},\n  {here, is, another},\n  ]";
        let d = read_one(input);
        let list = d.get_list("list").expect("list tag");
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_multiline_list_comment_filtered() {
        let input = "list: [\n  {first},\n  // this is a comment\n  {second},\n  ]";
        let d = read_one(input);
        let list = d.get_list("list").expect("list tag");
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_multiline_list_no_commas_with_comment() {
        let input =
            "list: [\n  {this is one dict},\n  // this is a comment\n  {here is another},\n  ]";
        let d = read_one(input);
        let list = d.get_list("list").expect("list tag");
        assert_eq!(list.len(), 2);
        let d0 = match &list[0] {
            Value::Dict(d) => d,
            _ => panic!("expected dict"),
        };
        assert!(d0.has("this") && d0.has("is") && d0.has("one") && d0.has("dict"));
        let d1 = match &list[1] {
            Value::Dict(d) => d,
            _ => panic!("expected dict"),
        };
        assert!(d1.has("here") && d1.has("is") && d1.has("another"));
    }

    // -----------------------------------------------------------------------
    // Collections variant (from Trio spec example)
    // -----------------------------------------------------------------------

    #[test]
    fn test_colls_list_dict() {
        let dicts = TrioReader::from_str(trio_colls())
            .read_all_dicts()
            .expect("dicts");
        assert_eq!(dicts.len(), 3);

        assert_eq!(dicts[0].get("type"), Some(&Value::make_str("list")));
        let list = dicts[0].get_list("val").expect("list");
        assert_eq!(list.len(), 3);

        assert_eq!(dicts[1].get("type"), Some(&Value::make_str("dict")));
        let inner = dicts[1].get_dict("val").expect("inner dict");
        assert!(inner.has("foo"));
        assert_eq!(
            inner.get_str("dis").map(|s| s.value.as_str()),
            Some("Dict!")
        );

        assert_eq!(dicts[2].get("type"), Some(&Value::make_str("grid")));
        let grid = dicts[2].get_grid("val").expect("grid");
        assert_eq!(grid.len(), 1);
    }

    // -----------------------------------------------------------------------
    // read_all_dicts / read_grid static helpers
    // -----------------------------------------------------------------------

    #[test]
    fn test_read_all_dicts() {
        let dicts = TrioReader::dicts_from_str(trio_colls()).expect("dicts");
        assert_eq!(dicts.len(), 3);
    }

    #[test]
    fn test_read_grid() {
        let grid = TrioReader::grid_from_str(trio_colls()).expect("grid");
        assert_eq!(grid.len(), 3);
    }

    // -----------------------------------------------------------------------
    // File-based tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_reads_axon_trio() {
        let content = include_str!("../../../../tests/fixtures/axon.trio");
        let dicts = TrioReader::from_str(content)
            .read_all_dicts()
            .expect("dicts");
        assert_eq!(dicts.len(), 3);
    }

    #[test]
    fn test_reads_points_and_equip_trio() {
        let content = include_str!("../../../../tests/fixtures/pointsAndEquip.trio");
        let dicts = TrioReader::from_str(content)
            .read_all_dicts()
            .expect("dicts");
        assert_eq!(dicts.len(), 1282);
    }

    // -----------------------------------------------------------------------
    // Iterator
    // -----------------------------------------------------------------------

    #[test]
    fn test_iterator_collects_all_dicts() {
        let result: Result<Vec<_>, _> = TrioReader::from_str(trio_colls()).collect();
        let dicts = result.expect("no parse errors");
        assert_eq!(dicts.len(), 3);
    }

    #[test]
    fn test_iterator_count() {
        assert_eq!(TrioReader::from_str(trio_colls()).count(), 3);
    }

    #[test]
    fn test_iterator_filter_map() {
        // Demonstrate a real iterator adaptor: keep only dicts with type=="list".
        let matches: Vec<_> = TrioReader::from_str(trio_colls())
            .filter_map(|r| {
                let d = r.expect("ok");
                if d.get("type") == Some(&Value::make_str("list")) {
                    Some(d)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(matches.len(), 1);
    }

    // -----------------------------------------------------------------------
    // from_reader
    // -----------------------------------------------------------------------

    #[test]
    fn test_from_reader() {
        // from_reader wraps any Read source in a BufReader automatically.
        let bytes = trio_colls().as_bytes();
        let dicts = TrioReader::from_reader(bytes)
            .read_all_dicts()
            .expect("dicts");
        assert_eq!(dicts.len(), 3);
    }
}
