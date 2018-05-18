
pub enum ButtonAction {
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

    pub fn update(&mut self, action : ButtonAction, button : Button) {
        let bit_pos = match button {
            Button::A      => 7,
            Button::B      => 6,
            Button::Select => 5,
            Button::Start  => 4,
            Button::Up     => 3,
            Button::Down   => 2,
            Button::Left   => 1,
            Button::Right  => 0,
        };

        match action {
            ButtonAction::Pressed  => self.buttons |= 1 << bit_pos,
            ButtonAction::Released => self.buttons &= !(1 << bit_pos),
        };
    }

    pub fn status(&self) -> u8 {
        self.buttons
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ButtonAction::*;
    use super::Button::*;
    #[test]
    fn update() {
        let mut c = Controller::new();

        c.update(Pressed, A);
        assert_eq!(c.status(), 0b10000000);

        c.update(Released, A);
        assert_eq!(c.status(), 0b00000000);

        c.update(Pressed, A);
        c.update(Pressed, Select);
        c.update(Pressed, Up);
        assert_eq!(c.status(), 0b10101000);

        c.update(Released, A);
        assert_eq!(c.status(), 0b00101000);
    }
}
