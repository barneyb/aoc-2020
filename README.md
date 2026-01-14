# Advent of Code 2020

Index of Advent of Code solutions: https://github.com/barneyb/aoc2017

My Advent of Code 2020 solutions/solvers in Rust. It's the usual:

    # curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    cargo test
    cargo run --release

If you want to see a specific day's solver in action, go find its branch and
check it out. Some code is in durable modules, but the actual executable only
solves one day: today. Don't look too closely at the branch structure; I let go
nuts on purpose to help learn how to articulate why keeping it hygienic matters. 

If you want to solve _your_ input, after checking out the right commit, replace
the `input.txt` file in the root with your input and do `cargo run` again. But
don't do that; the point of AoC isn't the stars, it's the pleasure of discovery.

## Java Playground

There are also a couple Java tidbits in there, where I needed to think through
some stuff. Having to fight the borrow checker and mutability (and to a lesser
extent, lifetimes) interfered with my ability to think through the "business"
problem, since they're still pretty unfamiliar. Yes, I use Java as a scripting
language for exploration. Shush.
