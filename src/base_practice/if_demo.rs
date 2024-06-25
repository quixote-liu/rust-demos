fn if_demo() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(" 数字太小，先增加 10 倍再说");

            10 * n
        } else {
            println!("数字太大，我们得让它减半");

            n / 2
        };

    println!("{} -> {}", n, big_n);
}

fn loop_demo() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("count is 3");
        }
        if count == 4 {
            println!("count is 4");
        }
        if count == 5 {
            println!("count is 5");
        }
        if count == 10 {
            println!("count is 10, END");
            break;
        }
    }
    
    // 多层循环
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // 这只会跳出 inner1 循环
                break 'inner1; // 这里使用 `break` 也是一样的
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_demo() {
        if_demo();
    }

    #[test]
    fn test_loop_demo() {
        loop_demo();
    }
}