use core;

#[derive(Debug, Clone, Copy)]
pub struct Hz {
    num: u32,
    den: u32,
}

impl Hz {
    pub const fn from_num(num: u32) -> Hz {
        Hz::from_num_den((num, 1))
    }

    pub const fn from_den(den: u32) -> Hz {
        Hz::from_num_den((1, den))
    }

    pub const fn from_num_den(num_den: (u32, u32)) -> Hz {
        Hz { num: num_den.0, den: num_den.1 }
    }

    pub const fn num(&self) -> u32 {
        self.num
    }

    pub const fn den(&self) -> u32 {
        self.den
    }

    pub const fn num_den(&self) -> (u32, u32) {
        (self.num, self.den)
    }

    pub const fn normalized(&self) -> Hz {
        Hz::from_num(self.num / self.den)
    }

    pub const fn as_f32(&self) -> f32 {
        self.num as f32 / self.den as f32
    }

    pub const fn as_u32(&self) -> u32 {
        self.normalized().num()
    }

    pub fn checked_mul(&self, rhs: u32) -> Option<Hz> {
        self.num.checked_mul(rhs).map(|num| Hz::from_num_den((num, self.den)))
    }

    pub fn checked_div(&self, rhs: u32) -> Option<Hz> {
        self.den.checked_mul(rhs).map(|den| Hz::from_num_den((self.num, den)))
    }

    pub fn checked_normalized(&self) -> Option<Hz> {
        self.num.checked_div(self.den).map(Hz::from_num)
    }

    pub fn checked_as_u32(&self) -> Option<u32> {
        self.checked_normalized().map(|v| v.num())
    }    

    pub fn reduced(&self) -> Self {
        let gcd = self.gcd();
        let num = self.num / gcd;
        let den = self.den / gcd;
        Hz { num, den }
    }

    #[inline]
    fn gcd(&self) -> u32 {
        // Use Stein's algorithm
        let mut m = self.num;
        let mut n = self.den;
        if m == 0 || n == 0 { return m | n }

        // find common factors of 2
        let shift = (m | n).trailing_zeros();

        // The algorithm needs positive numbers, but the minimum value
        // can't be represented as a positive one.
        // It's also a power of two, so the gcd can be
        // calculated by bitshifting in that case

        // Assuming two's complement, the number created by the shift
        // is positive for all numbers except gcd = abs(min value)
        // The call to .abs() causes a panic in debug mode
        if m == u32::min_value() || n == u32::min_value() {
            return 1 << shift
        }

        // guaranteed to be positive now, rest like unsigned algorithm
        // divide n and m by 2 until odd
        // m inside loop
        n >>= n.trailing_zeros();

        while m != 0 {
            m >>= m.trailing_zeros();
            if n > m { ::core::mem::swap(&mut n, &mut m) }
            m -= n;
        }

        n << shift
    }
}

impl core::ops::Mul<u32> for Hz {
    type Output = Hz;
    fn mul(self, rhs: u32) -> Hz {
        Hz { num: self.num * rhs, den: self.den }
    }
}

impl core::ops::Div<u32> for Hz {
    type Output = Hz;
    fn div(self, rhs: u32) -> Hz {
        assert!(rhs != 0);
        Hz { num: self.num, den: self.den * rhs }
    }
}

impl From<u32> for Hz {
    fn from(other: u32) -> Hz {
        Hz::from_num(other)
    }
}

impl From<Hz> for u32 {
    fn from(other: Hz) -> u32 {
        other.as_u32()
    }
}
