wit_bindgen::generate!("lsp-adapter");

macro_rules! log {
    ( $arg:expr ) => {
        $crate::zed_log($arg)
    };

    ( $( $arg:tt )* ) => {
        $crate::zed_log(&format!( $($arg)* ))
    };
}

struct NodeRuntime {
    id: NodeRuntimeId,
}

impl NodeRuntime {
    fn acquire() -> NodeRuntime {
        let id = zed_node_runtime_acquire();
        NodeRuntime { id }
    }

    fn binary_path(&self) -> Result<String, ()> {
        zed_node_runtime_binary_path(self.id)
    }
}

struct JsonLspAdapter;

impl LspAdapter for JsonLspAdapter {
    fn run() -> i32 {
        log!("Hey there, logging from within the guest");

        let node_runtime = NodeRuntime::acquire();
        log!("{:?}", node_runtime.binary_path());

        42
    }
}

export_lsp_adapter!(JsonLspAdapter);
