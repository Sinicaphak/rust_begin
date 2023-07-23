pub mod default_material;
pub mod scattered_bean;
pub mod lambertian;

use std::sync::Arc;
use lazy_static::lazy_static;
use crate::entity::hit_record::HitRecord;
use crate::entity::material::scattered_bean::ScatteredBean;
use default_material::*;
use crate::entity::ray::Ray;
use crate::entity::vec3::Color;

lazy_static! {
    // 默认材质,暂时还没用
    pub static ref DEFAULT_MATERIAL: Arc<DefaultMaterial> = Arc::new(DefaultMaterial::new());
}

pub trait Material{
    fn scatter(&self, ray: &Ray, hit_recode: &HitRecord) -> Option<ScatteredBean>;
}

