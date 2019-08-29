# clinte &nbsp; [![Build Status](https://travis-ci.com/gbmor/clinte.svg?branch=master)](https://travis-ci.com/gbmor/clinte) [![codecov](https://codecov.io/gh/gbmor/clinte/branch/master/graph/badge.svg)](https://codecov.io/gh/gbmor/clinte)

Command-line community notice board. Post simple notes for other users to see. 

## Features

* Username is tagged based on the executing user
* Shows the 30 most recent posts in descending order
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

To make a new post:

```
$ clinte post
```

`clinte` will then ask for the title of the post, and the body. The username will be
tagged automatically by your logged-in username, reflecting its intended use on
multi-user UNIX-like systems.

## Contributing

This is very new, so feel free to hack on it in any way you
please and submit a PR! I'll be working on it myself over the next several days
to flesh it out.

