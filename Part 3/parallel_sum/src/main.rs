
// Attribute to hide warnings for unused code and variables.
// #![allow(dead_code)]
// #![allow(unused_variables)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crossbeam::queue::SegQueue;


const SIZE: usize = 10_000_000;
const N_THREADS: usize = 10; // Assume that SIZE is multiple of N_THREADS

fn main() {
    let mut vector: Vec<i32> = Vec::new();

    for _ in 0..SIZE  {
        vector.push(1);
    }

    let now = Instant::now();
    let sum = sum_with_threads(&vector);
    let end = now.elapsed();
    println!("Sum: {}, threads took {:?} ms", sum, end.as_millis());
    
    let now = Instant::now();
    let sum = sum_with_crossbeam(&vector);
    let end = now.elapsed();
    println!("Sum: {}, crossbeam took {:?} ms", sum, end.as_millis());


    // For Part 4 of the tutorial

    let now = Instant::now();
    let sum = sum_with_threadpool(vector);
    let end = now.elapsed();
    println!("Sum: {}, threadpool took {:?} ms", sum, end.as_millis());
    

}

fn sum_with_threads(vector: &Vec<i32>) -> i32 {
    // TODO    
}


fn sum_with_crossbeam(vector: &Vec<i32>) -> i32 {
    // TODO    
}

fn sum_with_threadpool(vector: Vec<i32>) -> i32 {
    // TODO  Part 4 
}

