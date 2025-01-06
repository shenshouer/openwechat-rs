use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    // #[serde(rename = "IsOwner")]
    // pub is_owner: i32,
    // #[serde(rename = "MemberCount")]
    // pub member_count: i32,
    // #[serde(rename = "MemberList")]
    // pub chat_room_id: i32,
    // #[serde(rename = "UniFriend")]
    // pub uni_friend: i32,
    // #[serde(rename = "OwnerUin")]
    // pub owner_uin: i32,
    // #[serde(rename = "ChatRoomName")]
    // pub statues: i32,
    // #[serde(rename = "ChatRoomOwnerUin")]
    // pub attr_status: i64,
    // #[serde(rename = "Province")]
    // pub province: String,
    // #[serde(rename = "City")]
    // pub city: String,
    // #[serde(rename = "Alias")]
    // pub alias: String,
    // #[serde(rename = "DisplayName")]
    // pub display_name: String,
    // #[serde(rename = "KeyWord")]
    // pub key_word: String,
    // #[serde(rename = "EncryChatRoomId")]
    // pub encry_chat_room_id: String,
    #[serde(rename = "Uin")]
    pub uin: i64,
    #[serde(rename = "UserName")]
    pub user_name: String,
    #[serde(rename = "NickName")]
    pub nick_name: String,
    #[serde(rename = "HeadImgUrl")]
    pub head_img_url: String,
    #[serde(rename = "RemarkName")]
    pub remark_name: String,
    #[serde(rename = "PYInitial")]
    pub py_initial: String,
    #[serde(rename = "PYQuanPin")]
    pub py_quan_pin: String,
    #[serde(rename = "RemarkPYInitial")]
    pub remark_pyinitial: String,
    #[serde(rename = "RemarkPYQuanPin")]
    pub remark_pyquan_pin: String,
    #[serde(rename = "HideInputBarFlag")]
    pub hide_input_bar_flag: i32,
    #[serde(rename = "StarFriend")]
    pub star_friend: i32,
    #[serde(rename = "Sex")]
    pub sex: i32,
    #[serde(rename = "Signature")]
    pub signature: String,
    #[serde(rename = "AppAccountFlag")]
    pub app_account_flag: i32,
    #[serde(rename = "VerifyFlag")]
    pub verify_flag: i32,
    #[serde(rename = "ContactFlag")]
    pub contact_flag: i32,
    #[serde(rename = "WebWxPluginSwitch")]
    pub web_wx_plugin_switch: i32,
    #[serde(rename = "HeadImgFlag")]
    pub head_img_flag: i32,
    #[serde(rename = "SnsFlag")]
    pub sns_flag: i32,
    // MemberList Members
}
