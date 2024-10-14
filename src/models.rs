#[derive(Debug, Clone)]
pub struct Points {
    pub x: i128,
    pub y: i128,
}

#[derive(Debug, Clone)]
///fraction ds for performing the operations
pub struct fraction {
    pub num: i128,
    pub den: i128,
}
impl fraction {
    pub fn new(num: i128, den: i128) -> fraction {
        fraction { num, den }
    }
    pub fn add(&self, other: &fraction) -> fraction {
        let mut temp=fraction {
            num: self.num * other.den + other.num * self.den,
            den: self.den * other.den,
        };
        temp.simplify()
    }
    pub fn sub(&self, other: &fraction) -> fraction {
        let temp=fraction {
            num: self.num * other.den - other.num * self.den,
            den: self.den * other.den,
        };
        temp.simplify()
    }
    pub fn mul(&self, other: &fraction) -> fraction {
        let temp=fraction {
            num: self.num * other.num,
            den: self.den * other.den,
        };
        temp.simplify()
    }
    pub fn div(&self, other: &fraction) -> fraction {
        let temp=fraction {
            num: self.num * other.den,
            den: self.den * other.num,
        };
        temp.simplify()
    }
    pub fn gcd(&self) -> i128 {
        let mut a = self.num;
        let mut b = self.den;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    pub fn simplify(&self) -> fraction {
        let gcd = self.gcd();
        fraction {
            num: self.num / gcd,
            den: self.den / gcd,
        }
    }
}
