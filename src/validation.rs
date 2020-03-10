use regex::Regex;

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
    s.len() == length
}

pub fn is_num_string(s: &str) -> bool {
    let re = Regex::new(r"^\d+$").unwrap();
    re.is_match(s)
}

pub fn is_duplicate(s: &str) -> bool {
    let v: Vec<char> = s.chars().collect();

    for i in 0..(v.len()) {
        for j in (i + 1)..(v.len()) {
            if v[i] == v[j] {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_match_length() {
        assert!(is_match_length("0123", 4));
        assert!(is_match_length("012345", 6));
        assert!(is_match_length("", 0));
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
        let e = v.validate("0123");
        assert!(e.is_ok());

        let e = v.validate("01234");
        match e {
            Ok(_) => panic!("assert error"),
            Err(kind) => assert_eq!(ValidationError::MatchLength, kind),
        }

        let e = v.validate("012a");
        match e {
            Ok(_) => panic!("assert error"),
            Err(kind) => assert_eq!(ValidationError::NumString, kind),
        }

        let e = v.validate("0121");
        match e {
            Ok(_) => panic!("assert error"),
            Err(kind) => assert_eq!(ValidationError::NotDuplicate, kind),
        }
    }
}
