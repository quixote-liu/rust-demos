trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn demo() {
    // 填空
    let birds: Vec<Box<dyn Bird>> = vec![
        Box::new(Duck),
        Box::new(Swan)
    ];

    for bird in birds {
        bird.quack();
        // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
        // 因此，以下代码会报错
        // bird.fly();
    }

    // let birds_2: Vec<dyn Bird> = vec![
    //     Box::new(Duck),
    //     Box::new(Swan)
    // ];
    // let bird1: dyn Bird = Duck;
}


mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        demo();
    }
}