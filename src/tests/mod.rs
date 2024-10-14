

use crate::operation;
#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    use super::*;

    #[test]
    fn sampleTest() {
        let S: BigInt = BigInt::from(100);
        let N: i128 = 10;
        let K: i32 = 5;

        let M: i128 = 6;

        match operation(S, N, M, K) {
            Some(ret) => {
                assert_eq!(ret, BigInt::from(100));
            }
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn sampleTestForLargeSecret() {
        let S: BigInt = BigInt::from(1008734598645_i128);
        let N: i128 = 20;
        let K: i32 = 15;

        let M: i128 = 15;

        match operation(S, N, M, K) {
            Some(ret) => {
                assert_eq!(ret, BigInt::from(1008734598645_i128));
            }
            None => {
                assert!(false);
            }
        }
    }


    #[test]
    fn test_with_minimum_points() {
        let S: BigInt = BigInt::from(12345);
        let N: i128 = 5;
        let K: i32 = 3;

        let M: i128 = 3;

        match operation(S, N, M, K) {
            Some(ret) => {
                assert_eq!(ret, BigInt::from(12345));
            }
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_with_more_points() {
        let S: BigInt = BigInt::from(987654321);
        let N: i128 = 15;
        let K: i32 = 10;

        let M: i128 = 12;

        match operation(S, N, M, K) {
            Some(ret) => {
                assert_eq!(ret, BigInt::from(987654321));
            }
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_with_large_secret_and_more_points() {
        let S: BigInt = BigInt::from(987654321987654321_i128);
        let N: i128 = 25;
        let K: i32 = 20;

        let M: i128 = 22;

        match operation(S, N, M, K) {
            Some(ret) => {
                assert_eq!(ret, BigInt::from(987654321987654321_i128));
            }
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_with_insufficient_points() {
        let S: BigInt = BigInt::from(123456789);
        let N: i128 = 10;
        let K: i32 = 7;

        let M: i128 = 6;

        match operation(S, N, M, K) {
            Some(_) => {
                assert!(false, "Should not be able to reconstruct the secret with insufficient points");
            }
            None => {
                assert!(true);
            }
        }
    }
}
