# cuna

Another simple cue parser for Rust.
There is no document and the api needs improving, so [rcue](https://github.com/gyng/rcue) or [libcue.rs](https://github.com/mistydemeo/libcue.rs) (though I can't compile this) may be a better choice.

## performance
Here's a benchmark with a 42-line cue file.
``` rust
cuna                    time:   [24.431 us 24.577 us 24.722 us]
                        change: [+29.680% +31.073% +32.504%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

rcue                    time:   [94.889 us 95.035 us 95.208 us]
                        change: [+22.130% +23.267% +24.173%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

rcue_no_buf_reader      time:   [3.8906 us 3.9295 us 3.9688 us]
                        change: [-0.3731% +1.2804% +2.8758%] (p = 0.13 > 0.05)
                        No change in performance detected.```