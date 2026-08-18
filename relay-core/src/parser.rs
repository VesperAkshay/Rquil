use crate::model::RelayFile;
use std::fs;
use std::path::Path;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(e) => write!(f, "IO error: {}", e),
            ParseError::Toml(e) => write!(f, "TOML parsing error: {}", e),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err)
    }
}

impl From<toml::de::Error> for ParseError {
    fn from(err: toml::de::Error) -> Self {
        ParseError::Toml(err)
    }
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<RelayFile, ParseError> {
    let content = fs::read_to_string(path)?;
    let parsed: RelayFile = toml::from_str(&content)?;
    Ok(parsed)
}

pub fn load_secrets<P: AsRef<Path>>(path: P) -> Result<std::collections::HashMap<String, String>, ParseError> {
    let content = fs::read_to_string(path)?;
    let parsed: std::collections::HashMap<String, String> = toml::from_str(&content)?;
    Ok(parsed)
}

pub fn load_collection_config<P: AsRef<Path>>(path: P) -> Result<crate::model::CollectionConfig, ParseError> {
    let content = fs::read_to_string(path)?;
    let parsed: crate::model::CollectionConfig = toml::from_str(&content)?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_parse_valid_file() {
        let path = std::env::temp_dir().join("valid_test.rl");
        let mut file = File::create(&path).unwrap();
        writeln!(file, r#"
[meta]
name = "Test"
type = "http"

[request]
method = "GET"
url = "https://example.com"
"#).unwrap();

        let parsed = parse_file(&path).unwrap();
        assert_eq!(parsed.meta.name, "Test");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_parse_invalid_toml() {
        let path = std::env::temp_dir().join("invalid_test.rl");
        let mut file = File::create(&path).unwrap();
        writeln!(file, r#"
[meta
name = "Test
"#).unwrap();

        let result = parse_file(&path);
        assert!(matches!(result, Err(ParseError::Toml(_))));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_parse_missing_file() {
        let result = parse_file("definitely_does_not_exist_xyz.rl");
        assert!(matches!(result, Err(ParseError::Io(_))));
    }

    #[test]
    fn test_load_secrets_valid() {
        let path = std::env::temp_dir().join("test_secrets.toml");
        let mut file = File::create(&path).unwrap();
        writeln!(file, r#"
api_key = "super_secret"
token = "abc123xyz"
"#).unwrap();

        let secrets = load_secrets(&path).unwrap();
        assert_eq!(secrets.get("api_key").unwrap(), "super_secret");
        assert_eq!(secrets.get("token").unwrap(), "abc123xyz");
        let _ = std::fs::remove_file(path);
    }
}
