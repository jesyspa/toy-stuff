use super::MODULUS;

// Our table layout is non-trivial here.
// We only care about (n, k) pairs where k ≤ n, since if n < k1, then f(n, k1) = f(n, n).
// We need to compute f(i, 0) through f(i, i) before we can compute f(i+1, j), and
// that is the order we use for our table.
// f(i, 0) starts at index i*(i+1)/2.  The total size is one past the last index, i.e.
struct TableSumSplitter(Vec<u64>);

fn get_index(n: u64, k: u64) -> usize {
    (n * (n + 1) / 2 + k) as usize
}

impl TableSumSplitter {
    pub fn new_for(n: u64) -> Self {
        // capacity needs to be one greater than max index
        let mut v = Vec::with_capacity(get_index(n + 1, 0));
        v.push(1);
        v.push(0);
        v.push(1);
        Self(v)
    }

    pub fn read(&self, n: u64, k: u64) -> u64 {
        self.0[get_index(n, k.min(n))]
    }

    // Note that by construction we know k ≤ n.
    pub fn compute_impl(&self, n: u64, k: u64) -> u64 {
        (self.read(n, k-1) + self.read(n-k, k)) % MODULUS
    }

    pub fn compute(&mut self, n_orig: u64) -> u64 {
        for n in 2..=n_orig {
            self.0.push(0);
            self.0.push(1);
            for k in 2..=n {
                let v = self.compute_impl(n, k);
                self.0.push(v);
            }
        }
        self.0[get_index(n_orig, n_orig)]
    }
}

pub fn table_split(n: u64) -> u64 {
    TableSumSplitter::new_for(n).compute(n)
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
