# clinte &nbsp; [![Build Status](https://travis-ci.com/gbmor/clinte.svg?branch=master)](https://travis-ci.com/gbmor/clinte) [![codecov](https://codecov.io/gh/gbmor/clinte/branch/master/graph/badge.svg)](https://codecov.io/gh/gbmor/clinte)

Command-line community notice board. Post simple notes for other users to see. 

## Features

* Username is tagged based on the executing user
* Shows the 15 most recent posts in descending order
* Able to go back and edit your own posts
* Title <= 30 chars
* Body <= 500 chars

[![Screenshot](https://github.com/gbmor/clinte/blob/master/assets/clinte.png)](https://github.com/gbmor/clinte/blob/master/assets/clinte.png)

## Installation

Current build dependencies are as follows:

* `rustc/cargo >= 1.36`
* `libsqlite3-dev`

The installation for the build deps will vary based on your OS (`Linux, BSD`)

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

## Usage

Issuing the program name itself will list
the currently available posts, like in the screenshot above.

To display the 15 most recent posts:
```
$ clinte
```

To make a new post:

```
$ clinte post
```

`clinte` will then ask for the title of the post, and the body. The username will be
tagged automatically by your logged-in username, reflecting its intended use on
multi-user UNIX-like systems.

To edit a post:
```
$ clinte update
```
`clinte` will ask for the ID number of the post. If it's been authored by you,
then you will be asked for the new title and body after being shown the
previous title and body.

To delete a post:
```
$ clinte delete
```
Then `clinte` asks for the numeric ID of the post to delete.

## Contributing

This is very new, so feel free to hack on it in any way you
please and submit a PR! I'll be working on it myself over the next several days
to flesh it out.

## Notes

`clinte` has roughly 60 library dependencies from the `rust` ecosystem.
Nevertheless, the binary remains only `3.5MB` in size as of `v0.4.1`.
For your information, the direct library dependencies of `clinte` are:  
* `chrono >= 0.4`
* `clap >= 2.33`
* `lazy_static >= 1.4`
* `log >= 0.4`
* `rusqlite >= 0.20`
* `simplelog >= 0.7`
* `users >= 0.9`

The indirect dependencies (dependencies of dependencies) are:

`ansi_term, arrayref, arrayvec, atty, autocfg, backtrace, backtrace-sys, base64,
bitflags, blake2b_simd, byteorder, cc, cfg-if, cloudabi, constant_time_eq,
crossbeam-utils, dirs, dirs-sys, failure, failure_derive, fallible-iterator,
fallible-streaming-iterator, fuchsia-cprng, libc, libsqlite3-sys, linked-hash-map,
lru-cache, memchr, nodrop, num-integer, num-traits, pkg-config, proc-macro2, quote,
rand_core, rand_os, rdrand, redox_syscall, redox_users, rust-argon2, rustc-demangle,
strsim, syn, synstructure, term, textwrap, time, unicode-width, unicode-xid, vcpkg,
winapi, winapi-i686-pc-windows-gnu, winapi-x86_64-pc-windows-gnu`
