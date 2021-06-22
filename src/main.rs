use std::time::Instant;
// (B/T) x (B-1/T-1) = 0.5

// b^2 - b = 1/2 * (t^2 - t)
// 2b^2 - 2b - n^2 + n = 0;

// ax^2 + bxy + cy^2 + dx + ey + f = 0
// 2b^2 + 0bn +-1n^2 + 2b +-1n + 0

fn main() {
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
