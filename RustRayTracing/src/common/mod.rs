pub mod snow_flake;

use std::fmt::Display;
use math::round;
use rand::{Rng, thread_rng};

/// 保留小数点后两位
pub fn keep_two_decimal_places(num: f64) -> f64{
    round::floor(num, 2)
}

pub fn debug_print<T: Display>(to_print_1: T){
    println!("============== output ==============");
    println!("x: {to_print_1}");
    println!("============== output end ==============");
}
/// 在[0,1)之间获得随机数
pub fn gain_random_between_0_1() -> f64{
    let mut rng = thread_rng();
    return rng.gen_range(0.0..1.0);
}
/// 在[min,max)之间获得随机数
pub fn gain_random(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    return rng.gen_range(min..max);
}
/// 把x限制在min.max之间
pub fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min {
        return min;
    }

    if x > max {
        return max;
    }

    return x;
}