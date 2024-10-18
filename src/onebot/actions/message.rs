use matrix_sdk::{Room, RoomMemberships};
use matrix_sdk::ruma::{UserId};
use walle_core::action::SendMessage;
use walle_core::structs::SendMessageResp;
use walle_core::resp::{resp_error,};
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use walle_core::util::Value;
use crate::config::{CONFIG, LANG};
use crate::onebot::actions::handler::{MatrixHandler, RespResult};
use crate::error;
use crate::sql::DATABASE;
use crate::sql::table::{TableCommonOpera, matrix_events};

impl MatrixHandler {
    pub async fn send_message(&self , c: SendMessage) -> RespResult<SendMessageResp> {
        println!("Recv SendMessageAction: {:?}", c);
        match c.detail_type.as_str() {
            "group" => {
                let target_group_id = c.group_id.ok_or_else(|| error::bad_param("group_id is null"))?;
                let joined_rooms = self.get_client()?.joined_rooms();
                let room = joined_rooms.iter()
                    .find(|r| {
                        r.room_id().to_string().eq(&target_group_id)
                    });
                if let Some(room) = room {
                    // todo: 简单处理, 仅作文本信息, 日后处理富文本与MIME (消息段分析器)
                    let simple_msg = c.message.get(0)
                        .ok_or_else(|| error::bad_param("message is null: MsgSegment"))?
                        .data.clone().get("text")
                        .ok_or_else(|| error::bad_param("message is null: text"))?
                        .to_owned();
                    let Value::Str(simple_msg) = simple_msg else { todo!() };
                    let content = RoomMessageEventContent::text_plain(simple_msg);
                    let resp_eid = room.send(content).await
                        .map_err(|e| error::matrix_client_error(e))?.event_id;

                    Ok(
                        SendMessageResp {
                            message_id: resp_eid.to_string(),
                            time: get_time(&resp_eid.to_string()).await,
                        }
                    )
                } else {
                    Err(error::group_not_exist(""))
                }
            }
            "group_temp" => {
                todo!()
            }
            "private" => {
                let target_user_id = c.user_id.ok_or_else(|| error::bad_param("user_id is null"))?;
                let target_user_id = UserId::parse(&target_user_id)
                    .map_err(|_| error::bad_param("user_id cannot format"))?;

                // 查找私聊房间
                let joined_rooms = self.get_client()?.joined_rooms();
                let mut room: Option<&Room> = None;
                for r in joined_rooms.iter() {
                    if let Ok(members) = r.members(RoomMemberships::JOIN).await {
                        if members.len() != 2 { continue }
                        if members.iter().find(|m| m.user_id() == target_user_id).is_some() {
                            room = Some(r)
                        }
                    } else { continue }
                }
                let room: Room =
                    if let Some(room) = room { room.to_owned() }
                    else {
                        if let Ok(room) = self.get_client()?.create_dm(&target_user_id).await {
                            room
                        } else {
                            return Err(error::group_not_exist("无法从给定ID创建/进入私聊房间"))
                        }
                    };

                // todo: 简单处理, 仅作文本信息, 日后处理富文本与MIME (消息段分析器)
                let simple_msg = c.message.get(0)
                    .ok_or_else(|| error::bad_param("message is null: MsgSegment"))?
                    .data.clone().get("text")
                    .ok_or_else(|| error::bad_param("message is null: text"))?
                    .to_owned();
                let Value::Str(simple_msg) = simple_msg else { todo!() };
                let content = RoomMessageEventContent::text_plain(simple_msg);
                let resp_eid = room.send(content).await
                    .map_err(|e| error::matrix_client_error(e))?.event_id;

                Ok(
                    SendMessageResp {
                        message_id: resp_eid.to_string(),
                        time: get_time(&resp_eid.to_string()).await,
                    }
                )

            }
            ty => { Err(resp_error::unsupported_param(ty)) }
        }
    }
}

async fn get_time(eid: &str) -> f64 {
    let matrix_event_table = DATABASE.get_matrix_events_table();

    let interval = CONFIG.read().unwrap().onebot.query_self_event_interval_secs;
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval));

    let event = loop {
        let event = matrix_event_table.query(eid)
            .expect(&LANG.read().unwrap().error_database_table_query_failed.replace("{table}", matrix_events::TABLE_NAME));

        if let Some(e) = event {
            break e;
        } else {
            interval.tick().await;
        }
    };

    event.timestamp as f64 / 1000f64
}