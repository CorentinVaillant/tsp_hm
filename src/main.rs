
use std::{io::{stdout, Write}, time::Instant};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::test::compare_methods;

mod reject_sampling_tsp;
mod support_hm;
mod support_math;
mod support_tsp;
mod test;

pub fn main(){
    const N:usize = 500;
    let begin = Instant::now();

    println!("-- Starting comparaison with {N} vertices --" );
    print!("[");
    let results:Vec<_> = (0..10).into_par_iter().map(|i|{
        let i = i +10;
        let iter = 2_u32.pow(i) as usize;
        let result = (compare_methods::<N>(iter, 100, 0.01),i);
        print!("{iter},");
        let _ = stdout().flush();
        result
    }).collect();
    println!("]");

    for (r,i) in results{
        println!("- [avg resuslt with 2^{i} iterations]  -> {r}");
    }

    let time = Instant::now().duration_since(begin).as_secs_f64();
    println!("\x07 => DONE took : {time}s")
}