# Drawing pictures and benchmarks for xDDs

This project draws some pictures of some objects enumerated by my xdd Rust library.
It also has some benchmarking.

# To compile

This project is written in the Rust language. If you do not have a rust compiler,
[install it](https://www.rust-lang.org/tools/install).

This project relies on two other packages which need to be cloned in the 
same way as this git repository.

```bash
git clone https://github.com/AndrewConway/chessboard_tiling_pictures.git
git clone https://github.com/AndrewConway/xdd.git
git clone https://github.com/AndrewConway/lattice-picture.git
cd chessboard_tiling_pictures
cargo build --release
```

# Running

The following instructions assume you are in the `chessboard_tiling_pictures` directory
created above.

To make pictures, run
```bash
cargo run --release --example draw_coverings
```

To do benchmarks, run
```bash
cargo bench
```

Actually, this will only run benchmarks on the directed animals. To run benchmarks
on the chessboard coverings, edit `Cargo.toml` in this directory replacing
`name = "directed_animals"` by `name = "chessboard_coverings"` and rerun `cargo bench`. 
I expect there is an easier way; if you know, please tell me.

## License

Copyright 2022-2025 Andrew Conway.

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

