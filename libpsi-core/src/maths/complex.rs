use crate::Float;
use core::{fmt, ops};

#[macro_export]
macro_rules! complex {
    ($real:expr, $imaginary:expr) => {
        $crate::Complex::new($real, $imaginary)
    };
}

macro_rules! impl_ops {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<T: Float> ops::$trait for Complex<T> {
            type Output = Complex<T>;

            fn $method(self, other: Complex<T>) -> Complex<T> {
                Complex {
                    real: self.real $op other.real,
                    imaginary: self.imaginary $op other.imaginary,
                }
            }
        }
    };

    ($trait:ident, $method:ident, $op:tt, real) => {
        impl<T: Float> ops::$trait<T> for Complex<T> {
            type Output = Complex<T>;

            fn $method(self, other: T) -> Complex<T> {
                Complex {
                    real: self.real $op other,
                    imaginary: self.imaginary,
                }
            }
        }
    };

    ($trait_assign:ident, $method_assign:ident, $op:tt, assign) => {
        impl<T: Float> ops::$trait_assign for Complex<T> {
            fn $method_assign(&mut self, other: Complex<T>) {
                self.real = self.real $op other.real;
                self.imaginary = self.imaginary $op other.imaginary;
            }
        }
    };

    ($trait_assign:ident, $method_assign:ident, $op:tt, assign_real) => {
        impl<T: Float> ops::$trait_assign<T> for Complex<T> {
            fn $method_assign(&mut self, other: T) {
                self.real = self.real $op other;
            }
        }
    };
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Complex<T: Float> {
    pub real: T,
    pub imaginary: T,
}

impl<T: Float + fmt::Debug> fmt::Debug for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Complex {{ real: {:?}, imaginary: {:?} }}",
            self.real, self.imaginary
        )
    }
}

impl<T: Float + fmt::Display> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}

impl<T: Float> ops::Neg for Complex<T> {
    type Output = Complex<T>;

    fn neg(self) -> Complex<T> {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl<T: Float> From<T> for Complex<T> {
    fn from(real: T) -> Complex<T> {
        Complex {
            real,
            imaginary: T::zero(),
        }
    }
}

impl<T: Float> Complex<T> {
    pub fn new(real: T, imaginary: T) -> Complex<T> {
        Complex { real, imaginary }
    }

    pub fn get_conjugate(&self) -> Complex<T> {
        Complex {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    pub fn conjugate(&mut self) {
        self.imaginary = -self.imaginary;
    }

    pub fn phase(&self) -> T {
        T::atan2(self.imaginary, self.real)
    }

    pub fn norm2(&self) -> T {
        self.real * self.real + self.imaginary * self.imaginary
    }

    pub fn abs(&self) -> T {
        T::sqrt(self.norm2())
    }
}

impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);

impl<T: Float> ops::Mul for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, other: Complex<T>) -> Complex<T> {
        // (a + bi) * (c + di) = (ac - bd) + (ad + bc)i
        Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }
}

impl<T: Float> ops::Div for Complex<T> {
    type Output = Complex<T>;

    fn div(self, other: Complex<T>) -> Complex<T> {
        // (a + bi) / (c + di) = ((ac + bd) + (bc - ad)i) / (c² + d²)
        let denom = other.real * other.real + other.imaginary * other.imaginary;
        Complex {
            real: (self.real * other.real + self.imaginary * other.imaginary) / denom,
            imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denom,
        }
    }
}

impl_ops!(AddAssign, add_assign, +, assign);
impl_ops!(SubAssign, sub_assign, -, assign);

impl<T: Float> ops::MulAssign for Complex<T> {
    fn mul_assign(&mut self, other: Complex<T>) {
        let new_real = self.real * other.real - self.imaginary * other.imaginary;
        let new_imag = self.real * other.imaginary + self.imaginary * other.real;
        self.real = new_real;
        self.imaginary = new_imag;
    }
}

impl<T: Float> ops::DivAssign for Complex<T> {
    fn div_assign(&mut self, other: Complex<T>) {
        let denom = other.real * other.real + other.imaginary * other.imaginary;
        let new_real = (self.real * other.real + self.imaginary * other.imaginary) / denom;
        let new_imag = (self.imaginary * other.real - self.real * other.imaginary) / denom;
        self.real = new_real;
        self.imaginary = new_imag;
    }
}

impl_ops!(Add, add, +, real);
impl_ops!(Sub, sub, -, real);
impl_ops!(Mul, mul, *, real);
impl_ops!(Div, div, /, real);
