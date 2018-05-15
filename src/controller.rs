
pub enum ButtonStatus {
    Pressed,
    Released,
}

pub enum Button {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

pub struct Controller {
    buttons : u8,
}

impl Controller {
    pub fn new() -> Controller {
        Controller { buttons : 0 }
    }

    pub fn update(&mut self, status : ButtonStatus, button : Button) {
        let bit_pos = match button {
            Button::A => 7,
            Button::B => 6,
            Button::Select => 5,
            Button::Start => 4,
            Button::Up => 3,
            Button::Down => 2,
            Button::Left => 1,
            Button::Right => 0,
        };

        match status {
            ButtonStatus::Pressed  => self.buttons |= 1 << bit_pos,
            ButtonStatus::Released => self.buttons &= !(1 << bit_pos),
        };
    }
}
