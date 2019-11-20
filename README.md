## 2048-rs
[![Travis Build Status](https://travis-ci.org/adrienball/2048-rs.svg?branch=master)](https://travis-ci.org/adrienball/2048-rs)

This is a Rust implementation of the famous [2048 game](https://en.wikipedia.org/wiki/2048_\(video_game\)) which runs in the terminal.

In addition to the standard user inputs, an AI can be used to perform the moves. This AI leverages the [expectiminimax](https://en.wikipedia.org/wiki/Expectiminimax) algorithm to recommend the best next move at each step.

<img align="center" src="https://github.com/adrienball/2048-rs/blob/master/.img/screenshot.png?raw=true" alt="Screenshot"/>

### Installation from source

```bash
> git clone https://github.com/adrienball/2048-rs.git
> cd 2048-rs
> cargo run --release
```

Some parameters, like the probability of drawing a 4 tile, can be changed by passing additional parameters to the `cargo run` command:

```bash
> cargo run --release -- --proba-4 0.5
```

To get the list of available options, simply run:

```bash
> cargo run --release -- --help
```
