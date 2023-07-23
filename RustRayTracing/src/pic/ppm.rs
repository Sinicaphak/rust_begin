use crate::pic::crate_path;

/// 生成一个ppm文件
///
/// ## ppm文件格式：
/// ``` ppm
/// P3
/// weight height
/// 255
/// 255 255 255
/// 0 0 0
/// ......
/// ```
///
pub const FILE_TYPE_PPM: &str = "ppm";
pub const MAX_RGB: f64 = 256.0;
const HEADER_BEGIN: &str = "P3\n";
const HEADER_END: &str = "\n255\n";

pub struct PPM {
    height: usize,
    weight: usize,
    path: String,
    value: Vec<Vec<(usize, usize, usize)>>
}

impl PPM {
    pub fn new(height: usize, weight: usize, vec: Vec<Vec<(usize, usize, usize)>>) -> PPM{
        Self{
            height,
            weight,
            value: vec,
            path: crate_path(FILE_TYPE_PPM),
        }
    }

    // 返回ppm文件头部
    pub fn gain_header(&self) -> String{
        return format!("{}{} {}{}", HEADER_BEGIN, self.weight, self.height, HEADER_END);
    }

    pub fn path(&self) -> &str {
        return &self.path;
    }

    pub fn value(&self) -> &Vec<Vec<(usize, usize, usize)>> {
        return &self.value;
    }
}



