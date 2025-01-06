use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("GetLoginUuid error: {0}")]
    GetLoginUuid(String),
    #[error("Unknown status error: {0}")]
    StatusUnknown(String),
    #[error("Login timeout")]
    LoginTimeout,
    #[error("GetLoginInfo error: {0}")]
    GetLoginInfo(String),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Request clone failed")]
    RequestClone,
    #[error("Cookie parse error: {0}")]
    CookieParseError(#[from] cookie::ParseError),
    #[error("Login forbidden")]
    Forbidden,
    #[error("DumpHotReloadStorage error: {0}")]
    DumpHotReloadStorage(#[from] serde_json::Error),
    #[error("FetchStorage error: {0}")]
    FetchStorage(String),
    #[error("WebInit error: {0}")]
    WebInit(String),
    #[error("No base request")]
    NoBaseRequest,
    #[error("StatusNotify error: {0}")]
    StatusNotify(String),
}
