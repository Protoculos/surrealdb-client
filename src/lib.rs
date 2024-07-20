use base64;
use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientOptions {
    ns: String,
    db: String,
    auth: String,
    url: String,
}

impl ClientOptions {
    pub fn new(
        ns: String,
        db: String,
        url: String,
        login: Option<String>,
        password: Option<String>,
    ) -> Self {
        let auth = Self::encode_auth(
            login.unwrap_or_else(|| "root".to_string()),
            password.unwrap_or_else(|| "root".to_string()),
        );
        ClientOptions { ns, db, auth, url }
    }

    fn encode_auth(login: String, password: String) -> String {
        let auth_string = format!("{}:{}", login, password);
        base64::encode(auth_string)
    }
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self::new(
            "test".to_string(),
            "test".to_string(),
            "http://localhost:8000/sql".to_string(),
            Some("root".to_string()),
            Some("root".to_string()),
        )
    }
}

pub struct SurrealDBClient {
    client: reqwest::Client,
    options: ClientOptions,
}

impl SurrealDBClient {
    pub fn new(options: ClientOptions) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        Ok(Self { client, options })
    }

    pub async fn execute_sql(&self, sql: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);
        headers.insert("NS", self.options.ns.parse()?);
        headers.insert("DB", self.options.db.parse()?);
        headers.insert("Content-Type", "text/plain".parse()?);
        headers.insert(
            "Authorization",
            format!("Basic {}", self.options.auth).parse()?,
        );

        let response = self
            .client
            .request(reqwest::Method::POST, &self.options.url)
            .headers(headers)
            .body(sql.to_string())
            .send()
            .await?;

        let body = response.text().await?;
        Ok(body)
    }
}
