use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct TokenOffer {
    #[serde(rename = "channelSn")]
    pub channel_sn: String,
    #[serde(rename = "userSn")]
    pub user_sn: String,
    pub iat: i64,
    pub exp: i64,
}
