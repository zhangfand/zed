wit_bindgen::generate!("lsp-adapter");

struct JsonLspAdapter;

impl LspAdapter for JsonLspAdapter {
    fn run() -> i32 {
        42
    }
}

export_lsp_adapter!(JsonLspAdapter);
