use std::sync::Arc;
use matrix_sdk::Client;
use walle_core::{OneBot, alt::TracingHandler, event::Event, obc::ImplOBC, resp::Resp, action::Action, config::{Heartbeat, HttpServer, ImplConfig}, GetStatus, GetSelfs, GetVersion, ActionHandler, WalleResult, EventHandler};
use walle_core::config::{HttpClient, WebSocketServer};
use crate::onebot::actions::MatrixHandler;

pub async fn create_onebot(client: Client) -> Arc<OneBot<MatrixHandler, ImplOBC<Event>>> {
    let ah = MatrixHandler::new(client);
    // let ob = Arc::new(OneBot::new(
    //     TracingHandler::<Event, Action, Resp>::default(),
    //     ImplOBC::new("impl".to_string()),
    // ));
    let ob = Arc::new(OneBot::new(
        ah,
        ImplOBC::new("impl".to_string()),
    ));

    let impl_config = ImplConfig {
        http: vec![
            HttpServer {
                host: std::net::IpAddr::from([127, 0, 0, 1]),
                port: 5700,
                access_token: None,
            }
        ],
        http_webhook: vec![
            HttpClient {
                implt: None,
                platform: None,
                access_token: None,
                url: "http://127.0.0.1:5701".to_owned(),
                timeout: 4,
            },
        ],
        websocket: vec![
            WebSocketServer {
                host: std::net::IpAddr::from([127, 0, 0, 1]),
                port: 5702,
                access_token: None,
            }
        ],
        websocket_rev: vec![],
        heartbeat: Heartbeat::default(),
    };

    ob.start((), impl_config, true).await.unwrap();
    ob
}

