use actix_web::web::Data;
use redis::{AsyncCommands, Client};

pub async fn insert_new_location(
    redis_client: Data<Client>,
    lat: f32,
    lon: f32,
) -> Result<String, String> {
    let mut conn = match redis_client.get_multiplexed_tokio_connection().await {
        Ok(c) => c,
        Err(e) => {
            return Err(format!("Error: getting redis connection: {}", e));
        }
    };

    let coordinates = format!("{:.6}|{:.6}", lat, lon);
    if let Err(e) = conn
        .sadd::<&str, &str, bool>("locations", &coordinates)
        .await
    {
        return Err(format!("Error: Insertion location: {}", e));
    };

    Ok(coordinates)
}
