use crate::entity::ray::Ray;
use crate::entity::vec3::Color;

pub struct ScatteredBean {
    attenuation: Color,
    scattered: Ray,
}

impl ScatteredBean {
    pub fn new(attenuation: Color, scattered: Ray) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }


    pub fn attenuation(&self) -> &Color {
        &self.attenuation
    }
    pub fn scattered(&self) -> &Ray {
        &self.scattered
    }
}
