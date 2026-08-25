mod map;

use map::{DungeonMap, render_to_buffer};
use minifb::{Key, Window, WindowOptions};

const TILE_SIZE: usize = 24;

fn main() {
    let dungeon = DungeonMap::example();
    let (width, height, buffer) = render_to_buffer(&dungeon, TILE_SIZE);

    let mut window = Window::new(
        "Gemboard - Dungeon Map",
        width,
        height,
        WindowOptions::default(),
    )
    .expect("failed to open window");

    // Limit to ~60 fps; the map is static so this just keeps the window
    // responsive without redrawing unnecessarily.
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, width, height)
            .expect("failed to update window buffer");
    }
}
