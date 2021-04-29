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
    Tags(Vec<(String, String)>),
}

#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum DecodeStates {
    FlacHeaders,
    FlacFrames,
    AnySync,
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
    Tags(Vec<(String, String)>),
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
                    DecodeResult::FlacStream => self.state = Some(DecodeStates::FlacHeaders),
                    DecodeResult::Unrecognised => self.state = Some(DecodeStates::AnySync),
                    _ => return Err(io::Error::from(ErrorKind::NotFound)),
                },

                Some(DecodeStates::FlacHeaders) => match read_metadata_block_with_header(src) {
                    (_, Ok(DecodeResult::MoreData)) => return Ok(None),
                    
                    (last, Ok(DecodeResult::StreamInfo(s))) => {
                        if last {
                            self.state = Some(DecodeStates::FlacFrames)
                        };
                        return Ok(Some(DataFrame::StreamInfo(s)));
                    }

                    (last, Ok(DecodeResult::Tags(v))) => {
                        if last {
                            self.state = Some(DecodeStates::FlacFrames)
                        };
                        return Ok(Some(DataFrame::Tags(v)));
                    }

                    (last, Ok(DecodeResult::Unrecognised)) => {
                        if last {
                            self.state = Some(DecodeStates::FlacFrames)
                        };
                        continue;
                    }

                    _ => return Err(io::Error::from(ErrorKind::InvalidData)),
                },

                Some(DecodeStates::FlacFrames) => {
                    todo!();
                }

                Some(DecodeStates::AnySync) => {
                    todo!();
                }
            }
        }
    }
}
