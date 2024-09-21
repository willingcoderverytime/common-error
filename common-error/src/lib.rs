
pub mod handle_error_macro;

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct CommonError {
    pub code: String,
    pub message: String,
}

impl Display for CommonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error code [{}]:{} \n", self.code, self.message)
    }
}

impl Error for CommonError {}