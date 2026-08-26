mod map;

use map::{DungeonMap, Monster, render_to_buffer};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

const TILE_SIZE: usize = 24;

/// Converts a mouse pixel position into tile coordinates, if within the
/// map's bounds.
fn pixel_to_tile(mouse_x: f32, mouse_y: f32, tile_size: usize) -> (usize, usize) {
    (
        (mouse_x as usize) / tile_size,
        (mouse_y as usize) / tile_size,
    )
}

fn main() {
    let mut dungeon = DungeonMap::example();
    dungeon.add_monster(Monster::new(2, 2, 0x00E01010, "G"));
    dungeon.add_monster(Monster::new(14, 10, 0x0010A010, "O"));
    println!(
        "Loaded {} monster(s) onto the map.",
        dungeon.monsters().len()
    );

    let (width, height, _) = render_to_buffer(&dungeon, TILE_SIZE);

    let mut window = Window::new(
        "Gemboard - Dungeon Map",
        width,
        height,
        WindowOptions::default(),
    )
    .expect("failed to open window");

    // Limit to ~60 fps to keep the window responsive while dragging.
    window.set_target_fps(60);

    // Index of the monster currently being dragged, if any.
    let mut dragging: Option<usize> = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
            let (tile_x, tile_y) = pixel_to_tile(mouse_x, mouse_y, TILE_SIZE);

            if window.get_mouse_down(MouseButton::Left) {
                match dragging {
                    Some(index) => {
                        // Continue dragging: try to move the monster to
                        // the tile under the cursor.
                        dungeon.move_monster(index, tile_x, tile_y);
                    }
                    None => {
                        // A new click: start dragging if a monster is
                        // under the cursor.
                        if let Some(index) = dungeon.monster_at(tile_x, tile_y) {
                            dragging = Some(index);
                        }
                    }
                }
            } else {
                dragging = None;
            }
        }

        let (_, _, buffer) = render_to_buffer(&dungeon, TILE_SIZE);
        window
            .update_with_buffer(&buffer, width, height)
            .expect("failed to update window buffer");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_to_tile_converts_correctly() {
        assert_eq!(pixel_to_tile(0.0, 0.0, 24), (0, 0));
        assert_eq!(pixel_to_tile(23.9, 23.9, 24), (0, 0));
        assert_eq!(pixel_to_tile(24.0, 48.0, 24), (1, 2));
        assert_eq!(pixel_to_tile(100.0, 50.0, 24), (4, 2));
    }
}
