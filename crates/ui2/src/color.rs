use crate::{ThemeScales, ScaleEnum, ColorScale};
pub use crate::{theme, ButtonVariant, ElementExt, Theme};
use gpui2::{hsla, rgb, Hsla, WindowContext};
use strum::EnumIter;

#[derive(Clone, Copy)]
pub struct PlayerThemeColors {
    pub cursor: Hsla,
    pub selection: Hsla,
}

impl PlayerThemeColors {
    pub fn new(cx: &WindowContext, ix: usize) -> Self {
        let theme = theme(cx);

        if ix < theme.players.len() {
            Self {
                cursor: theme.players[ix].cursor,
                selection: theme.players[ix].selection,
            }
        } else {
            Self {
                cursor: rgb::<Hsla>(0xff00ff),
                selection: rgb::<Hsla>(0xff00ff),
            }
        }
    }
}

// TODO: Why is there both SyntaxColor and HighlightColor?
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct ThemeColor {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_transparent: Hsla,
    /// The background color of an elevated surface, like a modal, tooltip or toast.
    pub elevated_surface: Hsla,
    pub surface: Hsla,
    /// Window background color of the base app
    pub background: Hsla,
    /// Default background for elements like filled buttons,
    /// text fields, checkboxes, radio buttons, etc.
    /// - TODO: Map to step 3.
    pub filled_element: Hsla,
    /// The background color of a hovered element, like a button being hovered
    /// with a mouse, or hovered on a touch screen.
    /// - TODO: Map to step 4.
    pub filled_element_hover: Hsla,
    /// The background color of an active element, like a button being pressed,
    /// or tapped on a touch screen.
    /// - TODO: Map to step 5.
    pub filled_element_active: Hsla,
    /// The background color of a selected element, like a selected tab,
    /// a button toggled on, or a checkbox that is checked.
    pub filled_element_selected: Hsla,
    pub filled_element_disabled: Hsla,
    pub ghost_element: Hsla,
    /// The background color of a hovered element with no default background,
    /// like a ghost-style button or an interactable list item.
    /// - TODO: Map to step 3.
    pub ghost_element_hover: Hsla,
    /// - TODO: Map to step 4.
    pub ghost_element_active: Hsla,
    pub ghost_element_selected: Hsla,
    pub ghost_element_disabled: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,

    pub icon: Hsla,
    pub icon_muted: Hsla,
    pub icon_disabled: Hsla,
    pub icon_placeholder: Hsla,
    pub icon_accent: Hsla,

    pub syntax: SyntaxColor,

    pub status_bar: Hsla,
    pub title_bar: Hsla,
    pub toolbar: Hsla,
    pub tab_bar: Hsla,
    /// The background of the editor
    pub editor: Hsla,
    pub editor_subheader: Hsla,
    pub editor_active_line: Hsla,
    pub terminal: Hsla,
    pub image_fallback_background: Hsla,

    pub created: Hsla,
    pub modified: Hsla,
    pub deleted: Hsla,
    pub conflict: Hsla,
    pub hidden: Hsla,
    pub ignored: Hsla,
    pub renamed: Hsla,
    pub error: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
    pub success: Hsla,

    pub git_created: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
    pub git_conflict: Hsla,
    pub git_ignored: Hsla,
    pub git_renamed: Hsla,

    pub player: [PlayerThemeColors; 8],
}

