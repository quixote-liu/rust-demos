fn array_demo1() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(arr.len(), 5);

    let arr2 = ['a', 'b', 'c'];
    assert_eq!(arr2.len(), 3);
    assert_eq!(arr2[1], 'b');

    let arr3 = [1; 100];
    assert_eq!(arr3.len(), 100);
    assert_eq!(arr3[1], 1);
    assert_eq!(arr3[99], 1);

    let arr = ['a', 'b', 'c'];
    let ele = arr[0]; // 只修改此行来让代码工作
    assert!(ele == 'a');

    let names = [String::from("Sunfei"), "Sunface".to_string()];
    // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
    let name0 = names.get(0).unwrap();
    let _name1 = &names[1];
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_array_demo1() {
        array_demo1();
    }
}