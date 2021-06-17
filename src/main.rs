use fraction::*;
type D = fraction::BigFraction;
type DEC = fraction::DynaDecimal<usize, u8>;
type F = fraction::Fraction;

// Find the number of blue discs in 10^12 discs needed such that the probability of drawing exactly two blue discs is 50%

// B + R = total_discs

// Chance of getting 1 B in 1 draw is B/total_discs

// (B/T) x (B-1/T-1) = 0.5

// (B/T) x (B-1/T-1) = V
// Find B

// c = B/T
// d = B-1/T-1
// 0.5/c = d

// The challenge is that we don't know what T is only that we want it to be as small as possible
// So then we could brute force it by starting B at a given minimum every time and increase it until the result is too high

// As we increase B, V will gradually get closer to 0.5
// As soon as it is above 0.5, we either need to reduce B or increase T
// If we've been increasing B, we bump T
// If V is still over 0.5 then we start reducing B until V is under 0.5

use std::time::Instant;

const MINIMUM_TOTAL: u64 = 1000000000001;

fn main() {

    let mut total_discs = DEC::from(MINIMUM_TOTAL);

    let target = DEC::from(1) / DEC::from(2);

    let ratio = DEC::from(15)  / DEC::from(21);

    let mut blue_discs = ratio.clone().checked_mul(&total_discs).unwrap().trunc();

    let mut value = DEC::from(0);

    let mut blue_discs_up = true;

    let now = Instant::now();

    loop {

        value = (blue_discs.clone() / total_discs.clone()) * ((blue_discs.clone() - DEC::from(1)) / (total_discs.clone() - DEC::from(1)));

        // Swing eliminates some early steps by swinging B more strongly the further away V is from 0.5
        // As it gets close swing is bounded to 1 as minimum as we can't add pieces of discs
        let mut swing = (value.clone() - target.clone()).abs().checked_mul(&total_discs).unwrap().trunc();
        if swing.clone() < DEC::from(1) {
            swing = DEC::from(1);
        }

        if value == target  {
            break;
        }

        println!("({} / {}) * ( {} / {}) = {:.20} :::::: swing = {}", blue_discs, total_discs, (blue_discs.clone() - DEC::from(1)), (total_discs.clone() - DEC::from(1)), value ,swing);

        if value.clone() > target.clone() {
            if blue_discs_up {
                total_discs += DEC::from(1);
                blue_discs += DEC::from(2);
                blue_discs_up = false;
            } else {
                blue_discs -= swing.clone();
            }
        } else if value.clone() < target.clone(){
            if blue_discs_up {
                blue_discs += swing.clone();
            } else {
                total_discs += DEC::from(1);
                blue_discs += DEC::from(2);
                blue_discs_up = true;
            }
        }


    }

    println!("0.5 Found!");
    println!("({} / {}) * ( {} / {}) = {:.10}", blue_discs.clone(), total_discs.clone(), (blue_discs.clone() - DEC::from(1)), (total_discs.clone() - DEC::from(1)), value);
    println!("Solved in {}ms", now.elapsed().as_millis());

}