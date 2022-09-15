# git-brie - A modded git

## Synopsis

```
$ git branch --rename feat2 feature-2
[git-brie] $ git --move feat2 feature-2
```

## Description

A wrapper around git that mostly translates `git branch --rename` to `git branch --move`.

*Maybe in the future, it'll be a more generic mod-loader...*

## Installation

**Dependencies:** git

Download the binary from the release tab, and place it at `~/.local/bin/git`.

Alternatively:
- Create a symlink named `git` in one of your `PATH` directories that pointing to the executable binary
- Set `git-brie` as the shell alias for the `git` command
- Type `git-brie` or `git brie` every time you want to use `git`

## Developer notes

Currently, it exits unceremoniously when (weird IO) errors happen.

### Things that work

- git command passthrough
  - without infinite loop
  - with the right exit code
- `git --version` also reports our version when `GIT_BRIE_META=1`
- `git branch --rename <...>` (in that exact order) does `git branch --move <...>`
