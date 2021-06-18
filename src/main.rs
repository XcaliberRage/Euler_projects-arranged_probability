#![feature(associated_type_bounds)]
use std::time::Instant;
use primes::{PrimeSet, factors};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Mul, Div, Sub};
use num::{Num, Integer, PrimInt, Bounded, NumCast, ToPrimitive};
use num::traits::real::Real;


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
    pub fn new<T: ToPrimitive, U: Copy + Clone + PrimInt + Num + Bounded + NumCast + PartialOrd>(num: T, den: U) -> Self {

        if den == U::from(0).unwrap() {
            panic!("Null value assigned to denominator")
        }

        let n = Fraction {
            num: <i128 as NumCast>::from(num).unwrap(),
            den: <u128 as NumCast>::from(den).unwrap()
        };

        n.simplify()
    }

    // Simplifies the given Fraction via common factor
    pub fn simplify(&self) -> Self {

        let gcd = gcd(self.num, self.den);
        if gcd == 0 {
            return Self { num: self.num, den: self.den };
        }

        Fraction {
            num: (self.num / gcd as i128),
            den: (self.den / gcd)
        }

    }

    pub fn from_whole<T: ToPrimitive>(whole: T) -> Self {

        Fraction {
            num: <i128 as NumCast>::from(whole).unwrap(),
            den: 1
        }

    }

    pub fn print_as_reg(&self) -> String {

        let num = self.num;
        let den = self.den as i128;

        if num < den {
            return format!("{}", self);
        }

        if num % den == 0 {
            return format!("{}", num / den);
        }

        format!("{} {}/{}", (num / den), num % den, den)
    }

    pub fn print_as_dec(&self) -> f64 {
        self.num as f64 / self.den as f64
    }

    pub fn floor(self) -> i128 {

        self.num / self.den as i128

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
            a_fract = self * rhs;
            b_fract = rhs * self;
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

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.num;
        let den = self.den * rhs.den;

        Self::new(num, den)
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {

        let reciprocal = Fraction::new(rhs.den, rhs.num);

        self * reciprocal

    }
}

impl Sub<Self> for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut a_fract = self;
        let mut b_fract = rhs;

        if self.den != rhs.den {
            a_fract = self * rhs;
            b_fract = rhs * self;
        }

        Fraction::new(a_fract.num - b_fract.num, a_fract.den)
    }
}

impl Sub<i128> for Fraction {
    type Output = Self;

    fn sub(self, rhs: i128) -> Self::Output {
        let rhs_fract = Fraction::from_whole(rhs);
        let mut a_fract = self;
        let mut b_fract = rhs_fract;

        if self.den != rhs_fract.den {
            a_fract = self * rhs_fract;
            b_fract = rhs_fract * self;

        }

        Fraction::new(a_fract.num - b_fract.num, a_fract.den)
    }
}

const MINIMUM_TOTAL: u64 = 1000000000001;

fn main() {

    let now = Instant::now();
    let target_ratio = Fraction::new(15, 21);
    let mut total_discs = Fraction::from_whole(MINIMUM_TOTAL);
    let mut blue_discs = Fraction::from_whole((target_ratio * total_discs).floor());
    let mut value = (blue_discs / total_discs) * ((blue_discs - 1) / (total_discs - 1));

    println!("0.5 Found!");
    println!("({} / {}) * ( {} / {}) = {:.10}", blue_discs.print_as_dec(), total_discs.print_as_dec(), (blue_discs - 1).print_as_dec(), (total_discs - 1).print_as_dec(), value.print_as_dec());
    println!("Solved in {}ms", now.elapsed().as_millis());

}