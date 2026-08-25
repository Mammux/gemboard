//! A simple dungeon map made of tiles, plus helpers to render it as a
//! pixel buffer suitable for display in a window.

/// A single tile in the dungeon map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
}

/// A rectangular grid of tiles representing a dungeon map.
#[derive(Debug, Clone)]
pub struct DungeonMap {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>,
}

impl DungeonMap {
    /// Creates a new map filled entirely with walls.
    pub fn new(width: usize, height: usize) -> Self {
        DungeonMap {
            width,
            height,
            tiles: vec![Tile::Wall; width * height],
        }
    }

    /// Returns the tile at the given coordinates, if within bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.tiles[y * self.width + x])
    }

    /// Sets the tile at the given coordinates, if within bounds.
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x] = tile;
        }
    }

    /// Carves out a rectangular room of floor tiles (inclusive bounds),
    /// clamped to the map's dimensions.
    pub fn carve_room(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for y in y0..=y1.min(self.height.saturating_sub(1)) {
            for x in x0..=x1.min(self.width.saturating_sub(1)) {
                self.set(x, y, Tile::Floor);
            }
        }
    }

    /// Builds a small, fixed example dungeon: two rooms connected by a
    /// corridor.
    pub fn example() -> Self {
        let mut map = DungeonMap::new(20, 15);
        map.carve_room(1, 1, 6, 5);
        map.carve_room(12, 8, 18, 13);
        // Corridor connecting the two rooms.
        map.carve_room(6, 3, 12, 3);
        map.carve_room(12, 3, 12, 8);
        map
    }
}

/// Renders the dungeon map to an RGB pixel buffer, where each tile is
/// drawn as a `tile_size`-by-`tile_size` block of pixels.
///
/// The returned buffer is in the `0x00RRGGBB` format expected by
/// `minifb`, with a length of `map.width * tile_size * map.height * tile_size`.
pub fn render_to_buffer(map: &DungeonMap, tile_size: usize) -> (usize, usize, Vec<u32>) {
    const WALL_COLOR: u32 = 0x00303030;
    const FLOOR_COLOR: u32 = 0x00C2A46B;

    let px_width = map.width * tile_size;
    let px_height = map.height * tile_size;
    let mut buffer = vec![WALL_COLOR; px_width * px_height];

    for y in 0..map.height {
        for x in 0..map.width {
            let color = match map.get(x, y) {
                Some(Tile::Floor) => FLOOR_COLOR,
                _ => WALL_COLOR,
            };
            for ty in 0..tile_size {
                let row = y * tile_size + ty;
                let row_start = row * px_width + x * tile_size;
                for tx in 0..tile_size {
                    buffer[row_start + tx] = color;
                }
            }
        }
    }

    (px_width, px_height, buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_map_is_all_walls() {
        let map = DungeonMap::new(5, 4);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 4);
        for y in 0..4 {
            for x in 0..5 {
                assert_eq!(map.get(x, y), Some(Tile::Wall));
            }
        }
    }

    #[test]
    fn out_of_bounds_get_returns_none() {
        let map = DungeonMap::new(3, 3);
        assert_eq!(map.get(3, 0), None);
        assert_eq!(map.get(0, 3), None);
    }

    #[test]
    fn carve_room_creates_floor_tiles() {
        let mut map = DungeonMap::new(10, 10);
        map.carve_room(2, 2, 4, 4);
        for y in 2..=4 {
            for x in 2..=4 {
                assert_eq!(map.get(x, y), Some(Tile::Floor));
            }
        }
        // Outside the room should remain walls.
        assert_eq!(map.get(0, 0), Some(Tile::Wall));
        assert_eq!(map.get(5, 5), Some(Tile::Wall));
    }

    #[test]
    fn carve_room_clamps_to_map_bounds() {
        let mut map = DungeonMap::new(5, 5);
        map.carve_room(3, 3, 10, 10);
        assert_eq!(map.get(4, 4), Some(Tile::Floor));
    }

    #[test]
    fn example_map_has_expected_dimensions() {
        let map = DungeonMap::example();
        assert_eq!(map.width, 20);
        assert_eq!(map.height, 15);
        // A tile inside the first room should be a floor.
        assert_eq!(map.get(2, 2), Some(Tile::Floor));
        // A tile in the connecting corridor should be a floor.
        assert_eq!(map.get(9, 3), Some(Tile::Floor));
    }

    #[test]
    fn render_to_buffer_has_correct_dimensions() {
        let map = DungeonMap::new(4, 3);
        let (w, h, buffer) = render_to_buffer(&map, 8);
        assert_eq!(w, 32);
        assert_eq!(h, 24);
        assert_eq!(buffer.len(), 32 * 24);
    }

    #[test]
    fn render_to_buffer_colors_floor_and_wall_differently() {
        let mut map = DungeonMap::new(2, 1);
        map.set(1, 0, Tile::Floor);
        let (w, _h, buffer) = render_to_buffer(&map, 2);
        // Left tile (wall) block.
        assert_eq!(buffer[0], buffer[1]);
        // Right tile (floor) block differs from left.
        assert_ne!(buffer[0], buffer[w - 1]);
    }
}
