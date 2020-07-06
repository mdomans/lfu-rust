#[macro_use]
extern crate criterion;
extern crate bytes;
extern crate lfu;
extern crate rand;

use bytes::Bytes;
use criterion::{Criterion, Fun};
use rand::{Rng, SeedableRng, XorShiftRng};

fn insert_and_lookup_standard(mut n: u64) {
    let mut rng: XorShiftRng = SeedableRng::from_seed([1981, 1986, 2003, 2011]);
    let mut hash_map = ::std::collections::HashMap::new();

    while n != 0 {
        let key: String = (0..10).map(|_| rand::random::<u8>() as char).collect();
        if rng.gen::<bool>() {
            let value = Bytes::from((0..10).map(|_| rand::random::<u8>()).collect::<Vec<u8>>());
            hash_map.insert(key, value);
        } else {
            hash_map.get(&key);
        }
        n -= 1;
    }
}

fn insert_and_lookup_naive(mut n: u64) {
    let mut rng: XorShiftRng = SeedableRng::from_seed([1981, 1986, 2003, 2011]);
    let mut hash_map = lfu::LFU::new();

    while n != 0 {
        let key: String = (0..10).map(|_| rand::random::<u8>() as char).collect();
        if rng.gen::<bool>() {
            let value = Bytes::from((0..10).map(|_| rand::random::<u8>()).collect::<Vec<u8>>());
            hash_map.insert(key, value);
        } else {
            hash_map.get(&key);
        }
        n -= 1;
    }
}

macro_rules! insert_lookup {
    ($fn:ident, $s:expr) => {
        fn $fn(c: &mut Criterion) {
            let naive = Fun::new("naive", |b, i| b.iter(|| insert_and_lookup_naive(*i)));
            let standard = Fun::new("standard", |b, i| b.iter(|| insert_and_lookup_standard(*i)));

            let functions = vec![naive, standard];
            c.bench_functions(&format!("HashMap/{}", $s), functions, $s);
        }
    };
}

insert_lookup!(insert_lookup_1, 1);
insert_lookup!(insert_lookup_10, 10);
insert_lookup!(insert_lookup_100, 100);

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = insert_lookup_1, insert_lookup_10, insert_lookup_100
);

criterion_main!(benches);
