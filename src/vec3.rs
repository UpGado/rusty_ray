use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub fn clamp(num: f64, lo: f64, hi: f64) -> f64 {
    if num > hi {
        hi
    } else if num < lo {
        lo
    } else {
        num
    }
}

impl Vec3 {
    pub fn zeros() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Vec3 {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random();
            if v.length() <= 1.0 {
                return v;
            }
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn color_string(&self) -> String {
        let c = |x| (clamp(x, 0.0, 1.0) * 255.0) as u8;
        format!("{} {} {}", c(self.0), c(self.1), c(self.2))
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Self) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        let mut u = *self;
        u /= u.length();
        u
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3(vec.0 * self, vec.1 * self, vec.2 * self)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, vec: Vec3) -> Vec3 {
        (1.0 / self) * vec
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, n: f64) {
        self.0 = self.0 * n;
        self.1 = self.1 * n;
        self.2 = self.2 * n;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, n: f64) {
        *self *= 1.0 / n;
    }
}
