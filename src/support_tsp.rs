use crate::{support_hm::{DistancesMat, Permutation}, support_math::{ditsance_point, Point}};


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