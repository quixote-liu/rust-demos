extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
    hello();
    ferris_says_out();
}

fn hello() {
    println!("hello, rust");
}

fn ferris_says_out() {
    let out = b"Hello fellow Rustaceans!";
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}