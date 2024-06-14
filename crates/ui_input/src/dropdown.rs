use gpui::ClickEvent;
use ui::{popover_menu, prelude::*, ButtonLike, ContextMenu, PopoverMenuHandle};

#[derive(Clone, IntoElement)]
pub struct DropdownTrigger {
    id: ElementId,
    disabled: bool,
    selected: bool,
    value: SharedString,
    width: Option<DefiniteLength>,
}

impl DropdownTrigger {
    pub fn new(id: impl Into<ElementId>, label: SharedString) -> Self {
        Self {
            id: id.into(),
            disabled: false,
            selected: false,
            value: label,
            width: None,
        }
    }
}

impl Clickable for DropdownTrigger {
    fn on_click(self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self
    }
}

impl Selectable for DropdownTrigger {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Disableable for DropdownTrigger {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl FixedWidth for DropdownTrigger {
    fn width(mut self, width: DefiniteLength) -> Self {
        self.width = Some(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.width = Some(relative(1.));
        self
    }
}

impl RenderOnce for DropdownTrigger {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let disabled = self.disabled;
        let full_width = self.width == Some(relative(1.));

        ButtonLike::new(self.id).child(
            h_flex()
                .justify_between()
                .rounded_md()
                .bg(cx.theme().colors().editor_background)
                .pl_2()
                .pr_1p5()
                .py_0p5()
                .gap_2()
                .min_w_20()
                .when_else(
                    full_width,
                    |full_width| full_width.w_full(),
                    |auto_width| auto_width.flex_none().w_auto(),
                )
                .when_else(
                    disabled,
                    |disabled| disabled.cursor_not_allowed(),
                    |enabled| enabled.cursor_pointer(),
                )
                .child(Label::new(self.value).color(if disabled {
                    Color::Disabled
                } else {
                    Color::Default
                }))
                .child(
                    Icon::new(IconName::ChevronUpDown)
                        .size(IconSize::XSmall)
                        .color(if disabled {
                            Color::Disabled
                        } else {
                            Color::Muted
                        }),
                ),
        )
    }
}

#[derive(Clone, IntoElement)]
pub struct DropdownMenu {
    pub id: ElementId,
    disabled: bool,
    full_width: bool,
    /// This can be created using PopoverMenuHandle::default()
    handle: PopoverMenuHandle<ContextMenu>,
    possible_values: Vec<SharedString>,
    trigger: DropdownTrigger,
    value: SharedString,
}

impl DropdownMenu {
    pub fn new(
        id: impl Into<ElementId>,
        current_item: SharedString,
        handle: PopoverMenuHandle<ContextMenu>,
        _cx: &WindowContext,
    ) -> Self {
        let id = id.into();
        let trigger_id = ElementId::Name(format!("{}_trigger", id.clone()).into());
        let trigger = DropdownTrigger::new(trigger_id, current_item.clone());

        Self {
            id,
            disabled: false,
            full_width: false,
            handle,
            possible_values: Vec::new(),
            trigger,
            value: current_item,
        }
    }

    pub fn current_item(mut self, current_item: SharedString) -> Self {
        self.value = current_item;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }
}

impl Disableable for DropdownMenu {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let disabled = self.disabled;
        let trigger_id = ElementId::Name(format!("{}_trigger", self.id.clone()).into());

        popover_menu(self.id.clone())
            .with_handle(self.handle)
            .menu(move |cx| {
                let items = self.possible_values.clone();
                ContextMenu::build(cx, |mut menu, _| {
                    for item in items {
                        menu = menu.custom_entry(
                            move |_| Label::new(item.clone()).into_any_element(),
                            {
                                // TODO: Pass item handler
                                move |_| {}
                            },
                        )
                    }
                    menu
                })
                .into()
            })
            .trigger(self.trigger)
            .anchor(gpui::AnchorCorner::BottomRight)
    }
}
