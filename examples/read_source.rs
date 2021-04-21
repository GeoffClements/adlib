use adlib::Source;
use futures_util::stream::StreamExt;
use tokio;
use url::Url;

use std::env::current_dir;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut path = current_dir().unwrap();
    path.push("examples/music.flac");
    if let Ok(url) = Url::from_file_path(path) {
        let mut src = Source::from(url).await.unwrap();
        while let Some(m) = src.next().await {
            println!("{:?}", m);
        }
    }
}
