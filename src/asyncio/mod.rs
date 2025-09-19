use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncWrite;

pub struct AsyncToSyncWrite<W>
where
    W: FnMut(&[u8]),
{
    func: W,
}

impl<W> AsyncToSyncWrite<W>
where
    W: FnMut(&[u8]),
{
    pub fn new(func: W) -> AsyncToSyncWrite<W> {
        AsyncToSyncWrite { func }
    }
}

impl<W> AsyncWrite for AsyncToSyncWrite<W>
where
    W: FnMut(&[u8]) + Unpin,
{
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
        let this = self.get_mut();
        (this.func)(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}

// test
#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[tokio::test]
    async fn test_async_to_sync_write() {
        let data = b"Hello, world!";
        let mut reader = Cursor::new(data);
        let mut collected_data = Vec::new();
        let mut w = AsyncToSyncWrite::new(|data: &[u8]| {
            collected_data.extend_from_slice(data);
        });
        tokio::io::copy(&mut reader, &mut w).await.unwrap();

        assert_eq!(collected_data, data);
    }
}
