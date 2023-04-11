// Copyright (C) 2020 - 2022, J2 Innovations

//! Scanner implementation

use std::io::{Error, ErrorKind, Read};

/// Zinc decoder scanner
pub struct Scanner<'a, R: Read> {
    input: &'a mut R,
    pub cur: u8,
    pub(super) next: Option<Vec<u8>>,
    pub(super) last_peek: u8,
    pub is_eof: bool,
    pub(super) pos: u64,
    line: usize,
}

impl<'a, R: Read> Scanner<'a, R> {
    pub fn make(input: &'a mut R) -> Result<Self, Error> {
        let mut buf: [u8; 1] = [0];

        if let Err(err) = input.read_exact(&mut buf) {
            if err.kind() == ErrorKind::UnexpectedEof {
                Ok(Scanner {
                    cur: buf[0],
                    input,
                    next: None,
                    last_peek: 0xFF,
                    is_eof: true,
                    pos: 0,
                    line: 1,
                })
            } else {
                Err(err)
            }
        } else {
            Ok(Scanner {
                cur: buf[0],
                input,
                next: None,
                last_peek: 0xFF,
                is_eof: false,
                pos: 0,
                line: 1,
            })
        }
    }

    /// Consumes all space characters until EOS or non space character is found
    pub fn consume_spaces(&mut self) -> Result<(), Error> {
        loop {
            if !self.is_space() {
                return Ok(());
            }

            if let Err(err) = self.read() {
                if self.is_eof {
                    return Ok(());
                } else {
                    return Err(err);
                }
            }
        }
    }

    /// Consumes all white space characters, including new lines
    pub fn consume_white_spaces(&mut self) -> Result<(), Error> {
        loop {
            if !self.is_white_space() {
                return Ok(());
            }

            if let Err(err) = self.read() {
                if self.is_eof {
                    return Ok(());
                } else {
                    return Err(err);
                }
            }
        }
    }

    /// True if cur is a space char
    pub fn is_space(&self) -> bool {
        " \t".as_bytes().contains(&self.cur)
    }

    /// True if cur is a new line char
    pub fn is_newline(&self) -> bool {
        "\r\n".as_bytes().contains(&self.cur)
    }

    /// True if cur is a new line or a space char
    pub fn is_white_space(&self) -> bool {
        self.is_space() || self.is_newline()
    }

    /// True if cur is a digit char
    pub fn is_digit(&self) -> bool {
        self.cur.is_ascii_digit()
    }

    /// True if cur is a hex digit char
    pub fn is_hex_digit(&self) -> bool {
        self.cur.is_ascii_hexdigit()
    }

    /// True if cur is an upper case char
    pub fn is_upper(&self) -> bool {
        self.cur.is_ascii_uppercase()
    }

    /// True if cur is an lower case char
    pub fn is_lower(&self) -> bool {
        self.cur.is_ascii_lowercase()
    }

    /// True if cur is an alpha char
    pub fn is_alpha(&self) -> bool {
        self.is_lower() || self.is_upper()
    }

    /// True if cur is an alpha numeric char
    pub fn is_alpha_num(&self) -> bool {
        self.is_digit() || self.is_lower() || self.is_upper()
    }

    /// True if cur is included in the provide `chars` string
    pub fn is_any_of(&self, chars: &str) -> bool {
        chars.as_bytes().contains(&self.cur)
    }

    /// True if cur is included in the `range`
    pub fn is_in_range(&self, range: &std::ops::RangeInclusive<u8>) -> bool {
        range.contains(&self.cur)
    }

    /// Expect current char to match the `expect`, and read next char
    pub fn expect_and_consume(&mut self, expect: u8) -> Result<u8, Error> {
        if self.cur == expect {
            let cur = self.cur;
            if let Err(err) = self.read() {
                if self.is_eof {
                    return Ok(cur);
                } else {
                    return Err(err);
                }
            }
            Ok(cur)
        } else {
            self.make_expect_err(expect as char)
        }
    }

    /// Expect current char to be included in the `expect` string, and read next char
    pub fn expect_and_consume_any_of(&mut self, expect: &str) -> Result<u8, Error> {
        if self.is_any_of(expect) {
            let cur = self.cur;
            if let Err(err) = self.read() {
                if self.is_eof {
                    return Ok(cur);
                } else {
                    return Err(err);
                }
            }
            Ok(cur)
        } else {
            self.make_expect_err(expect)
        }
    }

