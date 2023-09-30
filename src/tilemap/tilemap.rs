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

    /// Retrieve the tile at given position on given layer
    /// this will return None if the position / layers doesn't exist
    pub fn get_tile<T: Into<Vector2u>>(&self, position: T, layer: u32) -> Option<u32> {
        let index = self.compute_index(position.into())?;

        self.tiles
            .get(layer as usize)
            .and_then(|v| v.get(index))
            .copied()
    }

    /// Set the tile at given position and layer
    /// this operation will fails if the position / layer doesn't exist
    pub fn set_tile<T: Into<Vector2u>>(
        &mut self,
        position: T,
        layer: u32,
        tile: u32,
    ) -> Result<(), TileMapError> {
        let index = self
            .compute_index(position.into())
            .ok_or(TileMapError::InvalidPosition)?;

        self.tiles
            .get_mut(layer as usize)
            .ok_or(TileMapError::InvalidLayer)
            .map(|v| v[index] = tile)
    }

    /// Retrieve the tile map size
    pub fn size(&self) -> Vector2u {
        self.size
    }

    /// Retrieve the number of layers
    pub fn layer_count(&self) -> u32 {
        self.layer_count
    }

    /// Write the tile map to given writer
    pub fn write(&self, mut writer: impl Write) -> Result<(), TileMapError> {
        let bytes: Vec<u8> = bincode::serialize(&self).map_err(|_| TileMapError::WriteError)?;
        writer
            .write_all(&bytes)
            .map_err(|_| TileMapError::WriteError)
    }

    /// Compute the vector index from given position
    fn compute_index<T: Into<Vector2u>>(&self, position: T) -> Option<usize>{
        let position = position.into();

        // Validate input
        if position.x >= self.size.x || position.y >= self.size.y{
            return None;
        }
        Some((position.x + position.y * self.size.x) as usize)
    }
}
impl TryFrom<File> for TileMap {
    type Error = TileMapError;
    fn try_from(value: File) -> Result<Self, Self::Error>{
        bincode::deserialize from(value).map err(| | TileMapError::ReadError)
    }
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_tile_map_new(){
        let tile_map = TileMap::new((20, 10), 2, 2);
        assert_eq!(tile_map.tiles.len(), 2);
        assert_eq!(tile_map.size.x, 20);
        assert_eq!(tile_map.size.y, 10);
        assert_eq!(tile_map.layer_count, 2);
        assert_eq!(tile_map.tiles.get(0).unwrap().len(), 20 * 10);
        assert_eq!(tile_map.tiles.get(1).unwrap().len(), 20 * 10);

        // Make sure first layer is fill with 2
        for i in 0.200{
            assert_eq!(tile_map.tiles[0][i], 2);
        } 
    }
}
