pub mod vec3;
pub mod ray;
pub mod hit_record;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod material;

use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use crate::entity::ray::Ray;
use hit_record::*;
use vec3::*;



pub trait Hittable{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub fn calculate_delta(center: &Point3, ray: &Ray, radius: f64) -> (f64, f64, f64, f64){
    let oc = ray.ori().clone() - center.clone();
    let a = dot(ray.dir(), ray.dir());
    let half_b = dot(&oc, ray.dir());
    let c = dot(&oc, &oc) - radius*radius;
    let delta = half_b * half_b - (a * c);

    return (a, half_b, c, delta);
}

pub fn hit_sphere(center: &Point3, ray: &Ray, radius: f64) -> f64 {
    let (a, half_b, _, delta) =
        calculate_delta(center, ray, radius);

    return
        if delta < 0.0 {
            -1.0
        } else {
            (-half_b - f64::sqrt(delta)) / a
        }
}
