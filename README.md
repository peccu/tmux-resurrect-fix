# fix tmux-resurrect

Update the `last` symbolic link in `~/.tmux/resurrect` from blank (3 lines) to latest not blank (greater than 3 lines).

## USAGE

```bash
## remove current wrong symlink
$ rm ~/.tmux/resurrect/last
## create correct symlink
$ time tmux-resurrect-fix ~/.tmux/resurrect/ last
## restore with it
$ ~/.tmux/plugins/tmux-resurrect/scripts/restore.sh
```

## INSTALL

- without docker (local cargo)

```bash
$ cargo build --release
$ sudo mv target/release/tmux-resurrect-fix /usr/local/bin
```

- use docker

```bash
$ docker run --rm -it --entrypoint bash --name rust -v $PWD:/app -w /app rust:slim -c 'cargo build --release'
$ sudo mv target/release/tmux-resurrect-fix /usr/local/bin
```
