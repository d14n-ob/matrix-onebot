use std::sync::Arc;
use matrix_sdk::{Client, ruma::{
    events::room::{
        message::SyncRoomMessageEvent,
        member::{
            StrippedRoomMemberEvent,
        }
    },
}, Room, ServerName};
use matrix_sdk::ruma::events::room::member::SyncRoomMemberEvent;
use matrix_sdk::ruma::UserId;
use crate::config::{CONFIG};
use crate::matrix::handlers::EventHandler;

pub async fn add_event_handlers(client: Client, event_handler: EventHandler) -> anyhow::Result<Client> {
    let event_handler = Arc::new(event_handler);

    // MessageEvent
    {
        let event_handler = Arc::clone(&event_handler);
        client.add_event_handler(move |ev: SyncRoomMessageEvent, room: Room| async move {
            event_handler.message(ev, room).await;
        });
    }

    // InviteEvent
    {
        let event_handler = Arc::clone(&event_handler);
        client.add_event_handler(|room_member: StrippedRoomMemberEvent, room: Room, client: Client| async move {
            event_handler.invite(room_member, room, client).await;
        });
    }

    // MemberEvent
    {
        let event_handler = Arc::clone(&event_handler);
        client.add_event_handler(move |ev: SyncRoomMemberEvent, room: Room| async move {
            event_handler.member(ev, room).await
        });
    }

    // RedactEvent 消息撤回
    // {
    //     let event_handler = Arc::clone(&event_handler);
    //     client.add_event_handler(move |ev: RoomRedactionEvent, room: Room, client: Client| async move {
    //
    //     })
    // }

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