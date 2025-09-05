use spacetimedb::SpacetimeType;

use crate::GRID_SIZE;

#[derive(Debug, SpacetimeType)]
pub(crate) struct Grid {
    inner: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            inner: vec![vec![Tile; GRID_SIZE]; GRID_SIZE],
        }
    }
}

#[derive(Clone, Debug, SpacetimeType)]
pub(crate) struct Tile;
