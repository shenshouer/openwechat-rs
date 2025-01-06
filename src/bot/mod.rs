use std::{io::Read, io::Write};

use log::{debug, warn};
use rand::Rng;

use crate::{
    caller::{Caller, Mode},
    consts::{Status, REGEX_REDIRECT_URI},
    errors::Error,
    message::{default_message_handler, MessageHandler},
    resp::{ResponseCheckLogin, ResponseWebInit},
    storage::{
        BaseRequest, HotReloadStorageItem, JSONFileHostReloadStorage, Storage, StorageItemFetcher,
    },
    sync_check::ResponseSyncCheck,
};

pub struct Bot<T: Read + Write + StorageItemFetcher> {
    /// 定义回调函数类型
    scan_callback: Option<fn(body: ResponseCheckLogin)>,
    /// 登陆回调
    login_callback: Option<fn(body: ResponseCheckLogin)>,
    /// 登出回调
    logout_callback: Option<fn(bot: Bot<T>)>,
    /// 获取UUID的回调
    uuid_callback: Option<fn(uuid: &str)>,
    /// 心跳回调
    sync_check_callback: Option<fn(body: ResponseSyncCheck)>,
    /// 获取消息成功的handle
    message_handler: MessageHandler,
    // /// 获取消息发生错误的handle, 返回err == nil 则尝试继续监听
    // MessageErrorHandler: MessageErrorHandler,
    uuid: String,
    device_id: String,
    caller: Caller,
    storage: Storage,
    hot_reload_storage: T,
}

impl<T: Read + Write + StorageItemFetcher> Bot<T> {
    pub async fn hot_login(&mut self) -> Result<(), Error> {
        match self.hot_reload_storage.fetch() {
            Err(e) => {
                warn!("hot reload storage error: {e}");
                return self.login().await;
            }
            Ok(items) => self.hot_login_init(items).await,
        }

        if let Err(e) = self.web_init().await {
            warn!("web init error: {e} try login");
            return self.login().await;
        }

        Ok(())
    }

    pub async fn login(&mut self) -> Result<(), Error> {
        let uuid = self.caller.get_login_uuid().await?;
        self.login_with_uuid(uuid.as_str()).await
    }

    pub async fn hot_login_init(&mut self, items: HotReloadStorageItem) {
        debug!("bot::hot_login_init");
        for cookie in items.cookies.into_iter() {
            self.caller.add_cookies(cookie).await
        }

        self.storage.login_info = items.login_info;
        self.storage.request = items.base_request;
        self.uuid = items.uuid.unwrap();
        self.caller.set_domain(items.wechat_domain);
    }

    /// 使用uuid登录
    pub async fn login_with_uuid(&mut self, uuid: &str) -> Result<(), Error> {
        self.uuid = uuid.to_string();
        if let Some(callback) = &self.uuid_callback {
            callback(uuid);
        }

        loop {
            let resp = self.caller.check_login(uuid).await?;
            match resp.status {
                Status::Success => {
                    debug!("登录成功 {}", resp.raw);
                    // 判断是否有登录回调，如果有执行它
                    let data =
                        REGEX_REDIRECT_URI
                            .captures(&resp.raw)
                            .ok_or(Error::GetLoginInfo(format!(
                                "从响应数据{}中解析redirect url数据失败",
                                resp.raw
                            )))?;
                    if data.len() != 2 {
                        return Err(Error::GetLoginInfo("没有匹配到 redirect url".to_owned()));
                    }
                    let redirect_uri = data.get(1).unwrap().as_str();
                    self.handle_login(redirect_uri).await?;

                    if let Some(login_callback) = self.login_callback.as_ref() {
                        login_callback(resp);
                    }

                    return Ok(());
                }
                Status::Scanned => {
                    // 此时 resp.raw 为用户图像数据
                    debug!("请在手机上确认登录");
                    if let Some(scan_callback) = self.scan_callback.as_ref() {
                        scan_callback(resp);
                    }
                }
                Status::Timeout => {
                    return Err(Error::LoginTimeout);
                }
                Status::Wait => {
                    debug!("等待扫码");
                    continue;
                }
                Status::Unknown(msg) => return Err(Error::StatusUnknown(msg)),
            }
        }
    }

