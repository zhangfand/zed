use ai::function_calling::OpenAIFunction;
use serde::{Serialize, Serializer};
use serde_json::json;

pub struct RepositoryContextRetriever;

impl OpenAIFunction for RepositoryContextRetriever {
    fn name(&self) -> String {
        "retrieve_context_from_repository".to_string()
    }
    fn description(&self) -> String {
        "Retrieve relevant content from repository with natural language".to_string()
    }
    fn system_prompt(&self) -> String {
        "'retrieve_context_from_repository'
                If more information is needed from the repository, to complete the users prompt reliably, pass up to 3 queries describing pieces of code or text you would like additional context upon.
                Do not make these queries general about programming, include very specific lexical references to the pieces of code you need more information on.
                We are passing these into a semantic similarity retrieval engine, with all the information in the current codebase included.
                As such, these should be phrased as descriptions of code of interest as opposed to questions".to_string()
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
impl Serialize for RepositoryContextRetriever {
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
