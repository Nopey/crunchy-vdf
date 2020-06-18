# Crunchy VDF
Deep fried keyvalues `?:P`

A Rust keyvalues parser.
competition time bay-be

## Plan:
1. Make it work, using pest (probably would get dq'd because parser generator)
2. Make it work, using nom (and benchmark the two, too!)

## Results
initial P.o.C (pest): ??? TODO: Benchmark pest?
nom, allocation heavy version (second commit, LTO off): 0.405s for pl_goldrush_halloween.vmf
