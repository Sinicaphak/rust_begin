extern crate core;

pub mod pic;
pub mod common;
pub mod entity;

use std::sync::Arc;
use lazy_static::lazy_static;
use pic::*;
use crate::pic::ppm::PPM;
use entity::vec3::*;
use crate::common::{debug_print, gain_random_between_0_1};
use crate::entity::*;
use crate::entity::camera::{ASPECT_RATIO, Camera};
use crate::entity::ray::*;
use crate::entity::hittable_list::*;
use crate::entity::material::DEFAULT_MATERIAL;
use crate::entity::sphere::Sphere;
use crate::entity::material::default_material::*;


// 光线最大反射次数
pub const MAX_REFLECT_LIMIT: usize = 100;
// 视屏参数
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize= ( (IMAGE_WIDTH as f64)  / ASPECT_RATIO ) as usize;
// 抗锯齿取样次数
const SAMPLES_PER_PIXEL: usize = 50;

pub fn run() {

    // 要渲染的物体
    let world = load_hittables();

    let camera = Camera::new();
    // 输出ppm
    let weight= IMAGE_WIDTH;
    let height= IMAGE_HEIGHT;
    let mut all_vec: Vec<Vec<(usize, usize, usize)>> = Vec::with_capacity(weight);

    for j in (0..=height - 1).rev() {
        let mut h_vec: Vec<(usize, usize, usize)> = Vec::with_capacity(height);
        for i in 0..weight {
            let mut color = Color::black();
            for _ in 0..SAMPLES_PER_PIXEL {
                // 计算每个像素的rgb
                let u: f64 = (i as f64 + gain_random_between_0_1())  / ( ( IMAGE_WIDTH - 1 ) as f64);
                let v: f64 = (j as f64 + gain_random_between_0_1())  / ( ( IMAGE_HEIGHT - 1 ) as f64);
                let ray = camera.get_ray(u, v);
                color += ray_color(ray, &world, MAX_REFLECT_LIMIT);
            }

            h_vec.push(
                write_color(color, SAMPLES_PER_PIXEL)
            );
        }
        all_vec.push(h_vec);
    }

    let ppm = PPM::new(height, weight, all_vec);

    // 持久化
    ppm_persistent(ppm);
}

/// 渲染物体,线性混合出背景色
fn ray_color<T: Hittable>(ray: Ray, world: &T, mut limit: usize) -> Color {
    // 光线反射次数用尽,返回空光
    if limit <= 0{
        return Color::black();
    }

    let bg_color_1: Color = Color::white();
    let bg_color_2: Color = Color::new(0.5, 0.7, 1.0);
    // 为什么t_min要设成0.0001???
    // 因为这样做会忽略方向很相同的反射光线,这会提高性能,并使得图更亮一点
    if let Some(hr) = world.hit(&ray, 0.0001, f64::INFINITY) {
        let target: Point3 = hr.point().clone() + hr.normal().clone() + Vec3::random_unit();
        limit -= 1;
        // 迭代出反射后的光线的路径
        let vec = ray_color(
            Ray::new(hr.point().clone(), target - hr.point().clone()),
            world,
            limit
        );
        return dot_num(0.5, &vec);
    }

    let dir_unit = ray.dir().unit_vector();
    let t = (dir_unit.y() + 1.0) * 0.5;
    return dot_num(1.0 - t, &bg_color_1) + dot_num(t, &bg_color_2);
}

/// 塞点物体进来渲染
fn load_hittables() -> HittableList {
    let mut world = HittableList::new();
    let sphere_one = Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        DEFAULT_MATERIAL.clone()
    );
    let sphere_two = Sphere::new(
        Point3::new(0.0, -100.0, -1.0),
        99.5,
        DEFAULT_MATERIAL.clone()
    );

    let arc_so = Arc::new(sphere_one);
    let arc_sw = Arc::new(sphere_two);

    world.add(arc_sw);
    world.add(arc_so);

    return world;
}

