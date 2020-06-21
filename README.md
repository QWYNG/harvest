# harvest 
![Rust](https://github.com/QWYNG/harvest/workflows/Rust/badge.svg?branch=master)

CLI application that does your git stashes finds
```
USAGE:
    harvest <pattern>

ARGS:
    <pattern>    pattern to search

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```
It looks up the diffs of all the git stashes and the current branch with pattern and outputs the stash as a stdout in the following format
```
harvest fn
stash@{0}: WIP on master: beb5221 rm tests module
 src/bm.rs | 2 ++
 1 file changed, 2 insertions(+)

stash@{1}: WIP on master: beb5221 rm tests module
 src/lib.rs | 1 +
 1 file changed, 1 insertion(+)
```