impl ThemeColor {
    pub fn new() -> Self {
        let scales = default_colors();
        let transparent = hsla(0.0, 0.0, 0.0, 0.0);

        let neutral = ColorScale::Slate;
        let focus_color = ColorScale::Indigo;
        let accent_color = ColorScale::Blue;

        let background = neutral.clone().value(3);
        let surface = neutral.clone().value(2);
        let editor = neutral.clone().value(1);

        let players = [
            PlayerThemeColors {
                cursor: ColorScale::Blue.value(9),
                selection: ColorScale::Blue.value(4),
            },
            PlayerThemeColors {
                cursor: ColorScale::Green.value(9),
                selection: ColorScale::Green.value(4),
            },
            PlayerThemeColors {
                cursor: ColorScale::Red.value(9),
                selection: ColorScale::Red.value(4),
            },
            PlayerThemeColors {
                cursor: ColorScale::Yellow.value(9),
                selection: ColorScale::Yellow.value(4),
            },
            PlayerThemeColors {
                cursor: ColorScale::Purple.value(9),
                selection: ColorScale::Purple.value(4),
            },
            PlayerThemeColors {
                cursor: ColorScale::Cyan.value(9),
                selection: ColorScale::Cyan.value(4),
            },
            PlayerThemeColors {
                cursor: ColorScale::Orange.value(9),
                selection: ColorScale::Orange.value(4),
            },
            PlayerThemeColors {
                cursor: ColorScale::Pink.value(9),
                selection: ColorScale::Pink.value(4),
            },
        ];

        Self {
            transparent,
            mac_os_traffic_light_red: rgb::<Hsla>(0xEC695E),
            mac_os_traffic_light_yellow: rgb::<Hsla>(0xF4BF4F),
            mac_os_traffic_light_green: rgb::<Hsla>(0x62C554),
            border: neutral.clone().value(5),
            border_variant: neutral.clone().value(4),
            border_focused: focus_color.clone().value(6),
            border_transparent: transparent,
            elevated_surface: neutral.clone().value(1),
            surface: surface.clone(),
            background: background.clone(),
            filled_element: neutral.clone().value(3),
            filled_element_hover: neutral.clone().value(4),
            filled_element_active: neutral.clone().value(5),
            filled_element_selected: neutral.clone().value(5),
            filled_element_disabled: transparent,
            ghost_element: transparent,
            ghost_element_hover: neutral.clone().value(4),
            ghost_element_active: neutral.clone().value(5),
            ghost_element_selected: neutral.clone().value(5),
            ghost_element_disabled: transparent,
            text: neutral.clone().value(12),
            text_muted: neutral.clone().value(11),
            text_placeholder: neutral.clone().value(11),
            text_disabled: neutral.clone().value(10),
            text_accent: accent_color.clone().value(11),
            icon: neutral.clone().value(12),
            icon_muted: neutral.clone().value(11),
            icon_placeholder: neutral.clone().value(11),
            icon_disabled: neutral.clone().value(10),
            icon_accent: accent_color.clone().value(11),
            syntax: SyntaxColor {
                comment: neutral.clone().value(11),
                keyword: ColorScale::Orange.clone().value(12),
                string: ColorScale::Lime.clone().value(12),
                function: ColorScale::Amber.clone().value(1),
            },
            status_bar: background.clone(),
            title_bar: background.clone(),
            toolbar: editor.clone(),
            tab_bar:surface.clone(),
            editor: editor.clone(),
            editor_subheader: surface.clone(),
            terminal: editor.clone(),
            editor_active_line: neutral.clone().value(3),

            image_fallback_background: neutral.clone().value(1),

            created: ColorScale::Green.clone().value(11),
            modified: ColorScale::Amber.clone().value(11),
            deleted: ColorScale::Red.clone().value(11),
            conflict: ColorScale::Red.clone().value(11),
            hidden: neutral.clone().value(11),
            ignored: neutral.clone().value(11),
            renamed: ColorScale::Iris.clone().value(11),
            error: ColorScale::Red.clone().value(11),
            warning: ColorScale::Amber.clone().value(11),
            info: ColorScale::Blue.clone().value(11),
            success: ColorScale::Green.clone().value(11),

            git_created: ColorScale::Green.clone().value(11),
            git_modified: ColorScale::Amber.clone().value(11),
            git_deleted: ColorScale::Red.clone().value(11),
            git_conflict: ColorScale::Red.clone().value(11),
            git_ignored: neutral.clone().value(11),
            git_renamed: ColorScale::Iris.clone().value(11),

            player: players,
        }
    }
}

/// Colors used exclusively for syntax highlighting.
///
/// For now we deserialize these from a theme.
/// These will be defined statically in the new theme.
#[derive(Default, PartialEq, EnumIter, Clone, Copy)]
pub enum HighlightColor {
    #[default]
    Default,
    Comment,
    String,
    Function,
    Keyword,
}

impl HighlightColor {
    pub fn hsla(&self) -> Hsla {
        let color = ThemeColor::new();

        match self {
            Self::Default => color.text,
            Self::Comment => color.syntax.comment,
            Self::String => color.syntax.string,
            Self::Function => color.syntax.function,
            Self::Keyword => color.syntax.keyword,
        }
    }
}

pub fn default_colors() -> ThemeScales {
    ThemeScales {
        name: "Default Scales".into(),
        scales: vec![
            ScaleEnum::Standard(ColorScale::Gray.into()),
            ScaleEnum::Standard(ColorScale::Mauve.into()),
            ScaleEnum::Standard(ColorScale::Slate.into()),
            ScaleEnum::Standard(ColorScale::Sage.into()),
            ScaleEnum::Standard(ColorScale::Olive.into()),
            ScaleEnum::Standard(ColorScale::Sand.into()),
            ScaleEnum::Standard(ColorScale::Gold.into()),
            ScaleEnum::Standard(ColorScale::Bronze.into()),
            ScaleEnum::Standard(ColorScale::Brown.into()),
            ScaleEnum::Standard(ColorScale::Yellow.into()),
            ScaleEnum::Standard(ColorScale::Amber.into()),
            ScaleEnum::Standard(ColorScale::Orange.into()),
            ScaleEnum::Standard(ColorScale::Tomato.into()),
            ScaleEnum::Standard(ColorScale::Red.into()),
            ScaleEnum::Standard(ColorScale::Crimson.into()),
            ScaleEnum::Standard(ColorScale::Pink.into()),
            ScaleEnum::Standard(ColorScale::Plum.into()),
            ScaleEnum::Standard(ColorScale::Purple.into()),
            ScaleEnum::Standard(ColorScale::Violet.into()),
            ScaleEnum::Standard(ColorScale::Iris.into()),
            ScaleEnum::Standard(ColorScale::Indigo.into()),
            ScaleEnum::Standard(ColorScale::Blue.into()),
            ScaleEnum::Standard(ColorScale::Cyan.into()),
            ScaleEnum::Standard(ColorScale::Teal.into()),
            ScaleEnum::Standard(ColorScale::Jade.into()),
            ScaleEnum::Standard(ColorScale::Green.into()),
            ScaleEnum::Standard(ColorScale::Grass.into()),
            ScaleEnum::Standard(ColorScale::Lime.into()),
            ScaleEnum::Standard(ColorScale::Mint.into()),
            ScaleEnum::Standard(ColorScale::Sky.into()),
            // ScaleEnum::Standard(ColorScale::Black.into()),
            // ScaleEnum::Standard(ColorScale::White.into()),
        ]
    }
}
