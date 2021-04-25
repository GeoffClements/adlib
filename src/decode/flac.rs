use bytes::{Buf, BytesMut};
// use claxon::metadata;

use super::DecodeResult;

pub(crate) fn start_of_flac_stream(src: &mut BytesMut) -> DecodeResult {
    const FLACMAGIC: &[u8] = b"fLaC";

    // Check length, need: magic + header
    if src.len() < FLACMAGIC.len() + 4 {
        return DecodeResult::MoreData;
    }

    if !src.starts_with(FLACMAGIC) {
        return DecodeResult::Unrecognised;
    }

    // Get enough data to decode a full set of metadata blocks
    let blocklength = src[4..].as_ref().get_u32() as usize & 0x00_FF_FF_FF;
    if src.len() < blocklength + 8 {
        return DecodeResult::MoreData;
    }

    // Now we have all header blocks

    DecodeResult::Unrecognised
}
