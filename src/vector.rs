pub fn for_range() {
    let vv = vec![100,20, 30];
    for i in &vv {
        println!("{i}");
    }

    let mut v2 = vec![1,2,3,4];
    for i in &mut v2 {
        *i += 20;
    }
    for i in &v2 {
        println!("{i}");
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn for_enum() {
    let row = vec![
        SpreadsheetCell::Int(12),
        SpreadsheetCell::Float(23.12),
        SpreadsheetCell::Text(String::from("hello")),
    ];
    for i in &row {
        match i {
        SpreadsheetCell::Int(i32) => println!("is int = {i32}"),
        SpreadsheetCell::Float(f64) => println!("is float = {f64}"),
        SpreadsheetCell::Text(ss) => println!("is text = {ss}"),
        }
    }
}