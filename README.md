# Rust Toy Git Implementation
A git clone written in rust following the Python tutorial 'Write yourself a Git!' https://wyag.thb.lt/#intro 

Note: the above tutorial was designed to work interoperably with git, so that all internal file formats matched exactly how git stores its data. I did NOT follow that convention so internal files produced by my git clone are not interoperable with the standard git tooling, instead I use serde for (de)serialisation etc. as my goal was to produce my own clone of git rather than a program that is interoperable with git
# Understanding Git
The below is a very very brief summary of the excellent information from: https://wyag.thb.lt/

Git is made up of a few core under the hood components:
- custom storage of files via their sha1 hash (where the hash is split into the first two chars and remainder, the first two chars are used as the folder name and the remainder is used as the file name)
- references which are objects which either point directly to a hash or indirectly to another reference
- trees which are made up of leaves each ultimately pointing to the hash of an object. The tree itself then has its own hash which we can reference. Hence a tree is the core way we group versions of a file. When you do a git checkout you can checkout a tree directly or checkout a commit which ultimately will point to a tree
- indexes ...
- commits ...
- tags ...
- branches ...
# Usage
From cargo you can pass arguments, e.g.
```bash
cargo run -- init "C:\\Users\\benja\\Documents\\code\\my_git_test"
```

For ease of use if using intellij to run/test the best option is to create run configs for each of the files in the src/bin directory.
From here you can then *set the working dir of the run config to be the git dir you want to work in* to help test.
# TODO
- add CICD/rust test runs and a status badge https://kerkour.com/rust-github-actions-ci-cd
- improve error handling
- simplify GitObject nested type