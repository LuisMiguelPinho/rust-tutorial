// Attribute to hide warnings for unused code and variables.
// #![allow(dead_code)]
// #![allow(unused_variables)]

use rand::Rng;
use rayon;
use rayon::prelude::*;
use std::ops::Add;
use std::time::Instant;

fn main() {
    let vec_size = 1_000_000;
    let mut rng = rand::thread_rng();

    let vec : Vec<i32> = (0..=vec_size)
                        .map(|_| rng.gen_range(1..1_000))
                        .collect();

    let mut sum = 0;

    // Sequential std dev with loops

    let now = Instant::now();
    for elem in &vec {
        sum = sum + elem;
    }
    let avg = sum as f32 / vec_size as f32;

    let mut sum = 0.0;
    for &elem in &vec {
        sum = sum + (elem as f32 - avg).powf(2.0);
    }
    let std_dev = (sum / vec_size as f32).sqrt();
    let end = now.elapsed();

    println!("Std dev = {:?}, sequential loop took {:?} ms", std_dev, end.as_millis());
    
    // Sequential iterator

    let now = Instant::now();

    // TODO

    let end = now.elapsed();

    println!("Std dev = {:?}, sequential iterator took {:?} ms", std_dev, end.as_millis());

    // Parallel iterator with sum

    let now = Instant::now();

    // TODO

    let end = now.elapsed();

    println!("Std dev = {:?}, parallel iterator with sum took {:?} ms", std_dev, end.as_millis());

    // Parallel iterator with reduce

    let now = Instant::now();

    // TODO

    let end = now.elapsed();

    println!("Std dev = {:?}, parallel iterator with reduce took {:?} ms", std_dev, end.as_millis());

    // Parallel iterator with fold and sum

    let now = Instant::now();

    // TODO

    let end = now.elapsed();

    println!("Std dev = {:?}, parallel iterator wiht fold and sum took {:?} ms", std_dev, end.as_millis());
}
