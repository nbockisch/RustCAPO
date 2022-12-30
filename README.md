# RustCAPO
RustCAPO is an implementation of NRAO's SSA CAPO (CASA, Archive, and Pipeline Options)
in the Rust programming language.  It is shipped as a library with a simple API and a command line
utility that calls the library and produces results suitable for use in a shell
script, enabling us to make quick CAPO enabled scripts.

CAPO is a configuration system that reads values from multiple property files,
these files are delimited by the combination of two options to it, CAPO_PATH
and CAPO_PROFILE:

CAPO_PATH is a colon delimited list of directories to search for property files,
like '/home/casa/capo:/home/ssa/capo:/etc/capo'. The same property can be read
from multiple files, and in this case the later property replaces the earlier
property. CAPO_PATH can be given as an argument to the library or CLI app, or
RustCAPO will look for a CAPO_PATH environment variable. If those two are missing
RustCAPO defaults to '/home/casa/capo:/home/ssa/capo:$HOME/.capo'. RustCAPO skips
over missing or unreadable property files (this is intentional).

CAPO_PROFILE describes the profile RustCAPO looks for, e.g. 'test', 'staging',
'production', and RustCAPO expects the property files on the CAPO_PATH it looks
for to be named $profile.properties, e.g., /home/casa/capo/test.properties.
CAPO_PROFILE can be an argument to the library or CLI app, or RustCAPO will look
for a CAPO_PROFILE environment variable. If both of those are missing RustCAPO
will complain and die (this is also intentional).

CAPO isn't yet robust against things like profiles with spaces in their name,
and it has only been tested under Linux and MacOS.

## Usage
### As a library
Library documentation can be generated from the doc comments. See the [Documentation](#Documentation)
section for more on this.

### As a binary
The binary provides the following options listed in its help message:

```
A Rust implementation of SSA CAPO (CASA, Archive, and Pipeline Options)

Usage: rustcapo [OPTIONS]

Options:
      --path <PATH>          Path of directories to search
  -A, --all                  Display all settings
  -q, --quiet                quiet mode; only display the value
      --settings <SETTINGS>  one or more settings to query, ignored if -A
  -P, --profile <PROFILE>    profile name to use, e.g. test, production
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Building
### As a library
Within the `dependencies` section of your `Cargo.toml` file, add the following:

`rustcapo = { git = "https://gitlab.nrao.edu/nbockisc/RustCAPO" }`

Then build the project to import and build the library for use.

### As a static binary
From within the repo on a Linux machine, run:

`RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu`

This will create a static binary in `target/x86_64-unknown-linux-gnu/release/rustcapo`

## Tests
Tests exist in the `tests` module. To run them, cd to the root of the repo and
run `cargo test --lib`. Note that the current tests exist to test the library
portion of this crate.

## Documentation
Documentation can be generated from the doc comments by running `cargo doc --open`
from within the repo. This will generate HTML documentation for the library that
will open in a web browser.

## License
You should have received a copy of the GNU General Public License
along with GoCAPO.  If not, see <https://www.gnu.org/licenses/gpl-3.0>.


Copyright (C) 2022 Associated Universities, Inc. Washington DC, USA.
