PREFIX?=/usr/local
_INSTDIR=$(PREFIX)
BINDIR?=$(_INSTDIR)/bin
DBDIR?=$(_INSTDIR)/clinte

clinte:
	@printf "\n%s\n" "Checking out latest tag..."
	git checkout $(git describe --tags --abbrev=0)

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
	@printf "\n%s\n" "Making sure we're on master..."
	git checkout master

	@printf "\n%s\n" "Updating from upstream repository..."
	git pull --rebase
	
	@printf "\n%s\n" "Checking out latest tag..."
	git checkout $(git describe --tags --abbrev=0)
	
	@printf "\n%s\n" "...Done!"

.PHONY: install
install:
	@printf "\n%s\n" "Installing clinte..."
	@printf "\n%s\n" "Creating directories..."
	mkdir -p $(DBDIR)

	@printf "\n%s\n" "Copying files..."
	install -m755 target/release/clinte $(BINDIR)
	install -m666 clinte.json $(DBDIR)
	
	@printf "\n%s\n" "...Done!"

.PHONY: upgrade
upgrade:
	@printf "\n%s\n" "Upgrading clinte..."
	install -m755 target/release/clinte $(BINDIR)
	@printf "\n%s\n" "...Done!"

.PHONY: test
test:
	@printf "\n%s\n" "Running tests..."
	RUST_TEST_THREADS=1 cargo test
	@printf "\n%s\n" "...Done!"

.PHONY: uninstall
uninstall:
	@printf "\n%s\n" "Uninstalling clinte..."
	rm -f $(BINDIR)/clinte
	@printf "\n%s\n" "...Done!"
	@printf "%s %s\n" "The database is still intact in" $(DBDIR)
