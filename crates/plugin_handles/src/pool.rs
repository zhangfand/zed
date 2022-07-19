use crate::{Resource, ResourceHandle};

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
