## Checkers 🦅
[![crates.io](https://img.shields.io/badge/tiqtak-hunter%20green
)](https://crates.io/crates/tiqtak)
[![npm version](https://img.shields.io/badge/tiqtak-hunter%20green
)](https://www.npmjs.com/package/tiqtak)

Simple Checkers engine, that handles all the `checkers` game logic for you, so you don't have to care.
The current implementation uses [MCTS(Monte-Carlo Tree Search)](https://gibberblot.github.io/rl-notes/single-agent/mcts.html), in the future, this program might be extended to allow users provide their desired search heuristic algorithm

Available on [npm](https://www.npmjs.com/package/tiqtak) and [crates.io](https://www.npmjs.com/package/tiqtak)

### To run flamegraph:
1. Simply install flamegraph with cargo or check [FlamegraphRs](https://github.com/flamegraph-rs/flamegraph?tab=readme-ov-file#macos)


### How to Run this library:
1. Simply clone this repository and cd into it
2. To use the rust version directly, run: `cargo run` or `cargo watch` depending on the `mode`
3. To generate the wasm build for js target, run: 
    a. `wasm-pack build --target bundler` for npm targets
    b. `wasm-pack build --target web` if you're trying to reference the build directly locally
    nb: you'd find the `build` in the pkg folder (root folder)



### Todo:
- [ ] More robust documentation
- [ ] More tests
- [ ] Migrate from using u64 to u32 for the board
    - [ ] Accepts moves(action) that use u64 or u32 (always allow any of them)
    - [ ] Internally transform the u64 moves format to u32 before applying it to the board. <br />
        >> u64 moves (action) would be more user friendly, why?? this makes the moves easily readable and understandable by humans using translating the board moves
    - [ ] Action can be in two different formats: 1. u64 format or u32 format, this is indicated in the struct field or the 15th bit (set bit -> u64 format, unset bit -> u32 format)
    ```
        /// from lsb to msb (i.e msb <- lsb)
        /// first 6 bits - src
        /// next 6 bits - tgt
        /// next 1 bit - captured
        /// next 1 bit - prompted
        /// next 1 bit - whether this action is in u32 or u64 format for the squares
        ///     if its u64 bit should be set to 1
        ///     if its u32 bit should be set to 0
        /// last 1 bit - free for now
    ```
- [ ] Fix releasing packages issue on github with (cross??)
- [ ] Examples on how this works
- [ ] Explain the architecture of this library
    - [ ] The board representation
    - [ ] MCTS approach?


### Credit:
1. [Monte Carlo Tree Search – beginners guide](https://int8.io/monte-carlo-tree-search-beginners-guide/)
2. [Monte-Carlo Tree Search (MCTS)](https://gibberblot.github.io/rl-notes/single-agent/mcts.html)
