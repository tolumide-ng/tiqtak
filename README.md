## Checkers 🦅
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
- [ ] Examples on how this works


### Credit:
1. [Monte Carlo Tree Search – beginners guide](https://int8.io/monte-carlo-tree-search-beginners-guide/)
2. [Monte-Carlo Tree Search (MCTS)](https://gibberblot.github.io/rl-notes/single-agent/mcts.html)