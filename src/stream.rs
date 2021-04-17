use bytes::Bytes;
use futures_util::stream::BoxStream;
use tokio::{fs::File, io::AsyncRead};
use tokio_util::io::ReaderStream;
use url::Url;

use std::io;

pub struct Source;

impl Source {
    pub async fn from<R: AsyncRead>(
        from: Url,
    ) -> io::Result<BoxStream<'static, io::Result<Bytes>>> {
        match from.scheme() {
            "file" => {
                let file = File::open(from.path()).await?;
                Ok(Box::pin(ReaderStream::new(file)))
            }
            _ => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }
}
