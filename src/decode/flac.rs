use bytes::{Buf, BytesMut};
use claxon::{
    input::BufferedReader,
    metadata::{self, read_metadata_block, MetadataBlock},
};

use crate::{decode::DecodeResult, stream::StreamInfo};

pub(crate) fn start_of_flac_stream(src: &mut BytesMut) -> DecodeResult {
    const FLACMAGIC: &[u8] = b"fLaC";

    if src.len() < FLACMAGIC.len() {
        return DecodeResult::MoreData;
    }
    if src.starts_with(FLACMAGIC) {
        let _ = src.split_to(FLACMAGIC.len());
        return DecodeResult::FlacStream;
    }

    DecodeResult::Unrecognised
}

pub(crate) fn read_metadata_block_with_header(
    src: &mut BytesMut,
) -> (bool, Result<DecodeResult, claxon::Error>) {
    let blocklength = src[..4].as_ref().get_u32() & 0x00_FF_FF_FF;
    if (src.len() as u32) < blocklength + 4 {
        return (false, Ok(DecodeResult::MoreData));
    }

    let mut src = src.split_to((blocklength + 4) as usize);
    let meta_header = src[0];
    let block_type = meta_header & 0b0111_1111;
    let is_last_meta = (meta_header >> 7) == 1;
    let _ = src.split_to(4);
    let mut bufreader = BufferedReader::new(src.as_ref());

    match read_metadata_block(&mut bufreader, block_type, blocklength) {
        Ok(MetadataBlock::StreamInfo(metadata::StreamInfo {
            sample_rate,
            channels,
            bits_per_sample,
            ..
        })) => (
            is_last_meta,
            Ok(DecodeResult::StreamInfo(StreamInfo {
                sample_rate,
                channels,
                bits_per_sample,
            })),
        ),
        Ok(_) => (is_last_meta, Ok(DecodeResult::Unrecognised)),
        Err(e) => (false, Err(e)),
    }
}
