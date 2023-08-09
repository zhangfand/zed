use std::any::Any;
use std::path::Path;
use std::sync::Arc;

use anyhow::anyhow;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

use collections::HashMap;
use node_runtime::NodeRuntime;
use util::http::HttpClient;

bindgen!();

struct ObjectStore {
    next_id: u64,
    objects: HashMap<Id, Box<dyn Any>>,
}

impl ObjectStore {
    fn new() -> ObjectStore {
        ObjectStore {
            next_id: 0,
            objects: HashMap::default(),
        }
    }

    fn register<T: Any>(&mut self, object: T) -> Id {
        let id = util::post_inc(&mut self.next_id);
        self.objects.insert(id, Box::new(object));
        id
    }

    fn get<T: Any>(&mut self, id: Id) -> wasmtime::Result<&mut T> {
        let Some(object) = self.objects.get_mut(&id) else {
            return Err(anyhow!("Host object id {id} does not exist"));
        };

        match object.downcast_mut::<T>() {
            Some(object) => return Ok(object),

            None => {
                let expected = std::any::type_name::<T>();
                return Err(anyhow!(
                    "Host object id {id} type mismatch, expected {expected:?}"
                ));
            }
        }
    }
}

struct Host {
    object_store: ObjectStore,
    node_runtime_id: NodeRuntimeId,
}

impl Host {
    fn new(http: &Arc<dyn HttpClient>) -> Host {
        let mut object_store = ObjectStore::new();
        let node_runtime_id = object_store.register(NodeRuntime::instance(http));

        Host {
            object_store,
            node_runtime_id,
        }
    }
}

impl LspAdapterImports for Host {
    fn zed_log(&mut self, text: String) -> wasmtime::Result<()> {
        println!("{}", text);
        Ok(())
    }

    fn zed_node_runtime_acquire(&mut self) -> wasmtime::Result<NodeRuntimeId> {
        Ok(self.node_runtime_id)
    }

    fn zed_node_runtime_binary_path(
        &mut self,
        id: NodeRuntimeId,
    ) -> wasmtime::Result<Result<String, ()>> {
        let runtime = self.object_store.get::<Arc<NodeRuntime>>(id)?;
        Ok(smol::block_on(async {
            let path = runtime.binary_path().await.map_err(|_| ())?;
            path.to_str().map(|str| str.to_owned()).ok_or(())
        }))
    }

    fn zed_node_runtime_npm_package_latest_version(
        &mut self,
        id: NodeRuntimeId,
        package: String,
    ) -> wasmtime::Result<Result<String, ()>> {
        let runtime = self.object_store.get::<Arc<NodeRuntime>>(id)?;
        Ok(smol::block_on(async {
            runtime
                .npm_package_latest_version(&package)
                .await
                .map_err(|_| ())
        }))
    }

    fn zed_node_runtime_npm_install_packages(
        &mut self,
        id: NodeRuntimeId,
        dir: String,
        packages: Vec<NpmPackage>,
    ) -> wasmtime::Result<Result<(), ()>> {
        let runtime = self.object_store.get::<Arc<NodeRuntime>>(id)?;
        let packages = packages
            .iter()
            .map(|p| (p.name.as_str(), p.version.as_str()));

        Ok(smol::block_on(async {
            runtime
                .npm_install_packages(Path::new(&dir), packages)
                .await
                .map_err(|_| ())
        }))
    }

    fn zed_node_runtime_npm_run_subcommand(
        &mut self,
        id: NodeRuntimeId,
        dir: Option<String>,
        subcommand: String,
        args: Vec<String>,
    ) -> wasmtime::Result<Result<(), ()>> {
        let runtime = self.object_store.get::<Arc<NodeRuntime>>(id)?;
        let dir = dir.as_ref().map(|d| Path::new(d));
        let args: Vec<_> = args.iter().map(|a| a.as_str()).collect();

        Ok(smol::block_on(async {
            runtime
                .npm_run_subcommand(dir, &subcommand, &args)
                .await
                .map_err(|_| ())?;
            Ok(())
        }))
    }
}

pub fn function(http: &Arc<dyn HttpClient>) -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let bytes = include_bytes!("../../../plugins/bin/json_language.wasm");
    let component = Component::new(&engine, bytes)?;

    let mut linker = Linker::new(&engine);
    LspAdapter::add_to_linker(&mut linker, |host: &mut Host| host)?;

    let mut store = Store::new(&engine, Host::new(http));
    let (bindings, _) = LspAdapter::instantiate(&mut store, &component, &linker)?;

    let answer = bindings.call_run(&mut store)?;
    println!("Life, universe, etc: {}", answer);

    Ok(())
}
