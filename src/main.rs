use std::{io, panic};
use std::io::Write;
use matrix_sdk::config::SyncSettings;
use crate::config::LANG;
use crate::constant::{VERSION};

mod matrix;
mod onebot;
mod constant;
mod error;
mod config;
mod sql;

#[tokio::main]
async fn main() {
    set_panic_hook();
    tracing_subscriber::fmt::init();
    hello();

    // 启动 MatrixClient
    // matrix::entry(ob).await;
    let client = matrix::create_client().await
        .expect(&LANG.read().unwrap().error_matrix_login_failed);

    // 初始化 OneBot
    let ob = onebot::create_onebot(client.clone()).await;

    // 初始化 MatrixClient
    let eh = matrix::EventHandler::init(ob);
    let client = matrix::add_event_handlers(client, eh).await
        .expect(&LANG.read().unwrap().error_matrix_add_event_handler_failed);

    client.sync(SyncSettings::default()).await
        .expect(&LANG.read().unwrap().error_matrix_sync_failed);
}

fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        println!("{}", panic_info);
        println!("Press Enter to continue...");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut String::new()).unwrap();
    }));
}

fn hello() {
    println!(
        "\
======
MATRIX-ONEBOT v{}
欢迎! 当前处于 v0.x.x 快速更新不稳定版本, 不会有预先的变动通知!
======\
", VERSION
    );
}