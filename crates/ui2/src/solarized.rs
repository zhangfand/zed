// A example theme for building out theme2

use crate::{theme2::{Theme, ThemeVariant, Appearance, RequiredScales}, ScaleType, ThemeColor, NewCustomScale, CustomScale};

struct SolarizedScale {
    pub neutral: CustomScale,
    pub neutral_dark: CustomScale,
    pub yellow: CustomScale,
    pub orange: CustomScale,
    pub red: CustomScale,
    pub magenta: CustomScale,
    pub violet: CustomScale,
    pub blue: CustomScale,
    pub cyan: CustomScale,
    pub green: CustomScale,
}

pub fn solarized() -> Theme {
    let scale = SolarizedScale {
        neutral: NewCustomScale::from_8_hex(
            "Neutral",
            ["#fdf6e3","#eee8d5","#93a1a1","#839496","#657b83","#586e75","#073642","#002b36",
        ]),
        neutral_dark: NewCustomScale::from_8_hex(
            "Neutral",
            [
                "#002b36",
                "#073642",
                "#586e75",
                "#657b83",
                "#839496",
                "#93a1a1",
                "#eee8d5",
                "#fdf6e3",
            ]
        ),
        yellow: NewCustomScale::from_hex("Yellow", "#b58900"),
        orange: NewCustomScale::from_hex("Orange", "#cb4b16"),
        red: NewCustomScale::from_hex("Red", "#dc322f"),
        magenta: NewCustomScale::from_hex("Magenta", "#d33682"),
        violet: NewCustomScale::from_hex("Violet", "#6c71c4"),
        blue: NewCustomScale::from_hex("Blue", "#268bd2"),
        cyan: NewCustomScale::from_hex("Cyan", "#2aa198"),
        green: NewCustomScale::from_hex("Green", "#859900")
    };

     let solarized_light = ThemeVariant {
        id: 0,
        name: "Solarized Light".into(),
        appearance: Appearance::Light,
        scales: (
            RequiredScales {
                neutral: ScaleType::Custom(scale.neutral.clone()),
                accent: ScaleType::Custom(scale.blue.clone()),
                positive: ScaleType::Custom(scale.green.clone()),
                negative: ScaleType::Custom(scale.red.clone()),
                caution: ScaleType::Custom(scale.orange.clone()),
            },
            vec![
                ScaleType::Custom(scale.yellow.clone()),
                ScaleType::Custom(scale.magenta.clone()),
                ScaleType::Custom(scale.violet.clone()),
                ScaleType::Custom(scale.cyan.clone()),
            ],
        ),
        // TODO: Populate this using RequiredScales
        color: ThemeColor::new()
    };

     let solarized_dark = ThemeVariant {
        id: 1,
        name: "Solarized Dark".into(),
        appearance: Appearance::Dark,
        scales: (
            RequiredScales {
                neutral: ScaleType::Custom(scale.neutral_dark.clone()),
                accent: ScaleType::Custom(scale.blue.clone()),
                positive: ScaleType::Custom(scale.green.clone()),
                negative: ScaleType::Custom(scale.red.clone()),
                caution: ScaleType::Custom(scale.orange.clone()),
            },
            vec![
                ScaleType::Custom(scale.yellow.clone()),
                ScaleType::Custom(scale.magenta.clone()),
                ScaleType::Custom(scale.violet.clone()),
                ScaleType::Custom(scale.cyan.clone()),
            ]
        ),
        // TODO: Populate this using RequiredScales
        color: ThemeColor::new()
    };

    let theme = Theme::new(
        "Solarized",
        Some("altercation (Ethan Schoonover)"),
        Some("http://ethanschoonover.com/solarized"),
        vec![solarized_light, solarized_dark],
        0,
    );

    theme
}
