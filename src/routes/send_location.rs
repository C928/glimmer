use crate::redis_entry::insert_new_location;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, post, HttpRequest, HttpResponse};
use log::{error, info};
use mobc_redis::mobc::Pool;
use mobc_redis::RedisConnectionManager;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ISPLocation {
    lat: f32,
    lon: f32,
}

#[get("/location")]
pub async fn send_isp_location(
    redis_conn_pool: Data<Pool<RedisConnectionManager>>,
    api_url: Data<String>,
    req: HttpRequest,
) -> HttpResponse {
    let mut ip = match req.peer_addr() {
        Some(ip) => ip.ip().to_string(),
        None => {
            error!("Retrieving client IP address.\nRequest: {:?}", req);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if ip == "127.0.0.1" {
        if let Some(ip_v4) = public_ip::addr_v4().await {
            ip = ip_v4.to_string();
        } else {
            error!("Determining public IP address of host");
            return HttpResponse::Ok().json(
                json!({"status": "error", "response": "Host public IP address \
                 could not be determined"}),
            );
        }
    }
    let url = api_url.replace("{}", &ip);
    let loc: ISPLocation = match reqwest::get(url).await {
        Ok(resp) => match resp.json().await {
            Ok(json) => json,
            Err(_) => {
                error!("Couldn't determine ISP location from request: {:?}", req);
                return HttpResponse::Ok().json(
                    json!({"status": "error", "response": "ISP location could not be determined"}),
                );
            }
        },
        Err(_) => {
            error!(
                "Couldn't reach external localisation API: {}",
                api_url.to_string()
            );
            return HttpResponse::Ok().json(
                json!({"status": "error", "response": "ISP location could not be determined"}),
            );
        }
    };

    let coordinates = match insert_new_location(redis_conn_pool, loc.lat, loc.lon).await {
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

#[derive(Deserialize)]
struct ExactLocation {
    lat: String,
    lon: String,
}

#[post("/location")]
pub async fn send_exact_location(
    redis_conn_pool: Data<Pool<RedisConnectionManager>>,
    loc_str: String,
) -> HttpResponse {
    let loc: ExactLocation = match serde_json::from_str(&loc_str) {
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

    let coordinates = match insert_new_location(redis_conn_pool, lat, lon).await {
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
