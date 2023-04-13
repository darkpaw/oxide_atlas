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
}

pub(crate) fn load_tile_matrix_data(file_path: &str) -> Result<HashMap<String, (f64, (f64, f64))>, Box<dyn std::error::Error>> {
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

        result.insert(identifier, (scale_denominator, top_left_corner));
    }

    Ok(result)
}
