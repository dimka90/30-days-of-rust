pub struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    max_connections: u32,
}

#[derive(Debug)]
pub struct DatabaseConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    max_connections: Option<u32>,
}

impl DatabaseConfigBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
            username: None,
            password: None,
            max_connections: None,
        }
    }
    pub fn port(mut self, host: String) -> Self {
        self.host =  Some(host);
        self
    }
}
