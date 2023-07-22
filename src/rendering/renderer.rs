use std::rc::Rc;

use super::Renderer;

pub struct Renderer3D {
    target: Option<Rc<dyn raw_window_handle::HasRawWindowHandle>>,
}

impl Renderer3D {
    pub fn new() -> Self {
        Self { target: None }
    }
}

impl Renderer for Renderer3D {
    fn set_target(&mut self, target: Rc<impl raw_window_handle::HasRawWindowHandle + 'static>) {
        self.target = Some(target)
    }
}
