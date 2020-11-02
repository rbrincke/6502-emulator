# NES

## Binary numbers

In binary form, numbers are signed or unsigned.

Unsigned numbers are positive numbers. In binary form, an 8-bit value may range from 0 until 255.

Signed numbers are numbers that are either positive or negative. Because computers simply represent numbers as sequences of bits, a special representation is needed to indicate the sign of the number. The NES processor uses the Two's Complement representation, which effectively treats the most significant bit, the sign bit, as negative.

For example, the number -128 is represented as `1000 0000` because that is the result of `-128 * 1 + 0 * 64 + 0 * 32 + 0 * 16 + 0 * 8 + 0 * 4 + 0 * 2 + 0 * 1`.

| Bit        |  1   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
|-----------:|:----:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Represents | -128 | 64  | 32  | 16  | 8   | 4   | 2   | 1   |

The largest negative number that can be represented in 8 bits is `-128`, whereas the largest positive number is `0111 1111` (which is `127`). Setting all bits to `1` represents the number `-1`.

A simple way to go from a binary number to its Two's Complement representation is to invert all of the bits, and add 1. For example, the number 3 is `0000 0011`, and its Two's Complement representation -3 is obtained by flipping all of the bits (`1111 1100`) and adding 1, resulting in `1111 1101`.

So why is it called Two's Complement? By definition, the sum of an 8-bit number and its Two's Complement representation is 2<sup>8</sup>.

## Jump to and return from subroutine (JSR, RTS)

The JSR instruction is 3 bytes long, the latter two of which are the address to which the program jumps. Oddly, the two bytes that are pushed onto the stack do not constitute the location where the program resumes (which would be right after the 3 bytes of the JSR instruction), but instead points to the location of byte 3 of the JSR instruction, one byte short of the actual location.

The RTS instruction in turn returns from a subroutine by popping two bytes off the stack. Because this location is actually at byte 3 of the JSR, the RTS instruction increments this value by 1, which sets the program counter to the correct address.

Because two bytes are pushed onto the stack, it is perhaps worth mentioning that popping something off the stack returns whatever was last pushed.

## Binary arithmetic

### Addition (ADC)

Using the Two's Complement representation introduced before, it is possible to easily add up any two numbers.

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

Note that the bit length of 8 has been exceeded, and there is actually a 9th so-called carryout bit. Whether an addition resulted in a carryout is stored in the Carry flag. The ADC operation stands for Add with Carry, and it uses the value of the carry flag as an additional input.

Imagine a program that wants to add the unsigned number 396 to itself. This number is too large for an 8-bit value, and adds two 16-bit values using two ADC instructions. The right-most part is added first, which leaves the Carry flag set. This is convenient, because the second addition on the left automatically uses the enabled carry flag to continue the addition.

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

### Subtraction (SBC)

Subtraction works much like addition, except digits are borrowed rather than carried.

<pre>
(borrow)         1
(+5)       0000 0101
(+3)       0000 0011
           --------- -
(+2)       0000 0010
</pre>

The SBC instruction is a subtract-with-carry, and there is no borrow flag. The inverse of the carry flag is used to borrow, serving as a not-borrow flag. Whenever the carry flag is clear, executing an SBC instruction will lead to a result that is off by 1 because a cleared carry flag indicates a borrow.

Any subtraction can be rewritten as an addition by inverting the subtrahend. For example, `5 - 3` is the same as `5 + (-3)`. This means a subtraction can be rewritten as an addition by taking the Two's Complement of the subtrahend. Taking the Two's Complement is done by inverting all bits, and adding one. Since the carry flag in its role as not-borrow already takes care of adding the 1, SBC is the same as ADC for which the second term's bits are inverted.

## Binary Coded Decimal (BCD)

In binary coded decimal form, bytes represent decimal numbers. The 6502 uses a packed binary decimal in which each two groups of four bytes each represent a number.
