use serde::Deserialize;
use serde_xml_rs::from_reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct TileMatrixSet {
    #[serde(rename = "TileMatrix", default)]
    tile_matrices: Vec<TileMatrix>,
}

#[derive(Debug, Deserialize)]
struct TileMatrix {
    #[serde(rename = "Identifier")]
    identifier: String,
    #[serde(rename = "ScaleDenominator")]
    scale_denominator: f64,
    #[serde(rename = "TopLeftCorner")]
    top_left_corner: String,
    #[serde(rename = "MatrixWidth")]
    matrix_width: u32,
    #[serde(rename = "MatrixHeight")]
    matrix_height: u32,
}

#[derive(Debug, Deserialize)]
pub struct TileMatrixDef {
    pub identifier: String,
    pub scale_denominator: f64,
    pub top_left_corner: (f64, f64),
    pub matrix_width: u32,
    pub matrix_height: u32,
}

pub(crate) fn load_tile_matrix_data(file_path: &str) -> Result<HashMap<String, TileMatrixDef>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let tile_matrix_set: TileMatrixSet = from_reader(buf_reader)?;

    let mut result = HashMap::new();

    for tile_matrix in tile_matrix_set.tile_matrices {
        let identifier = tile_matrix.identifier;
        let scale_denominator = tile_matrix.scale_denominator;
        let coords: Vec<f64> = tile_matrix
            .top_left_corner
            .split_whitespace()
            .map(|s| f64::from_str(s).unwrap())
            .collect();
        let top_left_corner = (coords[0], coords[1]);
        let matrix_width = tile_matrix.matrix_width;
        let matrix_height = tile_matrix.matrix_height;

        let tile_matrix_def = TileMatrixDef {
            identifier: identifier.clone(),
            scale_denominator,
            top_left_corner,
            matrix_width,
            matrix_height
        };

        result.insert(identifier, tile_matrix_def);
    }

    Ok(result)
}