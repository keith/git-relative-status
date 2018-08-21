# git-relative-status

`git-relative-status` is meant to provide a parseable output format for
relative paths in a git repo. This is useful for providing a fuzzy
finding interface for editors or shell functions.

# Installation

Manually, after [installing rust](https://rustup.rs/):

```sh
cargo install
```

You need to have `source ~/.cargo/env` somewhere in your shell
configuration, then `git-relative-status` will be available from
`~/.cargo/bin/git-relative-status`.

# Examples

```sh
$ pwd
/foo/bar/repo
$ echo "hi" >> README.md
$ git relative-status
README.md
$ mkdir foo; cd foo
$ git relative-status
../README.md
```

## Why not use `git status`

If you want a parseable format, you should be using
[`--porcelain`](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain)
to get a stable format (although realistically it probably won't change
much in this case). Unfortunately this is [not
relative](https://git-scm.com/docs/git-status#_porcelain_format_version_1).
This makes it difficult to fuzzy find files from a nested directory in a
repo.

## Why rust

```sh
$ time git-relative-status # Rust
0.07s user 0.24s system 166% cpu 0.189 total
$ time git-relative-status.py # Python
0.14s user 0.30s system 124% cpu 0.358 total
```

### TODO

- This currently uses my personal filtering preferences, currently it
  excludes deleted files. These options should be provided by command
  line arguments instead.
- This currently doesn't output the specific status, assuming consumers
  won't care about it for the original use case. This should also be an
  option.
- File renames will still show in the format `from -> to`. In this case
  it should only print the `to`.
- Verify the behavior with submodules. I have no idea if this will work
  with submodules at all. Specifically:
  - If you're in a submodule directory, where will the paths be relative
    to
  - If you're in a directory containing a nested submodule, I assume
    just the submodule will be available. It would be nice to show any
    dirty files from the submodule too.
