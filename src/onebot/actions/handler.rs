use once_cell::sync::OnceCell;
use std::sync::Arc;
use async_trait::async_trait;
use matrix_sdk::Client;
use matrix_sdk::ruma::exports::serde::de::Error;
use tokio::task::JoinHandle;
use walle_core::action::{Action, GetLatestEvents};
use walle_core::{ActionHandler, EventHandler, GetSelfs, GetStatus, GetVersion, OneBot, WalleError, WalleResult};
use walle_core::event::Event;
use walle_core::prelude::{Selft, Version};
use walle_core::resp::{Resp, RespError};
use crate::constant::{MATRIX_ONEBOT, ONEBOT_VERSION, PLATFORM, VERSION};
use crate::error::{self, map_action_parse_error};
use crate::onebot::model::actions::MatrixAction;

pub struct MatrixHandler {
    pub client: OnceCell<Arc<Client>>
}


impl MatrixHandler {
    async fn _handle(&self, action: Action) -> Result<Resp, RespError> {
        match MatrixAction::try_from(action).map_err(map_action_parse_error)? {
            //GetLatestEvents
            MatrixAction::GetSupportedActions {} => Self::get_supported_actions().map(Into::into),
            MatrixAction::GetStatus {} => Ok(self.get_status().await.into()),
            MatrixAction::GetVersion {} => Ok(self.get_version().into()),

            MatrixAction::SendMessage(c) => self.send_message(c).await.map(Into::into),
            MatrixAction::DeleteMessage(c) => self.delete_message(c).await.map(Into::into),

            MatrixAction::GetSelfInfo {} => self.get_self_info().await.map(Into::into),
            MatrixAction::GetUserInfo(c) => self.get_user_info(c).await.map(Into::into),
            MatrixAction::GetFriendList {} => self.get_friend_list().await.map(Into::into),
        }
    }
    pub fn new(client: Client) -> Self {
        MatrixHandler { client: OnceCell::from(Arc::new(client)) }
    }
}

pub type RespResult<T> = Result<T, RespError>;

impl MatrixHandler {
    pub async fn selft(&self) -> Result<Selft, RespError> {
        Ok(Selft {
            platform: PLATFORM.to_string(),
            user_id: self.get_client()?.user_id().unwrap().to_string()
        })
    }
    pub fn get_client(&self) -> Result<&Arc<Client>, RespError> {
        self.client.get().ok_or(error::client_not_initialized(""))
    }
    async fn get_latest_events(&self, c: GetLatestEvents) -> Result<Vec<Event>, RespError> {
        todo!()
    }
    fn get_supported_actions() -> RespResult<Vec<&'static str>> {
        Ok(vec![
            // "get_latest_events",
            "get_supported_actions",
            "get_status",
            "get_version",
            "send_message",
            "delete_message",
            "get_self_info",
            "get_user_info",
            "get_friend_list",
        ])
    }
}


// impl ActionHandler

#[async_trait]
impl GetStatus for MatrixHandler {
    async fn is_good(&self) -> bool {
        // 现阶段 Client 连接失败会直接 panic
        true
    }
    // async fn get_status(&self) -> Status
    // where
    //     Self: Sized,
    // {
    //     Status {
    //         good: self.is_good().await,
    //         bots: self.get_selfs().await
    //             .into_iter()
    //             .map(|selft: Selft| Bot {
    //                 selft,
    //                 online: true
    //             })
    //             .collect(),
    //     }
    // }
}

#[async_trait]
impl GetSelfs for MatrixHandler {
    async fn get_selfs(&self) -> Vec<Selft> {
        if let Ok(client) = self.get_client() {
            vec![Selft {
                platform: PLATFORM.to_owned(),
                user_id: client.user_id().unwrap().to_string()
            }]
        } else {
            vec![]
        }
        // 好奇为什么不从 selft 取
        // vec![self.selft().await.unwrap()]
    }

    async fn get_impl(&self, _: &Selft) -> String {
        MATRIX_ONEBOT.to_owned()
    }
}

impl GetVersion for MatrixHandler {
    fn get_version(&self) -> Version {
        Version {
            implt: MATRIX_ONEBOT.to_owned(),
            version: VERSION.to_owned(),
            onebot_version: ONEBOT_VERSION.to_string(),
        }
    }
}

#[async_trait]
impl ActionHandler for MatrixHandler {
    type Config = ();

    async fn start<AH, EH>(&self, _ob: &Arc<OneBot<AH, EH>>, _config: Self::Config) -> WalleResult<Vec<JoinHandle<()>>>
    where
        AH: ActionHandler<Event, Action, Resp> + Send + Sync + 'static,
        EH: EventHandler<Event, Action, Resp> + Send + Sync + 'static
    {
        Ok(vec![])
    }

    async fn call<AH, EH>(&self, action: Action, _ob: &Arc<OneBot<AH, EH>>) -> WalleResult<Resp>
    where
        AH: ActionHandler<Event, Action, Resp> + Send + Sync + 'static,
        EH: EventHandler<Event, Action, Resp> + Send + Sync + 'static
    {
        self._handle(action).await.map_err(|e| WalleError::custom(e.message))
    }
}