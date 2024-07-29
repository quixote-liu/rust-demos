/* 添加类型约束使下面代码正常运行 */
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32)
where 
    'a: 'b
{
    y = x;                      
    let r: &'b &'a i32 = &&0;   
}

fn demo() {
    let a = 1;
    let b = 2;
    f(&a, &b);
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
