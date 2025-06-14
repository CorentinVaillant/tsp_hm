use crate::support_math::{ditsance_point, Point};

pub type Permutation<const N:usize> = [usize;N];
pub type DistancesMat<const N:usize> = [[f64;N];N];

pub fn distance_mat<const N:usize>(coords:&[Point;N])->DistancesMat<N>{
    core::array::from_fn(|i|core::array::from_fn(|j|{
        if i == j{
            0.
        }else{
            ditsance_point(coords[i],coords[j])
        }
    }))
}

pub fn total_length<const N:usize>(tour:&Permutation<N>,distances:&DistancesMat<N>)->f64{
    (0..tour.len()).map(|i|distances[tour[i]][tour[(i+1)%N]]).sum()
}

pub fn best_solution_vec<const N:usize>(sample: &[Permutation<N>], distances:&DistancesMat<N>)->Vec<f64>{
    let mut best = Vec::with_capacity(sample.len());
        let mut min = total_length(&sample[0], distances);
        for i in 0..sample.len(){
            let length_i = total_length(&sample[i],&distances);
            if length_i <= min{
                min = length_i;
            }
            best.push(min);
        }
        best
}