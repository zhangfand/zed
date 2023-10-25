use gpui2::{Hsla, hsla, rgb};
use strum::EnumIter;

pub fn to_gpui_hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    hsla(h/360.0, s/100.0, l/100.0, a)
}

pub fn to_gpui_hue(h: f32) -> f32 {
    h/360.0
}

pub fn from_gpui_hue(h: f32) -> f32 {
    h*360.0
}

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

    pub fn hex_to_u32(hex: &str) -> Result<u32, std::num::ParseIntError> {
        u32::from_str_radix(&hex.trim_start_matches('#'), 16)
    }

    pub fn hex_to_hsla(hex: &str) -> Result<Hsla, std::num::ParseIntError> {
        ColorScale::hex_to_u32(hex).map(|color| rgb::<Hsla>(color))
    }

    pub fn hex_arr_to_hsla(hex: [&str; 12]) -> [Hsla; 12] {
        let mut hslas = [hsla(0., 0., 0., 0.); 12];
        for (i, hex) in hex.iter().enumerate() {
            hslas[i] = ColorScale::hex_to_hsla(hex).unwrap_or(hsla(0., 0., 0., 0.));
        }
        hslas
    }

    pub fn new(scale: ColorScale) -> Scale {
        let scale_steps = match scale {
            ColorScale::Gray => {
                ColorScale::hex_arr_to_hsla([
                    "#111111",
                    "#191919",
                    "#222222",
                    "#2a2a2a",
                    "#313131",
                    "#3a3a3a",
                    "#484848",
                    "#606060",
                    "#6e6e6e",
                    "#7b7b7b",
                    "#b4b4b4",
                    "#eeeeee"
                ])
            },
            ColorScale::Mauve => {
                ColorScale::hex_arr_to_hsla([
                    "#121113",
                    "#1a191b",
                    "#232225",
                    "#2b292d",
                    "#323035",
                    "#3c393f",
                    "#49474e",
                    "#625f69",
                    "#6f6d78",
                    "#7c7a85",
                    "#b5b2bc",
                    "#eeeef0"
                ])
            },
            ColorScale::Slate => {
                ColorScale::hex_arr_to_hsla([
                    "#111113",
                    "#18191b",
                    "#212225",
                    "#272a2d",
                    "#2e3135",
                    "#363a3f",
                    "#43484e",
                    "#5a6169",
                    "#696e77",
                    "#777b84",
                    "#b0b4ba",
                    "#edeef0"
                ])
            },
            ColorScale::Sage => {
                ColorScale::hex_arr_to_hsla([
                    "#101211",
                    "#171918",
                    "#202221",
                    "#272a29",
                    "#2e3130",
                    "#373b39",
                    "#444947",
                    "#5b625f",
                    "#63706b",
                    "#717d79",
                    "#adb5b2",
                    "#eceeed"
                ])
            },
            ColorScale::Olive => {
                ColorScale::hex_arr_to_hsla([
                    "#111210",
                    "#181917",
                    "#212220",
                    "#282a27",
                    "#2f312e",
                    "#383a36",
                    "#454843",
                    "#5c625b",
                    "#687066",
                    "#767d74",
                    "#afb5ad",
                    "#eceeec"
                ])
            },
            ColorScale::Sand => {
                ColorScale::hex_arr_to_hsla([
                    "#111110",
                    "#191918",
                    "#222221",
                    "#2a2a28",
                    "#31312e",
                    "#3b3a37",
                    "#494844",
                    "#62605b",
                    "#6f6d66",
                    "#7c7b74",
                    "#b5b3ad",
                    "#eeeeec"
                ])
            },
            ColorScale::Tomato => {
                ColorScale::hex_arr_to_hsla([
                    "#181111",
                    "#1f1513",
                    "#391714",
                    "#4e1511",
                    "#5e1c16",
                    "#6e2920",
                    "#853a2d",
                    "#ac4d39",
                    "#e54d2e",
                    "#ec6142",
                    "#ff977d",
                    "#fbd3cb"
                ])
            }
            ColorScale::Red => {
                ColorScale::hex_arr_to_hsla([
                    "#191111",
                    "#201314",
                    "#3b1219",
                    "#500f1c",
                    "#611623",
                    "#72232d",
                    "#8c333a",
                    "#b54548",
                    "#e5484d",
                    "#ec5d5e",
                    "#ff9592",
                    "#ffd1d9"
                ])
            }
            ColorScale::Ruby => {
                ColorScale::hex_arr_to_hsla([
                    "#191113",
                    "#1e1517",
                    "#3a141e",
                    "#4e1325",
                    "#5e1a2e",
                    "#6f2539",
                    "#883447",
                    "#b3445a",
                    "#e54666",
                    "#ec5a72",
                    "#ff949d",
                    "#fed2e1"
                ])
            }
            ColorScale::Crimson => {
                ColorScale::hex_arr_to_hsla([
                    "#191114",
                    "#201318",
                    "#381525",
                    "#4d122f",
                    "#5c1839",
                    "#6d2545",
                    "#873356",
                    "#b0436e",
                    "#e93d82",
                    "#ee518a",
                    "#ff92ad",
                    "#fdd3e8"
                ])
            }
            ColorScale::Pink => {
                ColorScale::hex_arr_to_hsla([
                    "#191117",
                    "#21121d",
                    "#37172f",
                    "#4b143d",
                    "#591c47",
                    "#692955",
                    "#833869",
                    "#a84885",
                    "#d6409f",
                    "#de51a8",
                    "#ff8dcc",
                    "#fdd1ea"
                ])
            }
            ColorScale::Plum => {
                ColorScale::hex_arr_to_hsla([
                    "#181118",
                    "#201320",
                    "#351a35",
                    "#451d47",
                    "#512454",
                    "#5e3061",
                    "#734079",
                    "#92549c",
                    "#ab4aba",
                    "#b658c4",
                    "#e796f3",
                    "#f4d4f4"
                ])
            }
            ColorScale::Purple => {
                ColorScale::hex_arr_to_hsla([
                    "#18111b",
                    "#1e1523",
                    "#301c3b",
                    "#3d224e",
                    "#48295c",
                    "#54346b",
                    "#664282",
                    "#8457aa",
                    "#8e4ec6",
                    "#9a5cd0",
                    "#d19dff",
                    "#ecd9fa"
                ])
            }
            ColorScale::Violet => {
                ColorScale::hex_arr_to_hsla([
                    "#14121f",
                    "#1b1525",
                    "#291f43",
                    "#33255b",
                    "#3c2e69",
                    "#473876",
                    "#56468b",
                    "#6958ad",
                    "#6e56cf",
                    "#7d66d9",
                    "#baa7ff",
                    "#e2ddfe"
                ])
            }
            ColorScale::Iris => {
                ColorScale::hex_arr_to_hsla([
                    "#13131e",
                    "#171625",
                    "#202248",
                    "#262a65",
                    "#303374",
                    "#3d3e82",
                    "#4a4a95",
                    "#5958b1",
                    "#5b5bd6",
                    "#6e6ade",
                    "#b1a9ff",
                    "#e0dffe"
                ])
            }
            ColorScale::Indigo => {
                ColorScale::hex_arr_to_hsla([
                    "#11131f",
                    "#141726",
                    "#182449",
                    "#1d2e62",
                    "#253974",
                    "#304384",
                    "#3a4f97",
                    "#435db1",
                    "#3e63dd",
                    "#5472e4",
                    "#9eb1ff",
                    "#d6e1ff"
                ])
            }
            ColorScale::Blue => {
                ColorScale::hex_arr_to_hsla([
                    "#0d1520",
                    "#111927",
                    "#0d2847",
                    "#003362",
                    "#004074",
                    "#104d87",
                    "#205d9e",
                    "#2870bd",
                    "#0090ff",
                    "#3b9eff",
                    "#70b8ff",
                    "#c2e6ff"
                ])
            }
            ColorScale::Cyan => {
                ColorScale::hex_arr_to_hsla([
                    "#0b161a",
                    "#101b20",
                    "#082c36",
                    "#003848",
                    "#004558",
                    "#045468",
                    "#12677e",
                    "#11809c",
                    "#00a2c7",
                    "#23afd0",
                    "#4ccce6",
                    "#b6ecf7"
                ])
            },
            ColorScale::Teal => {
                ColorScale::hex_arr_to_hsla([
                    "#0d1514",
                    "#111c1b",
                    "#0d2d2a",
                    "#023b37",
                    "#084843",
                    "#145750",
                    "#1c6961",
                    "#207e73",
                    "#12a594",
                    "#0eb39e",
                    "#0bd8b6",
                    "#adf0dd"
                ])
            },
            ColorScale::Jade => {
                ColorScale::hex_arr_to_hsla([
                    "#0d1512",
                    "#121c18",
                    "#0f2e22",
                    "#0b3b2c",
                    "#114837",
                    "#1b5745",
                    "#246854",
                    "#2a7e68",
                    "#29a383",
                    "#27b08b",
                    "#1fd8a4",
                    "#adf0d4"
                ])
            },
            ColorScale::Green => {
                ColorScale::hex_arr_to_hsla([
                    "#0e1512",
                    "#121b17",
                    "#132d21",
                    "#113b29",
                    "#174933",
                    "#20573e",
                    "#28684a",
                    "#2f7c57",
                    "#30a46c",
                    "#33b074",
                    "#3dd68c",
                    "#b1f1cb"
                ])
            },
            ColorScale::Grass => {
                ColorScale::hex_arr_to_hsla([
                    "#0e1511",
                    "#141a15",
                    "#1b2a1e",
                    "#1d3a24",
                    "#25482d",
                    "#2d5736",
                    "#366740",
                    "#3e7949",
                    "#46a758",
                    "#53b365",
                    "#71d083",
                    "#c2f0c2"
                ])
            },
            ColorScale::Brown => {
                ColorScale::hex_arr_to_hsla([
                    "#12110f",
                    "#1c1816",
                    "#28211d",
                    "#322922",
                    "#3e3128",
                    "#4d3c2f",
                    "#614a39",
                    "#7c5f46",
                    "#ad7f58",
                    "#b88c67",
                    "#dbb594",
                    "#f2e1ca"
                ])
            },
            ColorScale::Bronze => {
                ColorScale::hex_arr_to_hsla([
                    "#141110",
                    "#1c1917",
                    "#262220",
                    "#302a27",
                    "#3b3330",
                    "#493e3a",
                    "#5a4c47",
                    "#6f5f58",
                    "#a18072",
                    "#ae8c7e",
                    "#d4b3a5",
                    "#ede0d9"
                ])
            },
            ColorScale::Gold => {
                ColorScale::hex_arr_to_hsla([
                    "#121211",
                    "#1b1a17",
                    "#24231f",
                    "#2d2b26",
                    "#38352e",
                    "#444039",
                    "#544f46",
                    "#696256",
                    "#978365",
                    "#a39073",
                    "#cbb99f",
                    "#e8e2d9"
                ])
            },
            ColorScale::Sky => {
                ColorScale::hex_arr_to_hsla([
                    "#0d141f",
                    "#111a27",
                    "#112840",
                    "#113555",
                    "#154467",
                    "#1b537b",
                    "#1f6692",
                    "#197cae",
                    "#7ce2fe",
                    "#a8eeff",
                    "#75c7f0",
                    "#c2f3ff"
                ])
            },
            ColorScale::Mint => {
                ColorScale::hex_arr_to_hsla([
                    "#0e1515",
                    "#0f1b1b",
                    "#092c2b",
                    "#003a38",
                    "#004744",
                    "#105650",
                    "#1e685f",
                    "#277f70",
                    "#86ead4",
                    "#a8f5e5",
                    "#58d5ba",
                    "#c4f5e1"
                ])
            },
            ColorScale::Lime => {
                ColorScale::hex_arr_to_hsla([
                    "#11130c",
                    "#151a10",
                    "#1f2917",
                    "#28211d",
                    "#334423",
                    "#3d522a",
                    "#496231",
                    "#577538",
                    "#bdee63",
                    "#d4ff70",
                    "#bde56c",
                    "#e3f7ba"
                ])
            },
            ColorScale::Yellow => {
                ColorScale::hex_arr_to_hsla([
                    "#14120b",
                    "#1b180f",
                    "#2d2305",
                    "#362b00",
                    "#433500",
                    "#524202",
                    "#665417",
                    "#836a21",
                    "#ffe629",
                    "#ffff57",
                    "#f5e147",
                    "#f6eeb4"
                ])
            },
            ColorScale::Amber => {
                ColorScale::hex_arr_to_hsla([
                    "#16120c",
                    "#1d180f",
                    "#302008",
                    "#3f2700",
                    "#4d3000",
                    "#5c3d05",
                    "#714f19",
                    "#8f6424",
                    "#ffc53d",
                    "#ffd60a",
                    "#ffca16",
                    "#ffe7b3"
                ])
            },
            ColorScale::Orange => {
                ColorScale::hex_arr_to_hsla([
                    "#17120e",
                    "#1e160f",
                    "#331e0b",
                    "#462100",
                    "#562800",
                    "#66350c",
                    "#7e451d",
                    "#a35829",
                    "#f76b15",
                    "#ff801f",
                    "#ffa057",
                    "#ffe0c2"
                ])
            },
            _ => {
                ColorScale::hex_arr_to_hsla([
                    "#000000",
                    "#000000",
                    "#000000",
                    "#000000",
                    "#000000",
                    "#000000",
                    "#000000",
                    "#000000",
                    "#000000",
                    "#000000",
                    "#000000",
                    "#000000"
                ])
            }
        };

        Scale::new(scale, scale_steps)
    }

    /// Returns the One-based value of the given step in the scale.
    ///
    /// We usally reference steps as 1-12 instead of 0-11, so we
    /// automatically subtract 1 from the index.
    pub fn value(self, index: usize) -> Hsla {
        let color_scale = ColorScale::new(self);
        color_scale.steps[index - 1].value
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

        // Find the index of the closest matching color from the original scale
        let index = Scale::closest_scale_index(original_steps, input_hsla);

        let input_step_name = Color::new_name_from_index(index, input_hsla, ColorScale::Custom(name.clone()));
        let input_step = Color::new(input_step_name, input_hsla, ColorScale::Custom(name.clone()));

        // Initialize array with existing scales
        let mut steps_arr = original_steps.to_vec();

        // Replace the closest color with the input color
        steps_arr[index] = input_hsla;

        // Update the hue of all other colors in the scale to match the input
        for i in 0..12 {
            if i != index {
                steps_arr[i].h = input_hsla.h;
            }
        }

        // Convert to array
        let steps_hsla: [Hsla; 12] = steps_arr.try_into().expect("hue array wrong size");

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

impl From<ColorScale> for Scale {
    fn from(scale: ColorScale) -> Scale {
        ColorScale::new(scale)
    }
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

    pub fn iter(&self) -> std::slice::Iter<ScaleEnum> {
        self.scales.iter()
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
