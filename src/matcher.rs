use crate::error::AppError;
use regex::Regex;

// The contract — every match strategy must implement this
pub trait Matcher: Send + Sync {
    fn is_match(&self, line: &str) -> bool;
}


// Strategy 1 — plain literal search
pub struct LiteralMatcher {
    pattern: String,
}

impl LiteralMatcher {
    pub fn new(pattern: &str) -> Self {
        Self { pattern: pattern.to_string() }
    }
}

impl Matcher for LiteralMatcher {
    fn is_match(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }
}


// Strategy 2 — case insensitive search
pub struct CaseInsensitiveMatcher {
    pattern: String,  // stored already lowercased
}

impl CaseInsensitiveMatcher {
    pub fn new(pattern: &str) -> Self {
        Self { pattern: pattern.to_lowercase() }
    }
}

impl Matcher for CaseInsensitiveMatcher {
    fn is_match(&self, line: &str) -> bool {
        line.to_lowercase().contains(&self.pattern)
    }
}


// Strategy 3 — full regex search
pub struct RegexMatcher {
    re: Regex,
}

impl RegexMatcher {
    pub fn new(pattern: &str) -> Result<Self, AppError> {
        let re = Regex::new(pattern)
            .map_err(|e| AppError::InvalidPattern(e.to_string()))?;
        Ok(Self { re })
    }
}

impl Matcher for RegexMatcher {
    fn is_match(&self, line: &str) -> bool {
        self.re.is_match(line)
    }
}


// Factory — builds the right matcher based on flags
pub fn build_matcher(
    pattern: &str,
    ignore_case: bool,
    use_regex: bool,
) -> Result<Box<dyn Matcher>, AppError> {
    if use_regex {
        Ok(Box::new(RegexMatcher::new(pattern)?))
    } else if ignore_case {
        Ok(Box::new(CaseInsensitiveMatcher::new(pattern)))
    } else {
        Ok(Box::new(LiteralMatcher::new(pattern)))
    }
}