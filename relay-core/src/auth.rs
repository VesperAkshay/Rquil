use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use reqwest::Client;
use crate::model::RequestAuth;

static TOKEN_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

#[derive(Debug)]
pub enum AuthError {
    MissingConfig(String),
    Reqwest(reqwest::Error),
    Parse(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingConfig(m) => write!(f, "Missing auth config: {}", m),
            AuthError::Reqwest(e) => write!(f, "Auth request error: {}", e),
            AuthError::Parse(e) => write!(f, "Auth parsing error: {}", e),
        }
    }
}

impl std::error::Error for AuthError {}

impl From<reqwest::Error> for AuthError {
    fn from(err: reqwest::Error) -> Self { AuthError::Reqwest(err) }
}

pub async fn get_token(client: &Client, auth: &RequestAuth) -> Result<String, AuthError> {
    if auth.auth_type != "oauth2_client_credentials" {
        return Err(AuthError::MissingConfig(format!("Unsupported auth type: {}", auth.auth_type)));
    }

    let token_url = auth.token_url.as_ref().ok_or_else(|| AuthError::MissingConfig("token_url is required".into()))?;
    let client_id = auth.client_id.as_ref().ok_or_else(|| AuthError::MissingConfig("client_id is required".into()))?;
    let client_secret = auth.client_secret.as_ref().ok_or_else(|| AuthError::MissingConfig("client_secret is required".into()))?;

    let cache_key = format!("{}|{}", token_url, client_id);

    {
        let cache = TOKEN_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        if let Ok(guard) = cache.lock() {
            if let Some(token) = guard.get(&cache_key) {
                return Ok(token.clone());
            }
        }
    }

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");

    let res = client.post(token_url)
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await?;
        
    let res_text = res.text().await?;
    let parsed: serde_json::Value = serde_json::from_str(&res_text)
        .map_err(|e| AuthError::Parse(e.to_string()))?;

    let access_token = parsed.get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AuthError::Parse(format!("Response missing access_token. Body: {}", res_text)))?
        .to_string();

    {
        let cache = TOKEN_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        if let Ok(mut guard) = cache.lock() {
            guard.insert(cache_key, access_token.clone());
        }
    }

    Ok(access_token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_token_success() {
        let server = MockServer::start().await;
        
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "mocked_token_123"
            })))
            .mount(&server)
            .await;
            
        let client = Client::new();
        let auth = RequestAuth {
            auth_type: "oauth2_client_credentials".to_string(),
            token_url: Some(format!("{}/token", server.uri())),
            client_id: Some("test_client_id".to_string()),
            client_secret: Some("test_client_secret".to_string()),
        };
        
        let token = get_token(&client, &auth).await.unwrap();
        assert_eq!(token, "mocked_token_123");
    }
}
