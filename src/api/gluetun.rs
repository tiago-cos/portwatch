use std::env;
use ureq::serde_json::Value;

pub fn get_forwarded_port() -> u64 {
    let url = format!(
        "http://{}:{}/v1/openvpn/portforwarded",
        env::var("GLUETUN_HOST").expect("Failed to read Gluetun host"),
        env::var("GLUETUN_PORT").expect("Failed to read Gluetun port")
    );

    let response = ureq::get(&url)
        .call()
        .expect("Failed to send getForwardedPort request");

    let forwarded_port: Value = response.into_json().expect("Failed to read forwarded port");
    forwarded_port["port"]
        .as_u64()
        .expect("Failed to read port")
}
