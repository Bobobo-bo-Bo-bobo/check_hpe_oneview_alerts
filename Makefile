.PHONY: all build strip install clean
BINARY=check_hpe_oneview_alerts

all: build strip install

build:
	env PATH=${PATH}:${HOME}/.cargo/bin cargo build --release

strip: build
	strip --strip-all target/release/$(BINARY)

clean:
	env PATH=${PATH}:${HOME}/.cargo/bin cargo clean

install: strip
	test -d $(DESTDIR)/usr/lib/nagios/plugins || mkdir -m 0755 -p $(DESTDIR)/usr/lib/nagios/plugins
	install -m 0755 target/release/$(BINARY) $(DESTDIR)/usr/lib/nagios/plugins

uninstall:
	/bin/rm -f $(DESTDIR)/usr/lib/nagios/plugins/$(BINARY)

