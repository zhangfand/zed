use ai::function_calling::OpenAIFunctionCallingProvider;
use ai::skills::RewritePrompt;
use gpui::AppContext;
use language::{BufferSnapshot, OffsetRangeExt, ToOffset};
use semantic_index::skills::RepositoryContextRetriever;
use std::cmp;
use std::ops::Range;
use std::sync::Arc;
use std::{fmt::Write, iter};

use crate::codegen::CodegenKind;

fn outline_for_prompt(
    buffer: &BufferSnapshot,
    range: Range<language::Anchor>,
    cx: &AppContext,
) -> Option<String> {
    let indent = buffer
        .language_indent_size_at(0, cx)
        .chars()
        .collect::<String>();
    let outline = buffer.outline(None)?;
    let range = range.to_offset(buffer);

    let mut text = String::new();
    let mut items = outline.items.into_iter().peekable();

    let mut intersected = false;
    let mut intersection_indent = 0;
    let mut extended_range = range.clone();

    while let Some(item) = items.next() {
        let item_range = item.range.to_offset(buffer);
        if item_range.end < range.start || item_range.start > range.end {
            text.extend(iter::repeat(indent.as_str()).take(item.depth));
            text.push_str(&item.text);
            text.push('\n');
        } else {
            intersected = true;
            let is_terminal = items
                .peek()
                .map_or(true, |next_item| next_item.depth <= item.depth);
            if is_terminal {
                if item_range.start <= extended_range.start {
                    extended_range.start = item_range.start;
                    intersection_indent = item.depth;
                }
                extended_range.end = cmp::max(extended_range.end, item_range.end);
            } else {
                let name_start = item_range.start + item.name_ranges.first().unwrap().start;
                let name_end = item_range.start + item.name_ranges.last().unwrap().end;

                if range.start > name_end {
                    text.extend(iter::repeat(indent.as_str()).take(item.depth));
                    text.push_str(&item.text);
                    text.push('\n');
                } else {
                    if name_start <= extended_range.start {
                        extended_range.start = item_range.start;
                        intersection_indent = item.depth;
                    }
                    extended_range.end = cmp::max(extended_range.end, name_end);
                }
            }
        }

        if intersected
            && items.peek().map_or(true, |next_item| {
                next_item.range.start.to_offset(buffer) > range.end
            })
        {
            intersected = false;
            text.extend(iter::repeat(indent.as_str()).take(intersection_indent));
            text.extend(buffer.text_for_range(extended_range.start..range.start));
            text.push_str("<|");
            text.extend(buffer.text_for_range(range.clone()));
            text.push_str("|>");
            text.extend(buffer.text_for_range(range.end..extended_range.end));
            text.push('\n');
        }
    }

    Some(text)
}

fn generate_codegen_planning_prompt(
    user_prompt: String,
    language_name: Arc<str>,
    buffer: &BufferSnapshot,
    range: Range<language::Anchor>,
    cx: &AppContext,
) -> String {
    // Language name should always be refered to in all lowercase
    let language_name = language_name.to_lowercase();

    let mut prompt = String::new();

    // General Preamble
    // It would be interesting to see, if there is any improvement
    // putting this in a "System" prompt
    writeln!(prompt, "You're an expert {language_name} engineer.\n").unwrap();

    // let outline = outline_for_prompt(buffer: &BufferSnapshot, range: Range<language::Anchor>, cx: &AppContext) -> Option<String>;
    let outline = outline_for_prompt(buffer, range.clone(), cx);
    if let Some(outline) = outline {
        writeln!(
            prompt,
            "You're currently working inside the Zed editor on a file with the following outline:\n```{language_name}\n{outline}\n```"
        )
        .unwrap();
    }

    // Assume for now that we are just generating
    if range.clone().start == range.end {
        writeln!(prompt, "In particular, the user's cursor is current on the '<||>' span in the above outline, with no text selected.").unwrap();
    } else {
        writeln!(prompt, "In particular, the user has selected a section of the text between the '<|' and '|>' spans.").unwrap();
    }

    writeln!(
        prompt,
        "The user has provided the following prompt: '{user_prompt}'\n"
    )
    .unwrap();
    writeln!(
        prompt,
        "It is your task to identify if any additional context is needed from the repository"
    )
    .unwrap();

    prompt
}

