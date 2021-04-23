use bytes::BytesMut;
use tokio_util::codec::Decoder;

use std::io::{self, ErrorKind};

const FLACMAGIC: &[u8] = b"fLaC";
const OGGMAGIC: &[u8] = b"OggS";
const MAGICS: &[&[u8]] = &[FLACMAGIC, OGGMAGIC];

const MAXBUF: usize = 256 * 1024;

pub enum DataFrame {
    Flac(BytesMut),
}

enum DecodeHints {
    Flac,
}

#[derive(Default)]
pub struct DataFrameDecoder {
    hint: Option<DecodeHints>,
}

impl Decoder for DataFrameDecoder {
    type Item = DataFrame;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        self.hint = if self.hint.is_none() {
            if src.starts_with(FLACMAGIC) {
                Some(DecodeHints::Flac)
            } else {
                None
            }
        } else {
            None
        };

        if src.len() > MAXBUF {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("Frame of length is too large."),
            ));
        }

        Ok(None)
    }
}
