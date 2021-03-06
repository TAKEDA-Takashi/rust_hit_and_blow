use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    MatchLength,
    NumString,
    NotDuplicate,
}

pub struct Validator {
    digit: usize,
}

impl Validator {
    pub fn new(digit: usize) -> Validator {
        Validator { digit }
    }

    pub fn validate(&self, s: &str) -> Result<(), ValidationError> {
        if !is_match_length(s, self.digit) {
            return Err(ValidationError::MatchLength);
        }

        if !is_num_string(s) {
            return Err(ValidationError::NumString);
        }

        if is_duplicate(s) {
            return Err(ValidationError::NotDuplicate);
        }

        Ok(())
    }
}

pub fn is_match_length(s: &str, length: usize) -> bool {
    s.len() == length && s.chars().count() == length
}

pub fn is_num_string(s: &str) -> bool {
    let re = Regex::new(r"^\d+$").unwrap();
    re.is_match(s)
}

pub fn is_duplicate(s: &str) -> bool {
    s.len() != s.chars().collect::<HashSet<char>>().len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_match_length() {
        assert!(is_match_length("0123", 4));
        assert!(is_match_length("012345", 6));
        assert!(is_match_length("", 0));
        assert!(!is_match_length("１2", 4));
    }

    #[test]
    fn test_is_num_string() {
        let s = "0123";
        assert!(is_num_string(s));

        let s = "012a";
        assert!(!is_num_string(s));
    }

    #[test]
    fn test_is_duplicate() {
        let s = "0123";
        assert!(!is_duplicate(s));

        let s = "0133";
        assert!(is_duplicate(s));

        let s = "1231";
        assert!(is_duplicate(s));
    }

    #[test]
    fn test_validator_validate() {
        let v = Validator::new(4);

        assert!(v.validate("0123").is_ok());

        assert_eq!(Err(ValidationError::MatchLength), v.validate("01234"));

        assert_eq!(Err(ValidationError::NumString), v.validate("012a"));

        assert_eq!(Err(ValidationError::NotDuplicate), v.validate("0121"));
    }
}
