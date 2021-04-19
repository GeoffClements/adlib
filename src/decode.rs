use claxon::FlacReader;
use futures_util::{future::ready, stream::StreamExt};

use crate::{AdlibMessage, AdlibStream};

use std::io::Read;

pub trait Decoder {}

pub struct MaybeDecoder;

impl MaybeDecoder {
    pub async fn new(s: AdlibStream) -> Option<Box<dyn Decoder>> {
        let s = s.filter_map(|b| {
            ready(match b {
                Ok(AdlibMessage::Data(d)) => Some(d),
                _ => None,
            })
        });

        FlacDecoder
    }
}

pub struct FlacDecoder<R: Read> {
    inner: FlacReader<R>,
}

// impl<R: Read> FlacDecoder<R> {
//     pub fn new(inner: R) -> Option<Self> {

//     }
// }

impl<R: Read> Decoder for FlacDecoder<R> {}
