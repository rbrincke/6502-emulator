use std::sync::mpsc::Sender;

use crossterm::event::{read, Event, KeyCode, KeyEvent};

use hardware::keys::InputData::{AnyKey, EmulatorKey, ToggleMode};

#[derive(Clone)]
pub(crate) enum InputData {
    ToggleMode,
    EmulatorKey { c: char },
    AnyKey,
}

pub(crate) fn keys(keyboard_data: Sender<InputData>) {
    // Wait for keypress.
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                let message = match c {
                    'q' => {
                        drop(keyboard_data);
                        break;
                    }

                    'm' => ToggleMode,
                    'a' | 's' | 'd' | 'w' | 'r' => EmulatorKey { c },
                    _ => AnyKey,
                };

                keyboard_data.send(message).unwrap()
            }

            // Ignore.
            _ => {}
        }
    }
}
