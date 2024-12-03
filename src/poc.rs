fn decode_string(s: &str) -> u64 {
    let digits = s
        .chars()
        .fold((None, None), |acc, c| -> (Option<char>, Option<char>) {
            if !c.is_ascii_digit() {
                return acc;
            }
            match acc {
                (None, None) => (Some(c), Some(c)),
                (Some(c1), _) => (Some(c1), Some(c)),
                // This should be impossible
                _ => acc,
            }
        });
    let n_str = vec![digits.0.unwrap(), digits.1.unwrap()]
        .iter()
        .collect::<String>();

    n_str.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::poc::decode_string;

    #[test]
    fn test_basic_decode() {
        let basic_data = vec![
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for (s, n) in basic_data {
            assert_eq!(decode_string(s), n);
        }
    }
}
