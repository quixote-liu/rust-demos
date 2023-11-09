use std::str::FromStr;

pub fn path_demo_run() {
    let p  = String::from_str("/hello/lcs/sss/xxx").unwrap();
    let mut path = std::path::PathBuf::from(p);
    path.pop();
    println!("path = {:?}", path.clone());
    path.pop();
    println!("path = {:?}", path.clone());
}