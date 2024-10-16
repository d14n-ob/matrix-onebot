use crate::config::CONFIG;
use crate::onebot::actions::MatrixHandler;
use matrix_sdk::Client;
use std::sync::Arc;
use walle_core::{event::Event, obc::ImplOBC, ActionHandler, EventHandler, OneBot};

pub async fn create_onebot(client: Client) -> Arc<OneBot<MatrixHandler, ImplOBC<Event>>> {
    let ah = MatrixHandler::new(client);
    let ob = Arc::new(OneBot::new(
        ah,
        ImplOBC::new("impl".to_string()),
    ));

    let impl_config = (&CONFIG.read().unwrap().onebot_conn).to_owned();

    ob.start((), impl_config, true).await.unwrap();
    ob
}

