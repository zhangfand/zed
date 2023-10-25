// A example theme for building out theme2

use crate::{theme2::{Theme, ThemeAppearance, AppearanceMode, RequiredScales}, ScaleType, ThemeColor, NewCustomScale};

pub fn solarized_scales() -> Vec<ScaleType> {
    // let neutral_scale = NewCustomScale::from_8_hex(
    //     "#fdf6e3",
    //     "#eee8d5",
    //     "#93a1a1",
    //     "#839496",
    //     "#657b83",
    //     "#586e75",
    //     "#073642",
    //     "#002b36",
    // );

    let neutral = ScaleType::Custom("#b58900".into());
    let yellow = ScaleType::Custom("#b58900".into());
    let orange = ScaleType::Custom("#cb4b16".into());
    let red = ScaleType::Custom("#dc322f".into());
    let magenta = ScaleType::Custom("#d33682".into());
    let violet = ScaleType::Custom("#6c71c4".into());
    let blue = ScaleType::Custom("#268bd2".into());
    let cyan = ScaleType::Custom("#2aa198".into());
    let green = ScaleType::Custom("#859900".into());

    vec![
        neutral,
        yellow,
        orange,
        red,
        magenta,
        violet,
        blue,
        cyan,
        green,
    ]
}

pub fn solarized() -> Theme {
    let solarized_scales = solarized_scales();

     let solarized_light = ThemeAppearance {
        id: 0,
        name: "Solarized Light".into(),
        appearance: AppearanceMode::Light,
        scales: (
            RequiredScales::new(
                solarized_scales[0].clone(),
                solarized_scales[0].clone(),
                solarized_scales[0].clone(),
                solarized_scales[0].clone(),
                solarized_scales[0].clone(),
            ),
            vec![],
        ),
        // TODO: Populate this using RequiredScales
        color: ThemeColor::new()
    };

     let solarized_dark = ThemeAppearance {
        id: 1,
        name: "Solarized Dark".into(),
        appearance: AppearanceMode::Dark,
        scales: (
            RequiredScales::new(
                solarized_scales[0].clone(),
                solarized_scales[0].clone(),
                solarized_scales[0].clone(),
                solarized_scales[0].clone(),
                solarized_scales[0].clone(),
            ),
            vec![],
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
