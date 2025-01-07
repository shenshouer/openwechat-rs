use serde::{Deserialize, Serialize};

use super::{BaseResponse, SyncKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseSyncMessage {
    #[serde(rename = "BaseResponse")]
    pub base_response: BaseResponse,
    #[serde(rename = "AddMsgCount")]
    pub add_msg_count: usize,
    #[serde(rename = "AddMsgList")]
    pub add_msg_list: Vec<serde_json::Value>,
    #[serde(rename = "ModContactCount")]
    pub mod_contact_count: usize,
    #[serde(rename = "ModContactList")]
    pub mod_contact_list: Vec<serde_json::Value>,
    #[serde(rename = "DelContactCount")]
    pub del_contact_count: usize,
    #[serde(rename = "DelContactList")]
    pub del_contact_list: Vec<serde_json::Value>,
    #[serde(rename = "ModChatRoomMemberCount")]
    pub mod_chat_room_member_count: usize,
    #[serde(rename = "ModChatRoomMemberList")]
    pub mod_chat_room_member_list: Vec<serde_json::Value>,
    #[serde(rename = "Profile")]
    pub profile: Profile,
    #[serde(rename = "ContinueFlag")]
    pub continue_flag: i64,
    #[serde(rename = "SyncKey")]
    pub sync_key: SyncKey,
    #[serde(rename = "SKey")]
    pub s_key: String,
    #[serde(rename = "SyncCheckKey")]
    pub sync_check_key: SyncKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "BitFlag")]
    pub bit_flag: i64,
    #[serde(rename = "UserName")]
    pub user_name: BuffData,
    #[serde(rename = "NickName")]
    pub nick_name: BuffData,
    #[serde(rename = "BindUin")]
    pub bind_uin: i64,
    #[serde(rename = "BindEmail")]
    pub bind_email: BuffData,
    #[serde(rename = "BindMobile")]
    pub bind_mobile: BuffData,
    #[serde(rename = "Status")]
    pub status: i64,
    #[serde(rename = "Sex")]
    pub sex: i64,
    #[serde(rename = "PersonalCard")]
    pub personal_card: i64,
    #[serde(rename = "Alias")]
    pub alias: String,
    #[serde(rename = "HeadImgUpdateFlag")]
    pub head_img_update_flag: i64,
    #[serde(rename = "HeadImgUrl")]
    pub head_img_url: String,
    #[serde(rename = "Signature")]
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffData {
    #[serde(rename = "Buff")]
    pub buff: String,
}
