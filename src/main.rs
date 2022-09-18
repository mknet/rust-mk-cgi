mod cgi_axum_router;

use crate::cgi_axum_router::handle;
use axum::{routing::get, Router};

async fn root() -> String {
    "Hello World".to_string()
}

fn main() {
    let router = Router::new().route("/", get(root));
    handle(router);
}
