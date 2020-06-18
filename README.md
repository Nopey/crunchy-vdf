# Crunchy VDF
Deep fried keyvalues `?:P`

A Rust keyvalues parser.
competition time bay-be

## Plan:
1. Make it work, using pest (probably would get dq'd because parser generator)
2. Make it work, using nom (and benchmark the two, too!)


## Rough Benchmarks as I progress
(literally linux `time` command, and pl_goldrush_halloween.vmf a thousand times)
initial P.o.C (pest): ??? TODO: Benchmark pest?
nom, allocation heavy version (second commit, LTO off): 0.405s
nom, allocation light version (third commit, LTO off): 0.282s
nom, allocation light version (third commit, LTO on, panic abort, 1 codegen unit): 0.200s
nom, allocation light version (third commit, LTO on, panic abort, 1 codegen unit, default allocator rather than jemalloc): 0.216s

## Comparison

