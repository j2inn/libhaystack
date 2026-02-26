// Copyright (C) 2020 - 2024, J2 Innovations

//! Trio format encoder (writer)
//!
//! Encodes Haystack [`Dict`] / [`Grid`] values as Trio text.
//!
//! See <https://project-haystack.org/doc/docHaystack/Trio>

use crate::haystack::encoding::zinc::encode::ToZinc;
use crate::haystack::val::{Dict, Grid, Value};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// The two-space indentation used for embedded Zinc blocks and multi-line strings.
const INDENT: &str = "  ";

/// A newline followed by [`INDENT`], pre-computed to avoid repeated heap
/// allocations inside tight encoding loops.
const NL_INDENT: &str = "\n  ";

// ---------------------------------------------------------------------------
// Options
// ---------------------------------------------------------------------------

/// Additional options that control Trio output.
#[derive(Debug, Clone, Copy)]
pub struct TrioWriterOptions {
    /// When `true`, string values are written using the indented multi-line
    /// format instead of a single-line quoted Zinc string.
    pub multiline_strings: bool,
}

// ---------------------------------------------------------------------------
// Internal document model
// ---------------------------------------------------------------------------

enum TrioPart {
    Dict(Dict),
    Comment(String),
    NewLine,
}

// ---------------------------------------------------------------------------
// TrioWriter
// ---------------------------------------------------------------------------

/// Builds a Trio-encoded document from a sequence of dicts, comments, and
/// newlines.
///
/// # Examples
///
/// ```
/// use libhaystack::encoding::trio::encode::TrioWriter;
/// use libhaystack::dict;
/// use libhaystack::val::*;
///
/// let mut writer = TrioWriter::new();
/// writer
///     .add_comment("My sites")
///     .add_dict(dict! { "site" => Value::make_marker(), "dis" => Value::make_str("HQ") });
/// let trio = writer.to_trio_string();
/// assert!(trio.contains("site"));
/// assert!(trio.contains("dis:"));
/// ```
pub struct TrioWriter {
    parts: Vec<TrioPart>,
    options: Option<TrioWriterOptions>,
}

impl Default for TrioWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl TrioWriter {
    /// Creates a new `TrioWriter` with default options.
    pub fn new() -> Self {
        TrioWriter {
            parts: Vec::new(),
            options: None,
        }
    }

    /// Creates a new `TrioWriter` with the supplied [`TrioWriterOptions`].
    pub fn with_options(options: TrioWriterOptions) -> Self {
        TrioWriter {
            parts: Vec::new(),
            options: Some(options),
        }
    }

    // -----------------------------------------------------------------------
    // Builder methods
    // -----------------------------------------------------------------------

    /// Appends a dict to the document.
    pub fn add_dict(&mut self, dict: Dict) -> &mut Self {
        self.parts.push(TrioPart::Dict(dict));
        self
    }

    /// Appends every row of a grid to the document as individual dicts.
    pub fn add_grid(&mut self, grid: &Grid) -> &mut Self {
        for row in &grid.rows {
            self.parts.push(TrioPart::Dict(row.clone()));
        }
        self
    }

    /// Appends a comment line (`// text`) to the document.
    ///
    /// An empty `text` produces a bare `//` line.
    pub fn add_comment(&mut self, text: &str) -> &mut Self {
        self.parts.push(TrioPart::Comment(text.to_string()));
        self
    }

    /// Appends a blank line to the document.
    pub fn add_newline(&mut self) -> &mut Self {
        self.parts.push(TrioPart::NewLine);
        self
    }

    // -----------------------------------------------------------------------
    // Serialisation
    // -----------------------------------------------------------------------

    /// Writes the complete Trio-encoded document to `writer`.
    ///
    /// Each part is separated by a single `'\n'`.  Dicts are terminated with
    /// `\n---`.  A [`TrioPart::NewLine`] item contributes only the preceding
    /// separator newline (i.e. it causes a blank line between adjacent parts
    /// without adding extra content).
    ///
    /// # Examples
    ///
    /// ```
    /// use libhaystack::encoding::trio::encode::TrioWriter;
    /// use libhaystack::dict;
    /// use libhaystack::val::*;
    ///
    /// let mut writer = TrioWriter::new();
    /// writer.add_dict(dict! { "site" => Value::make_marker() });
    ///
    /// let mut buf = Vec::new();
    /// writer.to_trio(&mut buf).expect("write ok");
    /// assert!(String::from_utf8(buf).unwrap().contains("site"));
    /// ```
    pub fn to_trio<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut first = true;

        for part in &self.parts {
            if !first {
                writer.write_all(b"\n")?;
            }
            first = false;

            match part {
                TrioPart::Comment(text) => {
                    if text.is_empty() {
                        writer.write_all(b"//")?;
                    } else {
                        writer.write_all(b"// ")?;
                        writer.write_all(text.as_bytes())?;
                    }
                }
                TrioPart::Dict(dict) => {
                    let s = Self::dict_to_trio(dict, self.options.as_ref());
                    writer.write_all(s.as_bytes())?;
                    writer.write_all(b"\n---")?;
                }
                // A NewLine item provides only the preceding '\n' separator.
                TrioPart::NewLine => {}
            }
        }

