fn for_demo() {
    for n in 1..100 { // 修改此行，让代码工作
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in names.clone() {
        // do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // numbers中的元素实现了 Copy，因此无需转移所有权
    for n in numbers {
        // do something with name...
    }
    
    println!("{:?}", numbers);

    let a = [4,3,2,1];

    // 通过索引和值的方式迭代数组 `a` 
    for (i,v) in a.iter().enumerate() {
        println!("第{}个元素是{}",i+1,v);
    }
}

fn while_demo() {
    // 一个计数值
    let mut n = 1;

    // 当条件为真时，不停的循环
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n+=1;
    }
 
    println!("n 的值是 {}, 循环结束",n);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_for_demo() {
        for_demo();
    }

    #[test]
    fn test_while_demo() {
        while_demo()
    }
}