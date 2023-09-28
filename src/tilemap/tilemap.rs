use serde::{Deserialize, Serialize};
use sfml::system::Vector2u;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub enum TileMapError {
    InvalidPosition,
    InvalidLayer,
    WriteError,
    ReadError,
}

//Allow Serde Serialization / Deserialization of Vector2u
#[derive(Serialize, Deserialize)]
#[serde(remote = "Vector2u")]
struct Vector2uDef {
    x: u32,
    y: u32,
}

///TileMap is the raw representation of a tile TileMap
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TileMap {
    // The map tiles, the first vector is the layer, the second tiles is the row major order
    tiles: Vec<Vec<u32>>,
    // the tile map size
    #[serde(with = "Vector2uDef")]
    size: Vector2u,
    // The number of layers
    layer_count: u32,
}

impl TileMap {
    /// Create a new tile map of given size, with given number of layers
    /// The initial layers will be fill with provided default
    /// the others will be fill with 0 (air)
    pub fn new<T: Into<Vector2u>>(size: T, layer_count: u32, default: u32) -> Self {
        let size = size.into();
        let mut tiles = Vec::with_capacity(layer_count as usize);
        tiles.push(vec![default; (size.x * size.y) as usize]);

        for _ in 1..layer_count {
            tiles.push(vec![0; (size.x * size.y) as usize]);
        }

        TileMap {
            tiles,
            size,
            layer_count,
        }
    }
}
