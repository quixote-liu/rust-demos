#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn demo() {
    // 填空
    let v : Vec<IpAddr>= Vec::from([
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ]);
    
    // 枚举的比较需要派生 PartialEq 特征
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo() {
        demo()
    }
}