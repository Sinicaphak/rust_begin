mod common;

use once_cell::sync::OnceCell;
use RustRayTracing;
use RustRayTracing::common::snow_flake::snow_flake::Snowflake;
use RustRayTracing::pic::*;
use RustRayTracing::pic::ppm::FILE_TYPE_PPM;
use common::*;
use RustRayTracing::common::keep_two_decimal_places;
use RustRayTracing::entity::vec3::Color;


#[test]
#[ignore]
fn test_snow_flake(){
    let sf_id_worker = Snowflake::new(1, 1);
    let id = sf_id_worker.next_id();
    pretty_print(id);
}

#[test]
#[ignore]
fn test_gain_filename(){
    let c = crate_filename(FILE_TYPE_PPM);
    pretty_print(c);
}

#[test]
#[ignore]
fn test_keep_two_decimal_places(){
    assert_eq!(keep_two_decimal_places(123.45678), 123.45);
}

#[test]
fn test_for_rev(){
    for a in (0..3).rev() {
        common::pretty_print(a);
    }
}

static BLACK: OnceCell<Color> = OnceCell::new();

#[test]
fn test_once_cell(){
    assert!(BLACK.get().is_none());
    let s = BLACK.get_or_init(|| Color::black());
    pretty_print(s.x());
    let sd = BLACK.get();
    assert!(sd.is_some());
    pretty_print(sd.unwrap().x());
    assert_eq!(sd, BLACK.get());
}


