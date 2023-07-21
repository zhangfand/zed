pub mod zed {
    use anyhow::Result;
    use gpui::{
        actions,
        elements::{Label, MouseEventHandler},
        AppContext, Element, Entity, ModelHandle, Task, View, ViewContext, ViewHandle,
        WeakViewHandle,
    };
    use workspace::{
        item::Item, register_deserializable_item, ItemId, Pane, Workspace, WorkspaceId,
    };

    use crate::{animator::Animator, element::AnimatedBoxElement};

    actions!(animation_test, [Deploy]);

    pub fn init(cx: &mut AppContext) {
        cx.add_action(
            |workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>| {
                let animation_item = cx.add_view(|cx| AnimationTestItem::new(cx));
                workspace.add_item(Box::new(animation_item), cx);
            },
        );

        register_deserializable_item::<AnimationTestItem>(cx)
    }

    pub struct AnimationTestItem {
        animation: ModelHandle<Animator>,
    }

    impl AnimationTestItem {
        fn new(cx: &mut ViewContext<Self>) -> Self {
            let animation = cx.add_model(|cx| Animator::new(cx));
            cx.observe(&animation, |_, _, cx| cx.notify()).detach();

            AnimationTestItem { animation }
        }
    }

    impl Entity for AnimationTestItem {
        type Event = ();
    }

    impl View for AnimationTestItem {
        fn ui_name() -> &'static str {
            "AnimationTest"
        }

        fn render(&mut self, cx: &mut gpui::ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
            enum BoxClick {}
            workspace::pane::PaneBackdrop::new(
                cx.view_id(),
                MouseEventHandler::<BoxClick, _>::new(0, cx, |_, cx| {
                    Animator::render(&self.animation, cx, |cx| {
                        AnimatedBoxElement::new(cx).into_any()
                    })
                })
                .on_click(gpui::platform::MouseButton::Left, |e, v: &mut Self, cx| {
                    v.animation.update(cx, |animation, cx| {
                        animation.toggle_play();

                        cx.notify()
                    });
                })
                .into_any(),
            )
            .into_any()
        }
    }

    impl Item for AnimationTestItem {
        fn tab_content<V: View>(
            &self,
            _detail: Option<usize>,
            style: &theme::Tab,
            _cx: &AppContext,
        ) -> gpui::AnyElement<V> {
            Label::new("Animation Test", style.label.clone()).into_any()
        }

        fn serialized_item_kind() -> Option<&'static str> {
            Some("AnimationTestItem")
        }

        fn deserialize(
            _project: ModelHandle<project::Project>,
            _workspace: WeakViewHandle<Workspace>,
            _workspace_id: WorkspaceId,
            _item_id: ItemId,
            cx: &mut ViewContext<Pane>,
        ) -> Task<Result<ViewHandle<Self>>> {
            Task::Ready(Some(Ok(cx.add_view(|cx| Self::new(cx)))))
        }

        fn clone_on_split(&self, _workspace_id: WorkspaceId, cx: &mut ViewContext<Self>) -> Option<Self>
        where
            Self: Sized,
        {
            Some(AnimationTestItem::new(cx))
        }

    }
}

mod animator {
    use std::time::{Duration, Instant};

    use gpui::{AnyElement, Entity, ModelContext, ModelHandle, View, ViewContext};

    #[derive(Debug)]
    pub struct Animator {
        progress: f32,
        playing: bool,
        bounce: bool,
        duration: Duration,
        animation_state: AnimationState,
    }

    #[derive(Debug)]
    struct AnimationState {
        last_frame: Instant,
        bouncing: bool,
    }

    impl Entity for Animator {
        type Event = ();
    }

    const SECONDS_PER_FRAME: f32 = 1. / 60.;

    impl Animator {
        pub fn new(cx: &mut ModelContext<Self>) -> Self {
            let mut timer = Some(
                cx.background()
                    .timer(Duration::from_secs_f32(SECONDS_PER_FRAME)),
            );

            cx.spawn(|this, mut cx| async move {
                loop {
                    let this_timer = timer.take();
                    this_timer.unwrap().await;

                    this.update(&mut cx, |this, cx| {
                        let now = Instant::now();
                        let seconds_since = now
                            .duration_since(this.animation_state.last_frame)
                            .as_secs_f32();


                        if seconds_since < SECONDS_PER_FRAME {
                            // Attempt to reschedule the timer if we've already been drawn for some other reason
                            timer = cx
                                .background()
                                .timer(Duration::from_secs_f32(SECONDS_PER_FRAME - seconds_since))
                                .into();
                        } else {
                            timer = cx
                                .background()
                                .timer(Duration::from_secs_f32(SECONDS_PER_FRAME))
                                .into();
                            cx.notify()
                        }
                    })
                }
            })
            .detach();

            Self {
                progress: 0.,
                playing: true,
                bounce: true,
                animation_state: AnimationState {
                    last_frame: Instant::now(),
                    bouncing: false,
                },
                duration: Duration::from_secs(1),
            }
        }

