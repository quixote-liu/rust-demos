use std::str::FromStr;

fn str_demo() {
    let s: &str = "hello,world";

    let ss: Box<str> = "hello,world".into();
    greeting(&ss);
}

fn greeting(s: &str) {
    println!("greetiing: {}", s)
}

fn string_demo() {
    let mut s = "".to_string();
    s.push_str("hello,string");
    greeting(s.as_str());
}

fn string_demo2() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s.push('!');

    println!("{}", s)
}

fn string_replace() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}

fn string_demo3() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}

fn string_demo4() {
    let s1 = String::from("a");
    let s2 = String::from("b");
    let s3 = s1 + &s2;
    let s4: &str = &s3;
    println!("s4 = {}", s4);
}

fn bytes_demo() {
    let bytes_string = b"this is a byte string";
    println!("A bytes string: {:?}", bytes_string);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但是不支持 unicode 转义
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    let raw_string = br"\u{211D} is not escaped here";
    println!("raw_string: {:?}", raw_string.clone());

    if let Ok(res) = std::str::from_utf8(escaped) {
        println!("the raw_string value: {:?}", res);
    }
}

fn string_index() {
    let s1 = String::from("hi 武汉");
    let h = &s1[0..1];
    assert_eq!(h, "h");

    let s2 = &s1[3..6];
    assert_eq!(s2, "武");
}

fn string_range_char() {
    for e in "你好，武汉".chars() {
        println!("{e}");
    }
    for b in "你好".as_bytes() {
        println!("{b}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_range_char() {
        string_range_char();
    }

    #[test]
    fn test_string_index() {
        string_index();
    }

    #[test]
    fn test_str_demo() {
        str_demo();
    }

    #[test]
    fn test_string_demo() {
        string_demo();
    }

    #[test]
    fn test_string2_demo() {
        string_demo2();
    }

    #[test]
    fn test_string_replace() {
        string_replace();
    }

    #[test]
    fn test_string_demo4() {
        string_demo4();
    }

    #[test]
    fn test_bytes_demo() {
        bytes_demo();
    }
}