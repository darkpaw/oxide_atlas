mod matrixset_loader;
mod wmts_functions;
mod tile_rendering_flodraw;
mod tile_rendering;
mod server;


fn main() {
    let (latitude, longitude) = (40.7128, -74.0060);
    let zoom = 16;
    match wmts_functions::epsg4326_to_wmts_pixel(latitude, longitude, zoom) {
        Ok((pixel_x, pixel_y)) => println!("WMTS pixel coordinates: ({}, {})", pixel_x, pixel_y),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    let matrixsets= matrixset_loader::load_tile_matrix_data("data/tile_matrix_set.900913.256.xml").expect("TODO: panic message");

    for mset in matrixsets {
        println!("Matrix set: {}", mset.0);
    }

    wmts_functions::test_tile_indices();
    wmts_functions::test_tile_corners();

    tile_rendering::test_tile_rendering();

    server::start_server().expect("Start server failed.");

}
