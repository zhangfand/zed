use alacritty_terminal::{config::Config, event::EventListener, term::SizeInfo, Term};
use gpui::{
    actions,
    color::Color,
    elements::*,
    fonts::{with_font_cache, TextStyle},
    geometry::{rect::RectF, vector::vec2f},
    text_layout::Line,
    Entity, MutableAppContext, View, ViewContext,
};
use project::{Project, ProjectPath};
use settings::Settings;
use smallvec::SmallVec;
use util::ResultExt;
use workspace::{Item, Workspace};

mod event_listener;

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
    fn new() -> Terminal {
        //Basic Alacritty terminal architecture:
        //- Need to create an alacritty_terminal::event::EventListener impl
        //  (so the terminal can control title & such)
        //- Need to use their alacritty::tty::new() stuff to spin up a terminal
        //  + Details like setting up enviroment variables in sub process
        //- Need to create and store a logical terminal (alacritty_terminal::term::Term)
        //- Then when rendering, query the logical terminal and draw the output somehow
        //-Hints:
        //- Look at alacritty::WindowContext::new() for how to wire things up together
        //- Look at display for hints on how to query Terminal

        Terminal {}
    }

    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        let project = workspace.project().clone();
        if project.read(cx).is_remote() {
            cx.propagate_action();
        } else if let Some(_) = project
            .update(cx, |project, cx| project.create_buffer("", None, cx))
            .log_err()
        {
            workspace.add_item(Box::new(cx.add_view(|_cx| Terminal::new())), cx);
        }
    }
}

impl View for Terminal {
    fn ui_name() -> &'static str {
        "TerminalView"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        TerminalEl::new().boxed()
    }
}

struct TerminalEl {
    grid_data: String,
}

impl TerminalEl {
    fn new() -> TerminalEl {
        let grid_data = r#"mikayla@Mikaylas-MacBook-Air zed % ls -aslF
            total 352
            0 drwxr-xr-x  19 mikayla  staff     608 Jun 13 08:59 ./
            0 drwxr-xr-x  32 mikayla  staff    1024 Jun 15 10:26 ../
            8 -rw-r--r--   1 mikayla  staff      35 Jun 13 08:59 .dockerignore
            0 drwxr-xr-x  15 mikayla  staff     480 Jun 15 11:36 .git/
            0 drwxr-xr-x   3 mikayla  staff      96 Jun 13 08:59 .github/
            8 -rw-r--r--   1 mikayla  staff     169 Jun 13 08:59 .gitignore
            0 drwxr-xr-x   3 mikayla  staff      96 Jun 13 08:59 .vscode/
            288 -rw-r--r--   1 mikayla  staff  144472 Jun 14 16:53 Cargo.lock
            8 -rw-r--r--   1 mikayla  staff    1021 Jun 13 08:59 Cargo.toml
            8 -rw-r--r--   1 mikayla  staff     733 Jun 13 08:59 Dockerfile
            8 -rw-r--r--   1 mikayla  staff     519 Jun 13 08:59 Dockerfile.migrator
            8 -rw-r--r--   1 mikayla  staff      83 Jun 13 08:59 Procfile
            16 -rw-r--r--   1 mikayla  staff    5773 Jun 13 08:59 README.md
            0 drwxr-xr-x   6 mikayla  staff     192 Jun 13 08:59 assets/
            0 drwxr-xr-x  44 mikayla  staff    1408 Jun 14 12:52 crates/
            0 drwxr-xr-x   4 mikayla  staff     128 Jun 13 08:59 docs/
            0 drwxr-xr-x  13 mikayla  staff     416 Jun 13 08:59 script/
            0 drwxr-xr-x   9 mikayla  staff     288 Jun 13 09:00 styles/
            0 drwxr-xr-x@  5 mikayla  staff     160 Jun 13 09:02 target/
            mikayla@Mikaylas-MacBook-Air zed %"#;

        TerminalEl {
            grid_data: grid_data.to_string(),
        }
    }
}

struct LayoutState {
    lines: Vec<Line>,
    line_height: f32,
}

impl Element for TerminalEl {
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        cx: &mut gpui::LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        let chunks = vec![(self.grid_data.as_str(), None)].into_iter();

        let text_style = with_font_cache(cx.font_cache.clone(), || TextStyle {
            color: Color::white(),
            ..Default::default()
        }); //Here it's 14?

        let shaped_lines = layout_highlighted_chunks(
            chunks,
            &text_style,
            cx.text_layout_cache,
            &cx.font_cache,
            usize::MAX,
            self.grid_data.matches('\n').count() + 1,
        );
        let line_height = cx.font_cache.line_height(text_style.font_size);

        (
            constraint.max,
            LayoutState {
                lines: shaped_lines,
                line_height,
            },
        )
    }

    fn paint(
        &mut self,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        cx: &mut gpui::PaintContext,
    ) -> Self::PaintState {
        let mut origin = bounds.origin();
        dbg!(layout.line_height);

        for line in &layout.lines {
            let boundaries = RectF::new(origin, vec2f(bounds.width(), layout.line_height));
            dbg!(origin.y(), boundaries.max_y());

            if boundaries.intersects(visible_bounds) {
                line.paint(origin, visible_bounds, layout.line_height, cx);
            }

            origin.set_y(boundaries.max_y());
        }
    }

    fn dispatch_event(
        &mut self,
        _event: &gpui::Event,
        _bounds: gpui::geometry::rect::RectF,
        _visible_bounds: gpui::geometry::rect::RectF,
        _layout: &mut Self::LayoutState,
        _paint: &mut Self::PaintState,
        _cx: &mut gpui::EventContext,
    ) -> bool {
        false
        // unreachable!("Should never be called hopefully")
    }

    fn debug(
        &self,
        _bounds: gpui::geometry::rect::RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        _cx: &gpui::DebugContext,
    ) -> gpui::serde_json::Value {
        unreachable!("Should never be called hopefully")
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
