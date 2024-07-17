/* 添加合适的生命周期标注，让下面的代码工作 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demo() {
    let x = "hello";
    let y = "world11";
    let c = longest(x, y);

    print!("x={}, y={}, c={}", x, y, c);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo() {
        demo()
    }
}