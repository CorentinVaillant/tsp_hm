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