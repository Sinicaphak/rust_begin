use super::vec3::*;

#[derive(Debug, Clone)]
pub struct Ray{
    // 光线发出点
    ori: Point3,
    // 光线发出方向
    dir: Direction
}

impl Ray {
    pub fn new(ori: Point3, dir: Direction) -> Self {
        Ray{
            ori,
            dir,
        }
    }
    // t时刻光线传播到哪个位置
    pub fn at(&self, t: f64) -> Point3{
        self.ori.clone()+ dot_num(t, &self.dir)
    }

    pub fn ori(&self) -> &Point3 {
        &self.ori
    }
    pub fn dir(&self) -> &Direction {
        &self.dir
    }
}