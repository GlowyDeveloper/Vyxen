use skrifa::{
    FontRef, MetadataProvider,
    outline::{DrawSettings, OutlinePen},
    prelude::{LocationRef, Size},
};
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicU64, Ordering},
};
use zeno::{Command, Mask, PathBuilder, Transform};

use crate::Resource;

static NEXT_TEXT_ID: AtomicU64 = AtomicU64::new(1);

/// A font resource loaded from a TTF file.
///
/// # Examples
///
/// ```rust, ignore
/// use vyxen_resource::{Font, load_path};
///
/// let font = load_path::<Font>("path/to/font.ttf").unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Font {
    data: Vec<u8>,
    id: u64,
}

impl Resource for Font {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn load(data: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {
            data: data.to_vec(),
            id: NEXT_TEXT_ID.fetch_add(1, Ordering::Relaxed),
        })
    }
}

impl Font {
    /// Returns the unique ID of this font.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Generates a glyph map for the given string and size.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// use vyxen_resource::{Font, load_path};
    ///
    /// let font = load_path::<Font>("path/to/font.ttf").unwrap();
    /// let glyph_map = font.generate_glyph_map("Hello, World!".to_string(), 16.0).unwrap();
    /// ```
    pub fn generate_glyph_map(&self, str: String, size: f32) -> anyhow::Result<GlyphMap> {
        let font = FontRef::new(&self.data)?;
        let charmap = font.charmap();
        let outlines = font.outline_glyphs();
        let px = Size::new(size);
        let location = LocationRef::default();
        let glyph_metrics = font.glyph_metrics(px, location);

        let flip = Transform::scale(1.0, -1.0);

        let mut rasterized: HashMap<char, Rasterized> = HashMap::new();

        for c in str.chars().collect::<HashSet<_>>() {
            let gid = charmap.map(c).unwrap_or_default();
            let advance = glyph_metrics.advance_width(gid).unwrap_or(0.0);

            let Some(outline) = outlines.get(gid) else {
                rasterized.insert(
                    c,
                    Rasterized {
                        bitmap: vec![],
                        w: 0,
                        h: 0,
                        bearing_x: 0.0,
                        bearing_y: 0.0,
                        advance,
                    },
                );
                continue;
            };

            let mut path: Vec<Command> = Vec::new();
            let mut pen = Pen { path: &mut path };
            let settings = DrawSettings::unhinted(px, location);
            outline.draw(settings, &mut pen).ok();

            let (bitmap, placement) = Mask::new(&path).transform(Some(flip)).render();

            if placement.width == 0 || placement.height == 0 {
                rasterized.insert(
                    c,
                    Rasterized {
                        bitmap: vec![],
                        w: 0,
                        h: 0,
                        bearing_x: 0.0,
                        bearing_y: 0.0,
                        advance,
                    },
                );
                continue;
            }

            rasterized.insert(
                c,
                Rasterized {
                    bitmap,
                    w: placement.width,
                    h: placement.height,
                    bearing_x: placement.left as f32,
                    bearing_y: -placement.top as f32,
                    advance,
                },
            );
        }

        let atlas_width: u32 = 512;
        let mut cursor_x = 1;
        let mut cursor_y = 1;
        let mut shelf_height = 0;
        let mut atlas_height_needed = 1;

        let mut glyphs = HashMap::new();
        let mut placements = Vec::new();

        for (c, g) in &rasterized {
            if cursor_x + g.w + 1 > atlas_width {
                cursor_x = 1;
                cursor_y += shelf_height + 1;
                shelf_height = 0;
            }
            placements.push((*c, cursor_x, cursor_y));
            shelf_height = shelf_height.max(g.h);
            cursor_x += g.w + 1;
            atlas_height_needed = atlas_height_needed.max(cursor_y + shelf_height + 1);
        }

        let atlas_height = atlas_height_needed.max(1);
        let mut rgba = vec![0u8; (atlas_width * atlas_height * 4) as usize];

        for (c, px_x, px_y) in placements {
            let g = &rasterized[&c];
            for y in 0..g.h {
                for x in 0..g.w {
                    let coverage = g.bitmap[(y * g.w + x) as usize];
                    let ax = px_x + x;
                    let ay = px_y + y;
                    let idx = ((ay * atlas_width + ax) * 4) as usize;

                    rgba[idx] = 255;
                    rgba[idx + 1] = 255;
                    rgba[idx + 2] = 255;
                    rgba[idx + 3] = coverage;
                }
            }

            glyphs.insert(
                c,
                GlyphRect {
                    uv_min: (
                        px_x as f32 / atlas_width as f32,
                        px_y as f32 / atlas_height as f32,
                    ),
                    uv_max: (
                        (px_x + g.w) as f32 / atlas_width as f32,
                        (px_y + g.h) as f32 / atlas_height as f32,
                    ),
                    width: g.w as f32,
                    height: g.h as f32,
                    bearing_x: g.bearing_x,
                    bearing_y: g.bearing_y,
                    advance: g.advance,
                },
            );
        }

        Ok(GlyphMap {
            rgba,
            atlas_width,
            atlas_height,
            glyphs,
        })
    }
}

/// A single glyph's position/size within the atlas.
#[derive(Debug, Clone, Copy)]
pub struct GlyphRect {
    uv_min: (f32, f32),
    uv_max: (f32, f32),
    width: f32,
    height: f32,
    bearing_x: f32,
    bearing_y: f32,
    advance: f32,
}

impl GlyphRect {
    pub fn uv_min(&self) -> (f32, f32) {
        self.uv_min
    }
    pub fn uv_max(&self) -> (f32, f32) {
        self.uv_max
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn bearing_x(&self) -> f32 {
        self.bearing_x
    }
    pub fn bearing_y(&self) -> f32 {
        self.bearing_y
    }
    pub fn advance(&self) -> f32 {
        self.advance
    }
}

/// An atlas for the glyphs needed to draw one string at one size.
///
/// This gotten from `Font::generate_glyph_map`.
#[derive(Debug, Clone)]
pub struct GlyphMap {
    rgba: Vec<u8>,
    atlas_width: u32,
    atlas_height: u32,
    glyphs: HashMap<char, GlyphRect>,
}

impl GlyphMap {
    pub fn atlas_width(&self) -> u32 {
        self.atlas_width
    }
    pub fn atlas_height(&self) -> u32 {
        self.atlas_height
    }
    pub fn glyphs(&self) -> &HashMap<char, GlyphRect> {
        &self.glyphs
    }
    pub fn rgba(&self) -> &Vec<u8> {
        &self.rgba
    }
}

struct Pen<'a> {
    path: &'a mut Vec<Command>,
}

impl<'a> OutlinePen for Pen<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.move_to((x, y));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to((x, y));
    }
    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.path.quad_to((cx, cy), (x, y));
    }
    fn curve_to(&mut self, c0x: f32, c0y: f32, c1x: f32, c1y: f32, x: f32, y: f32) {
        self.path.curve_to((c0x, c0y), (c1x, c1y), (x, y));
    }
    fn close(&mut self) {
        self.path.close();
    }
}

struct Rasterized {
    bitmap: Vec<u8>,
    w: u32,
    h: u32,
    bearing_x: f32,
    bearing_y: f32,
    advance: f32,
}
