mod http;
use crate::http::http::{HttpRequestBuilder, HttpMethod};
fn main() {
    println!("Example 1: Simple GET");
    let request = HttpRequestBuilder::new("https://api.example.com/users").build();
    println!("{:?}\n", request);

    println!("Example 2: POST with data");
    let request = HttpRequestBuilder::new("https://api.example.com/users")
        .method(HttpMethod::POST)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "Alice", "age": 30}"#.to_string())
        .build();
    println!("{:?}\n", request)
}
