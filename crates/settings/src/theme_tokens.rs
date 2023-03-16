use gpui::{
    fonts::{properties_from_json, underline_from_json, TextStyle},
    AppContext,
};
use theme::ui::TokenText;
use util::ResultExt;

pub trait TokenTextIntoTextStyleExt {
    fn into_text_style(self, cx: &AppContext) -> TextStyle;
}

impl TokenTextIntoTextStyleExt for TokenText {
    fn into_text_style(self, cx: &AppContext) -> TextStyle {
        let properties = properties_from_json(self.weight, self.italic);
        let underline = underline_from_json(self.underline);

        let name = match self.text_type {
            theme::ui::TextType::Ui => "Arial",
            theme::ui::TextType::Editor => "Zed Mono",
        };

        // cx.settings<UiFontDetails>().font_size;
        // cx.settings<UiFontDetails>().font_family;

        let font_size = self.font_size + 6.;

        match TextStyle::new(
            name,
            font_size,
            properties,
            underline,
            self.color,
            &cx.font_cache(),
        )
        .log_err()
        {
            Some(text_style) => text_style,
            None => TextStyle::new(
                "Zed Sans",
                font_size,
                properties,
                underline,
                self.color,
                &cx.font_cache(),
            )
            .unwrap(),
        }
    }
}
