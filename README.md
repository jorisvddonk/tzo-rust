# Tzo-Rust 💎🦀

This is an implementation of the [Tzo Virtual Stack Machine](https://github.com/jorisvddonk/tzo) in [Rust](https://www.rust-lang.org/).

## Here be dragons!

As this is my first real Rust project, there might we a bit of weirdness in here. It's hard to take the JavaScript development mindset out of an engineer ;) - Tips and feedback are appreciated via issues or PRs :). In particular, adding Tzo foreign functions that require external state is a little cumbersome right now (see [questmark-rust](https://github.com/jorisvddonk/questmark-rust)), which I hope to be able to address.

Furthermore, the relationship between Tzo-Rust and the TypeScript implementation of Tzo is a little bit ill-defined at the moment. Ideally, I'd like to refactor Tzo as a project into a specification backed by multiple implementations, but work on that has yet to commence.

Due to the above, there's no crate for Tzo available yet for public consumption. This will be available as soon as soon as things stabilize a bit more!
