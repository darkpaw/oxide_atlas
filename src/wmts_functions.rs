use crate::matrixset_loader::TileMatrixDef;

pub fn wmts_tile_indices(coord: (f64, f64), tile_matrix: (i32, i32, f64, f64)) -> (u32, u32) {
    let (x, y) = coord;
    let (_zoom, num_tiles, origin_x, origin_y) = tile_matrix;

    let tile_width = 2.0 * 20037508.3427892 / num_tiles as f64; // Tile width in EPSG:3857
    let tile_height = 2.0 * 20037508.3427892 / num_tiles as f64; // Tile height in EPSG:3857

    let col = ((x - origin_x) / tile_width).floor() as u32;
    let row = ((origin_y - y) / tile_height).floor() as u32;

    (col, row)
}

pub fn test_tile_indices() {
    let coord = (-8236942.0, 4970241.0); // Example spatial coordinate pair in EPSG:3857
    let tile_matrix = (6, 64, -20037508.3427892, 20037508.3427892); // Zoom level 6, 64x64 tiles, top-left corner coordinates

    let (col, row) = wmts_tile_indices(coord, tile_matrix);
    println!("Tile indices: ({}, {})", col, row);
}


pub fn wmts_tile_corner_coordinates(
    tile_indices: (u32, u32),
    tile_matrix: &TileMatrixDef,
) -> ((f64, f64), (f64, f64)) {
    let (col, row) = tile_indices;

    let tile_width = 2.0 * 20037508.3427892 / tile_matrix.matrix_width as f64; // Tile width in EPSG:3857
    let tile_height = 2.0 * 20037508.3427892 / tile_matrix.matrix_height as f64; // Tile height in EPSG:3857

    let x_top_left = tile_matrix.top_left_corner.0 + col as f64 * tile_width;
    let y_top_left = tile_matrix.top_left_corner.1 - row as f64 * tile_height;

    let x_bottom_right = x_top_left + tile_width;
    let y_bottom_right = y_top_left - tile_height;

    ((x_top_left, y_top_left), (x_bottom_right, y_bottom_right))
}

pub fn test_tile_corners() {
    let tile_indices = (12, 31); // Example XY tile

    let tile_matrix = TileMatrixDef { // Zoom level 6, 64x64 tiles, top-left corner coordinates
        identifier: "6".to_string(),
        top_left_corner: ( - 20037508.3427892,20037508.3427892),
        scale_denominator: 8735660.375448715,
        matrix_width: 64,
        matrix_height: 64
    };


    let ((x_tl, y_tl), (x_br, y_br)) = wmts_tile_corner_coordinates(tile_indices, &tile_matrix);

    println!(
        "Tile corner coordinates: Top-Left ({}, {}), Bottom-Right ({}, {})",
        x_tl, y_tl, x_br, y_br
    );
}