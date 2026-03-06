use anyhow::Result;
use axum::http::{HeaderMap, HeaderValue};

const HOST: &str = "https://kinopoiskapiunofficial.tech/api/v2.2/films/301";

pub async fn fetch_kinopoisk(apikey: &str) -> Result<String> {
    let builder = reqwest::ClientBuilder::new();

    let mut header = HeaderMap::new();
    header.insert("X-API-KEY", HeaderValue::from_str(apikey)?);
    header.insert("Content-Type", HeaderValue::from_str("application/json")?);

    let client = builder.default_headers(header).build()?;
    let response = client.get(HOST).send().await?;

    Ok(response.text().await?)
}

mod test {
    use crate::jobs::kinopoisk_fetcher::fetch_kinopoisk;

    #[tokio::test]
    async fn fetch_kinopoisk_returns_non_empty_string_on_success() {
        // Arrange
        // ---

        // Act
        let result = fetch_kinopoisk("").await;

        // Assert
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty())
    }
}
