use futures_core::Stream;

use crate::source::PipeMessage;

pub trait Plug {
    fn add_prev(&'_ mut self, prev: &'_ dyn Stream<Item = PipeMessage>);
}

pub struct Blank<'a> {
    pub(crate) prev: &'a dyn Stream<Item = PipeMessage>,
}

impl<'a> Plug for Blank<'a> {
    fn add_prev(&'a mut self, prev: &'a dyn Stream<Item = PipeMessage>) {
        self.prev = prev;
    }
}
