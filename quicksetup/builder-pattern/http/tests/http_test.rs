use http::http::http::{HttpMethod, HttpRequest, HttpRequestBuilder};

#[test]
fn test_httpRequestBuilder(){
    let url = "www.dimka.com";
    let build = HttpRequestBuilder::new(&url);
    assert_eq!(build.body, None);

}