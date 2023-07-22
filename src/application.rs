use std::rc::Rc;

use winit::{
    dpi,
    event::{Event, WindowEvent},
    event_loop, monitor, window,
};

use crate::{input::InputManager, rendering::Renderer, timing::Timing};

#[derive(Debug)]
pub struct ApplicationDescriptor {
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
    pub resizeable: bool,
    pub title: String,
    pub with_logging: bool,
}

impl Default for ApplicationDescriptor {
    fn default() -> Self {
        ApplicationDescriptor {
            window_width: 800,
            window_height: 600,
            fullscreen: false,
            resizeable: false,
            title: "Application".to_string(),
            with_logging: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Globals {
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
    pub resizeable: bool,
    pub title: String,
    pub should_exit: bool,
}

impl Globals {
    pub(super) fn handle(
        &self,
        last: &Globals,
        window: Rc<window::Window>,
        monitor: &monitor::MonitorHandle,
    ) -> anyhow::Result<()> {
        if self.window_width != last.window_width {
            let mut new_size = window.inner_size();
            new_size.width = self.window_width;
            window.set_inner_size(new_size)
        }
        if self.window_height != last.window_height {
            let mut new_size = window.inner_size();
            new_size.height = self.window_height;
            window.set_inner_size(new_size)
        }
        if self.fullscreen != last.fullscreen {
            if self.fullscreen {
                window.set_fullscreen(Some(window::Fullscreen::Borderless(Some(monitor.clone()))))
            } else {
                window.set_fullscreen(None);
                window.set_inner_size(dpi::PhysicalSize::new(
                    self.window_width,
                    self.window_height,
                ));
            }
        }
        window.set_resizable(self.resizeable);
        if self.title != last.title {
            window.set_title(&self.title);
        }
        Ok(())
    }
}

impl From<ApplicationDescriptor> for Globals {
    fn from(value: ApplicationDescriptor) -> Self {
        Self {
            window_width: value.window_width,
            window_height: value.window_height,
            fullscreen: value.fullscreen,
            resizeable: value.resizeable,
            title: value.title,
            should_exit: false,
        }
    }
}

pub trait Application {
    fn descriptor(&self) -> ApplicationDescriptor {
        ApplicationDescriptor::default()
    }

    fn update(&mut self, globals: &mut Globals, input_manager: &InputManager, timing: &Timing);

    fn render(&self, renderer: &Renderer);
}

pub fn run<A: Application + 'static>(mut app: A) -> anyhow::Result<()> {
    let descriptor = app.descriptor();
    if descriptor.with_logging {
        let filter = tracing_subscriber::filter::EnvFilter::new("application_base=info,wgpu=warn");
        tracing_subscriber::fmt::fmt()
            .with_env_filter(filter)
            .init();
    }

    let event_loop = event_loop::EventLoop::new();
    let monitor = event_loop
        .primary_monitor()
        .ok_or(anyhow::anyhow!("no primary monitor"))?;
    let mut window_builder = window::WindowBuilder::default()
        .with_title(descriptor.title)
        .with_resizable(descriptor.resizeable)
        .with_inner_size(dpi::PhysicalSize::new(
            descriptor.window_width,
            descriptor.window_height,
        ));
    if descriptor.fullscreen {
        window_builder = window_builder
            .with_fullscreen(Some(window::Fullscreen::Borderless(Some(monitor.clone()))));
    }
    let window = window_builder.build(&event_loop)?;
    let window = Rc::new(window);

    let mut renderer = Renderer::new(window.clone());

    let mut globals: Globals = app.descriptor().into();
    let mut last_globals = globals.clone();

    let mut input_manager = InputManager::new();
    let mut timing = Timing::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::Resized(new_size) => {
                globals.window_width = new_size.width;
                globals.window_height = new_size.height;
            }
            _ => input_manager.update(event),
        },
        Event::MainEventsCleared => {
            timing.update();

            app.update(&mut globals, &input_manager, &timing);

            if let Err(err) = globals.handle(&last_globals, window.clone(), &monitor) {
                eprintln!("Exiting because of error: {}", err);
                control_flow.set_exit()
            }
            if globals.should_exit {
                control_flow.set_exit()
            }
            // maybe not clone every frame
            last_globals = globals.clone();
            input_manager.clear();
            window.request_redraw();
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => app.render(&renderer),
        _ => (),
    });
}
