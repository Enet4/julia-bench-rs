# Julia's micro-bench in Rust

This repository attempts to replicate the [micro benchmark](https://github.com/JuliaLang/julia/tree/master/test/perf/micro) in the Julia repository ([/test/perf](https://github.com/JuliaLang/julia/tree/master/test/perf)) with a Rust implementation, which aims to make a general performance comparison among other programming languages.

The resulting program is based on the C implementation, but it's still mostly idiomatic Rust and does not have unsafe code.
OpenBLAS is used as the BLAS implementation via the `blas` crate.

The project is experimental, and might still have bugs or unfair optimizations. Feel free to point them out if you find them.

## Building and running

Use the **nightly** channel of the Rust compiler. With rustup, installing the nightly toolchain and executing `rustup override set nightly` is sufficient. Then:

```bash
# in development mode (no optimizations)
cargo run
```

```bash
# in release mode (best performance)
cargo run --release
```

## Results

When comparing with the C (`-O3`) and Julia benchmarks on the same machine:

![](graph.png)

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