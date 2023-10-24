// https://www.figma.com/community/plugin/1105513882835626049

use gpui2::{SharedString, Hsla, hsla};
use palette::{Hsla as PaletteHsla};


pub fn palette_hsla_to_hsla(palette_hsla: PaletteHsla) -> Hsla {
    let hue = palette_hsla.color.hue.to_positive_degrees() / 360.0;
    let saturation = palette_hsla.color.saturation;
    let lightness = palette_hsla.color.lightness;
    let alpha = palette_hsla.alpha;

    hsla(hue as f32, saturation as f32, lightness as f32, alpha as f32)
}

pub fn hsla_to_palette_hsla(hsla: Hsla) -> PaletteHsla {
    let hue = hsla.h * 360.0;
    let saturation = hsla.s as f32;
    let lightness = hsla.l as f32;
    let alpha = hsla.a as f32;

    PaletteHsla::new(hue, saturation, lightness, alpha)
}

#[derive(Debug, Clone)]
pub enum ColorScale {
    Gray,
    Mauve,
    Slate,
    Sage,
    Olive,
    Sand,
    Gold,
    Bronze,
    Brown,
    Yellow,
    Amber,
    Orange,
    Tomato,
    Red,
    Ruby,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    Indigo,
    Blue,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Lime,
    Mint,
    Sky,
    Black,
    White,
    Custom(String),
}

impl ColorScale {
    fn name(&self) -> String {
        let label = match *self {
            ColorScale::Gray => "Gray",
            ColorScale::Mauve => "Mauve",
            ColorScale::Slate => "Slate",
            ColorScale::Sage => "Sage",
            ColorScale::Olive => "Olive",
            ColorScale::Sand => "Sand",
            ColorScale::Gold => "Gold",
            ColorScale::Bronze => "Bronze",
            ColorScale::Brown => "Brown",
            ColorScale::Yellow => "Yellow",
            ColorScale::Amber => "Amber",
            ColorScale::Orange => "Orange",
            ColorScale::Tomato => "Tomato",
            ColorScale::Red => "Red",
            ColorScale::Ruby => "Ruby",
            ColorScale::Crimson => "Crimson",
            ColorScale::Pink => "Pink",
            ColorScale::Plum => "Plum",
            ColorScale::Purple => "Purple",
            ColorScale::Violet => "Violet",
            ColorScale::Iris => "Iris",
            ColorScale::Indigo => "Indigo",
            ColorScale::Blue => "Blue",
            ColorScale::Cyan => "Cyan",
            ColorScale::Teal => "Teal",
            ColorScale::Jade => "Jade",
            ColorScale::Green => "Green",
            ColorScale::Grass => "Grass",
            ColorScale::Lime => "Lime",
            ColorScale::Mint => "Mint",
            ColorScale::Sky => "Sky",
            ColorScale::Black => "Black",
            ColorScale::White => "White",
            ColorScale::Custom(ref name) => name
        };
        label.into()
    }

}

#[derive(Debug, Clone)]
pub struct Color {
    pub name: String,
    pub hsla: PaletteHsla,
    pub scale: ColorScale,
}

impl Color {
    pub fn new<S: Into<String>>(name: S, hsla: PaletteHsla, scale: ColorScale) -> Self {
        Self {
            name: name.into(),
            hsla,
            scale,
        }
    }

    pub fn palette_hsla(&self) -> PaletteHsla {
        self.hsla
    }

    pub fn hsla(&self) -> Hsla {
        palette_hsla_to_hsla(self.hsla)
    }

