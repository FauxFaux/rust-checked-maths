use checked_sum::CheckedSum;
use rand::Rng;
use rayon::prelude::*;
use std::time::SystemTime;

fn main() {
    println!("-----------------------------------");
    println!("Unchecked maths performance test");
    println!("-----------------------------------");
    println!("i32");
    let now = SystemTime::now();
    let arr_i32: Vec<i32> = (1..=1_000_000_000)
        .into_par_iter()
        .map(|_| rand::rng().random_range(1..=1000000))
        .collect();
    println!("rand ints generated in: {:?}", now.elapsed().unwrap());
    let now = SystemTime::now();
    let s = arr_i32.clone().into_iter().sum::<i32>();
    println!("sum calculated in: {:?}", now.elapsed().unwrap());
    println!("sum: {:?}", s);

    println!("-----------------------------------");
    println!("i64");
    let now = SystemTime::now();
    let arr_i64: Vec<i64> = (1..=1_000_000_000)
        .into_par_iter()
        .map(|_| rand::rng().random_range(1..=1000000))
        .collect();
    println!("rand ints generated in: {:?}", now.elapsed().unwrap());
    let now = SystemTime::now();
    let s = arr_i64.clone().into_iter().sum::<i64>();
    println!("sum calculated in: {:?}", now.elapsed().unwrap());
    println!("sum: {:?}", s);

    println!("\n\n-----------------------------------");
    println!("CHECKED maths performance test");
    println!("-----------------------------------");
    println!("i32");
    let now = SystemTime::now();
    let s = arr_i32.into_iter().map(|x| x as i64).checked_sum();
    println!("sum calculated in: {:?}", now.elapsed().unwrap());
    println!("sum: {:?}", s);
    println!("sum (i32): {:?}", s.and_then(|s| i32::try_from(s).ok()));

    println!("-----------------------------------");
    println!("i64");
    let now = SystemTime::now();
    let s = arr_i64.into_iter().checked_sum();
    println!("sum calculated in: {:?}", now.elapsed().unwrap());
    println!("sum: {:?}", s);
}
