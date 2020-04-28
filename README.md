# 2048-rs
[![Travis Build Status](https://travis-ci.org/adrienball/2048-rs.svg?branch=master)](https://travis-ci.org/adrienball/2048-rs)

This is a Rust implementation of the famous [2048 game](https://en.wikipedia.org/wiki/2048_\(video_game\)) 
which runs in the terminal.

In addition to the standard user inputs, an AI can be used to perform the moves. This AI 
leverages the [expectiminimax](https://en.wikipedia.org/wiki/Expectiminimax) algorithm to 
recommend the best next move at each step.

<p align="left">
    <img src="./.img/screenshot.png?raw=true" alt="Game screenshot" width="230">
</p>

## Statistics

Here are the statistics of the AI with its default parameters:

| max tile reached | frequency |
|-----------------:|----------:|
|             2048 |     100 % |
|             4096 |     100 % |
|             8192 |      96 % |
|            16384 |      70 % |
|            32768 |      10 % |

In particular, these statistics correspond to a minimum branch probability of `0.001`. 
Decreasing this value would lead to better performance, as more branches would be explored, but this would also take more time.

Some other hardcoded parameters can be tweaked in order to further improve the algorithm. 
I have not performed an exhaustive grid search, thus the parameter set is probably sub-optimal.

## Installation

`Rust` and `cargo` must be installed to run this game:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### With Cargo

```bash
cargo install play-2048
```

Then, in order to play:

```bash
play-2048
```

### From source

```bash
> git clone https://github.com/adrienball/2048-rs.git
> cd 2048-rs
> cargo run --release
```

## Usage

You can change the probability of drawing a 4 tile:

```bash
> play-2048 --proba-4 0.5
```

Or adjust the minimum branch probability of the expectiminimax search:

```bash
> play-2048 --min-branch-proba 0.0001
```

To get the list of available options, simply run:

```bash
> play-2048 --help
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
