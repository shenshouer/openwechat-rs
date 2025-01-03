use crate::errors::Error;

pub type MessageErrorHandler = fn(err: Error);
