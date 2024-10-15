use std::sync::Arc;
use matrix_sdk::{Client, config::SyncSettings, ruma::{
    user_id,
    events::room::{
        message::SyncRoomMessageEvent,
        member::{
            StrippedRoomMemberEvent,
        }
    },
}, Room, ServerName};
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

    Ok(client)
}

pub async fn create_client() -> anyhow::Result<Client> {
    let alice = user_id!("@bot:matrix.meowl.cc");
    // let client = Client::builder().server_name(alice.server_name()).build().await?;
    let client = Client::builder()
        .server_name(&ServerName::parse("meowl-matrix.speed.micrsky.com").unwrap())
        .build().await?;
    client.matrix_auth().login_username(alice, "official_bot").send().await?;

    Ok(client)
}

// pub async fn entry(ob: Arc<OneBot<TracingHandler<Event, Action, Resp>, ImplOBC<Event>>>) {
//     let eh = EventHandler::init(ob);
//     let client = create_client().await.expect("登录失败");
// }