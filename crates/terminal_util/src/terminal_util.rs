use alacritty_terminal::{event::WindowSize, grid::Dimensions};
use gpui::geometry::vector::{vec2f, Vector2F};

// These constants are mostly irrelevant. We need to put *some* number in for the initial size, but the terminal will
// be resized immediately on the first draw.
const DEBUG_TERMINAL_WIDTH: f32 = 500.;
const DEBUG_TERMINAL_HEIGHT: f32 = 30.;
const DEBUG_CELL_WIDTH: f32 = 5.;
const DEBUG_LINE_HEIGHT: f32 = 5.;

#[derive(Clone, Copy, Debug)]
pub struct TerminalSize {
    pub cell_width: f32,
    pub line_height: f32,
    pub height: f32,
    pub width: f32,
}

impl TerminalSize {
    pub fn new(line_height: f32, cell_width: f32, size: Vector2F) -> Self {
        TerminalSize {
            cell_width,
            line_height,
            width: size.x(),
            height: size.y(),
        }
    }

    pub fn num_lines(&self) -> usize {
        (self.height / self.line_height).floor() as usize
    }

    pub fn num_columns(&self) -> usize {
        (self.width / self.cell_width).floor() as usize
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn cell_width(&self) -> f32 {
        self.cell_width
    }

    pub fn line_height(&self) -> f32 {
        self.line_height
    }
}
impl Default for TerminalSize {
    fn default() -> Self {
        TerminalSize::new(
            DEBUG_LINE_HEIGHT,
            DEBUG_CELL_WIDTH,
            vec2f(DEBUG_TERMINAL_WIDTH, DEBUG_TERMINAL_HEIGHT),
        )
    }
}

impl From<TerminalSize> for WindowSize {
    fn from(val: TerminalSize) -> Self {
        WindowSize {
            num_lines: val.num_lines() as u16,
            num_cols: val.num_columns() as u16,
            cell_width: val.cell_width() as u16,
            cell_height: val.line_height() as u16,
        }
    }
}

impl Dimensions for TerminalSize {
    /// Note: this is supposed to be for the back buffer's length,
    /// but we exclusively use it to resize the terminal, which does not
    /// use this method. We still have to implement it for the trait though,
    /// hence, this comment.
    fn total_lines(&self) -> usize {
        self.screen_lines()
    }

    fn screen_lines(&self) -> usize {
        self.num_lines()
    }

    fn columns(&self) -> usize {
        self.num_columns()
    }
}

#[derive(Error, Debug)]
pub struct TerminalError {
    pub directory: Option<PathBuf>,
    pub shell: Shell,
    pub source: std::io::Error,
}

impl TerminalError {
    pub fn fmt_directory(&self) -> String {
        self.directory
            .clone()
            .map(|path| {
                match path
                    .into_os_string()
                    .into_string()
                    .map_err(|os_str| format!("<non-utf8 path> {}", os_str.to_string_lossy()))
                {
                    Ok(s) => s,
                    Err(s) => s,
                }
            })
            .unwrap_or_else(|| {
                let default_dir =
                    dirs::home_dir().map(|buf| buf.into_os_string().to_string_lossy().to_string());
                match default_dir {
                    Some(dir) => format!("<none specified, using home directory> {}", dir),
                    None => "<none specified, could not find home directory>".to_string(),
                }
            })
    }

    pub fn shell_to_string(&self) -> String {
        match &self.shell {
            Shell::System => "<system shell>".to_string(),
            Shell::Program(p) => p.to_string(),
            Shell::WithArguments { program, args } => format!("{} {}", program, args.join(" ")),
        }
    }

    pub fn fmt_shell(&self) -> String {
        match &self.shell {
            Shell::System => "<system defined shell>".to_string(),
            Shell::Program(s) => s.to_string(),
            Shell::WithArguments { program, args } => format!("{} {}", program, args.join(" ")),
        }
    }
}

impl Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_string: String = self.fmt_directory();
        let shell = self.fmt_shell();

        write!(
            f,
            "Working directory: {} Shell command: `{}`, IOError: {}",
            dir_string, shell, self.source
        )
    }
}
