PREFIX?=/usr/local
_INSTDIR=$(PREFIX)
BINDIR?=$(_INSTDIR)/bin
DBDIR?=$(_INSTDIR)/clinte

clinte: src/main.rs src/db.rs src/logging.rs
	@echo
	@echo Building clinte. This may take a minute or two.
	cargo build --release
	@echo
	@echo ...Done\!

.PHONY: clean
clean:
	@echo
	@echo Cleaning build cache of artifacts...
	cargo clean
	@echo
	@echo ...Done\!

.PHONY: update
update:
	@echo
	@echo Updating from upstream repository...
	@echo
	git pull --rebase
	@echo
	@echo ...Done\!

.PHONY: install
install:
	@echo
	@echo Installing clinte...
	@echo
	@echo Creating directories...
	mkdir -p $(BINDIR)
	mkdir -p $(DBDIR)
	@echo
	@echo Copying files...
	install -m755 target/release/clinte $(BINDIR)
	touch $(DBDIR)/clinte.db
	chmod 666 $(DBDIR)/clinte.db
	@echo
	@echo ...Done!

.PHONY: test
test:
	@echo
	@echo Running tests...
	@echo
	cargo test
	@echo
	@echo ...Done!

.PHONY: uninstall
uninstall:
	@echo
	@echo Uninstalling clinte...
	@echo
	@echo Removing files
	rm -f $(BINDIR)/clinte
	@echo
	@echo ...Done\!
