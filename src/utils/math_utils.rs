pub fn find_divisors(v: &str) -> Vec<usize> {
    let n = v.len();
    let mut divisors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);
            let pair = n / i;
            if pair != i {
                divisors.push(pair);
            }
        }
    }
    divisors.sort(); // Optional: if you need sorted order
    divisors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_divisors() {
        let mut res: Vec<usize> = find_divisors("123123123");
        assert_eq!(res, vec![1, 3, 9]);

        res = find_divisors("22");
        assert_eq!(res, vec![1, 2]);

        res = find_divisors("38593859");
        assert_eq!(res, vec![1, 2, 4, 8])
    }
}
