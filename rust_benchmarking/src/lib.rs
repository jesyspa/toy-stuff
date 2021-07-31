#![feature(test)]
#[cfg(test)]

extern crate test;
extern crate rand;

pub mod array_of_pointers;
pub mod pointer_to_array;
mod common;

#[allow(soft_unstable)]
#[cfg(test)]
mod benchmarks {
    use crate::array_of_pointers;
    use crate::pointer_to_array;
    use test::Bencher;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    const MIN_STR_LENGTH: usize = 2;
    const MAX_STR_LENGTH: usize = 8;
    const MIN_STR_VALUE: i32 = 0;
    const MAX_STR_VALUE: i32 = 1000;
    const MIN_PREFIX_LENGTH: usize = 0;
    const MAX_PREFIX_LENGTH: usize = 8;
    const NUM_INSTRUCTIONS: usize = 10000;
    const SEED: u64 = 12345678;

    fn rand_string<T>(rng: &mut T, len: usize) -> String
    where T: Rng
    {
        let mut string = String::new();
        for _ in 0..len {
            string.push(rng.gen_range('a'..='z'))
        }
        string
    }

    fn rand_insertion<T>(rng: &mut T) -> (String, i32)
    where T: Rng
    {
        let len = rng.gen_range(MIN_STR_LENGTH ..= MAX_STR_LENGTH);
        let value = rng.gen_range(MIN_STR_VALUE ..= MAX_STR_VALUE);
        let string = rand_string(rng, len);
        (string, value)
    }

    fn rand_query<T>(rng: &mut T) -> String
    where T: Rng
    {
        let len = rng.gen_range(MIN_PREFIX_LENGTH ..= MAX_PREFIX_LENGTH);
        rand_string(rng, len)
    }

    #[derive(Default)]
    struct Inputs {
        insertions: Vec<(String, i32)>,
        queries: Vec<String>
    }

    impl Inputs {
        fn new(n: usize) -> Inputs {
            let mut rng: StdRng = SeedableRng::seed_from_u64(SEED);
            let mut inputs: Inputs = Default::default();
            for _ in 0..n {
                inputs.insertions.push(rand_insertion(&mut rng));
            }
            for _ in 0..n {
                inputs.queries.push(rand_query(&mut rng));
            }
            inputs
        }
    }

    #[bench]
    fn bench_pointer_to_array(b: &mut Bencher) {
        let inputs = Inputs::new(NUM_INSTRUCTIONS);
        b.iter(|| {
            let mut total: i32 = 0;
            let mut ms = pointer_to_array::MapSum::new();
            for (s, v) in &inputs.insertions {
                ms.insert(s, *v);
            }
            for s in &inputs.queries {
                total += ms.sum(s);
            }
            total
        });
    }

    #[bench]
    fn bench_array_of_pointers(b: &mut Bencher) {
        let inputs = Inputs::new(NUM_INSTRUCTIONS);
        b.iter(|| {
            let mut total: i32 = 0;
            let mut ms = array_of_pointers::MapSum::new();
            for (s, v) in &inputs.insertions {
                ms.insert(s, *v);
            }
            for s in &inputs.queries {
                total += ms.sum(s);
            }
            total
        });
    }
}
