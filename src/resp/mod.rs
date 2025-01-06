pub use check_login::ResponseCheckLogin;
pub use login_info::LoginInfo;
pub use sync_check::ResponseSyncCheck;
pub use web_init::ResponseWebInit;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

mod check_login;
mod login_info;
mod sync_check;
mod user;
mod web_init;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum Ret {
    Ok = 0,
    /// ticket error
    Ticket = -14,
    /// logic error
    Logic = -2,
    /// sys error
    System = -1,
    /// param error
    Param = 1,
    /// failed login warn
    FailedLoginWarn = 1100,
    /// failed login check
    FailedLoginCheck = 1101,
    /// cookie invalid
    CookieInvalid = 1102,
    /// login environmental abnormality
    LoginEnvAbnormality = 1203,
    /// operate too often
    OperateTooOften = 1205,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResponse {
    #[serde(rename = "Ret")]
    pub ret: Ret,
    #[serde(rename = "ErrMsg")]
    pub errmsg: String,
}

impl BaseResponse {
    pub fn is_ok(&self) -> bool {
        self.ret == Ret::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_response() {
        let json = r#"{"Ret":0,"ErrMsg":"ok"}"#;
        let resp: BaseResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.ret, Ret::Ok);
        assert_eq!(resp.errmsg, "ok");
    }
}
