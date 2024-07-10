// 修复错误并实现缺失的代码
fn demo() {
    let mut v: Vec<i32> = Vec::from([1, 2, 3]);
    for i in 0..3 {
        println!("{:?}", v[i])
    }

    for i in 0..5 {
        let new_val = i as i32 + 2;
        if v.len() > i {
            v[i] = new_val;
        } else {
            v.push(new_val);
        }

        // if let Some(x) = v.get(i) {
        //     v[i] = x + 1
        // } else {
        //     v.push(i + 2)
        // }
    }
    
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        demo();
    }
}