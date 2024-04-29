use openssl::x509;

enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// 枚举成员可以持有各种类型
#[derive(Debug, Clone)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Message::Quit => {
                write!(f, "Quit类型")
            },
            Message::Move { x, y } => {
                write!(f, "Move类型 x={}, y={}", x, y)
            },
            Message::Write(a) => {
                write!(f, "Write类型 a={}", a)
            },
            Message::ChangeColor(x, y, z) => {
                write!(f, "ChangeColor类型, x={}, y={}, z={}", x, y, z)
            },
        }
    }
}

fn enum_demo() {
    let _n_zero = Number::Zero;
    let _n_one = Number::One;

    let _m = Message::Move { x: 1, y: 2 };
    match _m {
        Message::Move { x, y } => {
            println!("类型为Move x={}, y={}", x, y);
        },
        _ => {
            println!("是其他类型");
        },
    }
    // 除了使用match，还可以使用if let
    let color = Message::ChangeColor(23, 24, 25);
    if let Message::ChangeColor(x, y, z) = color {
        println!("颜色改变, x={}, y={}, z={}", x, y, z);
    } else {
        println!("错误的枚举格式");
    }

    // 可以当做数据
    let msg: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("hello")),
    ];
    let show_msg = |msg: Message| {
        println!("{}", msg);
    };
    for m in msg.clone()     {
        show_msg(m);
    }
        
    let msg2 = &msg[..];
    let mut msg2_vec = msg2.to_vec();
    msg2_vec.push(Message::ChangeColor(1, 2, 3));
    for m in msg2_vec {
        show_msg(m);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enum_demo() {
        enum_demo();
    }
}

