  *=$7FF0
  !to "test.nes", plain
  !byte $4E, $45, $53, $1A, $02, $01
  !align $FFFF, $8000, $00
main
  bpl main
nmi
	lda #$3F
	sta $2006
  lda #$00
	sta $2006

  cpy #$20
  bne +
  stx $2007
  inx
  ldy #$00
+
  iny
  rti
reset
  sei
	cld
	ldx #$80
	stx $2000
  ldx #$00
	stx $2001

  ldx #$01
  ldy #$00
;; 	bit $2002
;; -
;; 		bit $2002
;; 		bpl -
;; 	lda #$00
;; 	tax
;; -
;; 		sta $0000, X
;; 		sta $0100, X
;; 		sta $0200, X
;; 		sta $0300, X
;; 		sta $0400, X
;; 		sta $0500, X
;; 		sta $0600, X
;; 		sta $0700, X
;; 		inx
;; 		bne -
;; -
;; 		bit $2002
;; 		bpl -
	jmp main
irq
  rti

  !align $FFFF, $FFFA, $00
  !word nmi
  !word reset
  !word irq

  ;; *=0
  ;; !fill $2000, $00
