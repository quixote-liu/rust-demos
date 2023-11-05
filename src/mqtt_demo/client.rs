use std::path::PathBuf;
use std::time::Duration;
use std::process;
use std::thread::sleep;

extern crate paho_mqtt as mqtt;

const BROKER : &str = "tcp://192.168.31.137:1883";
const CLIENT : &str = "rust_publish";
const TOPICS : &[&str] = &["testtopic/demo"];

pub fn demo_run() {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(BROKER.to_string())
        .client_id(CLIENT.to_string())
        .allow_disconnected_send_at_anytime(true)
        .persist_qos0(true)
        // .persistence(mqtt::PersistenceType::FilePath(PathBuf::from("./message_cache")))
        .max_buffered_messages(1000)
        .finalize();
    let cli = mqtt::Client::new(create_opts).unwrap();

    let conn_opt = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .finalize();

    if let Err(err) = cli.connect(conn_opt.clone()) {
        println!("连接北向mqtt broker 失败 error = {:?}", err);
        process::exit(1);
    }

    loop {
        let content = "hello, this is first message".to_string();
        let msg = mqtt::Message::new(TOPICS[0].to_string(), content, 0);
        if let Err(err) = cli.publish(msg) {
            println!("推送数据失败: {:?}", err);
        } else {
            println!("推送数据成功")
        }
        sleep(Duration::from_secs(1));
    }

    // let mut index = 0;
    // loop {
    //     // 先推送10次信息
    //     if index < 10 {
    //         let content = "hello, this is first message".to_string();
    //         let msg = mqtt::Message::new(TOPICS[0].to_string(), content, 0);
    //         if let Err(err) = cli.publish(msg) {
    //             println!("推送第一阶段数据失败: {:?}", err);
    //             break;
    //         } else {
    //             index += 1;
    //             sleep(Duration::from_secs(1));
    //             continue;
    //         }
    //     }

    //     // 断开连接, 并推送5次消息
    //     cli.disconnect(None).unwrap();
    //     if index < 15 {
    //         let content = "hello, this is second message".to_string();
    //         let msg = mqtt::Message::new(TOPICS[0].to_string(), content, 0);
    //         if let Err(err) = cli.publish(msg) {
    //             println!("向北向服务推送数据失败: {:?}", err);
    //         }
    //         index += 1;
    //         sleep(Duration::from_secs(1));
    //         continue;
    //     }

    //     // 恢复连接
    //     cli.connect(conn_opt.clone()).unwrap();
    //     break;
    // }

    // 等待10秒
    // sleep(Duration::from_secs(10));
}

