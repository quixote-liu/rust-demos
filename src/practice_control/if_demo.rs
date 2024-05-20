fn if_demo() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(" 数字太小，先增加 10 倍再说");

            10 * n
        } else {
            println!("数字太大，我们得让它减半");

            n / 2
        };

    println!("{} -> {}", n, big_n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_demo() {
        if_demo();
    }
}