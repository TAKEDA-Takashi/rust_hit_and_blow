use rand::seq::SliceRandom;
use regex::Regex;
use std::vec::Vec;

pub struct Numbers {
    value: Vec<u8>,
}

impl Numbers {
    pub fn new(v: &Vec<u8>) -> Numbers {
        Numbers { value: v.clone() }
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
}

pub fn get_random_numbers(digit: usize) -> Vec<u8> {
    if digit < 1 || digit > 9 {
        panic!("digit is in the range 1 <= digit <= 9");
    }

    let mut v = (0..=9).collect::<Vec<u8>>();
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    v[0..digit].to_vec()
}

pub fn string_to_u8vec(s: &str) -> Vec<u8> {
    if !is_num_string(s) {
        panic!("contains not number character");
    }

    let mut v = vec![];
    for c in s.chars() {
        v.push(c.to_digit(10).unwrap() as u8);
    }

    v
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
        let n = Numbers::new(&vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_numbers_count_hit() {
        let model = Numbers::new(&vec![0, 1, 2, 3]);
        let reply = Numbers::new(&vec![0, 1, 2, 3]);
        assert_eq!(4, model.count_hit(&reply));

        let model = Numbers::new(&vec![0, 1, 2, 3]);
        let reply = Numbers::new(&vec![0, 1, 2, 4]);
        assert_eq!(3, model.count_hit(&reply));

        let model = Numbers::new(&vec![1, 2, 3, 0]);
        let reply = Numbers::new(&vec![0, 1, 2, 3]);
        assert_eq!(0, model.count_hit(&reply));
    }

    #[test]
    fn test_numbers_count_blow() {
        let model = Numbers::new(&vec![0, 1, 2, 3]);
        let reply = Numbers::new(&vec![0, 1, 2, 3]);
        assert_eq!(0, model.count_blow(&reply));

        let model = Numbers::new(&vec![0, 1, 2, 3]);
        let reply = Numbers::new(&vec![0, 1, 2, 4]);
        assert_eq!(0, model.count_blow(&reply));

        let model = Numbers::new(&vec![1, 2, 3, 0]);
        let reply = Numbers::new(&vec![0, 1, 2, 3]);
        assert_eq!(4, model.count_blow(&reply));
    }

    #[test]
    fn test_get_random_numbers() {
        let n = get_random_numbers(4);
        assert_eq!(4, n.len());

        // not duplicate
        for i in 0..(n.len()) {
            for j in (i + 1)..(n.len()) {
                assert!(n[i] != n[j]);
            }
        }
    }

    #[test]
    fn test_string_to_u8vec() {
        let s = "0123";
        let v = string_to_u8vec(s);
        assert_eq!(vec![0, 1, 2, 3], v);
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
