# 6502 Emulator

In this repository you'll find a working emulator of the 6502 micro processing unit (MPU) and a small demo.

You have probably found this page because you are either implementing your own 6502 emulator or are curious about how it works. As always only the source code in `src/emulator` tells the whole story, but I've added [implementation notes](#emulator) below to clarify things that may not be immediately obvious (either to you, or to future me).

Alternatively you may just be curious about how the 6502 behaves, in which case you can go straight [to the demo](#demo). It allows you to run or step through assembly code and observe the 6502's registers and certain memory areas.

So why the 6502? When I bought a Raspberry Pi I came across [RetroPie](https://retropie.org.uk) and became increasingly curious about the inner workings of the Nintendo Entertainment System (NES), which uses a variant of this particular microprocessor. There is no better way to learn than to try to build one.

## Table of contents

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [Emulator](#emulator)
  - [Memory](#memory)
  - [Registers](#registers)
  - [Addressing](#addressing)
  - [Arithmetic](#arithmetic)
    - [Binary numbers](#binary-numbers)
      - [Addition (ADC)](#addition-adc)
      - [Subtraction (SBC)](#subtraction-sbc)
    - [Binary Coded Decimal](#binary-coded-decimal)
      - [Addition (ADC)](#addition-adc-1)
      - [Subtraction (SBC)](#subtraction-sbc-1)
  - [Jump to and return from subroutine](#jump-to-and-return-from-subroutine)
  - [Interrupts](#interrupts)
- [Demo](#demo)
  - [Assembly](#assembly)
    - [Installing the VASM Assembler](#installing-the-vasm-assembler)
  - [Usage](#usage)
  - [The demo machine](#the-demo-machine)
- [References](#references)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Emulator

Here I have added some general implementation notes. Together with the code, comments, and an extensive set of unit tests it should hopefully make it possible for anyone to follow along.

### Memory

The 6502 is an 8-bit microprocessor with a 16-bit address bus. With each slot 8-bit wide and 2<sup>16</sup> such slots available, it therefore has a total of 64 kilobytes of memory.

Depending on the hardware, all of this memory is potentially read/write accessible, though some of it should be avoided as it has innate meaning to the MPU itself.

| Address | Purpose |
|---------|---------|
| 0x0000..0x00ff  | Zero page |
| 0x0100..0x01ff  | Stack |
| 0xfffa..0xfffb  | NMI vector |
| 0xfffc..0xfffd  | Reset vector |
| 0xfffe..0xffff  | BRK and IRQ vector |

The first 256 bytes (0x0000 - 0x00ff) is accessed using zero page addressing, referred to as such because it represents the 0th page of memory as indicated by the leading 0x00 for the range.

The first page (0x0100 - 0x01ff) is the hardwired stack address space.

Addresses 0xfffa through 0xffff are hardwired to initialize the program counter after an MPU reset or interrupt.

### Registers

The 6502 has an 8-bit accumulator, 8-bit X and Y index registers, an 8-bit stack pointer, and a 16-bit program counter. Of these, only the program counter is made accessible outside of the crate mostly to allow for easy trap (infinite loop) detection. Others can be written to memory by using the corresponding instructions.

There are 7 status flags: carry, zero, interrupt, decimal (for binary coded decimal arithmetic), break, overflow and negative. The remaining bit is occupied by a reserved always-on flag, which is also present in this emulator.

How come none of the code seems to toggle the break flag directly? All of the material I have consulted states that the break flag does not really exist as a flag as such, and is only really ever pushed onto the stack as set by the PHP and BRK instructions rather than actually toggled.

### Addressing

Indirect addressing in the 6502 suffers from a bug with addresses 0x--FF (ending with FF) that has been reproduced in this emulator.

Relative addressing used for branches treat the 8-bit value as signed rather than unsigned, and is therefore able to go forwards as well as backwards.

### Arithmetic

The implementation of the ADC and SBC instructions is not entirely straighforward and warrants some explanation.

#### Binary numbers

In binary form, numbers are signed or unsigned.

Unsigned numbers are positive numbers. In binary form, an 8-bit value may range from 0 until 255.

Signed numbers are numbers that are either positive or negative. Because computers simply represent numbers as sequences of bits, a special representation is needed to indicate the sign of the number. The 6502 microprocessor uses the two's complement representation, which effectively treats the most significant bit, the sign bit, as negative.

For example, the number -128 is represented as `1000 0000` because that is the result of `-128 * 1 + 0 * 64 + 0 * 32 + 0 * 16 + 0 * 8 + 0 * 4 + 0 * 2 + 0 * 1`.

| Bit        |  1   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
|-----------:|:----:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Represents | -128 | 64  | 32  | 16  | 8   | 4   | 2   | 1   |

The largest negative number that can be represented in 8 bits is `-128`, whereas the largest positive number is `0111 1111` (which is `127`). Setting all bits to `1` represents the number `-1`.

A simple way to go from a binary number to its two's complement representation is to invert all of the bits, and add 1. For example, the number 3 is `0000 0011`, and its two's complement representation -3 is obtained by flipping all of the bits (`1111 1100`) and adding 1, resulting in `1111 1101`.

So why is it called two's complement? By definition, the sum of an 8-bit number and its two's complement representation is 2<sup>8</sup>.

##### Addition (ADC)

Using the two's complement representation, it is possible to easily add up any two numbers.

The examples below demonstrate that adding up two positive numbers, two negative numbers, or one positive and one negative number all lead to the correct results.

<pre>
(carry)           111
(+3)        0000 0011
(+5)        0000 0101
            --------- +
(+8)        0000 1000
</pre>

<pre> d
(carry)   1 1111 111
(-3)        1111 1101
(-5)        1111 1011
            --------- +
(-8)        1111 1000
</pre>

<pre>
(carry) 1   1111 1 1
(-3)        1111 1101
(+5)        0000 0101
            --------- +
(+2)        0000 0010
</pre>

Note that the bit length of 8 has been exceeded, and there is actually a 9th so-called carryout bit. Whether an addition resulted in a carryout is stored in the carry flag. The ADC operation stands for add-with-carry, and it uses the value of the carry flag as an additional input.

Imagine a program that wants to add the unsigned number 396 to itself. This number is too large for an 8-bit value, and adds two 16-bit values using two ADC instructions. The right-most part is added first, which leaves the carry flag set. This is convenient, because the second addition on the left automatically uses the enabled carry flag to continue the addition.

<pre>
(carry)           11         1 1
(+396)     0000 0001      1000 1100
(+396)     0000 0001      1000 1100
           --------- +    --------- +
(+792)     0000 0011      0001 1000
</pre>

For unsigned numbers, the ADC operation may return invalid results. Below is an example of what happens when the outcome exceeds what may be represented by an 8-bit signed integer.

<pre>
(carry)    1111 111   
(+127)     0111 1111
(+1)       0000 0001
           --------- +
(+2)       1000 0000
</pre>

Adding 127 and 1 leads to a result of -128, which is clearly incorrect. Two numbers with the sign bit set to 0 (positive) were added up, and the result is a number with the sign bit set to 1 (negative). Phrased differently, the sign of both inputs does not match the sign of the result.

This does not mean the overflow flag automatically indicates a result is invalid. The processor itself does not have a notion of signed or unsigned. If the numbers are considered unsigned, such as when adding `0111 1000` (+120) to itself, the operation results in `1111 0000` and leaves the overflow flag set. Whether this is relevant depends on whether the result represents a signed or unsigned number, and the flag is just there to support the cases for which it is.

##### Subtraction (SBC)

Subtraction works much like addition, except digits are borrowed rather than carried.

<pre>
(borrow)         1
(+5)       0000 0101
(+3)       0000 0011
           --------- -
(+2)       0000 0010
</pre>

The SBC instruction is a subtract-with-carry, and there is no borrow flag. The inverse of the carry flag is used to borrow, serving as a not-borrow flag. Whenever the carry flag is clear, executing an SBC instruction will lead to a result that is off by 1 because a cleared carry flag indicates a borrow.

Any subtraction can be rewritten as an addition by inverting the subtrahend. For example, `5 - 3` is the same as `5 + (-3)`. This means a subtraction can be rewritten as an addition by taking the two's complement of the subtrahend. 

Taking the two's complement is done by inverting all bits, and adding one. Since the carry flag in its role as not-borrow _already_ takes care of adding the 1, SBC can be performed  using ADC with the subtrahend's bits inverted.

#### Binary Coded Decimal

In binary coded decimal (BCD) form, bytes represent decimal rather than binary numbers. The 6502 uses a packed binary decimal format in which each byte contains two groups of four bits (a nibble) that each represent a decimal number.

Consider the number 99. In binary this is `0110 0011`, but in packed binary coded decimal it is `1001 1001`: each nibble represents the number 9. Note that it is much easier to use the hexadecimal notation 0x99.

Not every constellation of bits represents a valid BCD number. Specifically, any nibble representing a number outside of the range 0-9 are not legal binary coded decimal.

##### Addition (ADC)

So how does BCD addition work?

1. Split the bytes in two nibbles representing the least and most significant numbers. 
2. Add the least significant numbers and the carry. 
3. If the result exceeds 9, take the remainder and set an intermediate carry.
4. Add the two most significant numbers and the intermediate carry.
5. If the result exceeds 9, again take the remainder and set a carry.
6. Add up the results to obtain the outcome.

**Example**

It is perhaps easier to demonstrate through an example computing 99 + 9. No existing carry is set.

1. Represented in packed BCD, this means adding `1001 1001` and `0000 1001`, where the nibble on the right represents the least significant number.
2. Add `1001` to `1001`, leading to `11001` (decimal 18).
3. This number exceeds 9, so an intermediate carry is set and the remainder is taken, leading to `1000` (decimal 8).
4. Add `1001` and `0000` and the carry `0001`, leading to `1010` (decimal 10).
5. This exceeds 9, so the remainder is taken `0000` (decimal 0) and the carry is set.
6. The outcome is therefore `0000 1000` (decimal 8) with the carry set. This represents the decimal number 108.

A convenient way to take the remainder and account for the carry is to add 6 to any result exceeding 9.

For example, imagine the number `1100` (decimal 12). Adding 6 leads to `0001 0010`, representing the decimal number 12.

##### Subtraction (SBC)

*Binary* subtraction was rewritten as binary addition by inverting the subtrahend using twos' complement notation. A similar conversion can be applied for binary coded decimal subtraction.

For BCD subtraction to be rewritten as addition, the nine's complement is used to rewrite the subtrahend. In nine's complement each digit of a decimal number is set to 9 minus the digit's value. Then 1 is added to the result. The final outcome exceeds the result by 100.

It is perhaps easier to see what's going on in algebraic form. The nine's complement of some number B (where B is positive and < 100) is equivalent to substituting it by 99 - B. Subtracting A - B in the above method is therefore equivalent to A + (99 - B) + 1 - 100. These two operations are identical.

So how does BCD subtraction work?

1. Set the carry.
2. Take the nine's complement of the subtrahend.
3. Perform BCD addition.

Although there is no way to express negative numbers in BCD (directly), the result of a subtraction will be negative is the subtrahend exceeds the minuend. This is indicated by the carry, which is clear for results below zero and set otherwise.

**Example**

Let's compute 55 - 34.

1. Set the carry.
2. The nine's complement of 34 is 65: the most significant digit 3 becomes 6 (9 - 3 = 6) while the least significant digit becomes 5 (9 - 4 = 5).
3. The subtraction 55 - 34 then leads to 55 + 65 + 1 (the carry), which equals 21 with the carry set.

Note that the memory final outcome has the carry set. The result is interpreted as 121 - 100 = 21.

What if the numbers were the other way around? The subtraction 34 - 55 leads to 34 + 44 + 1, which equals 79 with the carry clear, indicating a result below zero. The result is interpreted as 79 - 100 = -21.

### Jump to and return from subroutine

Jump to subroutine (JSR) pushes the program counter plus _one_ onto the stack. At first glance this seems wrong. After all, JSR is a 3-byte instruction (1 for the instruction, 2 for the 16-bit address that follows) and so it is _one byte short_ of the actual resume location.

So what's going on here? The return from subroutine (RTS) instruction pops the program counter value pushed by JSR off the stack and _also_ increments it by one. It therefore balances out nicely, though none of this is immediately obvious.

### Interrupts

The 6502 has 3 interrupts: one software interrupt (BRK), and two hardware interrupts (IRQ and NMI).

All interrupts trigger roughly the same sequence of events: set the `Interrupt (Disable)` flag, push the program counter and the status flags onto the stack, and load the relevant interrupt handler. It is up to the programmer to ensure that any other registers are restored correctly.

So how are the interrupts different?

BRK represents a software interrupt, and is actually a 2-byte instruction. The program counter that is pushed on the stack skips the 2nd byte, unlike for the hardware interrupts (I often read this comes in handy when using BRK for debugging purposes, where the 2nd byte offers a convenient place to store debug information).

For BRK, the `Break` flag is set, which is useful to distinguish between software and hardware interrupts.

IRQ represents an interrupt request, a type of interrupt that is serviced when the `Interrupt (Disable)` flag is clear. It is level-triggered, and as such is both set and cleared from outside the MPU.

NMI is a non-maskable interrupt, unaffected by the `Interrupt` flag. It is edge-triggered, and is set from outside the MPU but cleared once the interrupt is serviced.

Both hardware interrupts are represented as externally accessible booleans (`irq` and `nmi`) that are polled during each instruction cycle.

After an interrupt, the RTI instruction resumes execution of the program. Since the correct program counter is pushed onto the stack, no correction as was required for `JSR` and `RTS` is necessary.

## Demo

An emulator is really no fun unless you can actually demonstrate that it does something meaningful (in the widest sense of the word).

I opted for a setup that does something mildly interesting while showing what is going on inside the emulator itself. This can either be run directly, though it's probably more fun to fiddle with the code yourself (as I have done many times).

![UI](https://gist.githubusercontent.com/rbrincke/68f94124c28123d813b849c691d689bc/raw/6548ec1b32feb35798942c00a6980573883dc4af/6502.gif)

*A look inside the machine*.

### Assembly

Writing machine code is tedious and prone to error. What we need to really get the most out of this is a decent assembler for the 6502.

The [VASM assembler](http://sun.hasenbraten.de/vasm/) seems to tick all the boxes: it supports the 6502 family, supports the `oldstyle` syntax suitable for this 8-bit MPU, and can output binaries.

#### Installing the VASM Assembler

If you are on an x86-64 machine, the quickest way to get started is by downloading the binaries from [here](http://www.compilers.de/vasm.html). This is not the latest version, but it'll work just fine.

To install the latest version of the VASM assembler, download its sources and compile them as follows.

```shell script
curl -OL http://sun.hasenbraten.de/vasm/daily/vasm.tar.gz
tar -xzf vasm.tar.gz
make -C vasm CPU=6502 SYNTAX=oldstyle
```

Once it has compiled, the file of interest is `vasm6502_oldstyle`. For this project that file is expected to reside in the `dependencies` directory, so copy it there.

```shell script
mkdir -p dependencies && cp vasm/vasm6502_oldstyle dependencies
```

Everything else can be safely deleted.

```shell script
rm vast.tar.gz
rm -r vasm
```

Note that it is possible to manually send files to the assembler as well.

```shell script
./dependencies/vasm6502_oldstyle code.asm -dotdir -Fbin -o ./code.bin
```

Writing assembly code is tricky, and it's a fair bet that things will not always work straight out of the gate. Fortunately `.asm` files can be sent to the assembler as part of the build so it is quick to iterate.

Cargo's supports this by means of the [build.rs](build.rs) file. It ensures the `.asm` file is sent to the assembler as part of the build whenever it changes and that the assembled binary is written to the target directory.

A macro of the form `asm!("name")` pulls the assembled binaries into the Rust code base.

### Usage

Code in this project relies on [Rust and Cargo](https://www.rust-lang.org/tools/install). Make sure it is installed, then open a terminal window and execute

```shell
cargo run
```

The demo program contains a matrix of 8 bits that are seeded using random numbers. It also shows the contents of the 6502's registers.

The demo starts in step mode and waits for a key press. Press any key to step to the next instruction. (Note that later in the program there is an `idle` loop that jumps to itself, making it look like nothing is moving.)

Press `m` to toggle to run mode (you can always press `m` again to switch back to step mode) such that it no longer waits for key presses. Use the keys `a`, `s`, `d` and `w` to shift the bits in the matrix left, right, up or down. Left and right shifts cause the carry bit to shift into the next row, while up and down simply translate the rows.

Use `r` to regenerate the random numbers. Note that you can toggle the mode using `m` at any point, or press `q` to quit.

### The demo machine

The demo machine is implemented as a simple application. Inputs and outputs go via a terminal window with the help of `crossterm`.

For any interaction with the outside world the machine assumes the following mapped memory areas.

| Address | Purpose |
|---------|---------|
| 0xddd0..0xddd8  | 8-bit display matrix |
| 0xeee0  | Random number generator |
| 0xfff0  | Key press ASCII value |
| 0xfff1  | Key IRQ (1 = unserviced, 0 = serviced) |

Eight addresses starting at `0xddd0` contain the display matrix and are directly mapped to the terminal window output approximately every 50ms. They appear as binary numbers and can be thought of as individual pixels forming a sprite. This was inspired by [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8), which does something highly similar.

The address `0xeee0` is a source for random numbers. A new random number is available whenever it is read.

Most interesting are `0xfff0` and `0xfff1` that support the [interrupt](#interrupts) for key presses. If one of the keys `a`, `s`, `d`, `w` or `r` is pressed, its ASCII value is written to `0xfff0` and `IRQ` is held. This in turn triggers the 6502's interrupt service request, which checks which key was pressed and acts accordingly. Address `0xfff1` is then set to 0 to indicate the MPU has serviced the interrupt, which in turn means the `IRQ` is released.

In step mode, holding down a valid input key causes the stack pointer to drift downwards. This is because the interrupt handler enables interrupts using `cli` at the end, and this is done before the `rti` instruction.

The assembly code can be found in `src/demo/demo.asm`.

## References

- [Andrew John Jacobs' very good overview of the 6502.](http://www.obelisk.me.uk/6502/)
- [6502.org.](http://www.6502.org)
- [Klaus Dormann's extensive 6502 test suite.](https://github.com/Klaus2m5/6502_65C02_functional_tests)
