  *=$7FF0
  ;; !to "test.nes", plain
  !byte $4E, $45, $53, $1A, $02, $01
  !align $FFFF, $8000, $00
main
  lda #$00
  bpl main

nmi
  lda #$3F
	sta $2006
  lda #$00
	sta $2006

  ldx #$01
  stx $2007

  ldx #$15
  stx $2007

  ldx #$19
  stx $2007

  ldx #$21
  stx $2007

  ;; lda #$20
  ;; sta $2006
  ;; lda #$00
  ;; sta $2006

  ;; lda #$00
  ;; sta $2007

  ;; lda #$23
  ;; sta $2006
  ;; lda #$C0
  ;; sta $2006
  ;; lda #$00
  ;; sta $2007

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
  ldx #$1A
	stx $2001

	jmp main

irq
  rti

  !align $FFFF, $FFFA, $00
  !word nmi
  !word reset
  !word irq
