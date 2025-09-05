pub(crate) mod lifecycle_reducers;
pub(crate) mod schedules;
pub(crate) mod types;

// Generate autogen files:
// spacetime generate --lang rust --out-dir ../client/src/stdb
// spacetime generate --lang rust --out-dir ../sidecar/src/stdb

pub(crate) const GAME_SIZE: usize = 1;
pub(crate) const GRID_SIZE: usize = 8;
