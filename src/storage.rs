use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use reqwest_cookie_store::CookieStore;
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

use crate::{caller::LoginInfo, resp::ResponseWebInit, Error};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Storage {
    pub login_info: Option<LoginInfo>,
    pub request: Option<BaseRequest>,
    pub web_init_reponse: Option<ResponseWebInit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseRequest {
    #[serde(rename = "Uin")]
    pub uin: i64,
    #[serde(rename = "Sid")]
    pub sid: String,
    #[serde(rename = "Skey")]
    pub skey: String,
    #[serde(rename = "DeviceID")]
    pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotReloadStorageItem {
    #[serde(serialize_with = "ser_cookies", deserialize_with = "de_cookies")]
    pub cookies: HashMap<String, CookieStore>,
    pub base_request: Option<BaseRequest>,
    pub login_info: Option<LoginInfo>,
    pub wechat_domain: Option<WechatDomain>,
    pub uuid: Option<String>,
}

fn de_cookies<'de, D>(deserializer: D) -> Result<HashMap<String, CookieStore>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    let mut cookies = HashMap::new();

    for (key, value) in map {
        let buffer = value.into_bytes();
        let cursor = std::io::Cursor::new(buffer);
        let cookie_store = CookieStore::load(cursor, |s| serde_json::from_str(s))
            .map_err(|e| serde::de::Error::custom(e.to_string()))?;
        cookies.insert(key, cookie_store);
    }

    Ok(cookies)
}

pub fn ser_cookies<S>(
    cookies: &HashMap<String, CookieStore>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(cookies.len()))?;
    for (key, v) in cookies.iter() {
        dbg!(key, v);
        let mut buffer = Vec::new();
        // 此处需要保存过期和非持久化的cookie
        v.save_incl_expired_and_nonpersistent(&mut buffer, serde_json::to_string)
            .map_err(|e| serde::ser::Error::custom(e.to_string()))?;
        map.serialize_entry(key, &String::from_utf8_lossy(&buffer))?;
    }
    map.end()
}

pub struct JSONFileHostReloadStorage {
    filename: String,
    file: Arc<RwLock<Option<std::fs::File>>>,
}

impl Default for JSONFileHostReloadStorage {
    fn default() -> Self {
        Self {
            filename: "storage.json".to_string(),
            file: Arc::new(RwLock::new(None)),
        }
    }
}

impl JSONFileHostReloadStorage {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            file: Arc::new(RwLock::new(None)),
        }
    }
}

impl std::io::Read for JSONFileHostReloadStorage {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut file = self.file.write().unwrap();
        if file.is_none() {
            *file = Some(std::fs::File::open(&self.filename)?);
        }
        file.as_mut().unwrap().read(buf)
    }
}

impl std::io::Write for JSONFileHostReloadStorage {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut file = self.file.write().unwrap();
        if file.is_none() {
            *file = Some(std::fs::File::create(&self.filename)?);
        }
        file.as_mut().unwrap().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        let mut file = self.file.write().unwrap();
        if file.is_none() {
            *file = Some(std::fs::File::create(&self.filename)?);
        }
        file.as_mut().unwrap().flush()
    }
}

pub trait StorageItemFetcher {
    fn fetch(&mut self) -> Result<HotReloadStorageItem, Error>;
}

impl StorageItemFetcher for JSONFileHostReloadStorage {
    fn fetch(&mut self) -> Result<HotReloadStorageItem, Error> {
        serde_json::from_reader(self).map_err(|e| Error::FetchStorage(e.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatDomain(String);

impl std::fmt::Display for WechatDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for WechatDomain {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl WechatDomain {
    pub fn new(domain: String) -> Self {
        Self(domain)
    }
    pub fn base_host(&self) -> String {
        format!("https://{}", self.0)
    }

    pub fn file_host(&self) -> String {
        format!("https://file.{}", self.0)
    }

    pub fn sync_host(&self) -> String {
        format!("https://webpush.{}", self.0)
    }
}
