pub mod ppm;

use crate::common::snow_flake::snow_flake::Snowflake;
use std::fs::{File};
use std::io::Write;
use ppm::*;

const PATH_PREFIX_PPM: &str = "./pic/ppm/";
const PATH_PREFIX_UNKNOWN: &str = "./pic/";

pub fn crate_filename(file_type: &str) -> String {
    let sf_worker = Snowflake::new_default();
    return sf_worker.next_id().to_string()+"."+ file_type;
}

pub fn crate_path(file_type: &str) ->String {
    match file_type {
        FILE_TYPE_PPM => PATH_PREFIX_PPM.to_owned()+ &crate_filename(file_type),
        _ => PATH_PREFIX_UNKNOWN.to_owned()+ &crate_filename(file_type),
    }
}
// 把ppm持久化到文件中
pub fn ppm_persistent(ppm: PPM){
    let mut file = File::create(ppm.path())
        .unwrap();

    file.write(format!("{}", ppm.gain_header()).as_bytes()).expect("写入文件错误");

    for vec in ppm.value() {
        for (r, g, b) in vec {
            file.write(format!("{} {} {}  ", r, g, b).as_bytes()).expect("写入文件错误");
        }
        file.write("\n".as_bytes()).expect("TODO: panic message");
    }
    
    file.flush().expect("TODO: panic message");
}