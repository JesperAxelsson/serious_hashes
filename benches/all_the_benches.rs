#![feature(test)]
extern crate rand;

extern crate test;
extern crate serious_hashes;

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::collections::hash_map::DefaultHasher;
    use test::Bencher;
    use std::collections::HashMap;
    use serious_hashes::*;
    use std::hash::{Hash, Hasher, BuildHasher};

    
    const VEC_COUNT: usize = 2_000;

    fn hash_test<H: Hasher, T: Hash>( s: &mut H, t: &T) -> u64 {
        t.hash(&mut *s);
        s.finish()
    }

    fn bench_u64_single<H: Hasher>(b: &mut Bencher, hash: &mut H) {
        let vec = get_random_range(VEC_COUNT as usize);

        b.iter(|| {
            for v in vec.iter() {
                test::black_box(hash_test(hash, v));
            }
        });
    }

    #[bench]
    #[ignore]
    fn u64_single_built_in_hash(b: &mut Bencher) {
        let mut h = DefaultHasher::new();
        bench_u64_single(b, &mut h);
    }

    #[bench]
    #[ignore]
    fn u64_single_murmur_x64(b: &mut Bencher) {
        let mut h = Murmur2_64a::new();
        bench_u64_single(b, &mut h);
    }

    #[bench]
    #[ignore]
    fn u64_single_u64_hash(b: &mut Bencher) {
        let mut h = U64Hash::new();
        bench_u64_single(b, &mut h);
    }

    #[bench]
    #[ignore]
    fn u64_single_id_hash(b: &mut Bencher) {
        let mut h = IdentityHash::new();
        bench_u64_single(b, &mut h);
    }


    fn bench_string_single<H: Hasher>(b: &mut Bencher, hash: &mut H) {
        let s = String::from("Hello world, Hello world");

        b.iter(|| {
            test::black_box(hash_test(hash, &s));
        });
    }

    #[bench]
    #[ignore]
    fn single_built_in(b: &mut Bencher) {
        let mut h = DefaultHasher::new();
        bench_string_single(b, &mut h);
    }

    #[bench]
    #[ignore]
    fn single_murmur_hash_x64(b: &mut Bencher) {
        let mut h = Murmur2_64a::new();
        bench_string_single(b, &mut h);
    }


    fn bench_insert_random_range<H: BuildHasher>(b: &mut Bencher, hash: H) {
        let data = get_random_range(VEC_COUNT);
        bench_insert(b, hash, &data);
    }

    fn bench_insert_linear_range<H: BuildHasher>(b: &mut Bencher, hash: H) {
        let data = get_range(VEC_COUNT);
        bench_insert(b, hash, &data);
    }

    fn bench_insert_random_order<H: BuildHasher>(b: &mut Bencher, hash: H) {
        let data = get_range_random_order(VEC_COUNT);
        bench_insert(b, hash, &data);
    }

    fn bench_insert<H: BuildHasher, T: Eq+Hash>(b: &mut Bencher, hash: H, data: &Vec<T>) {
        let mut map = HashMap::with_capacity_and_hasher(data.len(), hash);

        b.iter(|| {
            map.clear();
        
            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }
        });
    }

    fn bench_get_random_range<H: BuildHasher>(b: &mut Bencher, hash: H) {
        let data = get_random_range(VEC_COUNT);
        bench_get(b, hash, &data);
    }

    fn bench_get_linear_range<H: BuildHasher>(b: &mut Bencher, hash: H) {
        let data = get_range(VEC_COUNT);
        bench_get(b, hash, &data);
    }

    fn bench_get_random_order<H: BuildHasher>(b: &mut Bencher, hash: H) {
        let data = get_range_random_order(VEC_COUNT);
        bench_get(b, hash, &data);
    }

    fn bench_get<H: BuildHasher, T: Eq+Hash>(b: &mut Bencher, hash: H, data: &Vec<T>) {
        let mut map = HashMap::with_capacity_and_hasher(data.len(), hash);

        for s in data.iter() {
            test::black_box({
                map.entry(s).or_insert(0);
            });
        }


        b.iter(|| {

            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }


    #[bench]
    fn u64_insert_id_hash_random(b: &mut Bencher) {
        let h = IdentityHash::new();
        bench_insert_random_range(b, h);
    }

    #[bench]
    fn u64_get_id_hash_random(b: &mut Bencher) {
        let h = IdentityHash::new();
        bench_get_random_range(b, h);
    }

    #[bench]
    fn u64_insert_id_hash_linear(b: &mut Bencher) {
        let h = IdentityHash::new();
        bench_insert_linear_range(b, h);
    }

    #[bench]
    fn u64_get_id_hash_linear(b: &mut Bencher) {
        let h = IdentityHash::new();
        bench_get_linear_range(b, h);
    }


    #[bench]
    fn u64_insert_id_hash_random_order(b: &mut Bencher) {
        let h = IdentityHash::new();
        bench_insert_random_order(b, h);
    }

    #[bench]
    fn u64_get_id_hash_random_order(b: &mut Bencher) {
        let h = IdentityHash::new();
        bench_get_random_order(b, h);
    }


    #[bench]
    fn u64_insert_built_in(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = HashMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();

            for s in data.iter() {
               test::black_box(map.insert(s, s));
            }

        });
    }

    #[bench]
    fn u64_get_built_in(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map: HashMap<&u64, &u64>  = HashMap::with_capacity(data.len());

        for s in data.iter() {
            test::black_box({
                map.insert(s, s);
            });
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                    
                });
            }
        });
    }

    #[bench]
    #[ignore]
    fn u64_insert_murmur_x64(b: &mut Bencher) {
        let h = Murmur2_64a::new();
        bench_insert_random_range(b, h);
    }

     #[bench]
     #[ignore]
    fn u64_get_murmur_x64(b: &mut Bencher) {
        let h = Murmur2_64a::new();
        bench_get_random_range(b, h);
    }

    #[bench]
    #[ignore]
    fn u64_insert_u64hash(b: &mut Bencher) {
        let h = U64Hash::new();
        bench_insert_random_range(b, h);
    }

     #[bench]
     #[ignore]
    fn u64_get_u64hash(b: &mut Bencher) {
        let h = U64Hash::new();
        bench_get_random_range(b, h);
    }

    fn get_random_range(count: usize) -> Vec<u64> {
        use rand::{Rng, SeedableRng, StdRng};

        let mut vec = Vec::new();

        let seed: &[_] = &[4, 2, 4, 2];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        for _ in 0..count {
            vec.push(rng.gen::<u64>());
        }

        vec.sort();
        vec.dedup();

        vec    
    }
    
    fn get_range(count: usize) -> Vec<u64> {
        let mut vec = Vec::new();

        let count = count as u64;

        for i in 0..count {
            vec.push(i);
        }

        vec    
    }

    fn get_range_random_order(count: usize) -> Vec<u64> {
        use rand::{SeedableRng, StdRng};
        use rand::distributions::{IndependentSample, Range};

        let count = count as u64;

        let mut vec = Vec::new();

        for i in 0..count {
            vec.push(i);
        }

        let seed: &[_] = &[4, 2, 1, 2];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let range = Range::new(0, count-1);

        for i in 0..count {
            let other_i = range.ind_sample(&mut rng);
            // let car = random_door.ind_sample(rng)

            vec.swap(i as usize, other_i as usize);
        }

        vec
    }

}

 