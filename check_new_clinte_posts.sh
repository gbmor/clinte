#!/bin/sh

LOCAL_FILE="$HOME/.clinte.json"
LOCAL_HASH=$(sha256sum "$LOCAL_FILE" | cut -d' ' -f1)

DBFILE="/usr/local/clinte/clinte.json"
DBFILE_HASH=$(sha256sum "$DBFILE" | cut -d' ' -f1)

if [ "$LOCAL_HASH" != "$DBFILE_HASH" ]
then
    printf "%s\n" "New posts on clinte!"
fi