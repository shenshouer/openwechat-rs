use crate::consts::Status;

#[derive(Debug)]
pub struct ResponseCheckLogin {
    pub status: Status,
    pub raw: String,
}
