/* 使用三种方法修复下面的错误  */
fn invalid_output<'a>() -> &'a String {
    let a = String::from("foo");
    &a
}

fn demo() {
    let a = invalid_output();
    print!("a = {}", a);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo() {
        demo()
    }
}