use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

/// An enumeration representing various MIME (Multipurpose Internet Mail Extensions) types.
/// These types are used to indicate the nature of the data in a rich content message.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MimeType {
    /// Plain text, typically representing unformatted text. (e.g. Python's `_repr_` or `_repr_pretty_` methods).
    #[serde(rename = "text/plain")]
    Plain,
    /// HTML, (as displayed via Python's `_repr_html_` method).
    #[serde(rename = "text/html")]
    Html,
    /// LaTeX, (as displayed using Python's `_repr_latex_` method).
    #[serde(rename = "text/latex")]
    Latex,
    /// Raw JavaScript code.
    #[serde(rename = "application/javascript")]
    Javascript,
    /// Markdown text, (as displayed using Python's `_repr_markdown_` method).
    #[serde(rename = "text/markdown")]
    Markdown,

    /// SVG image text, (as displayed using Python's `_repr_svg_` method).
    #[serde(rename = "image/svg+xml")]
    Svg,

    /// PNG image data.
    #[serde(rename = "image/png")]
    Png,
    /// JPEG image data.
    #[serde(rename = "image/jpeg")]
    Jpeg,
    /// GIF image data.
    #[serde(rename = "image/gif")]
    Gif,

    /// Raw JSON Object
    #[serde(rename = "application/json")]
    Json,

    /// GeoJSON data, a format for encoding a variety of geographic data structures.
    #[serde(rename = "application/geo+json")]
    GeoJson,
    /// Data table in JSON format, requires both a `data` and `schema`.
    /// Example: `{data: [{'ghost': true, 'says': "boo"}], schema: {fields: [{name: 'ghost', type: 'boolean'}, {name: 'says', type: 'string'}]}}`.
    #[serde(rename = "application/vnd.dataresource+json")]
    DataTable,
    /// Plotly JSON Schema for for rendering graphs and charts.
    #[serde(rename = "application/vnd.plotly.v1+json")]
    Plotly,
    /// Jupyter/IPython widget view in JSON format.
    #[serde(rename = "application/vnd.jupyter.widget-view+json")]
    WidgetView,
    /// Jupyter/IPython widget state in JSON format.
    #[serde(rename = "application/vnd.jupyter.widget-state+json")]
    WidgetState,
    /// VegaLite data in JSON format for version 2 visualizations.
    #[serde(rename = "application/vnd.vegalite.v2+json")]
    VegaLiteV2,
    /// VegaLite data in JSON format for version 3 visualizations.
    #[serde(rename = "application/vnd.vegalite.v3+json")]
    VegaLiteV3,
    /// VegaLite data in JSON format for version 4 visualizations.
    #[serde(rename = "application/vnd.vegalite.v4+json")]
    VegaLiteV4,
    /// VegaLite data in JSON format for version 5 visualizations.
    #[serde(rename = "application/vnd.vegalite.v5+json")]
    VegaLiteV5,
    /// VegaLite data in JSON format for version 6 visualizations.
    #[serde(rename = "application/vnd.vegalite.v6+json")]
    VegaLiteV6,
    /// Vega data in JSON format for version 3 visualizations.
    #[serde(rename = "application/vnd.vega.v3+json")]
    VegaV3,
    /// Vega data in JSON format for version 4 visualizations.
    #[serde(rename = "application/vnd.vega.v4+json")]
    VegaV4,
    /// Vega data in JSON format for version 5 visualizations.
    #[serde(rename = "application/vnd.vega.v5+json")]
    VegaV5,

    /// Represents Virtual DOM (nteract/vdom) data in JSON format.
    #[serde(rename = "application/vdom.v1+json")]
    Vdom,

    // Catch all type for serde ease.
    // TODO: Implement a custom deserializer so this extra type isn't in resulting serializations.
    #[serde(other, rename = "application/vnd.runtimelib.unknown")]
    Other,
}

