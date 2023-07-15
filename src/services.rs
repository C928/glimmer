use crate::routes::{get_locations, send_exact_location, send_isp_location};
use actix_files::{Files, NamedFile};
use actix_web::http::StatusCode;
use actix_web::{web, Responder};

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(send_isp_location)
        .service(send_exact_location)
        .service(get_locations)
        .service(Files::new("/", "static").index_file("index.html"))
        .default_service(web::to(|| async {
            NamedFile::open_async("static/404.html")
                .await
                .customize()
                .with_status(StatusCode::NOT_FOUND)
        }));
}
