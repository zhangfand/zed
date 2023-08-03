wit_bindgen::generate!("lsp-adapter");

struct JsonLspAdapter;

impl LspAdapter for JsonLspAdapter {
    fn run() -> i32 {
        log("Hey there, logging from within the guest");
        42
    }
}

export_lsp_adapter!(JsonLspAdapter);
