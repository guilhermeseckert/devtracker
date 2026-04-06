use regex::Regex;
use std::sync::LazyLock;

static JIRA_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([A-Z][A-Z0-9]+-\d+)").unwrap());

/// Extract a Jira ticket ID from a branch name.
/// e.g. "feature/PROJ-123-add-login" -> Some("PROJ-123")
pub fn extract_ticket(branch: &str) -> Option<String> {
    JIRA_PATTERN
        .find(branch)
        .map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ticket() {
        assert_eq!(extract_ticket("feature/PROJ-123-add-login"), Some("PROJ-123".to_string()));
        assert_eq!(extract_ticket("PROJ-456/fix-thing"), Some("PROJ-456".to_string()));
        assert_eq!(extract_ticket("bugfix/ABC-1"), Some("ABC-1".to_string()));
        assert_eq!(extract_ticket("main"), None);
        assert_eq!(extract_ticket("develop"), None);
        assert_eq!(extract_ticket("feature/no-ticket-here"), None);
    }
}
