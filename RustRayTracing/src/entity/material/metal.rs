use crate::entity::hit_record::HitRecord;
use crate::entity::material::scattered_bean::ScatteredBean;
use crate::entity::ray::Ray;
use crate::entity::material::Material;
use crate::entity::vec3;
use crate::entity::vec3::Color;

pub struct Metal {
    // 反射颜色6
    albedo: Color,
}

impl Metal {

    pub fn new(albedo: Color) -> Self{
        Self {
            albedo,
        }
    }
}

impl Material for Metal {

    fn scatter(&self, ray: &Ray, hit_recode: &HitRecord) -> Option<ScatteredBean> {
        let reflected = vec3::reflect(&ray.dir().unit_vector(), hit_recode.normal());
        let scattered = Ray::new(ray.ori().clone(), reflected);
        let bean = ScatteredBean::new(self.albedo.clone(), scattered);
        return Some(bean);
    }

}