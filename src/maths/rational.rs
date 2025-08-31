use std::ops::{Add, Div, Mul, Sub};

use crate::{Int, NonZeroInt};

#[derive(Debug, Clone)]
pub struct Rational {
    numerator: Int,
    denominator: NonZeroInt,
}

impl Rational {
    pub fn from_int(value: Int) -> Self {
        Self {
            numerator: value,
            denominator: unsafe { NonZeroInt::new_unchecked(1) },
        }
    }

    /// # Safety
    /// `other` must be non-zero.
    pub unsafe fn div_unchecked(self, other: Rational) -> Self {
        let a = self.numerator;
        let b: Int = self.denominator.into();
        let c = other.numerator;
        let d: Int = other.denominator.into();

        Self {
            numerator: a * d,
            denominator: unsafe { NonZeroInt::new_unchecked(b * c) },
        }
    }

    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    pub fn evaluate_int(&self) -> Option<Int> {
        let a = self.numerator;
        let b: Int = self.denominator.into();

        if a % b != 0 { None } else { Some(a / b) }
    }
}

impl Add<Rational> for Rational {
    type Output = Self;

    fn add(self, other: Rational) -> Self {
        let a = self.numerator;
        let b: Int = self.denominator.into();
        let c = other.numerator;
        let d: Int = other.denominator.into();

        if b == d {
            Self {
                numerator: a + c,
                denominator: unsafe { NonZeroInt::new_unchecked(b) },
            }
        } else {
            Self {
                numerator: a * d + c * b,
                denominator: unsafe { NonZeroInt::new_unchecked(b * d) },
            }
        }
    }
}

impl Sub<Rational> for Rational {
    type Output = Self;

    fn sub(self, other: Rational) -> Self {
        let a = self.numerator;
        let b: Int = self.denominator.into();
        let c = other.numerator;
        let d: Int = other.denominator.into();

        if b == d {
            Self {
                numerator: a - c,
                denominator: unsafe { NonZeroInt::new_unchecked(b) },
            }
        } else {
            Self {
                numerator: a * d - c * b,
                denominator: unsafe { NonZeroInt::new_unchecked(b * d) },
            }
        }
    }
}

impl Mul<Rational> for Rational {
    type Output = Self;

    fn mul(self, other: Rational) -> Self {
        let a = self.numerator;
        let b: Int = self.denominator.into();
        let c = other.numerator;
        let d: Int = other.denominator.into();

        Self {
            numerator: a * c,
            denominator: unsafe { NonZeroInt::new_unchecked(b * d) },
        }
    }
}

impl Div<Rational> for Rational {
    type Output = Option<Self>;

    fn div(self, other: Rational) -> Option<Self> {
        if other.is_zero() {
            return None;
        }

        Some(unsafe { self.div_unchecked(other) })
    }
}
