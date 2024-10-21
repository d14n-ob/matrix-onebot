use matrix_sdk::RoomMemberships;
use matrix_sdk::ruma::{UserId};
use walle_core::action::GetUserInfo;
use walle_core::structs::UserInfo;
use crate::error::{friend_not_exist, matrix_client_error, unsupported_param};
use crate::onebot::actions::handler::RespResult;
use crate::onebot::MatrixHandler;

impl MatrixHandler {
    pub async fn get_self_info(&self) -> RespResult<UserInfo> {
        let client = self.get_client()?;
        let full_user_id = client.user_id().unwrap();
        let full_user_id_string = full_user_id.to_string();
        let user_id = full_user_id_string.to_string()
            .split("@").collect::<Vec<&str>>()[1]
            .split(":").collect::<Vec<&str>>()[0]
            .to_string();
        let user_display_name = client.get_profile(full_user_id).await
            // 查自己不存在应该算是客户端错误吧
            .map_err(|e| matrix_client_error(e))?.displayname;
        Ok(UserInfo {
            user_id: full_user_id_string,
            user_name: user_id,
            user_displayname: user_display_name.unwrap_or(String::new()),
            user_remark: Default::default(),
        })
    }

    pub async fn get_user_info(&self, info: GetUserInfo) -> RespResult<UserInfo> {
        let client = self.get_client()?;
        let user_id = info.user_id
            .split("@").collect::<Vec<&str>>()[1]
            .split(":").collect::<Vec<&str>>()[0]
            .to_string();
        let full_user_id = UserId::parse(&info.user_id)
            // todo: 下次也许可以自定义一个 user_id 拼写错误
            .map_err(|e| unsupported_param(e))?;
        let user_display_name = client.get_profile(&full_user_id).await
            // 应该是 用户 而非 好友 不存在
            .map_err(|e| friend_not_exist(e))?.displayname;
        Ok(UserInfo {
            user_id: info.user_id,
            user_name: user_id,
            user_displayname: user_display_name.unwrap_or(String::new()),
            user_remark: Default::default(),
        })
    }

    pub async fn get_friend_list(&self) -> RespResult<Vec<UserInfo>> {
        let rooms = self.get_client()?.joined_rooms();

        let self_full_user_id = self.get_client()?.user_id().unwrap();
        let mut friend_list: Vec<UserInfo> = Vec::new();

        // todo: 排序 + 二分筛选 2 人房间
        for room in rooms {
            if let Ok(is_direct) = room.is_direct().await { if !is_direct { continue } }
            // 宽松处理, is_direct Err 的也视为好友
            // todo: unwrap -> except
            let members = room.members(RoomMemberships::JOIN).await.unwrap();
            if members.len() != 2 { continue }
            for member in members {
                if member.user_id() == self_full_user_id { continue }
                let user_display_name = member.display_name();
                let full_user_id = member.user_id().to_string();
                let user_id = full_user_id
                    .split("@").collect::<Vec<&str>>()[1]
                    .split(":").collect::<Vec<&str>>()[0]
                    .to_string();
                friend_list.push(UserInfo {
                    user_id: full_user_id,
                    user_name: user_id,
                    user_displayname: user_display_name.unwrap_or("").to_owned(),
                    user_remark: "".to_owned(),
                })
            }
        }

        Ok(friend_list)
    }
}