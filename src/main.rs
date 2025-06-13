use crate::{reject_sampling_tsp::reject_sample_tsp, support_hm::hasting_met_tsp, support_math::Point, support_tsp::{distance_mat, total_length}};

mod reject_sampling_tsp;
mod support_hm;
mod support_math;
mod support_tsp;
mod test;

pub fn main(){
    let towns:[Point;10] = core::array::from_fn(|_|(rand::random_range(-100.0..100.0),rand::random_range(-100.0..100.0)));
    let distances = distance_mat(&towns);


    let sol = hasting_met_tsp(distances, 1., 10_000);

    println!("best solution hm : {:?}, |sol| = {}",sol, total_length(&sol, &distances));

    let sol = reject_sample_tsp(distances, 10_000);
    println!("best solution rs : {:?}, |sol| = {}",sol, total_length(&sol, &distances));
}