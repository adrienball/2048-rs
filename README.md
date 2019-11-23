# 2048-rs
[![Travis Build Status](https://travis-ci.org/adrienball/2048-rs.svg?branch=master)](https://travis-ci.org/adrienball/2048-rs)

This is a Rust implementation of the famous [2048 game](https://en.wikipedia.org/wiki/2048_\(video_game\)) which runs in the terminal.

In addition to the standard user inputs, an AI can be used to perform the moves. This AI leverages the [expectiminimax](https://en.wikipedia.org/wiki/Expectiminimax) algorithm to recommend the best next move at each step.

<p align="left">
    <img src="./.img/screenshot.png?raw=true" alt="Game screenshot" width="230">
</p>

## Installation from source

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

# License

## Apache 2.0/MIT

All original work licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
