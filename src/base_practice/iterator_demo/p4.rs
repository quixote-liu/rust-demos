fn demo() {
    let arr = vec![0; 10];
    for i in arr.iter() {
        println!("{}", i)
    }

    println!("{:?}",arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        demo()
    }
}