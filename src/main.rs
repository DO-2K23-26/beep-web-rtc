/**
 * @authors Mathias Durat <mathias.durat@etu.umontpellier.fr>, Tristan-Mihai Radulescu <tristan-mihai.radulescu@etu.umontpellier.fr>
 * @forked_from https://github.com/webrtc-rs/sfu (Rusty Rain <y@ngr.tc>)
 */
use std::{
    collections::HashMap,
    net::IpAddr,
    str::FromStr,
    sync::{mpsc, Arc},
};

use actix_web::rt::signal;
use app::workers::init_workers;
use signalling::web_server;
use tracing::info;
use wg::WaitGroup;

mod app;
mod config;
mod middleware;
mod signalling;
mod transport;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Your server is starting...");
    let config = match config::Config::from_env(
        "APP_KEY".to_string(),
        "VALUE_PORT_MIN".to_string(),
        "VALUE_PORT_MAX".to_string(),
        "SIGNAL_PORT".to_string(),
        "IP_ENDPOINT".to_string(),
    ) {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("Failed to load config: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to load config",
            ));
        }
    };

    info!("Starting Beep SFU Server");

    let host_addr = IpAddr::from_str("0.0.0.0").map_err(|e| {
        tracing::error!("Failed to parse host address: {:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse host address")
    })?;

    let ip_endpoint = config.ip_endpoint;

    let media_ports =
        Arc::new((config.value_port_min..=config.value_port_max).collect::<Vec<u16>>());

    let decode_service = middleware::verify_jwt::DecodeService::new(config.app_secret);

    let signal_port = config.signal_port;

    let (stop_tx, stop_rx) = crossbeam_channel::bounded::<()>(1);

    // ice_port -> worker

    let wait_group = WaitGroup::new();

    let media_port_thread_map: HashMap<u16, mpsc::Sender<transport::handlers::SignalingMessage>> =
        match init_workers(
            &wait_group,
            ip_endpoint,
            host_addr,
            media_ports.clone(),
            &stop_rx,
        ) {
            Ok(media_port_thread_map) => media_port_thread_map,
            Err(e) => {
                tracing::error!("Failed to init workers: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to init workers",
                ));
            }
        };
    info!("Starting media server with {} workers", media_ports.len());

    web_server::start(
        &host_addr.to_string(),
        &signal_port.to_string(),
        media_port_thread_map.clone(),
        decode_service,
    )
    .await?;

    info!("Press Ctrl-C to stop");
    std::thread::spawn(move || {
        let _ = signal::ctrl_c();
        stop_tx.send(()).unwrap();
    });

    let _ = stop_rx.recv();
    info!("Wait for Signaling Sever and Media Server Gracefully Shutdown...");
    wait_group.wait();

    Ok(())
}
