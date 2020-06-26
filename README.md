# cuna

Another simple cue parser for Rust.
There is no document and the api needs improving, so [rcue](https://github.com/gyng/rcue) or [libcue.rs](https://github.com/mistydemeo/libcue.rs) (though I can't compile this) may be a better choice.

## performance
Here's a benchmark with a 42-line cue file.
```
running 4 tests
test cue_sheet_bench          ... bench:     236,118 ns/iter (+/- 20,831)
test cuna_bench               ... bench:      28,214 ns/iter (+/- 3,266)
test rcue_bench               ... bench:     107,593 ns/iter (+/- 8,354)
test rcue_bench_no_buf_reader ... bench:       3,885 ns/iter (+/- 527)
```