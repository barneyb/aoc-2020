# Advent of Code 2020

My Advent of Code 2020 solutions/solvers in Rust. It's the usual (assuming you
have a Rust dev env, https://rustup.rs or otherwise):

    cargo run

If you want to see a specific day's solver in action, go find when its branch
merged into master and check out that revision. Most of the code is in durable
modules, but the actual executable only solves one day: today. The first few
days had two branches: one for the solution and one for modularization. I'm
learning too!

If you want to solve _your_ input, after checking out the right commit, replace
the `input.txt` file in the root with your input and do `cargo run` again. But
don't do that; the point of AoC isn't the stars, it's the pleasure of discovery.
