#!/bin/sh

check_clinte()
{
    OS=$(uname)
    LOCAL_FILE="$HOME/.clinte.json"
    DBFILE='/usr/local/clinte/clinte.json'

    if [ "$OS" = 'Linux' ]
    then
        LOCAL_HASH=$(sha256sum "$LOCAL_FILE" 2>/dev/null | cut -d' ' -f1)
        DBFILE_HASH=$(sha256sum "$DBFILE" | cut -d' ' -f1)
    else
        LOCAL_HASH=$(sha256 "$LOCAL_FILE" 2>/dev/null | cut -d' ' -f1)
        DBFILE_HASH=$(sha256 "$DBFILE" | cut -d' ' -f1)
    fi
   
    if [ "$LOCAL_HASH" != "$DBFILE_HASH" ]
    then
        printf '%s\n\n' 'New posts on clinte!'
    fi
}

if [ ! -e "$HOME/.hushclinte" ]; then
    check_clinte
fi