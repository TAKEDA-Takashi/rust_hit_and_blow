use regex::Regex;

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
}
