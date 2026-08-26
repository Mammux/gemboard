//! Asset management module for loading and caching tile textures and monster sprites.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A 2D sprite image stored as RGBA pixels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    /// RGBA bytes in row-major order: [r, g, b, a, r, g, b, a, ...]
    pub pixels: Vec<u8>,
}

impl Sprite {
    /// Creates a new sprite with the given width, height, and raw RGBA pixel data.
    pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        assert_eq!(pixels.len(), (width * height * 4) as usize);
        Sprite {
            width,
            height,
            pixels,
        }
    }

    /// Loads a sprite from a PNG or other image file path.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| e.to_string())?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        Ok(Sprite {
            width,
            height,
            pixels: rgba.into_raw(),
        })
    }

    /// Creates a solid-color sprite of specified dimensions (RGB in `0x00RRGGBB` format).
    pub fn solid_color(width: u32, height: u32, rgb_color: u32) -> Self {
        let r = ((rgb_color >> 16) & 0xFF) as u8;
        let g = ((rgb_color >> 8) & 0xFF) as u8;
        let b = (rgb_color & 0xFF) as u8;
        let count = (width * height) as usize;
        let mut pixels = Vec::with_capacity(count * 4);
        for _ in 0..count {
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(255);
        }
        Sprite {
            width,
            height,
            pixels,
        }
    }

    /// Returns the RGBA pixel color at the specified `(x, y)` coordinate.
    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 4] {
        if x >= self.width as usize || y >= self.height as usize {
            return [0, 0, 0, 0];
        }
        let idx = (y * self.width as usize + x) * 4;
        [
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        ]
    }

    /// Samples a pixel from the sprite scaled to `(target_w, target_h)` grid dimensions.
    pub fn sample_at(&self, tx: usize, ty: usize, target_w: usize, target_h: usize) -> [u8; 4] {
        if target_w == 0 || target_h == 0 || self.width == 0 || self.height == 0 {
            return [0, 0, 0, 0];
        }
        let src_x = (tx * self.width as usize) / target_w;
        let src_y = (ty * self.height as usize) / target_h;
        self.get_pixel(
            src_x.min(self.width as usize - 1),
            src_y.min(self.height as usize - 1),
        )
    }
}

/// AssetManager manages loading, storing, and looking up tile graphics and monster sprites.
#[derive(Debug, Clone, Default)]
pub struct AssetManager {
    base_dir: PathBuf,
    wall_tile: Option<Sprite>,
    floor_tile: Option<Sprite>,
    monster_sprites: HashMap<String, Sprite>,
}

impl AssetManager {
    /// Creates an empty AssetManager.
    pub fn empty() -> Self {
        AssetManager {
            base_dir: PathBuf::new(),
            wall_tile: None,
            floor_tile: None,
            monster_sprites: HashMap::new(),
        }
    }

    /// Creates an AssetManager with assets loaded from the given directory path.
    pub fn load_from_dir<P: AsRef<Path>>(base_dir: P) -> Self {
        let mut manager = AssetManager {
            base_dir: base_dir.as_ref().to_path_buf(),
            wall_tile: None,
            floor_tile: None,
            monster_sprites: HashMap::new(),
        };
        manager.reload();
        manager
    }

    /// Reloads all assets from `base_dir`.
    pub fn reload(&mut self) {
        if self.base_dir.as_os_str().is_empty() {
            return;
        }

        let tiles_dir = self.base_dir.join("tiles");
        let wall_path = tiles_dir.join("wall.png");
        if wall_path.exists() {
            if let Ok(sprite) = Sprite::load_from_file(&wall_path) {
                self.wall_tile = Some(sprite);
            }
        }

        let floor_path = tiles_dir.join("floor.png");
        if floor_path.exists() {
            if let Ok(sprite) = Sprite::load_from_file(&floor_path) {
                self.floor_tile = Some(sprite);
            }
        }

        let monsters_dir = self.base_dir.join("monsters");
        if monsters_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(monsters_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file()
                        && path
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|ext| ext.eq_ignore_ascii_case("png"))
                            == Some(true)
                    {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            if let Ok(sprite) = Sprite::load_from_file(&path) {
                                self.monster_sprites.insert(stem.to_string(), sprite.clone());
                                self.monster_sprites
                                    .insert(stem.to_lowercase(), sprite);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Sets the wall tile sprite.
    pub fn set_wall_tile(&mut self, sprite: Sprite) {
        self.wall_tile = Some(sprite);
    }

    /// Sets the floor tile sprite.
    pub fn set_floor_tile(&mut self, sprite: Sprite) {
        self.floor_tile = Some(sprite);
    }

    /// Adds or replaces a monster sprite associated with a label or identifier.
    pub fn add_monster_sprite(&mut self, label: impl Into<String>, sprite: Sprite) {
        let name = label.into();
        self.monster_sprites.insert(name.to_lowercase(), sprite.clone());
        self.monster_sprites.insert(name, sprite);
    }

    /// Returns the wall tile sprite if available.
    pub fn wall_tile(&self) -> Option<&Sprite> {
        self.wall_tile.as_ref()
    }

    /// Returns the floor tile sprite if available.
    pub fn floor_tile(&self) -> Option<&Sprite> {
        self.floor_tile.as_ref()
    }

    /// Looks up a monster sprite by label (e.g. "goblin", "G", "orc", "O").
    pub fn monster_sprite(&self, label: &str) -> Option<&Sprite> {
        self.monster_sprites
            .get(label)
            .or_else(|| self.monster_sprites.get(&label.to_lowercase()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_solid_color_creates_correct_dimensions_and_pixels() {
        let sprite = Sprite::solid_color(2, 2, 0x00FF0000); // Red
        assert_eq!(sprite.width, 2);
        assert_eq!(sprite.height, 2);
        assert_eq!(sprite.pixels.len(), 16);
        assert_eq!(sprite.get_pixel(0, 0), [255, 0, 0, 255]);
    }

    #[test]
    fn sprite_sample_at_scales_correctly() {
        let sprite = Sprite::solid_color(4, 4, 0x0000FF00); // Green
        let pixel = sprite.sample_at(1, 1, 8, 8);
        assert_eq!(pixel, [0, 255, 0, 255]);
    }

    #[test]
    fn asset_manager_loads_and_stores_sprites() {
        let mut manager = AssetManager::empty();
        assert!(manager.wall_tile().is_none());
        assert!(manager.floor_tile().is_none());
        assert!(manager.monster_sprite("goblin").is_none());

        let wall = Sprite::solid_color(4, 4, 0x00303030);
        let floor = Sprite::solid_color(4, 4, 0x00C2A46B);
        let goblin = Sprite::solid_color(4, 4, 0x0000FF00);

        manager.set_wall_tile(wall);
        manager.set_floor_tile(floor);
        manager.add_monster_sprite("goblin", goblin);

        assert!(manager.wall_tile().is_some());
        assert!(manager.floor_tile().is_some());
        assert!(manager.monster_sprite("goblin").is_some());
        assert!(manager.monster_sprite("GOBLIN").is_some());
    }
}
