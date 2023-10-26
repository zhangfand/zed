use gpui2::{hsla, rgb, Hsla};
use strum::EnumIter;

pub enum DefaultColor {
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

impl DefaultColor {
    pub fn value(&self, index: usize) -> Hsla {
        match self {
            DefaultColor::Gray => ColorScale::from_default(ColorScaleName::Gray).value(index),
            DefaultColor::Mauve => ColorScale::from_default(ColorScaleName::Mauve).value(index),
            DefaultColor::Slate => ColorScale::from_default(ColorScaleName::Slate).value(index),
            DefaultColor::Sage => ColorScale::from_default(ColorScaleName::Sage).value(index),
            DefaultColor::Olive => ColorScale::from_default(ColorScaleName::Olive).value(index),
            DefaultColor::Sand => ColorScale::from_default(ColorScaleName::Sand).value(index),
            DefaultColor::Gold => ColorScale::from_default(ColorScaleName::Gold).value(index),
            DefaultColor::Bronze => ColorScale::from_default(ColorScaleName::Bronze).value(index),
            DefaultColor::Brown => ColorScale::from_default(ColorScaleName::Brown).value(index),
            DefaultColor::Yellow => ColorScale::from_default(ColorScaleName::Yellow).value(index),
            DefaultColor::Amber => ColorScale::from_default(ColorScaleName::Amber).value(index),
            DefaultColor::Orange => ColorScale::from_default(ColorScaleName::Orange).value(index),
            DefaultColor::Tomato => ColorScale::from_default(ColorScaleName::Tomato).value(index),
            DefaultColor::Red => ColorScale::from_default(ColorScaleName::Red).value(index),
            DefaultColor::Ruby => ColorScale::from_default(ColorScaleName::Ruby).value(index),
            DefaultColor::Crimson => ColorScale::from_default(ColorScaleName::Crimson).value(index),
            DefaultColor::Pink => ColorScale::from_default(ColorScaleName::Pink).value(index),
            DefaultColor::Plum => ColorScale::from_default(ColorScaleName::Plum).value(index),
            DefaultColor::Purple => ColorScale::from_default(ColorScaleName::Purple).value(index),
            DefaultColor::Violet => ColorScale::from_default(ColorScaleName::Violet).value(index),
            DefaultColor::Iris => ColorScale::from_default(ColorScaleName::Iris).value(index),
            DefaultColor::Indigo => ColorScale::from_default(ColorScaleName::Indigo).value(index),
            DefaultColor::Blue => ColorScale::from_default(ColorScaleName::Blue).value(index),
            DefaultColor::Cyan => ColorScale::from_default(ColorScaleName::Cyan).value(index),
            DefaultColor::Teal => ColorScale::from_default(ColorScaleName::Teal).value(index),
            DefaultColor::Jade => ColorScale::from_default(ColorScaleName::Jade).value(index),
            DefaultColor::Green => ColorScale::from_default(ColorScaleName::Green).value(index),
            DefaultColor::Grass => ColorScale::from_default(ColorScaleName::Grass).value(index),
            DefaultColor::Lime => ColorScale::from_default(ColorScaleName::Lime).value(index),
            DefaultColor::Mint => ColorScale::from_default(ColorScaleName::Mint).value(index),
            DefaultColor::Sky => ColorScale::from_default(ColorScaleName::Sky).value(index),
            DefaultColor::Black => ColorScale::from_default(ColorScaleName::Black).value(index),
            DefaultColor::White => ColorScale::from_default(ColorScaleName::White).value(index),
        }
    }
}

/// The default set of color scales.
///
/// Create a full scale using `ColorScale::from_default()`.
#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum ColorScaleName {
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

impl ColorScaleName {
    pub fn label(&self) -> String {
        let label = match *self {
            ColorScaleName::Gray => "Gray",
            ColorScaleName::Mauve => "Mauve",
            ColorScaleName::Slate => "Slate",
            ColorScaleName::Sage => "Sage",
            ColorScaleName::Olive => "Olive",
            ColorScaleName::Sand => "Sand",
            ColorScaleName::Gold => "Gold",
            ColorScaleName::Bronze => "Bronze",
            ColorScaleName::Brown => "Brown",
            ColorScaleName::Yellow => "Yellow",
            ColorScaleName::Amber => "Amber",
            ColorScaleName::Orange => "Orange",
            ColorScaleName::Tomato => "Tomato",
            ColorScaleName::Red => "Red",
            ColorScaleName::Ruby => "Ruby",
            ColorScaleName::Crimson => "Crimson",
            ColorScaleName::Pink => "Pink",
            ColorScaleName::Plum => "Plum",
            ColorScaleName::Purple => "Purple",
            ColorScaleName::Violet => "Violet",
            ColorScaleName::Iris => "Iris",
            ColorScaleName::Indigo => "Indigo",
            ColorScaleName::Blue => "Blue",
            ColorScaleName::Cyan => "Cyan",
            ColorScaleName::Teal => "Teal",
            ColorScaleName::Jade => "Jade",
            ColorScaleName::Green => "Green",
            ColorScaleName::Grass => "Grass",
            ColorScaleName::Lime => "Lime",
            ColorScaleName::Mint => "Mint",
            ColorScaleName::Sky => "Sky",
            ColorScaleName::Black => "Black",
            ColorScaleName::White => "White",
            ColorScaleName::Custom(ref name) => name,
        };
        label.into()
    }

