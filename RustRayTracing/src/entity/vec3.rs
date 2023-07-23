use std::ops::{Add, AddAssign, Mul, Neg, Sub};
use std::process::Output;
use crate::common::{clamp, gain_random, gain_random_between_0_1, keep_two_decimal_places};
use crate::pic::ppm::MAX_RGB;
/// vec的三个参数都小于这个值,则vec会被认为等于原点
const NEAR_ZERO: f64 = 1.0e-8;

/// 用向量表示颜色
pub type Color = Vec3;
/// 用向量表示点的位置
pub type Point3 = Vec3;
/// 用向量表示方向
pub type Direction = Vec3;

/// 向量类

#[derive(Debug, Clone)]
pub struct Vec3{
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    /// 原点
    pub fn zero() -> Self{
        Self{
           x: 0.00,
           y: 0.00,
           z: 0.00,
        }
    }
    /// 在xy面上的向量
    pub fn xy(x: f64, y: f64) -> Self{
        Self{
            x,
            y,
            z: 0.00,
        }
    }
    /// 取随机单位向量
    pub fn random_unit() -> Self{
        loop {
            let vector = Vec3::random_between_0_1();
            if vector.length() < 1.0 {
                return vector.unit_vector();
            }
        }
    }
    /// 在xyz在0到1之间的向量中取随机向量
    pub fn random_between_0_1() -> Self{
        Self{
            x: gain_random_between_0_1(),
            y: gain_random_between_0_1(),
            z: gain_random_between_0_1(),
        }
    }
    /// 取在范围内取随机向量
    pub fn random(min: f64, max: f64) -> Self{
        Self{
            x: gain_random(min, max),
            y: gain_random(min, max),
            z: gain_random(min, max),
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Self{
            x, y, z,
        }
    }
    /// 模长平方
    pub fn length_square(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    /// 模长
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_square())
    }
    /// 单位向量
    pub fn unit_vector(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    /// 转为rgb值
    fn to_rgb(&self) -> (u64, u64, u64) {
        let x = (self.x * MAX_RGB) as u64;
        let y = (self.y * MAX_RGB) as u64;
        let z = (self.z * MAX_RGB) as u64;
        (x, y, z)
    }


    pub fn x(&self) -> &f64 {
        &self.x
    }
    pub fn y(&self) -> &f64 {
        &self.y
    }
    pub fn z(&self) -> &f64 {
        &self.z
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }
}

impl Color {
    pub fn white() -> Color {
        Color{
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn black() -> Color {
        Color{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// 加+
impl Add for Vec3{
    type Output = Self;

    fn add(self, rhs: Self) -> Self{
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// 加+
impl Add for &Vec3{
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output{
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
/// 自加+
impl AddAssign for Vec3{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
/// 减-
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
/// 取反
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
/// 等于==
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}
/// 点乘
/// 调用dot()方法, 不区分左右乘
pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z)
}
/// 叉乘
/// 调用cross()方法, 区分左乘 右乘
pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3{
    Vec3{
        x: (lhs.y * rhs.z) - (lhs.z * rhs.y),
        y: (lhs.x * rhs.z) - (lhs.z * rhs.x),
        z: (lhs.x * rhs.y) - (lhs.y * rhs.x),
    }
}
/// 数乘
/// 调用dot_num()方法, 不区分左右乘
pub fn dot_num(num: f64, vec: &Vec3) -> Vec3{
    return Vec3{
        x: vec.x * num,
        y: vec.y * num,
        z: vec.z * num,
    }
}
/// 向量除以一个数
/// 调用div_num()方法, 不区分左右乘
pub fn div_num(num: f64, vec: &Vec3) -> Vec3{
    return Vec3{
        x: vec.x / num,
        y: vec.y / num,
        z: vec.z / num,
    }
}
/// vec的三个参数的绝对值都大于qwe,则认为不等于原点,返回true
pub fn check_near_zero(vec: &Vec3) -> bool{
    if *vec.x() <= NEAR_ZERO || *vec.y() <= NEAR_ZERO || *vec.z() <= NEAR_ZERO {
        return false;
    }
    return true;
}

pub fn write_color(color: Color, samples_per_pixel: usize) -> (usize, usize, usize){
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let samples_per_pixel = samples_per_pixel as f64;

    (calculate_rgb(*r, samples_per_pixel), calculate_rgb(*g, samples_per_pixel), calculate_rgb(*b, samples_per_pixel))
}

fn calculate_rgb(mut rgb: f64, samples_per_pixel: f64) -> usize {
    rgb /= samples_per_pixel;
    rgb = f64::sqrt(rgb);
    (clamp(rgb, 0.0, 0.99999999) * MAX_RGB) as usize
}



