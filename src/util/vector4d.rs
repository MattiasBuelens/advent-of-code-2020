use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Vector4D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[allow(dead_code)]
impl Vector4D {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Vector4D {
        Vector4D { x, y, z, w }
    }

    pub fn zero() -> Vector4D {
        Vector4D::new(0, 0, 0, 0)
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs() + self.w.abs()
    }
}

impl Default for Vector4D {
    fn default() -> Self {
        Vector4D::zero()
    }
}

impl Add for Vector4D {
    type Output = Self;

    fn add(self: Vector4D, other: Vector4D) -> Vector4D {
        Vector4D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Vector4D {
    type Output = Self;

    fn sub(self: Vector4D, other: Vector4D) -> Vector4D {
        Vector4D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Vector4D {
    type Output = Self;

    fn neg(self) -> Vector4D {
        Vector4D::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl AddAssign for Vector4D {
    fn add_assign(&mut self, other: Self) {
        self.x.add_assign(other.x);
        self.y.add_assign(other.y);
        self.z.add_assign(other.z);
        self.w.add_assign(other.w);
    }
}

impl SubAssign for Vector4D {
    fn sub_assign(&mut self, other: Self) {
        self.x.sub_assign(other.x);
        self.y.sub_assign(other.y);
        self.z.sub_assign(other.z);
        self.w.sub_assign(other.w);
    }
}
