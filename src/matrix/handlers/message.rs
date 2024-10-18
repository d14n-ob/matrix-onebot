use matrix_sdk::{Room, RoomMemberships};
use matrix_sdk::ruma::events::room::message::{MessageType, SyncRoomMessageEvent};
use walle_core::event::Event;
use crate::config::{CONFIG, LANG};
use crate::matrix::handlers::EventHandler;
use crate::onebot::event_build;
use crate::sql::DATABASE;
use crate::sql::table::{matrix_events, matrix_messages, TableCommonOpera};

impl EventHandler {
    pub async fn message(
        &self,
        ev: SyncRoomMessageEvent,
        room: Room,
    ) {
        let matrix_events_table = DATABASE.get_matrix_events_table();
        let matrix_messages_table = DATABASE.get_matrix_messages_table();
        let insert_failed_msg = (&LANG.read().unwrap().error_database_table_insert_failed).to_owned();
        let query_failed_msg = (&LANG.read().unwrap().error_database_table_query_failed).to_owned();
        // todo: 重构 sqlx 异步数据库
        if query_event_is_in_db(&ev.event_id().to_string(), &matrix_events_table, query_failed_msg) {
            // 如果数据库中存在此消息, 说明是被同步的 历史消息, 不处理
            return;
        }
        save_message_in_db(&ev, &matrix_events_table, &matrix_messages_table, insert_failed_msg);

        println!("Message Received: {:?}", ev);
        println!("Members: {}", room.members(RoomMemberships::JOIN).await.unwrap().len());

        // todo: 也许不应该我拦截 - 可配置拦截
        // 如果是自己的信息, 不处理
        if ev.sender().to_string().eq(&CONFIG.read().unwrap().full_user_id) { return; }

        match room.members(RoomMemberships::JOIN).await.unwrap().len() {
            0 => {}
            1 => { todo!("什么面壁者") }
            2 => {
                // Private
                let event_message_private = event_build::message::private(ev);
                self.ob.handle_event(Event::from(event_message_private)).await.unwrap();
            }
            _ => {
                // Group
                let event_message_group = event_build::message::group(ev, room);
                self.ob.handle_event(Event::from(event_message_group)).await.unwrap();
            }
        }
    }
}

fn query_event_is_in_db(
    event_id: &str,
    matrix_events_table: &matrix_events::Table,
    query_failed_msg: String,
) -> bool {
    if let Some(_) = matrix_events_table.query(event_id)
        .expect(&query_failed_msg.replace("{table}", matrix_events::TABLE_NAME)) {
        true
    } else { false }
}

fn save_message_in_db(
    ev: &SyncRoomMessageEvent,
    matrix_events_table: &matrix_events::Table,
    matrix_messages_table: &matrix_messages::Table,
    insert_failed_msg: String,
) {
    // 入事件库
    let event_id = ev.event_id().to_string();
    let event_timestamp = ev.origin_server_ts().get()
        .to_string().parse::<i64>().unwrap_or(0);
    matrix_events_table.insert_or_update(matrix_events::Event {
        event_id: event_id.clone(),
        ty: "message".to_owned(),
        timestamp: event_timestamp,
    }).expect(
        &insert_failed_msg.replace("{table}", matrix_events::TABLE_NAME),
    );

    // 入消息库
    let msg_content = &ev.as_original().unwrap().content;
    let message: matrix_messages::Message = match &msg_content.msgtype {
        MessageType::Audio(_) => matrix_messages::Message {
            event_id,
            ty: "Audio".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::Emote(_) => matrix_messages::Message {
            event_id,
            ty: "Emote".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::File(_) => matrix_messages::Message {
            event_id,
            ty: "File".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::Image(_) => matrix_messages::Message {
            event_id,
            ty: "Image".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::Location(_) => matrix_messages::Message {
            event_id,
            ty: "Location".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::Notice(_) => matrix_messages::Message {
            event_id,
            ty: "Notice".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::ServerNotice(_) => matrix_messages::Message {
            event_id,
            ty: "ServerNotice".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::Text(c) => matrix_messages::Message {
            event_id,
            ty: "Text".to_owned(),
            body: c.body.to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::Video(_) => matrix_messages::Message {
            event_id,
            ty: "Video".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::VerificationRequest(_) => matrix_messages::Message {
            event_id,
            ty: "VerificationRequest".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        MessageType::_Custom(_) => matrix_messages::Message {
            event_id,
            ty: "_Custom".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        },
        _ => matrix_messages::Message {
            event_id,
            ty: "_Unknown".to_owned(),
            body: "".to_owned(),
            sender: ev.sender().to_string(),
            timestamp: event_timestamp,
        }
    };
    matrix_messages_table.insert_or_update(message).expect(
        &insert_failed_msg.replace("{table}", matrix_messages::TABLE_NAME),
    );
}