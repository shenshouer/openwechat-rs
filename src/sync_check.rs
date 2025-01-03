use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(strum::Display, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selector {
    /// 正常
    #[strum(to_string = "0", serialize = "0")]
    Normal,
    /// 新消息
    #[strum(to_string = "2", serialize = "2")]
    NewMessage,
    /// 联系人信息变更
    #[strum(to_string = "4", serialize = "4")]
    ModContact,
    /// 添加或删除联系人
    #[strum(to_string = "6", serialize = "6")]
    AddOrDelContact,
    /// 进入或退出聊天室
    #[strum(to_string = "7", serialize = "7")]
    ModChatroom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSyncCheck {
    #[serde(rename = "RetCode")]
    pub ret_code: String,
    #[serde(
        rename = "Selector",
        serialize_with = "ser_selector",
        deserialize_with = "de_selector"
    )]
    pub selector: Selector,
}

impl ResponseSyncCheck {
    pub fn is_success(&self) -> bool {
        self.ret_code == "0"
    }

    // pub fn is_normal(&self) -> bool {
    //     self.is_normal() && self.selector == Selector::Normal
    // }

    // pub fn has_new_message(&self) -> bool {
    //     self.is_normal() && self.selector == Selector::NewMessage
    // }
}

fn de_selector<'de, D>(deserializer: D) -> Result<Selector, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "0" => Ok(Selector::Normal),
        "2" => Ok(Selector::NewMessage),
        "4" => Ok(Selector::ModContact),
        "6" => Ok(Selector::AddOrDelContact),
        "7" => Ok(Selector::ModChatroom),
        _ => Err(serde::de::Error::unknown_variant(
            &s,
            &["0", "1", "2", "3", "4"],
        )),
    }
}

pub fn ser_selector<S>(selector: &Selector, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = selector.to_string();
    serializer.serialize_str(&s)
}
