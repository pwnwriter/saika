PROJECT=saika # hexdump in rust
BINDIR=target/debug
COMPILER=cargo
OPTION=build
DESTDIR=/usr/bin

install:
	cd src
	${COMPILER} ${OPTION}
	install -Dm755 ${BINDIR}/${PROJECT} $(DESTDIR)/${PROJECT}
	
uninstall:
	rm -rf $(DESTDIR)/${PROJECT}
