use std::time::Instant;
use num::integer::{sqrt, Roots};
use num::traits::Pow;
use num::traits::real::Real;
// (B/T) x (B-1/T-1) = 0.5

// b^2 - b = 1/2 * (t^2 - t)
// 2b^2 - 2b - n^2 + n = 0;

// ax^2 + bxy + cy^2 + dx + ey + f = 0
// 2b^2 + 0bn +-1n^2 + 2b +-1n + 0

// This method I literally googled because I was so lost
// It solves the diophantine equation... eep!
fn main_searched_solution() {
    let mut now = Instant::now();
    let mut b: u64 = 15;
    let mut t: u64 = 21;
    let target: u64 = 1000000000000;

    while t < target {

        println!("{} / {} blues", b, t);

        let btemp = 3 * b + 2 * t - 2;
        let ttemp = 4 * b + 3 * t - 3;

        b = btemp;
        t = ttemp;
    }

    println!("{} blues discs in {} total discs", b, t);
    println!("Solved in {}μs", now.elapsed().as_micros())
}

// This is the approach I _should_ have been able to figure out
// main_old.rs is my very naive approach where I learnt about defining my own types
fn main() {
    let mut now = Instant::now();
    let mut b: u64 = 15;
    let mut t: u64 = 21;
    let target: u64 = 1000000000000;
    let mut b_old = 3;

    while t < target {

        // This should tell us each successive b number
        let b_new = b * 6 - (b_old + 2);
        b_old = b;
        b = b_new;
        let b_f = b as f64;

        // Knowing that 0.5 = (b/t)(b-1/(t-1)
        // t = 0.5(1 +- sqrt(8b^2 - b8 + 1))
        // Thanks Wolfram Alpha!
        let b_pow = b_f.pow(2);
        let root: f64 = (8.0 * b_pow - (b_f * 8.0) + 1.0);
        let t_f = 0.5 * (1.0 + root.sqrt());

        // We know that t has to be an integer because b is defined by
        t = t_f as u64;

        println!("{} blue in {} discs", b, t);

    }


    println!("{} blues discs in {} total discs", b, t);
    println!("Solved in {}μs", now.elapsed().as_micros())
}

