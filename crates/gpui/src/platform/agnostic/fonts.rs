use crate::{
    fonts::{FontId, GlyphId, Metrics, Properties},
    geometry::{
        rect::{RectF, RectI},
        transform2d::Transform2F,
        vector::{Vector2F, Vector2I},
    },
    platform,
    text_layout::{RunStyle, LineLayout, Run, Glyph},
};
use font_kit::{
    canvas::{Canvas, Format, RasterizationOptions}, 
    handle::Handle, 
    hinting::HintingOptions, 
    source::SystemSource,
    sources::mem::MemSource,
};
use parking_lot::RwLock;
use swash::{
    Attributes, CacheKey, Charmap, FontRef,
    shape::{Direction, ShapeContext},
    scale::{ScaleContext, StrikeWith},
    text::Script,
};
use std::{
    path::Path,
    cell::RefCell,
    sync::Arc
};

thread_local! {
    static SYSTEM_SOURCE: SystemSource = SystemSource::new();
}

struct SwashFont {
    data: Arc<Vec<u8>>,
    offset: u32,
    key: CacheKey,
}

impl SwashFont {
    fn from_handle(handle: &Handle) -> Option<Self> {
        match handle {
            Handle::Path { path, font_index } => 
                Self::from_file(path, *font_index as usize),
            Handle::Memory { bytes, font_index } => 
                Self::from_data(bytes.clone(), *font_index as usize),
        }
    }

    fn from_file(path: &Path, index: usize) -> Option<Self> {
        let data = std::fs::read(path).ok()?;
        Self::from_data(Arc::new(data), index)
    }

    fn from_data(data: Arc<Vec<u8>>, index: usize) -> Option<Self> {
        let font = FontRef::from_index(&data, index)?;
        let (offset, key) = (font.offset, font.key);
        Some(Self { data, offset, key })
    }

    fn as_ref(&self) -> FontRef {
        FontRef {
            data: &self.data,
            offset: self.offset,
            key: self.key,
        }
    }
}

pub struct FontSystem(Arc<RwLock<FontSystemState>>);

struct FontSystemState {
    memory_source: MemSource,
    shape_context: ShapeContext,
    scale_context: ScaleContext,
    fonts: Vec<SwashFont>,
}

impl FontSystem {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(FontSystemState {
            memory_source: MemSource::empty(),
            shape_context: ShapeContext::new(),
            scale_context: ScaleContext::new(),
            fonts: Vec::new(),
        })))
    }
}

impl platform::FontSystem for FontSystem {
    fn add_fonts(&self, fonts: &[Arc<Vec<u8>>]) -> anyhow::Result<()> {
        self.0.write().add_fonts(fonts)
    }

    fn load_family(&self, name: &str) -> anyhow::Result<Vec<FontId>> {
        self.0.write().load_family(name)
    }

    fn select_font(&self, font_ids: &[FontId], properties: &Properties) -> anyhow::Result<FontId> {
        self.0.read().select_font(font_ids, properties)
    }

