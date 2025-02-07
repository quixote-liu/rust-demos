/* 使用三种方法修复下面的错误  */
fn invalid_output<'a>(s: &'a String) -> &'a String {
    s
}

fn demo() {
    let s = String::from("hello");
    let a = invalid_output(&s);
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