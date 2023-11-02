use gpui2::rgb;
use gpui2::Hsla;
use refineable::Refineable;

use crate::{
    default_color_scales, Appearance, GitStatusColors, PlayerColors, StatusColors, SyntaxTheme,
    SystemColors, ThemeColors, ThemeColorsRefinement, ThemeFamily, ThemeStyles, ThemeVariant,
};

// const syntax = {
//   tag: e`5CCFE6`,
//   func: e`FFD173`,
//   entity: e`73D0FF`,
//   string: e`D5FF80`,
//   regexp: e`95E6CB`,
//   markup: e`F28779`,
//   keyword: e`FFAD66`,
//   special: e`FFDFB3`,
//   comment: e`B8CFE6`.alpha(0.5),
//   constant: e`DFBFFF`,
//   operator: e`F29E74`
// }

// const vcs = {
//   added: e`87D96C`,
//   modified: e`80BFFF`,
//   removed: e`F27983`
// }

// const editor = {
//   fg: e`CCCAC2`,
//   bg: e`242936`,
//   line: e`1A1F29`,
//   selection: {
//     active: e`409FFF`.alpha(0.25),
//     inactive: e`409FFF`.alpha(0.13)
//   },
//   findMatch: {
//     active: e`695380`,
//     inactive: e`695380`.alpha(0.4)
//   },
//   gutter: {
//     active: e`8A9199`.alpha(0.8),
//     normal: e`8A9199`.alpha(0.4)
//   },
//   indentGuide: {
//     active: e`8A9199`.alpha(0.35),
//     normal: e`8A9199`.alpha(0.18)
//   }
// }

// const ui = {
//   fg: u`707A8C`,
//   bg: u`1F2430`,
//   line: u`171B24`,
//   selection: {
//     active: u`637599`.alpha(0.15),
//     normal: u`69758C`.alpha(0.12)
//   },
//   panel: {
//     bg: u`1C212B`,
//     shadow: u`12151C`.alpha(0.7)
//   }
// }

// const common = {
//   accent: u`FFCC66`,
//   error: u`FF6666`
// }

pub fn ayu_mirage_colors() -> ThemeColors {
    let mut colors = ThemeColors::default_dark();

    let ayu_colors = ThemeColorsRefinement {
        text: Some(rgb::<Hsla>(0xCCCAC2)),
        background: Some(rgb::<Hsla>(0x242936)),
        // background: Some(green),
        ..Default::default()
    };

    colors.refine(&ayu_colors);

    colors
}

pub fn ayu_family() -> ThemeFamily {
    ThemeFamily {
        id: "ayu".to_string(),
        name: "Ayu".into(),
        author: "Konstantin Pschera <me@kons.ch>".into(),
        themes: vec![ayu_light(), ayu_dark(), ayu_mirage()],
        scales: default_color_scales(),
    }
}

fn ayu_light() -> ThemeVariant {
    ThemeVariant {
        id: "ayu_light".to_string(),
        name: "Ayu Light".into(),
        appearance: Appearance::Light,
        styles: ThemeStyles {
            system: SystemColors::default(),
            colors: ThemeColors::default_light(),
            status: StatusColors::default(),
            git: GitStatusColors::default(),
            player: PlayerColors::default(),
            syntax: SyntaxTheme::default_light(),
        },
    }
}

fn ayu_dark() -> ThemeVariant {
    ThemeVariant {
        id: "ayu_dark".to_string(),
        name: "Ayu Dark".into(),
        appearance: Appearance::Dark,
        styles: ThemeStyles {
            system: SystemColors::default(),
            colors: ThemeColors::default_dark(),
            status: StatusColors::default(),
            git: GitStatusColors::default(),
            player: PlayerColors::default(),
            syntax: SyntaxTheme::default_dark(),
        },
    }
}

fn ayu_mirage() -> ThemeVariant {
    ThemeVariant {
        id: "ayu_mirage".to_string(),
        name: "Ayu Mirage".into(),
        appearance: Appearance::Dark,
        styles: ThemeStyles {
            system: SystemColors::default(),
            colors: ayu_mirage_colors(),
            status: StatusColors::default(),
            git: GitStatusColors::default(),
            player: PlayerColors::default(),
            syntax: SyntaxTheme::default_dark(),
        },
    }
}
