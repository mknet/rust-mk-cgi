use axum::http::Request;
use axum::Router;
use hyper::header::CONTENT_TYPE;
use hyper::Body;
use tower_service::Service;

pub fn handle(mut router: Router) {
    cgi::handle(|request: cgi::Request| -> cgi::Response {
        let future = async {
            let response = router.call(convert_request(request)).await.unwrap();
            let parts = response.into_parts();
            let content_type = parts
                .0
                .headers
                .get(CONTENT_TYPE)
                .map(|x| x.to_str().unwrap());
            let bytes = hyper::body::to_bytes(parts.1).await.unwrap();

            cgi::binary_response(parts.0.status, content_type, bytes.into())
        };

        futures::executor::block_on(future)
    });
}

fn convert_request(request: cgi::Request) -> axum::http::Request<Body> {
    let parts = request.into_parts();

    let real_parts = parts.0;

    let mut builder = Request::builder();

    builder.headers_mut().unwrap().extend(real_parts.headers);

    let axum_request = builder
        .method(real_parts.method)
        .uri(std::env::var("PATH_INFO").unwrap())
        .body(parts.1.into());

    axum_request.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::get;

    async fn root() -> String {
        "Hello World".to_string()
    }

    #[test]
    fn test_handle() {
        std::env::set_var("REQUEST_METHOD", "GET");
        std::env::set_var("SCRIPT_NAME", "axum.cgi");
        std::env::set_var("PATH_INFO", "/bla");

        let router = Router::new().route("/", get(root));

        handle(router)
    }
}
