use std::sync::{Arc, Mutex};

use inline_colorization::{color_green, color_red, color_reset, color_yellow};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{reject_sampling_tsp::reject_sample_tsp, support_hm::hasting_met_tsp, support_math::Point, support_tsp::{distance_mat, total_length}};

#[derive(Debug,Clone, Copy)]
pub enum Method{
    HastingMetropolis,
    RejectSampling,
    Both
}

#[derive(Debug,Clone, Copy)]
pub struct CompareResult{
    pub best_method : Method,
    pub hm_dist :f64,
    pub rs_dist :f64,
}

//-- Two methods comparaison --

pub fn compare_methods_once<const SIZE:usize>(iteration_per_func:usize,beta_hm:f64)->CompareResult{
    let towns:[Point;SIZE] = core::array::from_fn(|_|(rand::random_range(-100.0..100.0),rand::random_range(-100.0..100.0)));
    let distances = distance_mat(&towns);


    let sol_hm = hasting_met_tsp(distances, beta_hm, iteration_per_func);
    let sol_rs = reject_sample_tsp(distances, iteration_per_func);

    let hm_dist = total_length(&sol_hm, &distances);
    let rs_dist = total_length(&sol_rs, &distances);

    let best_method = if hm_dist < rs_dist{
        Method::HastingMetropolis
    }else if hm_dist > rs_dist{
        Method::RejectSampling
    }else {Method::Both};

    CompareResult { best_method, hm_dist, rs_dist}
}

#[allow(unused)]
#[derive(Debug,Clone, Copy)]
pub struct AvgCompareResult{
    pub best_method : Method,
    pub avg_dist_between_methods : f64
}

pub fn compare_methods<const N:usize>(iteration_per_func:usize,iteration:usize,beta_hm:f64)->AvgCompareResult{
    let rs_best = Arc::new(Mutex::new(0_usize));
    let avg_dist_between_methods = Arc::new(Mutex::new(0.));
    let n = iteration as f64;


    (0..iteration).into_par_iter().for_each(|_|{
        let rs_best = rs_best.clone();
        let avg_dist_between_methods = avg_dist_between_methods.clone();
    
        let result = compare_methods_once::<N>(iteration_per_func, beta_hm);
        match result.best_method {
            Method::RejectSampling => *rs_best.lock().expect("something went wrong, lock failed") += 1,
            _=> ()
        }

        let dist = (result.hm_dist - result.rs_dist).abs()/n;
        *avg_dist_between_methods.lock().expect("something went wrong, lock failed") += dist;
    });

    let rs_best = *rs_best.lock().unwrap();
    let avg_dist_between_methods = *avg_dist_between_methods.lock().unwrap();

    let best_method = if rs_best > iteration /2{
        Method::RejectSampling
    }else{
        Method::HastingMetropolis
    };

    AvgCompareResult{
        best_method,
        avg_dist_between_methods,
    }
}

impl std::fmt::Display for AvgCompareResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"AvgCompareResult {{")?;
        let color = match self.best_method {
            Method::HastingMetropolis => color_green,
            Method::RejectSampling => color_red,
            Method::Both => color_yellow,
        };
        writeln!(f,"\t best :{color} {:?} {color_reset}",self.best_method)?;

        writeln!(f,"\t average distance : {}",self.avg_dist_between_methods)?;
        writeln!(f,"}}")
    }
}

//-- beta comparaison --

pub type BetaComparaisonResult = f64;


pub fn compare_beta_hm_tsp<const SIZE: usize>(iterations:&[usize], betas:&[f64]) -> Vec<Vec<BetaComparaisonResult>>{
    let towns:[Point;SIZE] = core::array::from_fn(|_|(rand::random_range(-100.0..100.0),rand::random_range(-100.0..100.0)));
    let distances = distance_mat(&towns);

    let results = Arc::new(Mutex::new(Vec::with_capacity(iterations.len())));

    iterations.par_iter().for_each(|i|{

        let mut result : Vec<_> = betas.par_iter().map(|b|{

        let result  = hasting_met_tsp(distances, *b, *i);
        
        total_length(&result,&distances) 
        
    }).collect();
    result.sort_by(|r1,r2|r1.partial_cmp(r2).unwrap_or(std::cmp::Ordering::Equal));
    results.clone().lock().unwrap().push(result);
    });

    let results = Arc::try_unwrap(results)
    .expect("Arc has multiple strong references")
    .into_inner()
    .expect("Mutex cannot be locked");
    
    results 


}