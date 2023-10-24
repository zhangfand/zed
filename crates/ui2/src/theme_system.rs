// https://www.figma.com/community/plugin/1105513882835626049

use gpui2::{SharedString, Hsla, hsla};
use strum::EnumIter;

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
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
    pub value: Hsla,
    pub scale: ColorScale,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            name: "Untitled Color".into(),
            value: hsla(0., 0., 0., 0.),
            scale: ColorScale::Custom("Untitled Color Scale".into()),
        }
    }
}

impl Color {
    pub fn new<S: Into<String>>(name: S, hsla: Hsla, scale: ColorScale) -> Self {
        Self {
            name: name.into(),
            value: hsla,
            scale,
        }
    }

    pub fn new_name_from_index(index: usize, hsla: Hsla, scale: ColorScale) -> String {
        format!("{} {}", scale.name(), index)
    }

    pub fn hsla(&self) -> Hsla {
        self.value
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
    pub fn new(name: ColorScale, hues: [Hsla; 12]) -> Self {
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

    pub fn steps_arr_to_vec(steps: [Color; 12]) -> Vec<Color> {
        steps.iter().cloned().collect::<Vec<Color>>()
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

    pub fn closest_scale_index(scale_colors: [Hsla; 12], hsla_color: Hsla) -> usize {
       let mut best_match = 0;
       let mut best_score = f32::MIN;

       for (index, scale_color) in scale_colors.iter().enumerate() {
          let lum_diff = (hsla_color.l - scale_color.l).abs();
          let sat_diff = (hsla_color.s - scale_color.s).abs();
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

#[derive(Debug, Clone)]
pub struct NewCustomScale {
    pub name: Option<String>,
    pub steps: Option<Vec<Hsla>>,
}

impl NewCustomScale {
    pub fn step_arr_to_colors(steps: [Hsla; 12], name: String) -> [Color; 12] {
        let mut colors_vec = Vec::new();

        for (i, step) in steps.iter().enumerate() {
            let step_name = Color::new_name_from_index(i, *step, ColorScale::Custom(name.clone()));
            let color = Color::new(step_name, *step, ColorScale::Custom(name.clone()));
            colors_vec.push(color);
        }

        let colors: [Color; 12] = match colors_vec.try_into() {
            Ok(array) => array,
            Err(vec) => panic!("Unexpected vector length {}", vec.len()),
        };

        colors
    }

    pub fn new_from_hsla(input_name: Option<String>, input_hsla: Hsla) -> CustomScale {
        let default = NewCustomScale::default();
        let name = input_name.unwrap_or(default.name.unwrap());

        let steps_arr = Self::steps_from_hsla(
            default.steps,
            name.clone(),
            input_hsla
        );

        CustomScale {
            name: name.clone(),
            steps: Self::step_arr_to_colors(steps_arr, name.clone()),
        }
    }

    pub fn new_from_steps(input_name: Option<String>, input_steps: [Hsla; 12]) -> CustomScale {
        let default = NewCustomScale::default();
        let name = input_name.unwrap_or(default.name.unwrap());

        CustomScale {
            name: name.clone(),
            steps: Self::step_arr_to_colors(input_steps, name.clone()),
        }
    }

    fn steps_from_hsla(scales: Option<Vec<Hsla>>, name: String, input_hsla: Hsla) -> [Hsla; 12] {
        let original_hues = scales.expect("Scale doesn't have any hues");
        let original_steps = Scale::hsla_vec_to_arr(original_hues);
        let index = Scale::closest_scale_index(original_steps, input_hsla);

        let input_step_name = Color::new_name_from_index(index, input_hsla, ColorScale::Custom(name.clone()));
        let input_step = Color::new(input_step_name, input_hsla, ColorScale::Custom(name.clone()));

        let mut steps_arr = NewCustomScale::default().steps.expect("Somehow the default scale doesn't have any steps");

        steps_arr[index] = input_hsla;

        if index > 0 {
            steps_arr[index - 1].h = steps_arr[index].h;
        }
        if index < 11 {
            steps_arr[index + 1].h = steps_arr[index].h;
        }

        let mut steps_hsla: [Hsla; 12] = [hsla(0.0, 0.0, 0.0, 0.0); 12]; // Initialize with a neutral value.

        for (i, color) in steps_arr.iter().enumerate() {
            steps_hsla[i] = color.clone();
        }

        steps_hsla
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn hues(mut self, hues: Vec<Hsla>) -> Self {
        assert_eq!(hues.len(), 12);
        self.steps = Some(hues);
        self
    }
}


impl Default for NewCustomScale {
    fn default() -> Self {
        Self {
            name: Some("Untitled Custom Scale".into()),
            steps: Some(vec![
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
            ])
        }
    }
}

#[derive(Debug, Clone)]
pub struct CustomScale {
    pub name: String,
    pub steps: [Color; 12],
}

#[derive(Debug, Clone)]
pub enum ScaleEnum {
    Standard(Scale),
    Custom(CustomScale),
}

#[derive(Debug, Clone)]
pub struct ThemeScales {
    pub name: String,
    pub scales: Vec<ScaleEnum>,
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
            scales: self.scales,
        }
    }
}
