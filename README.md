# gemboard
Map application for the gemboard rpg, when playing with a tabletop TV

## Features

- **Realistic Top-Down Tile Graphics**: Renders PNG tilesets for walls and floor tiles instead of solid colors.
- **Monster Sprite Rendering**: Displays top-down monster sprites with alpha channel compositing/blending.
- **Asset Management System**: Organized directory structure (`assets/tiles` and `assets/monsters`) with CC0 realistic artwork included and instructions for adding custom art.
- **Interactive Map**: Drag-and-drop monster movement using mouse interaction.

## Running

This is a Rust application built with Cargo. To display a simple example
dungeon map in a window:

```sh
cargo run
```

Press `Escape` or close the window to exit.

## Assets & Custom Art

Top-down realistic tiles and monster sprites are stored in the `assets/` directory:
- `assets/tiles/`: PNG files for `wall.png` and `floor.png`.
- `assets/monsters/`: PNG monster sprites with transparency (`goblin.png`, `orc.png`, `skeleton.png`, etc.).

For licensing details and instructions on adding new tiles or monster tokens, see [`assets/README.md`](assets/README.md).

## Testing

```sh
cargo test
```
