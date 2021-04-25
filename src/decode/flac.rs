use bytes::{Buf, BytesMut};
use claxon::{input::BufferedReader, metadata::read_metadata_block};

use crate::decode::DecodeResult;

pub(crate) fn start_of_flac_stream(src: &mut BytesMut) -> DecodeResult {
    // Does not consume any buffer, see TODO: below
    const FLACMAGIC: &[u8] = b"fLaC";

    if src.len() < FLACMAGIC.len() {
        return DecodeResult::MoreData;
    }

    if src.starts_with(FLACMAGIC) {
        return DecodeResult::FlacStream;
    }

    DecodeResult::Unrecognised
}

pub(crate) fn read_metadata_block_with_header(src: &mut BytesMut) -> DecodeResult {
    // TODO: work out proper buffer consumption!!
    // Get enough data to decode a metadata block
    let blocklength = src[4..8].as_ref().get_u32() & 0x00_FF_FF_FF;
    if (src.len() as u32) < blocklength + 8 {
        return DecodeResult::MoreData;
    }
    let src = src.split_to(blocklength as usize);
    let meta_header = src[8];
    let block_type = meta_header & 0b0111_1111;
    let is_last_meta = (meta_header >> 7) == 1;
    let mut bufreader = BufferedReader::new(src.as_ref());
    match read_metadata_block(&mut bufreader, block_type, blocklength) {
        _ => {}
    };
}
