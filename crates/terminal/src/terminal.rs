use alacritty_terminal::{
    config::{Config, PtyConfig},
    grid::Scroll,
    term::SizeInfo,
    Term,
};
use gpui::{actions, elements::*, Entity, MutableAppContext, View, ViewContext, ViewHandle};
use project::{Project, ProjectPath};
use settings::Settings;
use smallvec::SmallVec;
use util::ResultExt;
use workspace::{Item, Workspace};

actions!(terminal, [Deploy]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Terminal::deploy);
}
//2 modes to keep track of:
//Normal command history mode where you're appending to a log
//Full control mode where the terminal has total control over rendering
pub struct Terminal {}

impl Entity for Terminal {
    type Event = ();
}

impl Terminal {
    fn init() -> Terminal {
        //Need to synthesize a config (probably easy)
        //Size Info (probably easy)
        //Event_Proxy tho...

        let terminal_size = SizeInfo::new(10.0, 10.0, 1.0, 1.0, 1.0, 1.0, false);
        //TODO:
        //1. See if we need to implement EventListener in Zed, or if they have one
        //2. See about the rendering of the terminal
        let t = Term::new(&Config::default(), terminal_size, terminal_size);
        t.scroll_to_point();
        Terminal {}
    }

    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        dbg!("HERERERER");
        let project = workspace.project().clone();
        if project.read(cx).is_remote() {
            cx.propagate_action();
        } else if let Some(buffer) = project
            .update(cx, |project, cx| project.create_buffer("", None, cx))
            .log_err()
        {
            workspace.add_item(Box::new(cx.add_view(|_cx| Terminal::init())), cx);
        }
    }
}

impl View for Terminal {
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

///Item is what workspace uses for deciding what to render in a pane
///Often has a file path or somesuch
impl Item for Terminal {
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

    fn project_path(&self, _cx: &gpui::AppContext) -> Option<ProjectPath> {
        None
    }

    fn project_entry_ids(&self, _cx: &gpui::AppContext) -> SmallVec<[project::ProjectEntryId; 3]> {
        todo!()
    }

    fn is_singleton(&self, _cx: &gpui::AppContext) -> bool {
        false
    }

    fn set_nav_history(&mut self, _: workspace::ItemNavHistory, _: &mut ViewContext<Self>) {}

    fn can_save(&self, _cx: &gpui::AppContext) -> bool {
        false
    }

    fn save(
        &mut self,
        _project: gpui::ModelHandle<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        unreachable!("save should not have been called");
    }

    fn save_as(
        &mut self,
        _project: gpui::ModelHandle<Project>,
        _abs_path: std::path::PathBuf,
        _cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        unreachable!("save_as should not have been called");
    }

    fn reload(
        &mut self,
        _project: gpui::ModelHandle<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        gpui::Task::ready(Ok(()))
    }
}
