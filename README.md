# Crunchy VDF
Deep fried keyvalues `?:P`

A Rust keyvalues parser.
competition time bay-be

## TODO:
Compare it to fastkv on my machine so I know how much more performance can be squeezed out.
Add feature gates for
* Experimental multithreading using rayon (top level)
* Handling escape sequences (Allocation Heaviness)
* Evaluating Conditionals (currently they're skipped)
* `#include` macro
* `#base` macro ? It'll be a pain, but whatever..

## Rough Benchmarks as I progress

initial P.o.C (pest): 1.609s
nom, allocation heavy version (second commit, LTO off): 0.405s
nom, allocation light version (third commit, LTO off): 0.282s
nom, allocation light version (third commit, LTO on, panic abort, 1 codegen unit): 0.200s
nom, allocation light version (third commit, LTO on, panic abort, 1 codegen unit, default allocator rather than jemalloc): 0.216s
nom, with rayon parallelism (NOTE: usertime): 0.512s
nom, with rayon parallelism (NOTE: realtime): 0.205s (gonna go back and check what the non-parallel version got for realtime )

### Rough Benchmarking Methodology
```
$ cargo build --release
$ time target/release/crunchy-vdf
```
`pl_goldrush_halloween.vmf` is embedded in the executable with either `include_bytes!` or `include_str!`, to avoid IO overhead.
The file is parsed a thousand times, and `#[no_mangle]` is used to ensure the unused return value is not optimized away.


## Comparison
swissChilli's [fastkv] doesn't work on my machine (as of cf53e6c),
and no other entries have been submitted.
According to one user's discord messages, they were getting about 200ms on halloween goldrush, so my parser is not horrendusly slow.

[fastkv]: https://github.com/swissChili/fastkv