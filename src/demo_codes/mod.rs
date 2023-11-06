mod async_demo;
use futures::{executor};

pub fn demo_codes_run() {
    executor::block_on(async_demo::async_main());
}