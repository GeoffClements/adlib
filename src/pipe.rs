use crate::{plug::Plug, sink::Sink, source::Source};

pub struct Pipe<P, IN, OUT> {
    plugs: Vec<P>,
    source: IN,
    sink: OUT,
}

impl<P: Plug, IN, OUT> Pipe<P, IN, OUT> {
    pub fn plug<'a>(&'a mut self, plug: P) -> &'a mut Self {
        self.plugs.push(plug);
        self
    }
}

pub struct PipeBuilder<IN, OUT> {
    source: Option<IN>,
    sink: Option<OUT>,
}

impl<IN: Source, OUT: Sink> PipeBuilder<IN, OUT> {
    pub fn new() -> Self {
        PipeBuilder {
            source: None,
            sink: None,
        }
    }

    pub fn source<'a>(&'a mut self, source: IN) -> &'a mut Self {
        self.source = Some(source);
        self
    }

    pub fn sink<'a>(&'a mut self, sink: OUT) -> &'a mut Self {
        self.sink = Some(sink);
        self
    }

    pub fn build<P: Plug>(self) -> Result<Pipe<P, IN, OUT>, &'static str> {
        if let Some(source) = self.source {
            if let Some(sink) = self.sink {
                return Ok(Pipe {
                    plugs: Vec::new(),
                    source: source,
                    sink: sink,
                });
            }
        }
        Err("Require both source and sink")
    }
}
