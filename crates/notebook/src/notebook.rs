use collections::HashMap;
use serde::{
    de::{self, value::SeqAccessDeserializer, MapAccess, SeqAccess},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    cells: Vec<Cell>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cell_type")]
pub enum Cell {
    #[serde(rename = "markdown")]
    Markdown {
        #[serde(deserialize_with = "deserialize_text")]
        source: Vec<String>,
    },
    #[serde(rename = "code")]
    Code {
        #[serde(deserialize_with = "deserialize_text")]
        source: Vec<String>,
        execution_count: Option<usize>,
        outputs: Vec<Output>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "output_type")]
pub enum Output {
    #[serde(rename = "execute_result")]
    ExecuteResult {
        execution_count: Option<usize>,
        #[serde(deserialize_with = "deserialize_text_map")]
        data: HashMap<String, Vec<String>>,
    },
    #[serde(rename = "display_data")]
    DisplayData { data: HashMap<String, Vec<String>> },
    #[serde(rename = "stream")]
    Stream { name: StreamName, text: Vec<String> },
    #[serde(rename = "error")]
    Error {
        ename: String,
        evalue: String,
        traceback: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StreamName {
    #[serde(rename = "stdout")]
    Stdout,
    #[serde(rename = "stderr")]
    Stderr,
}

struct TextMapVisitor;

impl<'de> de::Visitor<'de> for TextMapVisitor {
    type Value = HashMap<String, Vec<String>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map from strings to strings or arrays of strings")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut result = HashMap::<String, Vec<String>>::default();
        while let Some((key, value)) = map.next_entry()? {
            let value = match value {
                serde_json::Value::String(s) => {
                    vec![s]
                }
                serde_json::Value::Array(a) => a
                    .into_iter()
                    .filter_map(|value| {
                        if let serde_json::Value::String(s) = value {
                            Some(s)
                        } else {
                            None
                        }
                    })
                    .collect(),
                _ => return Err(de::Error::custom("expected a string or array of strings")),
            };
            result.insert(key, value);
        }

        Ok(result)
    }
}

fn deserialize_text_map<'de, D>(deserializer: D) -> Result<HashMap<String, Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(TextMapVisitor)
}

struct TextVisitor;

impl<'de> de::Visitor<'de> for TextVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or an array of strings")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(vec![s.to_string()])
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        Deserialize::deserialize(SeqAccessDeserializer::new(seq))
    }
}

fn deserialize_text<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(TextVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_micrograd() {
        let s = std::fs::read("/Users/nathan/src/micrograd/demo.ipynb").unwrap();
        let notebook: Notebook = serde_json::from_slice(&s).unwrap();
        println!("{:?}", notebook);
    }
}
