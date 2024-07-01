use std::{
    collections::HashMap, sync::mpsc::{self, Sender} // collections::HashMap, path, sync::mpsc::{self, Sender}
};

use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use bytes::Bytes;
use sha2::digest::typenum::uint;
use tracing::{error, info};

use crate::transport::handlers::{SignalingMessage, SignalingProtocolMessage};

use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct RTCSessionDescriptionSerializable {
    sdp: String,
    #[serde(rename = "type")]
    sdp_type: String,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenOffer {
    channelSn: i64,
    userSn: i64,
    iat: i64,
    exp: i64,
}

#[post("/offer/{token}")]
pub async fn handle_offer(
    path: web::Path<String>,
    offer_sdp: web::Json<RTCSessionDescriptionSerializable>,
    media_port_thread_map: Data<HashMap<u16, Sender<SignalingMessage>>>,
) -> impl Responder {

    let path = path.into_inner();
    let token = path.to_string();

    // Create a new validation object
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    // Decode the token
    let decoded_token = decode::<TokenOffer>(
        &token,
        &DecodingKey::from_secret("G9e1d_eQQQpnbiEAeBa7uqYXwRgtecNL".as_ref()),
        &validation
    );

    // Extract the claims before moving decoded_token
    let decoded_token = match decoded_token {
        Ok(token_data) => token_data,
        Err(e) => {
            error!("Token decoding error: {:?}", e);
            return HttpResponse::Unauthorized().body("Invalid or expired token");
        },
    };

    //cast in u64
    let channel_id = decoded_token.claims.channelSn as u64;
    let user_id = decoded_token.claims.userSn as u64;

    let sorted_ports: Vec<u16> = media_port_thread_map.keys()
        .map(|x| *x)
        .collect();

    let port = sorted_ports[(channel_id as usize) % sorted_ports.len()];

    let (response_tx, response_rx) = mpsc::channel();

    let payload_to_string = serde_json::to_string(&offer_sdp).map_err(|e| {
        error!("Error serializing offer: {}", e);
        HttpResponse::InternalServerError().body("Error serializing offer")
    });

    let offer_sdp = match payload_to_string {
        Ok(s) => {
            info!("Received offer: {}", s);
            Bytes::from(s)
        }
        Err(r) => return r,
    };

    match media_port_thread_map.get(&port) {
        Some(tx) => {
            tx.send(SignalingMessage {
                user_id,
                request: SignalingProtocolMessage::Offer {
                    channel_id,
                    user_id,
                    offer_sdp: offer_sdp,
                },
                response_tx,
            })
            .unwrap();
        }
        None => {
            return HttpResponse::InternalServerError().body("No media port available");
        }
    };

    let response = response_rx.recv().expect("receive answer offer");

    let to_send = match response {
        SignalingProtocolMessage::Answer {
            channel_id: _,
            user_id: _,
            answer_sdp,
        } => HttpResponse::Ok()
            .content_type("application/json")
            .body(answer_sdp),
        SignalingProtocolMessage::Err {
            channel_id,
            user_id,
            reason,
        } => {
            let reason_str = std::str::from_utf8(&reason).unwrap();
            error!(
                "Error for session {} endpoint {}: {}",
                channel_id, user_id, reason_str
            );
            HttpResponse::InternalServerError().body(format!(
                "Error for session {} endpoint {}: {}",
                channel_id, user_id, reason_str
            ))
        }
        SignalingProtocolMessage::Leave {
            channel_id,
            user_id,
        } => {
            error!("Session {} endpoint {} left", channel_id, user_id);
            return HttpResponse::InternalServerError().body("Session left");
        }
        SignalingProtocolMessage::Offer {
            channel_id,
            user_id,
            offer_sdp,
        } => {
            let offer_sdp_str = std::str::from_utf8(&offer_sdp).unwrap();
            error!(
                "Received offer for session {} endpoint {} while expecting answer with sdp {}",
                channel_id, user_id, offer_sdp_str
            );
            return HttpResponse::InternalServerError()
                .body("Received offer for session endpoint while expecting answer");
        }

        SignalingProtocolMessage::Ok {
            channel_id,
            user_id,
        } => {
            error!(
                "Received Ok for session {} endpoint {} while expecting answer",
                channel_id, user_id
            );
            return HttpResponse::InternalServerError()
                .body("Received Ok for session endpoint while expecting answer");
        }
    };
    to_send
}

#[post("/leave/{session}/{endpoint}")]
pub async fn leave() -> impl Responder {
    HttpResponse::Ok() //idk what to do here
}
