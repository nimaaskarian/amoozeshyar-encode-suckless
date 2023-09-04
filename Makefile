include config.mk
all: enc-am

install-options:
	@echo enc-am install options:
	@echo "DESTDIR  = $(DESTDIR)"
	@echo "PREFIX   = $(PREFIX)"

options:
	@echo enc compile options:
	@echo "COMPILER = $(CC)"
	@echo "CFLAGS   = $(CFLAGS)"

enc-am: enc-am.rs
	$(CC) enc-am.rs -o enc-am

install: enc-am install-options
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f enc-am $(DESTDIR)$(PREFIX)/bin

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/enc-am

clean:
	rm enc-am
