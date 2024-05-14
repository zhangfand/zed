use serde::{Deserialize, Deserializer, Serialize};

use crate::jupyter::{DisplayData, ErrorReply, ExecuteResult};

// Custom deserialization for source field since it may be a Vec<String> or String
pub fn list_or_string_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the source field as a serde_json::Value
    let source_value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    // Check if the source is an array of strings
    if let Some(source_array) = source_value.as_array() {
        // Join the array of strings into a single string
        let source_string = source_array
            .iter()
            .map(|s| s.as_str().unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(source_string)
    } else if let Some(source_str) = source_value.as_str() {
        // If source is already a string, return it
        Ok(source_str.to_string())
    } else {
        Err(serde::de::Error::custom("Invalid source format"))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cell_type", rename_all = "snake_case")]
pub enum NotebookCell {
    Code(CodeCell),
    Markdown(MarkdownCell),
    Raw(RawCell),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeCell {
    #[serde(deserialize_with = "list_or_string_to_string")]
    pub source: String,
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "output_type", rename_all = "snake_case")]
pub enum Output {
    Stream(StreamContent),
    DisplayData(DisplayData),
    ExecuteResult(ExecuteResult),
    Error(ErrorReply),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamContent {
    pub name: String,
    #[serde(deserialize_with = "list_or_string_to_string")]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkdownCell {
    #[serde(deserialize_with = "list_or_string_to_string")]
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawCell {
    #[serde(deserialize_with = "list_or_string_to_string")]
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notebookv4 {
    pub cells: Vec<NotebookCell>,
    // todo: more fields
}
