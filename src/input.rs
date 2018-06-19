
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use super::controller::{ ButtonAction, Button };

const BUTTON_A      : Keycode = Keycode::A;
const BUTTON_B      : Keycode = Keycode::S;
const BUTTON_SELECT : Keycode = Keycode::Z;
const BUTTON_START  : Keycode = Keycode::X;
const BUTTON_UP     : Keycode = Keycode::Up;
const BUTTON_DOWN   : Keycode = Keycode::Down;
const BUTTON_LEFT   : Keycode = Keycode::Left;
const BUTTON_RIGHT  : Keycode = Keycode::Right;

fn get_key_mapping(key : Keycode) -> Option<Button> {
    match key {
        Keycode::A     => Some(Button::A),
        Keycode::S     => Some(Button::B),
        Keycode::Z     => Some(Button::Select),
        Keycode::X     => Some(Button::Start),
        Keycode::Up    => Some(Button::Up),
        Keycode::Down  => Some(Button::Down),
        Keycode::Left  => Some(Button::Left),
        Keycode::Right => Some(Button::Right),
        _              => None,
    }
}


#[derive(Debug)]
pub enum EmulatorEvent {
    ControllerEvent { action : ButtonAction, button : Button },
    Continue,
    Exit,
}

pub struct EmulatorInput {
    pump : EventPump,
}

impl EmulatorInput {
    pub fn events(&mut self) -> Vec<EmulatorEvent> {
        let event_receiver = |event| match event {
                Event::Quit {..} => EmulatorEvent::Exit,

                Event::KeyDown { keycode : Some(key), .. } =>
                    match get_key_mapping(key) {
                        Some(button) =>
                            EmulatorEvent::ControllerEvent {
                                action : ButtonAction::Pressed,
                                button : button,
                            },
                        None => EmulatorEvent::Continue,
                    },

                Event::KeyUp { keycode : Some(key), .. } =>
                    match get_key_mapping(key) {
                        Some(button) =>
                            EmulatorEvent::ControllerEvent {
                                action : ButtonAction::Released,
                                button : button,
                            },
                        None => EmulatorEvent::Continue,
                    },

                _ => EmulatorEvent::Continue,
        };

        self.pump.poll_iter().map(event_receiver).collect()
    }
    pub fn new(pump : EventPump) -> EmulatorInput {
        EmulatorInput {
            pump : pump,
        }
    }
}
