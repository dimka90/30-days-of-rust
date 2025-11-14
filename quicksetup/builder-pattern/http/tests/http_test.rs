use http::http::http::{HttpMethod, HttpRequest, HttpRequestBuilder};

#[test]
fn test_http_request_builder(){
    let url = "www.dimka.com";
    let build = HttpRequestBuilder::new(&url);
    assert_eq!(build.url, url);
    assert_eq!(build.body, None);
    assert_eq!(build.method, HttpMethod::GET);
    assert_eq!(build.headers, Vec::new());

    assert_ne!(build.method, HttpMethod::POST)
}
#[test]
fn test_http_request_builder_method(){
    let url = "www.dimka.com";
    let http_method = HttpMethod::POST;
    let build = HttpRequestBuilder::new(&url)
                                    .method(HttpMethod::POST);

    assert_eq!(build.url, url);
    assert_eq!(build.body, None);
    assert_eq!(build.method, http_method);
    assert_eq!(build.headers, Vec::new());
}

#[test]
fn test_http_request_builder_url() {
    let url = "www.dimka.com";
    let http_method = HttpMethod::POST;
    let build = HttpRequestBuilder::new(&url)
                                    .method(HttpMethod::POST);

    assert_eq!(build.url, url);
    assert_eq!(build.body, None);
    assert_eq!(build.method, http_method);
    assert_eq!(build.headers, Vec::new());
}

#[test]
fn test_http_request_builder_body() {
    let url = "www.dimka.com";
    let http_method = HttpMethod::POST;
    let body = r#"{name:"dimka", "age": 20 }"#.to_string();
    let build = HttpRequestBuilder::new(&url)
                                    .method(HttpMethod::POST)
                                    .body(body.clone());
    assert_eq!(build.url, url);
    assert_eq!(build.body, Some(body));
    assert_eq!(build.method, http_method);
    assert_eq!(build.headers, Vec::new());

}