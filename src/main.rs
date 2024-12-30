use app::portsmith;
use dotenv::dotenv;

mod api;
mod app;

fn main() {
    dotenv().expect("Failed to read .env file");
    env_logger::init();
    portsmith::run();
}
