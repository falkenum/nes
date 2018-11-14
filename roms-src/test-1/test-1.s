
; iNES header
.segment "HEADER"

INES_MAPPER = 0
INES_MIRROR = 0 ; 0 = horizontal mirroring, 1 = vertical mirroring
INES_SRAM   = 0 ; 1 = battery backed SRAM at $6000-7FFF

.byte 'N', 'E', 'S', $1A ; ID
.byte $02 ; 16k PRG bank count
.byte $01 ; 4k CHR bank count
.byte INES_MIRROR | (INES_SRAM << 1) | ((INES_MAPPER & $f) << 4)
.byte (INES_MAPPER & %11110000)
.byte $0, $0, $0, $0, $0, $0, $0, $0 ; padding


; CHR ROM
.segment "TILES"

.byte $66
.byte $7F
.byte $FF
.byte $FF
.byte $FF
.byte $7E
.byte $3C
.byte $18

.dword $00000000
.dword $00000000

.byte $FC
.byte $FC
.byte $C0
.byte $F8
.byte $F8
.byte $C0
.byte $C0
.byte $C0

.dword $00000000
.dword $00000000

; Vectors, defined in CODE segment.
.segment "VECTORS"
.word nmi
.word reset
.word irq

; zero page variables
.segment "ZEROPAGE"
dummy:         .res 1 ; for EOR dummy (3 cycle "nop")
palette_start: .res 1 ; 0 for regular, 64 to include forbidden $0D
palette_index: .res 1 ; current index to palette data
ppu_ctrl:      .res 1 ; value to be written to $2001
ppu_emphasis:  .res 1 ; value for emphasis view
gamepad:       .res 1 ; polled gamepad
gamepad_last:  .res 1 ; last frame's gamepad
temp:          .res 1 ; temporary

.segment "OAM"
.assert ((* & $FF) = 0),error,"oam not aligned to page"
oam:     .res 256

; RAM variables
.segment "BSS"

; CODE
.segment "CODE"

main:	
	jmp main

nmi:	
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

	:	
	nop                           ;2
	nop                           ;2
	nop                           ;2
	nop                           ;2
	iny                           ;2
	bne :-                         ;2, +1 taken and +1 for page boundary
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

reset:	
	sei
	cld

	bit $2002
	:		
		bit $2002
		bpl :-

	ldx #$80
	stx $2000
	ldx #$0A
	stx $2001

	jmp main

irq:	
	rti
