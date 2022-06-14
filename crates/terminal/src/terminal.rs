use editor::Editor;
use gpui::{actions, elements::*, Entity, MutableAppContext, View, ViewContext};
use project::{Project, ProjectPath};
use settings::Settings;
use smallvec::SmallVec;
use util::ResultExt;
use workspace::{Item, Workspace};

actions!(terminal, [Deploy]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Terminal::deploy);
}

pub struct Terminal {}

pub struct TerminalView {}

impl Entity for Terminal {
    type Event = ();
}

impl Entity for TerminalView {
    type Event = ();
}

impl Terminal {
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        dbg!("HERERERER");
        let project = workspace.project().clone();
        if project.read(cx).is_remote() {
            cx.propagate_action();
        } else if let Some(buffer) = project
            .update(cx, |project, cx| project.create_buffer("", None, cx))
            .log_err()
        {
            workspace.add_item(
                Box::new(cx.add_view(|cx| Editor::for_buffer(buffer, Some(project.clone()), cx))),
                cx,
            );
        }
    }
}

impl View for TerminalView {
    fn ui_name() -> &'static str {
        "TerminalView"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let style = &cx
            .global::<Settings>()
            .theme
            .search
            .option_button
            .style_for(Default::default(), true);

        Label::new("Hello!".into(), style.text.clone())
            .contained()
            .with_style(style.container)
            .boxed()
    }
}

impl Item for TerminalView {
    fn tab_content(&self, style: &theme::Tab, cx: &gpui::AppContext) -> ElementBox {
        let settings = cx.global::<Settings>();
        let search_theme = &settings.theme.search;
        Flex::row()
            .with_child(
                Svg::new("icons/magnifier.svg")
                    .with_color(style.label.text.color)
                    .constrained()
                    .with_width(search_theme.tab_icon_width)
                    .aligned()
                    .boxed(),
            )
            .with_child(
                Label::new("Terminal".into(), style.label.clone())
                    .aligned()
                    .contained()
                    .with_margin_left(search_theme.tab_icon_spacing)
                    .boxed(),
            )
            .boxed()
    }

    fn project_path(&self, cx: &gpui::AppContext) -> Option<ProjectPath> {
        todo!()
    }

    fn project_entry_ids(&self, cx: &gpui::AppContext) -> SmallVec<[project::ProjectEntryId; 3]> {
        todo!()
    }

    fn is_singleton(&self, cx: &gpui::AppContext) -> bool {
        todo!()
    }

    fn set_nav_history(&mut self, _: workspace::ItemNavHistory, _: &mut ViewContext<Self>) {
        todo!()
    }

    fn can_save(&self, cx: &gpui::AppContext) -> bool {
        todo!()
    }

    fn save(
        &mut self,
        project: gpui::ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        todo!()
    }

    fn save_as(
        &mut self,
        project: gpui::ModelHandle<Project>,
        abs_path: std::path::PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        todo!()
    }

    fn reload(
        &mut self,
        project: gpui::ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        todo!()
    }
}

// impl View for Terminal {
//     fn ui_name() -> &'static str {
//         "Terminal"
//     }

//     fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
//     }
// }
