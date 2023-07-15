use actix_web::web::Data;
use actix_web::{middleware, rt, App, HttpServer};
use anyhow::bail;
use env_logger::Env;
use glimmer::config::Settings;
use glimmer::services;
use redis::{AsyncCommands, ConnectionLike};
use std::thread::sleep;
use std::time::Duration;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));
    let settings = Settings::new("config/local")?;
    let redis_client = redis::Client::open(settings.redis_url)?;
    let api_url = Data::new(settings.api_url);
    if !redis_client.is_open() {
        bail!("Redis instance hasn't been started");
    }

    let mut conn = match redis_client.get_multiplexed_tokio_connection().await {
        Ok(c) => c,
        Err(e) => {
            bail!("Error: getting redis connection: {}", e);
        }
    };

    let srv = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(redis_client.clone()))
            .app_data(api_url.clone())
            .configure(services)
    })
    .bind(("0.0.0.0", settings.application_port))?
    .run();

    let rt = rt::spawn(async move {
        let sleep_duration = 24 * 60 * 60;
        let expiry_time = 23 * 60 * 60 + 30 * 60;
        loop {
            if let Err(e) = conn.expire::<&str, usize>("locations", expiry_time).await {
                bail!("Setting expiry to redis SET. {e}");
            }
            sleep(Duration::from_secs(sleep_duration));
        }
    });

    srv.await?;
    rt.await??;

    Ok(())
}
