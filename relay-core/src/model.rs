use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RelayFile {
    pub meta: Meta,
    pub request: Request,
    #[serde(default)]
    pub script: Script,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Meta {
    pub name: String,
    #[serde(rename = "type")]
    pub req_type: String,
    #[serde(default)]
    pub notes: Option<Notes>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Notes {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Request {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: Option<RequestBody>,
    #[serde(default)]
    pub auth: Option<RequestAuth>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RequestBody {
    #[serde(rename = "type")]
    pub body_type: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct RequestAuth {
    #[serde(rename = "type")]
    pub auth_type: String,
    #[serde(default)]
    pub token_url: Option<String>,
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub client_secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Script {
    #[serde(default)]
    pub post_response: Vec<PostResponseScript>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PostResponseScript {
    pub assert: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct CollectionConfig {
    #[serde(default)]
    pub environments: HashMap<String, Environment>,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Environment {
    #[serde(default)]
    pub variables: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_sample_rl() {
        let sample = r#"
[meta]
name = "Create user"
type = "http"

[request]
method = "POST"
url = "{{base_url}}/api/users"

[request.headers]
"Content-Type" = "application/json"
"Authorization" = "Bearer {{auth_token}}"

[request.body]
type = "json"
content = """
{
  "name": "{{test_user_name}}",
  "email": "{{test_user_email}}"
}
"""

[request.auth]
type = "inherit"

[[script.post_response]]
assert = "res.status == 201"

[meta.notes]
text = "Added after the 2026-05 signup-flow regression."
"#;

        let parsed: RelayFile = toml::from_str(sample).expect("Failed to parse TOML");
        assert_eq!(parsed.meta.name, "Create user");
        assert_eq!(parsed.meta.req_type, "http");
        assert_eq!(parsed.meta.notes.unwrap().text, "Added after the 2026-05 signup-flow regression.");
        
        assert_eq!(parsed.request.method, "POST");
        assert_eq!(parsed.request.url, "{{base_url}}/api/users");
        
        let content_type = parsed.request.headers.get("Content-Type").unwrap();
        assert_eq!(content_type, "application/json");
        
        let body = parsed.request.body.unwrap();
        assert_eq!(body.body_type, "json");
        assert!(body.content.contains("{{test_user_name}}"));
        
        let auth = parsed.request.auth.unwrap();
        assert_eq!(auth.auth_type, "inherit");
        
        assert_eq!(parsed.script.post_response[0].assert, "res.status == 201");
    }
}
