DEBUG ?=

PROG := power-profiles-cfg
INSTALLDIR := /usr/lib

ifdef DEBUG
  release :=
else
  release := --release
endif

build:
	cargo build $(release)

install:
	@install --preserve-timestamps -D -m 644 power-profiles-cfg.service $(DESTDIR)$(INSTALLDIR)/systemd/system/power-profiles-cfg.service
	@install --preserve-timestamps -D -m 755 01power-profiles-cfg $(DESTDIR)$(INSTALLDIR)/pm-utils/sleep.d/01power-profiles-cfg
	@install --preserve-timestamps -D -m 755 target/release/$(PROG) $(DESTDIR)$(INSTALLDIR)/$(PROG)
	@printf "%s\n" "Install completed."

uninstall:
	@rm -f $(DESTDIR)$(INSTALLDIR)/systemd/system/power-profiles-cfg.service
	@rm -f $(DESTDIR)$(INSTALLDIR)/$(PROG)
	@printf "%s\n" "Uninstall completed."