use gpui2::{div, view, white, Context, ParentElement, Styled, View, WindowContext};
use ui::{h_stack, default_colors, v_stack, ScaleEnum};

pub struct ColorScaleStory {
    text: View<()>,
}

impl ColorScaleStory {
    pub fn view(cx: &mut WindowContext) -> View<()> {
        let default_scales = default_colors();

        view(cx.entity(|cx| ()), move |_, cx| {
            div()
                .size_full()
                .bg(white())
                .child(
                    v_stack().gap_1().p_2().children(
                        default_scales.scales.iter().map(move |scale_enum| {
                            let scale_steps = match scale_enum {
                                ScaleEnum::Standard(scale) => &scale.steps,
                                ScaleEnum::Custom(custom_scale) => &custom_scale.steps,
                            };
                            move || h_stack().gap_1().children(
                                scale_steps.iter().map(|color| div().size_8().bg(color.value.clone())),
                            )
                        }).map(|f| f())
                    )
                )
        })
    }
}

// let gray_scale = build_default_scale(ui::ColorScale::Gray);
// let ruby_scale = build_default_scale(ui::ColorScale::Ruby);
// let custom_color_scale = NewCustomScale::new_from_hsla(Some("Custom Test".into()), to_gpui_hsla(359., 94., 87., 1.));
// let custom_color_scale = NewCustomScale::new_from_hsla(Some("Custom Test".into()), to_gpui_hsla(119., 77., 81., 1.));
// let custom_color_scale = NewCustomScale::new_from_hsla(Some("Custom Test".into()), hsla(0.6305555556, 0.75, 0.52, 1.));

// let gray_steps = gray_scale.steps.clone();
// let ruby_steps = ruby_scale.steps.clone();
