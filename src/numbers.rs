use rand::seq::SliceRandom;
use std::str::FromStr;
use std::vec::Vec;

use crate::validation;

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

    pub fn count_hit(&self, reply: &Numbers) -> Option<usize> {
        if self.value.len() != reply.value.len() {
            return None;
        }
        Some(
            self.value
                .iter()
                .zip(reply.value.iter())
                .filter(|(a, b)| a == b)
                .count(),
        )
    }

    pub fn count_blow(&self, reply: &Numbers) -> Option<usize> {
        if self.value.len() != reply.value.len() {
            return None;
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

        Some(blow)
    }
}

impl FromStr for Numbers {
    type Err = ParseNumbersError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !validation::is_num_string(s) {
            return Err(ParseNumbersError::ContainsNotNumber);
        }

        if validation::is_duplicate(s) {
            return Err(ParseNumbersError::DupulicateNumber);
        }

        let mut v = vec![];
        for c in s.chars() {
            v.push(c.to_digit(10).unwrap() as u8);
        }

        Ok(Numbers { value: v })
    }
}

#[derive(Debug)]
pub enum ParseNumbersError {
    ContainsNotNumber,
    DupulicateNumber,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_vec(v: Vec<u8>) -> Numbers {
        Numbers { value: v }
    }

    #[test]
    fn test_new() {
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
    fn test_from_str() {
        assert!(Numbers::from_str("0123").is_ok());
        assert!(Numbers::from_str("012a").is_err());
        assert!(Numbers::from_str("0120").is_err());
    }

    #[test]
    fn test_count_hit() {
        let model = from_vec(vec![0, 1, 2, 3]);
        let reply = from_vec(vec![0, 1, 2, 3]);
        assert_eq!(4, model.count_hit(&reply).unwrap());

        let model = from_vec(vec![0, 1, 2, 3]);
        let reply = from_vec(vec![0, 1, 2, 4]);
        assert_eq!(3, model.count_hit(&reply).unwrap());

        let model = from_vec(vec![1, 2, 3, 0]);
        let reply = from_vec(vec![0, 1, 2, 3]);
        assert_eq!(0, model.count_hit(&reply).unwrap());

        let model = from_vec(vec![0, 1, 2, 3]);
        let reply = from_vec(vec![0, 1, 2, 3, 4]);
        assert!(model.count_hit(&reply).is_none());
    }

    #[test]
    fn test_count_blow() {
        let model = from_vec(vec![0, 1, 2, 3]);
        let reply = from_vec(vec![0, 1, 2, 3]);
        assert_eq!(0, model.count_blow(&reply).unwrap());

        let model = from_vec(vec![0, 1, 2, 3]);
        let reply = from_vec(vec![0, 1, 2, 4]);
        assert_eq!(0, model.count_blow(&reply).unwrap());

        let model = from_vec(vec![1, 2, 3, 0]);
        let reply = from_vec(vec![0, 1, 2, 3]);
        assert_eq!(4, model.count_blow(&reply).unwrap());

        let model = from_vec(vec![0, 1, 2, 3]);
        let reply = from_vec(vec![0, 1, 2, 3, 4]);
        assert!(model.count_blow(&reply).is_none());
    }
}
