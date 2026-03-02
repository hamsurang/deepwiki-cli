pub fn format_for_claude(text: &str, repo: &str, query_type: &str) -> String {
    format!("## DeepWiki: {} ({})\n\n{}", repo, query_type, text.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_adds_header() {
        let result = format_for_claude("some content", "facebook/react", "ask");
        assert!(result.starts_with("## DeepWiki: facebook/react (ask)"));
        assert!(result.contains("some content"));
    }

    #[test]
    fn test_format_trims_whitespace() {
        let result = format_for_claude("  content  \n\n", "owner/repo", "structure");
        assert!(result.ends_with("content"));
    }
}
