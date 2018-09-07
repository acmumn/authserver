extern crate acmumn_identity_server as identity;
extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;
extern crate syslog;
extern crate tokio;
extern crate url;
extern crate warp;

use std::net::{SocketAddr, ToSocketAddrs};
use std::process::exit;
use std::sync::Arc;

use failure::Error;
use identity::{log_err, routes, DB};
use structopt::StructOpt;
use url::Url;

fn main() {
    dotenv::dotenv().ok();
    let options = Options::from_args();
    options.start_logger();

    if let Err(err) = run(options) {
        log_err(err);
        exit(1);
    }
}

fn run(options: Options) -> Result<(), Error> {
    let serve_addr = options.serve_addr()?;
    let db = DB::connect(&options.database_url)?;

    let base_url = Arc::new(options.base_url);
    let routes = routes(db.clone(), options.mailer_server, base_url.clone());
    let server = warp::serve(routes).bind(serve_addr);

    tokio::run(server);
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "::structopt::clap::AppSettings::ColoredHelp"))]
struct Options {
    /// Turns off message output.
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Increases the verbosity. Default verbosity is errors and warnings.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,

    /// The base URL for unsubscribe links and template examples.
    #[structopt(short = "b", long = "base-url", env = "BASE_URL")]
    base_url: Url,

    /// The URL of the MySQL database.
    #[structopt(short = "d", long = "db", env = "DATABASE_URL")]
    database_url: String,

    /// The host to serve on.
    #[structopt(short = "h", long = "host", env = "HOST", default_value = "::")]
    host: String,

    /// The URL of the mailer server to use.
    #[structopt(short = "a", long = "mailer-server", env = "MAILER_SERVER")]
    mailer_server: Url,

    /// The port to serve on.
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "8000")]
    port: u16,

    /// The syslog server to send logs to.
    #[structopt(short = "s", long = "syslog-server", env = "SYSLOG_SERVER")]
    syslog_server: Option<String>,
}

impl Options {
    /// Get the address to serve on.
    fn serve_addr(&self) -> Result<SocketAddr, Error> {
        let addrs = (&self.host as &str, self.port)
            .to_socket_addrs()?
            .collect::<Vec<_>>();
        if addrs.is_empty() {
            bail!("No matching address exists")
        } else {
            Ok(addrs[0])
        }
    }

    /// Sets up logging as specified by the `-q`, `-s`, and `-v` flags.
    fn start_logger(&self) {
        if !self.quiet {
            let log_level = match self.verbose {
                0 => log::LevelFilter::Warn,
                1 => log::LevelFilter::Info,
                2 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            };

            let r = if let Some(ref server) = self.syslog_server {
                syslog::init_tcp(
                    server,
                    "identity".to_string(),
                    syslog::Facility::LOG_DAEMON,
                    log_level,
                )
            } else {
                // rifp https://github.com/Geal/rust-syslog/pull/38
                syslog::init(syslog::Facility::LOG_DAEMON, log_level, Some("identity"))
            };

            if let Err(err) = r {
                error!("Warning: logging couldn't start: {}", err);
            }
        }
    }
}
