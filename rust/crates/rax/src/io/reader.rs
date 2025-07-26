use std::io::BufRead;

use miette::IntoDiagnostic;

/// Trait for reading lines from a source.
pub trait IRaxReader {
    /// Reads the next line; returns `None` on EOF.
    fn read_line(&mut self) -> miette::Result<Option<String>>;
    /// Reads up to `count` lines, or until EOF.
    fn read_lines_by_count(&mut self, count: usize) -> miette::Result<Vec<String>>;
}

/// A buffered line reader that implements `IRaxReader`.
pub struct RaxReader<R: BufRead> {
    inner: R,
    buf: String,
}

impl<R: BufRead> RaxReader<R> {
    /// Create a new `RaxReader` from a type implementing `BufRead`.
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buf: String::new(),
        }
    }
}

impl<R: BufRead> IRaxReader for RaxReader<R> {
    /// Reads a single line from the inner reader.
    /// Returns `Ok(Some(line))` if a line is read, or `Ok(None)` on EOF.
    fn read_line(&mut self) -> miette::Result<Option<String>> {
        let mut buf = String::new();
        self.buf.clear();
        let n = self.inner.read_line(&mut buf).into_diagnostic()?;
        // Log the number of bytes read and the line content (for debugging)
        clerk::debug!(
            "[RaxReader] read_line: bytes read = {}, line = {:?}",
            n,
            buf
        );
        if n == 0 { Ok(None) } else { Ok(Some(buf)) }
    }

    /// Reads up to `count` lines from the inner reader.
    /// Stops early if EOF is reached.
    fn read_lines_by_count(&mut self, count: usize) -> miette::Result<Vec<String>> {
        let mut lines = Vec::with_capacity(count);
        for _i in 0..count {
            match self.read_line()? {
                Some(line) => {
                    // Log each line read (for debugging)
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
