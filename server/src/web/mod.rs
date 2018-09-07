//! The web-serving parts.

mod endpoints;
mod util;

use std::collections::BTreeMap;
use std::sync::Arc;

use futures::prelude::*;
use serde_json::Value;
use tera::{self, Context, Tera};
use url::Url;
use warp::{
    self,
    filters::BoxedFilter,
    http::{
        header::{HeaderValue, CONTENT_TYPE},
        status::StatusCode,
        Response,
    },
    reject, Filter, Rejection,
};

use {log_err, web::endpoints::*, DB};

#[derive(Deserialize)]
struct GetIndexParams {
    redirect: String,
}

/// Returns all the routes.
pub fn routes(
    db: DB,
    mailer_server_url: Url,
    base_url: Arc<Url>,
) -> BoxedFilter<(impl warp::Reply,)> {
    let mut tera = Tera::default();
    tera.register_global_function(
        "relative_url",
        Box::new(move |args| {
            let s = try_get_value!("relative_url", "path", String, args["path"]);
            let url = base_url.join(&s).map_err(|e| e.to_string())?;
            Ok(tera::to_value(&url.to_string()).unwrap())
        }),
    );
    tera.add_raw_templates(vec![
        ("base.html", include_str!("base.html")),
        ("error.html", include_str!("error.html")),
        ("get-index.html", include_str!("get-index.html")),
        ("post-index.html", include_str!("post-index.html")),
    ]).expect("Template error");

    let render = Arc::new(move |name: &str, context: Context| -> Response<String> {
        match tera.render(name, &context) {
            Ok(html) => {
                let mut res = Response::new(html);
                res.headers_mut()
                    .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
                res
            }
            Err(e) => {
                let mut s = e
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");
                error!("{}", s);

                let mut res = Response::new(s);
                res.headers_mut()
                    .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
                *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                res
            }
        }
    });
    let render2 = render.clone();
    let render3 = render.clone();

    let db2 = db.clone();
    let db3 = db.clone();
    let db4 = db.clone();

    fn err_handler<E: Into<::failure::Error>>(err: E) -> Rejection {
        log_err(err.into());
        reject::server_error()
    }

    warp::index()
        .and(warp::get2())
        .and(util::query_opt())
        .and_then(move |params: Option<GetIndexParams>| {
            let redirect = params
                .unwrap_or_else(|| GetIndexParams {
                    redirect: "acm.umn.edu".to_string(),
                })
                .redirect;
            get_index(None, &redirect, render.clone()).map_err(err_handler)
        })
        .or(warp::index()
            .and(warp::post2())
            .and(warp::body::form())
            .and_then(move |params| {
                post_index(params, render2.clone(), db.clone()).map_err(err_handler)
            }))
        .or(path!("main.css").and(warp::index()).map(|| {
            let mut res = Response::new(include_str!("main.css").to_string());
            res.headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("text/css"));
            res
        }))
        .or(path!("status")
            .and(warp::index())
            .and(warp::get2())
            .map(|| {
                let mut res = Response::new("".to_string());
                *res.status_mut() = StatusCode::NO_CONTENT;
                res
            }))
        .boxed()
}

/*
        .or(path!("send")
            .and(warp::index())
            .and(warp::post2())
            .and(warp::body::form())
            .and_then(move |params| {
                send(params, db.clone()).map_err(|e| {
                    log_err(e.into());
                    reject::server_error()
                })
            }))
        .or(path!("template" / u32)
            .and(warp::index())
            .and(
                warp::get2()
                    .and(warp::query::<BTreeMap<String, Value>>())
                    .or(warp::post2().and(warp::body::form::<BTreeMap<String, Value>>()))
                    .unify(),
            )
            .and_then(move |template_id: u32, values: BTreeMap<String, Value>| {
                let mut context = Context::new();
                for (k, v) in values {
                    context.add(&k, &v);
                }
                template(template_id, context, auth_server_url.as_ref(), db2.clone()).map_err(|e| {
                    log_err(e.into());
                    reject::server_error()
                })
            }))
        .or(path!("unsubscribe" / u32)
            .and(warp::index())
            .and(warp::get2())
            .and(warp::query())
            .and_then(move |mailing_list_id, params| {
                unsubscribe_get(mailing_list_id, params, db3.clone(), render2.clone()).map_err(
                    |e| {
                        log_err(e.into());
                        reject::server_error()
                    },
                )
            }))
        .or(path!("unsubscribe" / u32)
            .and(warp::index())
            .and(warp::post2())
            .and(warp::body::form())
            .and_then(move |mailing_list_id, params| {
                unsubscribe_post(mailing_list_id, params, db4.clone(), render3.clone()).map_err(
                    |e| {
                        log_err(e.into());
                        reject::server_error()
                    },
                )
            }))
            */
