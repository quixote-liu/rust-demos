// use std::{thread, time};

// async fn learn_song() {
//     thread::sleep(time::Duration::from_millis(500));
//     println!("Learn song!");
// }

// async fn sing_song() {
//     thread::sleep(time::Duration::from_millis(100));
//     println!("Sing song!");
// }

// async fn dance() {
//     thread::sleep(time::Duration::from_millis(800));
//     println!("Dance!");
// }

// async fn learn_and_sing_song() {
//     learn_song().await;
//     sing_song().await;
// }

// pub async fn async_main() {
//     let f1 = dance();
//     let f2 = learn_song();
//     let f3 = sing_song();
//     futures::join!(f1, f2, f3);
// }
