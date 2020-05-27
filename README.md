# clinte &nbsp; [![Build Status](https://travis-ci.com/gbmor/clinte.svg?branch=master)](https://travis-ci.com/gbmor/clinte) [![codecov](https://codecov.io/gh/gbmor/clinte/branch/master/graph/badge.svg)](https://codecov.io/gh/gbmor/clinte)

Command-line community notice board. Post text-only notes for other users to see.

## Features

- Username is tagged based on the executing user
- Shows the 15 most recent posts in descending order
- Able to go back and edit your own posts
- Title <= 30 chars
- Body <= 500 chars
- Calls `$EDITOR` when creating or modifying the body of a post
- If `$EDITOR` is unset, calls `nano`

[![Screenshot](https://github.com/gbmor/clinte/blob/master/assets/clinte.png)](https://github.com/gbmor/clinte/blob/master/assets/clinte.png)

## Installation

Current build dependencies are as follows:

- `rust >= 1.36`
- `libsqlite3-dev`

The installation for the build deps will vary based on your OS (`Linux, BSD`)

Clone the repository and jump into the directory:

```
$ git clone git://github.com/gbmor/clinte.git
...
$ cd clinte
$ git checkout $(git describe --tags --abbrev=0)
```

Run the makefile and install:

```
$ make
...
...Done!

$ sudo make install
```

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

`sqlite` expects the directory where the database lies to be writeable by the user. So, until I move this
to using another storage medium (maybe plain text?), keep that in mind.
