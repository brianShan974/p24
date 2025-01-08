use std::num::NonZeroI32;

use crate::{Float, Int};

#[derive(Debug, Clone)]
pub struct Rational {
    numerator: Int,
    denominator: NonZeroI32,
}

impl Rational {
    pub fn new(numerator: Int, denominator: Int) -> Option<Self> {
        Some(Self {
            numerator,
            denominator: NonZeroI32::new(denominator)?,
        })
    }

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
                numerator: a + c,
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

    pub fn simplify(&self) -> Self {
        if self.is_zero() {
            self.clone()
        } else {
            let a = self.numerator;
            let b: Int = self.denominator.into();
            let factor = gcd(a.abs(), b.abs());

            let sign = (a * b).signum();

            Self {
                numerator: sign * a / factor,
                denominator: unsafe { NonZeroI32::new_unchecked(b / factor) },
            }
        }
    }

    pub fn evaluate_float(&self) -> Float {
        let a = self.numerator;
        let b: Int = self.denominator.into();

        a as Float / b as Float
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

fn gcd(a: Int, b: Int) -> Int {
    if a == 1 || b == 1 {
        1
    } else if a == 0 {
        b
    } else if b == 0 {
        a
    } else if a > b {
        gcd(b, a % b)
    } else {
        gcd(a, b % a)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(6, gcd(48, 18));
}
