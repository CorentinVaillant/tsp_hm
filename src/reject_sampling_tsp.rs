use rand::seq::SliceRandom;
use crate::support_tsp::{total_length, DistancesMat, Permutation};

pub fn reject_sample_tsp<const N:usize>(distances:DistancesMat<N>,iteration:usize)->Permutation<N>{
    let mut rng = rand::rng();

    let mut p_t:Permutation<N> = core::array::from_fn(|i|i);
    p_t.shuffle(&mut rng);
    let mut best = p_t;
    let mut best_dist = total_length(&best, &distances);

    for _ in 0..iteration{
        p_t.shuffle(&mut rng);
        let p_t_length = total_length(&p_t, &distances);

        if p_t_length < best_dist{
            best = p_t;
            best_dist = p_t_length;
        }
    }
    
    best
}

pub fn uniform_sample_tsp<const N:usize>(iteration:usize)->Vec<Permutation<N>>{
    let mut rng = rand::rng();

    let mut p_t:Permutation<N> = core::array::from_fn(|i|i);
    p_t.shuffle(&mut rng);
    let mut sample = Vec::with_capacity(iteration);

    for _ in 0..iteration{
        p_t.shuffle(&mut rng);
        sample.push(p_t);
    }
    sample
}