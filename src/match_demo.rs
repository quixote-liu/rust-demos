pub fn run() {
    let favorite_color : Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("the favorite color is {}", color);
    } else if is_tuesday {
        println!("today is tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("the age is more than 30, {age}");
        } else {
            println!("the age is less than 30, {age}");
        }
    } else {
        println!("nothing");
    }
}

pub fn while_let() {
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("the top value is {top}");
    }
}

pub fn for_demo() {
    let array = vec![1,2,3,4];
    for (index, value) in array.iter().enumerate() {
        println!("the index is {index}, the value is {value}");
    }
}