use std::env;

pub fn login(username: &str, password: &str) -> String {
    let referer = format!(
        "http://{}:{}",
        env::var("GLUETUN_HOST").unwrap_or("gluetun".to_string()),
        env::var("QBITTORRENT_PORT").unwrap_or("8080".to_string())
    );
    let url = format!("{}/api/v2/auth/login", referer);

    let response = ureq::post(&url)
        .set("Referer", &referer)
        .send_form(&[("username", &username), ("password", &password)])
        .expect("Failed to send login request");

    let header = response
        .header("set-cookie")
        .expect("Failed to read cookie");

    let cookie = header.split(";").next().expect("Failed to read cookie");

    cookie.to_string()
}

pub fn set_listening_port(sid: &str, port: u64) {
    let url = format!(
        "http://{}:{}/api/v2/app/setPreferences",
        env::var("GLUETUN_HOST").unwrap_or("gluetun".to_string()),
        env::var("QBITTORRENT_PORT").unwrap_or("8080".to_string())
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
