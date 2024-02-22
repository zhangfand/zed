use collab_ui::FacePile;
use gpui::smallvec::smallvec;
use story::run_story;
use ui::{prelude::*, Avatar};

struct FacepileExample;

impl Render for FacepileExample {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x555555))
            .child(FacePile::new(smallvec![
                Avatar::new("https://avatars.githubusercontent.com/nathansobo").into_any_element(),
                Avatar::new("https://avatars.githubusercontent.com/as-cii").into_any_element(),
                Avatar::new("https://avatars.githubusercontent.com/maxbrunsfeld")
                    .into_any_element(),
            ]))
    }
}

fn main() {
    run_story(|_cx| FacepileExample);
}
