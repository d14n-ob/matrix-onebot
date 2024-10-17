use matrix_sdk::config::SyncSettings;
use crate::config::LANG;

mod matrix;
mod onebot;
mod constant;
mod error;
mod config;
mod sql;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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
