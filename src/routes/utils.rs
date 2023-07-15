use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

pub fn see_other(path: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish()
}
