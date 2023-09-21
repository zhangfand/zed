use std::env;

use anyhow::anyhow;
use erased_serde::serialize_trait_object;
use futures::AsyncReadExt;
use isahc::{http::StatusCode, Request, RequestExt};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;
use util::ResultExt;

use crate::{RequestMessage, Role, Usage};

lazy_static! {
    #[derive(Debug)]
    static ref OPENAI_API_KEY: Option<String> = env::var("OPENAI_API_KEY").ok();
}

trait FunctionParameter: erased_serde::Serialize {}
serialize_trait_object!(FunctionParameter);

#[derive(Serialize)]
struct OpenAIFunctionCallingRequest {
    model: String,
    messages: Vec<RequestMessage>,
    functions: Vec<Box<dyn OpenAIFunction>>,
}

#[derive(Clone, Debug, Deserialize)]
struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

impl FunctionCall {
    fn arguments(&self) -> anyhow::Result<serde_json::Value> {
        serde_json::from_str(&self.arguments).map_err(|err| anyhow!(err))
    }
}

#[derive(Debug, Deserialize)]
struct FunctionCallingMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
    pub function_call: FunctionCall,
}

#[derive(Debug, Deserialize)]
struct FunctionCallingChoice {
    pub index: usize,
    pub message: FunctionCallingMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIFunctionCallingResponse {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<FunctionCallingChoice>,
    pub usage: Usage,
}

impl OpenAIFunctionCallingResponse {
    fn get_message_content(&self) -> Vec<String> {
        self.choices
            .iter()
            .filter_map(|choice| choice.message.content.clone())
            .collect()
    }

    fn get_function_details(&self) -> Vec<FunctionCall> {
        self.choices
            .iter()
            .map(|choice| choice.message.function_call.clone())
            .collect()
    }
}

trait OpenAIFunction: erased_serde::Serialize {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> serde_json::Value;
}
serialize_trait_object!(OpenAIFunction);

pub struct CodeContextRetriever {}
impl Serialize for CodeContextRetriever {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        json!({"name": self.name(),
               "description": self.description(),
               "parameters": self.parameters()})
        .serialize(serializer)
    }
}
impl OpenAIFunction for CodeContextRetriever {
    fn name(&self) -> String {
        "retrieve_code_context_from_repository".to_string()
    }
    fn description(&self) -> String {
        "Retrieve relevant code snippets from repository with natural language".to_string()
    }
    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "queries": {
                    "title": "queries",
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "required": ["queries"]
        })
    }
}

async fn send_function_calling_request_to_openai(
    api_key: String,
    model: String,
    messages: Vec<RequestMessage>,
    functions: Vec<Box<dyn OpenAIFunction>>,
) -> anyhow::Result<OpenAIFunctionCallingResponse> {
    let request = OpenAIFunctionCallingRequest {
        model,
        messages,
        functions,
    };

    let json_data = serde_json::to_string(&request)?;
    let mut response = Request::post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(json_data)
        .unwrap()
        .send_async()
        .await?;

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    match response.status() {
        StatusCode::OK => {
            let response_data: OpenAIFunctionCallingResponse = serde_json::from_str(&body)?;
            Ok(response_data)
        }
        _ => Err(anyhow!("open ai function calling failed: {:?}", body)),
    }
}
