use base64::{prelude::BASE64_STANDARD, Engine};
use std::env;
use ureq::Request;

pub trait AuthRequestExt {
    fn basic_auth(self, username: &str, password: &str) -> Request;
    fn api_key(self, api_key: &str) -> Request;
    fn env_auth(self) -> Request;
}

impl AuthRequestExt for Request {
    fn basic_auth(self, username: &str, password: &str) -> Request {
        let encoded = BASE64_STANDARD.encode(&format!("{}:{}", username, password));
        let basic_auth = format!("Basic {}", encoded);
        self.set("Authorization", &basic_auth)
    }

    fn api_key(self, api_key: &str) -> Request {
        self.set("X-API-Key", api_key)
    }

    fn env_auth(self) -> Request {
        match std::env::var("GLUETUN_AUTH_TYPE").as_deref() {
            Ok("basic") => self.basic_auth(
                &env::var("GLUETUN_API_USERNAME").expect("Failed to read Gluetun API username"),
                &env::var("GLUETUN_API_PASSWORD").expect("Failed to read Gluetun API password"),
            ),
            Ok("apikey") => {
                self.api_key(&env::var("GLUETUN_API_KEY").expect("Failed to read Gluetun API key"))
            }
            _ => self,
        }
    }
}
