use adlib::{Framer, Source};
use futures_util::stream::StreamExt;
use tokio;
// use tokio_util::io::ReaderStream;
use url::Url;

use std::{convert::TryFrom, env::current_dir};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut path = current_dir().unwrap();
    path.push("examples/music.flac");
    if let Ok(url) = Url::from_file_path(path) {
        let src = Source::try_from(url).unwrap();
        let mut src = Framer::new(src);
        while let Some(Ok(m)) = src.next().await {
            println!("{:?}", m);
        }
    }
}
