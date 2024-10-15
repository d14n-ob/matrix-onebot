use matrix_sdk::{Client, Room};
use matrix_sdk::ruma::events::room::member::StrippedRoomMemberEvent;
use crate::matrix::unsafe_actions::unsafe_action_join_all_room;
use crate::matrix::handlers::EventHandler;

impl EventHandler {
    pub async fn invite(&self, room_member: StrippedRoomMemberEvent, room: Room, client: Client) {
        if room_member.state_key == client.user_id().unwrap() {
            tokio::spawn(unsafe_action_join_all_room(room));
        }
    }
}