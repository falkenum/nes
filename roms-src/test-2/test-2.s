
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
init_apu:
        ; Init $4000-4013
        ldy #$13
@loop:  lda @regs,y
        sta $4000,y
        dey
        bpl @loop
 
        ; We have to skip over $4014 (OAMDMA)
        lda #$0f
        sta $4015
        lda #$40
        sta $4017
   
        rts
@regs:
        .byte $30,$08,$00,$00
        .byte $30,$08,$00,$00
        .byte $80,$00,$00,$00
        .byte $30,$00,$00,$00
        .byte $00,$00,$00,$00

main:	
	
	lda #<279
	sta $4002
	
	lda #>279
	sta $4003
	
	lda #%10111111
	sta $4000

	jmp main

nmi:	
	rti

reset:	
	sei
	cld

	jsr init_apu

	jmp main
irq:	
	rti
