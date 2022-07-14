use super::MODULUS;

pub fn table_split(n: u64) -> u64 {
    if n < 2 { return 1; }
    let n = n as usize;
    let mut done = vec![1; n+1];
    let mut work = vec![0; n+1];
    for i in 2..=n {
        for j in 0..i {
            work[j] = done[j];
        }
        for j in i..=n {
            work[j] = work[j-i] + done[j] % MODULUS;
        }
        std::mem::swap(&mut done, &mut work);
    }
    return done[n];
}

#[cfg(test)]
mod tests {
    use super::table_split;
    #[test]
    fn test_trivial() {
        assert_eq!(table_split(1), 1);
        assert_eq!(table_split(2), 2);
        assert_eq!(table_split(3), 3);
        assert_eq!(table_split(4), 5);
        assert_eq!(table_split(5), 7);
    }

    #[test]
    fn test_slow() {
        assert!(table_split(250) > 0);
    }
}
