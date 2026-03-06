#[derive(Clone)]
pub struct Configuration {
    host: String,
    port: String,
    connection_str: String,
    redis_url: String,
    omdb_api_key: String,
}

impl Configuration {
    pub fn new(
        host: String,
        port: String,
        connection_str: String,
        redis_url: String,
        omdb_api_key: String,
    ) -> Self {
        Self {
            host,
            port,
            connection_str,
            redis_url,
            omdb_api_key,
        }
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_connection_string(&self) -> &str {
        &self.connection_str
    }

    pub fn get_redis_url(&self) -> &str {
        &self.redis_url
    }

    pub fn get_omdb_api_key(&self) -> &str {
        &self.omdb_api_key
    }
}
