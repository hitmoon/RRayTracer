use std::ops::{Add,Sub,Mul,Div,Neg,Index};

#[derive(Copy)]
pub struct Vec3 {

    pub e: [f64; 3],

}

pub type Point3 = Vec3;
pub type Color = Vec3;


impl Vec3 {

    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0]}
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

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3 { e : [self.e[0], self.e[1], self.e[2]] }
    }
}
