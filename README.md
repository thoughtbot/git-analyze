# git-analyze

git-analyze is a CLI tool built to help uncover how contributors commit to a
git repository.

## Features

* See off-hours contribution percentages of overall commits
* Review general stats about the git repository, including total committer and recent committer counts
* Generate [mailmap] contents to improve git-analyze reporting quality


```
git-analyze 0.1.0
Analyze git repositories

USAGE:
    git-analyze [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    generate-mailmap    Print a mailmap file to STDOUT based on repository contributors
    help                Prints this message or the help of the given subcommand(s)
    off-hours           View quarterly percentage of commits during off-hours
    overview            View basic statistics about the git repository
    team-history        View a quarterly breakdown of contributors and contributor changes
```

[mailmap]: https://git-scm.com/docs/gitmailmap

## Example

```
~/s/rails> git analyze
Total commits: 83602
First commit: 2004-11-24T01:04:44+00:00
Unique committers: 5488
Recent committers: 252
Top 10 committers:
* Rafael Mendonça França 5277
* Aaron Patterson 5207
* David Heinemeier Hansson 4504
* Ryuta Kamizono 3761
* Jeremy Kemper 3520
* José Valim 2569
* Xavier Noria 2538
* Rafael França 2211
* Yves Senn 1638
* Carlos Antonio da Silva 1531
```

## Installation

Precompiled binaries are available for [the latest release].

[the latest release]: https://github.com/thoughtbot/git-analyze/releases/latest

### Via Rust (Nightly)

```sh
git clone git@github.com:thoughtbot/git-analyze.git
cd git-analyze
cargo install --path .
```

## License

Copyright 2021 thoughtbot, inc. See the [LICENSE](LICENSE).
