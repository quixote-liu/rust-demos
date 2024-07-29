/* 添加合适的生命周期让下面代码工作 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl <'a> ImportantExcerpt<'a>{
    fn level(&'a self) -> i32 {
        3
    }
}

fn demo() {
    let hello = "hello";
    let ie = ImportantExcerpt{part: hello};
    let res = ie.level();
    print!("res = {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo() {
        demo()
    }
}
