//! Reader and writer for Warcraft III shadow maps, `war3map.shd`: the
//! static shadows World Editor bakes under doodads and cliffs, a byte per
//! cell, four cells to a terrain cell (32 world units), as the pathing
//! map's.
//!
//! The file has no header: its size is its grid's, which is the
//! terrain's (`war3map.w3e`): [`size_for_terrain`] gives it from the
//! terrain's points across and up.
//!
//! ```no_run
//! let bytes = std::fs::read("war3map.shd").unwrap();
//! let (w, h) = shd_rs::size_for_terrain(97, 97);
//! let shadows = shd_rs::parse(&bytes, w, h).unwrap();
//! assert_eq!(shadows.to_bytes(), bytes);
//! ```

pub use parser_rs::{BinError, BinResult};

/// Cells per terrain cell, across and up.
pub const PER_TERRAIN_CELL: u32 = 4;

/// What World Editor stores for a cell in shadow; `0` is none. Any other
/// value is kept as it is.
pub const SHADOW: u8 = 255;

/// The shadow grid's size, cells across and up, for a terrain of `width`
/// by `height` points (its cells, one less, times four).
pub fn size_for_terrain(width: u32, height: u32) -> (u32, u32) {
    (
        width.saturating_sub(1) * PER_TERRAIN_CELL,
        height.saturating_sub(1) * PER_TERRAIN_CELL,
    )
}

/// A whole `war3map.shd`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shadows {
    /// Cells across and up.
    pub width: u32,
    pub height: u32,
    /// `width * height` cells, row by row from the bottom left; 0 is no
    /// shadow.
    pub cells: Vec<u8>,
}

impl Shadows {
    /// The cell at column `x`, row `y` (from the bottom left).
    pub fn cell(&self, x: u32, y: u32) -> Option<u8> {
        (x < self.width && y < self.height).then(|| self.cells[(y * self.width + x) as usize])
    }

    /// A picture of it, `width * height` RGBA pixels, rows top to bottom
    /// (north up): dark where there's shadow, light where there's none.
    pub fn to_rgba(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.cells.len() * 4);
        for row in self.cells.chunks(self.width.max(1) as usize).rev() {
            for &c in row {
                let v = if c == 0 { 230 } else { 230 - (c as u32 * 190 / 255) as u8 };
                out.extend([v, v, v, 255]);
            }
        }
        out
    }

    /// The file's bytes: the cells, nothing else.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.cells.clone()
    }
}

/// Reads a `war3map.shd` whose grid is `width` by `height` cells
/// ([`size_for_terrain`]): its size must be that.
pub fn parse(bytes: &[u8], width: u32, height: u32) -> BinResult<Shadows> {
    let count = width as usize * height as usize;
    if bytes.len() != count {
        return Err(BinError::new(
            0,
            format!(
                "{} bytes for a {width}x{height} grid, which has {count}: is the terrain (war3map.w3e) the map's?",
                bytes.len()
            ),
        ));
    }
    Ok(Shadows { width, height, cells: bytes.to_vec() })
}

#[cfg(test)]
#[path = "lib.test.rs"]
mod tests;
