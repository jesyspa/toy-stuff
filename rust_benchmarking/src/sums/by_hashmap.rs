use super::MODULUS;
use std::collections::HashMap;

#[derive(Default)]
pub struct MemoSumSplitter(HashMap<(u64, u64), u64>);

impl MemoSumSplitter {
    // Split n into sums with summands at most k
    pub fn split_rec_impl(&mut self, n: u64, k: u64) -> u64 {
        if n == 0 || k == 1 {
            return 1;
        }
        if k > n { return self.split_rec(n, n); }
        (self.split_rec(n, k-1) + self.split_rec(n-k, k)) % MODULUS
    }

    pub fn split_rec(&mut self, n: u64, k: u64) -> u64 {
        if let Some(&v) = self.0.get(&(n, k)) {
            return v;
        }
        let v = self.split_rec_impl(n, k);
        self.0.insert((n, k), v);
        v
    }

    pub fn split(&mut self, n: u64) -> u64 {
        self.split_rec(n, n)
    }
}

pub fn memo_split(n: u64) -> u64 {
    MemoSumSplitter::default().split(n)
}

#[cfg(test)]
mod tests {
    use super::{memo_split, MemoSumSplitter};
    #[test]
    fn test_trivial() {
        assert_eq!(memo_split(1), 1);
        assert_eq!(memo_split(2), 2);
        assert_eq!(memo_split(3), 3);
        assert_eq!(memo_split(4), 5);
        assert_eq!(memo_split(5), 7);
    }

    #[test]
    fn test_memo_reuse() {
        let mut m = MemoSumSplitter::default();
        for _ in 0..4 {
            assert_eq!(m.split(1), 1);
            assert_eq!(m.split(2), 2);
            assert_eq!(m.split(3), 3);
            assert_eq!(m.split(4), 5);
            assert_eq!(m.split(5), 7);
        }
    }

    #[test]
    fn test_slow() {
        assert!(memo_split(250) > 0);
    }
}
