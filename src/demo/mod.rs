extern crate core;
extern crate crossterm;
extern crate emulator;
extern crate rand;

use std::io::stdout;
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use emulator::emulator::Emulator;
use emulator::memory::Memory;
use hardware::display::{display, DisplayData, Mode};
use hardware::interface::HardwareInterface;
use hardware::keys::keys;
use hardware::keys::InputData::{AnyKey, EmulatorKey, ToggleMode};

#[macro_use]
mod asm;
mod hardware;

impl Mode {
    fn toggle(&self) -> Mode {
        match self {
            Mode::Step => Mode::Run,
            Mode::Run => Mode::Step,
        }
    }
}

const REFRESH_INTERVAL_MS: u64 = 50;

fn main() {
    // Create memory with our assembled program.
    let memory = HardwareInterface::from_binary(asm6502!("demo"), 0x8000);

    // Attach memory to the emulator.
    let mut emulator = Emulator::new(memory);

    // Fire up display peripheral.
    let (display_sender, display_receiver) = channel();
    let terminal_thread = thread::spawn(move || {
        display(&mut stdout(), display_receiver);
    });

    // Fire up keyboard peripheral.
    let (keys_sender, keys_receiver) = channel();
    let keys_thread = thread::spawn(move || keys(keys_sender));

    // Go into raw mode to detect single key presses.
    enable_raw_mode().unwrap();

    // Wait for keypress before executing instruction.
    let mut step_mode = Mode::Step;

    // Refresh rate for Run mode..
    let refresh_interval = Duration::from_millis(REFRESH_INTERVAL_MS);
    let mut last_refresh = Instant::now();

    loop {
        // Type annotation present to unconfuse the IntelliJ plugin.
        let interface: &mut HardwareInterface = &mut emulator.memory;

        // Refresh screen immediately in Step mode (which waits for keypress), avoid
        // too many refreshes in Run mode.
        if step_mode == Mode::Step || last_refresh.elapsed() > refresh_interval {
            let display_data = DisplayData {
                registers: emulator.registers.clone(),
                next_instruction: interface.read(emulator.registers.program_counter),
                data: interface.display_matrix_data(),
                stack: interface.stack_around_stack_pointer(emulator.registers.stack_pointer),
                mode: step_mode,
            };

            display_sender.send(display_data).unwrap();
            last_refresh = Instant::now();
        }

        // Release interrupt request if appropriate.
        if interface.is_irq_handled() {
            emulator.irq = false;
        }

        // Check for key press.
        let input_data = match step_mode {
            Mode::Step => match keys_receiver.recv() {
                Ok(c) => Some(c),
                Err(_) => break,
            },

            Mode::Run => match keys_receiver.try_recv() {
                Ok(c) => Some(c),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => break,
            },
        };

        // Handle key press.
        if let Some(c) = input_data {
            match c {
                ToggleMode => {
                    step_mode = step_mode.toggle();
                }

                EmulatorKey { c } => {
                    interface.send_key(c);
                    interface.set_irq();
                    emulator.irq = true;
                }

                AnyKey => {}
            }
        }

        // Execute instruction.
        emulator.execute_next();

        // Avoid using too much CPU.
        thread::sleep(Duration::from_micros(250));
    }

    // Exit raw mode.
    disable_raw_mode().unwrap();

    // Drop the sender to stop the terminal screen thread.
    drop(display_sender);

    // Wait for threads to finish.
    terminal_thread.join().unwrap();
    keys_thread.join().unwrap();
}
