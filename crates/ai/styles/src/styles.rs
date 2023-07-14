use theme2::Theme;

pub struct AssistantStyle {
    sidebar: SidebarStyle,
    header: CellStyle,
    body: CellStyle,
}

impl AssistantStyle {
    fn for_theme(theme: Theme) -> Self {
        Self {
            sidebar: SidebarStyle::for_theme(theme),
        }
    }
}

pub struct SidebarStyle {
    panel: CellStyle,
    section: CellStyle,
    section_header: CellStyle,
    section_list_item: CellStyle,
}

struct CellStyle;

struct ButtonStyle {
    content: CellStyle,
    icon: Option<CellStyle>,
}