    async fn handle_login(&mut self, redirect_uri: &str) -> Result<(), Error> {
        debug!("bot::handle_login {}", redirect_uri);
        let info = self.caller.get_login_info(redirect_uri).await?;

        if self.device_id.is_empty() {
            self.device_id = get_random_device_id();
        }
        let base_req = BaseRequest {
            uin: info.wxuin,
            sid: info.wxsid.clone(),
            skey: info.skey.clone(),
            device_id: self.device_id.clone(),
        };
        self.storage.request = Some(base_req.clone());

        self.storage.login_info = Some(info);

        self.dump_hot_reload_storage().await?;

        let resp = self.web_init().await?;
        dbg!(&resp);
        self.storage.web_init_reponse = Some(resp);
        Ok(())
    }

    async fn dump_hot_reload_storage(&mut self) -> Result<(), Error> {
        let cookies = self.caller.get_coookies().await;
        let item = HotReloadStorageItem {
            cookies,
            base_request: self.storage.request.clone(),
            login_info: self.storage.login_info.clone(),
            wechat_domain: self.caller.get_domain(),
            uuid: Some(self.uuid.clone()),
        };

        serde_json::to_writer(&mut self.hot_reload_storage, &item)
            .map_err(Error::DumpHotReloadStorage)
    }

    pub async fn web_init(&self) -> Result<ResponseWebInit, Error> {
        debug!("bot::web_init");
        match self.storage.request.as_ref() {
            None => Err(Error::NoBaseRequest),
            Some(base_req) => self.caller.web_init(base_req).await,
        }
    }

    pub fn set_uuid_callback(&mut self, uuid_callback: fn(uuid: &str)) {
        self.uuid_callback = Some(uuid_callback);
    }

    pub fn set_hot_reload_storage(&mut self, hot_reload_storage: T) {
        self.hot_reload_storage = hot_reload_storage;
    }

    pub fn set_scan_callback(&mut self, scan_callback: fn(body: ResponseCheckLogin)) {
        self.scan_callback = Some(scan_callback);
    }

    pub fn set_login_callback(&mut self, login_callback: fn(body: ResponseCheckLogin)) {
        self.login_callback = Some(login_callback);
    }

    pub fn set_logout_callback(&mut self, logout_callback: fn(bot: Bot<T>)) {
        self.logout_callback = Some(logout_callback);
    }

    pub fn set_sync_check_callback(&mut self, sync_check_callback: fn(body: ResponseSyncCheck)) {
        self.sync_check_callback = Some(sync_check_callback);
    }

    pub fn set_message_handler(&mut self, message_handler: MessageHandler) {
        self.message_handler = message_handler;
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.caller.set_mod(mode);
    }
}

fn get_random_device_id() -> String {
    use core::fmt::Write;
    let mut rng = rand::thread_rng(); // 创建随机数生成器
    let mut device_id = String::with_capacity(16); // 预分配 16 字节的空间

    device_id.push('e'); // 在字符串开头添加字符 'e'

    for _ in 0..15 {
        let r: u8 = rng.gen_range(0..9); // 生成 0 到 8 之间的随机数
        write!(device_id, "{}", r).unwrap(); // 将随机数写入字符串
    }

    device_id // 返回生成的设备 ID
}

impl Default for Bot<JSONFileHostReloadStorage> {
    fn default() -> Self {
        Self {
            scan_callback: Default::default(),
            login_callback: Default::default(),
            logout_callback: Default::default(),
            uuid_callback: Default::default(),
            sync_check_callback: Default::default(),
            message_handler: default_message_handler,
            uuid: Default::default(),
            device_id: Default::default(),
            caller: Default::default(),
            storage: Default::default(),
            hot_reload_storage: Default::default(),
        }
    }
}
