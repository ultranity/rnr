<p align="center">
<img src="https://user-images.githubusercontent.com/8478202/107156909-59030580-6981-11eb-9374-95959b6ec067.png" width="350" height="350" alt="rnr">
</p>

<p align="center">
    <a href="https://github.com/ismaelgv/rnr/actions?query=workflow%3ARnR">
        <img src="https://github.com/ismaelgv/rnr/workflows/RnR/badge.svg" alt="Build Status"></a>
    <a href="https://crates.io/crates/rnr">
        <img src="https://img.shields.io/crates/v/rnr.svg" alt="Crates.io"></a>
    <a href="https://github.com/ismaelgv/rnr/blob/master/LICENSE">
        <img src="https://img.shields.io/crates/l/rnr.svg" alt="License"></a>
</p>

<p align="center">
    <b>RnR</b> is a command-line tool to <b>securely rename</b> multiple files
    and directories that supports regular expressions.
</p>

## Features
* Batch rename files and directories.
* Automated checks to avoid unwanted file collisions, removals or overwrites.
* Use regexp, including capture groups.
* Include directories recursively.
* Create backup files.
* Create and read operations from dump file.
* Undo operations from dump file.
* Exclude/include hidden files.
* Linux, Mac and Windows support, including terminal coloring.
* Extensive unit testing.
* Select limit of replacements.
* Apply text transformations to the replacements including capture groups.
* Convert UTF-8 file names to ASCII representation.
* Interactive rename (and optional delete) using your preferred text editor, similar to `vidir`.

# Install

## Binaries

