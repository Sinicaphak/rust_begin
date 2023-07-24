pub mod default_material;
pub mod scattered_bean;
pub mod lambertian;
pub mod metal;

use std::sync::Arc;
use crate::entity::hit_record::HitRecord;
use crate::entity::material::scattered_bean::ScatteredBean;
use default_material::*;
use crate::entity::ray::Ray;
use crate::entity::vec3::Color;

pub trait Material{
    fn scatter(&self, ray: &Ray, hit_recode: &HitRecord) -> Option<ScatteredBean>;
}

