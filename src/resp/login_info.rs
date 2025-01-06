use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LoginInfo {
    pub ret: i32,
    pub wxuin: i64,
    #[serde(rename = "isgrayscale")]
    pub is_gray_scale: i32,
    pub message: String,
    pub skey: String,
    pub wxsid: String,
    pub pass_ticket: String,
}

impl LoginInfo {
    pub fn ok(&self) -> bool {
        self.ret == 0
    }

    pub fn error(&self) -> Option<String> {
        if self.ok() {
            return None;
        }

        Some(self.message.clone())
    }
}
