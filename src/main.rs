use std::{error::Error, io::{stdout, BufRead, Write}, str::FromStr, time::Instant};

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{reject_sampling_tsp::uniform_sample_tsp, support_hm::sample_hasting_met_tsp, support_math::Point, support_tsp::{best_solution_vec, distance_mat}, test::{compare_beta_hm_tsp, compare_methods}};

mod reject_sampling_tsp;
mod support_hm;
mod support_math;
mod support_tsp;
mod test;

const SIZE:usize = 100;

pub fn main()->Result<(),String>{
    
    let msg = r#"
Choose an option :
    [1] => Compare the results between HM and RS
    [2] => Generate comparaison into a CSV 
    [3] => Compare Beta for HM into a CSV 
[1\2\3]: "#;
    
    let option = input_options(msg, &[1,2,3])?;
    let begin = Instant::now();
    match option{
        1 => compare_results(),
        2 => generate_csv_comparaison().map_err(|e|e.to_string())?,
        3 => generate_csv_beta_comparaison().map_err(|e|e.to_string())?,
        _ => unreachable!()
    };

    let time = Instant::now().duration_since(begin).as_secs_f64();
    println!("\x07 => DONE took : {time}s");
    Ok(())

}

fn generate_csv_beta_comparaison()->Result<(),Box<dyn Error>>{
    let part = 1_000;
    let b_range = 0.01..1.0;
    let betas:Vec<_> = (0..part).map(|i|b_range.start + i as f64*(b_range.end-b_range.start)/(part as f64) ).collect();
    let iterations = [10, 50,100,500,1_000];
    let results = compare_beta_hm_tsp::<SIZE>(&iterations, betas.as_slice());
    
    let mut wtr = csv::Writer::from_path("./beta_comparaison.csv").map_err(Box::new)?;
    wtr.write_field("beta")?;
    let res_strs = betas.iter().map(|v| v.to_string());
    wtr.write_record(res_strs)?;

    for (result,iter) in results.iter().zip(iterations){
        let field = format!("nombre d'itération : {iter}");
        wtr.write_field(field)?;
        let res_strs = result.iter().map(|v| v.to_string());
        wtr.write_record(res_strs)?;
    }

    wtr.flush()?;

    Ok(())
}

fn generate_csv_comparaison()->Result<(),Box<dyn Error>>{
    let towns:[Point;SIZE] = core::array::from_fn(|_|(rand::random_range(-100.0..100.0),rand::random_range(-100.0..100.0)));
    let distances = distance_mat(&towns);


    let part = 4;
    let range = 0.01..2.0;
    let betas:Vec<_> = (0..part).map(|i|range.start + i as f64*(range.end-range.start)/(part as f64) ).collect();

    let results:Vec<_> = betas.par_iter().map(|beta|{
        let sample = sample_hasting_met_tsp(distances, *beta, 1_000);
        best_solution_vec(sample.as_slice(), &distances)

    }).collect();

    let mut wtr = csv::Writer::from_path("./methods_comparaison.csv").map_err(Box::new)?;

    for (res,b) in results.iter().zip(betas){
        let field = format!("HM : beta = {b}");
        wtr.write_field(field)?;
        let res_strs = res.iter().map(|v| v.to_string());
        wtr.write_record(res_strs)?;
    }

    let sample = uniform_sample_tsp::<SIZE>(1_000);
    let result = best_solution_vec(sample.as_slice(), &distances);
    let field = format!("RS");
    wtr.write_field(field)?;
    let res_strs = result.iter().map(|v| v.to_string());
    wtr.write_record(res_strs)?;

    wtr.flush()?;


    Ok(())


}

fn compare_results(){
    println!("-- Starting comparaison with {SIZE} vertices --" );
    print!("[");
    let results:Vec<_> = (0..10).into_par_iter().map(|i|{
        let i = i +10;
        let iter = 2_u32.pow(i) as usize;
        let result = (compare_methods::<SIZE>(iter, 100, 0.01),i);
        print!("{iter},");
        let _ = stdout().flush();
        result
    }).collect();
    println!("]");

    for (r,i) in results{
        println!("- [avg resuslt with 2^{i} iterations]  -> {r}");
    }
}

fn input_options<T:FromStr + PartialEq>(msg:&str, options:&[T])->Result<T,String>{
    loop {
        print!("{msg}");
        stdout().flush().map_err(|e|e.to_string())?;
    
        let input = read_input()?;
        if let Ok(input) = input.parse::<T>(){
            if options.contains(&input){
                return Ok(input);
            }
        }
        println!("\"{input}\" is not a valid input, please enter a valid one.");
        
    }
}

fn read_input()->Result<String,String>{
    std::io::stdin().lock()
        .lines()
        .next().ok_or(String::from("Please give an argument"))?
        .map_err(|e|e.to_string())
}