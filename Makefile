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
	install -m755 -o root -g root target/release/clinte $(BINDIR)

	@if [ -f "$(DBDIR)/clinte.json" ]; then printf "\n%s\n" "clinte.json exists. Skipping ..."; else install -m666 -o root -g root clinte.json "$(DBDIR)"; fi
	@if [ -e /etc/profile.d ]; then printf "%s\n" "Installing check_new_clinte_posts.sh to /etc/profile.d" && install -m644 -o root -g root check_new_clinte_posts.sh /etc/profile.d/; fi

	install -m644 -o root -g root clinte.1 $(PREFIX)/share/man/man1/

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
	rm -f $(PREFIX)/share/man/man1/clinte.1
	@if [ -e /etc/profile.d ]; then printf "%s\n" "rm -f /etc/profile.d/check_new_clinte_posts.sh" && rm -f /etc/profile.d/check_new_clinte_posts.sh; fi
	@printf "\n%s\n" "...Done!"
	@printf "%s %s\n" "The posts are still intact in" $(DBDIR)
