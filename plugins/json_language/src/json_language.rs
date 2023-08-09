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
    pub fn acquire() -> NodeRuntime {
        let id = zed_node_runtime_acquire();
        NodeRuntime { id }
    }

    pub fn binary_path(&self) -> Result<String, ()> {
        zed_node_runtime_binary_path(self.id)
    }

    pub fn npm_package_latest_version(&self, package: &str) -> Result<String, ()> {
        zed_node_runtime_npm_package_latest_version(self.id, package)
    }

    pub fn npm_install_packages(&self, dir: &str, packages: &[NpmPackage]) -> Result<(), ()> {
        zed_node_runtime_npm_install_packages(self.id, dir, packages)
    }

    pub fn npm_run_subcommand(
        &self,
        dir: Option<&str>,
        subcommand: &str,
        args: &[String],
    ) -> Result<(), ()> {
        zed_node_runtime_npm_run_subcommand(self.id, dir, subcommand, args)
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
