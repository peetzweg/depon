# `depon` - "depends on"

Prevent execution of a subsequent command on the cli if the given dependency did not change.

```sh
Usage: depon [OBSERVE]...

Arguments:
  [OBSERVE]...

Options:
  -h, --help  Print help information
```

## Usage

```sh
> depon ./images/*.jpg && ./minify_images.sh 
```

The `./upload.sh ./task.json` part of this invocation will be executed if `depon` detects, that `./task.json` has been modified. If it does not have been modified, `depon` will prevent further execution.

## Install

```sh
cargo install depon
```

## How does it Work?

If `depon` detects a change in given dependencies or the dependencies have been touched, `depon` will exit with an error exit code, thereby preventing further execution of linked commands with the `&&` shell operator.
`Depon` keeps track of the dependencies between calls persistently in a `./depon.lock` file.
