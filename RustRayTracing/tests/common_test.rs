mod common;

use test112;
use test112::common::snow_flake::snow_flake::Snowflake;
use test112::pic::*;
use test112::pic::ppm::FILE_TYPE_PPM;
use common::*;
use test112::common::keep_two_decimal_places;


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


