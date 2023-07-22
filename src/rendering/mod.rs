use std::rc::Rc;

pub mod renderer;

pub trait Renderer {
    fn set_target(&mut self, target: Rc<impl raw_window_handle::HasRawWindowHandle + 'static>);
}
