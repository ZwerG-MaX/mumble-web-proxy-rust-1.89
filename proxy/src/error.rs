use futures::channel::mpsc;

// FIXME clean this up

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ServerTls(native_tls::Error),
    ClientConnection(tungstenite::Error),
    Misc(Box<dyn std::error::Error + Send>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::ServerTls(e) => write!(f, "server TLS error: {e}"),
            Error::ClientConnection(e) => write!(f, "client connection error: {e}"),
            Error::Misc(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::ServerTls(e) => Some(e),
            Error::ClientConnection(e) => Some(e),
            Error::Misc(e) => Some(e.as_ref()),
        }
    }
}

impl Error {
    pub fn is_connection_closed(&self) -> bool {
        matches!(
            self,
            Error::ClientConnection(tungstenite::Error::ConnectionClosed)
        )
    }
}

impl From<tungstenite::Error> for Error {
    fn from(e: tungstenite::Error) -> Self {
        Error::ClientConnection(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Error::ServerTls(e)
    }
}

impl From<rtp::Error> for Error {
    fn from(e: rtp::Error) -> Self {
        Error::Misc(Box::new(e))
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::Misc(Box::new(e))
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        panic!();
    }
}

impl From<mpsc::SendError> for Error {
    fn from(_: mpsc::SendError) -> Self {
        panic!();
    }
}
