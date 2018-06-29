  *=$7FF0
  ;; !to "test.nes", plain
  !byte $4E, $45, $53, $1A, $02, $01
  !align $FFFF, $8000, $00
main
  jmp main

nmi
;;; background palette
  lda #$3F                      ;2
  sta $2006                     ;4
  lda #$00                      ;2
  sta $2006                     ;4

  ldx #$01                      ;2
  stx $2007                     ;4
  ldx #$15                      ;2
  stx $2007                     ;4

  lda #$3F                      ;2
  sta $2006                     ;4
  lda #$05                      ;2
  sta $2006                     ;4

  ldx #$19                      ;2
  stx $2007                     ;4

  ldx #$00                      ;2
  stx $2001                     ;4
  ldx #$0A                      ;2

  ldy #$4F                      ;2

  ; no scrolling
  ; lda #$00                      ;2
  ; sta $2005                     ;4
  ; sta $2005                     ;4

; 52 total to here

-
  nop                           ;2
  nop                           ;2
  nop                           ;2
  nop                           ;2
  iny                           ;2
  bne -                         ;2, +1 taken and +1 for page boundary
; 13 per loop, 12 last loop

  nop                           ;2
  nop                           ;2
  nop                           ;2
  nop                           ;2
  nop                           ;2
  nop                           ;2
  nop                           ;2
  nop                           ;2

  stx $2001                     ;4

  ; stx $2007                     ;4

  ; no scrolling
  ; lda #$00                      ;2
  ; sta $2005                     ;4
  ; sta $2005                     ;4

  rti

reset
  sei
  cld

  bit $2002
-
  	bit $2002
  	bpl -

  ldx #$80
  stx $2000
  ldx #$0A
  stx $2001

  jmp main

irq
  rti

  !align $FFFF, $FFFA, $00
  !word nmi
  !word reset
  !word irq
