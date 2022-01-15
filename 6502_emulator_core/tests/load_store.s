    .org $0000
    lda #$32
    sta $2000

    ldx $2000
    stx $2032

    ldy $2000,X
    sty $2001

    txa
    tay

    sty $2002

    .org $FFFC
    .word $0000
    .word $0000
