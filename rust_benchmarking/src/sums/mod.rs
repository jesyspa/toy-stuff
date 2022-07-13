pub mod by_hashmap;
pub mod by_table;

const MODULUS: u64 = 100000007;

#[allow(soft_unstable)]
#[cfg(test)]
mod benchmarks {
    use super::by_hashmap;
    use super::by_table;
    use test::Bencher;

    const BENCH_VALUES: [u64; 6] = [5, 10, 20, 40, 80, 160];

    #[test]
    fn test_correctness() {
        for i in 1..100 {
            assert_eq!(by_hashmap::memo_split(i), by_table::table_split(i));
        }
    }

    #[bench]
    fn bench_by_hashmap(b: &mut Bencher) {
        b.iter(|| {
            let mut v = 0;
            for n in BENCH_VALUES {
                v += by_hashmap::memo_split(n)
            }
            v
        });
    }

    #[bench]
    fn bench_by_table(b: &mut Bencher) {
        b.iter(|| {
            let mut v = 0;
            for n in BENCH_VALUES {
                v += by_table::table_split(n)
            }
            v
        });
    }
}
