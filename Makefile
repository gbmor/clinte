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

.PHONY: install
install:
	@sh install.sh $(PREFIX)

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
