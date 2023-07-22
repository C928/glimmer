use crate::sending_location::{sending_exact_location_works, sending_isp_location_works};
use glimmer::config::Settings;
use mobc_redis::redis::Client;

mod sending_location;

#[actix_web::test]
async fn main() {
    let settings = Settings::new("config/local").unwrap();
    let redis_client = Client::open(settings.redis_url).expect("Connection to redis server");
    let mut redis_conn = redis_client
        .get_connection()
        .expect("Getting connection from client");

    let app_address = format!(
        "http://{}:{}",
        settings.application_host, settings.application_port
    );
    let reqwest_client = reqwest::Client::new();

    sending_exact_location_works(&app_address, &reqwest_client, &mut redis_conn).await;
    sending_isp_location_works(&app_address, &reqwest_client).await;
}
