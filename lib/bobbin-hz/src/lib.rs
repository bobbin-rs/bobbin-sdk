//! *bobbin-hz* provides a simple integer fraction type useful for representing
//! frequencies used in clock calculations. A set of math operations is provided 
//! that avoids divides where possible, minimizing rounding errors.
//! 
//! The representation is simple: a numerator and denominator, both `u32`. Care should
//! be taken to avoid overflowing either the numerator or denominator during a sequence of
//! operations; if this is likely, use either `reduce()` (which finds and divides by the GCD)
//! or `normalize()` (which performs a full division of the numerator by the denominator).
//! 
//! This crate is designed for `#![no_std]` operation.

#![no_std]
#![feature(const_fn)]

/// A fractional representation of a frequency.
#[derive(Debug, Clone, Copy)]
pub struct Hz {
    num: u32,
    den: u32,
}

impl Hz {
    /// Returns the zero value.
    pub const fn zero() -> Hz {
        Hz::from_num(0)
    }

    /// Returns a value `num / 1`.
    pub const fn from_num(num: u32) -> Hz {
        Hz::from_num_den((num, 1))
    }

    /// Returns a value `1 / den`.
    /// 
    /// Note: this may be removed from the api to avoid
    /// divide by zero checks.
    pub const fn from_den(den: u32) -> Hz {
        Hz::from_num_den((1, den))
    }

    /// Returns the fraction `num / den`.
    /// 
    /// Note: this may be removed from the api to avoid
    /// divide by zero checks.
    pub const fn from_num_den(num_den: (u32, u32)) -> Hz {
        Hz { num: num_den.0, den: num_den.1 }
    }

    /// Returns the numerator of the fraction.
    pub const fn num(&self) -> u32 {
        self.num
    }

    /// Returns the denominator of the fraction.
    pub const fn den(&self) -> u32 {
        self.den
    }

    /// Returns the numerator and denominator as (num, den).
    pub const fn num_den(&self) -> (u32, u32) {
        (self.num, self.den)
    }

    /// Returns the value normalized so that the denominator
    /// is 1.
    pub const fn normalized(&self) -> Hz {
        Hz::from_num(self.num / self.den)
    }

    /// Returns the value as a f32.
    pub const fn as_f32(&self) -> f32 {
        self.num as f32 / self.den as f32
    }

    /// Returns the value as a normalized u32.
    pub const fn as_u32(&self) -> u32 {
        self.normalized().num()
    }

    /// Performs a checked multiplication by `rhs`, returning None
    /// if the value would overflow.
    pub fn checked_mul(&self, rhs: u32) -> Option<Hz> {
        self.num.checked_mul(rhs).map(|num| Hz::from_num_den((num, self.den)))
    }

    /// Performs a checked division by `rhs`, returning None
    /// if the value would overflow.
    pub fn checked_div(&self, rhs: u32) -> Option<Hz> {
        self.den.checked_mul(rhs).map(|den| Hz::from_num_den((self.num, den)))
    }

    /// Performs a checked normalization, returning None
    /// if the value would overflow.
    pub fn checked_normalized(&self) -> Option<Hz> {
        self.num.checked_div(self.den).map(Hz::from_num)
    }

    /// Performs a checked normalization and conversion to u32, returning None
    /// if the value would overflow
    pub fn checked_as_u32(&self) -> Option<u32> {
        self.checked_normalized().map(|v| v.num())
    }    

    /// Returns the value with the numerator and denominator each
    /// divided by the greatest common denominator.
    pub fn reduced(&self) -> Self {
        let gcd = self.gcd();
        let num = self.num / gcd;
        let den = self.den / gcd;
        Hz { num, den }
    }

    /// Returns the value with the numerator and denominator switched.
    pub fn invert(&self) -> Self {
        Hz { num: self.den, den: self.num }
    }

    /// Finds the greatest common denominator using Stein's algorithm.
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
    type Output = Self;
    fn mul(self, rhs: u32) -> Hz {
        Hz { num: self.num * rhs, den: self.den }
    }
}

impl core::ops::Div<u32> for Hz {
    type Output = Self;
    fn div(self, rhs: u32) -> Hz {
        assert!(rhs != 0);
        Hz { num: self.num, den: self.den * rhs }
    }
}

impl core::ops::Shr<u32> for Hz {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        // Shift denominator left to avoid truncation
        Hz { num: self.num, den: self.den << rhs }
    }
}

impl core::ops::Shl<u32> for Hz {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        Hz { num: self.num << rhs, den: self.den }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm() {
        let hz = Hz::from(1);     
        assert_eq!(hz.as_u32(), 1u32);
        let hz_2 = hz * 2;
        assert_eq!(hz_2.as_u32(), 2u32);
        let hz_3 = hz_2 / 2;
        assert_eq!(hz_3.as_u32(), 1u32);
    }

    #[test]
    fn test_reduce() {
        let hz = Hz::from(1);
        let hz = hz * 2 / 2;
        assert_eq!(hz.num(), 2);
        assert_eq!(hz.den(), 2);
        let hz = hz.reduced();
        assert_eq!(hz.num(), 1);
        assert_eq!(hz.den(), 1);
    }

    #[test]
    fn test_shl() {
        let hz = Hz::from(1);
        let hz = hz << 2;
        assert_eq!(hz.num(), 4);
        assert_eq!(hz.den(), 1);
    }    

    #[test]
    fn test_shr() {
        let hz = Hz::from(8);
        let hz = hz >> 2;
        assert_eq!(hz.num(), 8);
        assert_eq!(hz.den(), 4);
    }
    #[test]
    fn test_pclk() {
        let pclk1 = Hz::from_num(54000000);
        let tim_pclk1 = pclk1 << 1;
        assert_eq!(tim_pclk1.as_u32(), 54000000 << 1);
    }
}