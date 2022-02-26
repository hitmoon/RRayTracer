use std::ops::{Add,Sub,Mul,Div,Neg,Index};
use crate::util;

#[derive(Copy, Clone)]
pub struct Vec3 {

    pub e: [f64; 3],

}

pub type Point3 = Vec3;
pub type Color = Vec3;


impl Vec3 {

    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0]}
    }

    pub fn from(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0].clone()
    }

    pub fn y(&self) -> f64 {
        self.e[1].clone()
    }

    pub fn z(&self) -> f64 {
        self.e[2].clone()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    pub fn cross(&self, v: &Vec3) -> Self {
        Vec3 { e: [ self.e[1] * v.e[2] - self.e[2] * v.e[1],
                    self.e[2] * v.e[0] - self.e[0] * v.e[2],
                    self.e[0] * v.e[1] - self.e[1] * v.e[0] ] }
    }

    pub fn unit_vector(&self) -> Self {
        self.clone() / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3 { e: [util::random_double(), util::random_double(), util::random_double()] }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 { e: [util::random_double_range(min, max), util::random_double_range(min, max), util::random_double_range(min, max)] }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0  { // In the same hemisphere as the normal
            return in_unit_sphere;
        }

        -in_unit_sphere
    }

    pub fn near_zero(self) -> bool {
        // Return true if the vector is close to zero in all dimensions
        let s = 1e-8;
        return self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s;
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * v.dot(n) * 2.0
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -uv.dot(n).min(1.0);
        let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = *n * -f64::sqrt((1.0_f64 - r_out_perp.length_squared()).abs());
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 { e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]] }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3 { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]] }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
       Vec3 { e: [self.e[0] * t, self.e[1] * t, self.e[2] * t] }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, v: Vec3) -> Self {
        Vec3 { e: [self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]] }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        let m = 1 as f64 / t;
        Vec3 { e: [self.e[0] * m, self.e[1] * m, self.e[2] * m] }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

