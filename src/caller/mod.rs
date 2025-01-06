use std::collections::HashMap;

use client::Client;
pub use http::LoginInfo;
pub use http::Mode;
use log::debug;
use reqwest_cookie_store::CookieStore;

use crate::resp::ResponseCheckLogin;
use crate::resp::ResponseWebInit;
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

    pub fn set_domain(&mut self, domain: Option<WechatDomain>) {
        self.client.set_domain(domain);
    }

    pub fn get_domain(&self) -> Option<WechatDomain> {
        self.client.get_domain().clone()
    }

    pub async fn web_init(&self, base_req: &BaseRequest) -> Result<ResponseWebInit, Error> {
        debug!("caller::web_init");
        self.client.web_init(base_req).await
    }

    pub async fn web_wx_status_notify(
        &self,
        base_req: &BaseRequest,
        user_name: &str,
        login_info: &LoginInfo,
    ) -> Result<(), Error> {
        debug!("caller::web_wx_status_notify");
        self.client
            .web_wx_status_notify(base_req, user_name, login_info)
            .await
    }
}
