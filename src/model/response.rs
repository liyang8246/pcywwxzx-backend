use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ResWithVerifyCode<T> {
    pub verifycode:     String,
    pub verifycode_url: String,
    pub response:       T,
}
