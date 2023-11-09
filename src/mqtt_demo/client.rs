use chrono::Utc;
use std::str::FromStr;
use std::time::Duration;
use std::process;
use std::thread::sleep;

extern crate paho_mqtt as mqtt;

const BROKER : &str = "tcp://10.10.10.73:1883";
const CLIENT : &str = "rust_publish111";
const TOPICS : &[&str] = &["testtopic/demo"];
const QOS: i32 = 0;

pub fn demo_run() {
    // demo2()
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
    let cli = mqtt::AsyncClient::new(create_opts).unwrap();

    let conn_opt = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .retry_interval(Duration::from_secs(1))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .finalize();

    cli.connect(conn_opt.clone());

    let rx = cli.start_consuming();

    cli.subscribe_many(TOPICS, &[QOS]);

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
        .allow_disconnected_send_at_anytime(true)
        .client_id("s".to_string())
        .persistence(mqtt::PersistenceType::File)
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
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, QOS);
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
            let msg = mqtt::Message::new(TOPICS[0].to_string(), content, QOS);
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
        .send_while_disconnected(true)
        // .allow_disconnected_send_at_anytime(true)
        .max_buffered_messages(100)
        .finalize();
    let cli = mqtt::AsyncClient::new(create_opts).unwrap();

    let conn_opt = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .retry_interval(Duration::from_secs(1))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .finalize();

    let _ = cli.connect(conn_opt.clone()).wait_for(Duration::from_secs(3));

    // 连接时发送5个消息
    for i in 0..5 {
        let content = format!("hello, this is first message , index = {}", i);
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, QOS);
        if let Err(err) = cli.publish(msg).wait_for(Duration::from_secs(1)) {
            println!("推送数据失败: {:?}, index = {}", err, i);
        } else {
            println!("推送数据成功: index = {}", i);
        }
        sleep(Duration::from_secs(1));
    }
    let _ = cli.disconnect(None).wait_for(Duration::from_secs(3));
    // 断开连接时发送5个消息
    for i in 0..5 {
        let content = format!("hello, this is second message , index = {}", i);
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, QOS);
        if let Err(err) = cli.publish(msg).wait_for(Duration::from_secs(1)) {
            println!("推送数据失败: {:?}, index = {}", err, i);
        } else {
            println!("推送数据成功: index = {}", i);
        }
        sleep(Duration::from_secs(1));
    }
    let _ = cli.reconnect().wait_for(Duration::from_secs(3));
    // 等待数据发送
    let mut index = 0;
    loop {
        let content = format!("hello, this is second message , index = {}", index);
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, QOS);
        if let Err(err) = cli.publish(msg).wait_for(Duration::from_secs(1)) {
            println!("推送数据失败: {:?}, index = {}", err, index);
        } else {
            println!("推送数据成功: index = {}", index);
        }
        index += 1;
        sleep(Duration::from_secs(3));
    }
}

// 一直循环发送消息
fn demo_auto_run() {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(BROKER.to_string())
        .client_id("9".to_string())
        .send_while_disconnected(true)
        // .allow_disconnected_send_at_anytime(false)
        .persistence(mqtt::PersistenceType::FilePath(std::path::PathBuf::from("./message_cache")))
        .max_buffered_messages(100)
        .finalize();
    let cli = mqtt::AsyncClient::new(create_opts).unwrap();

    let conn_opt = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .retry_interval(Duration::from_secs(1))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .finalize();

    let _ = cli.connect(conn_opt.clone()).wait_for(Duration::from_secs(3));

    let mut index = 0;
    loop {
        let content = format!("hello, this is test message , index = {}", index);
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, QOS);
        if let Err(err) = cli.publish(msg.clone()).wait_for(Duration::from_secs(1)) {
            println!("推送数据失败: {:?}, index = {}, msg.retained = {}", err, index, msg.retained());
        } else {
            println!("推送数据成功: index = {}, msg.retained = {}", index, msg.retained());
        }
        index += 1;
        sleep(Duration::from_secs(1));
    }
}