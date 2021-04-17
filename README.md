# adlib

Rust library for playing music asynchronously.

Currently a WIP.

The plan is to allow the composition of asynchronous [Stream]s that
propegate audio data to an ultimate sink object that will implement
[cpal].

Pipes (i.e. a chain of [Stream]s) can be created through composition
such as the following example:

```code Rust
let sink = Sink::from(<cpal object>);
let pipe = Source(<uri>)
    .buffer(<size>)
    .decoder()
    .gain(<gain control>)
    .volume(<volume control>);

sink.send_all(pipe);
```

In the above case we should be able to specify any valid uri for the
source object, thus both files and TCP streams should be possible.

New objects that implement [Stream] can be added to the library that
monitor and/or alter the audio data.

## Help

If you'd like to help then please contact me. I have limited time for
coding and progress may be slow as a consequence.

[Stream]: https://docs.rs/futures/0.3.14/futures/stream/index.html
[cpal]: https://docs.rs/cpal/0.13.3/cpal/
