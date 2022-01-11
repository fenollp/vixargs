# vixargs ~ `vi`sual `xargs` executes one command per window pane

Make the invisible palpable

```shell
cargo install --git https://github.com/fenollp/vixargs
# sudo apt install tmux

# Say the file ./commands has one shell command per line
vixargs -a commands
```

## to do
* work around reading from STDIN while still getting `tmux attach` to work
* replace `tmux` with a Rust solution [maybe this?](https://github.com/zellij-org/zellij)
* https://explainshell.com/explain?cmd=xargs
