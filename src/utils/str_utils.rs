pub fn split_into_segments(s: &str, segment_len: usize) -> Vec<&str> {
    let mut segments = Vec::new();
    let mut start = 0;
    while start < s.len() {
        let end = start + segment_len;
        segments.push(&s[start..end]);
        start = end;
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_segments() {
        let res = split_into_segments("123123123", 3);
        assert_eq!(res, vec!["123", "123", "123"])
    }
}
