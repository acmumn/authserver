//! A Rust client for the identity service.
#[macro_use]
extern crate failure;
extern crate futures;
extern crate hyper;
extern crate identity_common;
extern crate log;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

mod errors;
mod responses;

use failure::{Fail, ResultExt};
use futures::{
    future::{err, Either},
    prelude::*,
};
use hyper::{
    client::{connect::Connect, HttpConnector},
    header::{ACCEPT, ACCEPT_CHARSET, CONTENT_TYPE, COOKIE, USER_AGENT},
    Request, Uri,
};
pub use identity_common::{ClientData, Token};
use url::Url;

pub use errors::{Error, ErrorKind, Result};
use responses::Validate400Response;

/// The client.
pub struct Client<C = HttpConnector> {
    http: hyper::Client<C>,
    service_token: String,
    validate_uri: Uri,
}

impl Client<HttpConnector> {
    /// Creates a new client for the identity server at the given URL, using the given service
    /// token.
    pub fn new(base_url: Url, service_token: String) -> Result<Client> {
        Client::from_hyper(base_url, service_token, hyper::Client::new())
    }
}

impl<C: 'static + Connect + Sync> Client<C> {
    /// Creates a client from an existing Hyper client.
    pub fn from_hyper(
        base_url: Url,
        service_token: String,
        client: hyper::Client<C>,
    ) -> Result<Client<C>> {
        let validate_uri = match base_url.join("validate") {
            Ok(url) => match url.as_str().parse::<Uri>() {
                Ok(uri) => uri,
                Err(e) => {
                    return Err(e)
                        .context(ErrorKind::InvalidBaseURL(base_url))
                        .map_err(Error::from)
                }
            },
            Err(e) => {
                return Err(e)
                    .context(ErrorKind::InvalidBaseURL(base_url))
                    .map_err(Error::from)
            }
        };

        Ok(Client {
            http: client,
            service_token,
            validate_uri,
        })
    }

    /// Validates a token.
    pub fn validate(&self, token: String) -> impl Future<Item = Token, Error = Error> {
        const UA: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

        let r = Request::post(&self.validate_uri)
            .header(ACCEPT, mime::APPLICATION_JSON.as_ref())
            .header(ACCEPT_CHARSET, "utf-8")
            .header(CONTENT_TYPE, mime::TEXT_PLAIN.as_ref())
            .header(COOKIE, format!("auth={}", self.service_token))
            .header(USER_AGENT, UA)
            .body(token.into())
            .context(ErrorKind::InvalidServiceToken(None)); // Theoretically, this is the only
                                                            // thing that can go wrong.
        match r {
            Ok(req) => Either::A(
                self.http
                    .request(req)
                    .map_err(|e| ErrorKind::Hyper(e).into())
                    .and_then(|res| match res.status().as_u16() {
                        200 => Either::A(Either::A(
                            res.into_body()
                                .concat2()
                                .map_err(|e| ErrorKind::Hyper(e).into())
                                .and_then(|msg| {
                                    serde_json::from_slice(msg.as_ref()).map_err(|e| {
                                        e.context(ErrorKind::BadServerResponse(
                                            "Non-JSON response in 200 response".to_string(),
                                        )).into()
                                    })
                                }),
                        )),
                        400 => Either::A(Either::B(
                            res.into_body()
                                .concat2()
                                .map_err(|e| ErrorKind::Hyper(e).into())
                                .and_then(|msg| {
                                    err(match serde_json::from_slice(msg.as_ref()) {
                                        Ok(Validate400Response::Expired) => {
                                            ErrorKind::ExpiredToken(None).into()
                                        }
                                        Ok(Validate400Response::Invalid) => {
                                            ErrorKind::InvalidToken.into()
                                        }
                                        Err(e) => {
                                            e.context(ErrorKind::BadServerResponse(
                                                "Non-JSON response in 400 error".to_string(),
                                            )).into()
                                        }
                                    })
                                }),
                        )),
                        403 => Either::B(Either::A(
                            res.into_body()
                                .concat2()
                                .map_err(|e| ErrorKind::Hyper(e).into())
                                .and_then(|msg| {
                                    err(match String::from_utf8(msg.to_vec()) {
                                        Ok(msg) => ErrorKind::InvalidServiceToken(Some(msg)).into(),
                                        Err(e) => {
                                            e.context(ErrorKind::BadServerResponse(
                                                "Non-UTF8 response in 403 error".to_string(),
                                            )).into()
                                        }
                                    })
                                }),
                        )),
                        _ => Either::B(Either::B(err(ErrorKind::BadServerResponse(format!(
                            "Invalid status: {}",
                            res.status()
                        )).into()))),
                    }),
            ),
            Err(e) => Either::B(err(e.into())),
        }
    }
}
