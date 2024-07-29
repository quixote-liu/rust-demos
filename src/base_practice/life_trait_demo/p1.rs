/* 使用生命周期注释结构体
1. `r` 和 `s` 必须是不同生命周期
2. `s` 的生命周期需要大于 'r'
*/
struct DoubleRef<'a, T> {
    r: &'a T,
    s: &'a T
}
fn demo() {
    let r = 1;
    let s = 2;
    let dr = DoubleRef{r: &r, s: &s};
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