fn generate_content_prompt(
    user_prompt: String,
    snippets: Vec<String>,
    language_name: Arc<str>,
    buffer: &BufferSnapshot,
    range: Range<language::Anchor>,
    cx: &AppContext,
) -> String {
    // Language name should always be refered to in all lowercase
    let language_name = language_name.to_lowercase();

    let mut prompt = String::new();

    // General Preamble
    // It would be interesting to see, if there is any improvement
    // putting this in a "System" prompt
    writeln!(prompt, "You're an expert {language_name} engineer.\n").unwrap();

    if snippets.len() > 0 {
        writeln!(prompt, "You are currently working inside a large repository, here are a few additional code snippets which may provide useful context.").unwrap();
    }
    for snippet in &snippets {
        writeln!(prompt, "{snippet}").unwrap();
    }

    // let outline = outline_for_prompt(buffer: &BufferSnapshot, range: Range<language::Anchor>, cx: &AppContext) -> Option<String>;
    let outline = outline_for_prompt(buffer, range.clone(), cx);
    if let Some(outline) = outline {
        writeln!(
        prompt,
        "The file you are currently working on has the following outline:\n```{language_name}\n{outline}\n```"
    )
    .unwrap();
    }

    // Assume for now that we are just generating
    if range.clone().start == range.end {
        writeln!(prompt, "In particular, the user's cursor is current on the '<||>' span in the above outline, with no text selected.").unwrap();
    } else {
        writeln!(prompt, "In particular, the user has selected a section of the text between the '<|' and '|>' spans.").unwrap();
    }

    if range.clone().start == range.end {
        writeln!(
            prompt,
            "Modify the users code selected text based upon the users prompt: {user_prompt}"
        )
        .unwrap();
        writeln!(
            prompt,
            "You MUST reply with only the adjusted code, not the entire file."
        );
    } else {
        writeln!(
            prompt,
            "Assume the cursor is located where the `<|` marker is."
        )
        .unwrap();
        writeln!(
            prompt,
            "Text can't be replaced, so assume your answer will be inserted at the cursor."
        )
        .unwrap();
        writeln!(
            prompt,
            "Generate text based on the users prompt: {user_prompt}"
        )
        .unwrap();
    }

    writeln!(prompt, "Your answer MUST always be valid {language_name}").unwrap();
    writeln!(prompt, "Always wrap your response in a Markdown codeblock").unwrap();
    writeln!(prompt, "Never make remarks about the output.").unwrap();

    prompt
}

#[cfg(test)]
pub(crate) mod tests {

    use std::env;

    use super::*;
    use crate::{assistant_panel::tests::rust_lang, MessageId};
    use ai::{function_calling::OpenAIFunction, RequestMessage};
    use collections::HashMap;
    use gpui::AppContext;
    use indoc::indoc;
    use language::{language_settings, tree_sitter_rust, Buffer, Language, LanguageConfig, Point};
    use settings::SettingsStore;

