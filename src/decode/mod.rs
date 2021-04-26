mod flac;

use bytes::BytesMut;
use tokio_util::codec::Decoder;
// use claxon;

use crate::stream::StreamInfo;

use flac::{read_metadata_block_with_header, start_of_flac_stream};

use std::io::{self, ErrorKind};

// const OGGMAGIC: &[u8] = b"OggS";
// const MAGICS: &[&[u8]] = &[FLACMAGIC, OGGMAGIC];

const MAXBUF: usize = 128 * 1024;

#[derive(Debug)]
pub enum DataFrame {
    Flac(BytesMut),
    StreamInfo(StreamInfo),
}

#[derive(Debug)]
pub(crate) enum DecodeStates {
    FlacMetaBlock,
    FlacFrame,
}

#[derive(Default)]
pub struct DataFrameDecoder {
    state: Option<DecodeStates>,
}

pub(crate) enum DecodeResult {
    MoreData,
    Unrecognised,
    FlacStream,
    StreamInfo(StreamInfo),
}

impl Decoder for DataFrameDecoder {
    type Item = DataFrame;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // If buffer reaches max size discard it and start again
        if src.len() > MAXBUF {
            let _ = src.split();
            self.state = None;
            return Ok(None);
        }

        loop {
            match self.state {
                None => match start_of_flac_stream(src) {
                    DecodeResult::MoreData => return Ok(None),
                    DecodeResult::FlacStream => self.state = Some(DecodeStates::FlacMetaBlock),
                    DecodeResult::Unrecognised => continue,
                    _ => return Err(io::Error::from(ErrorKind::NotFound)),
                },

                Some(DecodeStates::FlacMetaBlock) => match read_metadata_block_with_header(src) {
                    (_, Ok(DecodeResult::MoreData)) => return Ok(None),
                    (last, Ok(DecodeResult::StreamInfo(s))) => {
                        if last {
                            self.state = Some(DecodeStates::FlacFrame)
                        };
                        return Ok(Some(DataFrame::StreamInfo(s)));
                    }
                    (last, Ok(DecodeResult::Unrecognised)) => {
                        if last {
                            self.state = Some(DecodeStates::FlacFrame)
                        };
                        continue;
                    }
                    _ => return Err(io::Error::from(ErrorKind::InvalidData)),
                },

                Some(DecodeStates::FlacFrame) => {
                    return Err(io::Error::from(ErrorKind::InvalidData))
                }
            }
        }

        // If we have magic set hint then look for and extract tags - return
        // self.hint = magic;

        // if we have hint look for and extract frame - return
        // If frame not found get more data - return

        // If no hint look for any frame and extract - return
        // If we find no frame get more data - return

        // Ok(None)
    }
}
