pub trait Sink {}

pub struct Null;

impl Null {}

impl Sink for Null {}
