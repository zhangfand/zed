// A example theme for building out theme2

use crate::theme2::{Theme, ThemeAppearance};

pub fn solarized() -> Theme {
    let solarized_dark: ThemeAppearance = todo!("Solarized Dark");
    let solarized_light: ThemeAppearance = todo!("Solarized Light");

    let theme = Theme::new(
        "Solarized",
        Some("altercation (Ethan Schoonover)"),
        Some("http://ethanschoonover.com/solarized"),
        vec![solarized_light, solarized_dark],
        0,
    );

    theme
}
