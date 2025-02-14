pub fn run() {
    demo2()
}

fn demo2() {
    fn main() {
        let mut s = String::from("Hello");
        append_world(&mut s); // 可变借用
        println!("{}", s); // 输出 "Hello world"
    }
    
    fn append_world(s: &mut String) {
        s.push_str(" world");
    }

    main()
}

fn demo1() {
    let mut u = user { active: true, username: String::from("lcs"), email: String::from("lcs@email.com"), sign_in_count: 32 };

    println!("the user email is = {}", u.email);

    u.email = String::from("lcs2@email.com");
    println!("the user email is = {}", u.email);

    println!("the user name = {}", u.name());

    let c = Coin::Dime;
    let val = value_in_cert(c);
    println!("the value_is_cert = {}", val);

    let cc = Coin::Quarter(String::from("hello,world"));
    let val2 = value_in_cents(cc);
    println!("value in cents = {}", val2);
}

struct user {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl user {
    fn name(&self) -> &String {
        let name = &self.username;
        return name;
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cert(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

pub fn max<T: std::cmp::PartialOrd>(array: &[T]) -> &T {
    let mut max_index = 0;
    let mut index = 0;
    while index < array.len() {
        if array[max_index] < array[index] {
            max_index = index;
        };
        index += 1;
    };
    let res = &array[max_index];
    return res;
}