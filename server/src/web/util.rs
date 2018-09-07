use serde::de::DeserializeOwned;
use warp::{self, Filter, Rejection};

/// An optional version of `warp::query`.
pub fn query_opt<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (Option<T>,), Error = Rejection> + Copy {
    warp::query().map(Some).or(warp::any().map(|| None)).unify()
}
