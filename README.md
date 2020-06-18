# Crunchy VDF
*Who's up for deep fried keyvalues `?:P`*

A Rust keyvalues parser.


Has an optional "parallel" feature, that isn't complete. It works, and is about as fast

Lacks several features, including actually evaluating conditionals, the #include and #base macros, and string escaping currently doesn't compile.

swissChilli's [fastkv] doesn't work on my machine (as of cf53e6c),
and no other entries have been submitted.
According to one user's discord messages, they were getting about 200ms on halloween goldrush, which is comparable to mine, so I assume my parser isn't horrendusly slow.

## TODO:
a bunch of tests aren't written, and `test_parse_auto_string` should check if the `escape_sequences` feature is enabled.

Compare it to fastkv on my machine so I know how much more performance can be squeezed out.

skip_braces bugs (applicable to both conditionals and parallel):
* strings are not skipped by skip_braces
* same for both kinds of comments

parallel's not measurably faster, at least with my rough benchmarking method on my 4 core laptop APU.

There's gotta be a SIMD way of searching for a char then `[u8]::contains(), but it probably will scale linearly with the number of chars we're testing for. SAD.

Implement the following features
* Handling escape sequences (with COWs, it'll be a little lighter on the allocator)
* Evaluating Conditionals (currently they're skipped)
* `#include` macro
* `#base` macro ? It'll be a pain, but whatever..

## Rough Benchmarks as I progress

* initial P.o.C (pest): 1.609s
* nom, allocation heavy version (second commit, LTO off): 0.405s
* nom, allocation light version (third commit, LTO off): 0.282s
* nom, allocation light version (third commit, LTO on, panic abort, 1 codegen unit): 0.200s
* nom, allocation light version (third commit, LTO on, panic abort, 1 codegen unit, default allocator rather than jemalloc): 0.216s
* nom, with rayon parallelism (NOTE: usertime): 0.512s
* nom, with rayon parallelism (NOTE: realtime): 0.205s (gonna go back and check what the non-parallel version got for realtime )

### Rough Benchmarking Method
```
$ cargo build --release
$ time target/release/crunchy-vdf
```
`pl_goldrush_halloween.vmf` is embedded in the executable with either `include_bytes!` or `include_str!`, to avoid IO overhead.
The file is parsed a thousand times, and `#[no_mangle]` is used to ensure the unused return value is not optimized away.

Output of lscpu:
```
Architecture:                    x86_64
CPU op-mode(s):                  32-bit, 64-bit
Byte Order:                      Little Endian
Address sizes:                   48 bits physical, 48 bits virtual
CPU(s):                          4
On-line CPU(s) list:             0-3
Thread(s) per core:              2
Core(s) per socket:              2
Socket(s):                       1
NUMA node(s):                    1
Vendor ID:                       AuthenticAMD
CPU family:                      21
Model:                           101
Model name:                      AMD A12-9700P RADEON R7, 10 COMPUTE CORES 4C+6G
Stepping:                        1
Frequency boost:                 enabled
CPU MHz:                         1529.984
CPU max MHz:                     2500.0000
CPU min MHz:                     1300.0000
BogoMIPS:                        4990.93
Virtualization:                  AMD-V
L1d cache:                       64 KiB
L1i cache:                       192 KiB
L2 cache:                        2 MiB
NUMA node0 CPU(s):               0-3
Vulnerability Itlb multihit:     Not affected
Vulnerability L1tf:              Not affected
Vulnerability Mds:               Not affected
Vulnerability Meltdown:          Not affected
Vulnerability Spec store bypass: Mitigation; Speculative Store Bypass disabled via prctl and seccomp
Vulnerability Spectre v1:        Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:        Mitigation; Full AMD retpoline, STIBP disabled, RSB filling
Vulnerability Tsx async abort:   Not affected
Flags:                           fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good acc_power nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs xop skinit wdt lwp fma4 tce nodeid_msr tbm topoext perfctr_core perfctr_nb bpext ptsc mwaitx cpb hw_pstate ssbd vmmcall fsgsbase bmi1 avx2 smep bmi2 xsaveopt arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif overflow_recov
```

[fastkv]: https://github.com/swissChili/fastkv
