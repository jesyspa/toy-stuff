pub struct PrimeVault {
    primes: Vec<u64>,
    next: u64,
}

fn is_prime_wrt(n: u64, primes: &[u64]) -> bool {
    for i in primes {
        if i*i > n { break; }
        if n % i == 0 { return false; }
    }
    return true;
}

impl PrimeVault {
    pub fn new() -> Self {
        PrimeVault { primes: Vec::new(), next: 2 }
    }

    pub fn extend(&mut self, limit: u64) -> Option<u64> {
        while self.next < limit && !is_prime_wrt(self.next, &self.primes) {
            self.next += 1;
        }
        if self.next < limit {
            let this = self.next;
            self.next += 1;
            self.primes.push(this);
            Some(this)
        } else {
            None
        }
    }
}

impl Default for PrimeVault {
    fn default() -> Self { Self::new() }
}

pub struct ExtendIter<'a> {
    vault: &'a mut PrimeVault,
    limit: u64,
    vault_ix: usize,
}

impl PrimeVault {
    pub fn extend_iter(&mut self, limit: u64) -> ExtendIter {
        ExtendIter { vault: self, limit, vault_ix: 0 }
    }
}

impl<'a> Iterator for ExtendIter<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let ix = self.vault_ix;
        self.vault_ix += 1;
        if ix < self.vault.primes.len() {
            let result = self.vault.primes[ix];
            if result < self.limit { Some(result) } else { None }
        } else {
            self.vault.extend(self.limit)
        }
    }
}

impl PrimeVault {
    pub fn factorize(&mut self, mut n: u64) -> Vec<u64> {
        let mut factors = Vec::new();
        for p in self.extend_iter(n+1) {
            if p > n { break; }
            while n % p == 0 {
                n /= p;
                factors.push(p);
            }
            if n == 1 { break; }
        }
        factors
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_extend_manually() {
        let mut vault = PrimeVault::new();
        assert_eq!(vault.extend(10), Some(2));
        assert_eq!(vault.extend(10), Some(3));
        assert_eq!(vault.extend(10), Some(5));
        assert_eq!(vault.primes, vec![2, 3, 5]);
        assert_eq!(vault.next, 6);
    }

    #[test]
    fn can_extend_with_limit_manually() {
        let mut vault = PrimeVault::new();
        assert_eq!(vault.extend(3), Some(2));
        assert_eq!(vault.extend(3), None);
        assert_eq!(vault.extend(4), Some(3));
        assert_eq!(vault.extend(5), None);
        assert_eq!(vault.primes, vec![2, 3]);
        assert_eq!(vault.next, 5);
    }

    #[test]
    fn can_extend_through_iter() {
        let mut vault = PrimeVault::new();
        assert_eq!(vault.extend_iter(10).collect::<Vec<_>>(), vec![2, 3, 5, 7]);
        assert_eq!(vault.primes, vec![2, 3, 5, 7]);
        assert_eq!(vault.next, 10);
    }

    #[test]
    fn can_extend_through_iter_twice() {
        let mut vault = PrimeVault::new();
        assert_eq!(vault.extend_iter(5).collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(vault.extend_iter(10).collect::<Vec<_>>(), vec![2, 3, 5, 7]);
        assert_eq!(vault.primes, vec![2, 3, 5, 7]);
        assert_eq!(vault.next, 10);
    }

    #[test]
    fn can_iter_through_part() {
        let mut vault = PrimeVault::new();
        assert_eq!(vault.extend_iter(10).collect::<Vec<_>>(), vec![2, 3, 5, 7]);
        assert_eq!(vault.extend_iter(5).collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(vault.primes, vec![2, 3, 5, 7]);
        assert_eq!(vault.next, 10);
    }

    #[test]
    fn factorize() {
        let mut vault = PrimeVault::new();
        assert_eq!(vault.factorize(10), vec![2, 5]);
        assert_eq!(vault.factorize(2), vec![2]);
        assert_eq!(vault.factorize(1), vec![]);
        assert_eq!(vault.factorize(8), vec![2, 2, 2]);
        assert_eq!(vault.factorize(36), vec![2, 2, 3, 3]);
        assert_eq!(vault.primes, vec![2, 3, 5]);
        assert_eq!(vault.next, 6);
        assert_eq!(vault.factorize(38), vec![2, 19]);
        assert_eq!(vault.primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(vault.factorize(37), vec![37]);
    }
}
