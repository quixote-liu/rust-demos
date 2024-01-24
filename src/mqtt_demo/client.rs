use chrono::Utc;
use std::time::Duration;
use std::process;
use std::thread::sleep;

extern crate paho_mqtt as mqtt;

// const BROKER : &str = "tcp://10.10.10.73:1883";
const BROKER : &str = "tcp://192.168.31.137:1883";
const CLIENT : &str = "rust_publish111";
const TOPICS : &[&str] = &["testtopic/demo"];

pub fn demo_run() {
    demo_auto_run()
}

// 接收离线消息
fn demo3() {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(BROKER.to_string())
        .client_id(CLIENT.to_string())
        .allow_disconnected_send_at_anytime(true)
        .max_buffered_messages(10)
        .delete_oldest_messages(true)
        .finalize();
    let cli = mqtt::Client::new(create_opts).unwrap();

    let conn_opt = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .retry_interval(Duration::from_secs(1))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .finalize();

    if let Err(err) = cli.connect(conn_opt.clone()) {
        println!("连接北向mqtt broker 失败 error = {:?}", err);
        process::exit(1);
    }

    let rx = cli.start_consuming();

    if let Err(err) = cli.subscribe(TOPICS[0], 1) {
        println!("Error subscribes topics {:?}", err);
    }

    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("{:?} {}", Utc::now(),msg);
        } else {
            println!("msg 为空")
        }
    }
}

// 主动在循环中不停的发送消息
fn demo2() {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(BROKER.to_string())
        .client_id(CLIENT.to_string())
        .allow_disconnected_send_at_anytime(true)
        // .persist_qos0(true)
        // .persistence(mqtt::PersistenceType::FilePath(PathBuf::from("./message_cache")))
        .max_buffered_messages(1000)
        .delete_oldest_messages(true)
        .finalize();
    let cli = mqtt::Client::new(create_opts).unwrap();

    let conn_opt = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .retry_interval(Duration::from_secs(1))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .finalize();

    if let Err(err) = cli.connect(conn_opt.clone()) {
        println!("连接北向mqtt broker 失败 error = {:?}", err);
        process::exit(1);
    }


    let mut is_disconnected = false;
    loop {
        let content = "hello, this is test message".to_string();
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, 1);
        if !is_disconnected {
            if let Err(err) = cli.publish(msg) {
                is_disconnected = true;
                println!("推送数据失败: {:?}", err);
            } else {
                println!("推送数据成功")
            }
        }
        if is_disconnected && !cli.is_connected() {
            let content = "hello, this is cache message".to_string();
            let msg = mqtt::Message::new(TOPICS[0].to_string(), content, 1);
            if let Err(err) = cli.publish(msg) {
                is_disconnected = true;
                println!("推送数据失败: {:?}", err);
            } else {
                println!("推送数据成功")
            }
        }
        println!("当前连接状态: is_disconnected = {}, cli.is_connected = {}",is_disconnected, cli.is_connected());
        sleep(Duration::from_secs(1));
    }
}

// 不能主动disconnect, 否则无法自动重连，维持不了旧会话
fn demo1() {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(BROKER.to_string())
        .client_id(CLIENT.to_string())
        .allow_disconnected_send_at_anytime(true)
        .persistence(mqtt::PersistenceType::File)
        .max_buffered_messages(1000)
        .finalize();
    let cli = mqtt::Client::new(create_opts).unwrap();

    let conn_opt = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .retry_interval(Duration::from_secs(1))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .finalize();

    if let Err(err) = cli.connect(conn_opt.clone()) {
        println!("连接北向mqtt broker 失败 error = {:?}", err);
        process::exit(1);
    }

    // 发送正常数据
    for _ in 0..5 {
        let content = "hello, this is test message".to_string();
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, 1);
        if let Err(err) = cli.publish(msg) {
            println!("推送数据失败: {:?}", err);
        } else {
            println!("推送数据成功");
        }
        sleep(Duration::from_secs(1));
    }
    // 断开连接
    let _ = cli.disconnect(None);
    // 开始发送离线数据
    for _ in 0..10 {
        let content = "hello, this is offline message".to_string();
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, 1);
        if let Err(err) = cli.publish(msg) {
            println!("推送离线数据失败: {:?}", err);
        } else {
            println!("推送离线数据成功");
        }
        sleep(Duration::from_secs(1));
    }
    // 不终止当前线程，进入循环等待
    let _ = cli.reconnect();
    loop {
        println!("等待离线数据发送....., cli.is_connected = {}", cli.is_connected());
        sleep(Duration::from_secs(1));
    }
}

// 一直循环发送消息
fn demo_auto_run() {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(BROKER.to_string())
        .client_id(CLIENT.to_string())
        // .allow_disconnected_send_at_anytime(true)
        // .max_buffered_messages(10)
        .finalize();
    let cli = mqtt::AsyncClient::new(create_opts).unwrap();

    let conn_opt = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .retry_interval(Duration::from_secs(1))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .finalize();

    let _ = cli.connect(conn_opt.clone()).wait_for(Duration::from_secs(3));

    let mut index = 0;
    loop {
        let content = format!("hello, this is test message , index = {}", index);
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, 0);
        if let Err(err) = cli.publish(msg).wait_for(Duration::from_secs(1)) {
            println!("推送数据失败: {:?}, index = {}", err, index);
        } else {
            println!("推送数据成功: index = {}", index);
        }
        index += 1;
        sleep(Duration::from_secs(1));
    }
}