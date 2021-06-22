#![feature(associated_type_bounds)]
use std::time::Instant;
use primes::{PrimeSet, factors};
use std::fmt;
use std::fmt::{Formatter, Debug};
use std::ops::{Add, Mul, Div, Sub, AddAssign, SubAssign};
use num::{Num, Integer, PrimInt, Bounded, NumCast, ToPrimitive};
use num::traits::real::Real;
use std::cmp::Ordering;
use num::integer::lcm;
use std::fs::File;
use std::io::Write;


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
#[derive(PartialEq, Copy, Clone, Eq, Debug)]
struct Fraction {
    num: i128,
    den: u128
}

fn gcd<T: Copy + Clone + PrimInt + Num + Bounded + NumCast + PartialOrd + Debug, U: Copy + Clone + PrimInt + Num + Bounded + NumCast + PartialOrd + Debug>(x: T, y: U) -> T {

    let mut x = x;
    let mut y = T::from(y).unwrap();
    while y != T::from(0).unwrap() {
        let t = y;
        y = x % y.clone();
        x = t;
    }
    x
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&&other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {

        let lcm = lcm(self.den, other.den);

        let me = Fraction::new_no_simp(self.num * (lcm / self.den) as i128, lcm);
        let them = Fraction::new_no_simp(other.num * (lcm / other.den ) as i128, lcm);

        me.num.cmp(&them.num)

    }
}

impl Fraction {
    pub fn new<T: ToPrimitive + Debug + Copy, U: Copy + Clone + PrimInt + Num + Bounded + NumCast + PartialOrd + Debug>(num: T, den: U) -> Self {

        if den == U::from(0).unwrap() {
            panic!("Null value assigned to denominator")
        }

        let n = Fraction {
            num: <i128 as NumCast>::from(num).unwrap(),
            den: <u128 as NumCast>::from(den).unwrap()
        };

        n.simplify()
    }

    pub fn new_no_simp<T: ToPrimitive, U: Copy + Clone + PrimInt + Num + Bounded + NumCast + PartialOrd>(num: T, den: U) -> Self {
        if den == U::from(0).unwrap() {
            panic!("Null value assigned to denominator")
        }

        Fraction {
            num: <i128 as NumCast>::from(num).unwrap(),
            den: <u128 as NumCast>::from(den).unwrap()
        }
    }

    // Simplifies the given Fraction via common factor
    pub fn simplify(&self) -> Self {

        let gcd = gcd(self.num, self.den).abs();
        if gcd == 0 {
            return Self { num: self.num, den: self.den };
        }

        Fraction {
            num: (self.num / gcd),
            den: (self.den as i128 / gcd).abs() as u128
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

    pub fn frac_floor(self) -> i128 {

        self.num / (self.den as i128)

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
            a_fract = Fraction::new_no_simp(self.num * rhs.den as i128, self.den * rhs.den);
            b_fract = Fraction::new_no_simp(rhs.num * self.den as i128, rhs.den * self.den);
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

impl AddAssign<Self> for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl AddAssign<i128> for Fraction {
    fn add_assign(&mut self, rhs: i128) {
        *self = *self + rhs
    }
}

impl SubAssign<Self> for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl SubAssign<i128> for Fraction {
    fn sub_assign(&mut self, rhs: i128) {
        *self = *self - rhs
    }
}

impl Mul<Self> for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {

        let num = self.num * rhs.num;
        let den = self.den * rhs.den;

        Self::new(num, den)
    }
}

impl Mul<i128> for Fraction {
    type Output = Self;

    fn mul(self, rhs: i128) -> Self::Output {
        let num = self.num * rhs;
        let den = self.den * rhs as u128;

        Self::new(num, den)
    }
}

impl Mul<u128> for Fraction {
    type Output = Self;

    fn mul(self, rhs: u128) -> Self::Output {
        let num = self.num * rhs as i128;
        let den = self.den * rhs;

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
            a_fract = Fraction::new_no_simp(self.num * rhs.den as i128, self.den * rhs.den);
            b_fract = Fraction::new_no_simp(rhs.num * self.den as i128, rhs.den * self.den);
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
            a_fract = Fraction::new(self.num * rhs_fract.den as i128, self.den * rhs_fract.den);
            b_fract = Fraction::new(rhs_fract.num * self.den as i128, rhs_fract.den * self.den);

        }

        Fraction::new(a_fract.num - b_fract.num, a_fract.den)
    }
}

const MINIMUM_TOTAL: u64 = 1000000000001;

fn main() -> std::io::Result<()> {

    let now = Instant::now();
    let initial_ratio = Fraction::new(15, 21);
    let target = Fraction::new(1, 2);
    let mut total_discs = Fraction::from_whole(MINIMUM_TOTAL);
    let mut blue_discs = Fraction::from_whole((initial_ratio * total_discs).frac_floor());
    let mut value = Fraction::from_whole(0);
    let mut blue_disc_incr = true;

    let mut loop_ct = 1;

    loop {

        value = (blue_discs / total_discs) * ((blue_discs - 1) / (total_discs - 1));

        let swing = Fraction::from_whole(((value.print_as_dec() - target.print_as_dec()) * total_discs.print_as_dec()).abs().floor().max(1.0));

        if value == target {
            break;
        }

        //println!("{}: ({} / {}) * ( {} / {}) = [{}] {} :::::::: swing: {}", loop_ct, blue_discs.print_as_dec(), total_discs.print_as_dec(), (blue_discs - 1).print_as_dec(), (total_discs - 1).print_as_dec(), value, value.print_as_dec(), swing.print_as_dec());
        println!("{}", loop_ct);

        if value < target {
            if blue_disc_incr {
                blue_discs += swing;
            } else {
                total_discs += 1;
                blue_discs += 2;
                blue_disc_incr = true;
            }
        }

        if value > target {
            if blue_disc_incr {
                blue_disc_incr = false;
                total_discs += 1;
                blue_discs += 1;
            } else {
                blue_discs -= swing;
            }
        }

        loop_ct += 1;

    }

    println!("0.5 Found!");
    println!("({} / {}) * ( {} / {}) = [{}] {}", blue_discs.print_as_dec(), total_discs.print_as_dec(), (blue_discs - 1).print_as_dec(), (total_discs - 1).print_as_dec(), value, value.print_as_dec());
    println!("Solved in {}ms, {} iterations", now.elapsed().as_millis(), loop_ct);

    Ok(())

}