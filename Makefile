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
	@install --preserve-timestamps -D -m 644 power-profiles-cfg.service $(INSTALLDIR)/systemd/system/power-profiles-cfg.service
	@install --preserve-timestamps -D -m 755 target/release/$(PROG) $(INSTALLDIR)/$(PROG)
	@printf "%s\n" "Install completed."

uninstall:
	@rm -f $(INSTALLDIR)/systemd/system/power-profiles-cfg.service
	@rm -f $(INSTALLDIR)/$(PROG)
	@printf "%s\n" "Uninstall completed."