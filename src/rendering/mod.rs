use std::rc::Rc;
use tracing::info;
use wgpu::{Instance, InstanceDescriptor};

pub struct Renderer {
    target: Rc<dyn raw_window_handle::HasRawWindowHandle>,

    instance: Instance,
}

impl Renderer {
    pub fn new(target: Rc<impl raw_window_handle::HasRawWindowHandle + 'static>) -> Self {
        let instance = Instance::new(InstanceDescriptor::default());

        Self { target, instance }
    }
}
