use std::sync::Arc;
use crate::entity::hit_record::HitRecord;
use crate::entity::Hittable;
use crate::entity::ray::Ray;

pub struct HittableList{
    hittables: Vec<Arc<dyn Hittable>>,
}

impl HittableList{

    pub fn new() -> Self{
        Self {
            hittables: Vec::new()
        }
    }

    pub fn clear(&mut self){
        self.hittables.clear();
    }

    pub fn add(&mut self, object:  Arc<dyn Hittable>){
        self.hittables.push(object);
    }

}

impl Hittable for HittableList {
    // 找出ray与Hittable_List中物体的哪个点相交
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.hittables.iter() {
            match object.hit(ray, t_min, t_max) {
                Some(hr) => {
                    closest_so_far = hr.t();
                    result = Some(hr);
                },
                None => continue
            }
        }

        return result;
    }
}