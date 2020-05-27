PREFIX?=/usr/local
_INSTDIR=$(PREFIX)
BINDIR?=$(_INSTDIR)/bin
DBDIR?=$(_INSTDIR)/clinte

clinte:
	@printf "\n%s\n" "Building clinte. This may take a minute or two."
	cargo build --release
	@printf "\n%s\n" "...Done!"

.PHONY: clean
clean:
	@printf "\n%s\n" "Cleaning build cache of artifacts..."
	cargo clean
	@printf "\n%s\n" "...Done!"

.PHONY: update
update:
	@printf "\n%s\n" "Updating from upstream repository..."
	git pull --rebase
	@printf "\n%s\n" "...Done!"

.PHONY: install
install:
	@printf "\n%s\n" "Installing clinte..."
	@printf "\n%s\n" "Creating directories..."
	mkdir -p $(BINDIR)
	mkdir -p $(DBDIR)
	@printf "\n%s\n" "Copying files..."
	install -m755 target/release/clinte $(BINDIR)
	touch $(DBDIR)/clinte.db
	chmod 666 $(DBDIR)/clinte.db
	chmod 777 $(DBDIR)
	@printf "\n%s\n" "...Done!"

.PHONY: test
test:
	@printf "\n%s\n" "Running tests..."
	cargo test
	@printf "\n%s\n" "...Done!"

.PHONY: uninstall
uninstall:
	@printf "\n%s\n" "Uninstalling clinte..."
	rm -f $(BINDIR)/clinte
	@printf "\n%s\n" "...Done!"
	@printf "%s %s\n" "The database is still intact in" $(DBDIR)
