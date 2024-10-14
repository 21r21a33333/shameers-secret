// shamers secreat sharing
use rand::Rng;

pub mod models;
pub use models::Points;
pub use models::fraction;

// Function to perform the secret
// sharing algorithm and encode the
// given secret
fn secreate_sharing(S: i128, N: i128, K: i128, Points: &mut Vec<Points>) {
    // vector to store the polynomial of degree K-1
    let mut poly: Vec<i128> = vec![0; K as usize];

    // random number generator of all other polynomial but not for the secret i.e poly[0]
    poly[0] = S;

    for i in 1..K {
        let mut p: i128 = 0;
        while (p != 0) {
            // assigning random number to the polynomial as a coefficient
            p = rand::thread_rng().gen_range(0..1000);
        }
        poly[i as usize] = p;
    }

    // generating n points from the polynomial
    for j in 1..N + 1 {
        let mut x: i128 = j;
        let mut y: i128 = 0;
        for i in 0..K {
            y = y.wrapping_add(poly[i as usize].wrapping_mul(x.pow(i as u32)));
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
fn generate_secret_key(x:Vec<i128> , y:Vec<i128>, M:i128)-> i128 {
    let mut ans=fraction::new(0,1);

    // iterating through all the points
    for i in 0..M{
        let mut l=fraction::new(y[i as usize],1);
        for j in 0..M{
            if i!=j{
                let temp=fraction::new(-x[j as usize],1);
                l=l.mul(&temp);
                let temp=fraction::new(x[i as usize]-x[j as usize],1);
                l=l.div(&temp);
            }
        }
        ans=ans.add(&l);
    }
    ans.num
}




// Function to encode and decode the
// given secret by using the above
// defined functions
fn operation(S: i128, N: i128,M:i128, K: i128) {
    let mut Points: Vec<Points> = Vec::new();
    secreate_sharing(S, N, K, &mut Points);
    println!("SECRET IS DIVIDED INTO {} PARTS", N);
    println!("NOW GENERATING THE SECRET KEY FROM K POINTS");
    if(M<K){
        println!("M should be greater than or equal to K");
        return;
    }

    let mut x: Vec<i128> = vec![0; M as usize];
    let mut y: Vec<i128> = vec![0; M as usize];


    // use first K points to generate the secret key
    for i in 0..M {
        x[i as usize] = Points[i as usize].x;
        y[i as usize] = Points[i as usize].y;
    }

    println!("SECRET CODE IS {}",generate_secret_key(x, y, M));

}

fn main() {
    let S: i128 = 100;
    let N: i128 = 10;
    let K: i128 = 5;

    let M=6;

    operation(S, N,M, K);
}
