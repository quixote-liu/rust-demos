// 练习5 使用特征作为函数参数

// 实现 `fn summary` 
// 修复错误且不要移除任何代码行
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

pub fn demo5() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    // println!("{:?}", post);
    // println!("{:?}", weibo);

    summary_v2(&post);
    summary_v2(&weibo);
}

// 在下面实现 `fn summary` 函数
fn summary(s: &impl Summary) {
    let _ = s.summarize();
}

fn summary_v2<T: Summary>(s: &T) {
    let _ = s.summarize();
}

// 可以通过+号，可以实现多个trait
// pub fn notify(item: &(impl Summary + Display)) {