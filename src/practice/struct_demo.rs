#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn struct_demo() {
    let age = 30;
    let p = Person{
        name: String::from("sunface"),
        age,
        hobby: String::from("write rust code"),
    };
    println!("person ={:?}", p);
}

#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn test_struct_demo() {
        struct_demo();
    }

}

// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
struct UintStruct;

impl UintStruct{
    fn do_some_things(&self) {
        println!("this is uint struct, and doing some things");
    }
}

// 填空，让代码工作
fn do_something_with_unit(u: UintStruct) { 
    u.do_some_things();
}

fn uint_struct_demo() {
    let us = UintStruct;
    do_something_with_unit(us);
}

#[cfg(test)]
mod test2 {
    use super::*;
   
    #[test]
    fn test_uint_struct_demo() {
        uint_struct_demo();
    }
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn check_color(p: Color) {
    let Color(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}

fn tuple_struct_demo() {
    let v = Color(0, 127, 255);
    check_color(v);
}

#[cfg(test)]
mod test3 {
    use super::*;

    #[test]
    fn test_tuple_struct_demo() {
        tuple_struct_demo();
    }
}

#[derive(Debug)]
struct User{
    name: String,
    age: u8,
}

fn struct_op_demo() {
    let age = 18;
    let mut u = User{
        name: String::from("lcs"),
        age,
    };
    u.age = 20;
    assert_eq!(u.age, 20);
}

#[cfg(test)]
mod test4 {
    use super::*;

    #[test]
    fn test_struct_op_demo() {
        struct_op_demo();
    }
}
