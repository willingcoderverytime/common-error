extern crate common_error;


#[doc = "zheshi yige xiaoxiao de wenti "]
pub struct SYS00001;

impl SYS00001 {
    pub fn throw(code: &str) -> (common_error::CommonError, String) {
        let msg = "sys error ".to_string() + code + "";
        let common_error = common_error::CommonError { code: "SYS00001".to_string(), message: msg.clone() };
        (common_error, msg)
    }
}

#[doc = "true"]
pub struct SYS00002;

impl SYS00002 {
    pub fn throw(file_name: &str, msg: &str) -> (common_error::CommonError, String) {
        let msg = "file ".to_string() + file_name + " open file error info : " + msg + "";
        let common_error = common_error::CommonError { code: "SYS00002".to_string(), message: msg.clone() };
        (common_error, msg)
    }
}
