# Gemboard Art & Asset Management System

This directory contains the graphic assets (tilesets and monster sprites) used by Gemboard for rendering tabletop RPG maps.

## Directory Structure

```
assets/
├── README.md           # Documentation and licensing info
├── tiles/              # Map tile graphics (PNG format)
│   ├── wall.png        # Top-down stone wall tile
│   └── floor.png       # Top-down stone floor tile
└── monsters/           # Monster sprite graphics (PNG format with alpha transparency)
    ├── goblin.png      # Monster sprite for "goblin"
    ├── G.png           # Monster sprite for label "G"
    ├── orc.png         # Monster sprite for "orc"
    ├── O.png           # Monster sprite for label "O"
    ├── skeleton.png    # Monster sprite for "skeleton"
    └── S.png           # Monster sprite for label "S"
```

## Art Style & Specifications

- **Style**: Realistic top-down view designed for tabletop TV map displays.
- **Tiles**: Seamlessly tileable 64x64 PNG images for walls and floor slabs.
- **Monsters**: 64x64 RGBA PNG sprites (tabletop mini tokens with transparent background and drop shadow).

## Licensing & Credits

All bundled art assets in this directory are released under the **CC0 1.0 Universal (Public Domain Dedication)** license. You are free to use, copy, modify, merge, publish, or distribute them without restriction.

### Recommended Sources for Additional Free Art

- **Kenney.nl** (https://kenney.nl): High quality public domain (CC0) top-down and tabletop assets.
- **OpenGameArt.org** (https://opengameart.org): Vast repository of CC0, CC-BY, and OpenGameArt-licensed RPG tilesets and monster tokens.
- **Itch.io** (https://itch.io/game-assets/tag-free): Community-submitted free game assets, sprites, and tabletop tokens.

## How to Add or Change Assets

1. **Adding Custom Tiles**:
   - Save your wall tile PNG image to `assets/tiles/wall.png`.
   - Save your floor tile PNG image to `assets/tiles/floor.png`.
   - Recommended tile dimensions are square (e.g., 32x32, 64x64, 128x128). The application will scale tile graphics to match the grid cell size automatically.

2. **Adding Monster Sprites**:
   - Save a PNG image with a transparent background in `assets/monsters/`.
   - File names correspond to monster labels or names (case-insensitive). For example, a monster created with `Monster::new(x, y, color, "goblin")` matches `assets/monsters/goblin.png`. If labeled `"G"`, save the sprite as `assets/monsters/G.png`.
