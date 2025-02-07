
/* 使用两种方法填空 */
fn demo() {
    let v: &'static str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo() {
        demo()
    }
}