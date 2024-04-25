fn owner_demo() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_owner_demo() {
        owner_demo();
    }
}
