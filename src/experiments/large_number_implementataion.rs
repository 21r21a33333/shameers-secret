use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;
use std::ops::{Add, Mul, Rem, Sub};

// The 12th Mersenne Prime: 2^127 - 1
const PRIME_STR: &str = "170141183460469231731687303715884105727";
lazy_static::lazy_static! {
    static ref PRIME: BigInt = BigInt::parse_bytes(PRIME_STR.as_bytes(), 10).unwrap();
}

// Helper function to generate random integers in range [0, prime)
fn random_in_range(prime: &BigInt) -> BigInt {
    let mut rng = thread_rng();
    rng.gen_bigint_range(&BigInt::zero(), prime)
}

// Evaluate the polynomial at x using Horner's method
fn eval_at(poly: &[BigInt], x: &BigInt, prime: &BigInt) -> BigInt {
    let mut accum = BigInt::zero();
    for coeff in poly.iter().rev() {
        accum = (accum * x + coeff) % prime;
    }
    accum
}

// Generate random shares
fn make_random_shares(secret: BigInt, minimum: usize, shares: usize, prime: &BigInt) -> Vec<(BigInt, BigInt)> {
    if minimum > shares {
        panic!("Pool secret would be irrecoverable.");
    }

    // Generate the random polynomial
    let mut poly = vec![secret];
    for _ in 1..minimum {
        poly.push(random_in_range(prime));
    }

    // Generate the shares
    let mut points = Vec::new();
    for i in 1..=shares {
        let x = BigInt::from(i);
        let y = eval_at(&poly, &x, prime);
        points.push((x, y));
    }

    points
}

// Extended Euclidean algorithm to find the inverse
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        return (a.clone(), BigInt::one(), BigInt::zero());
    }

    let (gcd, x1, y1) = extended_gcd(b, &(a % b));
    let x = y1.clone();
    let y = x1 - (a / b) * y1;

    (gcd, x, y)
}

// Modular division: num / den mod p
fn mod_div(num: &BigInt, den: &BigInt, p: &BigInt) -> BigInt {
    let (_, inv, _) = extended_gcd(den, p);
    (num * inv).rem(p) + p
}

// Lagrange interpolation at x = 0
fn lagrange_interpolate(x: &BigInt, x_s: &[BigInt], y_s: &[BigInt], p: &BigInt) -> BigInt {
    let k = x_s.len();
    let mut secret = BigInt::zero();

    for i in 0..k {
        let mut num = BigInt::one();
        let mut den = BigInt::one();
        for j in 0..k {
            if i != j {
                num = num.mul(x - &x_s[j]).rem(p);
                den = den.mul(&x_s[i] - &x_s[j]).rem(p);
            }
        }
        let term = mod_div(&(num * &y_s[i] % p), &den, p);
        secret = (secret + term).rem(p);
    }

    (secret + p) % p
}

// Recover the secret from shares
fn recover_secret(shares: &[(BigInt, BigInt)], prime: &BigInt) -> BigInt {
    if shares.len() < 3 {
        panic!("Need at least three shares.");
    }

    let (x_s, y_s): (Vec<BigInt>, Vec<BigInt>) = shares.iter().cloned().unzip();
    lagrange_interpolate(&BigInt::zero(), &x_s, &y_s, prime)
}

fn main() {
    // Example secret
    let secret = BigInt::from(1234);

    // Generate 6 shares with a threshold of 3
    let shares = make_random_shares(secret.clone(), 3, 6, &PRIME);

    // Output shares
    println!("Secret: {}", secret);
    println!("Shares:");
    for (x, y) in &shares {
        println!("  (x: {}, y: {})", x, y);
    }

    // Recover the secret from a subset of shares (first 3 shares)
    let recovered_secret = recover_secret(&shares[0..3], &PRIME);
    println!("Recovered Secret from 3 shares: {}", recovered_secret);
}
