use crate::{PaneId, PaneItemHandle, Workspace};
use collections::HashMap;
use gpui::{
    color::Color, elements::*, AppContext, CursorStyle, Element, ElementBox, LayoutContext,
    MouseButton, ViewContext, WeakViewHandle,
};
use settings::Settings;

pub struct WorkspaceComponent;

struct PaneTreeComponent {
    workspace: WeakViewHandle<Workspace>,
}

struct PaneComponent {
    workspace: WeakViewHandle<Workspace>,
    pane_id: PaneId,
}

struct TabBarComponent {
    workspace: WeakViewHandle<Workspace>,
    pane_id: PaneId,
    autoscroll: bool,
}

impl ViewComponent for WorkspaceComponent {
    type View = Workspace;

    fn render(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> ElementBox {
        todo!()
    }
}

impl Component for PaneComponent {
    fn render(&mut self, cx: &mut LayoutContext) -> ElementBox {
        let workspace = self.workspace.upgrade(cx).unwrap();
        let pane = workspace.read(cx).pane(self.pane_id).unwrap();

        Flex::column()
            .with_child(
                TabBarComponent {
                    workspace: self.workspace.clone(),
                    pane_id: self.pane_id,
                    autoscroll: true,
                }
                .boxed(),
            )
            .with_children(pane.active_item().map(|active_item| {
                ChildView::new(active_item.as_any(), cx)
                    .flex(1., true)
                    .boxed()
            }))
            .boxed()
    }
}

impl Component for TabBarComponent {
    fn render(&mut self, cx: &mut LayoutContext) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let theme = &theme.workspace.tab_bar;
        let workspace = self.workspace.upgrade(cx).unwrap();
        let workspace = workspace.read(cx);
        let pane = workspace.pane(self.pane_id).unwrap();
        let pane_is_active = workspace.active_pane_id == pane.id;
        let pane_items = pane
            .items()
            .iter()
            .map(|i| i.boxed_clone())
            .collect::<Vec<_>>();
        let active_item_ix = pane.active_item_index;

        let autoscroll = if self.autoscroll {
            Some(active_item_ix)
        } else {
            None
        };

        let id = pane.id;
        let mut row = Flex::row().scrollable::<Self, _>(id, autoscroll, cx);

        for (ix, (pane_item, detail_level)) in pane_items
            .iter()
            .zip(Self::tab_detail_levels(&pane_items, cx))
            .enumerate()
        {
            let detail_level = if detail_level == 0 {
                None
            } else {
                Some(detail_level)
            };
            let tab_is_first = ix == 0;
            let tab_is_active = ix == active_item_ix;
            let tab_style = theme.tab_style(pane_is_active, tab_is_active);
            row.add_child(Self::render_tab(
                pane_item,
                tab_is_first,
                detail_level,
                tab_style,
                cx,
            ));
        }

        // Style the filler as if it were an inactive tab in this pane
        let filler_style = theme.tab_style(pane_is_active, false);
        row.add_child(
            Empty::new()
                .contained()
                .with_style(filler_style.container)
                .with_border(filler_style.container.border)
                .flex(1., true)
                .named("filler"),
        );

        row.named("tab bar")
    }
}

impl TabBarComponent {
    fn tab_detail_levels(items: &[Box<dyn PaneItemHandle>], cx: &AppContext) -> Vec<usize> {
        let mut tab_detail_levels = vec![0; items.len()];
        let mut tab_descriptions = HashMap::default();
        let mut done = false;
        while !done {
            done = true;

            // Store item indices by their tab description.
            for (ix, (item, detail_level)) in items.iter().zip(&tab_detail_levels).enumerate() {
                if let Some(description) = item.tab_description(*detail_level, cx) {
                    if *detail_level == 0
                        || Some(&description) != item.tab_description(detail_level - 1, cx).as_ref()
                    {
                        tab_descriptions
                            .entry(description)
                            .or_insert(Vec::new())
                            .push(ix);
                    }
                }
            }

            // If two or more items have the same tab description, increase their level
            // of detail and try again.
            for (_, item_ixs) in tab_descriptions.drain() {
                if item_ixs.len() > 1 {
                    done = false;
                    for ix in item_ixs {
                        tab_detail_levels[ix] += 1;
                    }
                }
            }
        }

        tab_detail_levels
    }

    fn render_tab(
        item: &Box<dyn PaneItemHandle>,
        is_first_tab: bool,
        detail_level: Option<usize>,
        tab_style: &theme::Tab,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        enum Tab {}
        MouseEventHandler::<Tab>::new(item.id(), cx, |mouse_state, cx| {
            let tab_hovered = mouse_state.hovered();

            Flex::row()
                .with_child({
                    let indicator_diameter = 7.0;
                    let indicator_color = if item.has_conflict(cx) {
                        tab_style.icon_conflict
                    } else if item.is_dirty(cx) {
                        tab_style.icon_dirty
                    } else {
                        Color::transparent_black()
                    };

                    Empty::new()
                        .contained()
                        .with_background_color(indicator_color)
                        .with_corner_radius(indicator_diameter / 2.)
                        .constrained()
                        .with_width(indicator_diameter)
                        .with_height(indicator_diameter)
                        .aligned()
                        .boxed()
                })
                .with_child(
                    item.tab_content(detail_level, &tab_style, cx)
                        .aligned()
                        .contained()
                        .with_margin_left(tab_style.spacing)
                        .with_margin_right(tab_style.spacing)
                        .boxed(),
                )
                .with_child(
                    if tab_hovered {
                        enum TabCloseButton {}
                        MouseEventHandler::<TabCloseButton>::new(item.id(), cx, |mouse_state, _| {
                            let icon = Svg::new("icons/x_mark_8.svg");
                            if mouse_state.hovered() {
                                icon.with_color(tab_style.icon_close_active).boxed()
                            } else {
                                icon.with_color(tab_style.icon_close).boxed()
                            }
                        })
                        .with_padding(Padding::uniform(4.))
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(MouseButton::Left, move |_, cx| {
                            // TODO
                            // cx.dispatch_action(CloseItem { item_id, pane_id })
                        })
                        .boxed()
                    } else {
                        Empty::new().boxed()
                    }
                    .constrained()
                    .with_width(tab_style.close_icon_width)
                    .named("close-tab-button"),
                )
                .contained()
                .with_style({
                    let mut container = tab_style.container.clone();
                    if is_first_tab {
                        container.border.left = false;
                    }
                    container
                })
                .constrained()
                .with_height(tab_style.height)
                .boxed()
        })
        .boxed()
    }
}
