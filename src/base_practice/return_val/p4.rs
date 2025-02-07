use std::num::ParseIntError;

// 使用两种方式填空: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().map(|v| v + 2)
}

fn demo() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        demo()
    }
}