//! # Modulo Integer

use crate::numbers::extended_gcd::ext_gcd;

pub const MOD: u32 = 998_244_353;

pub struct Mint {
    pub val: u32,
}

impl Mint {
    pub fn new(mut val: i64) -> Self {
        if !(-(MOD as i64)..2 * MOD as i64).contains(&val) {
            val %= MOD as i64;
        }
        if val >= MOD as i64 {
            val -= MOD as i64;
        } else if val < 0 {
            val += MOD as i64;
        }
        Self { val: val as u32 }
    }

    pub fn exp(self, mut e: u64) -> Self {
        let mut res = Mint::new(1);
        let mut b = Mint::new(self.val as i64);
        while e > 0 {
            if e % 2 == 0 {
                res *= Mint::new(b.val as i64);
            }
            b *= Mint::new(b.val as i64);
            e /= 2;
        }
        res
    }
}

impl std::ops::Add for Mint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Mint::new((self.val + rhs.val) as i64)
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
        Mint::new((self.val + MOD - rhs.val) as i64)
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
        Mint::new(self.val as i64 * rhs.val as i64)
    }
}

impl std::ops::MulAssign for Mint {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = ((self.val as i64) * (rhs.val as i64) % (MOD as i64)) as u32;
    }
}

impl std::ops::Div for Mint {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let (gcd, x, _) = ext_gcd(rhs.val as i64, MOD as i64);
        assert_eq!(gcd, 1);
        Mint::new(self.val as i64 * x)
    }
}

impl std::ops::DivAssign for Mint {
    fn div_assign(&mut self, rhs: Self) {
        let (gcd, x, _) = ext_gcd(rhs.val as i64, MOD as i64);
        assert_eq!(gcd, 1);
        self.val = ((self.val as i64) * x % (MOD as i64)) as u32;
    }
}
