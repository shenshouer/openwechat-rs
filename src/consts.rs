use lazy_static::lazy_static;
use reqwest::header::HeaderValue;

pub(crate) const WEB_WX_INIT: &str = "/cgi-bin/mmwebwx-bin/webwxinit";
pub(crate) const WEB_WX_STATUS_NOTIFY: &str = "/cgi-bin/mmwebwx-bin/webwxstatusnotify";
// pub(crate) const WEB_WX_SYNC: &str = "/cgi-bin/mmwebwx-bin/webwxsync";
// pub(crate) const WEB_WX_SENDMSG: &str = "/cgi-bin/mmwebwx-bin/webwxsendmsg";
// pub(crate) const WEB_WX_GET_CONTACT: &str = "/cgi-bin/mmwebwx-bin/webwxgetcontact";
// pub(crate) const WEB_WX_SEND_MSG_IMG: &str = "/cgi-bin/mmwebwx-bin/webwxsendmsgimg";
// pub(crate) const WEB_WX_SEND_APP_MSG: &str = "/cgi-bin/mmwebwx-bin/webwxsendappmsg";
// pub(crate) const WEB_WX_SEND_VIDEO_MSG: &str = "/cgi-bin/mmwebwx-bin/webwxsendvideomsg";
// pub(crate) const WEB_WX_BATCH_GET_CONTACT: &str = "/cgi-bin/mmwebwx-bin/webwxbatchgetcontact";
// pub(crate) const WEB_WX_OP_LOG: &str = "/cgi-bin/mmwebwx-bin/webwxoplog";
// pub(crate) const WEB_WX_VERIFY_USER: &str = "/cgi-bin/mmwebwx-bin/webwxverifyuser";
// pub(crate) const SYNC_CHECK: &str = "/cgi-bin/mmwebwx-bin/synccheck";
// pub(crate) const WEB_WX_UPLOA_DMEDIA: &str = "/cgi-bin/mmwebwx-bin/webwxuploadmedia";
// pub(crate) const WEB_WX_GET_MSG_IMG: &str = "/cgi-bin/mmwebwx-bin/webwxgetmsgimg";
// pub(crate) const WEB_WX_GET_VOICE: &str = "/cgi-bin/mmwebwx-bin/webwxgetvoice";
// pub(crate) const WEB_WX_GET_VIDEO: &str = "/cgi-bin/mmwebwx-bin/webwxgetvideo";
// pub(crate) const WEB_WX_LOGOUT: &str = "/cgi-bin/mmwebwx-bin/webwxlogout";
// pub(crate) const WEB_WX_GET_MEDIA: &str = "/cgi-bin/mmwebwx-bin/webwxgetmedia";
// pub(crate) const WEB_WX_UPDATE_CHATROOM: &str = "/cgi-bin/mmwebwx-bin/webwxupdatechatroom";
// pub(crate) const WEB_WX_REVOKE_MSG: &str = "/cgi-bin/mmwebwx-bin/webwxrevokemsg";
// pub(crate) const WEB_WX_CHECK_UPLOAD: &str = "/cgi-bin/mmwebwx-bin/webwxcheckupload";
// pub(crate) const WEB_WX_PUSH_LOGIN_URL: &str = "/cgi-bin/mmwebwx-bin/webwxpushloginurl";
// pub(crate) const WEB_WX_GET_ICON: &str = "/cgi-bin/mmwebwx-bin/webwxgeticon";
// pub(crate) const WEB_WX_CREATE_CHATROOM: &str = "/cgi-bin/mmwebwx-bin/webwxcreatechatroom";
pub(crate) const WEB_WX_NEW_LOGIN_PAGE: &str =
    "https://wx.qq.com/cgi-bin/mmwebwx-bin/webwxnewloginpage";
pub(crate) const JS_LOGIN: &str = "https://login.wx.qq.com/jslogin";
pub(crate) const LOGIN: &str = "https://login.wx.qq.com/cgi-bin/mmwebwx-bin/login";
pub(crate) const QRCODE: &str = "https://login.weixin.qq.com/qrcode/";

pub(crate) const APP_ID: &str = "wx782c26e4c19acffb";

lazy_static! {
    pub static ref REGEX_UUID: regex::Regex = regex::Regex::new(r#"uuid = "(.*?)";"#).unwrap();
    pub static ref REGEX_STATUS_CODE: regex::Regex =
        regex::Regex::new(r#"window.code=(\d+);"#).unwrap();
    pub static ref REGEX_REDIRECT_URI: regex::Regex =
        regex::Regex::new(r#"window.redirect_uri="(.*?)""#).unwrap();
}

pub(crate) const STATUS_CODE_SUCCESS: &str = "200";
pub(crate) const STATUS_CODE_SCANNED: &str = "201";
pub(crate) const STATUS_CODE_TIMEOUT: &str = "400";
pub(crate) const STATUS_CODE_WAIT: &str = "408";

#[derive(Debug)]
pub(crate) enum Status {
    Success,
    Scanned,
    Timeout,
    Wait,
    Unknown(String),
}

pub(crate) const JSON_CONTENT_TYPE: HeaderValue =
    HeaderValue::from_static("application/json; charset=utf-8");
