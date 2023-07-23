use crate::entity::vec3::check_near_zero;
use crate::entity::vec3::Vec3;
use crate::entity::hit_record::HitRecord;
use crate::entity::material::Material;
use crate::entity::material::scattered_bean::ScatteredBean;
use crate::entity::ray::Ray;
use crate::entity::vec3::Color;

pub struct Lambertian{
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self{
        Self{
            albedo,
        }
    }
}

impl Material for Lambertian{
    fn scatter(&self, ray: &Ray, hit_recode: &HitRecord) -> Option<ScatteredBean> {
        let normal = hit_recode.normal().clone();
        let mut dir = &normal + &Vec3::random_unit();
        if check_near_zero(&dir){
            dir = normal;
        }
        let scatterred_ray = Ray::new(hit_recode.point().clone(), dir);
        let bean = ScatteredBean::new(self.albedo.clone(), scatterred_ray);
        return Some(bean);
    }
}