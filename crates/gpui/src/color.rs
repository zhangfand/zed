use crate::json::ToJson;
use anyhow::{anyhow, bail, Result};
use pathfinder_color::{ColorF, ColorU};
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use serde_json::json;
use std::{
    borrow::Cow,
    fmt,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Color(ColorU);

impl Color {
    pub fn transparent_black() -> Self {
        Self(ColorU::transparent_black())
    }

    pub fn black() -> Self {
        Self(ColorU::black())
    }

    pub fn white() -> Self {
        Self(ColorU::white())
    }

    pub fn red() -> Self {
        Self(ColorU::from_u32(0xff0000ff))
    }

    pub fn green() -> Self {
        Self(ColorU::from_u32(0x00ff00ff))
    }

    pub fn blue() -> Self {
        Self(ColorU::from_u32(0x0000ffff))
    }

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(ColorU::new(r, g, b, a))
    }

    pub fn from_u32(rgba: u32) -> Self {
        Self(ColorU::from_u32(rgba))
    }

    pub fn blend(source: Color, dest: Color) -> Color {
        // Skip blending if we don't need it.
        if source.a == 255 {
            return source;
        } else if source.a == 0 {
            return dest;
        }

        let source = source.0.to_f32();
        let dest = dest.0.to_f32();

        let a = source.a() + (dest.a() * (1. - source.a()));
        let r = ((source.r() * source.a()) + (dest.r() * dest.a() * (1. - source.a()))) / a;
        let g = ((source.g() * source.a()) + (dest.g() * dest.a() * (1. - source.a()))) / a;
        let b = ((source.b() * source.a()) + (dest.b() * dest.a() * (1. - source.a()))) / a;

        Self(ColorF::new(r, g, b, a).to_u8())
    }

    pub fn fade_out(&mut self, fade: f32) {
        let fade = fade.clamp(0., 1.);
        self.0.a = (self.0.a as f32 * (1. - fade)) as u8;
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let literal: Cow<str> = Deserialize::deserialize(deserializer)?;
        if let Some(digits) = literal.strip_prefix('#') {
            if let Ok(value) = u32::from_str_radix(digits, 16) {
                if digits.len() == 6 {
                    return Ok(Color::from_u32((value << 8) | 0xFF));
                } else if digits.len() == 8 {
                    return Ok(Color::from_u32(value));
                }
            }
        }

        // if let Some()

        Err(de::Error::invalid_value(
            Unexpected::Str(literal.as_ref()),
            &"#RRGGBB[AA]",
        ))
    }
}

impl ToJson for Color {
    fn to_json(&self) -> serde_json::Value {
        json!(format!(
            "0x{:x}{:x}{:x}{:x}",
            self.0.r, self.0.g, self.0.b, self.0.a
        ))
    }
}

impl Deref for Color {
    type Target = ColorU;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn parse_color(source: &mut &str) -> Result<Color> {
    if source.starts_with("rgba(") {
        parse_rgba(source)
    } else if source.starts_with("rgb(") {
        parse_rgb(source)
    } else if source.starts_with("#") {
        parse_hex(source)
    } else {
        bail!("invalid color format");
    }
}

fn parse_rgb(source: &mut &str) -> Result<Color> {
    parse_prefix(source, "rgb(")?;
    parse_whitespace(source);
    let r = parse_int(source)?;
    parse_comma(source)?;
    let g = parse_int(source)?;
    parse_comma(source)?;
    let b = parse_int(source)?;
    parse_whitespace(source);
    parse_prefix(source, ")")?;

    Ok(Color::new(r, g, b, 255))
}

fn parse_rgba(source: &mut &str) -> Result<Color> {
    parse_prefix(source, "rgba(")?;
    parse_whitespace(source);

    let r;
    let g;
    let b;
    if let Ok(color) = parse_color(source) {
        r = color.r;
        g = color.g;
        b = color.b;
    } else {
        r = parse_int(source)?;
        parse_comma(source)?;
        g = parse_int(source)?;
        parse_comma(source)?;
        b = parse_int(source)?;
    }

    parse_comma(source)?;
    let a = parse_float(source)?.clamp(0., 1.);
    parse_whitespace(source);
    parse_prefix(source, ")")?;

    Ok(Color::new(r, g, b, (a * 255.).round() as u8))
}

fn parse_hex(source: &mut &str) -> Result<Color> {
    parse_prefix(source, "#")?;

    let mut digits = String::new();
    loop {
        match source.chars().next() {
            Some(c) if c.is_digit(16) => {
                digits.push(c);
                *source = &source[1..];
            }
            Some(_) | None => break,
        }
    }

    let mut value =
        u32::from_str_radix(&digits, 16).map_err(|_| anyhow!("expected a hexadecimal number"))?;

    if digits.len() == 6 {
        value = (value << 8) | 0xFF;
    } else if digits.len() != 8 {
        bail!("expected 6 or 8 hexadecimal digits");
    }

    *source = &source[digits.len()..];

    Ok(Color::from_u32(value))
}

fn parse_int(source: &mut &str) -> Result<u8> {
    let mut digits = String::new();
    loop {
        match source.chars().next() {
            Some(c) if c.is_digit(10) => {
                digits.push(c);
                *source = &source[1..];
            }
            Some(_) | None => break,
        }
    }
    digits.parse().map_err(|_| anyhow!("expected an integer"))
}

fn parse_float(source: &mut &str) -> Result<f32> {
    let mut digits = String::new();
    loop {
        match source.chars().next() {
            Some(c) if c.is_digit(10) || c == '.' => {
                digits.push(c);
                *source = &source[1..];
            }
            Some(_) | None => break,
        }
    }
    digits.parse().map_err(|_| anyhow!("expected a float"))
}

fn parse_comma(source: &mut &str) -> Result<()> {
    parse_whitespace(source);
    parse_prefix(source, ",")?;
    parse_whitespace(source);
    Ok(())
}

fn parse_whitespace(source: &mut &str) {
    loop {
        if let Some(' ') = source.chars().next() {
            *source = &source[1..];
        } else {
            break;
        }
    }
}

fn parse_prefix(source: &mut &str, prefix: &str) -> Result<()> {
    *source = source
        .strip_prefix(prefix)
        .ok_or_else(|| anyhow!("expected \"{}\"", prefix))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rgb() {
        assert_eq!(
            parse_rgb(&mut "rgb(64, 64, 64)").unwrap(),
            Color::new(64, 64, 64, 255)
        )
    }

    #[test]
    fn test_parse_rgba() {
        assert_eq!(
            parse_rgba(&mut "rgba(64, 64, 64, 0.5)").unwrap(),
            Color::new(64, 64, 64, 128)
        );
        assert_eq!(
            parse_rgba(&mut "rgba(rgb(64, 64, 64), 0.5)").unwrap(),
            Color::new(64, 64, 64, 128)
        );
        assert_eq!(
            parse_rgba(&mut "rgba(rgba(64, 64, 64, 1.0), 0.5)").unwrap(),
            Color::new(64, 64, 64, 128)
        );
        // TODO!
        // assert_eq!(
        //     parse_rgba(&mut "rgba(#646464, 0.5)").unwrap(),
        //     Color::new(64, 64, 64, 128)
        // );
        // assert_eq!(
        //     parse_rgba(&mut "rgba(#646464FF, 0.5)").unwrap(),
        //     Color::new(64, 64, 64, 128)
        // );
    }
}
