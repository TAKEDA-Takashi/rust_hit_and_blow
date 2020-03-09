use rand::seq::SliceRandom;
use regex::Regex;
use std::str::FromStr;
use std::vec::Vec;

pub struct Numbers {
    value: Vec<u8>,
}

impl Numbers {
    pub fn new(digit: usize) -> Numbers {
        if digit < 1 || digit > 9 {
            panic!("digit is in the range 1 <= digit <= 9");
        }

        let mut v = (0..=9).collect::<Vec<u8>>();
        let mut rng = rand::thread_rng();
        v.shuffle(&mut rng);
        Numbers {
            value: v[0..digit].to_vec(),
        }
    }

    pub fn count_hit(&self, reply: &Numbers) -> usize {
        if self.value.len() != reply.value.len() {
            panic!("length not match");
        }
        self.value
            .iter()
            .zip(reply.value.iter())
            .filter(|(a, b)| a == b)
            .count()
    }

    pub fn count_blow(&self, reply: &Numbers) -> usize {
        if self.value.len() != reply.value.len() {
            panic!("length not match");
        }

        let mut blow = 0;

        for i in 0..(self.value.len()) {
            for j in 0..(reply.value.len()) {
                if i == j {
                    continue;
                }

                if self.value[i] == reply.value[j] {
                    blow += 1;
                }
            }
        }

        blow
    }

    fn from_vec(v: &Vec<u8>) -> Numbers {
        Numbers { value: v.clone() }
    }
}

impl FromStr for Numbers {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !is_num_string(s) {
            return Err("contains not number character".to_string());
        }

        if is_duplicate(s) {
            return Err("duplicate numbers".to_string());
        }

        let mut v = vec![];
        for c in s.chars() {
            v.push(c.to_digit(10).unwrap() as u8);
        }

        Ok(Self::from_vec(&v))
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
mod tests {
    use super::*;

    #[test]
    fn test_numbers_new() {
        let n = Numbers::new(4);
        assert_eq!(4, n.value.len());

        // not duplicate
        for i in 0..(n.value.len()) {
            for j in (i + 1)..(n.value.len()) {
                assert!(n.value[i] != n.value[j]);
            }
        }
    }

    #[test]
    fn test_numbers_from_str() {
        assert!(Numbers::from_str("0123").is_ok());
        assert!(Numbers::from_str("012a").is_err());
        assert!(Numbers::from_str("0120").is_err());
    }

    #[test]
    fn test_numbers_count_hit() {
        let model = Numbers::from_vec(&vec![0, 1, 2, 3]);
        let reply = Numbers::from_vec(&vec![0, 1, 2, 3]);
        assert_eq!(4, model.count_hit(&reply));

        let model = Numbers::from_vec(&vec![0, 1, 2, 3]);
        let reply = Numbers::from_vec(&vec![0, 1, 2, 4]);
        assert_eq!(3, model.count_hit(&reply));

        let model = Numbers::from_vec(&vec![1, 2, 3, 0]);
        let reply = Numbers::from_vec(&vec![0, 1, 2, 3]);
        assert_eq!(0, model.count_hit(&reply));
    }

    #[test]
    fn test_numbers_count_blow() {
        let model = Numbers::from_vec(&vec![0, 1, 2, 3]);
        let reply = Numbers::from_vec(&vec![0, 1, 2, 3]);
        assert_eq!(0, model.count_blow(&reply));

        let model = Numbers::from_vec(&vec![0, 1, 2, 3]);
        let reply = Numbers::from_vec(&vec![0, 1, 2, 4]);
        assert_eq!(0, model.count_blow(&reply));

        let model = Numbers::from_vec(&vec![1, 2, 3, 0]);
        let reply = Numbers::from_vec(&vec![0, 1, 2, 3]);
        assert_eq!(4, model.count_blow(&reply));
    }

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
