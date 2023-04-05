use crate::{PaneId, PaneItemHandle, Workspace};
use collections::HashMap;
use gpui::{
    elements::*, geometry::rect::RectF, AppContext, Element, ElementBox, LayoutContext,
    MeasurementContext, MutableAppContext, PaintContext, SizeConstraint, WeakViewHandle,
};
use settings::Settings;

pub struct WorkspaceElement(WeakViewHandle<Workspace>);
impl Element for WorkspaceElement {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        todo!()
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        todo!()
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        todo!()
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        cx: &gpui::DebugContext,
    ) -> serde_json::Value {
        todo!()
    }
}

struct TabBarElement {
    workspace: WeakViewHandle<Workspace>,
    pane_id: PaneId,
    autoscroll: bool,
}

impl CompositeElement for TabBarElement {
    // TODO: handle autoscroll

    fn render(&mut self, cx: &mut LayoutContext) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let workspace = self.workspace.upgrade(cx).unwrap();
        let workspace = workspace.read(cx);
        let pane = workspace.pane(self.pane_id).unwrap();
        let pane_active = workspace.active_pane_id == pane.id;
        let pane_items = pane.items();
        let active_item_ix = pane.active_item_index;

        let autoscroll = if self.autoscroll {
            Some(active_item_ix)
        } else {
            None
        };

        let mut row = Flex::row().scrollable::<Self, _>(pane.id, autoscroll, cx);
        // for (ix, (item, detail)) in
        //     pane_items
        //     .iter()
        //     .cloned()
        //     .zip(self.tab_details(cx))
        //     .enumerate()
        // {
        //     let detail = if detail == 0 { None } else { Some(detail) };
        //     let tab_active = ix == self.active_item_index;

        //     row.add_child({
        //         enum TabDragReceiver {}
        //         let mut receiver =
        //             dragged_item_receiver::<TabDragReceiver, _>(ix, ix, true, None, cx, {
        //                 let item = item.clone();
        //                 let pane = pane.clone();
        //                 let detail = detail.clone();

        //                 let theme = cx.global::<Settings>().theme.clone();

        //                 move |mouse_state, cx| {
        //                     let tab_style =
        //                         theme.workspace.tab_bar.tab_style(pane_active, tab_active);
        //                     let hovered = mouse_state.hovered();

        //                     enum Tab {}
        //                     MouseEventHandler::<Tab>::new(ix, cx, |_, cx| {
        //                         Self::render_tab(
        //                             &item,
        //                             pane.clone(),
        //                             ix == 0,
        //                             detail,
        //                             hovered,
        //                             tab_style,
        //                             cx,
        //                         )
        //                     })
        //                     .on_down(MouseButton::Left, move |_, cx| {
        //                         cx.dispatch_action(ActivateItem(ix));
        //                     })
        //                     .on_click(MouseButton::Middle, {
        //                         let item = item.clone();
        //                         move |_, cx: &mut EventContext| {
        //                             cx.dispatch_action(CloseItem {
        //                                 item_id: item.id(),
        //                                 pane: pane.clone(),
        //                             })
        //                         }
        //                     })
        //                     .boxed()
        //                 }
        //             });

        //         if !pane_active || !tab_active {
        //             receiver = receiver.with_cursor_style(CursorStyle::PointingHand);
        //         }

        //         receiver
        //             .as_draggable(
        //                 DraggedItem {
        //                     item,
        //                     pane: pane.clone(),
        //                 },
        //                 {
        //                     let theme = cx.global::<Settings>().theme.clone();

        //                     let detail = detail.clone();
        //                     move |dragged_item, cx: &mut RenderContext<Workspace>| {
        //                         let tab_style = &theme.workspace.tab_bar.dragged_tab;
        //                         Self::render_tab(
        //                             &dragged_item.item,
        //                             dragged_item.pane.clone(),
        //                             false,
        //                             detail,
        //                             false,
        //                             &tab_style,
        //                             cx,
        //                         )
        //                     }
        //                 },
        //             )
        //             .boxed()
        //     })
        // }

        // // Use the inactive tab style along with the current pane's active status to decide how to render
        // // the filler
        // let filler_index = pane_items.len();
        // let filler_style = theme.workspace.tab_bar.tab_style(pane_active, false);
        // enum Filler {}
        // row.add_child(
        //     // dragged_item_receiver::<Filler, _>(0, filler_index, true, None, cx, |_, _| {
        //     //     Empty::new()
        //     //         .contained()
        //     //         .with_style(filler_style.container)
        //     //         .with_border(filler_style.container.border)
        //     //         .boxed()
        //     // })
        //     .flex(1., true)
        //     .named("filler"),
        // );

        Empty::new().boxed()
    }
}

impl TabBarElement {
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
}
