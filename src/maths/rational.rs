use std::num::NonZeroI32;

use crate::Int;

#[derive(Debug, Clone)]
pub struct Rational {
    numerator: Int,
    denominator: NonZeroI32,
}

impl Rational {
    pub fn from_int(value: Int) -> Self {
        Self {
            numerator: value,
            denominator: unsafe { NonZeroI32::new_unchecked(1) },
        }
    }

    pub fn add(&self, other: &Rational) -> Self {
        let a = self.numerator;
        let b: Int = self.denominator.into();
        let c = other.numerator;
        let d: Int = other.denominator.into();

        if b == d {
            Self {
                numerator: a + c,
                denominator: unsafe { NonZeroI32::new_unchecked(b) },
            }
        } else {
            Self {
                numerator: a * d + c * b,
                denominator: unsafe { NonZeroI32::new_unchecked(b * d) },
            }
        }
    }

    pub fn sub(&self, other: &Rational) -> Self {
        let a = self.numerator;
        let b: Int = self.denominator.into();
        let c = other.numerator;
        let d: Int = other.denominator.into();

        if b == d {
            Self {
                numerator: a - c,
                denominator: unsafe { NonZeroI32::new_unchecked(b) },
            }
        } else {
            Self {
                numerator: a * d - c * b,
                denominator: unsafe { NonZeroI32::new_unchecked(b * d) },
            }
        }
    }

    pub fn mul(&self, other: &Rational) -> Self {
        let a = self.numerator;
        let b: Int = self.denominator.into();
        let c = other.numerator;
        let d: Int = other.denominator.into();

        Self {
            numerator: a * c,
            denominator: unsafe { NonZeroI32::new_unchecked(b * d) },
        }
    }

    pub fn div(&self, other: &Rational) -> Option<Self> {
        if other.is_zero() {
            return None;
        }

        Some(unsafe { self.div_unchecked(other) })
    }

    pub unsafe fn div_unchecked(&self, other: &Rational) -> Self {
        let a = self.numerator;
        let b: Int = self.denominator.into();
        let c = other.numerator;
        let d: Int = other.denominator.into();

        Self {
            numerator: a * d,
            denominator: unsafe { NonZeroI32::new_unchecked(b * c) },
        }
    }

    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    pub fn evaluate_int(&self) -> Option<Int> {
        let a = self.numerator;
        let b: Int = self.denominator.into();

        if a % b != 0 {
            None
        } else {
            Some(a / b)
        }
    }
}
