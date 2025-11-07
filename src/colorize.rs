pub struct ManColorizer;

impl ManColorizer {
    pub fn new(_theme: crate::app::Theme) -> Self {
        Self
    }

    pub fn is_section_header(&self, line: &str) -> bool {
        // Section headers are typically all caps, 2-20 chars, possibly with numbers
        if line.len() < 2 || line.len() > 30 {
            return false;
        }

        let uppercase_ratio = line
            .chars()
            .filter(|c| c.is_uppercase() || c.is_whitespace() || c.is_ascii_digit() || *c == '-')
            .count() as f32
            / line.len() as f32;

        // Common section header names
        let common_headers = [
            "NAME",
            "SYNOPSIS",
            "DESCRIPTION",
            "OPTIONS",
            "ARGUMENTS",
            "EXAMPLES",
            "FILES",
            "SEE ALSO",
            "AUTHOR",
            "COPYRIGHT",
            "BUGS",
            "HISTORY",
            "NOTES",
            "EXIT STATUS",
            "ENVIRONMENT",
        ];

        uppercase_ratio > 0.7
            && (line
                .chars()
                .all(|c| c.is_uppercase() || c.is_whitespace() || c == '-')
                || common_headers.iter().any(|&h| line.starts_with(h)))
    }

    pub fn looks_like_code(&self, line: &str) -> bool {
        // Check for code-like patterns: brackets, pipes, backticks, etc.
        line.contains('[') && line.contains(']')
            || line.contains('|')
            || line.contains('<') && line.contains('>')
            || line.contains('{') && line.contains('}')
            || (line.contains("()") || line.contains("[]"))
    }
}
