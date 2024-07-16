
// 填空
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // 实现下面的代码
        panic!("this is panic")
     }

    println!("Exercise Failed if printing out this line!");
}

fn demo() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        demo()
    }
}