# EnergyMon Rust Bindings

The `energymon-osp-polling-sys` crate provides declarations and linkage for the
`energymon-osp-polling-static` C library.
Following the *-sys package conventions, this crate does not define
higher-level abstractions over the native library functions.

The latest `energymon` C libraries can be found at
[https://github.com/connorimes/energymon](https://github.com/connorimes/energymon).

## Dependencies

In order to use this crate, you should have the `energymon` libraries
installed to the system where they can be found by `pkg-config`.

If the libraries are not found, the build process will try to fetch and
compile them.

## Usage
Add `energymon-osp-polling-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies.energymon-osp-polling-sys]
git = "https://github.com/connorimes/energymon-sys.git"
```
