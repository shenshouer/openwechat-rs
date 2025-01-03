// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Ret {
//     Ok = 0,
//     /// ticket error
//     Ticket = -14,
//     /// logic error
//     Logic = -2,
//     /// sys error
//     System = -1,
//     /// param error
//     Param = 1,
//     /// failed login warn
//     FailedLoginWarn = 1100,
//     /// failed login check
//     FailedLoginCheck = 1101,
//     /// cookie invalid
//     CookieInvalid = 1102,
//     /// login environmental abnormality
//     LoginEnvAbnormality = 1203,
//     /// operate too often
//     OperateTooOften = 1205,
// }

// pub struct BaseResponse {
//     pub ret: Ret,
//     pub errmsg: String,
// }

// impl BaseResponse {
//     pub fn is_ok(&self) -> bool {
//         self.ret == Ret::Ok
//     }
// }
