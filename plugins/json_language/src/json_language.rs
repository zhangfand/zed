wit_bindgen::generate!("lsp-adapter");

#[allow(unused)]
macro_rules! log {
    ( $arg:expr ) => {
        $crate::zed_log($arg)
    };

    ( $( $arg:tt )* ) => {
        $crate::zed_log(&format!( $($arg)* ))
    };
}

const SERVER_PATH: &'static str =
    "/node_modules/vscode-json-languageserver/bin/vscode-json-languageserver";

fn server_binary_arguments(server_path: String) -> Vec<String> {
    vec![server_path, "--stdio".to_owned()]
}

struct NodeRuntime {
    id: ObjectId,
}

#[allow(unused)]
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
export_lsp_adapter!(JsonLspAdapter);

impl LspAdapter for JsonLspAdapter {
    fn name() -> String {
        "json-language-server-plugin".to_owned()
    }

    fn fetch_latest_server_version() -> Result<String, ()> {
        NodeRuntime::acquire().npm_package_latest_version("vscode-json-languageserver")
    }

    fn fetch_server_binary(
        version: String,
        container_dir: String,
    ) -> Result<LanguageServerBinary, ()> {
        let server_path = container_dir.clone() + SERVER_PATH;
        let node = NodeRuntime::acquire();

        if zed_fs_file_info(&server_path).is_none() {
            node.npm_install_packages(
                &container_dir,
                &[NpmPackage {
                    name: "vscode-json-languageserver".to_owned(),
                    version,
                }],
            )?;
        }

        Ok(LanguageServerBinary {
            path: node.binary_path()?,
            arguments: server_binary_arguments(server_path),
        })
    }

    fn cached_server_binary(container_dir: String) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &NodeRuntime::acquire())
    }

    fn can_be_reinstalled() -> bool {
        true
    }

    fn installation_test_binary(container_dir: String) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &NodeRuntime::acquire())
    }

    fn initialization_options() -> Option<String> {
        Some(r#"{ "provideFormatter": true }"#.to_owned())
    }

    fn workspace_configuration() -> Option<String> {
        // let action_names = cx.all_action_names().collect::<Vec<_>>();
        // let staff_mode = cx.default_global::<StaffMode>().0;
        // let language_names = &self.languages.language_names();
        // let settings_schema = cx.global::<SettingsStore>().json_schema(
        //     &SettingsJsonSchemaParams {
        //         language_names,
        //         staff_mode,
        //     },
        //     cx,
        // );
        // Some(
        //     future::ready(serde_json::json!({
        //         "json": {
        //             "format": {
        //                 "enable": true,
        //             },
        //             "schemas": [
        //                 {
        //                     "fileMatch": [
        //                         schema_file_match(&paths::SETTINGS),
        //                         &*paths::LOCAL_SETTINGS_RELATIVE_PATH,
        //                     ],
        //                     "schema": settings_schema,
        //                 },
        //                 {
        //                     "fileMatch": [schema_file_match(&paths::KEYMAP)],
        //                     "schema": KeymapFile::generate_json_schema(&action_names),
        //                 }
        //             ]
        //         }
        //     }))
        //     .boxed(),
        // )
        None
    }

    fn language_ids() -> Vec<(String, String)> {
        vec![("JSON".to_owned(), "jsonc".to_owned())]
    }
}

fn get_cached_server_binary(
    container_dir: String,
    node: &NodeRuntime,
) -> Option<LanguageServerBinary> {
    let mut last_version_dir = None;

    for entry in zed_fs_read_dir(&container_dir)? {
        let Ok(entry) = entry else {
            continue;
        };

        if let Some(info) = zed_fs_file_info(&entry) {
            if info.is_dir {
                last_version_dir = Some(entry);
            }
        }
    }

    let server_path = last_version_dir? + SERVER_PATH;
    if zed_fs_file_info(&server_path).is_some() {
        Some(LanguageServerBinary {
            path: node.binary_path().ok()?,
            arguments: server_binary_arguments(server_path),
        })
    } else {
        None
    }
}