    /// Expect current char to be included in the `range`, and read next char
    pub fn expect_and_consume_any_in_range(
        &mut self,
        range: &std::ops::RangeInclusive<u8>,
    ) -> Result<u8, Error> {
        if self.is_in_range(range) {
            let cur = self.cur;
            if let Err(err) = self.read() {
                if self.is_eof {
                    return Ok(cur);
                } else {
                    return Err(err);
                }
            }
            Ok(cur)
        } else {
            self.make_generic_err(&format!(
                "Expected '{range:?}', found '{cur}'",
                cur = self.cur
            ))
        }
    }

    /// Expect the input to match the `expect` sting, advances the input until complete match or error
    pub fn expect_and_consume_seq(&mut self, seq: &str) -> Result<(), Error> {
        for (i, c) in seq.as_bytes().iter().enumerate() {
            if c != &self.cur {
                return self.make_expect_err(c);
            }
            if let Err(err) = self.read() {
                if self.is_eof && i == seq.len() - 1 {
                    return Ok(());
                } else {
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    /// Reads single char form input
    pub fn read(&mut self) -> Result<u8, Error> {
        if let Some(peek_bytes) = &mut self.next {
            self.cur = peek_bytes.remove(0);
            if peek_bytes.is_empty() {
                self.next = None;
            }

            self.increment_pos();
            Ok(self.cur)
        } else {
            match self.read_byte() {
                Ok(byte) => {
                    self.cur = byte;

                    self.increment_pos();
                    Ok(byte)
                }
                Err(err) => Err(err),
            }
        }
    }

    /// Advances the current input by one, errors out in case of IO Error.
    /// If current input is at EOS, silently ignore it and does nothing.
    pub fn advance(&mut self) -> Result<(), Error> {
        if let Err(err) = self.read() {
            if !self.is_eof {
                Err(err)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    /// Consume input until EOS or until the limit
    pub fn advance_by(&mut self, limit: u64) -> Result<u8, Error> {
        for _ in 0..limit {
            self.read()?;
        }
        Ok(self.cur)
    }

    /// Peeks for next element, errors out if end of stream or other read error
    /// is encountered.
    /// Each peek call will stash the value in an internal buffer, so a read call
    /// will consume the stashed data, then when stash is consumed, data is read from
    /// the stream again.
    pub fn peek(&mut self) -> Result<u8, Error> {
        let next = self.read_byte()?;

        Ok(self.buffer_peek(next))
    }

    /// Tries to peek any available data, returns `None`
    /// if there is any error while reading ahead
    /// See [peek](self::Scanner::peek)
    pub fn safe_peek(&mut self) -> Option<u8> {
        let next = match self.read_byte() {
            Ok(val) => val,
            Err(_) => return None,
        };

        Some(self.buffer_peek(next))
    }

    /// Create am error for an missing expect condition
    /// Current position and the current char are captured
    pub fn make_expect_err<T, E: std::fmt::Display>(&self, item: E) -> Result<T, Error> {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Scanner expected '{item}', found {cur:?}. Input position: {pos}[{look_ahead}], line {line}",
                cur = self.cur as char,
                pos = self.pos,
                look_ahead = self.next.as_ref().map_or(0, |b| b.len() as u64),
                line = self.line
            ),
        ))
    }

    /// Create a generic error
    /// Current position is captured
    pub fn make_generic_err<T>(&self, msg: &str) -> Result<T, Error> {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "{msg} Input position: {pos}[{look_ahead}], line {line}",
                pos = self.pos,
                look_ahead = self.next.as_ref().map_or(0, |b| b.len() as u64),
                line = self.line
            ),
        ))
    }

    /// Increment current input position and line number
    fn increment_pos(&mut self) {
        self.pos += 1;
        if self.is_newline() {
            self.line += 1;
        }
    }

    fn buffer_peek(&mut self, next: u8) -> u8 {
        if self.next.is_none() {
            self.next = Some(Vec::new());
        }
        if let Some(peek_bytes) = &mut self.next {
            peek_bytes.push(next);
            self.last_peek = next;
        }
        next
    }

    fn read_byte(&mut self) -> Result<u8, Error> {
        let mut buf: [u8; 1] = [0];
        match self.input.read_exact(&mut buf) {
            Ok(_) => Ok(buf[0]),
            Err(err) => {
                if err.kind() == ErrorKind::UnexpectedEof {
                    self.is_eof = true;
                }
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    #[test]
    fn test_zinc_scanner_empty() {
        let mut input = Cursor::new("".as_bytes());

        let scanner = super::Scanner::make(&mut input).expect("Scanner");

        assert!(scanner.is_eof, "Should be eof");
    }

    #[test]
    fn test_zinc_scanner_consume_spaces() {
        let mut input = Cursor::new("   abc  def".as_bytes());

        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        scanner.consume_spaces().expect("Valid");

        assert_eq!(scanner.cur, b'a');

        assert!(
            scanner.consume_spaces().is_ok(),
            "Should not consume spaces"
        );
        assert_eq!(scanner.cur, b'a');

        assert_eq!(scanner.advance_by(3).expect("Space"), b' ');

        scanner.consume_spaces().expect("Valid");

        assert_eq!(scanner.cur, b'd');
    }

    #[test]
    fn test_zinc_scanner_peek() {
        let mut input = Cursor::new("123".as_bytes());

        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        scanner.consume_spaces().expect("Valid");

        assert_eq!(scanner.cur, b'1');

        assert_eq!(scanner.peek().ok(), Some(b'2'));
        assert_eq!(scanner.cur, b'1');

        assert_eq!(scanner.peek().ok(), Some(b'3'));
        assert_eq!(scanner.cur, b'1');

        assert_eq!(scanner.read().ok(), Some(b'2'));
        assert_eq!(scanner.cur, b'2');

        assert_eq!(scanner.read().ok(), Some(b'3'));
        assert_eq!(scanner.cur, b'3');

        assert!(scanner.peek().is_err());
    }

    #[test]
    fn test_zinc_scanner_is_any_of() {
        let mut input = Cursor::new("8".as_bytes());

        let scanner = super::Scanner::make(&mut input).expect("Scanner");

        assert!(scanner.is_any_of("8abc"));
    }

    #[test]
    fn test_zinc_scanner_expect_seq() {
        let mut input = Cursor::new("abcDEF".as_bytes());

        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        assert_eq!(scanner.expect_and_consume_seq("abc").ok(), Some(()));
        assert_eq!(scanner.expect_and_consume_seq("DEF").ok(), Some(()));

        assert!(scanner.expect_and_consume_seq("123").is_err());
        assert!(scanner.expect_and_consume_seq("Fabc").is_err());
    }

    #[test]
    fn test_zinc_scanner_alpha_num_hex_space() {
        let mut input = Cursor::new("aB1 \t".as_bytes());

        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        assert_eq!(scanner.cur, b'a');

        assert!(scanner.is_alpha());
        assert!(scanner.is_lower());
        assert!(scanner.is_hex_digit());
        assert!(!scanner.is_digit());

        assert!(scanner.read().is_ok());
        assert_eq!(scanner.cur, b'B');
        assert!(scanner.is_alpha());
        assert!(scanner.is_upper());
        assert!(!scanner.is_lower());
        assert!(scanner.is_hex_digit());
        assert!(!scanner.is_digit());

        assert!(scanner.read().is_ok());
        assert_eq!(scanner.cur, b'1');
        assert!(!scanner.is_alpha());
        assert!(!scanner.is_upper());
        assert!(!scanner.is_lower());
        assert!(scanner.is_hex_digit());
        assert!(scanner.is_digit());

        assert!(scanner.read().is_ok());
        assert_eq!(scanner.cur, b' ');
        assert!(scanner.is_space());
        assert!(!scanner.is_alpha());
        assert!(!scanner.is_upper());
        assert!(!scanner.is_lower());
        assert!(!scanner.is_hex_digit());
        assert!(!scanner.is_digit());

        assert!(scanner.read().is_ok());
        assert_eq!(scanner.cur, b'\t');
        assert!(scanner.is_space());
        assert!(!scanner.is_alpha());
        assert!(!scanner.is_upper());
        assert!(!scanner.is_lower());
        assert!(!scanner.is_hex_digit());
        assert!(!scanner.is_digit());

        assert!(scanner.read().is_err());
        assert!(scanner.is_eof);
    }
}
