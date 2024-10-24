use matrix_sdk::{Room};
use matrix_sdk::ruma::events::room::member::SyncRoomMemberEvent;
use walle_core::event::{BaseEvent, FriendDecrease, FriendDecreaseEvent, FriendIncrease, FriendIncreaseEvent, GroupMemberDecrease, GroupMemberDecreaseEvent, GroupMemberIncrease, GroupMemberIncreaseEvent, Notice};
use crate::onebot::matrix::{get_self, get_time};

pub fn group_increase(ev: SyncRoomMemberEvent, room: Room) -> GroupMemberIncreaseEvent {
    // 以后再自己支持 sub_type 吧
    BaseEvent {
        id: ev.event_id().to_string(),
        time: get_time(ev.origin_server_ts()),
        implt: (),
        platform: (), 
        ty: Notice {
            selft: get_self(),
        },
        detail_type: GroupMemberIncrease {
            group_id: room.room_id().to_string(),
            user_id: ev.state_key().to_string(),
            operator_id: ev.sender().to_string(),
        },
        sub_type: (),
        extra: Default::default(),
    }
}

pub fn group_decrease(ev: SyncRoomMemberEvent, room: Room) -> GroupMemberDecreaseEvent {
    // 以后再自己支持 sub_type 吧
    BaseEvent {
        id: ev.event_id().to_string(),
        time: get_time(ev.origin_server_ts()),
        implt: (),
        platform: (),
        ty: Notice {
            selft: get_self(),
        },
        detail_type: GroupMemberDecrease {
            group_id: room.room_id().to_string(),
            user_id: ev.state_key().to_string(),
            operator_id: ev.sender().to_string(),
        },
        sub_type: (),
        extra: Default::default(),
    }
}

pub fn private_increase(ev: SyncRoomMemberEvent, friend: String) -> FriendIncreaseEvent {
    BaseEvent {
        id: ev.event_id().to_string(),
        time: get_time(ev.origin_server_ts()),
        implt: (),
        platform: (),
        ty: Notice {
            selft: get_self(),
        },
        detail_type: FriendIncrease {
            // 暂不确定好友是 sender 还是 state_key
            user_id: friend,
        },
        sub_type: (),
        extra: Default::default(),
    }
}

pub fn private_decrease(ev: SyncRoomMemberEvent) -> FriendDecreaseEvent {
    BaseEvent {
        id: ev.event_id().to_string(),
        time: get_time(ev.origin_server_ts()),
        implt: (),
        platform: (),
        ty: Notice {
            selft: get_self(),
        },
        detail_type: FriendDecrease {
            user_id: ev.sender().to_string(),
        },
        sub_type: (),
        extra: Default::default(),
    }
}