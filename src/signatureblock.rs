use regex::Regex;

/// Remove the `-- ` signature from a message.
/// https://en.wikipedia.org/wiki/Signature_block#Standard_delimiter
pub fn remove(str: String) -> String {
    let pattern = Regex::new(r"(?m)^-- ?(?s).*$").unwrap();
    pattern.replace_all(&str, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_sig() {
        let input = "Dear Alice,\nI hope you are well\n";
        assert_eq!(input.to_string(), remove(input.to_string()));
    }

    #[test]
    fn test_with_rfc3676_sig() {
        let input = "Dear Alice,\nI hope you are well\n-- \nEddie\nyour shipboard computer";
        assert_eq!(
            "Dear Alice,\nI hope you are well\n".to_string(),
            remove(input.to_string())
        );
    }

    #[test]
    fn test_with_missing_space_sig() {
        let input = "Dear Alice,\nI hope you are well\n--\nEddie\nyour shipboard computer";
        assert_eq!(
            "Dear Alice,\nI hope you are well\n".to_string(),
            remove(input.to_string())
        );
    }
}
