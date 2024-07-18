// 结构体的字段的生命周期需要 >= 结构体

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

fn demo()
{ 
  let var_a = 35;
  
  
  {
    let example: Example;
    let var_b = NoCopyType {};
    
    /* 修复错误 */
    example = Example { a: &var_a, b: &var_b };

    println!("(Success!) {:?}", example);
  }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo() {
        demo()
    }
}
