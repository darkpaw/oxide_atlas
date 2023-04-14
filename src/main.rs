mod matrixset_loader;
mod wmts_functions;
mod tile_rendering;
mod server;


fn main() {

    let matrix_sets= matrixset_loader::load_tile_matrix_data("data/tile_matrix_set.900913.256.xml").expect("TODO: panic message");

    for mset in &matrix_sets {
        println!("Matrix set: {}", mset.0);
    }

    wmts_functions::test_tile_indices();
    wmts_functions::test_tile_corners();

    tile_rendering::test_tile_rendering();

    server::start_server(matrix_sets).expect("Start server failed.");

}
