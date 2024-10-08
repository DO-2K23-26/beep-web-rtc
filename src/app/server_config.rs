use dtls::extension::extension_use_srtp::SrtpProtectionProfile;
use sfu::{RTCCertificate, ServerConfig};
use std::{error::Error, sync::Arc};

pub fn build_server_config() -> Result<Arc<ServerConfig>, Box<dyn Error>> {
    let key_pair = rcgen::KeyPair::generate(&rcgen::PKCS_ECDSA_P256_SHA256).map_err(|e| {
        tracing::error!("Failed to generate key pair: {:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to generate key pair")
    })?;
    let certificates = match RTCCertificate::from_key_pair(key_pair) {
        Ok(cert) => vec![cert],
        Err(e) => {
            tracing::error!("Failed to generate certificate: {:?}", e);
            return Err(Box::new(e));
        }
    };

    let dtls_handshake_config = Arc::new(
        dtls::config::ConfigBuilder::default()
            .with_certificates(
                certificates
                    .iter()
                    .map(|cert| cert.dtls_certificate.clone())
                    .collect(),
            )
            .with_srtp_protection_profiles(vec![SrtpProtectionProfile::Srtp_Aes128_Cm_Hmac_Sha1_80])
            .with_extended_master_secret(dtls::config::ExtendedMasterSecretType::Require)
            .build(false, None)
            .map_err(|e| {
                tracing::error!("Failed to build dtls handshake config: {:?}", e);
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to build dtls handshake config",
                )
            })?,
    );

    let sctp_endpoint_config = Arc::new(sctp::EndpointConfig::default());
    let sctp_server_config = Arc::new(sctp::ServerConfig::default());
    let server_config: Arc<sfu::ServerConfig> = Arc::new(
        sfu::ServerConfig::new(certificates)
            .with_dtls_handshake_config(dtls_handshake_config)
            .with_sctp_endpoint_config(sctp_endpoint_config)
            .with_sctp_server_config(sctp_server_config),
    );

    Ok(server_config)
}
