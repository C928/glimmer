use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, rt, web, HttpResponse};
use async_stream::stream;
use log::error;
use redis::{AsyncCommands, AsyncIter};
use std::convert::Infallible;

#[get("/locations")]
pub async fn get_locations(redis_client: Data<redis::Client>) -> HttpResponse {
    let mut redis_conn = match redis_client.get_multiplexed_tokio_connection().await {
        Ok(c) => c,
        Err(e) => {
            error!("Error: getting redis connection: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    return match rt::spawn(async move {
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .streaming(stream! {
                //todo: manage unwrap
                let mut loc_iter: AsyncIter<String> = redis_conn.sscan("locations").await.unwrap();
                while let Some(loc) = loc_iter.next_item().await {
                    yield Ok::<_, Infallible>(web::Bytes::from(format!("{loc}\n")));
                }
            })
    })
    .await
    {
        Ok(resp) => resp,
        Err(_) => {
            error!("Error: Spawning new actix thread");
            HttpResponse::InternalServerError().finish()
        }
    };
}
