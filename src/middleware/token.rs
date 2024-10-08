use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct TokenOffer {
    pub channelSn: String,
    pub userSn: String,
    pub iat: i64,
    pub exp: i64,
}
