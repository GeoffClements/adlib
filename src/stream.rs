use pin_project::pin_project;
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncWriteExt, ReadBuf},
    net::TcpStream,
};
use url::{Host, Url};

use std::{
    self,
    convert::{From, TryFrom},
    io::{self, ErrorKind},
    net::IpAddr,
    pin::Pin,
    task::{Context, Poll},
};

#[pin_project(project = SourceTypesProj)]
enum SourceTypes {
    File(#[pin] File),
    Tcp(#[pin] TcpStream),
}

#[pin_project]
pub struct Source {
    #[pin]
    inner: SourceTypes,
}

impl From<std::fs::File> for Source {
    fn from(file: std::fs::File) -> Self {
        Self {
            inner: SourceTypes::File(File::from_std(file)),
        }
    }
}

impl TryFrom<std::net::TcpStream> for Source {
    type Error = io::Error;

    fn try_from(tcp: std::net::TcpStream) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: SourceTypes::Tcp(TcpStream::from_std(tcp)?),
        })
    }
}

impl TryFrom<Url> for Source {
    type Error = io::Error;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        match url.scheme() {
            "file" => {
                let file = std::fs::File::open(url.path())?;
                Ok(Source::from(file))
            }
            "http" => {
                // TODO: handle name resolution
                // unwrap is safe as scheme is http
                let port = url.port_or_known_default().unwrap();
                let addr = match url.host() {
                    Some(Host::Ipv4(a)) => IpAddr::V4(a),
                    Some(Host::Ipv6(a)) => IpAddr::V6(a),
                    _ => return Err(io::Error::from(ErrorKind::InvalidInput)),
                };
                let tcp = std::net::TcpStream::connect((addr, port))?;
                Ok(Source::try_from(tcp)?)
            }

            _ => Err(io::Error::from(ErrorKind::NotFound)),
        }
    }
}

impl Source {
    // Used to send a string to a server, typically a http GET request
    pub async fn get(mut self, get: &str) -> io::Result<Self> {
        match self.inner {
            SourceTypes::Tcp(ref mut tcp) => {
                tcp.write_all(get.as_bytes()).await?;
                tcp.write_all(b"\r\n\r\n").await?;
                Ok(self)
            }
            _ => Err(io::Error::from(ErrorKind::InvalidInput)),
        }
    }
}

impl AsyncRead for Source {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.project();
        match this.inner.project() {
            SourceTypesProj::File(f) => f.poll_read(cx, buf),
            SourceTypesProj::Tcp(s) => s.poll_read(cx, buf),
        }
    }
}
