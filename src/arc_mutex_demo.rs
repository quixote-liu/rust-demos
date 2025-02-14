use std::sync::{OnceLock, Mutex, Arc};

// 定义 trait，要求实现 Send + Sync
trait Counter: Send + Sync {
    fn increment(&self);
    fn get_count(&self) -> u32;
}

// 具体实现类型
struct ThreadSafeCounter {
    count: Mutex<u32>,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        ThreadSafeCounter {
            count: Mutex::new(0),
        }
    }
}

impl Counter for ThreadSafeCounter {
    fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    fn get_count(&self) -> u32 {
        *self.count.lock().unwrap()
    }
}

// 全局变量（可变，使用 OnceLock + Arc<Mutex<T>>）
static GLOBAL_COUNTER: OnceLock<Arc<dyn Counter>> = OnceLock::new();

fn init_global_counter() -> Arc<dyn Counter> {
    Arc::new(ThreadSafeCounter::new())
}

pub fn run() {
    // 初始化全局计数器
    let counter = GLOBAL_COUNTER.get_or_init(|| init_global_counter());

    // 多线程中修改
    let handles: Vec<_> = (0..10)
        .map(|_| {
            // let counter = Arc::clone(counter);
            std::thread::spawn(move || {
                counter.increment();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.get_count()); // 输出 10
}
