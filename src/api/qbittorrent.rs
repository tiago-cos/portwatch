use std::env;

pub fn login(username: &str, password: &str) -> String {
    let url = format!(
        "http://{}:{}/api/v2/auth/login",
        env::var("GLUETUN_HOST").expect("Failed to read Gluetun host"),
        env::var("QBITTORRENT_PORT").expect("Failed to read qBittorrent port")
    );

    let username = urlencoding::encode(username);
    let password = urlencoding::encode(password);

    let response = ureq::post(&url)
        .set("Referer", url.split("/").nth(2).unwrap())
        .send_form(&[("username", &username), ("password", &password)])
        .expect("Failed to send login request");

    let header = response
        .header("Set-Cookie")
        .expect("Failed to read cookie");

    let cookie = header.split(";").next().expect("Failed to read cookie");

    cookie.to_string()
}

pub fn set_listening_port(sid: &str, port: u64) {
    let url = format!(
        "http://{}:{}/api/v2/app/setPreferences",
        env::var("GLUETUN_HOST").expect("Failed to read Gluetun host"),
        env::var("QBITTORRENT_PORT").expect("Failed to read qBittorrent port")
    );

    let response = ureq::post(&url)
        .set("Cookie", sid)
        .send_form(&[("json", &format!("{{\"listen_port\": {}}}", port))])
        .expect("Failed to send setListeningPort request");

    let status = response.status();
    if status != 200 {
        panic!("Failed to set listening port. Status: {}", status);
    }
}
