// 填空
fn demo() {  
    let mut s = String::from("hello, world");

    // let slice1: &str = &s; // 使用两种方法
    let slice1: &str = s.as_str(); // 使用两种方法
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");

    let mut slice3: String = String::from("hello, world"); 
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!")
}

fn demo2() {
    // 问题:  我们的代码中发生了多少次堆内存分配？
    // 你的回答: 
    // 基于 `&str` 类型创建一个 String,
    // 字符串字面量的类型是 `&str`
   let s: String = String::from("hello, world!");

   // 创建一个切片引用指向 String `s`
   let slice: &str = &s;

   // 基于刚创建的切片来创建一个 String
   let s: String = slice.to_string();

   assert_eq!(s, "hello, world!");

   println!("Success!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        demo();
        demo2()
    }
}
