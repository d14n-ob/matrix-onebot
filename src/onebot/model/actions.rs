use walle_core::action::{DeleteMessage, GetUserInfo, SendMessage};
use walle_core::prelude::TryFromAction;

#[derive(Debug, Clone, TryFromAction)]
pub enum MatrixAction {
    // GetLatestEvents(GetLatestEvents),
    GetSupportedActions {},
    GetStatus {},
    GetVersion {},

    SendMessage(SendMessage),
    DeleteMessage(DeleteMessage),

    GetSelfInfo {},
    GetUserInfo(GetUserInfo),
    GetFriendList {},
}