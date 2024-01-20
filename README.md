# timer-rs

Is a command line based module intended to be used in status bars.

## Basic Usage

1. Start timer-rs in a terminal
2. run "echo '[seconds]' /tmp/timer-rs_input" to second timer-rs a signal
3. timer-rs will display a bar on each newline which concludes after [seconds]

This inferface is likely to change.

## Use as an [i3status-rust](https://github.com/greshake/i3status-rust) block

```toml
[[block]]
block = "custom"
format = " $text "
command = " timer-rs "
persistent = true
[[block.click]]
button = "left"
cmd = "echo '300' > /tmp/timer-rs_input"
[[block.click]]
button = "right"
cmd = "echo '900' > /tmp/timer-rs_input"
```

Timer-rs can't output JSON currently. So icons need to be placed directly into the format value.
Like so:

```toml
format = " <icon> $text "
```

I use this nerdfont icon `󱎫`.

## Installation

Build from source using cargo.

```bash
$ git clone --branch main https://github.com/chellipse/timer-rs
$ cd timer-rs
$ cargo install --path .
```

Make sure ~/.cargo/bin is in your PATH.