        Ok(())
    }

    /// Returns the complete Trio-encoded document as a `String`.
    ///
    /// This is a convenience wrapper around [`to_trio`][TrioWriter::to_trio] that
    /// buffers into a `Vec<u8>` and converts to `String`, mirroring the
    /// `to_zinc_string` convention in the Zinc encoder.
    ///
    /// # Examples
    ///
    /// ```
    /// use libhaystack::encoding::trio::encode::TrioWriter;
    /// use libhaystack::dict;
    /// use libhaystack::val::*;
    ///
    /// let mut writer = TrioWriter::new();
    /// writer.add_dict(dict! { "site" => Value::make_marker() });
    /// assert!(writer.to_trio_string().contains("site"));
    /// ```
    pub fn to_trio_string(&self) -> String {
        let mut buf = Vec::new();
        self.to_trio(&mut buf)
            .expect("writing Trio to Vec<u8> cannot fail");
        String::from_utf8(buf).expect("Trio output is always valid UTF-8")
    }

    /// Encodes a single [`Dict`] as a Trio string (without an entity separator).
    ///
    /// Each tag occupies one line:
    ///
    /// | Value type | Encoding |
    /// |---|---|
    /// | [`Marker`][crate::val::Marker] | `tagName` |
    /// | [`Grid`][crate::val::Grid] | `tagName:Zinc:\n  <indented zinc>` |
    /// | [`Str`][crate::val::Str] with `multiline_strings` | `tagName: \n  <indented lines>` |
    /// | everything else | `tagName: <zinc>` |
    pub fn dict_to_trio(dict: &Dict, options: Option<&TrioWriterOptions>) -> String {
        let multiline = options.is_some_and(|o| o.multiline_strings);

        dict.iter()
            .map(|(name, value)| encode_tag(name, value, multiline))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl std::fmt::Display for TrioWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_trio_string())
    }
}

// ---------------------------------------------------------------------------
// Tag encoding
// ---------------------------------------------------------------------------

