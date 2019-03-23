# Dicey Dice
A quick-n-dirty implementation of a hexagonal based board game in the category of risk using six sided dice as the playing pieces.

## Related Project
This project is more the engine for the front-end being [wasm-dicey](https://github.com/kvsari/wasm-dicey).

## Building
Assuming [rust](https://www.rust-lang.org/) v1.33.0 installed; in project root run,
```console
cargo build
```

## Testing
Like building; in project root run,
```console
cargo test
```

## Running
This project is not meant to be run as stand-alone although it can be. You'll need to modify the `main` function to start a game with the desired parameters. Output is to console with a simple menu. This is not meant to be easy to use but to verify if the game logic works. To run,
```console
cargo run
```

## Learning Process
* Using a `HashMap` as the backing data structure for the movement tree is premature optimization. This was inspired by memoization in functional languages such as lisp and haskell. I should have started with a bog standard tree.
* Next time, use `Rc` or `Arc` when storing the `Board`s. This aught to reduce memory duplication in the `Consequence` enums.
* Investigate using a macro to generate the `Grid` so it can be a fixed size array rather than a vector. This would further allow `Copy` types in the codebase.
* The interior mutability for doing the scoring is a hack. It breaks the immutability of the game tree.
* The `Players` and `Player` structs are a mess. Although the immutability is a good thing, the rest of the design is terrible.
* Invest some effort in creating a proper game progression log which can be fetched by library consumers. The current `State` struct is very difficult to turn into string output in `wasm-dicey` for printing a battle log.
* Theres some serious tree-shaking/pruning that can be done.
* The tree shouldn't need to be dropped each turn and a new one generated.
* Make the game ruleset configurable through some kind of builder pattern or something.
* Data exposed to AI allows for some very rudimentary 'personalities' in AI's. Try implementing them next time.

And likely heaps more. This project was a good learning experience.
