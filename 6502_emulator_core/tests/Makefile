bin/vasm:
	mkdir -p /tmp/vasm
	wget -O /tmp/vasm/vasm.tar.gz http://sun.hasenbraten.de/vasm/release/vasm.tar.gz
	tar -xf /tmp/vasm/vasm.tar.gz -C /tmp/vasm
	cd /tmp/vasm/vasm; \
		$(MAKE) CPU=6502 SYNTAX=oldstyle
	mkdir -p bin
	cp /tmp/vasm/vasm/vasm6502_oldstyle bin/vasm
	rm -rf /tmp/vasm