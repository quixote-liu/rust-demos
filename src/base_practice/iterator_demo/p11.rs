use std::collections::HashMap;
fn demo() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}",folks);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        demo()
    }
}