use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // 实现 fmt 方法
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn demo() {
    let origin = Point { x: 0, y: 0 };
    // 填空
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

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