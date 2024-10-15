use walle_core::action::{GetLatestEvents, SendMessage};
use walle_core::prelude::TryFromAction;

#[derive(Debug, Clone, TryFromAction)]
pub enum MatrixAction {
    // GetLatestEvents(GetLatestEvents),
    GetSupportedActions {},
    GetStatus {},
    GetVersion {},

    SendMessage(SendMessage)
}