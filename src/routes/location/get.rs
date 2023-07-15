use crate::redis_entry::insert_new_location;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse};
use log::error;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Location {
    lat: f32,
    lon: f32,
}

#[get("/location")]
pub async fn send_isp_location(
    redis_client: Data<redis::Client>,
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
    let loc: Location = match reqwest::get(url).await {
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

    let coordinates = match insert_new_location(redis_client, loc.lat, loc.lon).await {
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
