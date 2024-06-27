// 练习1
// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        "I'm a good student".to_string()
    }
}
struct Teacher {}
impl Hello for Teacher {
    fn say_something(&self) -> String {
        "I'm not a bad teacher".to_string()
    }

    fn say_hi(&self) -> String {
        "Hi, I'm your new teacher".to_string()
    }
}

pub fn hello_demo() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!")
}