    /// Find a color scale by name.
    ///
    /// If the name is not found, a custom color scale will be returned.
    pub fn find_or_custom(name: String) -> Self {
            let standard_name = name.to_lowercase();

            match standard_name.as_str() {
                "gray" => ColorScaleName::Gray,
                "mauve" => ColorScaleName::Mauve,
                "slate" => ColorScaleName::Slate,
                "sage" => ColorScaleName::Sage,
                "olive" => ColorScaleName::Olive,
                "sand" => ColorScaleName::Sand,
                "gold" => ColorScaleName::Gold,
                "bronze" => ColorScaleName::Bronze,
                "brown" => ColorScaleName::Brown,
                "yellow" => ColorScaleName::Yellow,
                "amber" => ColorScaleName::Amber,
                "orange" => ColorScaleName::Orange,
                "tomato" => ColorScaleName::Tomato,
                "red" => ColorScaleName::Red,
                "ruby" => ColorScaleName::Ruby,
                "crimson" => ColorScaleName::Crimson,
                "pink" => ColorScaleName::Pink,
                "plum" => ColorScaleName::Plum,
                "purple" => ColorScaleName::Purple,
                "violet" => ColorScaleName::Violet,
                "iris" => ColorScaleName::Iris,
                "indigo" => ColorScaleName::Indigo,
                "blue" => ColorScaleName::Blue,
                "cyan" => ColorScaleName::Cyan,
                "teal" => ColorScaleName::Teal,
                "jade" => ColorScaleName::Jade,
                "green" => ColorScaleName::Green,
                "grass" => ColorScaleName::Grass,
                "lime" => ColorScaleName::Lime,
                "mint" => ColorScaleName::Mint,
                "sky" => ColorScaleName::Sky,
                "black" => ColorScaleName::Black,
                "white" => ColorScaleName::White,
                _ => ColorScaleName::Custom(name),
            }
        }

    pub fn find_or_none(name: String) -> Option<Self> {
            let standard_name = name.to_lowercase();

            match standard_name.as_str() {
                "gray" => Some(ColorScaleName::Gray),
                "mauve" => Some(ColorScaleName::Mauve),
                "slate" => Some(ColorScaleName::Slate),
                "sage" => Some(ColorScaleName::Sage),
                "olive" => Some(ColorScaleName::Olive),
                "sand" => Some(ColorScaleName::Sand),
                "gold" => Some(ColorScaleName::Gold),
                "bronze" => Some(ColorScaleName::Bronze),
                "brown" => Some(ColorScaleName::Brown),
                "yellow" => Some(ColorScaleName::Yellow),
                "amber" => Some(ColorScaleName::Amber),
                "orange" => Some(ColorScaleName::Orange),
                "tomato" => Some(ColorScaleName::Tomato),
                "red" => Some(ColorScaleName::Red),
                "ruby" => Some(ColorScaleName::Ruby),
                "crimson" => Some(ColorScaleName::Crimson),
                "pink" => Some(ColorScaleName::Pink),
                "plum" => Some(ColorScaleName::Plum),
                "purple" => Some(ColorScaleName::Purple),
                "violet" => Some(ColorScaleName::Violet),
                "iris" => Some(ColorScaleName::Iris),
                "indigo" => Some(ColorScaleName::Indigo),
                "blue" => Some(ColorScaleName::Blue),
                "cyan" => Some(ColorScaleName::Cyan),
                "teal" => Some(ColorScaleName::Teal),
                "jade" => Some(ColorScaleName::Jade),
                "green" => Some(ColorScaleName::Green),
                "grass" => Some(ColorScaleName::Grass),
                "lime" => Some(ColorScaleName::Lime),
                "mint" => Some(ColorScaleName::Mint),
                "sky" => Some(ColorScaleName::Sky),
                "black" => Some(ColorScaleName::Black),
                "white" => Some(ColorScaleName::White),
                _ => None,
            }
        }

