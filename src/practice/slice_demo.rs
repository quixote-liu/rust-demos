fn slice_demo() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello,world" as &str;

    let arr: [char; 2] = ['武', '汉'];
    let slice1: &[char] = &arr[..2];
    assert_eq!(std::mem::size_of_val(&slice1), 16);

    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr2[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    // 字符串切片
    let ss = String::from("hello");
    let ss1 = &ss[0..2];
    let ss2 = &ss[..2];
    assert_eq!(ss1, ss2);

    let s = "你好，世界";
    // 修改以下代码行，让代码工作起来
    let slice = &s[0..3]; // UTF-8编码中，中文汉字占用3个字节
    assert!(slice == "你");

    let mut s = String::from("hello world");
    // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    let ch = first_character(&s);
    println!("the first character is: {}", ch);
    s.clear();
} 

fn first_character(s: &str) -> &str {
    &s[..1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_demo() {
        slice_demo();
    }
}