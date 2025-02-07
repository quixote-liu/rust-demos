// 转移所有权
fn invalid_output1() -> String {
    String::from("foo")
}

// 返回静态生命周期
fn invalid_output2() -> &'static str {
    "foo"
}

fn invalid_output3<'a>(s: &'a String) -> &'a String {
    s
}

fn demo() {
}

