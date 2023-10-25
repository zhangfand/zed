use gpui2::{div, view, white, Context, ParentElement, Styled, View, WindowContext};
use ui::{h_stack, NewCustomScale, to_gpui_hsla};

pub struct ColorScaleStory {
    text: View<()>,
}

impl ColorScaleStory {
    pub fn view(cx: &mut WindowContext) -> View<()> {
        let custom_color_scale = NewCustomScale::new_from_hsla(Some("Custom Test".into()), to_gpui_hsla(359., 94., 87., 1.));
        // let custom_color_scale = NewCustomScale::new_from_hsla(Some("Custom Test".into()), to_gpui_hsla(119., 77., 81., 1.));
        // let custom_color_scale = NewCustomScale::new_from_hsla(Some("Custom Test".into()), hsla(0.6305555556, 0.75, 0.52, 1.));

        let steps = custom_color_scale.steps.clone();

        view(cx.entity(|cx| ()), move |_, cx| {
            div()
                .size_full()
                .bg(white())
                .child(
                    h_stack().gap_1().children(
                        steps.iter().map(|color| {
                            div()
                                .size_8()
                                .bg(color.value.clone())
                        })
                    )
                )
        })
    }
}
