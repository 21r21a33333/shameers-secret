use num_bigint::BigInt;
#[derive(Debug, Clone)]
pub struct Points {
    pub x: BigInt,
    pub y: BigInt,
}
#[derive(Debug, Clone)]
/// fraction struct for performing operations
pub struct fraction {
    pub num: BigInt,
    pub den: BigInt,
}

impl fraction {
    pub fn new(num: BigInt, den: BigInt) -> fraction {
        fraction { num, den }
    }

    pub fn add(&self, other: &fraction) -> fraction {
        let temp = fraction {
            num: &self.num * &other.den + &other.num * &self.den,
            den: &self.den * &other.den,
        };
        temp.simplify()
    }

    pub fn sub(&self, other: &fraction) -> fraction {
        let temp = fraction {
            num: &self.num * &other.den - &other.num * &self.den,
            den: &self.den * &other.den,
        };
        temp.simplify()
    }

    pub fn mul(&self, other: &fraction) -> fraction {
        let temp = fraction {
            num: &self.num * &other.num,
            den: &self.den * &other.den,
        };
        temp.simplify()
    }

    pub fn div(&self, other: &fraction) -> fraction {
        let temp = fraction {
            num: &self.num * &other.den,
            den: &self.den * &other.num,
        };
        temp.simplify()
    }

    pub fn gcd(&self) -> BigInt {
        let mut a = self.num.clone();
        let mut b = self.den.clone();
        while b != BigInt::from(0) {
            let temp = b.clone();
            b = a % &b;
            a = temp;
        }
        a
    }

    pub fn simplify(&self) -> fraction {
        let gcd = self.gcd();
        fraction {
            num: &self.num / &gcd,
            den: &self.den / &gcd,
        }
    }
}