    pub fn scale(&self) -> ColorScale {
        self.scale.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Scale {
    pub name: ColorScale,
    pub steps: [Color; 12],
}

impl Scale {
    pub fn new(name: ColorScale, hues: [PaletteHsla; 12]) -> Self {
        let steps = hues.iter().enumerate().map(|(i, &hue)| {
            let step = i + 1;
            let color_name = format!("{:?} {}", name, step);
            let scale = name.clone();
            Color::new(color_name, hue, scale)
        }).collect::<Vec<Color>>();

        Self {
            name,
            steps: match steps.try_into() {
                Ok(array) => array,
                Err(vec) => panic!("Expected a Vec of length 12, but it was {}", vec.len()),
            },
        }
    }

    pub fn by_step(&self, step: usize) -> Option<Color> {
        self.steps.get(step - 1).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct CustomScale {
    pub name: String,
    pub steps: [Color; 12],
}

impl CustomScale {
    pub fn builder(name: &str) -> CustomScaleBuilder {
        CustomScaleBuilder::new(name)
    }

    pub fn closest_scale_index(hsla_color: PaletteHsla) -> usize {
       let default_scale = CustomScale::default();
       let mut best_match = 0;
       let mut best_score = f32::MIN;

       for (index, scale_color) in default_scale.steps.iter().enumerate() {
          let lum_diff = (hsla_color.color.lightness - scale_color.hsla.color.lightness).abs();
          let sat_diff = (hsla_color.color.saturation - scale_color.hsla.color.saturation).abs();
          // Essentially magic numbers, Luminoisty is more visually important to the scale
          // than saturation so we weight it higher
          let score = (5.0 * lum_diff) + (3.0 * sat_diff);

          if score > best_score {
              best_score = score;
              best_match = index;
          }
       }
       best_match
    }
}

impl Default for CustomScale {
    fn default() -> Self {
        let hues: [PaletteHsla; 12] = [
            hsla_to_palette_hsla(hsla(0.0, 1.00, 0.99, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 1.00, 0.98, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 0.90, 0.96, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 1.00, 0.93, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 1.00, 0.90, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 0.94, 0.87, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 0.77, 0.81, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 0.70, 0.74, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 0.75, 0.59, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 0.69, 0.55, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 0.65, 0.49, 1.0)),
            hsla_to_palette_hsla(hsla(0.0, 0.63, 0.24, 1.0)),
        ];
        Self::builder("Untitled Custom Scale").hues(hues).build()
    }
}

#[derive(Debug, Clone)]
pub struct CustomScaleBuilder {
    pub name: String,
    pub hues: Option<[PaletteHsla; 12]>,
}

impl CustomScaleBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            hues: None,
        }
    }

    pub fn hues(mut self, hues: [PaletteHsla; 12]) -> Self {
        self.hues = Some(hues);
        self
    }

    pub fn build(self) -> CustomScale {
        let hues = self.hues.expect("Hues not set");

        let steps = hues.iter().enumerate().map(|(i, &hue)| {
            let step = i + 1;
            let color_name = format!("{} {}", self.name, step);
            let scale = ColorScale::Custom(self.name.clone());
            Color::new(color_name, hue, scale)
        }).collect::<Vec<Color>>();

        CustomScale {
            name: self.name,
            steps: match steps.try_into() {
                Ok(array) => array,
                Err(vec) => panic!("Expected a Vec of length 12, but it was {}", vec.len()),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScaleEnum {
    Standard(Scale),
    Custom(CustomScale),
}

#[derive(Debug, Clone)]
pub struct ThemeScales {
    pub name: String,
    pub scale_enums: Vec<ScaleEnum>,
}

impl ThemeScales {
    pub fn builder(name: &str) -> ThemeScalesBuilder {
        ThemeScalesBuilder::new(name)
    }
}

#[derive(Debug, Clone)]
pub struct ThemeScalesBuilder {
    pub name: String,
    pub scales: Vec<ScaleEnum>,
}

impl ThemeScalesBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            scales: Vec::new(),
        }
    }

    pub fn add_scale(mut self, scale: Scale) -> Self {
        self.scales.push(ScaleEnum::Standard(scale));
        self
    }

    pub fn add_custom_scale(mut self, custom_scale: CustomScale) -> Self {
        self.scales.push(ScaleEnum::Custom(custom_scale));
        self
    }

    pub fn build(self) -> ThemeScales {
        ThemeScales {
            name: self.name,
            scale_enums: self.scales,
        }
    }
}
