use std::vec;
use inventory::ShirtColor;
mod front_of_host;
mod demo;
mod vector;
mod file_demo;
mod inventory;
mod iterator;
mod box_demo;
mod thread_demo;
mod mutex_demo;
mod match_demo;
mod object_demo;
mod cal;
mod hello;

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

    // match_demo::run();
    // match_demo::while_let();
    // match_demo::for_demo();

    // println!("13/2 = {}", 13/2);

    hello::ferris_says_out();
    
}


