pub struct Session {
    pub(crate) database_url: String,
    pub echo: bool,
}
pub struct SessionMaker {
    pub(crate) database_url: String,
    pub echo: Option<bool>,
}

impl SessionMaker {
    pub fn new(url: &str) -> Self {
        Self {
            database_url: url.to_string(),
            echo: None,
        }
    }
    pub fn query(self, table_name: &str) -> String{
        format!("SELECT * from {}", table_name)
    }
}
