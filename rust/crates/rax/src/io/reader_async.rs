use async_trait::async_trait;
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// Async counterpart of `IRaxReader`.
///
/// Provides asynchronous line-oriented reading from a source,
/// allowing reading one line at a time or a batch of lines.
#[async_trait]
pub trait AsyncIRaxReader {
    /// Reads a single line from the underlying async source.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(String))` if a line was successfully read.
    /// - `Ok(None)` if the end of the input has been reached (EOF).
    /// - `Err(std::io::Error)` if an I/O error occurs while reading.
    async fn read_line(&mut self) -> Result<Option<String>, std::io::Error>;

    /// Reads up to `count` lines from the underlying async source.
    ///
    /// Stops early if the end of the input is reached.
    ///
    /// # Arguments
    ///
    /// - `count`: Maximum number of lines to read.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the lines that were read (may be fewer than
    /// `count` if EOF is reached), or an `std::io::Error` if an I/O error
    /// occurs.
    async fn read_lines_by_count(&mut self, count: usize) -> Result<Vec<String>, std::io::Error>;
}

/// A buffered asynchronous line reader implementing `AsyncIRaxReader`.
///
/// Wraps a type implementing `AsyncBufRead` and provides utilities for
/// reading lines individually or in batches, with optional debug logging.
pub struct AsyncRaxReader<R: AsyncBufRead + Unpin> {
    /// The inner async buffered reader.
    inner: R,
}

impl<R: AsyncBufRead + Unpin> AsyncRaxReader<R> {
    /// Creates a new `AsyncRaxReader` from a type implementing `AsyncBufRead`.
    ///
    /// # Arguments
    ///
    /// - `inner`: The async buffered reader to wrap.
    pub fn new(inner: R) -> Self { Self { inner } }
}

#[async_trait]
impl<R> AsyncIRaxReader for AsyncRaxReader<R>
where
    R: AsyncBufRead + Unpin + Send, // `Send` ensures safe crossing of await points
{
    /// Reads a single line asynchronously from the inner reader.
    ///
    /// Logs the number of bytes read and the content of the line if
    /// `clerk::debug!` logging is enabled.
    async fn read_line(&mut self) -> Result<Option<String>, std::io::Error> {
        let mut buf = String::new();
        let n = self.inner.read_line(&mut buf).await?;
        clerk::debug!(
            "[AsyncRaxReader] read_line: bytes read = {}, line = {:?}",
            n,
            buf
        );
        Ok((n > 0).then_some(buf))
    }

    /// Reads up to `count` lines asynchronously from the inner reader.
    ///
    /// Stops early if EOF is reached.
    ///
    /// Logs each line read with its index (for debugging).
    async fn read_lines_by_count(&mut self, count: usize) -> Result<Vec<String>, std::io::Error> {
        let mut lines = Vec::with_capacity(count);
        for i in 0..count {
            match self.read_line().await? {
                Some(line) => {
                    clerk::debug!(
                        "[AsyncRaxReader] read_lines_by_count: line {} = {:?}",
                        i + 1,
                        line
                    );
                    lines.push(line);
                }
                None => break,
            }
        }
        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use clerk::{LogLevel, init_log_with_level};
    use tokio::io::BufReader;

    use super::*;

    // Helper to create a BufReader from a string for async tests
    fn make_reader(data: &str) -> AsyncRaxReader<BufReader<&[u8]>> {
        AsyncRaxReader::new(BufReader::new(data.as_bytes()))
    }

    #[tokio::test]
    async fn test_read_line_some() {
        init_log_with_level(LogLevel::TRACE);
        let mut reader = make_reader("foo\nbar\n");
        let line1 = reader.read_line().await.unwrap();
        assert_eq!(line1.as_deref(), Some("foo\n"));
        let line2 = reader.read_line().await.unwrap();
        assert_eq!(line2.as_deref(), Some("bar\n"));
        let line3 = reader.read_line().await.unwrap();
        assert_eq!(line3, None);
    }

    #[tokio::test]
    async fn test_read_lines_by_count_partial() {
        init_log_with_level(LogLevel::TRACE);
        let mut reader = make_reader("a\nb\nc\n");
        let lines = reader.read_lines_by_count(2).await.unwrap();
        assert_eq!(lines, vec!["a\n".to_string(), "b\n".to_string()]);
        let lines = reader.read_lines_by_count(2).await.unwrap();
        assert_eq!(lines, vec!["c\n".to_string()]);
        let lines = reader.read_lines_by_count(1).await.unwrap();
        assert!(lines.is_empty());
    }

    #[tokio::test]
    async fn test_read_lines_by_count_empty() {
        init_log_with_level(LogLevel::TRACE);
        let mut reader = make_reader("");
        let lines = reader.read_lines_by_count(3).await.unwrap();
        assert!(lines.is_empty());
    }
}
