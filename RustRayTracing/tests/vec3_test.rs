mod common;

use common::*;
use RustRayTracing::entity::vec3::*;
use std::assert_eq;

#[test]
fn vec3_equal(){
    let (v1, v2) = get_vec();
    assert_eq!(v1 == v1, true);
}

#[test]
fn vec3_add(){
    let (v1, v2) = get_vec();
    let res_should = Vec3::new(3.0,6.0,9.0);
    assert_eq!(v1 + v2, res_should);
}
#[test]
fn vec3_add_by_ref(){
    let (v1, v2) = get_vec();
    let res_should = Vec3::new(3.0,6.0,9.0);
    assert_eq!(&v1 + &v2, res_should);
}
#[test]
fn vec3_sub(){
    let (v1, v2) = get_vec();
    let res_should = Vec3::new(-1.0,-2.0,-3.0);
    assert_eq!(v1 - v2, res_should);
}
#[test]
fn vec3_dot_mul(){
    let (v1, v2) = get_vec();
    assert_eq!(dot(&v1, &v2), 28_f64);
}

#[test]
fn vec3_num_mul(){
    let (v1, v2) = get_vec();
    assert_eq!(dot_num(2.0, &v1), v2);
}

#[test]
fn vec3_cross_mul(){
    let (v1, v2) = get_vec();
    let res_should = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(cross(&v1, &v2), res_should);
}

#[test]
fn vec3_div_num(){
    let (v1, v2) = get_vec();
    assert_eq!(div_num(2.0, &v2), v1);
}

fn get_vec() -> (Vec3, Vec3){
    (
        Vec3::new(1_f64,2_f64,3_f64),
        Vec3::new(2_f64,4_f64,6_f64),
    )
}