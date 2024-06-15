//! # Modulo Integer

use crate::numbers::extended_gcd::ext_gcd;

pub const MOD: u64 = 998_244_353;

pub struct Mint {
    pub val: u64,
}

impl Mint {
    pub fn new(val: u64) -> Self {
        Self {
            val: if val < MOD {
                val
            } else if val < 2 * MOD {
                val - MOD
            } else {
                val % MOD
            },
        }
    }

    pub fn inv(self) -> Self {
        let (gcd, x, _) = ext_gcd(self.val as i64, MOD as i64);
        assert_eq!(gcd, 1);
        Mint::new(x as u64)
    }

    pub fn exp(self, mut e: u64) -> Self {
        let mut res = Mint::new(1);
        let mut b = Mint::new(self.val);
        while e > 0 {
            if e % 2 == 1 {
                res *= Mint::new(b.val);
            }
            b *= Mint::new(b.val);
            e /= 2;
        }
        res
    }
}

impl std::ops::Add for Mint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Mint::new(self.val + rhs.val)
    }
}

impl std::ops::AddAssign for Mint {
    fn add_assign(&mut self, rhs: Self) {
        self.val += rhs.val;
        if self.val >= MOD {
            self.val -= MOD;
        }
    }
}

impl std::ops::Sub for Mint {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Mint::new(self.val + MOD - rhs.val)
    }
}

impl std::ops::SubAssign for Mint {
    fn sub_assign(&mut self, rhs: Self) {
        self.val += MOD - rhs.val;
        if self.val >= MOD {
            self.val -= MOD;
        }
    }
}

impl std::ops::Mul for Mint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Mint::new(self.val * rhs.val)
    }
}

impl std::ops::MulAssign for Mint {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = self.val * rhs.val % MOD;
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Div for Mint {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Mint::new(self.val * rhs.inv().val)
    }
}

impl std::ops::DivAssign for Mint {
    fn div_assign(&mut self, rhs: Self) {
        self.val = self.val * rhs.inv().val % MOD;
    }
}
