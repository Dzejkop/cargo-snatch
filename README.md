# cargo-snatch
Cargo snatch is a utility to reserve crate names in a community friendly manner.

It simplifies the reservation process and also ensures that there's a communication line between the crate owner and those interested in the release of it.

## Installation
This utility depends on [`gh`](https://cli.github.com/), [`git`](https://git-scm.com/) and [`cargo`](https://www.rust-lang.org/tools/install).

```
> cargo install --locked cargo-snatch
```

## Usage
```
# If running for the first time this command will guide you through the setup process
# Afterwards this command is fire-and-forget
> cargo snatch crate-name
```
