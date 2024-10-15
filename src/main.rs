use matrix_sdk::config::SyncSettings;

mod matrix;
mod onebot;
mod constant;
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // 启动 MatrixClient
    // matrix::entry(ob).await;
    let client = matrix::create_client().await.expect("登录失败");

    // 初始化 OneBot
    let ob = onebot::create_onebot(client.clone()).await;

    // 初始化 MatrixClient
    let eh = matrix::EventHandler::init(ob);
    let client = matrix::add_event_handlers(client, eh).await.expect("事件处理器添加失败");
    client.sync(SyncSettings::default()).await.expect("同步失败");
}
