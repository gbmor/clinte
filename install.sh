#!/bin/sh

# This is called in the Makefile.

if [ ! -e 'target/release/clinte' ]; then
    printf '\n%s "%s"\n\n' 'Please build clinte first:' 'make'
    exit 1
fi

PREFIX="$1"
BINDIR="$PREFIX/bin"
DBDIR="$PREFIX/clinte"
OS=$(uname)
BINGRP='root'
FILEGRP='root'

printf '\n%s\n' 'Installing clinte...'
printf '\n%s\n' 'Creating directories...'

mkdir -p "$DBDIR"

printf '\n%s\n' 'Copying files...'

if [ "$OS" = 'OpenBSD' ]; then
    BINGRP='bin'
    FILEGRP='wheel'
fi

install -m755 -o root -g "$BINGRP" target/release/clinte "$BINDIR"

if [ -f "$DBDIR/clinte.json" ]; then
    printf '\n%s\n' 'clinte.json exists. Skipping ...'
else 
    install -m666 -o root -g "$FILEGRP" clinte.json "$DBDIR"
fi

if [ -d /etc/profile.d ]; then
    printf '%s\n' 'Installing check_new_clinte_posts.sh to /etc/profile.d'
    install -m644 -o root -g "$FILEGRP" check_new_clinte_posts.sh /etc/profile.d/
fi

install -m644 -o root -g "$FILEGRP" clinte.1 "$PREFIX/share/man/man1/"

printf '\n%s\n' '...Done!'