    pub fn unwrap_or_default_scale(name: impl Into<String>, v: ThemeVariant) -> ColorScale {
        let name_string: String = name.into();
        let combined_scales = v.scales.into_iter().chain(v.extended_scales.into_iter());
        let scale_in_variant = combined_scales.clone().any(|scale| scale.name == name_string);
        let has_default = Self::find_or_none(name_string).is_some();

        if scale_in_variant {
            combined_scales.clone().find(|scale| scale.name == name_string).unwrap_or(ColorScale::from_default(ColorScaleName::Red))
        } else if has_default {
            Self::find_or_none(name_string).expect("Tried to unwrap scale but something happened").into()
        } else {
            ColorScale::from_default(ColorScaleName::Red)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorScaleStep {
    pub name: String,
    pub value: Hsla,
}

impl Default for ColorScaleStep {
    fn default() -> Self {
        Self {
            name: "Untitled Color".into(),
            value: hsla(0., 0., 0., 0.),
        }
    }
}

impl ColorScaleStep {
    pub fn new<S: Into<String>>(name: S, hsla: Hsla, scale: ColorScaleName) -> Self {
        Self {
            name: name.into(),
            value: hsla,
        }
    }

    pub fn new_name_from_index(index: usize, hsla: Hsla, scale: ColorScaleName) -> String {
        format!("{} {}", scale.label(), index)
    }
}

/// A set of 12 colors that are used throughout the UI. Until further docs are written refer to the
/// [Radix Color Docs](https://www.radix-ui.com/colors/docs/palette-composition/understanding-the-scale).
#[derive(Debug, Clone)]
pub struct ColorScale {
    pub name: String,
    pub scale: ColorScaleName,
    /// Use ColorScale::value() to access the steps.
    ///
    /// Using this directly will result in incorrect values.
    pub steps: [ColorScaleStep; 12],
}

impl ColorScale {
    pub fn new(scale: ColorScaleName, steps: [Hsla; 12]) -> Self {
        let steps = steps
            .iter()
            .enumerate()
            .map(|(i, &hue)| {
                let step = i + 1;
                let color_name = format!("{:?} {}", scale, step);
                let scale = scale.clone();
                ColorScaleStep::new(color_name, hue, scale)
            })
            .collect::<Vec<ColorScaleStep>>();

        Self {
            name: scale.label(),
            scale,
            steps: match steps.try_into() {
                Ok(array) => array,
                Err(vec) => panic!("Expected a Vec of length 12, but it was {}", vec.len()),
            },
        }
    }

    /// Returns the One-based value of the given step in the scale.
    ///
    /// We usally reference steps as 1-12 instead of 0-11, so we
    /// automatically subtract 1 from the index.
    pub fn value(self, index: usize) -> Hsla {
        self.steps[index - 1].value
    }

    /// Returns the One-based step in the scale.
    ///
    /// We usally reference steps as 1-12 instead of 0-11, so we
    /// automatically subtract 1 from the index.
    pub fn step(self, index: usize) -> ColorScaleStep {
        self.steps[index - 1]
    }

    /// Returns the `ColorScale` with the order of it's steps reversed.
    ///
    /// This is useful in scales like `neutral` where the dark and light variant
    /// of a theme often uses a reversed variant of the other's Scale.
    pub fn reversed(&self) -> Self {
        let mut reversed_steps = self.steps;
        reversed_steps.reverse();

        Self {
            name: self.name,
            scale: self.scale,
            steps: reversed_steps,
        }
    }

    // === Color Scale Builders ===

    /// Extrapolate a new color scale from a single HSLA value.
    ///
    /// Note: Scale extrapolation is a bit hacky right now, expect it to improve.
    pub fn from_hsla(input_name: String, input_hsla: Hsla) -> ColorScale {
        let default = ColorScale::default();
        let scale = ColorScaleName::find_or_custom(input_name);
        let name = scale.label();

        let steps_arr = Self::steps_to_hsla(default.steps, name.clone(), input_hsla);
        let steps = Self::hsla_to_steps(steps_arr, name.clone());

        ColorScale {
            name,
            scale,
            steps,
        }
    }

    /// Extrapolate a new color scale from a single hex value.
    ///
    /// Note: Scale extrapolation is a bit hacky right now, expect it to improve.
    pub fn from_hex(input_name: impl Into<String>, hex: &str) -> ColorScale {
        // TODO: gpui probably has better utilities for doing this conversion already.
        let hsla = ColorScale::hex_to_hsla(hex).expect("Bad hex value input");
        ColorScale::from_hsla(input_name.into(), hsla)
    }

    /// Creates a new color scale from exactly 12 hex values.
    pub fn from_hex_arr(name: String, hex: [&str; 12]) -> Self {
        let scale = ColorScaleName::find_or_custom(name);
        let mut hsla_arr = [hsla(0., 0., 0., 0.); 12];
        for (i, hex) in hex.iter().enumerate() {
            hsla_arr[i] = Self::hex_to_hsla(hex).expect("Bad hex value input");
        }
        let steps = Self::hsla_to_steps(hsla_arr, scale.label());

        Self {
            name: scale.label(),
            scale,
            steps
        }
    }

    /// TODO: Fill in the missing values by interpolating
    /// Note that this currently duplicates values instead of creating new colors
    ///
    /// Converts an 8-value hex array into a custom scale.
    ///
    /// This is a common format for defining neutral scales
    /// used in base16 themes and other theme formats
    pub fn from_8_hex(name: impl Into<String>, values: [&str; 8]) -> ColorScale {
        // TODO: Actually make this work
        // For the moment we will just repeat values

        let extended_hex_colors = [
            values[0], values[0], // Duplicate the first color
            values[1], values[2],
            values[3], values[3], // Duplicate the fourth color
            values[4], values[5],
            values[6], values[6], // Duplicate the seventh color
            values[7], values[7], // Duplicate the eighth color
        ];

        ColorScale::from_hex_arr(name.into(), extended_hex_colors)
    }

    // === Conversion & Construction Utilities ===

    /// Finds the best matching index in the scale for the given color.
    ///
    /// Ideally this will find where the given color would look most natural
    /// inserted into the scale.
    ///
    /// This is primarily used for extrapolating new scales from a single color.
    pub fn closest_scale_index(scale_colors: [Hsla; 12], hsla_color: Hsla) -> usize {
        let mut best_match = 0;
        let mut smallest_lum_diff = f32::MAX;

        for (index, scale_color) in scale_colors.iter().enumerate() {
            let lum_diff = (hsla_color.l - scale_color.l).abs();

            if lum_diff < smallest_lum_diff {
                smallest_lum_diff = lum_diff;
                best_match = index;
            }
        }
        best_match
    }

    fn hex_to_u32(hex: &str) -> Result<u32, std::num::ParseIntError> {
        u32::from_str_radix(&hex.trim_start_matches('#'), 16)
    }

    pub fn hex_to_hsla(hex: &str) -> Result<Hsla, std::num::ParseIntError> {
        Self::hex_to_u32(hex).map(|color| rgb::<Hsla>(color))
    }

    pub fn steps_arr_to_vec(steps: [ColorScaleStep; 12]) -> Vec<ColorScaleStep> {
        steps.iter().cloned().collect::<Vec<ColorScaleStep>>()
    }

    pub fn hsla_vec_to_arr(steps: Vec<Hsla>) -> [Hsla; 12] {
        if steps.len() != 12 {
            panic!("Expected a Vec of length 12, but it was {}", steps.len());
        }

        let mut arr = [hsla(0.0, 0.0, 0.0, 0.0); 12];
        for (i, step) in steps.iter().enumerate() {
            arr[i] = *step;
        }
        arr
    }

    pub fn hsla_to_steps(hslas: [Hsla; 12], name: String) -> [ColorScaleStep; 12] {
        let mut colors_vec = Vec::new();

        for (i, step) in hslas.iter().enumerate() {
            let step_name = ColorScaleStep::new_name_from_index(i, *step, ColorScaleName::Custom(name.clone()));
            let color = ColorScaleStep::new(step_name, *step, ColorScaleName::Custom(name.clone()));
            colors_vec.push(color);
        }

        let colors: [ColorScaleStep; 12] = match colors_vec.try_into() {
            Ok(array) => array,
            Err(vec) => panic!("Unexpected vector length {}", vec.len()),
        };

        colors
    }

    fn steps_to_hsla(steps: [ColorScaleStep; 12], name: String, input_hsla: Hsla) -> [Hsla; 12] {
        let original_hlsas: [Hsla; 12] = steps.iter().map(|step| step.value).collect::<Vec<Hsla>>().try_into().expect("Expected 12 steps");

        let closest_index_to_input = ColorScale::closest_scale_index(original_hlsas, input_hsla);

        let input_step_name =
            ColorScaleStep::new_name_from_index(closest_index_to_input, input_hsla, ColorScaleName::Custom(name.clone()));
        let input_step = ColorScaleStep::new(
            input_step_name,
            input_hsla,
            ColorScaleName::Custom(name.clone()),
        );

        let mut steps_arr = original_hlsas.to_vec();
        steps_arr[closest_index_to_input] = input_hsla;

        for i in 0..12 {
            if i != closest_index_to_input {
                steps_arr[i].h = input_hsla.h;
            }
        }

        let steps_hsla: [Hsla; 12] = steps_arr.try_into().expect("hue array wrong size");

        steps_hsla
    }
}

impl Default for ColorScale {
    fn default() -> Self {
        let name = "Untitled";
        let scale_name = ColorScaleName::Custom(name.into());
        let hslas = ColorScale::hsla_vec_to_arr(vec![
            hsla(0.0, 1.00, 0.99, 1.0),
            hsla(0.0, 1.00, 0.98, 1.0),
            hsla(0.0, 0.90, 0.96, 1.0),
            hsla(0.0, 1.00, 0.93, 1.0),
            hsla(0.0, 1.00, 0.90, 1.0),
            hsla(0.0, 0.94, 0.87, 1.0),
            hsla(0.0, 0.77, 0.81, 1.0),
            hsla(0.0, 0.70, 0.74, 1.0),
            hsla(0.0, 0.75, 0.59, 1.0),
            hsla(0.0, 0.69, 0.55, 1.0),
            hsla(0.0, 0.65, 0.49, 1.0),
            hsla(0.0, 0.63, 0.24, 1.0),
        ]);

        Self {
            name: scale_name.label(),
            scale: scale_name,
            steps: ColorScale::hsla_to_steps(hslas, name.into()),
        }
    }
}

impl From<ColorScaleName> for ColorScale {
    fn from(scale: ColorScaleName) -> ColorScale {
        ColorScale::from_default(scale)
    }
}

impl From<Hsla> for ColorScale {
    fn from(hsla: Hsla) -> ColorScale {
        ColorScale::from_hsla("Untitled".into(), hsla)
    }
}


#[derive(Debug, Default, Clone)]
pub enum Appearance {
    #[default]
    Dark,
    Light,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub author: Option<String>,
    pub url: Option<String>,
    pub appearances: Vec<ThemeVariant>,
    pub default_appearance: usize,
}

impl Theme {
    pub fn new(
        name: impl Into<String>,
        author: Option<impl Into<String>>,
        url: Option<impl Into<String>>,
        appearances: Vec<ThemeVariant>,
        default_appearance: usize,
    ) -> Theme {
        Theme {
            name: name.into(),
            author: author.map(Into::into),
            url: url.map(Into::into),
            appearances,
            default_appearance,
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn author(&mut self, author: String) -> &mut Self {
        self.author = Some(author);
        self
    }

    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    pub fn appearances(&mut self, appearances: Vec<ThemeVariant>) -> &mut Self {
        self.appearances = appearances;
        self
    }

    pub fn default_appearance(&mut self, default_appearance: usize) -> &mut Self {
        self.default_appearance = default_appearance;
        self
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Zed".to_string(),
            author: None,
            url: None,
            appearances: vec![
                ThemeVariant::default_dark(),
                ThemeVariant::default_light()
            ],
            default_appearance: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequiredScales {
    pub neutral: ColorScale,
    pub accent: ColorScale,
    pub positive: ColorScale,
    pub negative: ColorScale,
    pub caution: ColorScale,
}

impl RequiredScales {
    pub fn new(
        neutral: ColorScale,
        accent: ColorScale,
        positive: ColorScale,
        negative: ColorScale,
        caution: ColorScale,
    ) -> Self {
        Self {
            neutral,
            accent,
            positive,
            negative,
            caution,
        }
    }
}

impl IntoIterator for RequiredScales {
    type Item = ColorScale;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.neutral, self.accent, self.positive, self.negative, self.caution].into_iter()
    }
}

type ExtendedScales = Vec<ColorScale>;


#[derive(Debug, Clone)]
pub struct ThemeVariant {
    pub id: usize,
    pub name: String,
    pub appearance: Appearance,
    pub scales: RequiredScales,
    pub extended_scales: ExtendedScales,
    pub color: Option<ThemeColor>,
}

impl ThemeVariant {
    pub fn default_dark() -> Self {
        let required_scales = RequiredScales {
            neutral: ColorScaleName::Slate.into(),
            accent: ColorScaleName::Blue.into(),
            positive: ColorScaleName::Green.into(),
            negative: ColorScaleName::Red.into(),
            caution: ColorScaleName::Amber.into()
        };

        Self {
            id: 0,
            name: "Zed Dark".into(),
            appearance: Appearance::Dark,
            scales: required_scales.clone(),
            extended_scales: vec![
                ColorScaleName::Bronze.into(),
                ColorScaleName::Brown.into(),
                ColorScaleName::Crimson.into(),
                ColorScaleName::Cyan.into(),
                ColorScaleName::Gold.into(),
                ColorScaleName::Grass.into(),
                ColorScaleName::Gray.into(),
                ColorScaleName::Indigo.into(),
                ColorScaleName::Iris.into(),
                ColorScaleName::Jade.into(),
                ColorScaleName::Lime.into(),
                ColorScaleName::Mauve.into(),
                ColorScaleName::Mint.into(),
                ColorScaleName::Olive.into(),
                ColorScaleName::Orange.into(),
                ColorScaleName::Pink.into(),
                ColorScaleName::Plum.into(),
                ColorScaleName::Purple.into(),
                ColorScaleName::Sage.into(),
                ColorScaleName::Sand.into(),
                ColorScaleName::Sky.into(),
                ColorScaleName::Teal.into(),
                ColorScaleName::Tomato.into(),
                ColorScaleName::Violet.into(),
                ColorScaleName::Yellow.into(),
            ],
            color: None,
        }
    }

    pub fn default_light() -> Self {
        let required_scales = RequiredScales {
            neutral: ColorScaleName::Slate.into(),
            accent: ColorScaleName::Blue.into(),
            positive: ColorScaleName::Green.into(),
            negative: ColorScaleName::Red.into(),
            caution: ColorScaleName::Amber.into()
        };

        Self {
            id: 0,
            name: "Zed Light".into(),
            appearance: Appearance::Light,
            scales: required_scales.clone(),
            extended_scales: vec![
                ColorScaleName::Bronze.into(),
                ColorScaleName::Brown.into(),
                ColorScaleName::Crimson.into(),
                ColorScaleName::Cyan.into(),
                ColorScaleName::Gold.into(),
                ColorScaleName::Grass.into(),
                ColorScaleName::Gray.into(),
                ColorScaleName::Indigo.into(),
                ColorScaleName::Iris.into(),
                ColorScaleName::Jade.into(),
                ColorScaleName::Lime.into(),
                ColorScaleName::Mauve.into(),
                ColorScaleName::Mint.into(),
                ColorScaleName::Olive.into(),
                ColorScaleName::Orange.into(),
                ColorScaleName::Pink.into(),
                ColorScaleName::Plum.into(),
                ColorScaleName::Purple.into(),
                ColorScaleName::Sage.into(),
                ColorScaleName::Sand.into(),
                ColorScaleName::Sky.into(),
                ColorScaleName::Teal.into(),
                ColorScaleName::Tomato.into(),
                ColorScaleName::Violet.into(),
                ColorScaleName::Yellow.into(),
            ],
            color: None,
        }
    }
}

// Move to theme2::color.rs ================================================

#[derive(Debug, Clone, Copy)]
pub struct PlayerColor {
    pub cursor: Hsla,
    pub selection: Hsla,
}

type Players = [PlayerColor; 8];

impl PlayerColor {
    pub fn new(color: ColorScale) -> Self {
        Self {
            cursor: color.value(9),
            selection: color.value(4),
        }
    }

    pub fn players(v: ThemeVariant) -> Players {
        [
            PlayerColor::new(ColorScaleName::unwrap_or_default_scale("blue", v)),
            PlayerColor::new(ColorScaleName::unwrap_or_default_scale("green", v)),
            PlayerColor::new(ColorScaleName::unwrap_or_default_scale("red", v)),
            PlayerColor::new(ColorScaleName::unwrap_or_default_scale("yellow", v)),
            PlayerColor::new(ColorScaleName::unwrap_or_default_scale("purple", v)),
            PlayerColor::new(ColorScaleName::unwrap_or_default_scale("cyan", v)),
            PlayerColor::new(ColorScaleName::unwrap_or_default_scale("orange", v)),
            PlayerColor::new(ColorScaleName::unwrap_or_default_scale("pink", v)),

        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SyntaxColor {
    pub comment: Hsla,
    pub string: Hsla,
    pub function: Hsla,
    pub keyword: Hsla,
}

impl SyntaxColor {
    pub fn new() -> Self {
        let color = ThemeColor::new();

        Self {
            comment: color.syntax.comment,
            string: color.syntax.string,
            function: color.syntax.function,
            keyword: color.syntax.keyword,
        }
    }
}

/// ThemeColor is the primary interface for coloring elements in the UI.
///
/// It is a mapping layer between semantic theme colors and colors from the reference library.
///
/// While we are between zed and zed2 we use this to map semantic colors to the old theme.
#[derive(Debug, Clone, Copy)]
pub struct ThemeColor {
    pub transparent: Option<Hsla>,
    pub mac_os_traffic_light_red: Option<Hsla>,
    pub mac_os_traffic_light_yellow: Option<Hsla>,
    pub mac_os_traffic_light_green: Option<Hsla>,
    pub border: Option<Hsla>,
    pub border_variant: Option<Hsla>,
    pub border_focused: Option<Hsla>,
    pub border_transparent: Option<Hsla>,
    pub elevated_surface: Option<Hsla>,
    pub surface: Option<Hsla>,
    pub background: Option<Hsla>,
    pub element: Option<Hsla>,
    pub element_hover: Option<Hsla>,
    pub element_active: Option<Hsla>,
    pub element_selected: Option<Hsla>,
    pub element_disabled: Option<Hsla>,
    pub element_placeholder: Option<Hsla>,
    pub ghost_element: Option<Hsla>,
    pub ghost_element_hover: Option<Hsla>,
    pub ghost_element_active: Option<Hsla>,
    pub ghost_element_selected: Option<Hsla>,
    pub ghost_element_disabled: Option<Hsla>,
    pub text: Option<Hsla>,
    pub text_muted: Option<Hsla>,
    pub text_placeholder: Option<Hsla>,
    pub text_disabled: Option<Hsla>,
    pub text_accent: Option<Hsla>,
    pub icon: Option<Hsla>,
    pub icon_muted: Option<Hsla>,
    pub icon_disabled: Option<Hsla>,
    pub icon_placeholder: Option<Hsla>,
    pub icon_accent: Option<Hsla>,
    pub syntax: Option<SyntaxColor>,
    pub status_bar: Option<Hsla>,
    pub title_bar: Option<Hsla>,
    pub toolbar: Option<Hsla>,
    pub tab_bar: Option<Hsla>,
    pub editor: Option<Hsla>,
    pub editor_subheader: Option<Hsla>,
    pub editor_active_line: Option<Hsla>,
    pub terminal: Option<Hsla>,
    pub created: Option<Hsla>,
    pub modified: Option<Hsla>,
    pub deleted: Option<Hsla>,
    pub conflict: Option<Hsla>,
    pub hidden: Option<Hsla>,
    pub ignored: Option<Hsla>,
    pub renamed: Option<Hsla>,
    pub error: Option<Hsla>,
    pub warning: Option<Hsla>,
    pub info: Option<Hsla>,
    pub success: Option<Hsla>,
    pub git_created: Option<Hsla>,
    pub git_modified: Option<Hsla>,
    pub git_deleted: Option<Hsla>,
    pub git_conflict: Option<Hsla>,
    pub git_ignored: Option<Hsla>,
    pub git_renamed: Option<Hsla>,
    pub player: Option<[PlayerColor; 8]>,
}

impl Default for ThemeColor {
    fn default() -> Self {
        let transparent = hsla(0.0, 0.0, 0.0, 0.0);

        Self {
            transparent: Some(transparent),

            mac_os_traffic_light_red: Some(rgb::<Hsla>(0xEC695E)),
            mac_os_traffic_light_yellow: Some(rgb::<Hsla>(0xF4BF4F)),
            mac_os_traffic_light_green: Some(rgb::<Hsla>(0x62C554)),

            border: Some(DefaultColor::Slate.value(4)),
            border_variant: Some(DefaultColor::Slate.value(5)),
            border_focused: Some(DefaultColor::Slate.value(6)),
            border_transparent: Some(DefaultColor::Slate.value(4)),

            surface: Some(DefaultColor::Slate.value(2)),
            elevated_surface: Some(transparent),
            background: Some(DefaultColor::Slate.value(3)),

            element: Some(DefaultColor::Slate.value(3)),
            element_hover: Some(DefaultColor::Slate.value(4)),
            element_active: Some(DefaultColor::Slate.value(5)),
            element_selected: Some(DefaultColor::Slate.value(5)),
            element_disabled: Some(DefaultColor::Slate.value(3)),
            element_placeholder: Some(DefaultColor::Slate.value(1)),

            ghost_element: Some(transparent),
            ghost_element_hover: Some(DefaultColor::Slate.value(4)),
            ghost_element_active: Some(DefaultColor::Slate.value(5)),
            ghost_element_selected: Some(DefaultColor::Slate.value(5)),
            ghost_element_disabled: Some(transparent),

            text: Some(DefaultColor::Slate.value(12)),
            text_muted: Some(DefaultColor::Slate.value(11)),
            text_placeholder: Some(DefaultColor::Slate.value(11)),
            text_disabled: Some(DefaultColor::Slate.value(10)),
            text_accent: Some(DefaultColor::Blue.value(11)),

            icon: Some(DefaultColor::Slate.value(12)),
            icon_muted: Some(DefaultColor::Slate.value(11)),
            icon_placeholder: Some(DefaultColor::Slate.value(11)),
            icon_disabled: Some(DefaultColor::Slate.value(11)),
            icon_accent: Some(DefaultColor::Blue.value(11)),

            status_bar: Some(DefaultColor::Slate.value(1)),
            title_bar: Some(DefaultColor::Slate.value(1)),
            toolbar: Some(DefaultColor::Slate.value(1)),
            tab_bar: Some(DefaultColor::Slate.value(1)),
            editor: Some(DefaultColor::Slate.value(1)),
            editor_subheader: Some(DefaultColor::Slate.value(1)),
            editor_active_line: Some(DefaultColor::Slate.value(1)),
            terminal: Some(DefaultColor::Slate.value(1)),

            created: Some(DefaultColor::Green.value(9)),
            modified: Some(DefaultColor::Amber.value(9)),
            deleted: Some(DefaultColor::Red.value(9)),
            conflict: Some(DefaultColor::Amber.value(9)),
            hidden: Some(DefaultColor::Slate.value(11)),
            ignored: Some(DefaultColor::Slate.value(10)),
            renamed: Some(DefaultColor::Blue.value(9)),
            error: Some(DefaultColor::Red.value(9)),
            warning: Some(DefaultColor::Amber.value(9)),
            info: Some(DefaultColor::Blue.value(9)),
            success: Some(DefaultColor::Green.value(9)),

            git_created: Some(DefaultColor::Green.value(9)),
            git_modified: Some(DefaultColor::Amber.value(9)),
            git_deleted: Some(DefaultColor::Slate.value(10)),
            git_conflict: Some(DefaultColor::Amber.value(9)),
            git_ignored: Some(DefaultColor::Slate.value(11)),
            git_renamed: Some(DefaultColor::Blue.value(9)),

            player: Some([
                PlayerColor::new(ColorScaleName::Blue.into()),
                PlayerColor::new(ColorScaleName::Green.into()),
                PlayerColor::new(ColorScaleName::Red.into()),
                PlayerColor::new(ColorScaleName::Yellow.into()),
                PlayerColor::new(ColorScaleName::Purple.into()),
                PlayerColor::new(ColorScaleName::Cyan.into()),
                PlayerColor::new(ColorScaleName::Orange.into()),
                PlayerColor::new(ColorScaleName::Pink.into()),
            ]),
            syntax: None,
        }
    }
}

impl ThemeColor {
    pub fn new(v: ThemeVariant) -> Self {
        let transparent = hsla(0.0, 0.0, 0.0, 0.0);
        let vcolor = v.color.unwrap_or_default();

        Self {
            background: vcolor.background.or(Self::default().background),
            border: vcolor.border.or(Self::default().border),
            border_focused: vcolor.border_focused.or(Self::default().border_focused),
            border_transparent: vcolor.border_transparent.or(Self::default().border_transparent),
            border_variant: vcolor.border_variant.or(Self::default().border_variant),
            conflict: vcolor.conflict.or(Self::default().conflict),
            created: vcolor.created.or(Self::default().created),
            deleted: vcolor.deleted.or(Self::default().deleted),
            editor: vcolor.editor.or(Self::default().editor),
            editor_active_line: vcolor.editor_active_line.or(Self::default().editor_active_line),
            editor_subheader: vcolor.editor_subheader.or(Self::default().editor_subheader),
            elevated_surface: vcolor.elevated_surface.or(Self::default().elevated_surface),
            error: vcolor.error.or(Self::default().error),
            element: vcolor.element.or(Self::default().element),
            element_active: vcolor.element_active.or(Self::default().element_active),
            element_disabled: vcolor.element_disabled.or(Self::default().element_disabled),
            element_hover: vcolor.element_hover.or(Self::default().element_hover),
            element_selected: vcolor.element_selected.or(Self::default().element_selected),
            element_placeholder: vcolor.element_placeholder.or(Self::default().element_placeholder),
            ghost_element: vcolor.ghost_element.or(Self::default().ghost_element),
            ghost_element_active: vcolor.ghost_element_active.or(Self::default().ghost_element_active),
            ghost_element_disabled: vcolor.ghost_element_disabled.or(Self::default().ghost_element_disabled),
            ghost_element_hover: vcolor.ghost_element_hover.or(Self::default().ghost_element_hover),
            ghost_element_selected: vcolor.ghost_element_selected.or(Self::default().ghost_element_selected),
            git_conflict: vcolor.git_conflict.or(Self::default().git_conflict),
            git_created: vcolor.git_created.or(Self::default().git_created),
            git_deleted: vcolor.git_deleted.or(Self::default().git_deleted),
            git_ignored: vcolor.git_ignored.or(Self::default().git_ignored),
            git_modified: vcolor.git_modified.or(Self::default().git_modified),
            git_renamed: vcolor.git_renamed.or(Self::default().git_renamed),
            hidden: vcolor.hidden.or(Self::default().hidden),
            icon: vcolor.icon.or(Self::default().icon),
            icon_accent: vcolor.icon_accent.or(Self::default().icon_accent),
            icon_disabled: vcolor.icon_disabled.or(Self::default().icon_disabled),
            icon_muted: vcolor.icon_muted.or(Self::default().icon_muted),
            icon_placeholder: vcolor.icon_placeholder.or(Self::default().icon_placeholder),
            ignored: vcolor.ignored.or(Self::default().ignored),
            info: vcolor.info.or(Self::default().info),
            mac_os_traffic_light_green: vcolor.mac_os_traffic_light_green.or(Self::default().mac_os_traffic_light_green),
            mac_os_traffic_light_red: vcolor.mac_os_traffic_light_red.or(Self::default().mac_os_traffic_light_red),
            mac_os_traffic_light_yellow: vcolor.mac_os_traffic_light_yellow.or(Self::default().mac_os_traffic_light_yellow),
            modified: vcolor.modified.or(Self::default().modified),
            player: vcolor.player.or(Some(PlayerColor::players(v))),
            renamed: vcolor.renamed.or(Self::default().renamed),
            status_bar: vcolor.status_bar.or(Self::default().status_bar),
            success: vcolor.success.or(Self::default().success),
            surface: vcolor.surface.or(Self::default().surface),
            syntax: vcolor.syntax.or(Self::default().syntax),
            tab_bar: vcolor.tab_bar.or(Self::default().tab_bar),
            terminal: vcolor.terminal.or(Self::default().terminal),
            text: vcolor.text.or(Self::default().text),
            text_accent: vcolor.text_accent.or(Self::default().text_accent),
            text_disabled: vcolor.text_disabled.or(Self::default().text_disabled),
            text_muted: vcolor.text_muted.or(Self::default().text_muted),
            text_placeholder: vcolor.text_placeholder.or(Self::default().text_placeholder),
            title_bar: vcolor.title_bar.or(Self::default().title_bar),
            toolbar: vcolor.toolbar.or(Self::default().toolbar),
            transparent: vcolor.transparent.or(Self::default().transparent),
            warning: vcolor.warning.or(Self::default().warning),
        }
    }
}

// Move to theme2::default.rs ================================================

/// TODO: Move this to default.rs inside of the theme2 crate when we move it
impl ColorScale {
    /// Get one of the default color scales by ColorScaleName
    pub fn from_default(scale_name: ColorScaleName) -> Self {
         match scale_name {
            ColorScaleName::Gray => Self::from_hex_arr(
                ColorScaleName::Gray.label(),
                [
                "#111111", "#191919", "#222222", "#2a2a2a", "#313131", "#3a3a3a", "#484848",
                "#606060", "#6e6e6e", "#7b7b7b", "#b4b4b4", "#eeeeee",
            ]),
            ColorScaleName::Mauve => Self::from_hex_arr(
                ColorScaleName::Mauve.label(),
                [
                "#121113", "#1a191b", "#232225", "#2b292d", "#323035", "#3c393f", "#49474e",
                "#625f69", "#6f6d78", "#7c7a85", "#b5b2bc", "#eeeef0",
            ]),
            ColorScaleName::Slate => Self::from_hex_arr(
                ColorScaleName::Slate.label(),
                [
                "#111113", "#18191b", "#212225", "#272a2d", "#2e3135", "#363a3f", "#43484e",
                "#5a6169", "#696e77", "#777b84", "#b0b4ba", "#edeef0",
            ]),
            ColorScaleName::Sage => Self::from_hex_arr(
                ColorScaleName::Sage.label(),
                [
                "#101211", "#171918", "#202221", "#272a29", "#2e3130", "#373b39", "#444947",
                "#5b625f", "#63706b", "#717d79", "#adb5b2", "#eceeed",
            ]),
            ColorScaleName::Olive => Self::from_hex_arr(
                ColorScaleName::Olive.label(),
                [
                "#111210", "#181917", "#212220", "#282a27", "#2f312e", "#383a36", "#454843",
                "#5c625b", "#687066", "#767d74", "#afb5ad", "#eceeec",
            ]),
            ColorScaleName::Sand => Self::from_hex_arr(
                ColorScaleName::Sand.label(),
                [
                "#111110", "#191918", "#222221", "#2a2a28", "#31312e", "#3b3a37", "#494844",
                "#62605b", "#6f6d66", "#7c7b74", "#b5b3ad", "#eeeeec",
            ]),
            ColorScaleName::Tomato => Self::from_hex_arr(
                ColorScaleName::Tomato.label(),
                [
                    "#181111", "#1f1513", "#391714", "#4e1511", "#5e1c16", "#6e2920", "#853a2d",
                    "#ac4d39", "#e54d2e", "#ec6142", "#ff977d", "#fbd3cb",
                ]
            ),
            ColorScaleName::Red => Self::from_hex_arr(
                ColorScaleName::Red.label(),
                [
                    "#191111", "#201314", "#3b1219", "#500f1c", "#611623", "#72232d", "#8c333a",
                    "#b54548", "#e5484d", "#ec5d5e", "#ff9592", "#ffd1d9",
                ]
            ),
            ColorScaleName::Ruby => Self::from_hex_arr(
                ColorScaleName::Ruby.label(),
                [
                    "#191113", "#1e1517", "#3a141e", "#4e1325", "#5e1a2e", "#6f2539", "#883447",
                    "#b3445a", "#e54666", "#ec5a72", "#ff949d", "#fed2e1",
                ]
            ),
            ColorScaleName::Crimson => Self::from_hex_arr(
                ColorScaleName::Crimson.label(),
                [
                    "#191114", "#201318", "#381525", "#4d122f", "#5c1839", "#6d2545", "#873356",
                    "#b0436e", "#e93d82", "#ee518a", "#ff92ad", "#fdd3e8",
                ]
            ),
            ColorScaleName::Pink => Self::from_hex_arr(
                ColorScaleName::Pink.label(),
                [
                    "#191117", "#21121d", "#37172f", "#4b143d", "#591c47", "#692955", "#833869",
                    "#a84885", "#d6409f", "#de51a8", "#ff8dcc", "#fdd1ea",
                ]
            ),
            ColorScaleName::Plum => Self::from_hex_arr(
                ColorScaleName::Plum.label(),
                [
                    "#181118", "#201320", "#351a35", "#451d47", "#512454", "#5e3061", "#734079",
                    "#92549c", "#ab4aba", "#b658c4", "#e796f3", "#f4d4f4",
                ]
            ),
            ColorScaleName::Purple => Self::from_hex_arr(
                ColorScaleName::Purple.label(),
                [
                    "#18111b", "#1e1523", "#301c3b", "#3d224e", "#48295c", "#54346b", "#664282",
                    "#8457aa", "#8e4ec6", "#9a5cd0", "#d19dff", "#ecd9fa",
                ]
            ),
            ColorScaleName::Violet => Self::from_hex_arr(
                ColorScaleName::Violet.label(),
                [
                    "#14121f", "#1b1525", "#291f43", "#33255b", "#3c2e69", "#473876", "#56468b",
                    "#6958ad", "#6e56cf", "#7d66d9", "#baa7ff", "#e2ddfe",
                ]
            ),
            ColorScaleName::Iris => Self::from_hex_arr(
                ColorScaleName::Iris.label(),
                [
                    "#13131e", "#171625", "#202248", "#262a65", "#303374", "#3d3e82", "#4a4a95",
                    "#5958b1", "#5b5bd6", "#6e6ade", "#b1a9ff", "#e0dffe",
                ]
            ),
            ColorScaleName::Indigo => Self::from_hex_arr(
                ColorScaleName::Indigo.label(),
                [
                    "#11131f", "#141726", "#182449", "#1d2e62", "#253974", "#304384", "#3a4f97",
                    "#435db1", "#3e63dd", "#5472e4", "#9eb1ff", "#d6e1ff",
                ]
            ),
            ColorScaleName::Blue => Self::from_hex_arr(
                ColorScaleName::Blue.label(),
                [
                    "#0d1520", "#111927", "#0d2847", "#003362", "#004074", "#104d87", "#205d9e",
                    "#2870bd", "#0090ff", "#3b9eff", "#70b8ff", "#c2e6ff",
                ]
            ),
            ColorScaleName::Cyan => Self::from_hex_arr(
                ColorScaleName::Cyan.label(),
                [
                "#0b161a", "#101b20", "#082c36", "#003848", "#004558", "#045468", "#12677e",
                "#11809c", "#00a2c7", "#23afd0", "#4ccce6", "#b6ecf7",
            ]),
            ColorScaleName::Teal => Self::from_hex_arr(
                ColorScaleName::Teal.label(),
                [
                "#0d1514", "#111c1b", "#0d2d2a", "#023b37", "#084843", "#145750", "#1c6961",
                "#207e73", "#12a594", "#0eb39e", "#0bd8b6", "#adf0dd",
            ]),
            ColorScaleName::Jade => Self::from_hex_arr(
                ColorScaleName::Jade.label(),
                [
                "#0d1512", "#121c18", "#0f2e22", "#0b3b2c", "#114837", "#1b5745", "#246854",
                "#2a7e68", "#29a383", "#27b08b", "#1fd8a4", "#adf0d4",
            ]),
            ColorScaleName::Green => Self::from_hex_arr(
                ColorScaleName::Green.label(),
                [
                "#0e1512", "#121b17", "#132d21", "#113b29", "#174933", "#20573e", "#28684a",
                "#2f7c57", "#30a46c", "#33b074", "#3dd68c", "#b1f1cb",
            ]),
            ColorScaleName::Grass => Self::from_hex_arr(
                ColorScaleName::Grass.label(),
                [
                "#0e1511", "#141a15", "#1b2a1e", "#1d3a24", "#25482d", "#2d5736", "#366740",
                "#3e7949", "#46a758", "#53b365", "#71d083", "#c2f0c2",
            ]),
            ColorScaleName::Brown => Self::from_hex_arr(
                ColorScaleName::Brown.label(),
                [
                "#12110f", "#1c1816", "#28211d", "#322922", "#3e3128", "#4d3c2f", "#614a39",
                "#7c5f46", "#ad7f58", "#b88c67", "#dbb594", "#f2e1ca",
            ]),
            ColorScaleName::Bronze => Self::from_hex_arr(
                ColorScaleName::Bronze.label(),
                [
                "#141110", "#1c1917", "#262220", "#302a27", "#3b3330", "#493e3a", "#5a4c47",
                "#6f5f58", "#a18072", "#ae8c7e", "#d4b3a5", "#ede0d9",
            ]),
            ColorScaleName::Gold => Self::from_hex_arr(
                ColorScaleName::Gold.label(),
                [
                "#121211", "#1b1a17", "#24231f", "#2d2b26", "#38352e", "#444039", "#544f46",
                "#696256", "#978365", "#a39073", "#cbb99f", "#e8e2d9",
            ]),
            ColorScaleName::Sky => Self::from_hex_arr(
                ColorScaleName::Sky.label(),
                [
                "#0d141f", "#111a27", "#112840", "#113555", "#154467", "#1b537b", "#1f6692",
                "#197cae", "#7ce2fe", "#a8eeff", "#75c7f0", "#c2f3ff",
            ]),
            ColorScaleName::Mint => Self::from_hex_arr(
                ColorScaleName::Mint.label(),
                [
                "#0e1515", "#0f1b1b", "#092c2b", "#003a38", "#004744", "#105650", "#1e685f",
                "#277f70", "#86ead4", "#a8f5e5", "#58d5ba", "#c4f5e1",
            ]),
            ColorScaleName::Lime => Self::from_hex_arr(
                ColorScaleName::Lime.label(),
                [
                    "#11130c", "#151a10", "#1f2917", "#28211d", "#334423", "#3d522a", "#496231",
                    "#577538", "#bdee63", "#d4ff70", "#bde56c", "#e3f7ba",
                ]
            ),
            ColorScaleName::Yellow => Self::from_hex_arr(
                ColorScaleName::Yellow.label(),
                [
                    "#14120b", "#1b180f", "#2d2305", "#362b00", "#433500", "#524202", "#665417",
                    "#836a21", "#ffe629", "#ffff57", "#f5e147", "#f6eeb4",
                ]
            ),
            ColorScaleName::Amber => Self::from_hex_arr(
                ColorScaleName::Amber.label(),
                [
                    "#16120c", "#1d180f", "#302008", "#3f2700", "#4d3000", "#5c3d05", "#714f19",
                    "#8f6424", "#ffc53d", "#ffd60a", "#ffca16", "#ffe7b3",
                ]
            ),
            ColorScaleName::Orange => Self::from_hex_arr(
                ColorScaleName::Orange.label(),
                [
                    "#17120e", "#1e160f", "#331e0b", "#462100", "#562800", "#66350c", "#7e451d",
                    "#a35829", "#f76b15", "#ff801f", "#ffa057", "#ffe0c2",
                ]
            ),
            _ => Self::from_hex_arr(
                ColorScaleName::Black.label(),
                [
                    "#000000", "#000000", "#000000", "#000000", "#000000", "#000000", "#000000",
                    "#000000", "#000000", "#000000", "#000000", "#000000",
                ]
            ),
        }
    }
}
