// todo: 多账户AH实现
// use std::cell::OnceCell;
// use std::sync::Arc;
// use tokio::task::JoinHandle;
// use walle_core::{ActionHandler, EventHandler, GetSelfs, GetStatus, GetVersion, OneBot, WalleResult};
// use walle_core::action::Action;
// use walle_core::event::Event;
// use walle_core::prelude::{Resp, Selft, Version};
// use crate::onebot::actions::MatrixHandler;
// use crate::onebot::matrix::get_self;

// 此文件很不完善!
// 最简化实现, 等待未来实现

// struct MatrixActionHandler;
//
// impl GetStatus for MatrixActionHandler {
//     async fn is_good(&self) -> bool {
//         true
//     }
// }
//
// impl GetSelfs for MatrixActionHandler {
//     async fn get_selfs(&self) -> Vec<Selft> {
//         vec![get_self()]
//     }
//
//     async fn get_impl(&self, selft: &Selft) -> String {
//         let _ = selft;
//         "matrix-onebot".to_string()
//     }
// }
//
// impl GetVersion for MatrixActionHandler {
//     fn get_version(&self) -> Version {
//         Version {
//             implt: "matrix-onebot".to_string(),
//             version: "0.1.0".to_string(),
//             onebot_version: 12.to_string(),
//         }
//     }
// }
//
// #[async_trait::async_trait]
// impl ActionHandler<Event, Action, Resp> for MatrixActionHandler {
//     type Config = ();
//
//     async fn start<AH, EH>(&self, ob: &Arc<OneBot<AH, EH>>, config: Self::Config) -> WalleResult<Vec<JoinHandle<()>>>
//     where
//         AH: ActionHandler<Event, Action, Resp> + Send + Sync + 'static,
//         EH: EventHandler<Event, Action, Resp> + Send + Sync + 'static
//     {
//         let single_handler = MatrixHandler {
//             client: OnceCell::default()
//         };
//         match single_handler.start {  }
//     }
//
//     async fn call<AH, EH>(&self, action: Action, ob: &Arc<OneBot<AH, EH>>) -> WalleResult<Resp>
//     where
//         AH: ActionHandler<Event, Action, Resp> + Send + Sync + 'static,
//         EH: EventHandler<Event, Action, Resp> + Send + Sync + 'static
//     {
//         todo!()
//     }
// }