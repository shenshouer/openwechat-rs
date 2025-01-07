use std::{collections::HashMap, time::Duration};

use log::{debug, warn};
use reqwest::{
    header::{CONTENT_TYPE, SET_COOKIE},
    redirect::Policy,
    Body, Method, Request, Response,
};
use reqwest_cookie_store::CookieStore;
use tokio::sync::Mutex;
use url::Url;

use crate::{
    caller::http::sync_check,
    consts::{JSON_CONTENT_TYPE, WEB_WX_INIT},
    errors::Error,
    resp::{
        LoginInfo, ResponseCheckLogin, ResponseSyncCheck, ResponseSyncMessage, ResponseWebInit,
        SyncKey,
    },
    storage::{BaseRequest, WechatDomain},
};

use super::http::{
    check_login, get_login_info, get_login_uuid, sync_message, web_wx_status_notify, Mode,
};

pub struct Client {
    client: reqwest::Client,
    hooks: Option<Vec<Box<dyn HttpHook>>>,
    domain: Option<WechatDomain>,
    cookies: Mutex<HashMap<String, CookieStore>>,
    pub mode: Mode,
}

impl Default for Client {
    fn default() -> Self {
        let mut c = Self::new(Mode::Normal);
        // c.hooks = Some(vec![Box::new(UserAgentHook)]);
        c.add_http_hook(vec![Box::new(UserAgentHook)]);
        c
    }
}

impl Client {
    pub fn new(mode: Mode) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .redirect(Policy::none()) // 默认会自动重定向
                .build()
                .unwrap(),
            hooks: None,
            domain: None,
            mode,
            cookies: Mutex::new(HashMap::new()),
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_domain(&mut self, domain: Option<WechatDomain>) {
        self.domain = domain;
    }

    pub fn get_domain(&self) -> Option<WechatDomain> {
        self.domain.clone()
    }
    pub fn add_http_hook(&mut self, hooks: Vec<Box<dyn HttpHook>>) {
        if let Some(h) = &mut self.hooks {
            h.extend(hooks);
        } else {
            self.hooks = Some(hooks);
        }
    }

    async fn do_http(&self, mut req: Request) -> Result<Response, Error> {
        if let Some(hooks) = &self.hooks {
            for hook in hooks {
                hook.before_request(&mut req);
            }
        }

        let mut resp = None;
        let mut err = None;
        for i in 0..MAX_RETRY {
            let req = req.try_clone().ok_or(Error::RequestClone)?;
            match self.client.execute(req).await {
                Ok(r) => {
                    resp = Some(r);
                    break;
                }
                Err(e) => {
                    err = Some(e);
                    warn!("try times: {i} error: {}", err.as_ref().unwrap());
                }
            }
        }

        if let Some(e) = err {
            Err(e.into())
        } else {
            let resp = resp.unwrap();
            if let Some(hooks) = &self.hooks {
                for hook in hooks {
                    hook.after_request(&resp);
                }
            }
            Ok(resp)
        }
    }

    pub async fn parse_cookies(&self, resp: &Response) {
        let resp_cookies = resp
            .headers()
            .get_all(SET_COOKIE)
            .iter()
            .map(|hv| {
                let hv = hv.to_owned();
                let xs = String::from_utf8(hv.as_bytes().to_vec())
                    .map(|s| {
                        let xx = cookie::Cookie::split_parse(s)
                            .filter_map(|r| r.ok()) // TODO:
                            .collect::<Vec<cookie::Cookie<'static>>>();
                        xx
                    })
                    .unwrap(); // TODO:
                xs
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let mut store = CookieStore::new(None);
        store.store_response_cookies(resp_cookies.into_iter(), resp.url());

        let mut cookies = self.cookies.lock().await;
        let path = format!(
            "{}://{}{}",
            resp.url().scheme(),
            resp.url().host_str().unwrap(),
            resp.url().path()
        );
        cookies.insert(path, store);
    }

    pub async fn add_cookies(&mut self, (url, cookie): (String, CookieStore)) {
        self.cookies.lock().await.insert(url, cookie);
    }

    pub async fn execute(&self, req: Request) -> Result<Response, Error> {
        let resp = self.do_http(req).await?;
        self.parse_cookies(&resp).await;
        Ok(resp)
    }

    pub async fn get_coookies(&self) -> HashMap<String, CookieStore> {
        let cookies = self.cookies.lock().await;
        cookies.clone()
    }

    /// 获取登录uuid
    pub async fn get_login_uuid(&self) -> Result<String, Error> {
        get_login_uuid(self).await
    }

    pub async fn check_login(&self, uuid: &str) -> Result<ResponseCheckLogin, Error> {
        check_login(self, uuid).await
    }

    pub async fn get_login_info(&mut self, url: &str) -> Result<LoginInfo, Error> {
        debug!("client::get_login_info {}", url);
        get_login_info(self, url).await
    }

    /// 请求获取初始化信息
    pub async fn web_init(&self, base_req: &BaseRequest) -> Result<ResponseWebInit, Error> {
        debug!("client::web_init");
        let init_url_str = format!(
            "{}{}",
            self.domain
                .as_ref()
                .ok_or(Error::WebInit("no domain".to_string()))?
                .base_host(),
            WEB_WX_INIT
        );
        let mut init_url = Url::parse(&init_url_str)
            .map_err(|e| Error::WebInit(format!("解析初始化url: {init_url_str} 失败: {e}")))?;
        init_url
            .query_pairs_mut()
            .append_pair("_", &chrono::Utc::now().timestamp().to_string());

        let mut req = reqwest::Request::new(Method::POST, init_url);
        *req.body_mut() = Some(Body::from(serde_json::to_vec(base_req).unwrap()));
        req.headers_mut().append(CONTENT_TYPE, JSON_CONTENT_TYPE);

        let res: ResponseWebInit = self
            .execute(req)
            .await?
            .json()
            .await
            .map_err(|e| Error::WebInit(format!("解析web init数据失败: {e}")))?;

        Ok(res)
    }

    pub async fn web_wx_status_notify(
        &self,
        base_req: &BaseRequest,
        user_name: &str,
        login_info: &LoginInfo,
    ) -> Result<(), Error> {
        debug!("client::web_wx_status_notify");
        web_wx_status_notify(self, base_req, user_name, login_info).await
    }

    pub async fn sync_check(
        &self,
        device_id: &str,
        web_init_resp: &ResponseWebInit,
        login_info: &LoginInfo,
    ) -> Result<ResponseSyncCheck, Error> {
        debug!("client::sync_check");
        sync_check(self, device_id, web_init_resp, login_info).await
    }

    pub async fn sync_message(
        &self,
        base_req: &BaseRequest,
        sync_key: &SyncKey,
        login_info: &LoginInfo,
    ) -> Result<ResponseSyncMessage, Error> {
        sync_message(self, base_req, sync_key, login_info).await
    }
}

const MAX_RETRY: u8 = 3;

pub trait HttpHook {
    fn before_request(&self, req: &mut Request);
    fn after_request(&self, resp: &Response);
}

pub struct UserAgentHook;

impl HttpHook for UserAgentHook {
    fn before_request(&self, req: &mut Request) {
        req.headers_mut()
            .insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36".parse().unwrap());
    }

    fn after_request(&self, _resp: &Response) {}
}
