extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

pub fn ferris_says_out() {
    let out = String::from("Hello fellow Rustaceans!");
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    say(&out, width, &mut writer).unwrap();
}