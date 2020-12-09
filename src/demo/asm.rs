/// Load the compiled 6502 binary for \<name\>.asm from the OUT_DIR.
macro_rules! asm6502 {
    ($l:literal) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", $l, ".bin")).to_vec()
    };
}
