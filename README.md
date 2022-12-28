Advent of code 2022 solutions
==

This is the code I wrote for advent of code 2022. I got all the stars, although I am
not confident that every solution is correct for all inputs (looking at you, day 17).

This was a project to learn more about Rust for me. This was a fairly intense period
where lots of learning was done, and I am fairly confident that I am writing better
Rust code on day 25 than on day 5. The solutions don't focus on being brief or fast,
I've instead tried to explore parts of the language that I think are good to learn
about, such as error handling and data types.

Usage
--

If you wish, you can download all the inputs to your local machine by running: `cargo run --bin get_available_inputs`.
It expects you to give it your advent of code session cookie as input, `cat $SESSION_COOKIE | cargo run --bin get_available_inputs`.

This creates a file under `input/day_nn/input`. To run, use `cargo run --release --bin day_nn < input/day_nn/input`. 