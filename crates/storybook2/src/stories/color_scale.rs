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
                    v_stack().gap_0p5().p_2().children(
                        default_scales.scales.iter().map(move |scale_enum| {
                            let scale_steps = match scale_enum {
                                ScaleEnum::Standard(scale) => &scale.steps,
                                ScaleEnum::Custom(custom_scale) => &custom_scale.steps,
                            };
                            move || h_stack().gap_0p5().children(
                                scale_steps.iter().map(|color| div().w_8().h_4().bg(color.value.clone())),
                            )
                        }).map(|f| f())
                    )
                )
        })
    }
}
