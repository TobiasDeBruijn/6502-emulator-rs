    .org $0000

    ldx #$32
    txs

    lda #$64
    pha

    ldy $132
    sty $8000

    lda #$10
    sta $120    ; Store onto the stack, which is at 0x0100
    ldx #$21    ; Stack pointer points at next free byte, so a PLA operation will read from 0x20
    txs         ; Transfer X to the stack pointer
    pla         ; Pull 0x020 into A
    sta $8001   ; Store A


    .org $FFFC
    .word 0000
    .word 0000
