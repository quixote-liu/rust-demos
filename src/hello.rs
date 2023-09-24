extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

pub fn ferris_says_out() {
    let out = String::from("Hello fellow Rustaceans!");
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    say(&out, width, &mut writer).unwrap();
}

pub fn demo_test() {
    let a = 100;
    let a = 200;
    print!("{}", a);

    let tup = (100, 2.3, "hello");
    print!("tup.0 = {}, tup.1 = {}, tup.2 = {}\n", tup.0, tup.1, tup.2);

    let a = [2, 3, 4, 5];
    let b = [2, 3, 4, 5,6];

    fn five() -> i32 {
        5
    }
    print!("five() = {}\n", five());

    let c = if five() > 0 {1} else {0};
    print!("c value = {}\n", c);

    let mut i = 0;
    while i < 10 {
        println!("while index = {}", i);
        i += 1;
    };

    for i in a.iter() {
        println!("a = {}", i);
    };
}

