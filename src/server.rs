use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/tiles")]
async fn get_tile(web::Query(info): web::Query<TileInfo>) -> impl Responder {
    let projection = info.tile_matrix.split(':').collect::<Vec<&str>>()[1];
    let zoom = info.tile_matrix.split(':').collect::<Vec<&str>>()[2];

    HttpResponse::Ok().body(format!(
        "TileMatrix: {}\nProjection: {}\nZoom: {}\nTileCol: {}\nTileRow: {}",
        info.tile_matrix, projection, zoom, info.tile_col, info.tile_row
    ))
}

#[derive(serde::Deserialize)]
struct TileInfo {
    tile_matrix: String,
    tile_col: u32,
    tile_row: u32,
}

#[actix_rt::main]
pub async fn start_server() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_owned());

    HttpServer::new(|| {
        App::new()
            .service(get_tile)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}