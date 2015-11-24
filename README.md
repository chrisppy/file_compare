# file_compare

[![Build Status](https://img.shields.io/travis/red-oxide/file_compare.svg?style=flat-square)](https://travis-ci.org/red-oxide/file_compare)
[![License](https://img.shields.io/github/license/red-oxide/file_compare.svg?style=flat-square)](https://github.com/red-oxide/file_compare/blob/master/LICENSE)

CLI application for comparing files

## Installation
```
git clone https://github.com/red-oxide/file_compare.git
cd file_compare
cargo build --release
```
#### Linux
```
sudo mv target/release/file_compare /usr/local/bin
```

#### OSX
```
sudo mv <your_download_location>/file_compare /usr/local/bin/file_compare
```

#### Windows
- Create a folder for file_compare
- search for `env`
- open "edit your environment variables"
- edit `PATH`
- paste folder path to the end of the string i.e.: `<path_stuff_here>;C:/file_compare;`

## Usage
To use file_compare, use must add it to your path. Then you can call file_compare like so:
```
file_compare /dir/file_input /dir/file_to_compare_against
```

## Options
To see options run the following:
```
file_compare -h
```

## Bugs and feature requests

Please submit bugs and feature requests [here](http://github.com/red-oxide/file_compare/issues). Pull requests are always welcome.

## Copyright and License
(C) Copyright 2015 by Chris Palmer and contributors

[The list of contributors is available on GitHub](https://github.com/red-oxide/file_compare/graphs/contributors).

This application is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

See LICENSE for more information.
