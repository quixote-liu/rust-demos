fn demo() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        demo()
    }
}