impl From<String> for MimeType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "text/plain" => MimeType::Plain,
            "text/html" => MimeType::Html,
            "text/latex" => MimeType::Latex,
            "application/javascript" => MimeType::Javascript,
            "text/markdown" => MimeType::Markdown,

            "application/json" => MimeType::Json,
            "application/vnd.dataresource+json" => MimeType::DataTable,
            "application/vnd.plotly.v1+json" => MimeType::Plotly,
            "image/svg+xml" => MimeType::Svg,
            "image/png" => MimeType::Png,
            "image/jpeg" => MimeType::Jpeg,
            "image/gif" => MimeType::Gif,

            "application/vnd.jupyter.widget-view+json" => MimeType::WidgetView,
            "application/vnd.jupyter.widget-state+json" => MimeType::WidgetState,

            "application/geo+json" => MimeType::GeoJson,

            "application/vnd.vegalite.v2+json" => MimeType::VegaLiteV2,
            "application/vnd.vegalite.v3+json" => MimeType::VegaLiteV3,
            "application/vnd.vegalite.v4+json" => MimeType::VegaLiteV4,
            "application/vnd.vegalite.v5+json" => MimeType::VegaLiteV5,
            "application/vnd.vegalite.v6+json" => MimeType::VegaLiteV6,

            "application/vnd.vega.v3+json" => MimeType::VegaV3,
            "application/vnd.vega.v4+json" => MimeType::VegaV4,
            "application/vnd.vega.v5+json" => MimeType::VegaV5,

            "application/vdom.v1+json" => MimeType::Vdom,

            _ => MimeType::Other,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct JupyterMessage {
    #[serde(skip_serializing)]
    zmq_identities: Vec<Bytes>,
    pub header: Header,
    pub parent_header: Option<Header>,
    pub metadata: Value,
    pub content: JupyterMessageContent,
    #[serde(skip_serializing)]
    pub buffers: Vec<Bytes>,
}

