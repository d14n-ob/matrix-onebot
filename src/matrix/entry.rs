use std::sync::Arc;
use matrix_sdk::{Client, ruma::{
    events::room::{
        message::SyncRoomMessageEvent,
        member::{
            StrippedRoomMemberEvent,
        }
    },
}, Room, ServerName};
use matrix_sdk::ruma::UserId;
use crate::config::{CONFIG, LANG};
use crate::matrix::handlers::EventHandler;
use crate::sql::DATABASE;

pub async fn add_event_handlers(client: Client, event_handler: EventHandler) -> anyhow::Result<Client> {
    let event_handler = Arc::new(event_handler);

    // MessageEvent
    {
        // todo: RwLock impl Send -> 换用 Tokio RwLock
        let matrix_events_table = DATABASE.get_matrix_events_table();
        let matrix_messages_table = DATABASE.get_matrix_messages_table();
        let insert_failed_msg = (&LANG.read().unwrap().error_database_table_insert_failed).to_owned();
        let query_failed_msg = (&LANG.read().unwrap().error_database_table_query_failed).to_owned();
        let event_handler = Arc::clone(&event_handler);
        client.add_event_handler(move |ev: SyncRoomMessageEvent, room: Room| async move {
            event_handler.message(ev, room, matrix_events_table, matrix_messages_table, insert_failed_msg, query_failed_msg).await;
        });
    }

    // InviteEvent
    {
        let event_handler = Arc::clone(&event_handler);
        client.add_event_handler(|room_member: StrippedRoomMemberEvent, room: Room, client: Client| async move {
            event_handler.invite(room_member, room, client).await;
        });
    }

    Ok(client)
}

pub async fn create_client() -> anyhow::Result<Client> {
    let user_id = &CONFIG.read().unwrap().full_user_id;
    let user_id = UserId::parse(user_id)?;

    let server_domain = &CONFIG.read().unwrap().server_domain;
    let client =
        if server_domain.is_empty() {
            Client::builder().server_name(user_id.server_name()).build().await?
        } else {
            Client::builder()
                .server_name(&ServerName::parse(server_domain).unwrap())
                .build().await?
        };

    let password = &CONFIG.read().unwrap().password;
    client.matrix_auth().login_username(user_id, password).send().await?;

    Ok(client)
}