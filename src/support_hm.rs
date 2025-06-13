use crate::support_tsp::{total_length, DistancesMat, Permutation};


pub fn hasting_met_tsp<const N:usize>(distances:DistancesMat<N>,beta:f64, iteration:usize)->Permutation<N>{
    let mut p_t = core::array::from_fn(|i|i);
    let mut best = p_t;
    let mut best_dist = total_length(&best, &distances);

    for _ in 0..iteration{
        let p_prime = permute(p_t);

        let pi_pt = tsp_distribution(&p_t, &distances, beta);
        let pi_p = tsp_distribution(&p_prime, &distances, beta);

        //Symetric proposition:
        let alpha = if pi_pt > 0.{
            let a = pi_p/pi_pt;
            f64::min(1., a)
        }else{
            1.
        };

        let u = rand::random::<f64>();

        //taking the draw
        if u < alpha{
            p_t = p_prime;
            let p_t_length = total_length(&p_t, &distances);
            if p_t_length <= best_dist{
                best = p_t;
                best_dist = p_t_length;
            }
        }
        
    }
    best
}



fn tsp_distribution<const N:usize>(tour:&Permutation<N>, distances:&DistancesMat<N>,beta:f64)->f64{
    f64::exp(-beta * total_length(tour, distances))
}

fn permute<const N:usize>(p:Permutation<N>)->Permutation<N>{
    let mut p = p;
    let i = rand::random::<u32>() as usize %N;
    let mut j = rand::random::<u32>() as usize %N;
    while i == j {
        j = rand::random::<u32>() as usize %N;
    }

    p.swap(i, j);

    p
}