// https://www.figma.com/community/plugin/1105513882835626049
//
// use gpui2::{Hsla, hsla};

// pub enum ThemeAppearance {
//     Light,
//     Dark,
// }

// pub struct ThemeScales {
//     pub appearance: ThemeAppearance,
//     pub red: ThemeScale,
// }

// pub struct ThemeScale {
//     pub value: [Hsla; 12],
// }

// impl Default for ThemeScale {
//     fn default() -> Self {
//         Self {
//             value: [
//                 hsla(0.0, 1.00, 0.99, 1.0),
//                 hsla(0.0, 1.00, 0.98, 1.0),
//                 hsla(0.0, 0.90, 0.96, 1.0),
//                 hsla(0.0, 1.00, 0.93, 1.0),
//                 hsla(0.0, 1.00, 0.90, 1.0),
//                 hsla(0.0, 0.94, 0.87, 1.0),
//                 hsla(0.0, 0.77, 0.81, 1.0),
//                 hsla(0.0, 0.70, 0.74, 1.0),
//                 hsla(0.0, 0.75, 0.59, 1.0),
//                 hsla(0.0, 0.69, 0.55, 1.0),
//                 hsla(0.0, 0.65, 0.49, 1.0),
//                 hsla(0.0, 0.63, 0.24, 1.0),
//             ],
//         }
//     }
// }

// impl ThemeScale {
//     pub fn new(values: [Hsla; 12]) -> Self {
//         Self {
//             value: values,
//         }
//     }

//     pub fn value(&mut self, ix: usize, value: Hsla) -> &mut Self {
//         self.value[ix] = value;
//         self
//     }

//     pub fn closest_value(value: Hsla) -> usize {
//         // We want to find the closest value in the scale to the input value.
//         // We can ignore the hue and alpha values. We want to compare the input value's saturation and lightness values to the scale's saturation and lightness values.
//         // Find the closest of each, then weight the results 3:2 in favor of the lightness value.
//     }

//     pub fn build_values_from_hsla(&mut self, value: Hsla) -> Self {
//         let closest = self.closest_value(value);
//         self
//     }
// }

use gpui2::{SharedString, Hsla, hsla};
use palette::{Hsla as PaletteHsla};


pub fn palette_hsla_to_hsla(palette_hsla: PaletteHsla) -> Hsla {
    let hue = palette_hsla.color.hue.to_positive_degrees() / 360.0;
    let saturation = palette_hsla.color.saturation;
    let lightness = palette_hsla.color.lightness;
    let alpha = palette_hsla.alpha;

    hsla(hue as f32, saturation as f32, lightness as f32, alpha as f32)
}

#[derive(Debug, Clone, Copy)]
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
}

impl ColorScale {
    fn name(&self) -> SharedString {
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
        self.scale
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
            let scale = name;
            Color::new(&color_name, hue, scale)
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
pub struct ThemeScales {
    pub name: String,
    pub colors: [Color; 12],
    pub scale: Scale,
}

impl ThemeScales {
    pub fn builder(name: &str) -> ThemeScalesBuilder {
        ThemeScalesBuilder::new(name)
    }
}

#[derive(Debug, Clone)]
pub struct ThemeScalesBuilder {
    pub name: String,
    pub colors: Option<[Color; 12]>,
    pub scale: Option<Scale>,
}

impl ThemeScalesBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            colors: None,
            scale: None,
        }
    }

    pub fn colors(mut self, colors: [Color; 12]) -> Self {
        self.colors = Some(colors);
        self
    }

    pub fn scale(mut self, scale: Scale) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn build(self) -> ThemeScales {
        let scale = self.scale.expect("Scale not set");
        let colors = self.colors.expect("Colors not set");

        ThemeScales {
            name: self.name,
            colors,
            scale,
        }
    }
}
