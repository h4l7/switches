use rand::{thread_rng, Rng};
use std::{collections::HashSet, hash::Hash};

/// https://stackoverflow.com/questions/2394246/algorithm-to-select-a-single-random-combination-of-values
pub fn rand_combination<T: Copy + Eq + Hash + PartialEq>(xs: &[T], k: usize) -> HashSet<T> {
    assert!(xs.len() >= k);

    let n = xs.len();
    let mut rng = thread_rng();
    let mut ys = HashSet::<T>::new();

    for j in n - k..n {
        let t: usize = rng.gen_range(0..=j);

        if !ys.contains(&xs[t]) {
            ys.insert(xs[t]);
        } else {
            ys.insert(xs[j]);
        }
    }

    ys
}
