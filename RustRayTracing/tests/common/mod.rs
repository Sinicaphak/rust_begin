use std::fmt::Display;

pub fn pretty_print<T: Display>(to_print: T){
    println!("============== output ==============");
    println!("{to_print}");
    println!("============== output end ==============");
}

