use crate::api::{gluetun, qbittorrent};
use log::info;
use port_check::is_port_reachable_with_timeout;
use std::{env, path::Path, thread::sleep, time::Duration};

fn is_accessible(port: u64) -> bool {
    let address = gluetun::get_public_ip();
    let url = format!("http://{}:{}", address, port);

    is_port_reachable_with_timeout(url, Duration::from_secs(5))
}

fn port_changed(port: u64) -> bool {
    if !Path::new("port.txt").exists() {
        return true;
    }

    let file = std::fs::read_to_string("port.txt").expect("Failed to read port.txt");
    let old_port = file.parse::<u64>().expect("Failed to parse port.txt");

    port != old_port
}

fn handle_port_change(port: u64) {
    let sid = qbittorrent::login(
        &env::var("QBITTORRENT_USERNAME").expect("Failed to read qBittorrent username"),
        &env::var("QBITTORRENT_PASSWORD").expect("Failed to read qBittorrent password"),
    );

    qbittorrent::set_listening_port(&sid, port);

    std::fs::write("port.txt", port.to_string()).expect("Failed to write port.txt");
}

fn handle_inaccessible(port: u64) {
    let sid = qbittorrent::login(
        &env::var("QBITTORRENT_USERNAME").expect("Failed to read qBittorrent username"),
        &env::var("QBITTORRENT_PASSWORD").expect("Failed to read qBittorrent password"),
    );

    qbittorrent::set_listening_port(&sid, port + 1);

    sleep(Duration::from_secs(5));

    qbittorrent::set_listening_port(&sid, port);
}

pub fn run() {
    let port = gluetun::get_forwarded_port();

    if port_changed(port) {
        info!("Port changed to {}", port);
        return handle_port_change(port);
    } else if !is_accessible(port) {
        info!("Port {} is not accessible, changing it temporarily", port);
        return handle_inaccessible(port);
    }

    info!("Port {} is accessible", port);
}
