# cargo-snatch
Cargo snatch is a utility to reserve crate names in a community friendly manner.

If you're anything like me you probably had an idea for a crate or a bigger project. You kept thinking about it for some time and came up with various features and technical details - and perhaps most importantly - a name.

So you went ahead and reserved the crate name on crates.io using the standard, slightly annoying process that looked something like:
```
> mkdir my-crate-name
> cd my-crate-name
> cargo init
> cargo publish
error: 2 files in the working directory contain changes that were not yet committed into git:

Cargo.toml
src/main.rs
> # Oh, ok
> cargo publish --allow-dirty
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error (status 400 Bad Request): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for more information on configuring these fields
> # ugh, fine
> $EDITOR Cargo.toml # and fill out the necessary fields
> cargo publish --allow-dirty
```

and hooray! You've succesfully reserved a name on crates.io.

But then life gets in the way. Maybe you get busy with a different project. Maybe you start coding, encounter a difficult issue and get discouraged. Maybe you simply get bored and start fiddling with something else.

There are many valid reasons for anybody to abandon a reserved crate on crates.io. But now, you've created a problem.

A few months or years go by and someone else has an idea simmilar to yours, they start thinking of a name for their toy project and come up the same one too! But their dismay that crate already exists. Worse than that, it's just an empty package with no contact info to the owner. If they're lucky, maybe you have some contact info on your associated GitHub profile and they reach out to you to see if you're willing to release the name.

But there are still a few hurdles to overcome - I know that if I were in their place I would have never sent an email in the first place. The value of a presumably cool name is not enough for me to go against my social anxiety like that.

And even if they do manage to contact you in some way - will you respond? How easy it would be for the email to get lost in your Inbox full of very important things. And we're all busy people, I wouldn't judge you if you've read the email, put it aside for later, except the later would never come.

This tool solves 2 issues. First it makes it super simple to reserve a name with just a single command. Second it establishes a way for those interested in your crate name to request a release with as few barriers as possible. 

This barrier removal is achieved with a "snatches" repository. It's a mostly empty repository associated with your GH account where anyone can create an issue (from a template, very easy and not social anxiety inducing) to request the release. This repo is then linked in all snatched crates.

I've designed this experience to be as seamless as possible - this utility will interactively guide you through the process of creating this repository and will automatically attach it to your snatched crates.

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

