use actix_web::web::Data;
use mobc_redis::mobc::Pool;
use mobc_redis::redis::AsyncCommands;
use mobc_redis::RedisConnectionManager;

pub async fn insert_new_location(
    redis_conn_pool: Data<Pool<RedisConnectionManager>>,
    lat: f32,
    lon: f32,
) -> Result<String, String> {
    let mut conn = match redis_conn_pool.get().await {
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
