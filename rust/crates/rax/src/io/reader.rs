use std::io::BufRead;

/// Trait representing a line-oriented reader.
///
/// This trait abstracts over sources that can provide lines of text,
/// allowing reading one line at a time or a batch of lines.
pub trait IRaxReader {
    /// Reads a single line from the underlying source.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(String))` if a line was successfully read.
    /// - `Ok(None)` if the end of the input has been reached (EOF).
    /// - `Err(std::io::Error)` if an I/O error occurs while reading.
    fn read_line(&mut self) -> Result<Option<String>, std::io::Error>;

    /// Reads up to `count` lines from the underlying source.
    ///
    /// Stops reading early if the end of the input is reached.
    ///
    /// # Arguments
    ///
    /// - `count`: Maximum number of lines to read.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the lines that were read (may be less than
    /// `count` if EOF is reached), or an `std::io::Error` if an I/O error
    /// occurs.
    fn read_lines_by_count(&mut self, count: usize) -> Result<Vec<String>, std::io::Error>;
}

/// A buffered line reader that implements `IRaxReader`.
///
/// Wraps a `BufRead` and provides utilities for reading lines individually
/// or in batches, with optional debug logging.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RaxReader<R: BufRead> {
    /// The inner buffered reader.
    inner: R,
    /// Internal buffer reused for reading lines.
    buf: String,
}

impl<R: BufRead> RaxReader<R> {
    /// Creates a new `RaxReader` from a type implementing `BufRead`.
    ///
    /// # Arguments
    ///
    /// - `inner`: The buffered reader to wrap.
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buf: String::new(),
        }
    }
}

impl<R: BufRead> IRaxReader for RaxReader<R> {
    /// Reads a single line from the inner reader.
    ///
    /// Clears the internal buffer before reading to ensure no leftover content.
    /// Returns `None` at EOF.
    ///
    /// Logs the number of bytes read and the content of the line if
    /// `clerk::debug!` logging is enabled.
    fn read_line(&mut self) -> Result<Option<String>, std::io::Error> {
        let mut buf = String::new();
        self.buf.clear();
        let n = self.inner.read_line(&mut buf)?;
        clerk::debug!(
            "[RaxReader] read_line: bytes read = {}, line = {:?}",
            n,
            buf
        );
        if n == 0 { Ok(None) } else { Ok(Some(buf)) }
    }

    /// Reads up to `count` lines from the inner reader.
    ///
    /// Stops early if EOF is reached.
    ///
    /// Logs each line read with its index (for debugging).
    fn read_lines_by_count(&mut self, count: usize) -> Result<Vec<String>, std::io::Error> {
        let mut lines = Vec::with_capacity(count);
        for _i in 0..count {
            match self.read_line()? {
                Some(line) => {
                    clerk::debug!(
                        "[RaxReader] read_lines_by_count: line {} = {:?}",
                        _i + 1,
                        line
                    );
                    lines.push(line)
                }
                None => break,
            }
        }
        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use clerk::{LogLevel, init_log_with_level};

    use super::*;

    #[test]
    fn test_read_line_some() {
        init_log_with_level(LogLevel::TRACE);
        // Test reading lines from a multi-line string
        let data = "hello\nworld\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let line1 = reader.read_line().unwrap();
        assert_eq!(line1, Some("hello\n".to_string()));
        let line2 = reader.read_line().unwrap();
        assert_eq!(line2, Some("world\n".to_string()));
        let line3 = reader.read_line().unwrap();
        assert_eq!(line3, None);
    }

    #[test]
    fn test_read_lines_by_count_less_than_available() {
        init_log_with_level(LogLevel::TRACE);
        // Test reading fewer lines than available
        let data = "a\nb\nc\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(2).unwrap();
        assert_eq!(lines, vec!["a\n".to_string(), "b\n".to_string()]);
    }

    #[test]
    fn test_read_lines_by_count_more_than_available() {
        init_log_with_level(LogLevel::TRACE);
        // Test reading more lines than available (should stop at EOF)
        let data = "x\ny\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(5).unwrap();
        assert_eq!(lines, vec!["x\n".to_string(), "y\n".to_string()]);
    }

    #[test]
    fn test_read_lines_by_count_zero() {
        init_log_with_level(LogLevel::TRACE);
        // Test reading zero lines (should return empty vec)
        let data = "foo\nbar\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(0).unwrap();
        assert!(lines.is_empty());
    }

    #[test]
    fn test_read_line_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        // Test reading from empty input (should return None)
        let data = "";
        let mut reader = RaxReader::new(Cursor::new(data));
        let line = reader.read_line().unwrap();
        assert_eq!(line, None);
    }

    #[test]
    fn test_read_lines_by_count_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        // Test reading lines from empty input (should return empty vec)
        let data = "";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(3).unwrap();
        assert!(lines.is_empty());
    }

    #[test]
    fn test_read_line_single_line_no_newline() {
        init_log_with_level(LogLevel::TRACE);
        // Test reading a single line without a trailing newline
        let data = "singleline";
        let mut reader = RaxReader::new(Cursor::new(data));
        let line = reader.read_line().unwrap();
        assert_eq!(line, Some("singleline".to_string()));
        let line2 = reader.read_line().unwrap();
        assert_eq!(line2, None);
    }

    #[test]
    fn test_read_lines_by_count_exact() {
        init_log_with_level(LogLevel::TRACE);
        // Test reading exactly the number of lines available
        let data = "1\n2\n3\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(3).unwrap();
        assert_eq!(
            lines,
            vec!["1\n".to_string(), "2\n".to_string(), "3\n".to_string()]
        );
    }

    #[test]
    fn test_read_lines_by_count_with_logging() {
        init_log_with_level(LogLevel::TRACE);
        // Test with logging enabled (output will be printed)
        let data = "log1\nlog2\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(2).unwrap();
        assert_eq!(lines, vec!["log1\n".to_string(), "log2\n".to_string()]);
    }
}
