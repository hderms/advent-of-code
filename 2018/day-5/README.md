# Day 5
`cargo build --release`
`./target/release/day-5`

Runs the algorithm 10k times so we can get a read on how long each solution takes. Appears to be about 170 microseconds on my machine under WLS2 (AMD Ryzen 9 3900x).

Got interested in trying to improve the performance of this AoC challenge based on the following blogpost that compared Rust performance to Javascript: [post](https://cesarvr.io/post/rust-performance/). The algorithm presented in the post did a lot of heap allocations in a tight loop so I decided to see what performance I could wring out as a Rust novice. Seems to be roughly 20 times faster. 