# Crunchy VDF
Deep fried keyvalues `?:P`

A Rust keyvalues parser.
competition time bay-be

## TODO:
Compare it to fastkv on my machine so I know how much more performance can be squeezed out.
Add feature gates for
* Handling escape sequences (Allocation Heaviness)
* Evaluating Conditionals (currently they're skipped)
* Experimental multithreading using rayon (top level)

## Rough Benchmarks as I progress
(literally linux `time` command, and pl_goldrush_halloween.vmf a thousand times)
initial P.o.C (pest): 1.609s
nom, allocation heavy version (second commit, LTO off): 0.405s
nom, allocation light version (third commit, LTO off): 0.282s
nom, allocation light version (third commit, LTO on, panic abort, 1 codegen unit): 0.200s
nom, allocation light version (third commit, LTO on, panic abort, 1 codegen unit, default allocator rather than jemalloc): 0.216s

## Comparison

