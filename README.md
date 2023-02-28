<div align="center">
  <img src=".github/screenshot-light.png#gh-light-mode-only" width="600" />
  <img src=".github/screenshot-dark.png#gh-dark-mode-only" width="600" />
</div>

# Macfetch

![https://img.shields.io/github/issues/gantoreno/macfetch](https://img.shields.io/github/issues/gantoreno/macfetch) ![https://img.shields.io/github/forks/gantoreno/macfetch](https://img.shields.io/github/forks/gantoreno/macfetch) ![https://img.shields.io/github/stars/gantoreno/macfetch](https://img.shields.io/github/stars/gantoreno/macfetch) ![https://img.shields.io/github/license/gantoreno/macfetch](https://img.shields.io/github/license/gantoreno/macfetch)

A macOS [Neofetch](https://github.com/dylanaraps/neofetch) alternative, previously written in [C++](https://en.wikipedia.org/wiki/C++), now written in [Rust](https://www.rust-lang.org/).

> **Note**
> To view the previous files, visit the [`legacy`](https://github.com/gantoreno/macfetch/tree/legacy) branch, which contains the old C++ files for this project.

## Why?

_"But seriously, why? Why another Neofetch alternative?"_ I hear you ask, and the answer is pretty simple. There are multiple Neofetch rewrites out there, most of them probably faster than Macfetch, but besides doing absolutely the same as Neofetch (but much faster), they all share something in common, **macOS is not supported**.

- [`paleofetch`](https://github.com/ss7m/paleofetch) only supports Arch (and it's no longer maintained).
- [`fastfetch`](https://github.com/LinusDierheimer/fastfetch) works only on a handful of Linux distros.
- [`freshfetch`](https://github.com/K4rakara/freshfetch) same as the above.
- [`macchina`](https://github.com/Macchina-CLI/macchina) lacks the Neofetch aesthetic.
- [`afetch`](https://github.com/13-CF/afetch) is awesome, but not what I'm looking for out of the box.

Yes, Linux is supported, and that's great, but it's about time for a macOS alternative to come out, and that's why Macfetch (as its name suggests) is **macOS-only**, and not only that, **it's 95% faster**.

## Installation

You can either [install](https://github.com/gantoreno/macfetch#homebrew-installation) Macfetch via [Homebrew](https://brew.sh/), or you can [build from source](https://github.com/gantoreno/macfetch#build-from-source).

### Homebrew Installation

To install Macfetch via Homebrew, just tap the repository, and install like so:

```sh
$ brew tap gantoreno/macfetch
$ brew install macfetch
```

And voilà! Macfetch should now be installed under `/usr/local/Cellar/macfetch/<version>`.

### Build From Source

To build from source. Start by cloning the repo:

```sh
$ git clone https://github.com/gantoreno/macfetch.git
$ cd macfetch
```

And simply run with `cargo`:

```sh
$ cargo run # for the debug target
$ cargo run --release # for the optimized release target
```

To build, same thing:

```sh
$ cargo build # for the debug target
$ cargo build --release # for the optimized release target
```

Your binary should be available under `target/x86_64-apple-darwin/`and withing the folder of the build target you chose (either `debug` or `release`).

## Performance Notes

Macfetch, although _blazingly fast_, is not perfect. One of the segments in particular represents a huge drawback in performance, at least for the first cold run of the program – the GPU segment. This is because, as of right now, I haven't found better & optimal way of querying for the GPU vendor & name other than spawning a shell process and reading from `SPDisplaysDataType` with `system_profiler`, which is painfully slow.

For now, the workaround is to cache that value into `/Library/Caches/macfetch` and just run the command for the first run. After that, the segment should only read for the cached value, having a significan improvement in execution time.

## License

Licensed under the [MIT](https://opensource.org/licenses/MIT) license.