impl JupyterMessage {
    pub fn new(content: JupyterMessageContent) -> JupyterMessage {
        let header = Header {
            msg_id: Uuid::new_v4().to_string(),
            username: "runtimelib".to_string(),
            session: Uuid::new_v4().to_string(),
            date: Utc::now(),
            msg_type: content.message_type().to_owned(),
            version: "5.3".to_string(),
        };

        JupyterMessage {
            zmq_identities: Vec::new(),
            header,
            parent_header: None, // Empty for a new message
            metadata: json!({}),
            content,
            buffers: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub msg_id: String,
    pub username: String,
    pub session: String,
    pub date: DateTime<Utc>,
    pub msg_type: String,
    pub version: String,
}

#[allow(dead_code)] // This will be used once we have ZeroMQ sockets in Zed.
const DELIMITER: &[u8] = b"<IDS|MSG>";

/// A `MimeBundle` is a collection of data associated with different MIME types.
/// It allows for the representation of rich content that can be displayed in multiple formats.
/// These are found in the `data` field of a `DisplayData` and `ExecuteResult` messages/output types.
///
/// # Examples
///
/// ```rust
/// use runtimelib::media::{MimeBundle, MimeType};
///
/// let raw = r#"{
///    "text/plain": "FancyThing()",
///    "text/html": "<h1>Fancy!</h1>",
///    "application/json": {"fancy": true}
/// }"#;
///
/// let mime_bundle: MimeBundle = serde_json::from_str(raw).unwrap();
///
/// let richest = mime_bundle.richest(&[MimeType::Html, MimeType::Json, MimeType::Plain]);
///
/// if let Some((mime_type, content)) = richest {
///    println!("Richest MIME type: {:?}", mime_type);
///    println!("Content: {:?}", content);
/// }
/// ```
///
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MimeBundle {
    /// A map of MIME types to their corresponding data, represented as JSON `Value`.
    #[serde(flatten)]
    pub content: HashMap<MimeType, Value>,

    #[serde(flatten)]
    pub unknown_content: HashMap<String, Value>,
}

impl MimeBundle {
    /// Find the richest media based on a priority order of MIME types.
    /// The richest content is the first MIME type in the priority order that exists in the bundle.
    ///
    /// # Arguments
    ///
    /// * `priority_order` - A slice of `MimeType` representing the desired priority order.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of the selected `MimeType` and its corresponding content as a `Value`.
    /// Returns `None` if none of the MIME types in the priority order are present in the bundle.
    pub fn richest(&self, priority_order: &[MimeType]) -> Option<(MimeType, Value)> {
        for mime_type in priority_order {
            if let Some(content) = self.content.get(mime_type) {
                return Some((mime_type.clone(), content.clone()));
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum JupyterMessageContent {
    ExecuteRequest(ExecuteRequest),
    ExecuteReply(ExecuteReply),
    KernelInfoRequest(KernelInfoRequest),
    KernelInfoReply(KernelInfoReply),
    StreamContent(StreamContent),
    DisplayData(DisplayData),
    UpdateDisplayData(UpdateDisplayData),
    ExecuteInput(ExecuteInput),
    ExecuteResult(ExecuteResult),
    ErrorReply(ErrorReply),
    CommOpen(CommOpen),
    CommMsg(CommMsg),
    CommClose(CommClose),
    ShutdownRequest(ShutdownRequest),
    ShutdownReply(ShutdownReply),
    InputRequest(InputRequest),
    InputReply(InputReply),
    CompleteRequest(CompleteRequest),
    CompleteReply(CompleteReply),
    HistoryRequest(HistoryRequest),
    HistoryReply(HistoryReply),
    IsCompleteRequest(IsCompleteRequest),
    IsCompleteReply(IsCompleteReply),
    Status(Status),
}

impl JupyterMessageContent {
    pub fn message_type(&self) -> &str {
        match self {
            JupyterMessageContent::ExecuteRequest(_) => "execute_request",
            JupyterMessageContent::ExecuteReply(_) => "execute_reply",
            JupyterMessageContent::KernelInfoRequest(_) => "kernel_info_request",
            JupyterMessageContent::KernelInfoReply(_) => "kernel_info_reply",
            JupyterMessageContent::StreamContent(_) => "stream",
            JupyterMessageContent::DisplayData(_) => "display_data",
            JupyterMessageContent::UpdateDisplayData(_) => "update_display_data",
            JupyterMessageContent::ExecuteInput(_) => "execute_input",
            JupyterMessageContent::ExecuteResult(_) => "execute_result",
            JupyterMessageContent::ErrorReply(_) => "error",
            JupyterMessageContent::CommOpen(_) => "comm_open",
            JupyterMessageContent::CommMsg(_) => "comm_msg",
            JupyterMessageContent::CommClose(_) => "comm_close",
            JupyterMessageContent::ShutdownRequest(_) => "shutdown_request",
            JupyterMessageContent::ShutdownReply(_) => "shutdown_reply",
            JupyterMessageContent::InputRequest(_) => "input_request",
            JupyterMessageContent::InputReply(_) => "input_reply",
            JupyterMessageContent::CompleteRequest(_) => "complete_request",
            JupyterMessageContent::CompleteReply(_) => "complete_reply",
            JupyterMessageContent::HistoryRequest(_) => "history_request",
            JupyterMessageContent::HistoryReply(_) => "history_reply",
            JupyterMessageContent::IsCompleteRequest(_) => "is_complete_request",
            JupyterMessageContent::IsCompleteReply(_) => "is_complete_reply",
            JupyterMessageContent::Status(_) => "status",
        }
    }
}

impl JupyterMessageContent {
    pub fn from_type_and_content(msg_type: &str, content: Value) -> serde_json::Result<Self> {
        match msg_type {
            "execute_request" => Ok(JupyterMessageContent::ExecuteRequest(
                serde_json::from_value(content)?,
            )),
            "execute_input" => Ok(JupyterMessageContent::ExecuteInput(serde_json::from_value(
                content,
            )?)),
            "execute_reply" => Ok(JupyterMessageContent::ExecuteReply(serde_json::from_value(
                content,
            )?)),
            "kernel_info_request" => Ok(JupyterMessageContent::KernelInfoRequest(
                serde_json::from_value(content)?,
            )),
            "kernel_info_reply" => Ok(JupyterMessageContent::KernelInfoReply(
                serde_json::from_value(content)?,
            )),
            "stream" => Ok(JupyterMessageContent::StreamContent(
                serde_json::from_value(content)?,
            )),
            "display_data" => Ok(JupyterMessageContent::DisplayData(serde_json::from_value(
                content,
            )?)),
            "update_display_data" => Ok(JupyterMessageContent::UpdateDisplayData(
                serde_json::from_value(content)?,
            )),
            "execute_result" => Ok(JupyterMessageContent::ExecuteResult(
                serde_json::from_value(content)?,
            )),
            "error" => Ok(JupyterMessageContent::ErrorReply(serde_json::from_value(
                content,
            )?)),
            "comm_open" => Ok(JupyterMessageContent::CommOpen(serde_json::from_value(
                content,
            )?)),
            "comm_msg" => Ok(JupyterMessageContent::CommMsg(serde_json::from_value(
                content,
            )?)),
            "comm_close" => Ok(JupyterMessageContent::CommClose(serde_json::from_value(
                content,
            )?)),

            "shutdown_request" => Ok(JupyterMessageContent::ShutdownRequest(
                serde_json::from_value(content)?,
            )),
            "shutdown_reply" => Ok(JupyterMessageContent::ShutdownReply(
                serde_json::from_value(content)?,
            )),

            "input_request" => Ok(JupyterMessageContent::InputRequest(serde_json::from_value(
                content,
            )?)),

            "input_reply" => Ok(JupyterMessageContent::InputReply(serde_json::from_value(
                content,
            )?)),

            "complete_request" => Ok(JupyterMessageContent::CompleteRequest(
                serde_json::from_value(content)?,
            )),

            "complete_reply" => Ok(JupyterMessageContent::CompleteReply(
                serde_json::from_value(content)?,
            )),

            "history_request" => Ok(JupyterMessageContent::HistoryRequest(
                serde_json::from_value(content)?,
            )),

            "history_reply" => Ok(JupyterMessageContent::HistoryReply(serde_json::from_value(
                content,
            )?)),

            "is_complete_request" => Ok(JupyterMessageContent::IsCompleteRequest(
                serde_json::from_value(content)?,
            )),

            "is_complete_reply" => Ok(JupyterMessageContent::IsCompleteReply(
                serde_json::from_value(content)?,
            )),

            "status" => Ok(JupyterMessageContent::Status(serde_json::from_value(
                content,
            )?)),

            _ => Err(serde_json::Error::custom(format!(
                "Unsupported message type: {}",
                msg_type
            ))),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecuteRequest {
    pub code: String,
    pub silent: bool,
    pub store_history: bool,
    pub user_expressions: HashMap<String, String>,
    pub allow_stdin: bool,
}

impl From<ExecuteRequest> for JupyterMessage {
    fn from(req: ExecuteRequest) -> Self {
        JupyterMessage::new(JupyterMessageContent::ExecuteRequest(req))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecuteReply {
    pub status: String,
    pub execution_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KernelInfoRequest {}

impl From<KernelInfoRequest> for JupyterMessage {
    fn from(req: KernelInfoRequest) -> Self {
        JupyterMessage::new(JupyterMessageContent::KernelInfoRequest(req))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KernelInfoReply {
    pub protocol_version: String,
    pub implementation: String,
    pub implementation_version: String,
    pub language_info: LanguageInfo,
    pub banner: String,
    pub help_links: Vec<HelpLink>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageInfo {
    pub name: String,
    pub version: String,
    pub mimetype: String,
    pub file_extension: String,
    pub pygments_lexer: String,
    pub codemirror_mode: Value,
    pub nbconvert_exporter: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HelpLink {
    pub text: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamContent {
    pub name: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayData {
    pub data: MimeBundle,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDisplayData {
    pub data: MimeBundle,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecuteInput {
    pub code: String,
    pub execution_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecuteResult {
    pub execution_count: i64,
    pub data: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorReply {
    pub ename: String,
    pub evalue: String,
    pub traceback: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommOpen {
    pub comm_id: String,
    pub target_name: String,
    pub data: HashMap<String, String>,
    pub target_module: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommMsg {
    pub comm_id: String,
    pub data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommClose {
    pub comm_id: String,
    pub data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShutdownRequest {
    pub restart: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShutdownReply {
    pub restart: bool,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputRequest {
    pub prompt: String,
    pub password: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputReply {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompleteRequest {
    pub code: String,
    pub cursor_pos: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompleteReply {
    pub matches: Vec<String>,
    pub cursor_start: i64,
    pub cursor_end: i64,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsCompleteReply {
    pub status: String,
    pub indent: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryRequest {
    pub output: bool,
    pub raw: bool,
    pub hist_access_type: String,
    pub session: i64,
    pub start: i64,
    pub stop: i64,
    pub n: i64,
    pub pattern: String,
    pub unique: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryReply {
    pub history: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsCompleteRequest {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub execution_state: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute_request_serialize() {
        let request = ExecuteRequest {
            code: "print('Hello, World!')".to_string(),
            silent: false,
            store_history: true,
            user_expressions: HashMap::new(),
            allow_stdin: false,
        };
        let request_value = serde_json::to_value(&request).unwrap();

        let expected_request_value = serde_json::json!({
            "code": "print('Hello, World!')",
            "silent": false,
            "store_history": true,
            "user_expressions": {},
            "allow_stdin": false
        });

        assert_eq!(request_value, expected_request_value);
    }

    #[test]
    fn richest_middle() {
        let raw = r#"{
            "text/plain": "Hello, world!",
            "text/html": "<h1>Hello, world!</h1>",
            "application/json": {
                "name": "John Doe",
                "age": 30
            },
            "application/vnd.dataresource+json": {
                "data": [
                    {"name": "Alice", "age": 25},
                    {"name": "Bob", "age": 35}
                ],
                "schema": {
                    "fields": [
                        {"name": "name", "type": "string"},
                        {"name": "age", "type": "integer"}
                    ]
                }
            },
            "application/octet-stream": "Binary data"
        }"#;

        let bundle: MimeBundle = serde_json::from_str(raw).unwrap();

        let richest = bundle.richest(&[MimeType::Html]);

        let (mime_type, content) = richest.unwrap();

        assert_eq!(mime_type, MimeType::Html);
        assert_eq!(content, "<h1>Hello, world!</h1>");
    }

    #[test]
    fn find_table() {
        let raw = r#"{
            "text/plain": "Hello, world!",
            "text/html": "<h1>Hello, world!</h1>",
            "application/json": {
                "name": "John Doe",
                "age": 30
            },
            "application/vnd.dataresource+json": {
                "data": [
                    {"name": "Alice", "age": 25},
                    {"name": "Bob", "age": 35}
                ],
                "schema": {
                    "fields": [
                        {"name": "name", "type": "string"},
                        {"name": "age", "type": "integer"}
                    ]
                }
            },
            "application/octet-stream": "Binary data"
        }"#;

        let bundle: MimeBundle = serde_json::from_str(raw).unwrap();

        let richest = bundle.richest(&[MimeType::DataTable, MimeType::Json, MimeType::Html]);

        let (mime_type, content) = richest.unwrap();

        assert_eq!(mime_type, MimeType::DataTable);
        assert_eq!(
            content,
            serde_json::json!({
                "data": [
                    {"name": "Alice", "age": 25},
                    {"name": "Bob", "age": 35}
                ],
                "schema": {
                    "fields": [
                        {"name": "name", "type": "string"},
                        {"name": "age", "type": "integer"}
                    ]
                }
            })
        );
    }

    #[test]
    fn find_nothing_and_be_happy() {
        let raw = r#"{
            "application/fancy": "Too ✨ Fancy ✨ for you!"
        }"#;

        let bundle: MimeBundle = serde_json::from_str(raw).unwrap();

        let richest = bundle.richest(&[
            MimeType::DataTable,
            MimeType::Json,
            MimeType::Html,
            MimeType::Svg,
            MimeType::Plain,
        ]);

        let binary_data = bundle.unknown_content.get("application/fancy");
        assert_eq!(
            binary_data,
            Some(&serde_json::json!("Too ✨ Fancy ✨ for you!"))
        );

        assert_eq!(richest, None);
    }

    #[test]
    fn from_string() {
        let mime_type: MimeType = "text/plain".to_string().into();
        assert_eq!(mime_type, MimeType::Plain);
    }

    #[test]
    fn edge_case() {
        let raw = r#"{
            "text/plain": "Hello, world!",
            "text/html": "<h1>Hello, world!</h1>",
            "application/json": {
                "name": "John Doe",
                "age": 30
            },
            "application/vnd.dataresource+json": {
                "data": [
                    {"name": "Alice", "age": 25},
                    {"name": "Bob", "age": 35}
                ],
                "schema": {
                    "fields": [
                        {"name": "name", "type": "string"},
                        {"name": "age", "type": "integer"}
                    ]
                }
            },
            "application/octet-stream": "Binary data"
        }"#;

        let bundle: MimeBundle = serde_json::from_str(raw).unwrap();

        let richest = bundle.richest(&[]);
        assert_eq!(richest, None);
    }

    #[test]
    fn direct_access() {
        let raw = r#"{
            "text/plain": "🦀 Hello from Rust! 🦀",
            "text/html": "<h1>Hello, world!</h1>",
            "application/json": {
                "name": "John Doe",
                "age": 30
            },
            "application/vnd.dataresource+json": {
                "data": [
                    {"name": "Alice", "age": 25},
                    {"name": "Bob", "age": 35}
                ],
                "schema": {
                    "fields": [
                        {"name": "name", "type": "string"},
                        {"name": "age", "type": "integer"}
                    ]
                }
            },
            "application/octet-stream": "Binary data"
        }"#;

        let bundle: MimeBundle = serde_json::from_str(raw).unwrap();

        assert_eq!(
            bundle.content.get(&MimeType::Html).unwrap(),
            &serde_json::json!("<h1>Hello, world!</h1>")
        );

        assert_eq!(
            bundle
                .content
                .get(&MimeType::from("text/plain".to_string()))
                .unwrap(),
            "🦀 Hello from Rust! 🦀"
        )
    }
}
