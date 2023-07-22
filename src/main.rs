use application::{run, Application};
use rendering::Renderer;

pub mod application;
pub mod input;
pub mod rendering;
pub mod timing;

//
// TODO: - fix fullscreen
//       - fix resizeable
//

fn main() {
    let app = App {};
    run(app).unwrap();
}

struct App {}

impl Application for App {
    fn update(
        &mut self,
        globals: &mut application::Globals,
        input_manager: &input::InputManager,
        timing: &timing::Timing,
    ) {
        if input_manager.key_pressed(input::Key::Escape) {
            globals.should_exit = true
        }
    }

    fn render(&self, renderer: &Renderer) {}
}
