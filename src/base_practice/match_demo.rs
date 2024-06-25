enum Direction {
    East,
    West,
    North,
    South,
}

fn demo1() {
    let dire = Direction::South;
    match dire {
        Direction::South => {println!("this dire is South")},
        __ => {
            println!("East, West, North")
        },
    }
}

fn demo2() {
    let mut v = String::from("hello");
    let r = &mut v;
    match r {
        value => value.push_str(" world")
    }
}

fn match_demo() {
    demo1();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo1() {
        match_demo();
    }
}
