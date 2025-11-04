   #[derive(Debug)]
    struct HttpRequest {
        method: String,
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    }

#[derive(Debug)]
struct HttpRequestBuilder{
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>
}

impl  HttpRequestBuilder {


    
}