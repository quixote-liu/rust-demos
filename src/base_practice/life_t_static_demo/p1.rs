//https://practice-zh.course.rs/lifetime/static.html

/* 使用两种方法填空 */
fn demo() {
    __;
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}