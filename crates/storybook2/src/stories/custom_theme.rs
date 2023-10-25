use gpui2::{div, view, white, Context, ParentElement, Styled, View, WindowContext, Element};
use ui::{h_stack, v_stack, ScaleType};

pub struct CustomThemeStory {
    text: View<()>,
}

impl CustomThemeStory {
    pub fn render_scales(scales: Vec<ScaleType>) -> impl Element<ViewState = ()> {
        v_stack().w_full().gap_1().children(
            scales
                .iter()
                .map(move |scale_enum| {
                    let scale_steps = match scale_enum {
                        ScaleType::Standard(scale) => &scale.steps,
                        ScaleType::Custom(custom_scale) => &custom_scale.steps,
                    };
                    move || {
                        let scale_name = match scale_enum {
                            ScaleType::Standard(scale) => scale.name.label().clone(),
                            ScaleType::Custom(custom_scale) => custom_scale.name.clone(),
                        };

                        v_stack()
                            .gap_0p5()
                            .child(div().text_sm().child(scale_name))
                            .child(
                                h_stack().gap_1().children(
                                    scale_steps
                                        .iter()
                                        .map(|color| div().size_8().flex_1().bg(color.value.clone())),
                                ))
                    }
                })
                .map(|f| f()),
        )
    }

    pub fn appearance_scales_to_vec(theme_appearance: ui::theme2::ThemeAppearance) -> Vec<ScaleType> {
        vec![
            theme_appearance.scales.0.neutral.clone(),
            theme_appearance.scales.0.accent.clone(),
            theme_appearance.scales.0.positive.clone(),
            theme_appearance.scales.0.negative.clone(),
            theme_appearance.scales.0.caution.clone()
        ].into_iter().chain(theme_appearance.scales.1.clone().into_iter()).collect::<Vec<_>>()
    }

    pub fn render_theme_appearance(theme_appearance: ui::theme2::ThemeAppearance) -> impl Element<ViewState = ()> {
        let appearance = theme_appearance.clone();
        let appearance_name = appearance.name.clone();
        let appearance_scales = Self::appearance_scales_to_vec(appearance.clone());

        let neutral_scale_type = appearance.scales.0.neutral.clone();
        let neutral = match neutral_scale_type {
            ScaleType::Standard(scale) => scale,
            ScaleType::Custom(custom_scale) => custom_scale.into(),
        };

        v_stack().w_full().mx_4().px_4().py_4().rounded_xl().bg(neutral.step_hsla(1).clone()).text_color(neutral.step_hsla(12).clone())
            .child(div().text_2xl().child(appearance_name))
            .child(Self::render_scales(appearance_scales))
    }

    pub fn render_theme_info(theme: ui::theme2::Theme) -> impl Element<ViewState = ()> {
        let theme_family = theme.name;
        let theme_author = theme.author.unwrap_or("No Author set".to_string());
        let theme_url = theme.url.unwrap_or("No Url set".to_string());
        let theme_apperances = theme.appearances;
        let theme_apperances_cloned = theme_apperances.clone();
        let theme_default_appearance = theme_apperances_cloned.get(theme.default_appearance).expect("Default appearance index out of bounds");

        v_stack().w_full().px_8().py_4()
            .child(div().text_2xl().child(theme_family))
            .child(div().text_sm().child(
                format!("Author: {}", theme_author)
            ))
            .child(div().text_sm().child(
                format!("Theme Link: {}", theme_url)
            ))
            .child(
                h_stack().gap_0p5().text_sm()
                    .child("Appearances: ")
                    .children(
                        theme_apperances.iter().enumerate().map(|(i, appearance)| {
                            div().text_sm().child(
                                if i != theme_apperances.len() - 1 {
                                    format!("{}, ", appearance.name.clone())
                                } else {
                                    appearance.name.clone()
                                }
                            )
                        }))
                    .child(
                        div().text_sm().child(" – ")
                    )
                    .child(
                        div().text_sm().child(format!("(Default: {})", theme_default_appearance.name))
                    )
            )
    }

    pub fn view(cx: &mut WindowContext) -> View<()> {
        // TODO:
        // - [x] Theme Family name, author, url, appearances, default_appearance (name not id)
        // - [x] Theme Appearance name (id) - (Dark, Light)
        // - [x] Theme Apperance Scales

        let theme = ui::solarized::solarized();
        let theme_appearances = theme.clone().appearances;

        view(cx.entity(|cx| ()), move |_, cx| {
            v_stack().size_full().flex_1().py_4()
                .bg(white())
                .child(
                    Self::render_theme_info(theme.clone())
                )
                .children(
                    theme_appearances.iter().map(|appearance| {
                        Self::render_theme_appearance(appearance.clone())
                    })
                )
        })
    }
}
