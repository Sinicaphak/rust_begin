use crate::entity::material::Material;
use crate::entity::material::scattered_bean::ScatteredBean;
use crate::entity::hit_record::HitRecord;
use crate::entity::ray::Ray;
use crate::entity::vec3;
use crate::entity::vec3::{Color, Vec3};

pub struct DefaultMaterial {
    // 反射颜色
    albedo: Color,
}

impl DefaultMaterial {

    pub fn new() -> Self {
        Self {
            albedo: Color::black(),
        }
    }

}


impl Material for DefaultMaterial {

    fn scatter(&self, ray: &Ray, hit_recode: &HitRecord) -> Option<ScatteredBean> {
        // 默认散射黑色的光
        let default_color = self.albedo.clone();

        let dir = hit_recode.normal().clone() + Vec3::random_unit();
        // 默认散射方向为法线附近的随机方向
        let default_ray: Ray = Ray::new( ray.ori().clone(), dir);

        let result = ScatteredBean::new(default_color, default_ray);
        return Some(result);
    }

}