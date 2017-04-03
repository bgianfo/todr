todr
----
`todr` is a command line tool which allows you to interact with your todoist
account from your terminal.

[![Linux build status](https://travis-ci.org/bgianfo/todr.svg?branch=master)](https://travis-ci.org/bgianfo/todr)
[![GitHub license](https://img.shields.io/github/license/bgianfo/todr.svg)]()

### Building/Installation

`todr` is written in Rust, so you'll need to grab a
[Rust installation](https://www.rust-lang.org/) in order to compile it.
Building is easy:

```
$ git clone https://github.com/bgianfo/todr
$ cd todr
$ cargo build --release
$ ./target/release/todr
```

Eventually I'd like to create a crate.

### Configuration

To configure todr you simply need to export your todoist authentication token as an environment variable.

1. First, log into [todoist.com](http://todoist.com).
2. Under the gear icon at the upper right, select "Todoist Settings".
3. Select the "Account" tab.
4. Copy the value next to "API token".

In your shell (or place in your .profile/.bashrc)
```
export TODR_AUTHTOKEN='<paste your token here>'
```

Now you can run todr:
```
$ ./target/release/todr
>> help

Commands:

  h | help  - This help message

  i | items - List all active todo items.

  p | proj  - List all active projects.

  q | quit  - Exit the application.

>> i
  Andy's Wedding Prep ()
    Buy Tickets for Andy's Wedding (Apr 1)
    Book hotel for andy's wedding (Apr 1)
  Update Wiki Documentation ()
    Update Testing Documentation ()
      Add page for combinatorial testing. ()
>>
```

### Running tests

To run the test suite, use:

```
$ cargo test
```

from the repository root.