/// Encodes a single tag key/value pair as a Trio line (or lines).
fn encode_tag(name: &str, value: &Value, multiline_strings: bool) -> String {
    match value {
        // Marker: just the tag name.
        Value::Marker => name.to_string(),

        // Grid: embedded Zinc block.
        Value::Grid(grid) => {
            let zinc = grid
                .to_zinc_string()
                .expect("Grid Zinc encoding should never fail for well-formed data");
            // Strip trailing whitespace/newlines and indent every line.
            let indented = zinc.trim().replace('\n', NL_INDENT);
            format!("{}:Zinc:\n{}{}", name, INDENT, indented)
        }

        // String with multiline option: indented multi-line format.
        Value::Str(s) if multiline_strings => {
            let indented = s.value.replace('\n', NL_INDENT);
            format!("{}: \n{}{}", name, INDENT, indented)
        }

        // Everything else: single-line Zinc encoding.
        _ => {
            // to_zinc_string only fails for Grid (handled above); all other
            // value types are infallible.
            let zinc = value
                .to_zinc_string()
                .expect("Zinc encoding should never fail for scalar/collection values");
            format!("{}: {}", name, zinc)
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
    use crate::haystack::val::*;

    /// Creates the standard test dict: marker, str, list, inner-dict.
    /// BTreeMap alphabetical order: dict, list, marker, str
    fn make_dict() -> Dict {
        dict! {
            "marker" => Value::make_marker(),
            "str"    => Value::make_str("A string"),
            "list"   => Value::make_list(vec![Value::make_bool(true)]),
            "dict"   => Value::make_dict(dict! { "foo" => Value::make_str("bar") })
        }
    }

    /// Expected lines for `make_dict()` without a trailing separator.
    const MAKE_DICT_TRIO: &str = concat!(
        "dict: {foo:\"bar\"}\n",
        "list: [T]\n",
        "marker\n",
        "str: \"A string\"",
    );

    fn to_str(writer: &TrioWriter) -> String {
        writer.to_trio_string()
    }

    // -----------------------------------------------------------------------
    // add_dict
    // -----------------------------------------------------------------------

    #[test]
    fn test_add_dict() {
        let mut writer = TrioWriter::new();
        writer.add_dict(make_dict());
        let expected = format!("{}\n---", MAKE_DICT_TRIO);
        assert_eq!(to_str(&writer), expected);
    }

    #[test]
    fn test_to_trio_writer() {
        // Verify to_trio() writes the same bytes as to_trio_string().
        let mut writer = TrioWriter::new();
        writer.add_dict(make_dict());
        let mut buf = Vec::new();
        writer.to_trio(&mut buf).expect("write ok");
        assert_eq!(String::from_utf8(buf).unwrap(), to_str(&writer));
    }

    #[test]
    fn test_add_dict_with_inner_grid() {
        let mut writer = TrioWriter::new();
        let dict = dict! {
            "grid" => Value::make_grid_from_dicts(vec![
                dict! { "foo" => Value::make_str("bar"), "boo" => Value::make_int(1) }
            ])
        };
        writer.add_dict(dict);
        // Grid columns are sorted alphabetically: boo, foo
        let expected = concat!(
            "grid:Zinc:\n",
            "  ver:\"3.0\"\n",
            "  boo,foo\n",
            "  1,\"bar\"\n",
            "---"
        );
        assert_eq!(to_str(&writer), expected);
    }

    // -----------------------------------------------------------------------
    // add_grid
    // -----------------------------------------------------------------------

    #[test]
    fn test_add_grid_writes_all_rows() {
        let mut writer = TrioWriter::new();
        let grid = Grid::make_from_dicts(vec![make_dict(), make_dict()]);
        writer.add_grid(&grid);
        let expected = format!("{}\n---\n{}\n---", MAKE_DICT_TRIO, MAKE_DICT_TRIO);
        assert_eq!(to_str(&writer), expected);
    }

    // -----------------------------------------------------------------------
    // add_comment
    // -----------------------------------------------------------------------

    #[test]
    fn test_add_comment_single() {
        let mut writer = TrioWriter::new();
        writer.add_comment("Hello");
        assert_eq!(to_str(&writer), "// Hello");
    }

    #[test]
    fn test_add_comment_empty() {
        let mut writer = TrioWriter::new();
        writer.add_comment("");
        assert_eq!(to_str(&writer), "//");
    }

    #[test]
    fn test_add_comment_multiple() {
        let mut writer = TrioWriter::new();
        writer
            .add_comment("This is a comment")
            .add_comment("This is another comment");
        assert_eq!(
            to_str(&writer),
            "// This is a comment\n// This is another comment"
        );
    }

    // -----------------------------------------------------------------------
    // add_newline
    // -----------------------------------------------------------------------

    #[test]
    fn test_add_newline_between_comments() {
        let mut writer = TrioWriter::new();
        writer
            .add_comment("This is a comment")
            .add_newline()
            .add_comment("This is another comment");
        assert_eq!(
            to_str(&writer),
            "// This is a comment\n\n// This is another comment"
        );
    }

    // -----------------------------------------------------------------------
    // to_trio (full document)
    // -----------------------------------------------------------------------

    #[test]
    fn test_full_document() {
        let mut writer = TrioWriter::new();
        writer
            .add_comment("")
            .add_comment("Copyright J2 Innovations")
            .add_comment("")
            .add_newline()
            .add_dict(make_dict())
            .add_newline()
            .add_comment("The second dict...")
            .add_dict(make_dict());

        let expected = format!(
            concat!(
                "//\n",
                "// Copyright J2 Innovations\n",
                "//\n",
                "\n",
                "{}\n---\n",
                "\n",
                "// The second dict...\n",
                "{}\n---"
            ),
            MAKE_DICT_TRIO, MAKE_DICT_TRIO
        );
        assert_eq!(to_str(&writer), expected);
    }

    // -----------------------------------------------------------------------
    // dict_to_trio
    // -----------------------------------------------------------------------

    #[test]
    fn test_dict_to_trio_basic() {
        let trio = TrioWriter::dict_to_trio(&make_dict(), None);
        assert_eq!(trio, MAKE_DICT_TRIO);
    }

    #[test]
    fn test_dict_to_trio_multiline_string() {
        // BTreeMap order: bool, num, str
        let dict = dict! {
            "bool" => Value::make_bool(true),
            "num"  => Value::make_number(42.0),
            "str"  => Value::make_str("{\n  \"foo\": \"bar\"\n}")
        };
        let opts = TrioWriterOptions {
            multiline_strings: true,
        };
        let trio = TrioWriter::dict_to_trio(&dict, Some(&opts));
        let expected = concat!(
            "bool: T\n",
            "num: 42\n",
            "str: \n",
            "  {\n",
            "    \"foo\": \"bar\"\n",
            "  }",
        );
        assert_eq!(trio, expected);
    }

    #[test]
    fn test_dict_to_trio_multiline_empty_string() {
        let dict = dict! {
            "bool" => Value::make_bool(true),
            "num"  => Value::make_number(42.0),
            "str"  => Value::make_str("")
        };
        let opts = TrioWriterOptions {
            multiline_strings: true,
        };
        let trio = TrioWriter::dict_to_trio(&dict, Some(&opts));
        assert_eq!(trio, "bool: T\nnum: 42\nstr: \n  ");
    }

    #[test]
    fn test_dict_to_trio_null_value() {
        let dict = dict! { "nothing" => Value::Null };
        let trio = TrioWriter::dict_to_trio(&dict, None);
        assert_eq!(trio, "nothing: N");
    }

    #[test]
    fn test_dict_to_trio_na_value() {
        let dict = dict! { "na" => Value::make_na() };
        let trio = TrioWriter::dict_to_trio(&dict, None);
        assert_eq!(trio, "na: NA");
    }

    // -----------------------------------------------------------------------
    // Display / to_string
    // -----------------------------------------------------------------------

    #[test]
    fn test_to_string_delegates_to_to_trio_string() {
        let mut writer = TrioWriter::new();
        writer.add_comment("test");
        assert_eq!(writer.to_string(), writer.to_trio_string());
    }
}
