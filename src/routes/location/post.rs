use crate::redis_entry::insert_new_location;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{post, HttpResponse};
use log::{error, info};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Location {
    lat: String,
    lon: String,
}

#[post("/location")]
pub async fn send_exact_location(
    redis_client: Data<redis::Client>,
    loc_str: String,
) -> HttpResponse {
    let loc: Location = match serde_json::from_str(&loc_str) {
        Ok(l) => l,
        Err(_) => {
            info!("Received malformed location json: {}", loc_str);
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(json!({"status": "error", "response": "Malformed location json"}));
        }
    };

    let lon = match loc.lon.parse::<f32>() {
        Ok(l) => l,
        Err(_) => {
            info!("Received invalid coordinates: {}", loc_str);
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(json!({"status": "error", "response": "Invalid longitude coordinates"}));
        }
    };

    let lat = match loc.lat.parse::<f32>() {
        Ok(l) => l,
        Err(_) => {
            info!("Received invalid coordinates: {}", loc_str);
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(json!({"status": "error", "response": "Invalid latitude coordinates"}));
        }
    };

    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        info!("Received invalid coordinates: {}", loc_str);
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(json!({"status": "error", "response": "Malformed location json"}));
    }

    let coordinates = match insert_new_location(redis_client, lat, lon).await {
        Ok(c) => c,
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(json!({"status": "ok", "response": coordinates}))
}
