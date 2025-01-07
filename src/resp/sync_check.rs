use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::EnumString;

use crate::Error;

#[derive(strum::Display, EnumString, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selector {
    /// 正常
    #[strum(serialize = "0")]
    Normal,
    /// 新消息
    #[strum(serialize = "2")]
    NewMessage,
    /// 联系人信息变更
    #[strum(serialize = "4")]
    ModContact,
    /// 添加或删除联系人
    #[strum(serialize = "6")]
    AddOrDelContact,
    /// 进入或退出聊天室
    #[strum(serialize = "7")]
    ModChatroom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSyncCheck {
    #[serde(rename = "retcode")]
    pub ret_code: String,
    #[serde(
        rename = "selector",
        serialize_with = "ser_selector",
        deserialize_with = "de_selector"
    )]
    pub selector: Selector,
}

impl ResponseSyncCheck {
    fn is_success(&self) -> bool {
        self.ret_code == "0"
    }

    pub fn error(self) -> Result<Self, Error> {
        if !self.is_success() {
            return Err(Error::SyncCheck(self.ret_code));
        }

        Ok(self)
    }

    pub fn is_normal(&self) -> bool {
        self.is_success() && self.selector == Selector::Normal
    }

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

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_parse_sync_check() {
        let json_str = r#"{retcode:"0",selector:"7"}"#;
        let re = Regex::new(r#"(?P<key>\w+):"#).unwrap();
        let corrected_json = re.replace_all(json_str, r#""$key":"#);
        println!("corrected_json:{}", corrected_json);
        let resp: ResponseSyncCheck = serde_json::from_str(&corrected_json).unwrap();
        dbg!(resp);
    }
}
