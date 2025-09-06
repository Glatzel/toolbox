use async_trait::async_trait;
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// Async counterpart of `IRaxReader`.
#[async_trait]
pub trait AsyncIRaxReader {
    async fn read_line(&mut self) -> mischief::Result<Option<String>>;
    async fn read_lines_by_count(&mut self, count: usize) -> mischief::Result<Vec<String>>;
}
/// Buffered async reader implementing `AsyncIRaxReader`.
pub struct AsyncRaxReader<R: AsyncBufRead + Unpin> {
    inner: R,
}

impl<R: AsyncBufRead + Unpin> AsyncRaxReader<R> {
    pub fn new(inner: R) -> Self { Self { inner } }
}

#[async_trait]
impl<R> AsyncIRaxReader for AsyncRaxReader<R>
where
    R: AsyncBufRead + Unpin + Send, // `Send` lets it cross await points safely
{
    async fn read_line(&mut self) -> mischief::Result<Option<String>> {
        let mut buf = String::new();
        let n = self.inner.read_line(&mut buf).await?;
        clerk::debug!(
            "[AsyncRaxReader] read_line: bytes read = {}, line = {:?}",
            n,
            buf
        );
        Ok((n > 0).then_some(buf))
    }

    async fn read_lines_by_count(&mut self, count: usize) -> mischief::Result<Vec<String>> {
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
