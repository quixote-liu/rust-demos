// From 被包含在 `std::prelude` 中，因此我们没必要手动将其引入到当前作用域来
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // 实现 `from` 方法
    fn from(v: i32) -> Self {
        Self{value: v}
    }
}

// 填空
fn demo() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30_i32.into();
    assert_eq!(num.value, 30);

    println!("Success!")
}