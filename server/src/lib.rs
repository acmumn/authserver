#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate tera;
extern crate tokio_threadpool;
extern crate url;
#[macro_use]
extern crate warp;

#[macro_use]
mod macros;

mod db;
mod errors;
mod web;

pub use db::DB;
pub use errors::{Error, ErrorKind, Result};
pub use web::routes;

/// Logs an error, including its causes and backtrace (if possible).
pub fn log_err(err: failure::Error) {
    let mut first = true;
    let num_errs = err.iter_chain().count();
    if num_errs <= 1 {
        error!("{}", err);
    } else {
        for cause in err.iter_chain() {
            if first {
                first = false;
                error!("           {}", cause);
            } else {
                error!("caused by: {}", cause);
            }
        }
    }
    let bt = err.backtrace().to_string();
    if bt != "" {
        error!("{}", bt);
    }
}
