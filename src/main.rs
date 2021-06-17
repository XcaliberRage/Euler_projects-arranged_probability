#![feature(associated_type_bounds)]
use std::time::Instant;
use primes::{PrimeSet, factors};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Mul};
use num::{Num, Integer, PrimInt, Bounded, NumCast};


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
#[derive(PartialEq, Copy, Clone)]
struct Fraction {
    num: i128,
    den: u128
}

fn gcd<T: Copy + Clone + PrimInt + Num + Bounded + NumCast + PartialOrd, U: Copy + Clone + PrimInt + Num + Bounded + NumCast + PartialOrd>(x: T, y: U) -> U {
    let mut x = U::from(x).unwrap();
    let mut y = y;
    while y != U::from(0).unwrap() {
        let t = y;
        y = x % y.clone();
        x = t;
    }
    x
}

impl Fraction {
    pub fn new<T, U>(num: T, den: U) -> Self {

        let n = Fraction {
            num: <i128 as Trait>::From::from(num),
            den: <u128>::From::from(den)
        };

        n.simplify()
    }

    // Simplifies the given Fraction via common factor
    pub fn simplify(&self) -> Self {

        let gcd = gcd(self.num, self.den);

        Fraction {
            num: (self.num / gcd),
            den: (self.den / gcd)
        }

    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl Add<Self> for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {

        let mut a_fract = self;
        let mut b_fract = rhs;

        if self.den != rhs.den {
            a_fract = self * rhs.den;
            b_fract = rhs * self.den;
        }

        Fraction::new(a_fract.num + b_fract.num, a_fract.den)

    }
}

impl Add<i128> for Fraction {
    type Output = Self;

    fn add(self, rhs: i128) -> Self::Output {

        let rhs_as_fract = Fraction::new(rhs, 1);

        self + rhs_as_fract

    }
}

impl Mul<Self> for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.num;
        let den = self.den * self.den;

        Self::new(num, den)
    }
}

impl Mul<i128> for Fraction {
    type Output = Self;

    fn mul(self, rhs: i128) -> Self::Output {
        let num = self.num * rhs;
        let den = self.den * rhs.clone();

        Self::new(num, den)
    }
}


const MINIMUM_TOTAL: u64 = 1000000000001;

fn main() {

    let target_ratio = Fraction::new(15, 21);

    for i in 0..20 {

        let mul_fract = target_ratio * i;
        println!("{} * {} = {}", target_ratio, i, mul_fract);

        let add_fract = mul_fract + target_ratio;
        println!("{} + {} = {}", mul_fract, target_ratio, add_fract);

        let mul_fract_b = add_fract * mul_fract;
        println!("{} * {} = {}", add_fract, mul_fract, mul_fract_b);

    }


    /*println!("0.5 Found!");
    println!("({} / {}) * ( {} / {}) = {:.10}", blue_discs.clone(), total_discs.clone(), (blue_discs.clone() - DEC::from(1)), (total_discs.clone() - DEC::from(1)), value);
    println!("Solved in {}ms", now.elapsed().as_millis());*/

}