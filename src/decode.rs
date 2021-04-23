use bytes::BytesMut;
use tokio_util::codec::Decoder;
// use claxon;

use std::io::{self, ErrorKind};

const FLACMAGIC: &[u8] = b"fLaC";
// const OGGMAGIC: &[u8] = b"OggS";
// const MAGICS: &[&[u8]] = &[FLACMAGIC, OGGMAGIC];

const MAXBUF: usize = 128 * 1024;

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

        if src.len() > MAXBUF {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("Frame is too large."),
            ));
        }

        // If no hint check magic marker and set magic
        let magic = if self.hint.is_none() {
            match src {
                _ if src.starts_with(FLACMAGIC) => Some(DecodeHints::Flac),
                _ => None,
            }
        } else {
            None
        };

        // If we have magic set hint then look for and extract tags - return
        if let Some(magic) = magic {
            // look for and extract tags - return
            let magic_len = match magic {
                DecodeHints::Flac => FLACMAGIC.len(),
            };

            match magic {
                DecodeHints::Flac => {
                    if src.len() < magic_len + 32 {
                        return Ok(None);
                    }

                    // let metablock = claxon::metadata::read_metadata_block_with_header();
                }
            }
        }
        // self.hint = magic;

        // if we have hint look for and extract frame - return
        // If frame not found get more data - return

        // If no hint look for any frame and extract - return
        // If we find no frame get more data - return

        Ok(None)
    }
}
