use std::collections::HashMap;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use std::env;
use crate::wmts_functions::wmts_tile_corner_coordinates;
use crate::matrixset_loader::TileMatrixDef;
use crate::tile_rendering::{convert_to_png_bytes, render_debug_tile};


#[get("/map")]
async fn get_map() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../misc/leaflet_map.html"))
}


#[get("/tiles")]
async fn get_tile(data: Data<HashMap<String, TileMatrixDef>>, web::Query(info): web::Query<TileInfo>) -> impl Responder {

    let projection = info.tile_matrix.split(':').collect::<Vec<&str>>()[1];
    let zoom = info.tile_matrix.split(':').collect::<Vec<&str>>()[2];
    let tile_indices = (info.tile_col, info.tile_row);

    println!("[{}] Z {zoom} X {} Y {} P {}", "YYMMDD:HHMMSSmmm", info.tile_col, info.tile_row, projection);

    if projection != "900913" {
        return HttpResponse::NotFound().body("Projection not supported.");
    }


    let matrix_set_info = data.to_owned();

    match matrix_set_info.get(zoom) {
        Some(tile_matrix_def) => {

            // println!("Zoom {zoom}: x tiles = {}", tile_matrix_def.matrix_width);
            let ((x_tl, y_tl), (x_br, y_br))
                = wmts_tile_corner_coordinates(tile_indices, tile_matrix_def);

            println!(
                "Tile corner coordinates: Top-Left ({}, {}), Bottom-Right ({}, {})",
                x_tl, y_tl, x_br, y_br
            );

            let tile_image
                = render_debug_tile(tile_matrix_def, tile_indices.0, tile_indices.1);
            let tile_png_bytes
                = convert_to_png_bytes(&tile_image);

            // HttpResponse::Ok().body(format!(
            //     "TileMatrix: {}\nProjection: {}\nZoom: {}\nTileCol: {}\nTileRow: {}\nTileX: {}\nTileY: {}",
            //     info.tile_matrix, projection, zoom, info.tile_col, info.tile_row, x_tl, y_tl
            // ))

            return HttpResponse::Ok()
                .content_type("image/png")
                .body(tile_png_bytes)

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
            .service(get_map)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}