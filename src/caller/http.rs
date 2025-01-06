use chrono::Utc;
use log::debug;
use reqwest::{header::HeaderValue, Method, StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    caller::client::Client,
    consts::{
        Status, APP_ID, JS_LOGIN, LOGIN, REGEX_STATUS_CODE, REGEX_UUID, STATUS_CODE_SCANNED,
        STATUS_CODE_SUCCESS, STATUS_CODE_TIMEOUT, STATUS_CODE_WAIT, WEB_WX_NEW_LOGIN_PAGE,
    },
    errors::Error,
    resp::ResponseCheckLogin,
    storage::WechatDomain,
};

/// normal 网页版模式
const MODE_NORMAL: &str = "normal";
/// desktop 桌面模式，uos electron套壳
const MODE_DESKTOP: &str = "desktop";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Desktop,
}

impl Mode {
    pub fn as_str(&self) -> &str {
        match self {
            Mode::Normal => MODE_NORMAL,
            Mode::Desktop => MODE_DESKTOP,
        }
    }
}

pub async fn get_login_uuid(client: &Client) -> Result<String, Error> {
    let mut redirect_url = Url::parse(WEB_WX_NEW_LOGIN_PAGE).map_err(|e| {
        Error::GetLoginUuid(format!("解析url: {WEB_WX_NEW_LOGIN_PAGE} 失败:\n {e}"))
    })?;
    if client.mode == Mode::Desktop {
        redirect_url
            .query_pairs_mut()
            .append_pair("mod", client.mode.as_str());
    }

    let mut login_url = Url::parse(JS_LOGIN)
        .map_err(|e| Error::GetLoginUuid(format!("解析url: {JS_LOGIN} 失败:\n {e}")))?;
    login_url
        .query_pairs_mut()
        .append_pair("redirect_uri", redirect_url.as_str())
        .append_pair("appid", APP_ID)
        .append_pair("fun", "new")
        .append_pair("lang", "zh_CN")
        .append_pair("_", &format!("{}", Utc::now().timestamp_millis()));
    if client.mode == Mode::Desktop {
        login_url.query_pairs_mut().append_pair("mod", MODE_DESKTOP);
    }

    let req = reqwest::Request::new(Method::GET, login_url);
    let resp = client
        .execute(req)
        .await
        .map_err(|e| Error::GetLoginUuid(format!("请求url: {JS_LOGIN} 失败:\n {e}")))?
        .text()
        .await
        .map_err(|e| {
            Error::GetLoginUuid(format!("解析请求url: {JS_LOGIN} 的响应数据失败:\n {e}"))
        })?;

    let uuid = REGEX_UUID
        .captures(&resp)
        .ok_or(Error::GetLoginUuid(format!(
            "从响应数据{resp}中解析UUID数据失败"
        )))?
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
    Ok(uuid)
}

/// 检查登录状态
pub async fn check_login(client: &Client, uuid: &str) -> Result<ResponseCheckLogin, Error> {
    let mut login_url = Url::parse(LOGIN)
        .map_err(|e| Error::GetLoginUuid(format!("解析url: {LOGIN} 失败:\n {e}")))?;

    let now_timestamp = Utc::now().timestamp_millis();
    login_url
        .query_pairs_mut()
        .append_pair("loginicon", "true")
        .append_pair("uuid", uuid)
        .append_pair("tip", "0")
        .append_pair("r", &format!("{}", now_timestamp / 1579))
        .append_pair("_", &format!("{}", now_timestamp));

    debug!("check_login req {}", login_url.as_str());

    let req = reqwest::Request::new(Method::GET, login_url);
    let resp = client
        .execute(req)
        .await
        .map_err(|e| Error::GetLoginUuid(format!("请求url: {LOGIN} 失败:\n {e}")))?
        .text()
        .await
        .map_err(|e| Error::GetLoginUuid(format!("解析请求url: {LOGIN} 的响应数据失败:\n {e}")))?;

    let status_code = REGEX_STATUS_CODE
        .captures(&resp)
        .ok_or(Error::GetLoginUuid(format!(
            "从响应数据{resp}中解析status code数据失败"
        )))?
        .get(1)
        .unwrap()
        .as_str();

    let status = match status_code {
        STATUS_CODE_SUCCESS => Status::Success,
        STATUS_CODE_SCANNED => Status::Scanned,
        STATUS_CODE_TIMEOUT => Status::Timeout,
        STATUS_CODE_WAIT => Status::Wait,
        _ => Status::Unknown(status_code.to_string()),
    };
    Ok(ResponseCheckLogin { status, raw: resp })
}

