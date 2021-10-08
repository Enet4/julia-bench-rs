# Julia's micro-bench in Rust

[![Rust CI](https://github.com/Enet4/julia-bench-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Enet4/julia-bench-rs/actions/workflows/rust.yml) [![dependency status](https://deps.rs/repo/github/Enet4/julia-bench-rs/status.svg)](https://deps.rs/repo/github/Enet4/julia-bench-rs)

This repository attempts to replicate the [micro benchmark](https://github.com/JuliaLang/julia/tree/master/test/perf/micro) in the Julia repository ([/test/perf](https://github.com/JuliaLang/julia/tree/master/test/perf)) with a Rust implementation, which aims to make a general performance comparison among other programming languages.

The resulting program is based on the C implementation, but it's still mostly idiomatic Rust and does not have unsafe code.

This program has two modes:

- By default, benchmarks with matrices (`randmatstat` and `randmatmul`) will use `ndarray` with experimental BLAS support.

- With the `direct_blas` Cargo feature, these benchmarks will use the C BLAS API directly via the `cblas` crate, which is currently faster but requires unsafe code.

In both cases, OpenBLAS is used as the BLAS implementation.

The project is experimental, and might still have bugs or unfair optimizations. Feel free to point them out if you find them.

## Building and running

This program uses a specific Rust **nightly** toolchain to work. With rustup, installing the toolchain mentioned in [rust-toolchain](rust-toolchain) is sufficient. Then:

```bash
cargo run --release
```

To run the direct BLAS version instead:

```bash
cargo run --release --features direct_blas
```

## Results

The Rust benchmarks (using nightly-2018-04-16) were compared with C (`-O3`) and Julia on the same machine.

![](benchmarks.svg)

| cputime |  lang
|---------|--------------------
| 1.000   |  C
| 1.000   |  Rust (direct BLAS)
| 1.060   |  Rust
| 1.180   |  Julia

Most benchmarks are faster in Rust than in Julia, sometimes even faster than C. The `fibonacci` benchmark relies on `black_box`, and so might not be very well optimized. These results also depend on the toolchain used. The toolchain `nightly-2017-10-24` is older, but would yield slightly better metrics.

## License

Apache-2.0 / MIT

This code is based on the C implementation of the benchmark, with the MIT license:

> Copyright (c) 2009-2016: Jeff Bezanson, Stefan Karpinski, Viral B. Shah,
> and other contributors:
>
> https://github.com/JuliaLang/julia/contributors
>
> Permission is hereby granted, free of charge, to any person obtaining
> a copy of this software and associated documentation files (the
> "Software"), to deal in the Software without restriction, including
> without limitation the rights to use, copy, modify, merge, publish,
> distribute, sublicense, and/or sell copies of the Software, and to
> permit persons to whom the Software is furnished to do so, subject to
> the following conditions:
>
> The above copyright notice and this permission notice shall be
> included in all copies or substantial portions of the Software.
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
> EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
> MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
> NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
> LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
> WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
