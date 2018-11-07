
#[derive(Debug)]
pub enum ButtonAction {
    Pressed,
    Released,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Controller {
    current_buttons : u8,
    stored_buttons : u8,
    strobe : bool,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            current_buttons : 0,
            stored_buttons : 0,
            strobe : false,
        }
    }

    pub fn update(&mut self, action : ButtonAction, button : Button) {
        let bit_pos = match button {
            Button::A      => 0,
            Button::B      => 1,
            Button::Select => 2,
            Button::Start  => 3,
            Button::Up     => 4,
            Button::Down   => 5,
            Button::Left   => 6,
            Button::Right  => 7,
        };

        match action {
            ButtonAction::Pressed  => self.current_buttons |= 1 << bit_pos,
            ButtonAction::Released => self.current_buttons &= !(1 << bit_pos),
        };

        // update the internal shift register whenever the strobe is active
        if self.strobe {
            self.stored_buttons = self.current_buttons;
        }

        println!("{:?}", self);
    }

    pub fn read_next(&mut self) -> u8 {
        let ret = self.stored_buttons & 1;
        if !self.strobe {
            self.stored_buttons = self.stored_buttons >> 1;
        }
        ret
    }

    // buttons are continually loaded into shift register when strobe is set
    pub fn set_strobe(&mut self, val : u8) {
        self.strobe = (val & 1) == 1;
        self.stored_buttons = self.current_buttons;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ButtonAction::*;
    use super::Button::*;
    #[test]
    fn reading() {
        let mut c = Controller::new();

        c.update(Pressed, A);
        assert_eq!(c.read_next(), 0);
        c.set_strobe(1);

        c.update(Pressed, A);
        assert_eq!(c.read_next(), 1);
        assert_eq!(c.read_next(), 1);

    }
}