    fn font_metrics(&self, font_id: FontId) -> Metrics {
        self.0.read().font_metrics(font_id)
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<RectF> {
        self.0.write().typographic_bounds(font_id, glyph_id)
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Vector2F> {
        self.0.read().advance(font_id, glyph_id)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.0.read().glyph_for_char(font_id, ch)
    }

    fn rasterize_glyph(
        &self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        subpixel_shift: Vector2F,
        scale_factor: f32,
    ) -> Option<(RectI, Vec<u8>)> {
        self.0
            .write()
            .rasterize_glyph(font_id, font_size, glyph_id, subpixel_shift, scale_factor)
    }

    fn layout_line(&self, text: &str, font_size: f32, runs: &[(usize, RunStyle)]) -> LineLayout {
        // TODO: Had to change this to write because swash needs to modify the contexts.
        self.0.write().layout_line(text, font_size, runs)
    }

    fn wrap_line(&self, text: &str, font_id: FontId, font_size: f32, width: f32) -> Vec<usize> {
        self.0.read().wrap_line(text, font_id, font_size, width)
    }
}

impl FontSystemState {
    fn add_fonts(&mut self, fonts: &[Arc<Vec<u8>>]) -> anyhow::Result<()> {
        self.memory_source.add_fonts(
            fonts
                .iter()
                .map(|bytes| Handle::from_memory(bytes.clone(), 0)),
        )?;
        Ok(())
    }

    fn load_family(&mut self, name: &str) -> anyhow::Result<Vec<FontId>> {
        let mut font_ids = Vec::new();

        let family = self
            .memory_source
            .select_family_by_name(name)
            .or_else(|_| SYSTEM_SOURCE.with(|ss| ss.select_family_by_name(name)))?;
        for font in family.fonts() {
            let font = SwashFont::from_handle(font)
                .ok_or(anyhow::anyhow!("Failed to load font"))?;
            font_ids.push(FontId(self.fonts.len()));
            self.fonts.push(font);
        }
        Ok(font_ids)
    }

    fn select_font(&self, font_ids: &[FontId], properties: &Properties) -> anyhow::Result<FontId> {
        // TODO: Come up with way to select swash font from properties
        Ok(font_ids[0])
        // let candidates = font_ids
        //     .iter()
        //     .map(|font_id| self.fonts[font_id.0].properties())
        //     .collect::<Vec<_>>();
        // let idx = font_kit::matching::find_best_match(&candidates, properties)?;
        // Ok(font_ids[idx])
    }

    fn font_metrics(&self, font_id: FontId) -> Metrics {
        let swash_metrics = self.fonts[font_id.0].as_ref().metrics(&[]);

        Metrics {
            units_per_em: swash_metrics.units_per_em as u32,
            ascent: swash_metrics.ascent,
            descent: swash_metrics.descent,
            line_gap: 0.0, // Not found in swash. May need to recreate it
            underline_position: swash_metrics.underline_offset,
            underline_thickness: swash_metrics.stroke_size,
            cap_height: swash_metrics.cap_height,
            x_height: swash_metrics.x_height,
            bounding_box: Default::default(), // Not found in swash. May need to recreate it
        }
    }

    fn typographic_bounds(&mut self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<RectF> {
        let font = self.fonts[font_id.0].as_ref();
        let mut scaler = self.scale_context.builder(font).build();
        let outline = scaler.scale_outline(glyph_id as u16).ok_or(anyhow::anyhow!("Failed to scale outline"))?;
        let bounds = outline.bounds();
        Ok(RectF::new(
                Vector2F::new(bounds.min.x, bounds.min.y), 
                Vector2F::new(bounds.max.x, bounds.max.y)))
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Vector2F> {
        let font = self.fonts[font_id.0].as_ref();
        let glyph_metrics = font.glyph_metrics(&[]);

        Ok(Vector2F::new(
            glyph_metrics.advance_width(glyph_id as u16),
            glyph_metrics.advance_height(glyph_id as u16)))
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        let font = self.fonts[font_id.0].as_ref();
        Some(font.charmap().map(ch) as u32)
    }

    fn rasterize_glyph(
        &mut self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        subpixel_shift: Vector2F,
        scale_factor: f32,
    ) -> Option<(RectI, Vec<u8>)> {
        let font = self.fonts[font_id.0].as_ref();
        let mut scaler = self.scale_context.builder(font)
            .size(font_size)
            .build();

        let image = scaler.scale_bitmap(glyph_id as u16, StrikeWith::BestFit)?;

        Some((
            RectI::new(Vector2I::new(image.placement.left, image.placement.top),
                       Vector2I::new(image.placement.width as i32, image.placement.height as i32)), 
            image.data
        ))
    }

    fn layout_line(&mut self, text: &str, font_size: f32, runs: &[(usize, RunStyle)]) -> LineLayout {
        // Merge runs with the same font
        let last_run: RefCell<Option<(usize, FontId)>> = Default::default();
        let font_runs = runs
            .iter()
            .filter_map(|(len, style)| {
                let mut last_run = last_run.borrow_mut();
                if let Some((last_len, last_font_id)) = last_run.as_mut() {
                    if style.font_id == *last_font_id {
                        *last_len += *len;
                        None
                    } else {
                        let result = (*last_len, *last_font_id);
                        *last_len = *len;
                        *last_font_id = style.font_id;
                        Some(result)
                    }
                } else {
                    *last_run = Some((*len, style.font_id));
                    None
                }
            })
            .chain(std::iter::from_fn(|| last_run.borrow_mut().take()));

        // Get font text per run
        let mut previous_index: isize = -1;
        let font_run_slices = font_runs
            .map(|(len, font_id)| {
                let start = (previous_index + 1) as usize;
                let end = start + len;
                previous_index = end as isize;
                ((start..=end), font_id)
            });

        // Shape each run into positioned glyphs
        let mut completed_runs = Vec::new();
        let mut current_advance = 0.0;
        let mut max_ascent = f32::MIN;
        let mut min_descent = f32::MAX;
        for (run_range, font_id) in font_run_slices {
            let run_text = &text[run_range.clone()];
            let font_ref = self.fonts[font_id.0].as_ref();

            let mut shaper = self.shape_context.builder(font_ref)
                .script(Script::Latin)
                .size(font_size)
                .build();

            let metrics = shaper.metrics();
            max_ascent = max_ascent.max(metrics.ascent);
            min_descent = min_descent.min(metrics.descent);

            shaper.add_str(run_text);

            let mut glyphs = Vec::new();
            let mut next_advance = current_advance;
            shaper.shape_with(|glyph_cluster| {
                for (glyph, source_range) in glyph_cluster.glyphs.iter().zip(glyph_cluster.components.iter()) {
                    let glyph_x = glyph.x + current_advance;
                    next_advance = next_advance.max(glyph_x + glyph.advance);
                    glyphs.push(Glyph {
                        id: glyph.id as u32,
                        position: Vector2F::new(glyph_x, glyph.y),
                        index: source_range.start as usize
                    });
                }
            });
            completed_runs.push(Run {
                font_id,
                glyphs,
            });
            current_advance = next_advance;
        }

        LineLayout {
            width: current_advance,
            ascent: max_ascent,
            descent: min_descent,
            runs: completed_runs,
            len: text.len(),
            font_size,
        }
    }

    fn wrap_line(&self, text: &str, font_id: FontId, font_size: f32, width: f32) -> Vec<usize> {
        // TODO: Use swash's analyze to break lines
        Vec::new()
    }
}
