	.org $0
	lda #$32
	tax
	stx $8000

	ldy #$64
	tya
	tax
	stx $8001

	ldx #$10
	txa
	tay
	sty $8002

	.org $FFFC
	.word 0000
	.word 0000
