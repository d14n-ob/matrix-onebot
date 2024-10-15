use matrix_sdk::{Client, Room, RoomMemberships};
use matrix_sdk::ruma::events::room::message::SyncRoomMessageEvent;
use walle_core::event::Event;
use crate::matrix::handlers::EventHandler;
use crate::onebot::event_build;

impl EventHandler {
    pub async fn message(&self, ev: SyncRoomMessageEvent, room: Room) {
        println!("Message Received: {:?}", ev);
        println!("Members: {}", room.members(RoomMemberships::JOIN).await.unwrap().len());

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