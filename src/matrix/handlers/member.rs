use matrix_sdk::{Room, RoomMemberships};
use matrix_sdk::ruma::events::room::member::{MembershipState, SyncRoomMemberEvent};
use walle_core::event::Event;
use crate::config::LANG;
use crate::matrix::EventHandler;
use crate::onebot::event_build;
use crate::sql::DATABASE;
use crate::sql::table::{matrix_events, TableCommonOpera};

impl EventHandler {
    pub async fn member(&self, ev: SyncRoomMemberEvent, room: Room) {
        let matrix_events_table = DATABASE.get_matrix_events_table();
        let insert_failed_msg = (&LANG.read().unwrap().error_database_table_insert_failed).to_owned();
        let query_failed_msg = (&LANG.read().unwrap().error_database_table_query_failed).to_owned();
        if query_event_is_in_db(&ev.event_id().to_string(), &matrix_events_table, query_failed_msg) {
            // 如果数据库中存在此消息, 说明是被同步的 历史消息, 不处理
            return;
        }
        save_message_in_db(&ev, &room, &matrix_events_table, insert_failed_msg);

        let is_direct =
            if let Ok(is_direct) = room.is_direct().await {
                is_direct
            } else { false };
        if is_direct {
            // Private
            match ev.membership() {
                MembershipState::Ban => {}
                MembershipState::Invite => {}
                MembershipState::Join => {
                    let friend = room.members(RoomMemberships::JOIN).await.unwrap()
                        // 应该自身同意请求后是 sender
                        .iter().find(|r| !r.user_id().eq(ev.sender()))
                        .unwrap().user_id().to_string();
                    let event_friend_increase = event_build::member::private_increase(ev, friend);
                    self.ob.handle_event(Event::from(event_friend_increase)).await.unwrap();
                }
                MembershipState::Knock => {
                    let event_friend_decrease = event_build::member::private_decrease(ev);
                    self.ob.handle_event(Event::from(event_friend_decrease)).await.unwrap();
                }
                MembershipState::Leave => {
                    let event_friend_decrease = event_build::member::private_decrease(ev);
                    self.ob.handle_event(Event::from(event_friend_decrease)).await.unwrap();
                }
                MembershipState::_Custom(_) => {}
                _ => {}
            }
        } else {
            // Group
            match ev.membership() {
                // 空事件 等有 sub_type 了再说
                MembershipState::Ban => {}
                MembershipState::Invite => {}
                MembershipState::Join => {
                    let event_member_increase_group = event_build::member::group_increase(ev, room);
                    self.ob.handle_event(Event::from(event_member_increase_group)).await.unwrap()
                }
                MembershipState::Knock => {
                    let event_member_decrease_group = event_build::member::group_decrease(ev, room);
                    self.ob.handle_event(Event::from(event_member_decrease_group)).await.unwrap()
                }
                MembershipState::Leave => {
                    let event_member_decrease_group = event_build::member::group_decrease(ev, room);
                    self.ob.handle_event(Event::from(event_member_decrease_group)).await.unwrap()
                }
                MembershipState::_Custom(_) => {}
                _ => {}
            }
        }

    }
}

// 摆烂了, 现在的太耦合了
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
    ev: &SyncRoomMemberEvent,
    room: &Room,
    matrix_events_table: &matrix_events::Table,
    insert_failed_msg: String,
) {
    // 入事件库
    let event_id = ev.event_id().to_string();
    let event_timestamp = ev.origin_server_ts().get()
        .to_string().parse::<i64>().unwrap_or(0);

    matrix_events_table.insert_or_update(matrix_events::Event {
        event_id: event_id.clone(),
        ty: "member".to_owned(),
        room_id: room.room_id().to_string(),
        timestamp: event_timestamp,
    }).expect(
        &insert_failed_msg.replace("{table}", matrix_events::TABLE_NAME),
    );
}