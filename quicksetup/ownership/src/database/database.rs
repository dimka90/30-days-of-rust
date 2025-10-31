#[derive(Debug)]
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
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }
    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
    pub fn max_connections(mut self, max_con: u32) -> Self {
        self.max_connections = Some(max_con);
        self
    }
    pub fn build(self) -> Result<DatabaseConfig, &'static str> {
        Ok(DatabaseConfig {
            host: self.host.ok_or("host name is required")?,
            port: self.port.unwrap_or(5432),
            username: self.username.ok_or("Username is required")?,
            password: self.password.ok_or("password is required")?,
            max_connections: self.max_connections.unwrap_or(10),
        })
    }
}
