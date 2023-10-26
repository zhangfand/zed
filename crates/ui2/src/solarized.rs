// A example theme for building out theme2

use crate::theme2::{Theme, ThemeVariant, Appearance, RequiredScales, ColorScale, ColorScaleName};

struct SolarizedScale {
    pub neutral: ColorScale,
    pub neutral_dark: ColorScale,
    pub yellow: ColorScale,
    pub orange: ColorScale,
    pub red: ColorScale,
    pub magenta: ColorScale,
    pub violet: ColorScale,
    pub blue: ColorScale,
    pub cyan: ColorScale,
    pub green: ColorScale,
}

pub fn solarized() -> Theme {
    let scale = SolarizedScale {
        neutral: ColorScale::from_8_hex(
            "Neutral",
            ["#fdf6e3","#eee8d5","#93a1a1","#839496","#657b83","#586e75","#073642","#002b36",
        ]),
        neutral_dark: ColorScale::from_8_hex(
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
        yellow: ColorScale::from_hex("Yellow", "#b58900"),
        orange: ColorScale::from_hex("Orange", "#cb4b16"),
        red: ColorScale::from_hex("Red", "#dc322f"),
        magenta: ColorScale::from_hex("Magenta", "#d33682"),
        violet: ColorScale::from_hex("Violet", "#6c71c4"),
        blue: ColorScale::from_hex("Blue", "#268bd2"),
        cyan: ColorScale::from_hex("Cyan", "#2aa198"),
        green: ColorScale::from_hex("Green", "#859900")
    };

     let solarized_light = ThemeVariant {
        id: 0,
        name: "Solarized Light".into(),
        appearance: Appearance::Light,
        scales: RequiredScales {
            neutral: scale.neutral.clone(),
            accent: scale.blue.clone(),
            positive: scale.green.clone(),
            negative: scale.red.clone(),
            caution: scale.orange.clone(),
        },
        extended_scales:
            vec![
                scale.yellow.clone(),
                scale.magenta.clone(),
                scale.violet.clone(),
                scale.cyan.clone()
            ],
        // TODO: Populate this using RequiredScales
        color: None
    };

     let solarized_dark = ThemeVariant {
        id: 1,
        name: "Solarized Dark".into(),
        appearance: Appearance::Dark,
        scales: RequiredScales {
            neutral: scale.neutral_dark.clone(),
            accent: scale.blue.clone(),
            positive: scale.green.clone(),
            negative: scale.red.clone(),
            caution: scale.orange.clone(),
        },
        extended_scales:
            vec![
                scale.yellow.clone(),
                scale.magenta.clone(),
                scale.violet.clone(),
                scale.cyan.clone()
            ],
        // TODO: Populate this using RequiredScales
        color: None
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
