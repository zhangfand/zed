use std::any::{Any, TypeId};

pub trait Resource: Any + Send {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;
    fn clone_resource(&self) -> Box<dyn Resource>;
}

impl<T: Any + Clone + Send> Resource for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
    fn clone_resource(&self) -> Box<dyn Resource> {
        Box::new(self.clone()) as Box<dyn Resource>
    }
}

impl dyn Resource {
    pub fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }
}
