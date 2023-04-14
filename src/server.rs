use std::collections::HashMap;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use std::env;
use crate::wmts_functions::wmts_tile_corner_coordinates;
use crate::matrixset_loader::TileMatrixDef;

#[get("/tiles")]
async fn get_tile(data: Data<HashMap<String, TileMatrixDef>>, web::Query(info): web::Query<TileInfo>) -> impl Responder {

    let projection = info.tile_matrix.split(':').collect::<Vec<&str>>()[1];
    let zoom = info.tile_matrix.split(':').collect::<Vec<&str>>()[2];
    let tile_indices = (info.tile_col, info.tile_row);

    let matrix_set_info = data.to_owned();

    match matrix_set_info.get(zoom) {
        Some(tile_matrix_def) => {

            println!("Zoom {zoom}: x tiles = {}", tile_matrix_def.matrix_width);
            let ((x_tl, y_tl), (x_br, y_br)) = wmts_tile_corner_coordinates(tile_indices, tile_matrix_def);

            println!(
                "Tile corner coordinates: Top-Left ({}, {}), Bottom-Right ({}, {})",
                x_tl, y_tl, x_br, y_br
            );

            HttpResponse::Ok().body(format!(
                "TileMatrix: {}\nProjection: {}\nZoom: {}\nTileCol: {}\nTileRow: {}\nTileX: {}\nTileY: {}",
                info.tile_matrix, projection, zoom, info.tile_col, info.tile_row, x_tl, y_tl
            ))

        },
        None => {
            println!("Failed to find TM def.");
            return HttpResponse::NotFound().body("TileMatrix not known for given zoom.");
        }
    }

}

#[derive(serde::Deserialize)]
struct TileInfo {
    tile_matrix: String,
    tile_col: u32,
    tile_row: u32,
}

#[actix_rt::main]
pub async fn start_server(matrix_sets: HashMap<String, TileMatrixDef>) -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_owned());

    let matrix_sets_data = Data::new(matrix_sets);

    HttpServer::new( move || {
        App::new()
            .app_data(matrix_sets_data.clone())
            .service(get_tile)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}