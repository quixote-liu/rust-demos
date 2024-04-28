fn tuple_demo() {
    let t0: (u8, i16) = (0, -1);
    let t1: (u8, (i16, u32)) = (8, (1, 2));
    assert_eq!(1, t1.1.0);
    // 填空让代码工作
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    // 打印长度超过12的元组
    #[derive(Debug)]
    struct long_tuple(
        i32, i32, i32, i32,
        i32, i32, i32, i32,
        i32, i32, i32, i32,
        i32,
    );
    let too_long_tuple:long_tuple = long_tuple(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);

    // 使用模式匹配来解构元组
    let tup = (1, 1.4, "hello");
    let (a, b, c) = tup;
    assert_eq!(a, 1);
    assert_eq!(b, 1.4);
    assert_eq!(c, "hello");

    // 解构式赋值
    let (x, y, z);
    (x, y, z) = (1, 2, 3);
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(z, 3);

    // 函数的参数和返回值
    let arg = (3, 4);
    let res = some_compute(arg);
    println!("res.0={}, res.1={}", res.0, res.1);
}

fn some_compute(mut arg: (i32, i32)) -> (i32, i32) {
    (arg.0+arg.1, arg.0*arg.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_demo() {
        tuple_demo();
    }
}