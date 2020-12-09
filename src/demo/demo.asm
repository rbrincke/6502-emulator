DATA_START = $ddd0       ; First of 8 output bytes.
RANDOM = $eee0           ; Random number seed.
KEY = $fff0              ; Key press, either 'a' or 'd'.
KEY_IRQ_HANDLED = $fff1  ; Handled interrupt.

  .org $8000

reset:
  ldx #$ff
  txs
  jsr seed
  cli                    ; Accept interrupts from here on out.
  jmp idle

seed:
  lda RANDOM
  sta DATA_START
  lda RANDOM
  sta DATA_START + 1
  lda RANDOM
  sta DATA_START + 2
  lda RANDOM
  sta DATA_START + 3
  lda RANDOM
  sta DATA_START + 4
  lda RANDOM
  sta DATA_START + 5
  lda RANDOM
  sta DATA_START + 6
  lda RANDOM
  sta DATA_START + 7
  rts

check_move:
  ldy KEY
  cpy #'r'
  beq seed
  cpy #'a'
  beq jmp_move_left
  cpy #'d'
  beq move_right
  cpy #'w'
  beq move_up
  cpy #'s'
  beq move_down
  rts

jmp_move_left:           ; Destination out of range.
  jmp move_left

move_up:
  ldx DATA_START
  lda DATA_START + 1
  sta DATA_START
  lda DATA_START + 2
  sta DATA_START + 1
  lda DATA_START + 3
  sta DATA_START + 2
  lda DATA_START + 4
  sta DATA_START + 3
  lda DATA_START + 5
  sta DATA_START + 4
  lda DATA_START + 6
  sta DATA_START + 5
  lda DATA_START + 7
  sta DATA_START + 6
  stx DATA_START + 7
  rts

move_down:
  ldx DATA_START + 7
  lda DATA_START + 6
  sta DATA_START + 7
  lda DATA_START + 5
  sta DATA_START + 6
  lda DATA_START + 4
  sta DATA_START + 5
  lda DATA_START + 3
  sta DATA_START + 4
  lda DATA_START + 2
  sta DATA_START + 3
  lda DATA_START + 1
  sta DATA_START + 2
  lda DATA_START
  sta DATA_START + 1
  stx DATA_START
  rts

move_right:
  clc
  ror DATA_START
  ror DATA_START + 1
  ror DATA_START + 2
  ror DATA_START + 3
  ror DATA_START + 4
  ror DATA_START + 5
  ror DATA_START + 6
  ror DATA_START + 7
  rts

move_left:
  clc
  rol DATA_START
  rol DATA_START + 1
  rol DATA_START + 2
  rol DATA_START + 3
  rol DATA_START + 4
  rol DATA_START + 5
  rol DATA_START + 6
  rol DATA_START + 7
  rts

iterrupt_handler:
  sei
  jsr check_move
  lda #$0
  sta KEY_IRQ_HANDLED
  cli
  rti                    ; Note that an interrupt can come in before this.

idle:
  jmp idle

  .org $fffc
  .word reset
  .word iterrupt_handler
