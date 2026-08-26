//! A simple dungeon map made of tiles, plus helpers to render it as a
//! pixel buffer suitable for display in a window.

use crate::assets::AssetManager;

/// A single tile in the dungeon map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
}

/// A monster placed on the map, tracked by its tile position and a color
/// used to render it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monster {
    pub x: usize,
    pub y: usize,
    /// Color used to render this monster, in `0x00RRGGBB` format.
    pub color: u32,
    /// A short label identifying this monster (e.g. "G" for goblin).
    pub label: String,
}

impl Monster {
    /// Creates a new monster at the given position with the given color
    /// and label.
    pub fn new(x: usize, y: usize, color: u32, label: impl Into<String>) -> Self {
        Monster {
            x,
            y,
            color,
            label: label.into(),
        }
    }
}

/// A rectangular grid of tiles representing a dungeon map.
#[derive(Debug, Clone)]
pub struct DungeonMap {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>,
    monsters: Vec<Monster>,
}

impl DungeonMap {
    /// Creates a new map filled entirely with walls.
    pub fn new(width: usize, height: usize) -> Self {
        DungeonMap {
            width,
            height,
            tiles: vec![Tile::Wall; width * height],
            monsters: Vec::new(),
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

    /// Returns a slice of the monsters currently on the map.
    pub fn monsters(&self) -> &[Monster] {
        &self.monsters
    }

    /// Returns true if the given tile exists and is a floor tile.
    pub fn is_floor(&self, x: usize, y: usize) -> bool {
        matches!(self.get(x, y), Some(Tile::Floor))
    }

    /// Adds a monster to the map at the given position, as long as the
    /// target tile is a floor tile and not already occupied by another
    /// monster. Returns `true` if the monster was placed, or `false` if the
    /// position is invalid (out of bounds, a wall, or occupied).
    pub fn add_monster(&mut self, monster: Monster) -> bool {
        if !self.is_floor(monster.x, monster.y) {
            return false;
        }
        if self.monster_at(monster.x, monster.y).is_some() {
            return false;
        }
        self.monsters.push(monster);
        true
    }

    /// Returns the index of the monster occupying the given tile, if any.
    pub fn monster_at(&self, x: usize, y: usize) -> Option<usize> {
        self.monsters.iter().position(|m| m.x == x && m.y == y)
    }

    /// Attempts to move the monster at `index` to the given tile. The move
    /// only succeeds if the target tile is a floor tile within bounds and
    /// is not already occupied by another monster. Returns `true` if the
    /// monster was moved.
    pub fn move_monster(&mut self, index: usize, x: usize, y: usize) -> bool {
        if index >= self.monsters.len() {
            return false;
        }
        if !self.is_floor(x, y) {
            return false;
        }
        if self
            .monsters
            .iter()
            .enumerate()
            .any(|(i, m)| i != index && m.x == x && m.y == y)
        {
            return false;
        }
        self.monsters[index].x = x;
        self.monsters[index].y = y;
        true
    }
}

/// Renders the dungeon map to an RGB pixel buffer using the provided assets manager.
///
/// If an asset for a tile or monster is present, it will be sampled and rendered.
/// Otherwise, rendering falls back to solid colors.
///
/// The returned buffer is in the `0x00RRGGBB` format expected by `minifb`.
pub fn render_to_buffer_with_assets(
    map: &DungeonMap,
    tile_size: usize,
    assets: &AssetManager,
) -> (usize, usize, Vec<u32>) {
    const WALL_COLOR: u32 = 0x00303030;
    const FLOOR_COLOR: u32 = 0x00C2A46B;

    let px_width = map.width * tile_size;
    let px_height = map.height * tile_size;
    let mut buffer = vec![WALL_COLOR; px_width * px_height];

    let wall_sprite = assets.wall_tile();
    let floor_sprite = assets.floor_tile();

    for y in 0..map.height {
        for x in 0..map.width {
            let tile = map.get(x, y);
            let sprite = match tile {
                Some(Tile::Floor) => floor_sprite,
                _ => wall_sprite,
            };

            let fallback_color = match tile {
                Some(Tile::Floor) => FLOOR_COLOR,
                _ => WALL_COLOR,
            };

            let tile_x0 = x * tile_size;
            let tile_y0 = y * tile_size;

            for ty in 0..tile_size {
                let row = tile_y0 + ty;
                if row >= px_height {
                    continue;
                }
                let row_start = row * px_width;

                for tx in 0..tile_size {
                    let col = tile_x0 + tx;
                    if col >= px_width {
                        continue;
                    }

                    let pixel_color = if let Some(s) = sprite {
                        let rgba = s.sample_at(tx, ty, tile_size, tile_size);
                        if rgba[3] == 255 {
                            ((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | (rgba[2] as u32)
                        } else if rgba[3] == 0 {
                            fallback_color
                        } else {
                            let a = rgba[3] as u32;
                            let fg_r = rgba[0] as u32;
                            let fg_g = rgba[1] as u32;
                            let fg_b = rgba[2] as u32;

                            let bg_r = (fallback_color >> 16) & 0xFF;
                            let bg_g = (fallback_color >> 8) & 0xFF;
                            let bg_b = fallback_color & 0xFF;

                            let r = (fg_r * a + bg_r * (255 - a)) / 255;
                            let g = (fg_g * a + bg_g * (255 - a)) / 255;
                            let b = (fg_b * a + bg_b * (255 - a)) / 255;

                            (r << 16) | (g << 8) | b
                        }
                    } else {
                        fallback_color
                    };

                    buffer[row_start + col] = pixel_color;
                }
            }
        }
    }

    for monster in &map.monsters {
        let x0 = monster.x * tile_size;
        let y0 = monster.y * tile_size;

        if let Some(sprite) = assets.monster_sprite(&monster.label) {
            for ty in 0..tile_size {
                let row = y0 + ty;
                if row >= px_height {
                    continue;
                }
                let row_start = row * px_width;

                for tx in 0..tile_size {
                    let col = x0 + tx;
                    if col >= px_width {
                        continue;
                    }

                    let rgba = sprite.sample_at(tx, ty, tile_size, tile_size);
                    let src_a = rgba[3] as u32;

                    if src_a == 0 {
                        continue;
                    }

                    let idx = row_start + col;
                    let dst_u32 = buffer[idx];

                    if src_a == 255 {
                        buffer[idx] =
                            ((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | (rgba[2] as u32);
                    } else {
                        let dst_r = (dst_u32 >> 16) & 0xFF;
                        let dst_g = (dst_u32 >> 8) & 0xFF;
                        let dst_b = dst_u32 & 0xFF;

                        let r = (rgba[0] as u32 * src_a + dst_r * (255 - src_a)) / 255;
                        let g = (rgba[1] as u32 * src_a + dst_g * (255 - src_a)) / 255;
                        let b = (rgba[2] as u32 * src_a + dst_b * (255 - src_a)) / 255;

                        buffer[idx] = (r << 16) | (g << 8) | b;
                    }
                }
            }
        } else {
            // Draw the monster as a slightly inset square if no sprite exists.
            let inset = (tile_size / 6).max(1);
            for ty in inset..tile_size.saturating_sub(inset) {
                let row = y0 + ty;
                if row >= px_height {
                    continue;
                }
                let row_start = row * px_width;
                for tx in inset..tile_size.saturating_sub(inset) {
                    let col = x0 + tx;
                    if col >= px_width {
                        continue;
                    }
                    buffer[row_start + col] = monster.color;
                }
            }
        }
    }

    (px_width, px_height, buffer)
}

/// Renders the dungeon map to an RGB pixel buffer, where each tile is
/// drawn as a `tile_size`-by-`tile_size` block of pixels, using default assets from `assets/`.
pub fn render_to_buffer(map: &DungeonMap, tile_size: usize) -> (usize, usize, Vec<u32>) {
    let assets = AssetManager::load_from_dir("assets");
    render_to_buffer_with_assets(map, tile_size, &assets)
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
    fn add_monster_succeeds_on_floor_tile() {
        let mut map = DungeonMap::new(5, 5);
        map.carve_room(1, 1, 3, 3);
        let monster = Monster::new(2, 2, 0x00FF0000, "G");
        assert!(map.add_monster(monster));
        assert_eq!(map.monsters().len(), 1);
        assert_eq!(map.monster_at(2, 2), Some(0));
    }

    #[test]
    fn add_monster_fails_on_wall_tile() {
        let mut map = DungeonMap::new(5, 5);
        map.carve_room(1, 1, 3, 3);
        let monster = Monster::new(0, 0, 0x00FF0000, "G");
        assert!(!map.add_monster(monster));
        assert_eq!(map.monsters().len(), 0);
    }

    #[test]
    fn add_monster_fails_out_of_bounds() {
        let mut map = DungeonMap::new(5, 5);
        let monster = Monster::new(10, 10, 0x00FF0000, "G");
        assert!(!map.add_monster(monster));
        assert_eq!(map.monsters().len(), 0);
    }

    #[test]
    fn add_monster_fails_when_tile_occupied() {
        let mut map = DungeonMap::new(5, 5);
        map.carve_room(1, 1, 3, 3);
        assert!(map.add_monster(Monster::new(2, 2, 0x00FF0000, "G")));
        assert!(!map.add_monster(Monster::new(2, 2, 0x0000FF00, "O")));
        assert_eq!(map.monsters().len(), 1);
    }

    #[test]
    fn move_monster_to_floor_tile_succeeds() {
        let mut map = DungeonMap::new(5, 5);
        map.carve_room(1, 1, 3, 3);
        map.add_monster(Monster::new(1, 1, 0x00FF0000, "G"));
        assert!(map.move_monster(0, 3, 3));
        assert_eq!(map.monsters()[0].x, 3);
        assert_eq!(map.monsters()[0].y, 3);
    }

    #[test]
    fn move_monster_to_wall_tile_fails() {
        let mut map = DungeonMap::new(5, 5);
        map.carve_room(1, 1, 3, 3);
        map.add_monster(Monster::new(1, 1, 0x00FF0000, "G"));
        assert!(!map.move_monster(0, 0, 0));
        // Position should remain unchanged.
        assert_eq!(map.monsters()[0].x, 1);
        assert_eq!(map.monsters()[0].y, 1);
    }

    #[test]
    fn move_monster_out_of_bounds_fails() {
        let mut map = DungeonMap::new(5, 5);
        map.carve_room(1, 1, 3, 3);
        map.add_monster(Monster::new(1, 1, 0x00FF0000, "G"));
        assert!(!map.move_monster(0, 100, 100));
    }

    #[test]
    fn move_monster_onto_another_monster_fails() {
        let mut map = DungeonMap::new(5, 5);
        map.carve_room(1, 1, 3, 3);
        map.add_monster(Monster::new(1, 1, 0x00FF0000, "G"));
        map.add_monster(Monster::new(2, 2, 0x0000FF00, "O"));
        assert!(!map.move_monster(0, 2, 2));
        assert_eq!(map.monsters()[0].x, 1);
        assert_eq!(map.monsters()[0].y, 1);
    }

    #[test]
    fn move_monster_invalid_index_fails() {
        let mut map = DungeonMap::new(5, 5);
        map.carve_room(1, 1, 3, 3);
        assert!(!map.move_monster(0, 2, 2));
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

    #[test]
    fn render_to_buffer_draws_monster_on_top() {
        let mut map = DungeonMap::new(2, 1);
        map.set(1, 0, Tile::Floor);
        map.add_monster(Monster::new(1, 0, 0x00FF00FF, "M"));
        let (w, _h, buffer) = render_to_buffer(&map, 6);
        // Center pixel of the floor tile should be the monster's color.
        let center = 3 * w + 8; // row 3 (mid of 6), col within tile 1 (inset 1..5)
        assert_eq!(buffer[center], 0x00FF00FF);
    }

    #[test]
    fn render_to_buffer_with_assets_renders_graphic_tiles_and_sprites() {
        use crate::assets::Sprite;

        let mut map = DungeonMap::new(2, 1);
        map.set(1, 0, Tile::Floor);
        map.add_monster(Monster::new(1, 0, 0x00000000, "goblin"));

        let mut assets = AssetManager::empty();
        let wall_sprite = Sprite::solid_color(2, 2, 0x00111111);
        let floor_sprite = Sprite::solid_color(2, 2, 0x00222222);
        let goblin_sprite = Sprite::solid_color(2, 2, 0x00333333);

        assets.set_wall_tile(wall_sprite);
        assets.set_floor_tile(floor_sprite);
        assets.add_monster_sprite("goblin", goblin_sprite);

        let (w, _h, buffer) = render_to_buffer_with_assets(&map, 4, &assets);
        assert_eq!(w, 8);
        assert_eq!(buffer[0], 0x00111111); // Wall tile (tile 0)
        assert_eq!(buffer[4], 0x00333333); // Goblin sprite over floor tile (tile 1)
    }

    #[test]
    fn render_to_buffer_with_assets_blends_semi_transparent_sprite() {
        use crate::assets::Sprite;

        let mut map = DungeonMap::new(1, 1);
        map.set(0, 0, Tile::Floor);
        map.add_monster(Monster::new(0, 0, 0x00000000, "ghost"));

        let mut assets = AssetManager::empty();
        let floor_sprite = Sprite::solid_color(1, 1, 0x00000000);
        let ghost_sprite = Sprite::new(1, 1, vec![255, 255, 255, 128]);

        assets.set_floor_tile(floor_sprite);
        assets.add_monster_sprite("ghost", ghost_sprite);

        let (_w, _h, buffer) = render_to_buffer_with_assets(&map, 1, &assets);
        assert_eq!(buffer[0], 0x00808080);
    }
}
