# clinte &nbsp; [![Build Status](https://travis-ci.com/gbmor/clinte.svg?branch=master)](https://travis-ci.com/gbmor/clinte) [![codecov](https://codecov.io/gh/gbmor/clinte/branch/master/graph/badge.svg)](https://codecov.io/gh/gbmor/clinte)

Command-line community notice board for public-access UNIX systems. Post text-only notes for other users to see.

## Features

- Username is tagged based on the executing user
- Shows the 15 most recent posts in descending order
- Able to edit or delete your own posts
- Title <= 30 chars
- Body <= 500 chars
- Calls `$EDITOR` when creating or modifying the body of a post
- If `$EDITOR` is unset, calls `nano`
- Stores posts in JSON
- Uses advisory locking via `flock(2)` to synchronize access to the posts file

[![Screenshot](https://github.com/gbmor/clinte/blob/master/assets/clinte.png)](https://github.com/gbmor/clinte/blob/master/assets/clinte.png)

## Installation

Clone the repository and jump into the directory:

```
$ git clone git://github.com/gbmor/clinte.git
...
$ cd clinte
```

Run the makefile and install:

```
$ make
...
...Done!

$ sudo make install
```

`make` will automatically checkout the latest tag and build from there.

## Upgrading

**Note:** v1.0.0 used sqlite3, which presented some issues. v2.0.0 uses a json structure for posts,
as this will be safer on a multi-user system. When upgrading from v1.0.0 to v2.0.0, you won't be
able to save the posts without using a third-party tool to dump the `posts` table to json, and
manually adjusting it to fit the expected format (which can be seen in the included `clinte.json`).

*If upgrading from v1.0.0 -> v2.0.0, do a fresh install, including removing the database directory 
`/usr/local/clinte`. The following applies to upgrading when already running at least v2.0.0*

```
$ make update
$ make
$ make upgrade
```

This will:

* checkout `master`
* pull / rebase changes from upstream
* checkout the latest tag
* rebuild
* replace the `clinte` binary, but leave the posts file untouched.

## Usage

Issuing the program name itself will list
the currently available posts, like in the screenshot above.

**Display recent posts**

```
$ clinte
```

**Create a post**

```
$ clinte post
```

`clinte` will then ask for the title of the post, and the body. The username will be
tagged automatically by your logged-in username, reflecting its intended use on
multi-user UNIX-like systems.

**Edit a post**

```
$ clinte update [id]
```

If the `[id]` argument is absent, `clinte` will ask for the ID number of the post.
If it's been authored by you, then you will be asked for the new title.
Your `$EDITOR` will be called, and will be populated with the previous body.

**Delete a post**

```
$ clinte delete [id]
```

If the `[id]` argument is absent,  `clinte` asks for the numeric ID of the post to delete.

**Verbose logging**

```
$ clinte -v [post|update|delete] [id]
```
Use this flag if something's going wrong. Additional information will be written to
`/tmp/clinte_$USER.log` that will, hopefully, reveal the cause of the error.

## Notes

The file where the posts are stored, `/usr/local/clinte/clinte.json`, must be writeable by all
users on the system. Keep this in mind.