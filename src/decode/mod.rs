mod flac;

use bytes::BytesMut;
use tokio_util::codec::Decoder;
// use claxon;

use flac::{start_of_flac_stream, read_metadata_block_with_header};

use std::io;

// const OGGMAGIC: &[u8] = b"OggS";
// const MAGICS: &[&[u8]] = &[FLACMAGIC, OGGMAGIC];

const MAXBUF: usize = 128 * 1024;

pub enum DataFrame {
    Flac(BytesMut),
}

pub(crate) enum DecodeHints {
    FlacMetaBlock
}

#[derive(Default)]
pub struct DataFrameDecoder {
    hint: Option<DecodeHints>,
}

pub(crate) enum DecodeResult {
    MoreData,
    Unrecognised,
    FlacStream,
    FlacMetaBlock,
}

impl Decoder for DataFrameDecoder {
    type Item = DataFrame;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // If buffer reaches max size discard it and start again
        if src.len() > MAXBUF {
            let _ = src.split();
            return Ok(None);
        }

        // If no hint check if we are at the start of a flac stream
        if self.hint.is_none() {
            match start_of_flac_stream(src) {
                DecodeResult::MoreData => return Ok(None),
                DecodeResult::FlacStream => {
                    match read_metadata_block_with_header(src) {
                        DecodeResult::MoreData => return Ok(None),
                        DecodeResult::FlacMetaBlock => {
                            self.hint = Some(DecodeHints::FlacMetaBlock);
                            return Ok(Some(DataFrame::Flac))
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // If we have magic set hint then look for and extract tags - return
        // self.hint = magic;

        // if we have hint look for and extract frame - return
        // If frame not found get more data - return

        // If no hint look for any frame and extract - return
        // If we find no frame get more data - return

        Ok(None)
    }
}

