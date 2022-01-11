# vixargs ~ `vi`sual `xargs` executes one command per window pane

Make the invisible palpable

```shell
cargo install --git https://github.com/fenollp/vixargs
# sudo apt install tmux

# Say the file ./commands has one shell command per line
vixargs -a commands
```

[![asciicast](https://asciinema.org/a/461227.svg)](https://asciinema.org/a/461227)

## to do
* work around reading from STDIN while still getting `tmux attach` to work
* replace `tmux` with a Rust solution [maybe this?](https://github.com/zellij-org/zellij)
* https://explainshell.com/explain?cmd=xargs
