use crate::{
    color::{rgb, Hsla},
    ThemeColors,
};

use super::scale_sl;

pub struct OneThemes {
    pub default: OnePalette,
    pub dark: OnePalette,
}

#[derive(Clone, Copy, Debug)]
pub struct OnePalette {
    pub base: Hsla,
    pub surface: Hsla,
    pub overlay: Hsla,
    pub muted: Hsla,
    pub subtle: Hsla,
    pub text: Hsla,
    pub love: Hsla,
    pub gold: Hsla,
    pub rose: Hsla,
    pub pine: Hsla,
    pub foam: Hsla,
    pub iris: Hsla,
    pub highlight_low: Hsla,
    pub highlight_med: Hsla,
    pub highlight_high: Hsla,
}

impl OnePalette {
    // TODO: Change these to actual one dark :(
    pub fn default() -> OnePalette {
        OnePalette {
            base: rgb(0x191724),
            surface: rgb(0x1f1d2e),
            overlay: rgb(0x26233a),
            muted: rgb(0x6e6a86),
            subtle: rgb(0x908caa),
            text: rgb(0xe0def4),
            love: rgb(0xeb6f92),
            gold: rgb(0xf6c177),
            rose: rgb(0xebbcba),
            pine: rgb(0x31748f),
            foam: rgb(0x9ccfd8),
            iris: rgb(0xc4a7e7),
            highlight_low: rgb(0x21202e),
            highlight_med: rgb(0x403d52),
            highlight_high: rgb(0x524f67),
        }
    }

    pub fn dark() -> OnePalette {
        Self::default()
    }
}

pub fn dark() -> ThemeColors {
    theme_colors(&OnePalette::dark())
}

fn theme_colors(p: &OnePalette) -> ThemeColors {
    ThemeColors {
        base: scale_sl(p.base, (0.8, 0.8), (1.2, 1.2)),
        surface: scale_sl(p.surface, (0.8, 0.8), (1.2, 1.2)),
        overlay: scale_sl(p.overlay, (0.8, 0.8), (1.2, 1.2)),
        muted: scale_sl(p.muted, (0.8, 0.8), (1.2, 1.2)),
        subtle: scale_sl(p.subtle, (0.8, 0.8), (1.2, 1.2)),
        text: scale_sl(p.text, (0.8, 0.8), (1.2, 1.2)),
        highlight_low: scale_sl(p.highlight_low, (0.8, 0.8), (1.2, 1.2)),
        highlight_med: scale_sl(p.highlight_med, (0.8, 0.8), (1.2, 1.2)),
        highlight_high: scale_sl(p.highlight_high, (0.8, 0.8), (1.2, 1.2)),
        success: scale_sl(p.foam, (0.8, 0.8), (1.2, 1.2)),
        warning: scale_sl(p.gold, (0.8, 0.8), (1.2, 1.2)),
        error: scale_sl(p.love, (0.8, 0.8), (1.2, 1.2)),
        inserted: scale_sl(p.foam, (0.8, 0.8), (1.2, 1.2)),
        deleted: scale_sl(p.love, (0.8, 0.8), (1.2, 1.2)),
        modified: scale_sl(p.rose, (0.8, 0.8), (1.2, 1.2)),
    }
}