        pub fn playing(&self) -> bool {
            self.playing
        }

        pub fn toggle_play(&mut self) {
            self.playing = !self.playing;
            self.animation_state.last_frame = Instant::now();
        }

        pub fn pause(&mut self) {
            self.playing = false;
        }

        pub fn play(&mut self) {
            self.playing = true;
            self.animation_state.last_frame = Instant::now();
        }

        pub fn render<V: View>(
            this: &ModelHandle<Animator>,
            cx: &mut ViewContext<V>,
            f: impl FnOnce(&mut AnimationContext<V>) -> AnyElement<V>,
        ) -> AnyElement<V> {
            let progress = this.update(cx, |this, _cx| {
                let now = Instant::now();
                let seconds_since = now
                    .duration_since(this.animation_state.last_frame)
                    .as_secs_f32();

                if this.playing {
                    let total_seconds = this.duration.as_secs_f32();
                    let percent_change = seconds_since / total_seconds;

                    if this.animation_state.bouncing {
                        this.progress -= percent_change;
                    } else {
                        this.progress += percent_change;
                    }

                    if this.progress >= 1. {
                        this.progress = 1.;
                        this.animation_state.bouncing = true;
                    } else if this.progress <= 0. {
                        this.progress = 0.;
                        this.animation_state.bouncing = false;
                    }
                }

                this.animation_state.last_frame = now;

                this.progress
            });


            let mut cx = AnimationContext {
                progress,
                view_cx: cx,
            };

            f(&mut cx)
        }
    }

    pub struct AnimationContext<'a, 'b, 'c, V: View> {
        pub view_cx: &'a mut ViewContext<'b, 'c, V>,
        progress: f32,
    }

    impl<'a, 'b, 'c, V: View> AnimationContext<'a, 'b, 'c, V> {
        pub fn progress(&self) -> f32 {
            self.progress
        }
    }
}

mod element {
    use gpui::{color::Color, geometry::vector::vec2f, Element, View};

    use crate::animator::AnimationContext;

    #[derive(Debug)]
    pub struct AnimatedBoxElement {
        progress: f32,
    }

    impl AnimatedBoxElement {
        pub fn new<V: View>(cx: &mut AnimationContext<V>) -> Self {
            AnimatedBoxElement {
                progress: cx.progress(),
            }
        }
    }

    impl<V: View> Element<V> for AnimatedBoxElement {
        type LayoutState = ();

        type PaintState = ();

        fn layout(
            &mut self,
            constraint: gpui::SizeConstraint,
            _view: &mut V,
            _cx: &mut gpui::LayoutContext<V>,
        ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
            (constraint.max, ())
        }

        fn paint(
            &mut self,
            scene: &mut gpui::SceneBuilder,
            _bounds: gpui::geometry::rect::RectF,
            visible_bounds: gpui::geometry::rect::RectF,
            _layout: &mut Self::LayoutState,
            _view: &mut V,
            _cx: &mut gpui::ViewContext<V>,
        ) -> Self::PaintState {
            let width = self.progress * visible_bounds.width();

            scene.push_quad(gpui::Quad {
                bounds: gpui::geometry::rect::RectF::new(
                    visible_bounds.origin(),
                    vec2f(width, visible_bounds.height()),
                ),
                background: Some(Color::red()),
                ..Default::default()
            })
        }

        fn rect_for_text_range(
            &self,
            _range_utf16: std::ops::Range<usize>,
            _bounds: gpui::geometry::rect::RectF,
            _visible_bounds: gpui::geometry::rect::RectF,
            _layout: &Self::LayoutState,
            _paint: &Self::PaintState,
            _view: &V,
            _cx: &gpui::ViewContext<V>,
        ) -> Option<gpui::geometry::rect::RectF> {
            None
        }

        fn debug(
            &self,
            _bounds: gpui::geometry::rect::RectF,
            _layout: &Self::LayoutState,
            _paint: &Self::PaintState,
            _view: &V,
            _cx: &gpui::ViewContext<V>,
        ) -> gpui::serde_json::Value {
            gpui::serde_json::json!({
                "progress": self.progress,
            })
        }
    }
}
