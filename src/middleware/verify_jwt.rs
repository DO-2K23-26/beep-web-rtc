use std::error::Error;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use tracing::error;

use super::token::TokenOffer;

#[derive(Debug, Clone)]
pub struct DecodeService {
    pub app_secret: String,
}

impl DecodeService {
    pub fn new(app_secret: String) -> Self {
        DecodeService { app_secret }
    }

    pub fn decode_token(&self, token: &str) -> Result<TokenOffer, Box<dyn Error>> {
        let validation = Validation::new(Algorithm::HS256);

        // Decode the token
        let decoded_token = decode::<TokenOffer>(
            token,
            &DecodingKey::from_secret(self.app_secret.as_ref()),
            &validation,
        );

        // Extract the claims before moving decoded_token
        let decoded_token: jsonwebtoken::TokenData<TokenOffer> = match decoded_token {
            Ok(token_data) => token_data,
            Err(e) => {
                error!("Token decoding error: {:?}", e);
                return Err(Box::new(e));
            }
        };

        Ok(decoded_token.claims)
    }
}
