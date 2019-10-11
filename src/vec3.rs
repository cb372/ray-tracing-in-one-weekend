use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign };

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {

    pub fn x(self) -> f64 { self.0 }
    pub fn y(self) -> f64 { self.1 }
    pub fn z(self) -> f64 { self.2 }

    pub fn r(self) -> f64 { self.0 }
    pub fn g(self) -> f64 { self.1 }
    pub fn b(self) -> f64 { self.2 }

    pub fn negate(self) -> Self {
        Self(-(self.0), -(self.1), -(self.2))
    }

    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn make_unit_vector(&mut self) {
        // TODO how do we call self.length() without a copy happening?
        let k = 1.0 / self.length();
        *self *= k;
    }

    pub fn dot(v1: Self, v2: Self) -> f64 {
        v1.0*v2.0 + v1.1*v2.1 + v1.2*v2.2
    }

    pub fn cross(v1: Self, v2: Self) -> Self {
        Self(
            v1.1 * v2.2 - v1.2 * v2.1,
            v1.2 * v2.0 - v1.0 * v2.2,
            v1.0 * v2.1 - v1.1 * v2.0
        )
    }

}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self(
            self.0 * other,
            self.1 * other,
            self.2 * other
        )
    }
}

impl MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2
        )
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self(
            self.0 * other,
            self.1 * other,
            self.2 * other
        )
    }
}

impl Div<Self> for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self(
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2
        )
    }
}

impl DivAssign<Self> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self(
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self(
            self.0 / other,
            self.1 / other,
            self.2 / other
        )
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self(
            self.0 / other,
            self.1 / other,
            self.2 / other
        )
    }
}
