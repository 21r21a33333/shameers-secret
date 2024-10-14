use std::ops::BitOrAssign;

// shamers secreat sharing
use num_bigint::BigInt;

use rand::Rng;

pub mod models;
pub mod tests;

pub use models::fraction;
pub use models::Points;



// Function to perform the secret
// sharing algorithm and encode the
// given secret
fn secreate_sharing(S: BigInt, N: i128, K: i32, Points: &mut Vec<Points>) {
    // vector to store the polynomial of degree K-1
    let K = K;
    let mut poly: Vec<BigInt> = vec![BigInt::from(0); K as usize];

    // random number generator of all other polynomial but not for the secret i.e poly[0]
    poly[0] = BigInt::from(S);

    for i in 1..K {
        let mut p: BigInt = BigInt::from(0);
        while (p != BigInt::ZERO) {
            // assigning random number to the polynomial as a coefficient
            p = rand::thread_rng().gen_range(BigInt::from(1)..BigInt::from(1000));
        }
        poly[i as usize] = p;
    }

    // generating n points from the polynomial
    for j in 1..N {
        let x: BigInt = BigInt::from(j);
        let mut y: BigInt = BigInt::from(0);
        for i in 0..K {
            y = y + &poly[i as usize] * x.pow(i as u32);
        }
        Points.push(Points { x, y });
    }
}

// Function to generate the secret
// back from the given points
// This function will use Lagrange Basis Polynomial
// Instead of finding the complete Polynomial
// We only required the poly[0] as our secret code,
// thus we can get rid of x terms
fn generate_secret_key(x: Vec<BigInt>, y: Vec<BigInt>, M: i128) -> BigInt {
    let mut ans = fraction::new(BigInt::from(0), BigInt::from(1));

    // iterating through all the points
    for i in 0..M {
        let mut l = fraction::new(y[i as usize].clone(), BigInt::from(1));
        for j in 0..M {
            if i != j {
                let temp = fraction::new(-x[j as usize].clone(), BigInt::from(1));
                l = l.mul(&temp);
                let temp = fraction::new(x[i as usize].clone() - x[j as usize].clone(), BigInt::from(1));
                l = l.div(&temp);
            }
        }
        ans = ans.add(&l);
    }
    ans.num
}

// Function to encode and decode the
// given secret by using the above
// defined functions
pub fn operation(S: BigInt, N: i128, M: i128, K: i32) -> Option<BigInt> {
    let mut Points: Vec<Points> = Vec::new();
    secreate_sharing(S, N, K, &mut Points);
    println!("SECRET IS DIVIDED INTO {} PARTS", N);
    println!("NOW GENERATING THE SECRET KEY FROM K POINTS");
    if ((M as i32) < K) {
        println!("M should be greater than or equal to K");
        return None;
    }

    let mut x: Vec<BigInt> = vec![BigInt::from(0); M as usize];
    let mut y: Vec<BigInt> = vec![BigInt::from(0); M as usize];

    // use first K points to generate the secret key
    for i in 0..M {
        x[i as usize] = Points[i as usize].x.clone();
        y[i as usize] = Points[i as usize].y.clone();
    }
    let ret = generate_secret_key(x, y, M);
    println!("SECRET CODE IS {}", ret);
    Some(ret)
}

fn main() {
    let S: BigInt = BigInt::from(100);
    let N: i128 = (10);
    let K: i32 = 5;

    let M: i128 = (6);

    operation(S, N, M, K);
}



