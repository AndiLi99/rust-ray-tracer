# Rust Ray Tracer
A follow through of https://raytracing.github.io/books/RayTracingInOneWeekend.html in rust.

Notes
- How can light bouncing between two colored objects with no absorption be modeled, without making the light darker?

Ideas
- Use a different color model besides RGB
  - Perceptual color models
  - Subtractive color models for materials
  - HSL / HSV for specifiying color


 # Benchmarking
https://nnethercote.github.io/perf-book/benchmarking.html

Hyperfine is used to benchmark the program. 

## Results
### Preliminary Results (before any optimizations)
Using `hyperfine "cargo run --release"`.

```
Benchmark 1: cargo run --release
  Time (mean ± σ):      2.718 s ±  0.012 s    [User: 2.530 s, System: 0.019 s]
  Range (min … max):    2.699 s …  2.743 s    10 runs
```