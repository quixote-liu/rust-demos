/* 添加类型约束使下面代码正常运行 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn demo() {
    let part = "hello";
    let ie = ImportantExcerpt{part: part};
    let res = ie.announce_and_return_part("hello");
    println!("Success!, result={}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo() {
        demo()
    }
}