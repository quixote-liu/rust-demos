/* 添加 HRTB 使下面代码正常运行！ */
fn call_on_ref_zero<F>(f: F)
where for<'a> F: Fn(&'a i32)
{
    let zero = 0;
    f(&zero);
}

fn demo() {
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

