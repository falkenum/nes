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

;;; OAM
    ldx #$00
    stx $2003

    ldx #$00
    stx $2004                   ; y

    ldx #$00
    stx $2004                   ; tile

    ldx #$00
    stx $2004                   ; attr

    ldx #$00
    stx $2004                   ; x

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
    ldx #$16
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
