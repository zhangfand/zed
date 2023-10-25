use gpui2::{div, view, white, Context, ParentElement, Styled, View, WindowContext, Element};
use ui::{h_stack, v_stack, ScaleType, Label};

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
                            .child(Label::new(scale_name))
                            .child(
                                h_stack().h_4().gap_1().children(
                                    scale_steps
                                        .iter()
                                        .map(|color| div().flex_1().bg(color.value.clone())),
                                ))
                    }
                })
                .map(|f| f()),
        )
    }

    pub fn apperance_scales(theme_appearance: ui::theme2::ThemeAppearance) -> impl Element<ViewState = ()> {
        let scales = vec![
            theme_appearance.scales.0.neutral.clone(),
            theme_appearance.scales.0.accent.clone(),
            theme_appearance.scales.0.positive.clone(),
            theme_appearance.scales.0.negative.clone(),
            theme_appearance.scales.0.caution.clone()
        ].into_iter().chain(theme_appearance.scales.1.clone().into_iter()).collect::<Vec<_>>();

        Self::render_scales(scales)
    }

    pub fn render_theme_info(theme: ui::theme2::Theme) -> impl Element<ViewState = ()> {
        let theme_family = theme.name;
        let theme_author = theme.author.unwrap_or("No Author set".to_string());
        let theme_url = theme.url.unwrap_or("No Url set".to_string());
        let theme_apperances = theme.appearances;
        let theme_apperances_cloned = theme_apperances.clone();
        let theme_default_appearance = theme_apperances_cloned.get(theme.default_appearance).expect("Default appearance index out of bounds");

        v_stack().w_full().p_8()
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
        // - [ ] Theme Appearance name (id) - (Dark, Light)
        // - [x] Theme Apperance Scales

        let theme = ui::solarized::solarized();

        view(cx.entity(|cx| ()), move |_, cx| {
            v_stack().size_full().flex_1().bg(white())
                .child(
                    Self::render_theme_info(theme.clone())
                )
        })
    }
}
