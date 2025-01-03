use std::collections::HashMap;

use client::Client;
pub use http::Mode;
pub use http::{LoginInfo, ResponseCheckLogin};
use log::debug;
use reqwest_cookie_store::CookieStore;

use crate::{
    errors::Error,
    storage::{BaseRequest, WechatDomain},
};
pub mod client;
mod http;

#[derive(Default)]
pub struct Caller {
    client: Client,
    // path: Option<Url>,
}

impl Caller {
    // pub fn new(client: Client) -> Self {
    //     Self { client }
    // }

    // pub fn set_path(&mut self, path: Option<Url>) {
    //     self.path = path;
    // }

    pub fn set_mod(&mut self, mode: Mode) {
        self.client.set_mode(mode);
    }

    /// 获取登录的uuid
    pub async fn get_login_uuid(&self) -> Result<String, Error> {
        self.client.get_login_uuid().await
    }

    /// 检查是否登录成功
    pub async fn check_login(&self, uuid: &str) -> Result<ResponseCheckLogin, Error> {
        self.client.check_login(uuid).await
    }

    /// 获取登录信息
    pub async fn get_login_info(&mut self, url: &str) -> Result<LoginInfo, Error> {
        debug!("caller::get_login_info {}", url);
        self.client.get_login_info(url).await
    }

    pub async fn add_cookies(&mut self, (url, cookie): (String, CookieStore)) {
        self.client.add_cookies((url, cookie)).await
    }

    pub async fn get_coookies(&self) -> HashMap<String, CookieStore> {
        self.client.get_coookies().await
    }

    pub fn get_domain(&self) -> Option<WechatDomain> {
        self.client.get_domain().clone()
    }

    pub async fn web_init(&self, base_req: &BaseRequest) -> Result<(), Error> {
        self.client.web_init(base_req).await
    }
}
