
// Attribute to hide warnings for unused code and variables.
// #![allow(dead_code)]
// #![allow(unused_variables)]

use rayon;
use rayon::prelude::*;
use std::ops::Add;
use std::time::Instant;

fn main() {
    let mut pi = 0.0;
    let mut sum = 0.0;
    let num_steps = 100_000_000;
    let mut factor = 1.0;

    // Sequential loop

    let now = Instant::now();
    for i in 0..num_steps {
        sum = sum + factor / (2.0 * (i as f32) + 1.0);
        factor = -factor;
    }

    pi = 4.0 * sum;
    let end = now.elapsed();

    println!("Pi = {:?}, sequential loop took {:?} ms", pi, end.as_millis());
    
    // Sequential iterator

    let now = Instant::now();

    // TODO

    let end = now.elapsed();

    println!("Pi = {:?}, sequential iterator took {:?} ms", pi, end.as_millis());

    // Parallel iterator

    let now = Instant::now();

    // TODO


    let end = now.elapsed();

    println!("Pi = {:?}, parallel iterator took {:?} ms", pi, end.as_millis());
    
}
