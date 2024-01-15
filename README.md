# rustGit
A git clone written in rust following the Python tutorial 'Write yourself a Git!' https://wyag.thb.lt/#intro 

Note: the above tutorial was designed to work interoperably with git, so that all internal file formats matched exactly how git stores its data. I did NOT follow that convention so internal files produced by my git clone are not interoperable with the standard git tooling, instead I use serde for (de)serialisation etc. as my goal was to produce my own clone of git rather than a program that is interoperable with git

# Usage
From cargo you can pass arguments, e.g.
```bash
cargo run -- init "C:\\Users\\benja\\Documents\\code\\my_git_test"
```

For ease of use if using intellij to run/test the best option is to create run configs for each of the files in the src/bin directory.
From here you can then *set the working dir of the run config to be the git dir you want to work in* to help test.