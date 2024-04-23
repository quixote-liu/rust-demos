use serde_json;
use serde::{Serialize, Deserialize};
use std::{fs::File, io::Write};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn serialize_point() {
    let point = Point{x: 1, y:2};
    let res = serde_json::to_string(&point).unwrap();
    let mut f = File::create("./point.json").unwrap();
    let res = f.write(res.as_bytes()).unwrap();
    println!("file write size = {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_serialize_point() {
        serialize_point();
    }
}