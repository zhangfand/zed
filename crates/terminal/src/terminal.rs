use std::sync::Arc;

use alacritty_terminal::{
    config::{Config, Program, PtyConfig},
    event_loop::{EventLoop, Msg},
    grid::Dimensions,
    sync::FairMutex,
    term::SizeInfo,
    tty, Term,
};
use event_listener::ZedTranslator;
use gpui::{
    actions,
    color::Color,
    elements::*,
    fonts::{with_font_cache, TextStyle},
    geometry::{rect::RectF, vector::vec2f},
    text_layout::Line,
    Entity, MutableAppContext, View, ViewContext,
};
use mio_extras::channel::Sender;
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
pub struct Terminal {
    loop_tx: Sender<Msg>,
    term: Arc<FairMutex<Term<ZedTranslator>>>,
}

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
        //- Also need to create an event loop for the PTY I/O, term looks for the same
        //  PtyConfig as tty::new(), so they're communicating through kernel
        //-Hints:
        //- Look at alacritty::WindowContext::new() for how to wire things up together
        //- Look at display for hints on how to query Terminal

        //Goals:
        //Not just a crappy terminal,
        //Full zed features like collaboration, multicursor, etc.

        let zed_proxy = ZedTranslator {};

        let pty_config = PtyConfig {
            shell: Some(Program::Just("zsh".to_string())),
            working_directory: None,
            hold: false,
        };

        // TODO: Modify settings to populate the alacritty config
        let config = Config {
            pty_config: pty_config.clone(),
            ..Default::default()
        };
        let size_info = SizeInfo::new(100., 100., 5., 5., 0., 0., false);

        let term = Term::new(&config, size_info, zed_proxy.clone());
        let term = Arc::new(FairMutex::new(term));

        let pty = tty::new(&pty_config, &size_info, None).expect("Could not create tty");

        let event_loop = EventLoop::new(
            Arc::clone(&term),
            zed_proxy.clone(),
            pty,
            pty_config.hold,
            false,
        );

        let loop_tx = event_loop.channel();
        let io_thread = event_loop.spawn();

        Terminal { loop_tx, term }
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

    fn render(&mut self, _cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        TerminalEl::new(self.term.clone()).boxed()
    }
}

struct TerminalEl {
    grid_data: Arc<FairMutex<Term<ZedTranslator>>>,
}

impl TerminalEl {
    fn new(term: Arc<FairMutex<Term<ZedTranslator>>>) -> TerminalEl {
        TerminalEl { grid_data: term }
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
        let line = self
            .grid_data
            .lock()
            .grid()
            .display_iter()
            .map(|c| c.c)
            .collect::<String>();

        let chunks = vec![(&line[..], None)].into_iter();

        let text_style = with_font_cache(cx.font_cache.clone(), || TextStyle {
            color: Color::white(),
            ..Default::default()
        }); //Here it's 14?

        //Nescessary to send the
        let shaped_lines = layout_highlighted_chunks(
            chunks,
            &text_style,
            cx.text_layout_cache,
            &cx.font_cache,
            usize::MAX,
            line.matches('\n').count() + 1,
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
