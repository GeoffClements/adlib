use bytes::Bytes;
use futures_core::Stream;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[non_exhaustive]
pub enum PipeMessage {
    Data(Bytes),
}

pub trait Source: Stream<Item = PipeMessage> {}

struct Zero;
const SIZE: usize = 1024;

impl Stream for Zero {
    type Item = PipeMessage;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let zeros = Bytes::from(vec![0u8; SIZE]);
        Poll::Ready(Some(PipeMessage::Data(zeros)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(SIZE))
    }
}

impl Source for Zero {}