### GitHub Releases
You can download binaries from [latest release
page](https://github.com/ismaelgv/rnr/releases), choose the compressed
file corresponding to your platform. These compressed files contain the
executable and other additional content such as completion files (*Bash*, *Zsh*,
*fish* and *PowerShell*).

### Arch Linux
A package is available in the AUR
([`rnr`](https://aur.archlinux.org/packages/rnr/)) to install latest version of
*RnR* on Arch Linux.

### Homebrew
You can use [Homebrew package manager](https://brew.sh) to install this tool in macOS or Linux systems.
```sh
brew install rnr
```

## From Source
*RnR* is written in Rust. You can build it from source using Cargo.

### From git repository
```sh
git clone https://github.com/ismaelgv/rnr .
cargo install
```
### From Crates.io
```sh
cargo install rnr
```
# Usage

## Options
Check a detailed description of the application usage and all its options using: `rnr help`.

## Default behavior
* Checks all operations to avoid overwriting existing files.
* *Dry-run* by default.
* Only **UTF-8 valid** input arguments and filenames.
* Works on files and symlinks (ignores directories).
* Accepts multiple files as arguments.
* Accepts a **regex** to generate matches. These expressions have same
  limitations of `regex` crate. You can check regex syntax
  [here](https://docs.rs/regex/#syntax). It supports numbered and named *capture
  groups*.
* If max depth is not provided to recursive mode, it is assumed *infinite*.
* Does not generate backups.
* Output is *colored* (only ANSI terminals).
* Ignore hidden files and directories.
* Dump all operations into a file in force mode, and also by default in
  `editor` dry-run mode (to avoid losing edit history). You can disable this
  with `--no-dump`. This dump file can be used to undo operations from
  `from-file` subcommand.
* Number of replacements set to one.
* Automatically creates missing parent directories when the target path requires
  them (e.g. renaming `file.txt` to `new_dir/sub/file.txt` creates
  `new_dir/sub/` on the fly).

## Examples
* [Rename a list of files](#rename-a-list-of-files)
    * [Include directories](#include-directories)
    * [Multiple replacements](#multiple-replacements)
    * [Rename into a new subdirectory](#rename-into-a-new-subdirectory)
    * [Combination with other UNIX tools](#combination-with-other-unix-tools)
* [Recursive rename](#recursive-rename)
    * [Recursive rename with max directory depth](#recursive-rename-with-max-directory-depth)
    * [Recursive rename including directories and hidden files](#recursive-rename-including-directories-and-hidden-files)
* [Interactive editor rename](#interactive-editor-rename)
    * [Rename files in the editor](#rename-files-in-the-editor)
    * [Delete files in the editor](#delete-files-in-the-editor)
    * [Choose your editor](#choose-your-editor)
    * [Recursive editor rename](#recursive-editor-rename)
* [Undo/redo operations using dump file](#undoredo-operations-using-dump-file)
* [Create backup files before renaming](#create-backup-files-before-renaming)
* [Convert UTF-8 file names to ASCII](#convert-utf-8-file-names-to-ascii)
* [Advanced regex examples](#advanced-regex-examples)
    * [Replace extensions](#replace-extensions)
    * [Replace numbers](#replace-numbers)
    * [Capture groups](#capture-groups)
    * [Capture several named groups and swap them](#capture-several-named-groups-and-swap-them)
    * [Capture several groups and apply a transformation](#capture-several-groups-and-apply-a-transformation)


__NOTE:__ If the regular expression `EXPRESSION` contains `-` as initial
character, the application with parse it as an argument. You need to use `--`
after flags and before the arguments, for example `rnr regex -f -- '-foo' '-bar'
[...]`.

__WINDOWS NOTE:__ In the examples that use `*`, you need to expand the wildcard
in PowerShell, for example: `rnr regex a b (Get-Item ./*)`. This is not supported in
`cmd.exe`.

### Rename a list of files
You can pass a list of files to be renamed as arguments:
```sh
rnr regex -f file renamed ./file-01.txt ./one/file-02.txt ./one/file-03.txt
```
*Original tree*
```
.
├── file-01.txt
├── file-02.txt
├── file-03.txt
└── one
    ├── file-01.txt
    ├── file-02.txt
    └── file-03.txt
```
*Renamed tree*
```
.
├── renamed-01.txt
├── file-02.txt
├── file-03.txt
└── one
    ├── file-01.txt
    ├── renamed-02.txt
    └── renamed-03.txt
```

#### Include directories
Directories are ignored by default but you can also include them to be renamed using the option `-D`.
```sh
rnr regex -f -D foo bar ./*
```
*Original tree*
```
.
├── foo
│   └── foo.txt
└── foo.txt
```
*Renamed tree*
```
.
├── bar
│   └── foo.txt
└── bar.txt
```

#### Multiple replacements
The replacement limit is set to 1 by default, but you can configure this limit
to replace multiple non-overlapping matches. All matches will be replaced if
this option is set to 0.

```sh
rnr regex -f -l 0 o u ./*
```
*Original tree*
```
.
├── foo.txt
├── foofoo.txt
├── foofoofoo.txt
└── foofoofoofoo.txt
```
*Renamed tree*
```
.
├── fuu.txt
├── fuufuu.txt
├── fuufuufuu.txt
└── fuufuufuufuu.txt
```

#### Rename into a new subdirectory
When the target path contains directories that do not exist yet, `rnr`
creates them automatically — no manual `mkdir` needed.

```sh
rnr regex -f '(.*)' 'archive/2024/${1}' ./*
```

*Original tree*
```
.
├── report-01.txt
├── report-02.txt
└── report-03.txt
```
*Renamed tree*
```
.
└── archive
    └── 2024
        ├── report-01.txt
        ├── report-02.txt
        └── report-03.txt
```

The same applies to any depth of nesting — `rnr` will create the full chain of
missing parent directories.

#### Combination with other UNIX tools
You can combine `rnr` with other UNIX tools using pipes to pass arguments.

##### Find files older than 1 day and rename them
```sh
find . -type f +mtime 1 | xargs rnr regex -f file renamed
```

##### Read list of files from a file
```sh
cat file_list.txt | xargs rnr regex -f file rename
```

`file_list.txt` content:
```
file-01.txt
one/file-02.txt
one/file-03.txt
```

### Recursive rename
If recursive (`-r`) option is passed, `rnr` will look for al files in the path recursively without depth limit.
```sh
rnr regex -f -r file renamed ./
```
*Original tree*
```
.
├── file-01.txt
├── file-02.txt
├── file-03.txt
└── one
    ├── file-01.txt
    ├── file-02.txt
    ├── file-03.txt
    └── two
        ├── file-01.txt
        ├── file-02.txt
        ├── file-03.txt
        └── three
            ├── file-01.txt
            ├── file-02.txt
            └── file-03.txt
```
*Renamed tree*
```
.
├── renamed-01.txt
├── renamed-02.txt
├── renamed-03.txt
└── one
    ├── renamed-01.txt
    ├── renamed-02.txt
    ├── renamed-03.txt
    └── two
        ├── renamed-01.txt
        ├── renamed-02.txt
        ├── renamed-03.txt
        └── three
            ├── renamed-01.txt
            ├── renamed-02.txt
            └── renamed-03.txt
```
#### Recursive rename with max directory depth
Similarly, you can set a maximum directory depth in combination with recursive operations.
```sh
rnr regex -f -r -d 2 file renamed ./
```
*Original tree*
```
.
├── file-01.txt
├── file-02.txt
├── file-03.txt
└── one
    ├── file-01.txt
    ├── file-02.txt
    ├── file-03.txt
    └── two
        ├── file-01.txt
        ├── file-02.txt
        ├── file-03.txt
        └── three
            ├── file-01.txt
            ├── file-02.txt
            └── file-03.txt
```
*Renamed tree*
```
.
├── renamed-01.txt
├── renamed-02.txt
├── renamed-03.txt
└── one
    ├── renamed-01.txt
    ├── renamed-02.txt
    ├── renamed-03.txt
    └── two
        ├── file-01.txt
        ├── file-02.txt
        ├── file-03.txt
        └── three
            ├── file-01.txt
            ├── file-02.txt
            └── file-03.txt
```

#### Recursive rename including directories and hidden files
`rnr` ignore hidden files by default to speed up the operations and avoid problems with some particular directories like `.git/` or `.local/`. You can include hidden files passing `-x` option. Also, you can use include directories `-D` option with `-r` too.
```sh
rnr regex -f -r -D -x foo bar ./
```
*Original tree*
```
.
├── .foo_hidden_file.txt
├── foo.txt
├── foo
│   ├── foo.txt
│   └── foo
│       └── foo.txt
└── .foo_hidden_dir
    └── foo.txt
```
*Renamed tree*
```
.
├── .bar_hidden_file.txt
├── bar.txt
├── bar
│   ├── bar.txt
│   └── bar
│       └── bar.txt
└── .bar_hidden_dir
    └── bar.txt
```

### Interactive editor rename
The `editor` subcommand opens a list of paths in your preferred text editor.
After you save and exit, `rnr` applies any renames (and optionally deletions)
you made.

**Temp file:** `rnr` writes the path list to a temporary file in the OS temp
directory (e.g. `/tmp/rnr-editor-XXXXXX.txt` on Linux/macOS) and passes that
path to the editor. The file is automatically removed when `rnr` finishes.

**Recursive mode:** When `-r` is used, `rnr` first collects **all** matching
paths from every subdirectory (applying `--max-depth` and `--hidden` filters as
usual), then opens the editor **once** with the complete list.

**Editor selection:** `--editor <CMD>` → `$VISUAL` → `$EDITOR` → `vi`

#### Rename files in the editor
By default (no `--delete`) every line in the editor corresponds positionally to
one source path.  Edit a path to rename that file.  The line count **must not
change** — removing a line is an error.

```sh
rnr editor -f ./*
```

The editor will open with content like:
```
/path/to/file-01.txt
/path/to/file-02.txt
/path/to/file-03.txt
```

Edit and save to, for example:
```
/path/to/renamed-01.txt
/path/to/file-02.txt
/path/to/file-03.txt
```

*Original tree*
```
.
├── file-01.txt
├── file-02.txt
└── file-03.txt
```
*Renamed tree*
```
.
├── renamed-01.txt
├── file-02.txt
└── file-03.txt
```

#### Delete files in the editor
Pass `--delete` to enable file deletion.  Each line is then prefixed with a
1-based index and a tab character (`INDEX<TAB>PATH`).  Remove a line entirely
to delete that file; change the path after the tab to rename it.

```sh
rnr editor -f --delete ./*
```

The editor will open with content like:
```
1	/path/to/file-01.txt
2	/path/to/file-02.txt
3	/path/to/file-03.txt
```

Remove line 2 and save:
```
1	/path/to/file-01.txt
3	/path/to/file-03.txt
```

*Result:* `file-02.txt` is deleted; `file-01.txt` and `file-03.txt` are
unchanged.

#### Choose your editor
Use `--editor` to override the default editor selection:
```sh
rnr editor -f --editor nano ./*
rnr editor -f --editor "code --wait" ./*
```

If `--editor` is not given, `rnr` checks `$VISUAL`, then `$EDITOR`, and falls
back to `vi`.

#### Recursive editor rename
Combine `-r` with the `editor` subcommand to collect all files from a directory
tree first, then edit the full list in one session:

```sh
rnr editor -f -r ./
```

The editor opens with **all** paths found in the tree.  The same rename/delete
rules apply as in the non-recursive case.

```sh
rnr editor -f -r -d 2 --delete ./
```

This limits collection to 2 directory levels deep and enables deletion.

### Undo/redo operations using dump file
When you perform a renaming operation, `rnr` will create by default a dump file in the current directory you executed the command. This file can be used to easily revert the operations using `from-file` and `-u` option.

*Rename operation*
```sh
rnr regex -f foo bar ./*
```
*Undo previous operation*
```sh
rnr from-file -f -u rnr-[timestamp].json
```

If you want to redo the operation just pass the dump file without any additional argument:
```sh
rnr from-file -f rnr-[timestamp].json

```

### Create backup files before renaming
`rnr` can create backup files before renaming for any operation passing `-b` option. The backup files names are ensured to be unique and won't be overwritten if another backup is created. If you are working with many large files, take into account that files will be duplicated.

```sh
rnr regex -f -b file renamed ./*
```

*Original tree*
```
.
├── file-01.txt
├── file-02.txt
└── file-03.txt
```
*Renamed tree*
```
.
├── file-01.txt.bk
├── file-02.txt.bk
├── file-03.txt.bk
├── renamed-01.txt
├── renamed-02.txt
└── renamed-03.txt
```

### Convert UTF-8 file names to ASCII
`rnr`can convert UTF-8 file names to their ASCII representation. This feature uses
[AnyAscii library](https://github.com/anyascii/anyascii) to perform the
transliteration. To avoid conflicts with paths, the characters that would be translated
to `/` are changed to `_` instead.

You can run:
```sh
rnr to-ascii ./*
```
Or:
```sh
rnr to-ascii -r .
```

*Original tree*
```
.
├── fïlé-01.txt
├── FïĹÊ-02.txt
└── file-03.txt
```
*Renamed tree*
```
.
├── file-01.txt
├── FILE-02.txt
└── file-03.txt
```

### Advanced regex examples
More info about regex used [in the `regex` package](https://docs.rs/regex).
#### Replace extensions
```
rnr regex -f '\..*$' '.txt' ./*
```
*Original tree*
```
.
├── file-01.ext1
├── file-02.ext2
└── file-03.ext3
```
*Renamed tree*
```
.
├── file-01.txt
├── file-02.txt
└── file-03.txt
```

#### Replace numbers
```
rnr regex -f '\d' '1' ./*
```
*Original tree*
```
.
├── file-01.txt
├── file-02.txt
└── file-03.txt
```
*Renamed tree*
```
.
├── file-11.txt
├── file-12.txt
└── file-13.txt
```
#### Capture groups
1. Capture three unnamed groups [`name(1)-number(2).extension(3)`].
2. Swap group 1 (name) and group 2 (number).
```sh
rnr regex -f '(\w+)-(\d+).(\w+)' '${2}-${1}.${3}' ./*
```
*Original tree*
```
.
├── file-01.txt
├── file-02.txt
└── file-03.txt
```
*Renamed tree*
```
.
├── 01-file.txt
├── 02-file.txt
└── 03-file.txt
```
__SHELL NOTE:__ In shells like Bash and zsh, make sure to wrap the `REPLACEMENT`
pattern in single quotes. Otherwise, capture group indices will be replaced by
expanded shell variables.

#### Capture several named groups and swap them
1. Capture two digits as `number`.
2. Capture extension as `ext`.
3. Swap groups.
```sh
rnr regex -f '(?P<number>\d{2})\.(?P<ext>\w{3})' '${ext}.${number}' ./*
```
*Original tree*
```
.
├── file-01.txt
├── file-02.txt
└── file-03.txt
```
*Renamed tree*
```
.
├── file-txt.01
├── file-txt.02
└── file-txt.03
```

#### Capture several groups and apply a transformation

1. Capture three unnamed groups [`name(1)-number(2).extension(3)`].
2. Swap group 1 (name) and group 2 (number).
3. Transform replacement to uppercase.

```sh
rnr regex -f -t upper '(\w+)-(\d+)' '${2}-${1}' ./*
```
*Original tree*
```
.
├── file-01.txt
├── file-02.txt
└── file-03.txt
```
*Renamed tree*
```
.
├── 01-FILE.txt
├── 02-FILE.txt
└── 03-FILE.txt
```
