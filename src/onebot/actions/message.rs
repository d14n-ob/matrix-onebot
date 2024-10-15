use matrix_sdk::ruma::{room_id, user_id, RoomId, UserId};
use walle_core::action::SendMessage;
use walle_core::structs::SendMessageResp;
use walle_core::resp::{resp_error, RespError};
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use walle_core::util::Value;
use crate::onebot::actions::handler::{MatrixHandler, RespResult};
use crate::error;

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
                    let Value::Str(simple_msg) = simple_msg else { todo!() };;
                    let content = RoomMessageEventContent::text_plain(simple_msg);
                    let resp_eid = room.send(content).await
                        .map_err(|e| error::matrix_client_error(e))?.event_id;

                    Ok(
                        SendMessageResp {
                            message_id: resp_eid.to_string(),
                            // 等我维护上数据库再提供
                            time: 0f64,
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

                let room = self.get_client()?.create_dm(&target_user_id).await;
                if let Ok(room) = room {
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
                            // 等我维护上数据库再提供
                            time: 0f64,
                        }
                    )
                } else {
                    Err(error::group_not_exist("无法从给定ID创建/进入私聊房间"))
                }

            }
            ty => { Err(resp_error::unsupported_param(ty)) }
        }
    }
}