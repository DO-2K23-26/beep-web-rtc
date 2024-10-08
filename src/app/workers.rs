use std::{
    collections::HashMap,
    error::Error,
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::{mpsc, Arc},
};
use tracing::span;
use wg::WaitGroup;

use crate::transport::{self, sync_run};

use super::server_config::build_server_config;

pub fn init_workers(
    wait_group: &WaitGroup,
    ip_endpoint: IpAddr,
    host_addr: IpAddr,
    media_ports: Arc<Vec<u16>>,
    stop_rx: &crossbeam_channel::Receiver<()>,
) -> Result<HashMap<u16, mpsc::Sender<transport::handlers::SignalingMessage>>, Box<dyn Error>> {
    let server_config = build_server_config().map_err(|e| {
        tracing::error!("Failed to build server config: {:?}", e);
        e
    })?;

    let mut media_port_thread_map: HashMap<
        u16,
        mpsc::Sender<transport::handlers::SignalingMessage>,
    > = HashMap::new();
    let n_port = media_ports.len();
    for i in 0..n_port {
        let port = media_ports[i];
        let worker = wait_group.add(1);
        let stop_rx = stop_rx.clone();
        let (signaling_tx, signaling_rx) = mpsc::channel();

        // Bind to the UDP socket
        let socket = UdpSocket::bind(format!("{host_addr}:{port}")).map_err(|e| {
            tracing::error!("Failed to bind udp socket: {:?}", e);
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to bind udp socket")
        })?;
        let socket_endpoint = SocketAddr::new(ip_endpoint, port);

        media_port_thread_map.insert(port, signaling_tx);
        let server_config_clone = server_config.clone();

        std::thread::spawn(move || {
            let _span = span!(tracing::Level::INFO, "worker", port = port).entered();

            // Run the SFU handler, passing in the owned values
            match sync_run(
                stop_rx,
                socket,
                signaling_rx,
                server_config_clone,
                socket_endpoint,
            ) {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Failed to run sfu: {:?}", e);
                }
            }
            _span.exit();
            worker.done();
        });
    }

    Ok(media_port_thread_map)
}
