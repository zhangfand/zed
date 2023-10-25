use crate::{ScaleType, ThemeColor};

#[derive(Debug, Default, Clone)]
pub enum AppearanceMode {
    #[default]
    Dark,
    Light,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub author: Option<String>,
    pub url: Option<String>,
    pub appearances: Vec<ThemeAppearance>,
    pub default_appearance: usize,
}

impl Theme {
    pub fn new(
        name: impl Into<String>,
        author: Option<impl Into<String>>,
        url: Option<impl Into<String>>,
        appearances: Vec<ThemeAppearance>,
        default_appearance: usize,
    ) -> Theme {
        Theme {
            name: name.into(),
            author: author.map(Into::into),
            url: url.map(Into::into),
            appearances,
            default_appearance,
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn author(&mut self, author: String) -> &mut Self {
        self.author = Some(author);
        self
    }

    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    pub fn appearances(&mut self, appearances: Vec<ThemeAppearance>) -> &mut Self {
        self.appearances = appearances;
        self
    }

    pub fn default_appearance(&mut self, default_appearance: usize) -> &mut Self {
        self.default_appearance = default_appearance;
        self
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Untitled Theme".to_string(),
            author: None,
            url: None,
            appearances: vec![],
            default_appearance: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeAppearance {
    pub id: usize,
    pub name: String,
    pub appearance: Vec<AppearanceMode>,
    pub scales: Vec<ScaleType>,
    pub color: ThemeColor,
}
