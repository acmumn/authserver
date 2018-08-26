use std::fmt::{Display, Formatter, Result as FmtResult};

use failure::{Backtrace, Context, Fail};
use url::Url;

/// A convenient alias for Result.
pub type Result<T> = ::std::result::Result<T, Error>;

/// The kind of an application error.
#[derive(Debug, Fail)]
pub enum ErrorKind {
    /// An invalid response was received from the server. Maybe the server is running an
    /// incompatible version of the server software, maybe it's running a different piece of
    /// software entirely?
    #[fail(display = "Invalid server response: {}", _0)]
    BadServerResponse(String),

    /// The token was invalid because it had expired. If a URL is present, the member can
    /// request a new token by navigating to it (in a browser).
    #[fail(display = "Token is expired")]
    ExpiredToken(Option<Url>),

    /// An error occurred while trying make a request to the server.
    #[fail(display = "Couldn't make request: {}", _0)]
    Hyper(::hyper::Error),

    /// An invalid URL was given as a base URL.
    #[fail(display = "Invalid base URL: {}", _0)]
    InvalidBaseURL(Url),

    /// An invalid service token was given.
    #[fail(display = "Invalid service token")]
    InvalidServiceToken(Option<String>),

    /// The string given for authentication isn't a valid JWT, or was structurally invalid in some
    /// other way. This indicates that not only is the token not valid, it was not issued by the
    /// identity server.
    #[fail(display = "Token was structurally invalid")]
    InvalidToken,
}

/// An application error.
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.inner.get_context()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(&self.inner, f)
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl<E: Into<ErrorKind>> From<E> for Error {
    fn from(err: E) -> Error {
        Context::new(err.into()).into()
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}
