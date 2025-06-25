#[cfg(test)]
mod tests {
    use super::*;
    use crate::weaver_forge::WeaverForge;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_weaver_config_parsing() {
        // Test that we can parse the weaver.yaml configuration
        let config = WeaverForge::new("weaver.yaml");
        assert!(config.is_ok(), "Failed to parse weaver.yaml");
        
        let forge = config.unwrap();
        // Verify basic configuration loaded
        assert_eq!(forge.config.version, "0.4.0");
        assert_eq!(forge.config.target, "rust");
    }

    #[test]
    fn test_simple_jq_filters() {
        let forge = WeaverForge::new("weaver.yaml").unwrap();
        
        // Test identity filter
        let data = serde_json::json!({"key": "value"});
        let result = forge.process_with_jq(&data, ".");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data);
        
        // Test field access
        let data = serde_json::json!({"field": {"subfield": "test"}});
        let result = forge.process_with_jq(&data, ".field.subfield");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), serde_json::json!("test"));
    }

    #[test]
    fn test_group_attributes() {
        let forge = WeaverForge::new("weaver.yaml").unwrap();
        
        // Test attribute grouping
        let data = serde_json::json!([
            {
                "groups": [{
                    "attributes": [
                        {"id": "http.request.method"},
                        {"id": "http.response.status"},
                        {"id": "db.operation"}
                    ]
                }]
            }
        ]);
        
        let result = forge.group_attributes(&data);
        assert!(result.is_ok());
        
        let grouped = result.unwrap();
        assert!(grouped.get("http").is_some());
        assert!(grouped.get("db").is_some());
    }

    #[test]
    fn test_case_filters() {
        // Test the custom filters
        assert_eq!(super::snake_case_filter("HelloWorld"), "hello_world");
        assert_eq!(super::camel_case_filter("hello_world"), "helloWorld");
        assert_eq!(super::pascal_case_filter("hello_world"), "HelloWorld");
        assert_eq!(super::kebab_case_filter("hello_world"), "hello-world");
        assert_eq!(super::screaming_snake_case_filter("hello_world"), "HELLO_WORLD");
    }

    #[test]
    fn test_validation_loop() {
        // 80/20 validation loop: ensure core functionality works
        let test_cases = vec![
            (".", serde_json::json!({"test": "data"}), serde_json::json!({"test": "data"})),
            (".test", serde_json::json!({"test": "value"}), serde_json::json!("value")),
            (".missing", serde_json::json!({"test": "value"}), serde_json::json!(null)),
        ];
        
        let forge = WeaverForge::new("weaver.yaml").unwrap();
        
        for (filter, input, expected) in test_cases {
            let result = forge.process_with_jq(&input, filter);
            assert!(result.is_ok(), "Filter {} failed", filter);
            assert_eq!(result.unwrap(), expected, "Filter {} produced unexpected result", filter);
        }
    }
}