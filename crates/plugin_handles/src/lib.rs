use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod resource;
pub use resource::*;

mod pool;
pub use pool::*;

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
