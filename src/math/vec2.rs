use nalgebra::{RealField, Scalar};
use alga::general::{Additive, Identity};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
pub struct Vec2<T: Scalar> {
    pub x: T,
    pub y: T
}

impl<T: Scalar> Vec2<T> {
    pub fn new() -> Vec2<T> where T: Identity<Additive> {
        Vec2 {
            x: nalgebra::zero(),
            y: nalgebra::zero()
        }
    }

    pub fn from_values(x: T, y: T) -> Vec2<T> {
        Vec2 {
            x,
            y
        }
    }

    pub fn len(&self) -> T where T: RealField {
        (self.x*self.x + self.y*self.y).sqrt()
    }
}

// Begin mathematical operators -----------------------------------------------

impl<T: Scalar + Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec2::from_values(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Scalar + Add<Output = T>> Add<T> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        Vec2::from_values(self.x + rhs, self.y + rhs)
    }
}

impl<T: Scalar + AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Scalar + Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec2::from_values(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Scalar + Sub<Output = T>> Sub<T> for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        Vec2::from_values(self.x - rhs, self.y - rhs)
    }
}

impl<T: Scalar + SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// Scalar multiplication
impl<T: Scalar + Add<Output = T> + Mul<Output = T>> Mul for Vec2<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Scalar + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vec2::from_values(self.x * rhs, self.y * rhs)
    }
}

impl<T: Scalar + Div<Output = T>> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Vec2::from_values(self.x / rhs, self.y / rhs)
    }
}

// End of mathematical operators ----------------------------------------------