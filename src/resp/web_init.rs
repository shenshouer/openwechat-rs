use serde::{Deserialize, Serialize};

use super::{user::User, BaseResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseWebInit {
    #[serde(rename = "BaseResponse")]
    pub base_response: BaseResponse,
    #[serde(rename = "Count")]
    pub count: i32,
    #[serde(rename = "ContactList")]
    pub contact_list: Vec<serde_json::Value>,
    #[serde(rename = "SyncKey")]
    pub sync_key: SyncKey,
    #[serde(rename = "User")]
    pub user: User,
    #[serde(rename = "ChatSet")]
    pub chat_set: String,
    #[serde(rename = "SKey")]
    pub skey: String,
    #[serde(rename = "ClientVersion")]
    pub client_version: i64,
    #[serde(rename = "SystemTime")]
    pub system_time: i64,
    #[serde(rename = "GrayScale")]
    pub gray_scale: i64,
    #[serde(rename = "InviteStartCount")]
    pub invite_start_count: i64,
    #[serde(rename = "MPSubscribeMsgCount")]
    pub mp_subscribe_msg_count: i64,
    #[serde(rename = "MPSubscribeMsgList")]
    pub mpsubscribe_msg_list: Vec<serde_json::Value>,
    #[serde(rename = "ClickReportInterval")]
    pub click_report_interval: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncKey {
    #[serde(rename = "Count")]
    pub count: i32,
    #[serde(rename = "List")]
    pub list: Vec<KVPair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KVPair {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Val")]
    pub val: String,
}
