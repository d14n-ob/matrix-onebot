use matrix_sdk::Room;
use matrix_sdk::ruma::events::room::MediaSource::Plain;
use matrix_sdk::ruma::events::room::message::{MessageType, RoomMessageEventContent, SyncRoomMessageEvent};
use walle_core::prelude::*;
use walle_core::event::{Group, GroupMessageEvent, Message, Private, PrivateMessageEvent};
use crate::config::CONFIG;
use crate::onebot::matrix::{get_self, get_time};

pub fn private(ev: SyncRoomMessageEvent) -> PrivateMessageEvent {
    let eid: String = ev.event_id().into();

    BaseEvent {
        id: eid.clone(),
        time: get_time(ev.origin_server_ts()),
        implt: (),
        platform: (),
        ty: get_message(eid, ev),
        detail_type: Private,
        sub_type: (),
        extra: Default::default()
    }
}

pub fn group(ev: SyncRoomMessageEvent, room: Room) -> GroupMessageEvent {
    let eid: String = ev.event_id().into();

    BaseEvent {
        id: eid.clone(),
        time: get_time(ev.origin_server_ts()),
        implt: (),
        platform: (),
        ty: get_message(eid, ev),
        detail_type: Group {
            group_id: room.room_id().into(),
        },
        sub_type: (),
        extra: Default::default(),
    }
}



fn get_message(eid: String, ev: SyncRoomMessageEvent) -> Message {
    let msg_content = &ev.as_original().unwrap().content;
    let msg_type = &msg_content.msgtype;

    let debug_struct_message = get_struct_debug_msg(msg_content);
    let alt_message = get_alt_msg(msg_type);

    let message = if CONFIG.read().unwrap().onebot.is_message_forward_struct {
        debug_struct_message.clone()
    } else { alt_message.clone() };
    let alt_message = if CONFIG.read().unwrap().onebot.is_alt_message_forward_struct {
        debug_struct_message
    } else { alt_message };

    Message {
        selft: get_self(),
        message_id: eid.clone(),
        message: Segments::from([MsgSegment::from(message)]),
        alt_message,
        user_id: ev.sender().into(),
    }
}

fn get_struct_debug_msg(msg_content: &RoomMessageEventContent) -> String {
    let msg_type = &msg_content.msgtype;

    match msg_type {
        // MessageType::Audio(_) => { format!("{:?}", msg_content) }
        // MessageType::Emote(_) => { format!("{:?}", msg_content) }
        // MessageType::File(_) => { format!("{:?}", msg_content) }
        // MessageType::Image(content) => { format!("{:?}", msg_content) }
        // MessageType::Location(_) => { format!("{:?}", msg_content) }
        // MessageType::Notice(_) => { format!("{:?}", msg_content) }
        // MessageType::ServerNotice(_) => { format!("{:?}", msg_content) }
        // MessageType::Text(content) => { format!("{:?}", msg_content) }
        // MessageType::Video(_) => { format!("{:?}", msg_content) }
        // MessageType::VerificationRequest(_) => { format!("{:?}", msg_content) }
        // MessageType::_Custom(_) => { format!("{:?}", msg_content) }
        _ => {
            format!("{:?}", msg_content)
        }
    }
}

fn get_alt_msg(msg_type: &MessageType) -> String {
    match msg_type {
        MessageType::Audio(_) => { String::from("暂未实现") }
        MessageType::Emote(_) => { String::from("暂未实现") }
        MessageType::File(_) => { String::from("暂未实现") }
        MessageType::Image(content) => {
            if let Plain(uri) = &content.source {
                format!("[Image: {}]", uri.to_string())
            } else {
                "[Image: Encrypted | 暂未实现]".to_owned()
            }
        }
        MessageType::Location(_) => { String::from("暂未实现") }
        MessageType::Notice(_) => { String::from("暂未实现") }
        MessageType::ServerNotice(_) => { String::from("暂未实现") }
        MessageType::Text(content) => { content.body.to_owned() }
        MessageType::Video(_) => { String::from("暂未实现") }
        MessageType::VerificationRequest(_) => { String::from("暂未实现") }
        MessageType::_Custom(_) => { String::from("暂未实现") }
        _ => { String::from("暂未实现") }
    }
}