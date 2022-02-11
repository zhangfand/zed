use crate::{
    fonts::{FontId, GlyphId},
    geometry::{
        rect::{RectF, RectI},
        vector::Vector2F,
    },
    platform,
    text_layout::{LineLayout, RunStyle},
};
use std::sync::Arc;

pub struct FontSystem;

impl FontSystem {
    pub fn new() -> Self {
        FontSystem
    }
}

impl platform::FontSystem for FontSystem {
    fn add_fonts(&self, fonts: &[Arc<Vec<u8>>]) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn load_family(&self, name: &str) -> anyhow::Result<Vec<FontId>> {
        unimplemented!()
    }

    fn select_font(&self, font_ids: &[FontId], properties: &Properties) -> anyhow::Result<FontId> {
        unimplemented!()
    }

    fn font_metrics(&self, font_id: FontId) -> Metrics {
        unimplemented!()
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<RectF> {
        unimplemented!()
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Vector2F> {
        unimplemented!()
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        unimplemented!()
    }

    fn rasterize_glyph(
        &self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        subpixel_shift: Vector2F,
        scale_factor: f32,
    ) -> Option<(RectI, Vec<u8>)> {
        unimplemented!()
    }

    fn layout_line(&self, text: &str, font_size: f32, runs: &[(usize, RunStyle)]) -> LineLayout {
        unimplemented!()
    }

    fn wrap_line(&self, text: &str, font_id: FontId, font_size: f32, width: f32) -> Vec<usize> {
        unimplemented!()
    }
}
