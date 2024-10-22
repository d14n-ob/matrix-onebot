use matrix_sdk::Room;
use matrix_sdk::ruma::events::room::member::{MembershipState, SyncRoomMemberEvent};
use walle_core::event::Event;
use crate::matrix::EventHandler;
use crate::onebot::event_build;

impl EventHandler {
    pub async fn member(&self, ev: SyncRoomMemberEvent, room: Room) {
        let is_direct =
            if let Ok(is_direct) = room.is_direct().await {
                is_direct
            } else { false };
        if is_direct {
            // Private
            match ev.membership() {
                MembershipState::Ban => {}
                MembershipState::Invite => {

                }
                MembershipState::Join => {}
                MembershipState::Knock => {}
                MembershipState::Leave => {}
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