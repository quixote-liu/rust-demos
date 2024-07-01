mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;

struct Sheep {naked: bool, name: String}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

trait Animal {
    // 关联函数签名： Self指代实现者的类型
    fn new(name: String) -> Self;

    fn name(&self) -> String;

    fn noise(&self) -> String;

    fn talk(&self) {
        println!("{} sys {}", self.name(), self.noise());
    }
}

impl Animal for Sheep {
    fn new(name: String) -> Sheep {
        Sheep{name: name, naked: false}
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn noise(&self) -> String {
        if self.is_naked() {
            "baaaaaah".to_string()
        } else {
            "baaaaah~".to_string()
        }
    }

    // 默认的特征方法可以被重写
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn trait_demo() {
    // 这里的类型注释时必须的
    let mut dolly: Sheep = Animal::new("Dolly".to_string());
    // TODO ^ 尝试去除类型注释，看看会发生什么

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trait_demo() {
        trait_demo()
    }

    #[test]
    fn test_hello_demo() {
        p1::hello_demo()
    }

    #[test]
    fn test_multiply_demo() {
        p3::multiply_demo()
    }

    #[test]
    fn test_foo_bar_demo() {
        p4::foo_bar_demo()
    }
}