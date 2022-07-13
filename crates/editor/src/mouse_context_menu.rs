use std::any::{Any, TypeId};

use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    geometry::vector::Vector2F, impl_internal_actions, Action, MutableAppContext, ViewContext,
    ViewHandle,
};
use workspace::GoBack;

use crate::{
    DisplayPoint, Editor, EditorMode, FindAllReferences, GoToDefinition, Rename, SelectMode,
    ToggleCodeActions,
};

#[derive(Clone, PartialEq)]
pub struct DeployMouseContextMenu {
    pub position: Vector2F,
    pub point: DisplayPoint,
}

impl_internal_actions!(editor, [DeployMouseContextMenu]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(deploy_context_menu);
}

pub struct MouseContextMenuState {
    pub view: ViewHandle<ContextMenu>,
    pub opened_at: Option<DisplayPoint>,
}

impl MouseContextMenuState {
    pub fn new(cx: &mut ViewContext<Editor>) -> Self {
        let editor_handle = cx.weak_handle();
        let view = cx.add_view(|cx| {
            let mut menu = ContextMenu::new(cx);
            // Move the cursor to the relevant point before dispatching the action
            menu.on_before_confirm(move |action, cx| {
                if !Self::should_move_cursor(action) {
                    return;
                }

                if let Some(editor_handle) = editor_handle.upgrade(cx) {
                    editor_handle.update(cx, |editor, cx| {
                        if let Some(opened_at) = editor.mouse_context_menu_state.opened_at.take() {
                            editor.change_selections(None, cx, |s| {
                                s.clear_disjoint();
                                s.set_pending_display_range(
                                    opened_at..opened_at,
                                    SelectMode::Character,
                                );
                            });
                        }
                    })
                }
            });
            menu
        });

        Self {
            view,
            opened_at: None,
        }
    }

    pub fn should_move_cursor(action: &Box<dyn Action>) -> bool {
        action.type_id() != TypeId::of::<GoBack>()
    }
}

pub fn deploy_context_menu(
    editor: &mut Editor,
    &DeployMouseContextMenu { position, point }: &DeployMouseContextMenu,
    cx: &mut ViewContext<Editor>,
) {
    // Don't show context menu for inline editors
    if editor.mode() != EditorMode::Full {
        return;
    }

    // Don't show the context menu if there isn't a project associated with this editor
    if editor.project.is_none() {
        return;
    }

    // Store point in the state so that we can move the cursor to that position
    // before dispatching an action
    editor.mouse_context_menu_state.opened_at = Some(point);

    editor.mouse_context_menu_state.view.update(cx, |menu, cx| {
        menu.show(
            position,
            vec![
                ContextMenuItem::item("Back", GoBack { pane: None }),
                ContextMenuItem::Separator,
                ContextMenuItem::item("Rename Symbol", Rename),
                ContextMenuItem::item("Go To Definition", GoToDefinition),
                ContextMenuItem::item("Find All References", FindAllReferences),
                ContextMenuItem::item(
                    "Code Actions",
                    ToggleCodeActions {
                        deployed_from_indicator: false,
                    },
                ),
            ],
            cx,
        );
    });
    cx.notify();
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::test::EditorLspTestContext;

    use super::*;

    #[gpui::test]
    async fn test_mouse_context_menu(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            fn te|st()
                do_work();"});
        let point = cx.display_point(indoc! {"
            fn test()
                do_w|ork();"});
        cx.update_editor(|editor, cx| {
            deploy_context_menu(
                editor,
                &DeployMouseContextMenu {
                    position: Default::default(),
                    point,
                },
                cx,
            )
        });

        cx.assert_editor_state(indoc! {"
            fn test()
                do_w|ork();"});
        cx.editor(|editor, app| assert!(editor.mouse_context_menu_state.view.read(app).visible()));
    }
}
