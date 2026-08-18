use crate::model::Request;
use reqwest::{Client, Method, header::{HeaderMap, HeaderName, HeaderValue}};
use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub time_ms: u128,
}

#[derive(Debug)]
pub enum HttpError {
    Reqwest(reqwest::Error),
    InvalidMethod(String),
    Auth(crate::auth::AuthError),
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::Reqwest(e) => write!(f, "Request error: {}", e),
            HttpError::InvalidMethod(m) => write!(f, "Invalid HTTP method: {}", m),
            HttpError::Auth(e) => write!(f, "Auth error: {}", e),
        }
    }
}

impl std::error::Error for HttpError {}

impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> Self {
        HttpError::Reqwest(err)
    }
}

/// Executes a resolved request.
pub async fn execute(client: &Client, req: &Request) -> Result<Response, HttpError> {
    let method = Method::from_bytes(req.method.as_bytes())
        .map_err(|_| HttpError::InvalidMethod(req.method.clone()))?;
        
    let mut builder = client.request(method, &req.url);

    let mut header_map = HeaderMap::new();
    for (k, v) in &req.headers {
        if let (Ok(name), Ok(value)) = (HeaderName::from_bytes(k.as_bytes()), HeaderValue::from_str(v)) {
            header_map.insert(name, value);
        }
    }
    builder = builder.headers(header_map);

    if let Some(auth) = &req.auth {
        if auth.auth_type == "oauth2_client_credentials" {
            let token = crate::auth::get_token(client, auth).await.map_err(HttpError::Auth)?;
            builder = builder.bearer_auth(token);
        }
    }

    if let Some(body) = &req.body {
        builder = builder.body(body.content.clone());
    }

    let start = Instant::now();
    let res = builder.send().await?;
    let time_ms = start.elapsed().as_millis();

    let status = res.status().as_u16();
    
    let mut res_headers = HashMap::new();
    for (k, v) in res.headers() {
        if let Ok(v_str) = v.to_str() {
            res_headers.insert(k.to_string(), v_str.to_string());
        }
    }

    let body = res.text().await?;

    Ok(Response {
        status,
        headers: res_headers,
        body,
        time_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    
    #[tokio::test]
    async fn test_execute_request() {
        let server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/api/test"))
            .respond_with(ResponseTemplate::new(200).set_body_string("Success!"))
            .mount(&server)
            .await;
            
        let client = Client::new();
        let mut req = Request {
            method: "GET".to_string(),
            url: format!("{}/api/test", server.uri()),
            headers: HashMap::new(),
            body: None,
            auth: None,
        };
        req.headers.insert("Accept".to_string(), "text/plain".to_string());
        
        let response = execute(&client, &req).await.unwrap();
        
        assert_eq!(response.status, 200);
        assert_eq!(response.body, "Success!");
    }
}
