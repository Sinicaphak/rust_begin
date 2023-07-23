use std::sync::Arc;
use crate::entity::material::Material;
use crate::entity::*;
use crate::entity::vec3::*;
use crate::entity::hit_record::HitRecord;
use crate::entity::ray::*;

#[derive(Clone)]
pub struct Sphere{
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self{
            center,
            radius,
            material,
        }
    }


    pub fn center(&self) -> &Point3 {
        &self.center
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (a, half_b, _, delta) =
            calculate_delta(&self.center, ray, self.radius);

        if delta < 0.0 {
            return None;
        }

        let sqrt_delta = f64::sqrt(delta);
        let mut root = (-half_b - sqrt_delta) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_delta) / a;
        }

        if root < t_min || root > t_max {
            return None;
        }
        let intersection: Point3 = ray.at(root);
        let normal = div_num(self.radius, &( intersection.clone() - (self.center.clone()) ));
        let hit_record = HitRecord::new(
            intersection,
            normal,
            ray,
            root,
            self.material.clone()
        );

        return Some(hit_record);
    }
}