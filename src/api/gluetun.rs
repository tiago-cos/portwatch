use std::env;
use ureq::serde_json::Value;

use crate::utils::auth::AuthRequestExt;

pub fn get_forwarded_port() -> u64 {
    let url = format!(
        "http://{}:{}/v1/openvpn/portforwarded",
        env::var("GLUETUN_HOST").unwrap_or("gluetun".to_string()),
        env::var("GLUETUN_PORT").unwrap_or("8000".to_string())
    );

    let response = ureq::get(&url)
        .env_auth()
        .call()
        .expect("Failed to send getForwardedPort request");

    let forwarded_port: Value = response.into_json().expect("Failed to read forwarded port");
    forwarded_port["port"]
        .as_u64()
        .expect("Failed to read port")
}

pub fn get_public_ip() -> String {
    let url = format!(
        "http://{}:{}/v1/publicip/ip",
        env::var("GLUETUN_HOST").unwrap_or("gluetun".to_string()),
        env::var("GLUETUN_PORT").unwrap_or("8000".to_string())
    );

    let response = ureq::get(&url)
        .env_auth()
        .call()
        .expect("Failed to send getPublicIp request");

    let public_ip: Value = response.into_json().expect("Failed to read public IP");
    public_ip["public_ip"]
        .as_str()
        .expect("Failed to read IP")
        .to_string()
}
