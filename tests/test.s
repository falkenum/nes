    *=$7FF0
    ;; !to "test.nes", plain
    !byte $4E, $45, $53, $1A, $02, $01
    !align $FFFF, $8000, $00
main
    lda #$00
    bpl main

nmi
;;; background palette
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

;;; sprite palette
    lda #$3F
    sta $2006
    lda #$11
    sta $2006

    ldx #$27
    stx $2007
    stx $2007
    stx $2007

;;; asserts
    ldy #$02

    lda #$00
    sta $2006
    sta $2006

    ldx $2007                   ; discard readbuf

    ldx $2007
    cpx #$66
    bne +

    ldx $2007
    cpx #$7F
    bne +

    ldx $2000
    cpx #$00
    bne +


    ldy #$00                    ; pass
+
    lda #$20
    sta $2006
    lda #$00
    sta $2006

    sty $2007

;;; OAM
    ldx #$00
    stx $2003
    ldx #$00
    stx $2004
    ldx #$00
    stx $2004
    stx $2004
    stx $2004

;;; no scrolling
    ldx #$00
    stx $2005
    stx $2005

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
    ldx #$1E
    stx $2001

    ;; init OAM
    ldx #$00
    stx $2003

    ldx #$FF
    ldy #$FF
-
    stx $2004
    dey
    cpy #$FF
    bne -

    jmp main

irq
    rti

    !align $FFFF, $FFFA, $00
    !word nmi
    !word reset
    !word irq
