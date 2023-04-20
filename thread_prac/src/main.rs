use std::time;
use rayon::prelude::*;

pub fn fibonacci_recursive(n:i32) -> u64{
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!(
            "zero is not a right argument to fibonacci_recursive()!"
        ),
        1 | 2 => 1,
        _ => fibonacci_recursive(n-1) + fibonacci_recursive(n-2)
    }
}

fn main() {
    let now = time::Instant::now();
    fibonacci_recursive(8);
    fibonacci_recursive(12);
    fibonacci_recursive(12);
    fibonacci_recursive(20);
    fibonacci_recursive(20);
    fibonacci_recursive(20);
    fibonacci_recursive(20);
    fibonacci_recursive(28);
    fibonacci_recursive(28);
    fibonacci_recursive(28);
    fibonacci_recursive(28);
    fibonacci_recursive(36);
    fibonacci_recursive(46);
    fibonacci_recursive(46);
    fibonacci_recursive(46);
    println!("1: time elapsed {:?}", now.elapsed());

    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let now = time::Instant::now();
    let numbers: Vec<i32> = vec![8, 12, 12, 20, 20, 20, 20, 28, 28, 28, 28, 36, 46, 46, 46];
    let outcomes: Vec<u64> = numbers.into_par_iter().map(|n| fibonacci_recursive(n)).collect();
    println!("2: time elapsed {:?}", now.elapsed());
    println!("{:?}", outcomes);
    
}   
