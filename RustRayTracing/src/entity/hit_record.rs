use std::sync::Arc;
use crate::entity::material::Material;
use crate::entity::ray::Ray;
use crate::entity::vec3::*;
use crate::entity::material::*;
use crate::entity::material::default_material::DefaultMaterial;

pub struct HitRecord {
    // 击中点
    point: Point3,
    // 法线
    normal: Vec3,
    // 击中时间
    t: f64,
    // 此时击中正面为true
    front_face: bool,
    // 击中材质
    material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(point: Point3, outward_normal: Vec3, ray: &Ray, t: f64, material: Arc<dyn Material>) -> Self {
        let front_face = dot(ray.dir(), &outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
            material: material.clone(),
        }
    }


    pub fn point(&self) -> &Point3 {
        &self.point
    }
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }

}