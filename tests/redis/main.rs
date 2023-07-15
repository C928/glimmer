use glimmer::config::Settings;
use redis::Commands;

mod generate_locations;

#[actix_web::test]
async fn main() {
    let settings = Settings::new("config/local").expect("Opening config file");
    let client = redis::Client::open(settings.redis_url).expect("Connection to redis server");
    let mut conn = client
        .get_connection()
        .expect("Getting connection from redis client");

    // let locations_vec = helpers::generate_locations(100000);
    let locations_vec = generate_locations::generate_realistic_locations(100000);
    conn.sadd::<&str, Vec<String>, bool>("locations", locations_vec)
        .expect("Inserting locations vector to redis SET");
}
