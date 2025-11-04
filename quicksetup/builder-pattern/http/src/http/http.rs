#[derive(Debug)]
pub struct HttpRequest {
   pub  method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub struct HttpRequestBuilder {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}
#[derive(Debug)]
pub enum  HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    HEAD
}

impl HttpRequestBuilder {
    pub fn new(url: &str) -> Self {
        Self {
            method: HttpMethod::GET,
            url: url.to_string(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }
    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            body: self.body,
        }
    }
}
