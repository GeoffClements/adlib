use crate::stream::{AdlibMessage, AdlibStreamItem};
use bytes::BytesMut;
use futures_util::{Stream, StreamExt};
use tokio_util::{
    codec::{Decoder, FramedRead},
    io::StreamReader,
};

use std::io;

pub struct Detecter<St> {
    stream: St,
}

impl<St> Detecter<St>
where
    St: Stream<Item = io::Result<AdlibMessage>>,
{
    pub fn new(stream: St) -> Self {
        let stream = stream.filter_map(|m| async {
            if let Ok(AdlibMessage::Data(d)) = m {
                Some(Ok(d))
            } else {
                None
            }
        });
        let sr = StreamReader::new(stream);
        let frames = FramedRead::new(sr, DetectCodec);
        Self { frames }
    }
}

struct DetectCodec;

impl Decoder for DetectCodec {
    type Item = AdlibStreamItem;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(Box::pin(AdlibMessage::EndOfStream)))
    }
}
