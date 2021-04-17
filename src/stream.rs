use bytes::Bytes;
use futures_util::stream::{BoxStream, StreamExt};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use url::Url;

use std::{fmt, io};

#[non_exhaustive]
pub enum AdMessage {
    Data(Bytes),
}

pub struct Source;

impl Source {
    pub async fn from(from: Url) -> io::Result<BoxStream<'static, io::Result<AdMessage>>> {
        let strm = match from.scheme() {
            "file" => {
                let file = File::open(from.path()).await?;
                ReaderStream::new(file)
            }
            _ => return Err(io::Error::from(io::ErrorKind::NotFound)),
        };

        Ok(Box::pin(strm.map(|m| match m {
            Ok(m) => Ok(AdMessage::Data(m)),
            Err(e) => Err(e),
        })))
    }
}

impl fmt::Debug for AdMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdMessage::Data(d) => f
                .debug_struct("AdMessage")
                .field("Data. Length: ", &d.len())
                .finish(),
        }
    }
}
