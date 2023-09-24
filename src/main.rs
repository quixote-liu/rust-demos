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
mod enum_demo;

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

    let case = vec![-3,0,1,-3,1,1,1,-3,10,0];
    cal::unique_occurrences::Solution::unique_occurrences(case);
    

}


