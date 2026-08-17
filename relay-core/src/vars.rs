use std::collections::HashMap;

/// Interpolates variables in the format `{{var_name}}` using the provided `vars` map.
/// If a variable is not found in the map, it is left untouched.
pub fn interpolate(input: &str, vars: &HashMap<String, String>) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'{') {
            chars.next(); // consume the second '{'
            let mut var_name = String::new();
            let mut found_end = false;
            
            while let Some(vc) = chars.next() {
                if vc == '}' && chars.peek() == Some(&'}') {
                    chars.next(); // consume the second '}'
                    found_end = true;
                    break;
                }
                var_name.push(vc);
            }
            
            if found_end {
                let trimmed_var = var_name.trim();
                if let Some(val) = vars.get(trimmed_var) {
                    result.push_str(val);
                } else {
                    // Variable not found, leave it as is
                    result.push_str("{{");
                    result.push_str(&var_name);
                    result.push_str("}}");
                }
            } else {
                // Malformed syntax, didn't find closing '}}'
                result.push_str("{{");
                result.push_str(&var_name);
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Resolves variables across all four scopes into a single lookup table.
/// Precedence order (highest to lowest, meaning higher overwrites lower):
/// 1. Request
/// 2. Environment
/// 3. Collection
/// 4. Global
pub fn resolve_scopes(
    global: &HashMap<String, String>,
    collection: &HashMap<String, String>,
    env: &HashMap<String, String>,
    request: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut resolved = HashMap::new();
    
    // Insert from lowest precedence to highest
    for (k, v) in global {
        resolved.insert(k.clone(), v.clone());
    }
    for (k, v) in collection {
        resolved.insert(k.clone(), v.clone());
    }
    for (k, v) in env {
        resolved.insert(k.clone(), v.clone());
    }
    for (k, v) in request {
        resolved.insert(k.clone(), v.clone());
    }
    
    resolved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_single_var() {
        let mut vars = HashMap::new();
        vars.insert("base_url".to_string(), "https://api.example.com".to_string());
        
        let input = "{{base_url}}/users";
        let expected = "https://api.example.com/users";
        assert_eq!(interpolate(input, &vars), expected);
    }

    #[test]
    fn test_interpolate_multiple_vars() {
        let mut vars = HashMap::new();
        vars.insert("greeting".to_string(), "Hello".to_string());
        vars.insert("name".to_string(), "World".to_string());
        
        let input = "{{greeting}}, {{name}}!";
        let expected = "Hello, World!";
        assert_eq!(interpolate(input, &vars), expected);
    }

    #[test]
    fn test_interpolate_missing_var() {
        let vars = HashMap::new();
        let input = "Bearer {{token}}";
        let expected = "Bearer {{token}}"; // left untouched
        assert_eq!(interpolate(input, &vars), expected);
    }

    #[test]
    fn test_interpolate_with_spaces_inside() {
        let mut vars = HashMap::new();
        vars.insert("token".to_string(), "secret123".to_string());
        
        let input = "Authorization: Bearer {{ token }}";
        let expected = "Authorization: Bearer secret123";
        assert_eq!(interpolate(input, &vars), expected);
    }

    #[test]
    fn test_interpolate_malformed() {
        let vars = HashMap::new();
        let input = "This has an unclosed {{var";
        let expected = "This has an unclosed {{var";
        assert_eq!(interpolate(input, &vars), expected);
    }

    #[test]
    fn test_resolve_scopes_precedence() {
        let mut global = HashMap::new();
        global.insert("var_a".to_string(), "global_a".to_string());
        global.insert("var_b".to_string(), "global_b".to_string());

        let mut collection = HashMap::new();
        collection.insert("var_b".to_string(), "collection_b".to_string());
        collection.insert("var_c".to_string(), "collection_c".to_string());

        let mut env = HashMap::new();
        env.insert("var_c".to_string(), "env_c".to_string());
        env.insert("var_d".to_string(), "env_d".to_string());

        let mut request = HashMap::new();
        request.insert("var_d".to_string(), "request_d".to_string());
        request.insert("var_e".to_string(), "request_e".to_string());

        let resolved = resolve_scopes(&global, &collection, &env, &request);

        // global_a is untouched
        assert_eq!(resolved.get("var_a").unwrap(), "global_a");
        // collection_b overrides global_b
        assert_eq!(resolved.get("var_b").unwrap(), "collection_b");
        // env_c overrides collection_c
        assert_eq!(resolved.get("var_c").unwrap(), "env_c");
        // request_d overrides env_d
        assert_eq!(resolved.get("var_d").unwrap(), "request_d");
        // request_e is uniquely request
        assert_eq!(resolved.get("var_e").unwrap(), "request_e");
    }
}
