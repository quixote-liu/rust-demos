enum Book {
    Papery(u32),
    Electronic{url: String}
}

pub fn enum_demo() {
    let book = Book::Papery(32);
    let ele = Book::Electronic { url: String::from("hello") };
    match book {
        Book::Papery(uu) => println!("the book is papery"),
        Book::Electronic { url } => println!("the book is electronic")
    }

    match ele {
        Book::Papery(uu) => println!("the ele is papery"),
        Book::Electronic { url } => println!("the ele is electronic")
    }
}

pub fn if_let() {
    let i = 0;
    match i {
        1 => {
            println!("the value of i is 1");
        },
        0 => {
            println!("the value of i is 0");
        },
        _ => {}
    }
}
