pub struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    max_connections: u32,
}

pub struct DatabaseConfigBuilder{
    host: Option<String>,
    port: Option<u16>,
    username:Option<String>,
    password: Option<String>,
    max_connections: Option<u32>
}