// /// 通知微信状态
// fn web_wx_status_notify() {}

/// 获取登录信息
pub async fn get_login_info(client: &mut Client, url: &str) -> Result<LoginInfo, Error> {
    let u = Url::parse(url)
        .map_err(|e| Error::GetLoginInfo(format!("解析redirect uri: {url} 失败:\n {e}")))?;

    client.set_domain(
        u.domain()
            .map(|domain| WechatDomain::new(domain.to_string())),
    );

    let mut req = reqwest::Request::new(Method::GET, u);

    match client.mode {
        Mode::Desktop => {
            let headers = req.headers_mut();
            headers.append("client-version", UOS_PATCH_CLIENT_VERSION);
            headers.append("extspam", UOS_PATCH_EXTSPAM);
        }
        Mode::Normal => {}
    }
    let resp = client.execute(req).await?;

    debug!("get_login_info response header: {:?} ", resp.headers());
    // 判断是否重定向
    if resp.status() != StatusCode::MOVED_PERMANENTLY {
        debug!(
            "get_login_info response text: {} ",
            resp.text().await.unwrap()
        );
        return Err(Error::GetLoginInfo(format!(
            "{}: try to login with Desktop Mode",
            Error::Forbidden,
        )));
    }

    let text = resp
        .text()
        .await
        .map_err(|e| Error::GetLoginInfo(format!("解析响应失败:\n {e}")))?;

    debug!("LoginInfo xml data: {}", text);
    serde_xml_rs::from_str(&text).map_err(|e| Error::GetLoginInfo(format!("解析响应失败:\n {e}")))
}

const UOS_PATCH_CLIENT_VERSION: HeaderValue = HeaderValue::from_static("2.0.0");
const UOS_PATCH_EXTSPAM: HeaderValue = HeaderValue::from_static(
    concat!(
        "Go8FCIkFEokFCggwMDAwMDAwMRAGGvAESySibk50w5Wb3uTl2c2h64jVVrV7gNs06GFlWplHQbY/5FfiO++1yH4ykC",
        "yNPWKXmco+wfQzK5R98D3so7rJ5LmGFvBLjGceleySrc3SOf2Pc1gVehzJgODeS0lDL3/I/0S2SSE98YgKleq6Uqx6ndTy9yaL9qFxJL7eiA/R",
        "3SEfTaW1SBoSITIu+EEkXff+Pv8NHOk7N57rcGk1w0ZzRrQDkXTOXFN2iHYIzAAZPIOY45Lsh+A4slpgnDiaOvRtlQYCt97nmPLuTipOJ8Qc5p",
        "M7ZsOsAPPrCQL7nK0I7aPrFDF0q4ziUUKettzW8MrAaiVfmbD1/VkmLNVqqZVvBCtRblXb5FHmtS8FxnqCzYP4WFvz3T0TcrOqwLX1M/DQvcHa",
        "GGw0B0y4bZMs7lVScGBFxMj3vbFi2SRKbKhaitxHfYHAOAa0X7/MSS0RNAjdwoyGHeOepXOKY+h3iHeqCvgOH6LOifdHf/1aaZNwSkGotYnYSc",
        "W8Yx63LnSwba7+hESrtPa/huRmB9KWvMCKbDThL/nne14hnL277EDCSocPu3rOSYjuB9gKSOdVmWsj9Dxb/iZIe+S6AiG29Esm+/eUacSba0k8",
        "wn5HhHg9d4tIcixrxveflc8vi2/wNQGVFNsGO6tB5WF0xf/plngOvQ1/ivGV/C1Qpdhzznh0ExAVJ6dwzNg7qIEBaw+BzTJTUuRcPk92Sn6QDn",
        "2Pu3mpONaEumacjW4w6ipPnPw+g2TfywJjeEcpSZaP4Q3YV5HG8D6UjWA4GSkBKculWpdCMadx0usMomsSS/74QgpYqcPkmamB4nVv1JxczYIT",
        "IqItIKjD35IGKAUwAA=="
    )
);

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
