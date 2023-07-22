use std::collections::HashSet;

use winit::event;

pub struct InputManager {
    pressed_keys: HashSet<Key>,
    just_pressed_keys: HashSet<Key>,
    mouse_position: (f32, f32),
    pressed_mouse_buttons: HashSet<MouseButton>,
    just_pressed_mouse_buttons: HashSet<MouseButton>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            just_pressed_keys: HashSet::new(),
            mouse_position: (0.0, 0.0),
            pressed_mouse_buttons: HashSet::new(),
            just_pressed_mouse_buttons: HashSet::new(),
        }
    }

    pub fn update(&mut self, event: event::WindowEvent) {
        match event {
            event::WindowEvent::KeyboardInput { input, .. } if input.virtual_keycode.is_some() => {
                if input.state == event::ElementState::Pressed {
                    self.pressed_keys
                        .insert(input.virtual_keycode.unwrap().into());
                    self.just_pressed_keys
                        .insert(input.virtual_keycode.unwrap().into());
                } else {
                    self.pressed_keys
                        .remove(&input.virtual_keycode.unwrap().into());
                }
            }
            event::WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x as f32, position.y as f32)
            }
            event::WindowEvent::MouseInput { state, button, .. } => {
                if state == event::ElementState::Pressed {
                    self.pressed_mouse_buttons.insert(button.into());
                    self.just_pressed_mouse_buttons.insert(button.into());
                } else {
                    self.pressed_mouse_buttons.remove(&button.into());
                }
            }
            _ => (),
        }
    }

    pub fn clear(&mut self) {
        self.just_pressed_keys.clear();
        self.just_pressed_mouse_buttons.clear()
    }

    pub fn pressed_keys(&self) -> &HashSet<Key> {
        &self.pressed_keys
    }

    pub fn key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn just_pressed_keys(&self) -> &HashSet<Key> {
        &self.just_pressed_keys
    }

    pub fn key_just_pressed(&self, key: Key) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    pub fn pressed_mouse_buttons(&self) -> &HashSet<MouseButton> {
        &self.pressed_mouse_buttons
    }

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    pub fn just_pressed_mouse_buttons(&self) -> &HashSet<MouseButton> {
        &self.just_pressed_mouse_buttons
    }

    pub fn mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_mouse_buttons.contains(&button)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Key {
    NotImplemented = 0,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Up,
    Down,
    Left,
    Right,
    Tab,
    Delete,
    RightShift,
    LeftShift,
    RightCtrl,
    LeftCtrl,
    RightMeta,
    LeftMeta,
    RightAlt,
    LeftAlt,
    Backspace,
    Enter,
    Space,
}

impl From<event::VirtualKeyCode> for Key {
    fn from(value: event::VirtualKeyCode) -> Self {
        match value {
            event::VirtualKeyCode::Key1 => Key::One,
            event::VirtualKeyCode::Key2 => Key::Two,
            event::VirtualKeyCode::Key3 => Key::Three,
            event::VirtualKeyCode::Key4 => Key::Four,
            event::VirtualKeyCode::Key5 => Key::Five,
            event::VirtualKeyCode::Key6 => Key::Six,
            event::VirtualKeyCode::Key7 => Key::Seven,
            event::VirtualKeyCode::Key8 => Key::Eight,
            event::VirtualKeyCode::Key9 => Key::Nine,
            event::VirtualKeyCode::Key0 => Key::Zero,
            event::VirtualKeyCode::A => Key::A,
            event::VirtualKeyCode::B => Key::B,
            event::VirtualKeyCode::C => Key::C,
            event::VirtualKeyCode::D => Key::D,
            event::VirtualKeyCode::E => Key::E,
            event::VirtualKeyCode::F => Key::F,
            event::VirtualKeyCode::G => Key::G,
            event::VirtualKeyCode::H => Key::H,
            event::VirtualKeyCode::I => Key::I,
            event::VirtualKeyCode::J => Key::J,
            event::VirtualKeyCode::K => Key::K,
            event::VirtualKeyCode::L => Key::L,
            event::VirtualKeyCode::M => Key::M,
            event::VirtualKeyCode::N => Key::N,
            event::VirtualKeyCode::O => Key::O,
            event::VirtualKeyCode::P => Key::P,
            event::VirtualKeyCode::Q => Key::Q,
            event::VirtualKeyCode::R => Key::R,
            event::VirtualKeyCode::S => Key::S,
            event::VirtualKeyCode::T => Key::T,
            event::VirtualKeyCode::U => Key::U,
            event::VirtualKeyCode::V => Key::V,
            event::VirtualKeyCode::W => Key::W,
            event::VirtualKeyCode::X => Key::X,
            event::VirtualKeyCode::Y => Key::Y,
            event::VirtualKeyCode::Z => Key::Z,
            event::VirtualKeyCode::Escape => Key::Escape,
            event::VirtualKeyCode::F1 => Key::F1,
            event::VirtualKeyCode::F2 => Key::F2,
            event::VirtualKeyCode::F3 => Key::F3,
            event::VirtualKeyCode::F4 => Key::F4,
            event::VirtualKeyCode::F5 => Key::F5,
            event::VirtualKeyCode::F6 => Key::F6,
            event::VirtualKeyCode::F7 => Key::F7,
            event::VirtualKeyCode::F8 => Key::F8,
            event::VirtualKeyCode::F9 => Key::F9,
            event::VirtualKeyCode::F10 => Key::F10,
            event::VirtualKeyCode::F11 => Key::F11,
            event::VirtualKeyCode::F12 => Key::F12,
            event::VirtualKeyCode::Delete => Key::Delete,
            event::VirtualKeyCode::Left => Key::Left,
            event::VirtualKeyCode::Up => Key::Up,
            event::VirtualKeyCode::Right => Key::Right,
            event::VirtualKeyCode::Down => Key::Down,
            event::VirtualKeyCode::Back => Key::Backspace,
            event::VirtualKeyCode::Return => Key::Enter,
            event::VirtualKeyCode::Space => Key::Space,
            event::VirtualKeyCode::LAlt => Key::LeftAlt,
            event::VirtualKeyCode::LControl => Key::LeftCtrl,
            event::VirtualKeyCode::LShift => Key::LeftShift,
            event::VirtualKeyCode::LWin => Key::LeftMeta,
            event::VirtualKeyCode::RAlt => Key::RightAlt,
            event::VirtualKeyCode::RControl => Key::RightCtrl,
            event::VirtualKeyCode::RShift => Key::RightShift,
            event::VirtualKeyCode::RWin => Key::RightMeta,
            event::VirtualKeyCode::Tab => Key::Tab,
            _ => Key::NotImplemented,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MouseButton {
    NotImplemented = 0,
    Left,
    Middle,
    Right,
}

impl From<event::MouseButton> for MouseButton {
    fn from(value: event::MouseButton) -> Self {
        match value {
            event::MouseButton::Left => MouseButton::Left,
            event::MouseButton::Right => MouseButton::Right,
            event::MouseButton::Middle => MouseButton::Middle,
            _ => MouseButton::NotImplemented,
        }
    }
}