    #[gpui::test]
    fn test_outline_for_prompt(cx: &mut AppContext) {
        cx.set_global(SettingsStore::test(cx));
        language_settings::init(cx);
        let text = indoc! {"
            struct X {
                a: usize,
                b: usize,
            }

            impl X {

                fn new() -> Self {
                    let a = 1;
                    let b = 2;
                    Self { a, b }
                }

                pub fn a(&self, param: bool) -> usize {
                    self.a
                }

                pub fn b(&self) -> usize {
                    self.b
                }
            }
        "};
        let buffer =
            cx.add_model(|cx| Buffer::new(0, 0, text).with_language(Arc::new(rust_lang()), cx));
        let snapshot = buffer.read(cx).snapshot();

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(1, 4))..snapshot.anchor_before(Point::new(1, 4)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    <||>a: usize
                    b
                impl X
                    fn new
                    fn a
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(8, 12))..snapshot.anchor_before(Point::new(8, 14)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X
                    fn new() -> Self {
                        let <|a |>= 1;
                        let b = 2;
                        Self { a, b }
                    }
                    fn a
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(6, 0))..snapshot.anchor_before(Point::new(6, 0)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X
                <||>
                    fn new
                    fn a
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(8, 12))..snapshot.anchor_before(Point::new(13, 9)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X
                    fn new() -> Self {
                        let <|a = 1;
                        let b = 2;
                        Self { a, b }
                    }

                    pub f|>n a(&self, param: bool) -> usize {
                        self.a
                    }
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(5, 6))..snapshot.anchor_before(Point::new(12, 0)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X<| {

                    fn new() -> Self {
                        let a = 1;
                        let b = 2;
                        Self { a, b }
                    }
                |>
                    fn a
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(18, 8))..snapshot.anchor_before(Point::new(18, 8)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X
                    fn new
                    fn a
                    pub fn b(&self) -> usize {
                        <||>self.b
                    }
            "})
        );
    }

    // WIP: This is here just to test the entrypoint for function calling.
    // #[gpui::test]
    // fn test_planning_prompt(cx: &mut AppContext) {
    //     let api_key: String = env::var("OPENAI_API_KEY").ok().unwrap();

    //     cx.set_global(SettingsStore::test(cx));
    //     language_settings::init(cx);
    //     let text = indoc! {"
    //         struct X {
    //             a: usize,
    //             b: usize,
    //         }

    //         impl X {

    //             fn new() -> Self {
    //                 let a = 1;
    //                 let b = 2;
    //                 Self { a, b }
    //             }

    //             pub fn a(&self, param: bool) -> usize {
    //                 self.a
    //             }

    //             pub fn b(&self) -> usize {
    //                 self.b
    //             }
    //         }
    //     "};
    //     let language = Arc::new(rust_lang());
    //     let buffer = cx.add_model(|cx| Buffer::new(0, 0, text).with_language(language.clone(), cx));
    //     let snapshot = buffer.read(cx).snapshot();
    //     let range =
    //         snapshot.anchor_before(Point::new(7, 12))..snapshot.anchor_before(Point::new(20, 0));

    //     // let user_prompt =
    //     //     "Please create a new function which adds together the result of the a and b functions"
    //     //         .to_string();

    //     let user_prompt =
    //         "Please create a new function which adds together the result of the a and b functions"
    //             .to_string();

    //     let prompt =
    //         generate_codegen_planning_prompt(user_prompt, language.name(), &snapshot, range, cx);

    //     println!("{}", &prompt);

    //     let messages = vec![RequestMessage {
    //         role: ai::Role::User,
    //         content: prompt,
    //     }];

    //     let functions: Vec<Box<dyn OpenAIFunction>> = vec![
    //         Box::new(RewritePrompt),
    //         Box::new(RepositoryContextRetriever),
    //     ];

    //     let function_map = functions
    //         .into_iter()
    //         .map(|function| (function.name(), function))
    //         .collect::<HashMap<String, Box<dyn OpenAIFunction>>>();

    //     let provider = OpenAIFunctionCallingProvider::new(api_key);
    //     let data = provider.complete("gpt-4-0613".to_string(), messages, functions);

    //     let data = smol::future::block_on(data).unwrap();

    //     // let function_name = &data.name;
    //     // if let Some(function) = function_map.get(function_name) {
    //     //     match function_name.as_str() {
    //     //         "rewrite_prompt" => {
    //     //             // let prompt = function.complete(data.arguments);
    //     //         }
    //     //         "retrieve_context_from_repository" => {}
    //     //     }
    //     // }

    //     panic!();
    // }
}
