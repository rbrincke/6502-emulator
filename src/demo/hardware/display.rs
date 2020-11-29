use std::io::Write;
use std::sync::mpsc::Receiver;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{style, Attribute, Color, Print},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use emulator::emulator::registers::{Flag, Flag::*, Registers};

static STATUS_FLAGS: [Flag; 8] = [
    Carry, Zero, Interrupt, Decimal, Break, Reserved, Overflow, Negative,
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Mode {
    Step,
    Run,
}

pub(crate) struct DisplayData {
    pub(crate) registers: Registers,
    pub(crate) next_instruction: u8,
    pub(crate) data: [u8; 8],
    pub(crate) stack: Vec<StackEntry>,
    pub(crate) mode: Mode,
}

pub(crate) struct StackEntry {
    pub(crate) stack_pointer: u8,
    pub(crate) stack_value: u8,
}

const COL1: u16 = 5;
const COL2: u16 = 25;
const COL3: u16 = 40;
const COL4: u16 = 47;
const COL5: u16 = 65;

const LN_ADDR: u16 = 3;
const LN_NEXT_INST: u16 = 4;
const LN_MODE: u16 = 6;
const LN_STACK_PTR: u16 = 8;
const LN_ACC: u16 = 9;
const LN_X: u16 = 10;
const LN_Y: u16 = 11;
const LN_STATUS_START: u16 = 13;
const LN_DATA_START: u16 = 2;
const LN_STACK_CONTENTS_START: u16 = 2;
const LN_INSTRUCTIONS_START: u16 = 15;

pub(crate) fn display<W: Write>(stdout: &mut W, receiver: Receiver<DisplayData>) -> () {
    queue!(stdout, EnterAlternateScreen, Hide).unwrap();
    queue_labels(stdout);

    stdout.flush().unwrap();

    loop {
        // Read such that this thread is blocked when nothing interesting is going on.
        match receiver.recv() {
            Ok(display_data) => update_display(stdout, &display_data),
            Err(_) => break,
        }
    }

    execute!(stdout, Show, LeaveAlternateScreen).unwrap();
}

// Rewrite all of the data.
fn update_display<W: Write>(stdout: &mut W, display_data: &DisplayData) {
    print_value(
        stdout,
        display_data.registers.program_counter,
        LN_ADDR,
        COL2,
    );
    print_value(stdout, display_data.next_instruction, LN_NEXT_INST, COL2);
    print_value(
        stdout,
        display_data.registers.stack_pointer,
        LN_STACK_PTR,
        COL2,
    );
    print_value(stdout, display_data.registers.accumulator, LN_ACC, COL2);
    print_value(stdout, display_data.registers.x, LN_X, COL2);
    print_value(stdout, display_data.registers.y, LN_Y, COL2);

    for (idx, flag) in STATUS_FLAGS.iter().enumerate() {
        print_value(
            stdout,
            display_data.registers.status.get(*flag),
            LN_STATUS_START + idx as u16,
            25,
        );
    }

    for (idx, e) in display_data.stack.iter().enumerate() {
        print_value(
            stdout,
            e.stack_pointer,
            LN_STACK_CONTENTS_START + 1 + idx as u16,
            COL3,
        );
        print_value(
            stdout,
            e.stack_value,
            LN_STACK_CONTENTS_START + 1 + idx as u16,
            COL4,
        );
    }

    queue!(
        stdout,
        MoveTo(COL2, LN_MODE),
        Print(format!("{:?} ", display_data.mode))
    )
    .unwrap();

    for (idx, b) in display_data.data.iter().enumerate() {
        print_byte(stdout, *b, LN_DATA_START + 1 + idx as u16, COL5);
    }

    stdout.flush().unwrap();
}

// Print all of the labels.
fn queue_labels<W: Write>(stdout: &mut W) {
    let p = Print(
        style("6502 emulator demo")
            .with(Color::Black)
            .attribute(Attribute::Underlined)
            .attribute(Attribute::Bold),
    );

    queue!(stdout, MoveTo(COL1, 1), p).unwrap();

    // Section 1.
    print_label(stdout, "Address", LN_ADDR, COL1);
    print_label(stdout, "Next instruction", LN_NEXT_INST, COL1);
    print_label(stdout, "Stack pointer", LN_STACK_PTR, COL1);
    print_label(stdout, "Accumulator", LN_ACC, COL1);
    print_label(stdout, "X", LN_X, COL1);
    print_label(stdout, "Y", LN_Y, COL1);

    for (idx, flag) in STATUS_FLAGS.iter().enumerate() {
        print_label(
            stdout,
            &format!("{:?}", flag),
            LN_STATUS_START + idx as u16,
            5,
        );
    }

    // Section 2.
    print_label(stdout, "Stack address/value", LN_STACK_CONTENTS_START, COL3);

    // Section 3.
    print_label(stdout, "Data", LN_DATA_START, COL5);
    print_label(stdout, "Mode", LN_MODE, COL1);

    print_label(stdout, "'q' to quit", LN_INSTRUCTIONS_START, COL3);
    print_label(
        stdout,
        "'m' to toggle mode (any key to step)",
        LN_INSTRUCTIONS_START + 1,
        COL3,
    );
    print_label(
        stdout,
        "'r' to regenerate matrix",
        LN_INSTRUCTIONS_START + 2,
        COL3,
    );
    print_label(
        stdout,
        "'a', 's', 'd', 'w' for left, down, right, up",
        LN_INSTRUCTIONS_START + 3,
        COL3,
    );
}

fn print_label<W: Write>(stdout: &mut W, text: &str, line: u16, column: u16) {
    queue!(
        stdout,
        MoveTo(column, line),
        Print(style(text).with(Color::DarkYellow))
    )
    .unwrap()
}

fn print_value<W: Write, V: Into<u16>>(stdout: &mut W, value: V, line: u16, column: u16) {
    queue!(
        stdout,
        MoveTo(column, line),
        Print(format!("{:#04x}", value.into()))
    )
    .unwrap()
}

fn print_byte<W: Write>(stdout: &mut W, value: u8, line: u16, column: u16) {
    queue!(
        stdout,
        MoveTo(column, line),
        Print(format!("{:#010b}", value))
    )
    .unwrap()
}
