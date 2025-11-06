// mod http;
// use crate::http::http::{HttpRequestBuilder, HttpMethod};
// fn main() {
//     println!("Example 1: Simple GET");
//     let request = HttpRequestBuilder::new("https://api.example.com/users").build();
//     println!("{:?}\n", request);

//     println!("Example 2: POST with data");
//     let request = HttpRequestBuilder::new("https://api.example.com/users")
//         .method(HttpMethod::POST)
//         .header("Content-Type", "application/json")
//         .header("Authorization", "Bearer token123")
//         .body(r#"{"name": "Alice", "age": 30}"#.to_string())
//         .build();
//     println!("{:?}\n", request)
// }

use core::num;
use std::vec;

fn main() {
    // Linux-style newlines (\n)
    let linux_request = "GET /index.html HTTP/1.1\nHost: example.com\nUser-Agent: curl/7.55.1";
    
    println!("=== Linux-style (\\n) ===");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers.iter().map(|num| num * 3).filter(|n| *n >2));
    // println!("{:?}", linux_request.lines().collect::<Vec<_>>());

    // println!("{:?}", linux_request.lines())
    // for (i, line) in linux_request.lines().enumerate() {
    //     println!("Line {}: {:?}", i + 1, line);
    // }

    // // Windows-style newlines (\r\n)
    // let windows_request = "GET /index.html HTTP/1.1\r\nHost: example.com\r\nUser-Agent: curl/7.55.1";
    
    // println!("\n=== Windows-style (\\r\\n) ===");
    // for (i, line) in windows_request.lines().enumerate() {
    //     println!("Line {}: {:?}", i + 1, line);
    // }
}

