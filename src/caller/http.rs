use chrono::Utc;
use log::debug;
use reqwest::{header::CONTENT_TYPE, Body, Method, StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    caller::client::Client,
    consts::{
        Status, APP_ID, JSON_CONTENT_TYPE, JS_LOGIN, LOGIN, REGEX_STATUS_CODE, REGEX_SYNC_CHECK,
        REGEX_UUID, STATUS_CODE_SCANNED, STATUS_CODE_SUCCESS, STATUS_CODE_TIMEOUT,
        STATUS_CODE_WAIT, SYNC_CHECK, UOS_PATCH_CLIENT_VERSION, UOS_PATCH_EXTSPAM,
        WEB_WX_NEW_LOGIN_PAGE, WEB_WX_STATUS_NOTIFY, WEB_WX_SYNC,
    },
    errors::Error,
    resp::{
        BaseResponse, LoginInfo, ResponseCheckLogin, ResponseSyncCheck, ResponseSyncMessage,
        ResponseWebInit, SyncKey,
    },
    storage::{BaseRequest, WechatDomain},
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

/// 通知微信状态
pub async fn web_wx_status_notify(
    client: &Client,
    base_req: &BaseRequest,
    user_name: &str,
    login_info: &LoginInfo,
) -> Result<(), Error> {
    debug!("web_wx_status_notify");
    let path = format!(
        "{}{}",
        client.get_domain().unwrap().base_host(),
        WEB_WX_STATUS_NOTIFY,
    );
    let mut notify_url = Url::parse(&path)
        .map_err(|e| Error::StatusNotify(format!("解析url: {path} 失败:\n {e}")))?;
    notify_url
        .query_pairs_mut()
        .append_pair("lang", "zh_CN")
        .append_pair("pass_ticket", &login_info.pass_ticket);

    let content = serde_json::json!({
        "BaseRequest": base_req,
        "ClientMsgId": Utc::now().timestamp(),
        "Code":        3,
        "FromUserName": user_name,
        "ToUserName": user_name,
    });

    let mut req = reqwest::Request::new(Method::POST, notify_url);
    *req.body_mut() = Some(Body::from(serde_json::to_vec(&content).unwrap()));
    req.headers_mut().append(CONTENT_TYPE, JSON_CONTENT_TYPE);

    let text = client
        .execute(req)
        .await
        .map_err(|e| Error::StatusNotify(format!("请求url: {path} 失败:\n {e}")))?
        .text()
        .await
        .map_err(|e| Error::StatusNotify(format!("解析web_wx_status_notify数据失败: {e}")))?;

    let resp: ResponseWebWxStatusNotify = serde_json::from_str(&text).unwrap();
    dbg!(&resp);
    if !resp.base_response.is_ok() {
        return Err(Error::StatusNotify(format!(
            "web_wx_status_notify失败: {}",
            resp.base_response.errmsg
        )));
    }
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct ResponseWebWxStatusNotify {
    #[serde(rename = "BaseResponse")]
    base_response: BaseResponse,
    #[serde(rename = "MsgID")]
    msg_id: String,
}

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

pub async fn sync_check(
    client: &Client,
    device_id: &str,
    web_init_resp: &ResponseWebInit,
    login_info: &LoginInfo,
) -> Result<ResponseSyncCheck, Error> {
    debug!("sync_check");
    let path = format!("{}{}", client.get_domain().unwrap().base_host(), SYNC_CHECK);

    let mut synccheck_url = Url::parse(&path)
        .map_err(|e| Error::SyncCheck(format!("解析sync check url: {path} 失败:\n {e}")))?;

    let timestamp = Utc::now().timestamp();
    let sync_key = web_init_resp
        .sync_key
        .list
        .iter()
        .map(|kv| format!("{kv}"))
        .collect::<Vec<_>>()
        .join("|");

    synccheck_url
        .query_pairs_mut()
        .append_pair("r", &format!("{timestamp}"))
        .append_pair("skey", &login_info.skey)
        .append_pair("sid", &login_info.wxsid)
        .append_pair("uin", &format!("{}", login_info.wxuin))
        .append_pair("deviceid", device_id)
        .append_pair("_", &format!("{timestamp}"))
        .append_pair("synckey", &sync_key);

    let req = reqwest::Request::new(Method::GET, synccheck_url);
    let resp_text = client
        .execute(req)
        .await
        .map_err(|e| Error::SyncCheck(format!("请求url: {path} 失败:\n {e}")))?
        .text()
        .await
        .map_err(|e| Error::SyncCheck(format!("解析sync check数据失败: {e}")))?;

    debug!("resp_text:{}", resp_text);

    let results = REGEX_SYNC_CHECK
        .captures(&resp_text)
        .ok_or(Error::SyncCheck(format!(
            "从响应数据{resp_text}中解析sync check数据失败"
        )))?;

    if results.len() != 3 {
        return Err(Error::SyncCheck(format!(
            "从响应数据{resp_text}中解析window.synccheck数据失败"
        )));
    }

    let value = serde_json::json!({
        "retcode": results.get(1).unwrap().as_str(),
        "selector": results.get(2).unwrap().as_str(),
    });

    let resp: ResponseSyncCheck = serde_json::from_value(value)
        .map_err(|e| Error::SyncCheck(format!("组装ResponseSyncCheck数据错误: {e}")))?;

    dbg!("sync_check", &resp);

    Ok(resp)
}

pub async fn sync_message(
    client: &Client,
    base_req: &BaseRequest,
    sync_key: &SyncKey,
    login_info: &LoginInfo,
) -> Result<ResponseSyncMessage, Error> {
    debug!("sync_message");

    let path = format!(
        "{}{}",
        client.get_domain().unwrap().base_host(),
        WEB_WX_SYNC
    );

    let mut sync_url =
        Url::parse(&path).map_err(|e| Error::Sync(format!("解析sync url: {path} 失败:\n {e}")))?;

    sync_url
        .query_pairs_mut()
        .append_pair("sid", &login_info.wxsid)
        .append_pair("skey", &login_info.skey)
        .append_pair("pass_ticket", &login_info.pass_ticket);

    let mut req = reqwest::Request::new(Method::POST, sync_url);
    req.headers_mut().append(CONTENT_TYPE, JSON_CONTENT_TYPE);

    let content = serde_json::json!({
        "BaseRequest": base_req,
        "SyncKey": sync_key,
        "rr": Utc::now().timestamp(),
    });

    *req.body_mut() = Some(Body::from(serde_json::to_vec(&content).unwrap()));

    let resp: ResponseSyncMessage = client.execute(req).await?.json().await?;

    dbg!(&resp);

    Ok(resp)
}
