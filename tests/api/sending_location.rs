use glimmer::config::Settings;
use rand::Rng;
use redis::{Commands, Connection};

pub async fn sending_exact_location_works(
    app_address: &String,
    reqwest_client: &reqwest::Client,
    redis_conn: &mut Connection,
) {
    // count number of entries before insertion of a new location
    let c1 = redis_conn
        .scard::<&str, u64>("locations")
        .expect("Counting number of entries in redis locations SET");

    let mut rng = rand::thread_rng();
    let lat: f32 = rng.gen_range(-90.0..=90.0);
    let lon: f32 = rng.gen_range(-180.0..=180.0);
    let lat = format!("{:.6}", lat);
    let lon = format!("{:.6}", lon);

    let json: serde_json::Value = reqwest_client
        .post(&format!("{}/location", app_address))
        .json(&serde_json::json!({
            "lat": lat,
            "lon": lon,
        }))
        .send()
        .await
        .expect("Sending POST request to /location")
        .json()
        .await
        .expect("Receiving JSON from POST request to /location server endpoint");

    // count number of entries after insertion of a new location
    let c2 = redis_conn
        .scard::<&str, u64>("locations")
        .expect("Counting number of entries in redis locations SET");

    assert_eq!(json["status"].as_str(), Some("ok"));
    assert_eq!(
        json["response"].as_str(),
        Some(&*format!("{}|{}", lat, lon))
    );
    // the new entry was inserted
    assert_eq!(c1 + 1, c2);
}

pub async fn sending_isp_location_works(app_address: &String, client: &reqwest::Client) {
    let response = client
        .get(&format!("{}/location", app_address))
        .send()
        .await
        .expect("Sending GET request to /location server endpoint");

    assert!(response.status().is_success());
}
