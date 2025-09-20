use rand::Rng;
use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    let arr: Vec<i32> = (1..=1_000_000_000)
        .map(|_| rand::rng().random_range(1..=1000000))
        .collect();
    println!("rand ints generated in: {:?}", now.elapsed().unwrap());
    let now = SystemTime::now();
    let s = arr.into_iter().sum::<i32>();
    println!("sum calculated in: {:?}", now.elapsed().unwrap());
    println!("sum: {:?}", s);

    println!("-----------------------------------");
    println!("i64");
    let now = SystemTime::now();
    let arr: Vec<i64> = (1..=1_000_000_000)
        .map(|_| rand::rng().random_range(1..=1000000))
        .collect();
    println!("rand ints generated in: {:?}", now.elapsed().unwrap());
    let now = SystemTime::now();
    let s = arr.into_iter().sum::<i64>();
    println!("sum calculated in: {:?}", now.elapsed().unwrap());
    println!("sum: {:?}", s)
}
