use std::error::Error;

use rppal::gpio::{ Gpio, Level, InputPin};

use std::sync::mpsc::Sender;
use std::thread;

use std::time::{Duration};
use std::thread::JoinHandle;

pub fn run_button_pad(mut pad: ButtonPad) -> JoinHandle<()>{
    thread::spawn(move || {
        loop {
            let hid_events = pad.detect_changes();
            pad.button_sender.send(hid_events).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    })
}

pub struct ButtonInitializer {
    pub pin: u8,
    pub code: u8,
    pub key: &'static str
}

struct Button {
   pin: InputPin,
   state: Level,
   possible_state:Level,
   code: u8,
   repeat: u8
}

impl Button {
    pub fn new(pin: InputPin, code: u8 ) -> Button {
        let state = pin.read();
        let possible_state = pin.read();    
        let button: Button = Button {
            pin: pin,
            state: state,
            possible_state: possible_state,
            code: code,
            repeat: 0
        };
        button
    }
}
#[derive(Debug)]
pub struct HIDEvent {
    pub io_state: IOState,
    pub code: u8
}
#[derive(Debug)]
pub enum IOState {
    Pressed,
    Released,
    Repeated
}
pub struct ButtonPad {
    buttons: Vec<Button>,
    pub button_sender: Sender<Vec<HIDEvent>>
}

impl ButtonPad {
  pub fn new( pins: &Vec<ButtonInitializer>, button_sender: Sender<Vec<HIDEvent>>) -> Result<ButtonPad, Box<dyn Error>> {
      let mut buttons : Vec<Button> = Vec::with_capacity(pins.len());

      let gpio = Gpio::new()?;
      for pin in pins {
        let button = Button::new(gpio.get(pin.pin)?.into_input(), pin.code);
        buttons.push(button);
      }
      let pad: ButtonPad = ButtonPad {
        buttons: buttons,
        button_sender
      };
     Ok(pad)
  }

  pub fn detect_changes(&mut self) -> Vec<HIDEvent> {
      let mut hid_events: Vec<HIDEvent> = Vec::with_capacity(self.buttons.len());

      for mut button in &mut self.buttons {
        let option_io_state : Option<IOState> =  ButtonPad::detect_button_changes(&mut button);
        match option_io_state {
            Some(io_state) => {
                hid_events.push(
                    HIDEvent{
                        io_state: io_state,
                        code: button.code
                    }  
                );
            },
            None => ()
        }          
      }
      self.detect_possible_changes();
      hid_events
  }

  fn detect_possible_changes(&mut self) {
      for button in &mut self.buttons{
        button.possible_state = button.pin.read()
      }
  }

  fn detect_button_changes(button: &mut Button) -> Option<IOState> {
      if button.possible_state != button.state {
          if button.pin.read() == button.possible_state {
              button.state = button.possible_state;
              // change state ... reset the repeat counter
              button.repeat = 0;
              if button.state == Level::Low {
                Some(IOState::Pressed)
              } else {
                Some(IOState::Released)
              }
          } else {
              button.possible_state = button.state;
              None
          }
      } else {
          if button.state == Level::Low {
            button.repeat += 1;
            if button.repeat > 20 && button.repeat % 5 == 0 {
                button.repeat -= 5;
                Some(IOState::Repeated)
            } else {
                None
            } 
          } else {
            None
          }
      }
    }
}

pub mod helpers {

    #[allow(dead_code)]
    pub fn ba_to_console(hid_events: Vec<super::HIDEvent>, button_initializers: &Vec<super::ButtonInitializer>){
        for h_e in hid_events{
            print_h_e(&h_e.io_state, h_e.code, code_to_key(h_e.code, button_initializers));
        }
    }

    #[allow(dead_code)]
    fn print_h_e<T>(io_state: &super::IOState, code: u8, key: T) where T: std::fmt::Display {
        match io_state {
            super::IOState::Pressed => println!("{} was pressed code: {}", key, code),
            super::IOState::Released => println!("{} was released: code {}", key, code),
            super::IOState::Repeated =>  println!("{} was repeated: code {}", key, code), 
        }
    }

    #[allow(dead_code)]
    fn code_to_key(code: u8, button_initializers: &Vec<super::ButtonInitializer>) -> &str{
        let bi = button_initializers.iter().find(|bi|
            bi.code == code
        );
        match bi {
            Some(s) => s.key,
            None => ""
        }
    }
}


