mod pic;
mod common;
mod entity;

use std::time::Instant;
use test112::run;
use crate::pic::ppm::PPM;


fn main() {
    let now = Instant::now();  // 程序起始时间
    run();
    let end = now.elapsed().as_secs(); // 程序终止时间
    println!("程序运行了 {:?} 秒",end);
}
