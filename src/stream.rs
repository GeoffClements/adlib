use bytes::Bytes;
use futures_util::stream::{once, BoxStream, Stream, StreamExt};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use url::Url;

use crate::decode::Detecter;

use std::{fmt, io};

pub type AdlibStreamItem = BoxStream<'static, io::Result<AdlibMessage>>;

impl<T> AdlibStream for T where T: Stream {}

pub trait AdlibStream: Stream {
    fn detector(self) -> Detecter<Self>
    where
        Self: Sized,
    {
        Detecter::new(self)
    }
}

#[non_exhaustive]
pub enum AdlibMessage {
    Data(Bytes),
    EndOfStream,
}

pub struct Source;

impl Source {
    pub async fn from(from: Url) -> io::Result<AdlibStreamItem> {
        let strm = match from.scheme() {
            "file" => {
                let file = File::open(from.path()).await?;
                ReaderStream::new(file)
            }
            _ => return Err(io::Error::from(io::ErrorKind::NotFound)),
        };

        let eos = once(async { Ok(AdlibMessage::EndOfStream) });
        Ok(strm
            .map(|m| match m {
                Ok(m) => Ok(AdlibMessage::Data(m)),
                Err(e) => Err(e),
            })
            .chain(eos)
            .boxed())
    }

    // pub async fn decoder() -> Option<dyn Decoder> {}
}

impl fmt::Debug for AdlibMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdlibMessage::Data(d) => f.debug_struct("Data").field("Bytes", &d.len()).finish(),
            AdlibMessage::EndOfStream => f.debug_struct("End Of Stream").finish(),
        }
    }
}
