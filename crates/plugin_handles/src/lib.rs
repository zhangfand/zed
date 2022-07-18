use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod resource;
pub use resource::*;

pub struct ResourcePool {
    resources: Vec<Box<dyn Resource>>,
}

impl ResourcePool {
    pub fn new() -> Self {
        ResourcePool {
            resources: Vec::new(),
        }
    }

    // pub fn register_resource<R: Resource, H: ResourceHandle>(&mut self) {
    //     let resource_id = TypeId::of::<R>();
    //     let handle_id = TypeId::of::<H>();
    //     self.registered.insert(resource_id, handle_id);
    // }

    pub fn clear(&mut self) {
        self.resources.clear();
    }

    pub fn add<T: Resource + Clone, H: ResourceHandle>(&mut self, resource: T) -> H {
        let index: u32 = self.resources.len().try_into().unwrap();
        self.resources.push(Box::new(resource));
        index.into()
    }

    fn get_resource<H: ResourceHandle>(&mut self, handle: H) -> Option<Box<dyn Resource>> {
        let index: u32 = handle.into();
        let resource = self.resources.get(index as usize)?.as_ref();
        Some(resource.clone_resource())
    }

    pub fn get<T: Resource + Clone, H: ResourceHandle>(&mut self, handle: H) -> Option<T> {
        let resource = self.get_resource(handle)?;
        let resource: Box<T> = resource.as_boxed_any().downcast().ok()?;
        Some(*resource)
    }
}

pub trait ResourceHandle: Serialize + DeserializeOwned + From<u32> + Into<u32> + 'static {}

macro_rules! resource_handle {
    ($t:ident) => {
        #[derive(Deserialize, Serialize)]
        pub struct $t(u32);

        impl Into<u32> for $t {
            fn into(self) -> u32 {
                self.0
            }
        }

        impl From<u32> for $t {
            fn from(index: u32) -> Self {
                Self(index)
            }
        }

        impl ResourceHandle for $t {}
    };
}

resource_handle!(RopeHandle);
resource_handle!(LanguageHandle);

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn test_resource_pool() {
        // Define a resource type.
        // Note that it is not Clone.
        pub struct Values {
            values: Vec<u8>,
        }
        resource_handle!(ValuesHandle);

        // Instantiate the resource type.
        // Wrap it in an Arc<Mutex<...>> to make it Clone.
        let my_values = Arc::new(Mutex::new(Values {
            values: vec![1, 2, 3],
        }));

        // Create a new ResourcePool.
        let mut resource_pool = ResourcePool::new();
        let handle: ValuesHandle = resource_pool.add(my_values.clone());

        // Emulate a callback.
        // Modify the resource in the resource pool.
        {
            let my_values_new: Arc<Mutex<Values>> = resource_pool.get(handle).unwrap();
            my_values_new.lock().unwrap().values.push(4);
        }

        // Clear the resource pool, releasing all resources.
        resource_pool.clear();

        // The resource should've been modified
        assert_eq!(my_values.lock().unwrap().values, vec![1, 2, 3, 4]);
    }
}
