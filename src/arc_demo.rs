use std::sync::OnceLock;
use std::sync::Arc;

// 定义 trait，要求实现 Send + Sync
trait Config: Send + Sync {
    fn get_value(&self) -> &str;
}

// 具体实现类型
struct AppConfig {
    value: String,
}

impl AppConfig {
    fn new(value: &str) -> Self {
        AppConfig {
            value: value.to_string(),
        }
    }
}

impl Config for AppConfig {
    fn get_value(&self) -> &str {
        &self.value
    }
}

// 全局变量（只读，使用 OnceCell + Arc）
static GLOBAL_CONFIG: OnceLock<Arc<dyn Config>> = OnceLock::new();

fn init_global_config() -> Arc<dyn Config> {
    Arc::new(AppConfig::new("default_value"))
}

pub fn arc_demo() {
    // 初始化全局配置（线程安全，仅一次）
    let config = GLOBAL_CONFIG.get_or_init(|| init_global_config());

    // 多线程中访问
    let handle = std::thread::spawn(move || {
        println!("Thread 1: {}", config.get_value());
    });

    let config_clone = Arc::clone(config);
    let handle2 = std::thread::spawn(move || {
        println!("Thread 2: {}", config_clone.get_value());
    });

    handle.join().unwrap();
    handle2.join().unwrap();
}
