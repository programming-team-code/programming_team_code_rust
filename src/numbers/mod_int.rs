//! # Modulo Integer

pub const MOD: u32 = 998_244_353;

pub struct Mint {
    pub val: u32,
}

impl Mint {
    pub fn new(mut val: i64) -> Self {
        //TODO does val need to be mut?
        let imod = MOD as i64;
        if !(-imod..imod).contains(&val) {
            val %= imod;
        }
        if val < 0 {
            val += imod;
        }
        Mint { val: val as u32 }
    }

    /*
    pub fn pow(self) -> Self {
    }

    pub fn inv(self) -> Self {
    }
    */
}

impl std::fmt::Display for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl std::ops::Add for Mint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let res = self.val + rhs.val;
        Mint {
            val: if res < MOD { res } else { res - MOD },
        }
    }
}

impl std::ops::Sub for Mint {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let res = self.val + MOD - rhs.val;
        Mint {
            val: if res < MOD { res } else { res - MOD },
        }
    }
}
