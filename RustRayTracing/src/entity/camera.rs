use crate::entity::ray::Ray;
use crate::entity::vec3::{div_num, dot_num, Point3, Vec3};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    vertical: Vec3,
    horizontal: Vec3
}

impl Camera {
    pub fn new() -> Self{
        let viewport_height: f64 = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length: f64 = 1.0;

        let origin = Point3::zero();
        let horizontal = Point3::new( viewport_width, 0.0, 0.0);
        let vertical = Point3::new( 0.0, viewport_height, 0.0);;
        let lower_left_corner = origin.clone() - div_num(2.0, &horizontal)
            - div_num(2.0, &vertical) - Point3::new(0.0, 0.0, focal_length);

        Self{
            origin,
            lower_left_corner,
            horizontal,
            vertical
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        let dir =
            self.lower_left_corner.clone() +
                dot_num(u, &self.horizontal) +
                dot_num(v, &self.vertical) -
                self.origin.clone();
        return Ray::new(self.origin.clone(), dir);
    }
}