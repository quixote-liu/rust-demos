use std::vec;

use inventory::ShirtColor;


mod front_of_host;

mod demo;

mod vector;

mod filetest;

mod inventory;

mod iterator;

fn main() {
    // demo::run();
    vector::for_range();

    vector::for_enum();

    filetest::open_file();

    // filetest::unwarp_test();

    let username_result = filetest::read_username_from_file();
    match username_result {
        Ok(username)=> println!("the username is ={:?}", username),
        Err(error) => panic!("read username from file failed: {:?}", error)
    }

    let store = inventory::Inventory{
        shirts: vec![ShirtColor::Bule, ShirtColor::Red, ShirtColor::Bule],
    };
    let shirt = store.most_stocked();
    match shirt {
        ShirtColor::Bule => println!("the bule"),
        ShirtColor::Red => println!("the red"),
    }
}


