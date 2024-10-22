use crate::operation;
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num_bigint::BigInt;

    use crate::generate_secret_key;

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
                assert!(
                    false,
                    "Should not be able to reconstruct the secret with insufficient points"
                );
            }
            None => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_generate_secret_key() {
        let x = vec![
            BigInt::from(64187),
            BigInt::from(86161),
            BigInt::from(71255),
            BigInt::from(33632),
            BigInt::from(85129),
            BigInt::from(37211),
            BigInt::from(61825),
            BigInt::from(33993),
            BigInt::from(43354),
            BigInt::from(65949),
        ];

        let y = vec![
            BigInt::from_str("83170346815119420729033463125487").unwrap(),
            BigInt::from_str("362481456080081416671699581241167").unwrap(),
            BigInt::from_str("140219838487651707565494163659775").unwrap(),
            BigInt::from_str("3284733188830334278541021603812").unwrap(),
            BigInt::from_str("341287068920226225205274744921615").unwrap(),
            BigInt::from_str("5446187954478934605461249241967").unwrap(),
            BigInt::from_str("68953115777245579114887248768975").unwrap(),
            BigInt::from_str("3464846665902497988392574069775").unwrap(),
            BigInt::from_str("11691785587361433033741245509040").unwrap(),
            BigInt::from_str("95230067684814801669123102472975").unwrap(),
        ];

        let secret = generate_secret_key(x, y,10);
        assert_eq!(secret, BigInt::from(100)); // Replace with the expected secret value
    }

}
