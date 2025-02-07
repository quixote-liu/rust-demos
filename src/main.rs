mod front_of_host;
mod demo;
mod vector;
mod inventory;
mod iterator;
mod box_demo;
mod thread_demo;
mod mutex_demo;
mod match_demo;
mod object_demo;
mod cal;
mod hello;
mod enum_demo;
mod add;
mod demo_codes;
mod display_demo;
mod mqtt_demo;
mod path_demo;
mod command_demo;
mod json_demo;
mod practice;
mod base_practice;
mod file_demo;
mod xinput_demo;
mod screen_rotate;

use std::{vec, fmt::{Display, Debug}, collections::btree_map::Values, ops::Add, process::Command, io::Read};
use encoding::codec::utf_8::from_utf8;
use inventory::ShirtColor;
use crate::demo::max;
use std::fs::File;

fn main() {
    // demo::run();
    // vector::for_range();

    // vector::for_enum();

    // filetest::open_file();

    // // filetest::unwarp_test();

    // let username_result = filetest::read_username_from_file();
    // match username_result {
    //     Ok(username)=> println!("the username is ={:?}", username),
    //     Err(error) => panic!("read username from file failed: {:?}", error)
    // }

    // let store = inventory::Inventory{
    //     shirts: vec![ShirtColor::Bule, ShirtColor::Red, ShirtColor::Bule],
    // };
    // let shirt = store.most_stocked();
    // match shirt {
    //     ShirtColor::Bule => println!("the bule"),
    //     ShirtColor::Red => println!("the red"),
    // }

    // box_demo::box_demo();

    // thread_demo::run();
    // thread_demo::run_move();
    // thread_demo::run_mpsc();

    // mutex_demo::run();
    // let array = [1, 2, 3, 1,2,34,2,123,3323,453];
    // let res = demo::max(&array);
    // println!("demo::max = {}", res);

    // match_demo::run();
    // match_demo::while_let();
    // match_demo::for_demo();

    // println!("13/2 = {}", 13/2);

    // hello::ferris_says_out();
    // hello::demo_test();


    // enum_demo::enum_demo();
    // enum_demo::if_let();

    // let case = vec![-3,0,1,-3,1,1,1,-3,10,0];
    // cal::unique_occurrences::Solution::unique_occurrences(case);
    
    // for i in 1..2 {
    //     println!("hello,world")
    // }
    // let str1 = String::from("hello,world");
    // println!("{}", str1.get(0..1).unwrap());
    
    // let mut a = vec![2, 0];
    // let m = 1;
    // let mut b = vec![1];
    // let n = 1;
    // cal::merge::Solution::merge(&mut a, m, &mut b, n);

    // mqtt_demo::client::demo_run();

    // path_demo::path_demo_run();

    // let sound = command_demo::get_fpad7_sound();
    // println!("sound = {:?}", sound);
    // command_demo::set_fpad7_volumn(50);
    // command_demo::set_fpad7_enable(false);


    // cal_demo()

    // some_demo();

    // some_demo();

    // for i in 10..0 {
    //     println!("{i}")
    // }

    // command_demo::exe_cmd()

    screen_rotate::output_message()
}

fn cal_demo() {
    let volume:u8 = 40;
    let ratio = (volume as f64)/100.0;
    println!("ratio = {}", ratio);
}

fn some_demo() {
    if let Ok(mut file) = File::open("/sys/class/misc/vendor_storage/modecode") {
        let mut model_code = String::new();
        let _ = file.read_to_string(&mut model_code);
        println!("model_code = [{}]", model_code);
    }

    if let Ok(mut file) = File::open("/sys/class/misc/vendor_storage/serialnum") {
        let mut sn = String::new();
        let _ = file.read_to_string(&mut sn);
        println!("serialnum = [{}]", sn);
    }

    println!("END")
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display+Clone, U: Clone+Debug
{
    90
}

// fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


// 最大值
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &v in list {
        if v > max {
            max = v;
        }
    }
    return max;
}


