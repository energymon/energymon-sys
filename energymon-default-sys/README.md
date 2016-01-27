# EnergyMon Rust Bindings

The `energymon-default-sys` crate provides declarations and linkage for the
`energymon-default` C library.
Following the *-sys package conventions, this crate does not define
higher-level abstractions over the native library functions.

The latest `energymon` C libraries can be found at
[https://github.com/energymon/energymon](https://github.com/energymon/energymon).

## Dependencies

In order to use this crate, you should have the `energymon` libraries
installed to the system where they can be found by `pkg-config`.

This crate depends on the `energymon-sys` crate.

## Usage
Add `energymon-default-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies.energymon-default-sys]
git = "https://github.com/energymon/energymon-default-sys.git"
```

If an implementation is not found by `pkg-config`, one is compiled.
In this case, the implementation to compile and use can be specified by
setting the environment variable `ENERGYMON_DEFAULT_IMPL`, e.g.:

```sh
ENERGYMON_DEFAULT_IMPL=rapl cargo build
```

Otherwise a dummy implementation is used.
See the `energymon` documentation for acceptable values.
