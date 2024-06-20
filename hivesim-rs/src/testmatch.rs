use regex::Regex;

#[derive(Clone, Debug)]
pub struct TestMatcher {
    pub suite: Regex,
    pub test: Regex,
    pub pattern: String,
}

impl TestMatcher {
    pub fn new(pattern: &str) -> Self {
        let parts = Self::split_regexp(pattern);
        let suite = Regex::new(&format!("(?i:{})", parts[0])).unwrap();
        let test = if parts.len() > 1 {
            Regex::new(&format!("(?i:{})", parts[1..].join("/"))).unwrap()
        } else {
            Regex::new("").unwrap()
        };
        Self {
            suite,
            test,
            pattern: pattern.to_string(),
        }
    }

    pub fn match_test(&self, suite: &str, test: &str) -> bool {
        if !self.suite.is_match(suite) {
            return false;
        }

        if test != "" && !self.test.is_match(test) {
            return false;
        }

        true
    }

    /// split_regexp splits the pattern into /-separated parts.
    /// This is based off the golang implementation of testmatch.rs
    fn split_regexp(pattern: &str) -> Vec<&str> {
        let mut pattern = pattern;
        let mut parts = Vec::with_capacity(pattern.matches('/').count());
        let mut square_bracket_counter = 0;
        let mut parenthesis_counter = 0;
        let mut index = 0;
        while index < pattern.len() {
            match pattern.chars().nth(index).unwrap() {
                '[' => square_bracket_counter += 1,
                ']' => {
                    if square_bracket_counter > 0 {
                        square_bracket_counter -= 1;
                    }
                }
                '(' => {
                    if square_bracket_counter == 0 {
                        parenthesis_counter += 1;
                    }
                }
                ')' => {
                    if square_bracket_counter == 0 {
                        parenthesis_counter -= 1;
                    }
                }
                '\\' => {
                    index += 1;
                }
                '/' => {
                    if square_bracket_counter == 0 && parenthesis_counter == 0 {
                        parts.push(&pattern[..index]);
                        pattern = &pattern[index + 1..];
                        index = 0;
                        continue;
                    }
                }
                _ => {}
            }
            index += 1;
        }
        parts.push(pattern);
        parts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_regexp() {
        let pattern = "suite/test";
        let parts = TestMatcher::split_regexp(pattern);
        assert_eq!(parts, vec!["suite", "test"]);

        let pattern = "suite/test/1";
        let parts = TestMatcher::split_regexp(pattern);
        assert_eq!(parts, vec!["suite", "test", "1"]);

        let pattern = "suite/test/1/2";
        let parts = TestMatcher::split_regexp(pattern);
        assert_eq!(parts, vec!["suite", "test", "1", "2"]);

        let pattern = "suite/test/1/2/3";
        let parts = TestMatcher::split_regexp(pattern);
        assert_eq!(parts, vec!["suite", "test", "1", "2", "3"]);

        let pattern = "suite/test/1/2/3/4";
        let parts = TestMatcher::split_regexp(pattern);
        assert_eq!(parts, vec!["suite", "test", "1", "2", "3", "4"]);

        let pattern = "suite/test/1/2/3/4/5";
        let parts = TestMatcher::split_regexp(pattern);
        assert_eq!(parts, vec!["suite", "test", "1", "2", "3", "4", "5"]);

        let pattern = "suite/test/1/2/3/4/5/6";
        let parts = TestMatcher::split_regexp(pattern);
        assert_eq!(parts, vec!["suite", "test", "1", "2", "3", "4", "5", "6"]);

        let pattern = "suite/test/1/2/3/4/5/6/7";
        let parts = TestMatcher::split_regexp(pattern);
        assert_eq!(
            parts,
            vec!["suite", "test", "1", "2", "3", "4", "5", "6", "7"]
        );
    }

    #[test]
    fn test_match_test() {
        let matcher = TestMatcher::new("sim/test");

        assert_eq!(matcher.match_test("sim", "test"), true);
        assert_eq!(matcher.match_test("Sim", "Test"), true);
        assert_eq!(matcher.match_test("Sim", "TestTest"), true);
        assert_eq!(matcher.match_test("Sim", "Tst"), false);

        let matcher = TestMatcher::new("/test");

        assert_eq!(matcher.match_test("sim", "test"), true);
        assert_eq!(matcher.match_test("", "Test"), true);
        assert_eq!(matcher.match_test("bob", "test"), true);
    }

    #[test]
    fn test_match_suite() {
        let matcher = TestMatcher::new("sim");

        assert_eq!(matcher.match_test("sim", ""), true);
        assert_eq!(matcher.match_test("Sim", ""), true);
        assert_eq!(matcher.match_test("Sim", "Test"), true);
        assert_eq!(matcher.match_test("Sim", "Tst"), true);
        assert_eq!(matcher.match_test("Sim", "Tst"), true);
        assert_eq!(matcher.match_test("Sim", "Tst"), true);
    }
}
