# shd-rs

Reader and writer for Warcraft III shadow maps, `war3map.shd`: the
static shadows World Editor bakes under doodads and cliffs, a byte per
cell, four cells to a terrain cell (32 world units), as the pathing
map's. A library and a small CLI; the [wc3-rs](../wc3-rs) editor shows
the file as a picture in its texture viewer.

Every map of the game and the corpus (208 files) has exactly its
terrain's size, and reads and writes back.

## Format

No header: `16 × cells` bytes, the terrain's cells (`war3map.w3e`'s
points less one, across and up) four by four, row by row from the
bottom left. 0 is no shadow; World Editor stores 255 for a shadow, and
other values are kept as they are.

So the size comes from the terrain: `size_for_terrain(width, height)`
gives the grid for a terrain of that many points, and `parse(bytes,
width, height)` checks the file has it.

One description of the format has the shadows drawn per terrain cell at
`indexX * 512 / 15` within it, smaller at the cell's edges; that's how
the game draws them, not what the file stores.

## Library

```rust
let (w, h) = shd_rs::size_for_terrain(terrain.width, terrain.height);
let shadows = shd_rs::parse(&std::fs::read("war3map.shd")?, w, h)?;
let rgba = shadows.to_rgba(); // north up, dark where there's shadow
```

## CLI

```sh
shd-rs war3map.shd 97 97   # check against a terrain of 97 by 97 points
```

## Tests

```sh
cargo test
SHD_CORPUS=<folder of map folders> cargo test --release corpus -- --ignored
```

The corpus test reads each map's `war3map.w3e` for the size.
